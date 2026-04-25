use std::fs;
use std::path::PathBuf;

fn shell_html() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = root.join("assets").join("editor_shell.html");
    fs::read_to_string(path).expect("read editor_shell.html")
}

fn editor_toolbar_config(html: &str) -> &str {
    let start = html.find("toolbar: [").expect("find editor toolbar start");
    let tail = &html[start..];
    let end = tail
        .find("toolbarConfig:")
        .expect("find editor toolbar config end");
    &tail[..end]
}

#[test]
fn has_language_selector_control() {
    let html = shell_html();
    assert!(
        html.contains("id=\"langSelect\""),
        "expected a language selector element with id=langSelect"
    );
}

#[test]
fn includes_english_and_chinese_ui_dictionary() {
    let html = shell_html();
    assert!(
        html.contains("const UI_TEXT ="),
        "expected UI_TEXT dictionary in editor shell"
    );
    assert!(html.contains("Open"), "expected English UI labels");
    assert!(html.contains("打开"), "expected Chinese UI labels");
}

#[test]
fn defaults_to_english_without_saved_locale() {
    let html = shell_html();
    assert!(
        html.contains("const storedLocale = readStoredLocale();"),
        "expected bootstrap variable for saved locale"
    );
    assert!(
        html.contains("let appLocale = storedLocale ? normalizeLocale(storedLocale) : \"en\";"),
        "expected default locale to be English when no saved preference exists"
    );
}

#[test]
fn defaults_to_wysiwyg_editor_mode() {
    let html = shell_html();
    assert!(
        html.contains("let currentMode = \"wysiwyg\";"),
        "expected the editor shell to open in WYSIWYG mode by default"
    );
}

#[test]
fn only_forces_preview_container_visible_in_split_mode() {
    let html = shell_html();
    assert!(
        !html.contains("display: flex !important;"),
        "expected no CSS override that forces the preview container visible"
    );
    assert!(
        html.contains("if (currentMode !== \"sv\")"),
        "expected a split-mode guard before preview visibility overrides"
    );
    assert!(
        html.contains("previewEl.style.display = 'flex';"),
        "expected the preview container to be forced into flex layout in split mode"
    );
}

#[test]
fn toolbar_contains_version_badge() {
    let html = shell_html();
    assert!(
        html.contains("id=\"appVersion\""),
        "expected a toolbar element with id=appVersion"
    );
    assert!(
        html.contains("class=\"version-badge\""),
        "expected a dedicated version badge class in the editor shell"
    );
}

#[test]
fn app_shell_handles_redo_shortcuts_when_toolbar_redo_exists() {
    let html = shell_html();
    assert!(
        html.contains("function runEditorHistoryCommand(command)"),
        "expected app shell history shortcut bridge"
    );
    assert!(
        html.contains("key === \"y\"") && html.contains("runEditorHistoryCommand(\"redo\")"),
        "expected Ctrl/Cmd+Y to trigger editor redo"
    );
    assert!(
        html.contains("key === \"z\" && event.shiftKey")
            && html.contains("runEditorHistoryCommand(\"redo\")"),
        "expected Ctrl/Cmd+Shift+Z to trigger editor redo"
    );
}

#[test]
fn editor_toolbar_has_tooltip_annotation_pass() {
    let html = shell_html();
    assert!(
        html.contains("function annotateEditorToolbar()"),
        "expected app shell to annotate editor toolbar buttons with titles"
    );
    assert!(
        html.contains("setTimeout(annotateEditorToolbar, 200);"),
        "expected app shell to retry tooltip annotation after editor renders async toolbar items"
    );
}

#[test]
fn desktop_toolbar_keeps_secondary_tools_under_more_menu() {
    let html = shell_html();
    assert!(
        html.contains(
            r#"{ name: "more", toolbar: ["outline", "code-theme", "export", "both", "preview", "fullscreen"] }"#
        ),
        "expected secondary editor actions to live under the More toolbar menu"
    );
    assert!(
        !html.contains("\"content-theme\""),
        "expected desktop app not to expose the inert content theme preview button"
    );
}

#[test]
fn desktop_toolbar_hides_unreliable_authoring_tools() {
    let html = shell_html();
    let toolbar = editor_toolbar_config(&html);
    for tool in [r#""upload""#, r#""outdent""#, r#""indent""#, r#""code""#] {
        assert!(
            !toolbar.contains(tool),
            "expected desktop toolbar not to expose unreliable authoring tool {tool}"
        );
    }
    assert!(
        toolbar.contains(r#""inline-code""#),
        "expected inline code formatting to remain available"
    );
}

#[test]
fn app_shell_does_not_expose_upload_ipc() {
    let html = shell_html();
    assert!(
        !html.contains("function handleImageUpload"),
        "expected no renderer upload handler when upload UI is hidden"
    );
    assert!(
        !html.contains("cmd: \"upload_image\""),
        "expected renderer not to send upload_image IPC"
    );
    assert!(
        !html.contains("statusImageUploaded"),
        "expected no upload-only status strings in the app shell"
    );
}

#[test]
fn app_shell_confirms_native_close_when_tabs_are_dirty() {
    let html = shell_html();
    assert!(
        html.contains("function hasDirtyTabs()"),
        "expected helper that checks dirty tabs before native close"
    );
    assert!(
        html.contains("message.event === \"close_requested\""),
        "expected host close request handling"
    );
    assert!(
        html.contains("cmd: \"close_confirmed\""),
        "expected renderer to notify host after close is confirmed"
    );
}
