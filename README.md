# Slint Stu - 一个基于 Rust 和 Slint 的终端工具原型

[![Rust](https://img.shields.io/badge/Rust-1.86.0-orange?logo=rust)](https://www.rust-lang.org/)
[![Slint](https://img.shields.io/badge/Slint-1.0+-blue?logo=slint)](https://slint-ui.com/)

## 项目描述

**Slint Stu** 是一个使用 Rust 和 Slint UI 框架构建的桌面应用原型。目前，它实现了一个无边框窗口界面，支持拖动、macOS 风格的标题栏（交通灯按钮）和基本布局。项目的**最终目标**是演变为一个类似于 Xshell 的终端工具，支持 SSH 远程连接、终端仿真、多标签会话管理、文件传输等功能。

这是一个学习和实验项目，旨在探索 Rust 在 GUI 开发中的应用，并逐步构建一个高效的跨平台终端客户端。

### 当前状态
- 无边框窗口，支持鼠标左键拖动。
- 自定义 UI 组件（如 MyIconButton，用于标题栏按钮的 hover 和 pressed 状态切换）。
- 基本布局：HeaderBar（顶部栏）、MainWindow（主内容区）和 FooterBar（底部栏）。
- 未来计划：集成终端仿真库（如 `vte` 或 `alacritty_terminal`），添加 SSH 支持（使用 `russh`  crate），实现多标签和配置管理。

## 特性

- **UI 框架**：使用 Slint 构建 declarative UI，支持自定义组件和事件处理。
- **窗口管理**：无边框设计，支持拖动和缩放（未来扩展）。
- **自定义按钮**：交通灯风格的关闭、最小化、最大化按钮，支持 hover、pressed 状态和动画过渡。
- **跨平台**：基于 Rust 和 Slint，支持 Windows、macOS 和 Linux。
- **未来功能**（ roadmap）：
  - SSH 连接和会话管理。
  - 内置终端仿真，支持 ANSI 颜色和输入/输出。
  - 多标签界面和会话切换。
  - 配置保存（如主机列表、认证密钥）。
  - 文件传输（SFTP 支持）。

## 安装和依赖

### 先决条件
- Rust 1.86.0 或更高版本（推荐使用 `rustup` 安装）。
- Cargo（Rust 的包管理器）。
- Slint SDK（通过 Cargo 依赖自动安装）。

### 步骤
1. 克隆仓库：
  ```
   git clone https://github.com/jychen/slint_stu.git
   cd slint_stu
   ```

2. 安装依赖：
   ```
   cargo build
   ```

3. 运行应用：
   ```
   cargo run
   ```

如果遇到构建问题，确保你的系统有必要的工具链（例如，macOS 上需要 Xcode Command Line Tools）。

## 使用说明

- **运行**：执行 `cargo run`，应用会启动一个无边框窗口。
- **交互**：
  - 用鼠标左键点击并拖动标题栏（HeaderBar）来移动窗口。
  - 悬停或点击标题栏按钮：观察图标状态切换（normal -> hover -> pressed）。
- **开发**：UI 定义在 `ui/` 目录下（Slint 文件），Rust 逻辑在 `src/main.rs` 中。
- **自定义**：你可以修改 `ui/app.slint` 来调整布局，或添加新组件。

## 贡献

欢迎贡献！如果你想帮助实现终端功能或其他改进：
1. Fork 仓库。
2. 创建你的分支（`git checkout -b feature/new-feature`）。
3. 提交更改（`git commit -m 'Add new feature'`）。
4. 推送分支（`git push origin feature/new-feature`）。
5. 打开 Pull Request。

请遵循 Rust 代码风格，并为新功能添加测试。

## 许可证

本项目采用 MIT 许可证。详情见 [LICENSE](LICENSE) 文件（如果不存在，可以添加一个标准 MIT 许可证）。

## 联系

- 作者：jychen (1920389863@qq.com)


感谢使用 Slint Stu！如果有 bug 或建议，请提交 Issue。
