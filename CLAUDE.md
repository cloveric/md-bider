# md-bider (马得笔) - Project Documentation

## Overview

md-bider is a desktop Markdown editor built with Rust (wry/tao) for the native window and an embedded WebView for the editor UI. It supports three editing modes: Instant Render (IR), Split Preview (SV), and WYSIWYG.

## Architecture

### Tech Stack
- **Backend**: Rust with `wry` (WebView) and `tao` (window management)
- **Frontend**: Vanilla HTML/JS/CSS in a single `editor_shell.html`
- **Editor Engine**: Minified JS bundle embedded as base64 (`engine.js.b64`)
- **Markdown Parser**: Lute WASM, loaded at runtime from CDN
- **Icons**: Lucide icon set (SVG sprite, embedded as base64)
- **i18n**: Chinese translation file embedded as base64

### File Structure
```
src/
  main.rs          - Rust entry point, window creation, IPC handler
  lib.rs           - Library module (desktop IPC, io utilities)
  ui.rs            - Editor mode enum, locale helpers
assets/
  editor_shell.html - Complete frontend UI (toolbar, tabs, editor integration)
  app_icon.png      - Window icon
  vendor/
    engine.js.b64   - Editor engine JS (base64 encoded)
    engine.css.b64  - Editor engine CSS (base64 encoded)
    icon_ant.js.b64 - Lucide SVG icon sprite (base64 encoded)
    i18n_zh_cn.js.b64 - Chinese i18n strings (base64 encoded)
```

### Data Flow
1. Rust creates a native window with `tao` and a WebView with `wry`
2. Base64-encoded assets are injected as `window.__ENGINE_*_B64__` globals via initialization script
3. `editor_shell.html` decodes and evals these assets to bootstrap the editor
4. The editor loads Lute WASM from CDN (`cdn.jsdelivr.net/npm/vditor@3.11.2`) asynchronously
5. IPC between frontend and Rust uses `window.ipc.postMessage()` (frontend→Rust) and `webview.evaluate_script()` (Rust→frontend)

### Key Design Decisions
- **Embedded assets**: All core JS/CSS/icons are embedded as base64 in the binary via `include_str!`. This makes the app self-contained (single executable) but requires rebuilding to update assets.
- **CDN for Lute WASM**: The Lute parser (~4MB WASM) is loaded from CDN at runtime to avoid bloating the binary. This requires internet connectivity.
- **Base64 with certutil headers**: The `.b64` files use `-----BEGIN/END CERTIFICATE-----` headers (certutil format on Windows) for compatibility with the build pipeline.

## Lessons Learned & Debugging Notes

### Bug: Global string replacement corrupting unrelated code

**Problem**: When doing a global `vditor → mdbider` replacement on the minified engine.js, the replacement also affected:
1. **CDN URLs**: `cdn.jsdelivr.net/npm/vditor@` became `cdn.jsdelivr.net/npm/mdbider@` — this npm package doesn't exist, causing 404 errors for all CDN resources (Lute WASM, i18n, themes).
2. **Lute WASM API method names**: Methods like `SetVditorCodeBlockPreview`, `HTML2VditorDOM`, `SpinVditorIRDOM` etc. were renamed to `SetMdBiderCodeBlockPreview`, etc. — but the Lute WASM binary still exports the original names, causing `t.SetMdBiderCodeBlockPreview is not a function` errors.
3. **Version strings**: `"3.11.2"` was replaced to `"1.0.0"`, further breaking CDN paths.

**Root Cause**: The minified JS contains multiple categories of identifiers that look the same syntactically but serve different purposes:
- **UI identifiers** (class names, CSS selectors, global names) — safe to rename
- **CDN package coordinates** (npm package name, version) — must match the actual npm registry
- **WASM API method names** (Lute exports) — must match what the WASM binary exports
- **Property access patterns** (`.lute.`, `Lute.New()`) — must match the WASM runtime

**Fix**: Apply replacements in order of specificity:
1. First, set CDN URL and protect it from further replacement
2. Keep all Lute/WASM-related identifiers intact: `.lute.` property accesses, `Lute` class name, `*Vditor*` in Lute API methods
3. Only rename UI-level identifiers: CSS class prefixes, constructor name, i18n global name

**Prevention checklist for future engine.js modifications**:
- [ ] After replacement, verify CDN URL is a real, accessible URL (test with `curl -sI`)
- [ ] Search for `Vditor` in Lute API method names (pattern: `Set*Vditor*`, `*2Vditor*`, `Spin*Vditor*`) — these must NOT be renamed
- [ ] Verify `.lute.` property accesses are intact (should be ~70 occurrences)
- [ ] Verify `Lute.New`, `Lute.Caret`, `Lute.Version` etc. are intact
- [ ] Run the app and check for unhandled promise rejections (the Lute init is async, errors may be silent)
- [ ] Check that `editorRoot` has children after initialization (0 children = editor failed to render)

### Bug: `.lute.` replacement breaking `absolute`

**Problem**: An earlier attempt to rename `.lute.` to `.mp.` used a broad regex that also matched substrings inside words. The word `absolute` (in CSS property strings) contains `lute` and was corrupted to `abso.mp.` — causing `Unexpected number` syntax errors.

**Fix**: When replacing property access patterns like `.lute.`, always check that the preceding character is NOT an alphabetic character. Use a negative lookbehind: `/(?<![a-zA-Z])\.lute\./g`.

### Debugging tip: Add a visible debug overlay

When debugging WebView issues where you can't easily access DevTools:
1. Add a `<pre>` element with fixed positioning and high z-index
2. Log messages to it via `el.textContent += msg + "\n"`
3. Add `window.addEventListener("unhandledrejection", ...)` to catch async errors
4. Use timed checks (`setTimeout`) to monitor DOM state over time
5. Remove the overlay before committing

This technique revealed the `t.SetMdBiderCodeBlockPreview is not a function` error that was silently swallowed by an async promise chain.
