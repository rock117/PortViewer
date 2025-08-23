# Port Viewer - 项目结构和使用指南

## 🤔 为什么要在 `src-tauri` 目录运行 `cargo run`？

### **Tauri 2 项目架构解释**

Tauri 2 采用**分离式架构**，将前端和后端完全分开：

```
windows-port-viewer/
├── src-tauri/           # 🦀 Rust 后端项目（独立的 Rust 应用）
│   ├── Cargo.toml       # Rust 项目的依赖配置
│   ├── src/
│   │   ├── main.rs      # Rust 应用程序入口
│   │   ├── models.rs    # 数据模型
│   │   ├── network.rs   # 网络连接获取
│   │   ├── process.rs   # 进程信息处理
│   │   └── filter.rs    # 连接过滤逻辑
│   └── build.rs         # Tauri 构建脚本
├── dist/                # 🌐 前端文件
│   └── index.html       # Web 界面
├── tauri.conf.json      # ⚙️ Tauri 应用配置
└── icons/               # 🎨 应用图标
    └── icon.ico
```

### **关键理解**

#### **1. `src-tauri/` 是独立的 Rust 项目**
- 包含完整的 `Cargo.toml` 配置文件
- 有自己的依赖管理和构建系统
- 可以独立编译和运行
- **这是真正的 Rust 应用程序所在地**

#### **2. 项目根目录是 Tauri 工作空间**
- 主要用于 Tauri 配置（`tauri.conf.json`）
- 存放前端文件（`dist/`）
- 存放应用资源（`icons/`）
- **不是 Rust 项目本身**

## 🚀 正确的运行方式

### **方法 1：使用 Tauri CLI（推荐）**
```bash
# 从项目根目录运行
cd c:\rock\coding\code\my\rust\windows-port-viewer
cargo tauri dev
```

**优势：**
- ✅ 自动处理前端和后端的集成
- ✅ 启动开发服务器和热重载
- ✅ 完整的 Tauri 开发体验
- ✅ 自动打开 GUI 窗口

### **方法 2：直接运行后端（调试用）**
```bash
# 从 src-tauri 目录运行
cd c:\rock\coding\code\my\rust\windows-port-viewer\src-tauri
cargo run
```

**用途：**
- 🔧 调试后端 Rust 代码
- 🔧 测试 Tauri 命令
- 🔧 验证网络连接获取功能
- 🔧 独立运行 Rust 应用程序

### **方法 3：构建发布版本**
```bash
# 从项目根目录构建
cd c:\rock\coding\code\my\rust\windows-port-viewer
cargo tauri build

# 或者从 src-tauri 目录构建后端
cd src-tauri
cargo build --release
```

## 📁 项目结构详解

### **后端部分（`src-tauri/`）**
```
src-tauri/
├── Cargo.toml           # Rust 项目配置
│   ├── [package]        # 项目元信息
│   ├── [dependencies]   # Rust 依赖（tauri, serde, windows）
│   └── [build-dependencies] # 构建依赖（tauri-build）
├── src/
│   ├── main.rs          # 主程序入口，包含 Tauri 命令
│   ├── models.rs        # ConnectionInfo 结构体和枚举
│   ├── network.rs       # Windows API 网络连接获取
│   ├── process.rs       # 进程信息和名称获取
│   └── filter.rs        # 连接过滤逻辑
└── build.rs             # Tauri 构建脚本
```

### **前端部分（`dist/`）**
```
dist/
└── index.html           # 完整的 GUI 界面
    ├── HTML 结构        # 端口列表、过滤器、统计面板
    ├── CSS 样式         # 现代化渐变设计
    └── JavaScript       # Tauri API 调用和交互逻辑
```

### **配置部分**
```
tauri.conf.json          # Tauri 应用配置
├── app                  # 应用基本信息
├── build                # 构建配置
├── bundle               # 打包配置
└── plugins              # 插件配置
```

## 🔧 开发工作流

### **日常开发**
```bash
# 1. 启动开发环境
cd c:\rock\coding\code\my\rust\windows-port-viewer
cargo tauri dev

# 2. 修改后端代码（自动重新编译）
# 编辑 src-tauri/src/*.rs 文件

# 3. 修改前端代码（自动刷新）
# 编辑 dist/index.html
```

### **调试后端**
```bash
# 单独测试 Rust 后端
cd src-tauri
cargo run

# 查看日志输出
cargo run 2>&1 | tee debug.log
```

### **构建发布版本**
```bash
# 构建完整应用
cargo tauri build

# 只构建后端
cd src-tauri
cargo build --release
```

## ❓ 常见问题解答

### **Q: 为什么不能在项目根目录运行 `cargo run`？**
**A:** 因为项目根目录不是 Rust 项目，没有有效的 `Cargo.toml`。真正的 Rust 项目在 `src-tauri/` 目录中。

### **Q: 我应该使用哪种运行方式？**
**A:** 
- **开发 GUI 功能**：使用 `cargo tauri dev`
- **调试后端逻辑**：使用 `cd src-tauri && cargo run`
- **发布应用**：使用 `cargo tauri build`

### **Q: 如何安装 Tauri CLI？**
**A:** 
```bash
cargo install tauri-cli
```

### **Q: 构建失败怎么办？**
**A:** 
```bash
# 清理缓存
cd src-tauri
cargo clean

# 重新构建
cargo build
```

## 🎯 总结

**为什么在 `src-tauri` 运行 `cargo run`？**

1. **`src-tauri/` 是真正的 Rust 项目** - 包含 `Cargo.toml` 和源代码
2. **项目根目录是工作空间** - 用于 Tauri 配置和前端文件
3. **分离式架构** - 前端和后端完全独立
4. **开发灵活性** - 可以独立调试后端或完整运行 GUI

**推荐的开发方式：**
- 🎯 **日常开发**：`cargo tauri dev`（从根目录）
- 🔧 **后端调试**：`cargo run`（从 `src-tauri` 目录）
- 📦 **发布构建**：`cargo tauri build`（从根目录）

这样的架构设计让你可以灵活地开发和调试 Tauri 应用程序！🚀
