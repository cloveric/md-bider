use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct UploadedAssetRegistry {
    asset_dirs: HashSet<PathBuf>,
}

impl UploadedAssetRegistry {
    pub fn register_document_path(&mut self, path: &Path) {
        if let Some(parent) = path.parent() {
            self.asset_dirs.insert(parent.join("assets"));
        }
    }

    pub fn resolve_request_path(&self, request_path: &str) -> Option<PathBuf> {
        let key = request_path.trim_start_matches('/');
        if !key.starts_with("assets/") {
            return None;
        }

        let file_name = sanitize_upload_name(key.strip_prefix("assets/").unwrap_or(key));
        self.asset_dirs
            .iter()
            .map(|dir| dir.join(&file_name))
            .find(|path| path.is_file())
    }
}

pub fn sanitize_upload_name(name: &str) -> String {
    let basename = Path::new(name)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("image.png");
    let sanitized = basename
        .chars()
        .map(|ch| {
            if ch.is_control() || matches!(ch, '/' | '\\' | ':') {
                '_'
            } else {
                ch
            }
        })
        .collect::<String>();
    let trimmed = sanitized.trim_matches('.').trim();
    if trimmed.is_empty() {
        "image.png".to_owned()
    } else {
        trimmed.to_owned()
    }
}

pub fn content_type_for_path(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => "image/png",
    }
}

#[cfg(test)]
mod tests {
    use super::{UploadedAssetRegistry, sanitize_upload_name};

    #[test]
    fn upload_name_is_reduced_to_safe_basename() {
        assert_eq!(sanitize_upload_name("../outside.png"), "outside.png");
        assert_eq!(sanitize_upload_name("nested/evil.jpg"), "evil.jpg");
        assert_eq!(sanitize_upload_name("bad\nname.jpg"), "bad_name.jpg");
        assert_eq!(sanitize_upload_name("..."), "image.png");
    }

    #[test]
    fn registry_only_resolves_assets_paths() {
        let dir = std::env::temp_dir().join(format!("md-bider-assets-test-{}", std::process::id()));
        let assets_dir = dir.join("assets");
        std::fs::create_dir_all(&assets_dir).expect("create assets dir");
        let image_path = assets_dir.join("photo.png");
        std::fs::write(&image_path, b"png").expect("write image");
        let document_path = dir.join("demo.md");
        std::fs::write(&document_path, b"![photo](assets/photo.png)").expect("write document");

        let mut registry = UploadedAssetRegistry::default();
        registry.register_document_path(&document_path);

        assert_eq!(
            registry.resolve_request_path("/assets/photo.png"),
            Some(image_path)
        );
        assert_eq!(registry.resolve_request_path("/index.html"), None);

        let _ = std::fs::remove_dir_all(dir);
    }
}
