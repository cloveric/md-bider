# Version Badge Design

**Date:** 2026-03-06

## Goal

在应用顶部工具栏右侧常驻显示当前版本号，避免用户在截图、桌面快捷方式和不同打包文件之间混淆版本。

## Context

当前界面顶部右侧只显示文件路径和状态文本，没有任何版本标识。窗口标题虽然可以扩展，但在不同系统和截图场景下可见性不稳定，不适合作为主要版本识别入口。

## Options

### Option 1: Toolbar Badge

在顶部工具栏右侧新增只读版本标签，显示 `v0.x.y`。

- 优点：始终可见，和现有状态信息处于同一视觉区域，最适合截图和远程排查。
- 缺点：占用少量顶部宽度，需要兼顾中英文界面和窄窗口。

### Option 2: Window Title

在窗口标题中拼接版本号。

- 优点：实现简单，对 HTML 布局零改动。
- 缺点：标题栏在不同平台和截图里不稳定，不能保证用户始终看得见。

### Option 3: About Entry Only

增加“关于”入口，在弹层或菜单里显示版本号。

- 优点：界面最干净。
- 缺点：不满足“避免弄混版本”的主要诉求。

## Decision

采用 Option 1，在工具栏右侧增加常驻版本标签。

## Architecture

版本号来源直接使用 Rust 编译时常量 `env!("CARGO_PKG_VERSION")`，由桌面端在 WebView 初始化脚本中注入到前端全局变量。前端读取该变量并在工具栏右侧渲染 `v{version}` 文本，不参与语言切换逻辑，也不依赖本地存储。

## Components

- `src/main.rs`
  - 注入 `window.__APP_VERSION__`
- `assets/editor_shell.html`
  - 新增版本标签 DOM
  - 新增轻量样式
  - 初始化时填充版本文本
- `tests/*`
  - 增加回归测试，确保版本变量被注入且版本标签存在于工具栏右侧

## Error Handling

如果版本变量缺失，前端回退为 `vdev`，避免出现空白 UI 元素或脚本报错。

## Testing

- 为注入脚本增加测试，确认包含当前 Cargo 版本号
- 为 HTML 模板增加测试，确认存在工具栏版本标签
- 运行全量 `cargo test`
- 运行 `cargo build --release`
