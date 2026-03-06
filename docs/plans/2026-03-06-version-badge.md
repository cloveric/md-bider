# Version Badge Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在顶部工具栏右侧常驻显示当前应用版本号，并保持跨语言和不同启动方式都能稳定显示。

**Architecture:** 版本号直接取自 Cargo 包版本，桌面端通过初始化脚本注入到 WebView，全前端只负责展示。这样不会引入新的存储状态，也不会和语言持久化逻辑耦合。

**Tech Stack:** Rust, Wry, Tao, HTML, CSS, Vanilla JavaScript, Cargo tests

---

### Task 1: Add failing tests for version exposure

**Files:**
- Modify: `C:\Users\hangw\projects\md-bider\tests\runtime_persistence.rs`
- Test: `C:\Users\hangw\projects\md-bider\tests\runtime_persistence.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn initialization_script_exposes_current_app_version() {
    let script = md_bider::app_init::build_initialization_script();

    assert!(script.contains("window.__APP_VERSION__"));
    assert!(script.contains(env!("CARGO_PKG_VERSION")));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test initialization_script_exposes_current_app_version --test runtime_persistence`
Expected: FAIL because the initialization script does not expose the version yet

**Step 3: Write minimal implementation**

Add a shared helper that builds the initialization script and includes the Cargo package version.

**Step 4: Run test to verify it passes**

Run: `cargo test initialization_script_exposes_current_app_version --test runtime_persistence`
Expected: PASS

**Step 5: Commit**

```bash
git add tests/runtime_persistence.rs src/main.rs src/lib.rs src/app_init.rs
git commit -m "feat: expose app version to webview"
```

### Task 2: Add failing test for toolbar version badge

**Files:**
- Modify: `C:\Users\hangw\projects\md-bider\tests\editor_shell_i18n.rs`
- Test: `C:\Users\hangw\projects\md-bider\tests\editor_shell_i18n.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn toolbar_contains_version_badge() {
    let html = include_str!("../assets/editor_shell.html");

    assert!(html.contains("id=\"appVersion\""));
    assert!(html.contains("class=\"version-badge\""));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test toolbar_contains_version_badge --test editor_shell_i18n`
Expected: FAIL because the toolbar badge is not present yet

**Step 3: Write minimal implementation**

Add the badge markup, small badge styles, and front-end code to populate the text from `window.__APP_VERSION__`.

**Step 4: Run test to verify it passes**

Run: `cargo test toolbar_contains_version_badge --test editor_shell_i18n`
Expected: PASS

**Step 5: Commit**

```bash
git add assets/editor_shell.html tests/editor_shell_i18n.rs
git commit -m "feat: show version badge in toolbar"
```

### Task 3: Verify, package, and release

**Files:**
- Modify: `C:\Users\hangw\projects\md-bider\Cargo.toml`
- Modify: `C:\Users\hangw\projects\md-bider\Cargo.lock`

**Step 1: Bump release version**

Update the Cargo package version for the new release.

**Step 2: Run tests**

Run: `cargo test`
Expected: PASS with 0 failed

**Step 3: Run release build**

Run: `cargo build --release`
Expected: PASS with exit code 0

**Step 4: Commit and tag**

```bash
git add Cargo.toml Cargo.lock assets/editor_shell.html src/*.rs tests/*.rs docs/plans/*.md
git commit -m "feat: show app version in toolbar"
git tag v0.4.7
```

**Step 5: Push and release**

```bash
git push origin main
git push origin v0.4.7
gh release create v0.4.7 dist/md-bider-v0.4.7-windows-x64.zip dist/md-bider-v0.4.7-macos-arm64.zip --generate-notes
```
