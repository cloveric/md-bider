use std::sync::OnceLock;

const ENGINE_JS_B64: &str = include_str!("../assets/vendor/engine.js.b64");
const ENGINE_CSS_B64: &str = include_str!("../assets/vendor/engine.css.b64");
const ENGINE_ICON_B64: &str = include_str!("../assets/vendor/icon_ant.js.b64");
const I18N_ZH_CN_B64: &str = include_str!("../assets/vendor/i18n_zh_cn.js.b64");

static INIT_SCRIPT: OnceLock<String> = OnceLock::new();

fn certutil_base64_body(data: &str) -> String {
    data.lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>()
}

pub fn build_initialization_script() -> &'static str {
    INIT_SCRIPT
        .get_or_init(|| {
            let js_b64 = certutil_base64_body(ENGINE_JS_B64);
            let css_b64 = certutil_base64_body(ENGINE_CSS_B64);
            let icon_b64 = certutil_base64_body(ENGINE_ICON_B64);
            let i18n_b64 = certutil_base64_body(I18N_ZH_CN_B64);
            let app_version = env!("CARGO_PKG_VERSION");

            format!(
                "window.__ENGINE_JS_B64__ = {}; window.__ENGINE_CSS_B64__ = {}; window.__ENGINE_ICON_B64__ = {}; window.__ENGINE_I18N_B64__ = {}; window.__APP_VERSION__ = {};",
                serde_json::to_string(&js_b64).unwrap_or_else(|_| "\"\"".to_owned()),
                serde_json::to_string(&css_b64).unwrap_or_else(|_| "\"\"".to_owned()),
                serde_json::to_string(&icon_b64).unwrap_or_else(|_| "\"\"".to_owned()),
                serde_json::to_string(&i18n_b64).unwrap_or_else(|_| "\"\"".to_owned()),
                serde_json::to_string(app_version).unwrap_or_else(|_| "\"dev\"".to_owned())
            )
        })
        .as_str()
}
