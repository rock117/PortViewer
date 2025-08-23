# Port Viewer - GUI 使用指南

## 🎉 Tauri 2 GUI 应用程序已完成！

Port Viewer 现在支持两种运行模式：
- **命令行模式 (CLI)** - 传统的终端界面
- **图形界面模式 (GUI)** - 现代化的桌面应用程序

## 🚀 启动应用程序

### GUI 模式（推荐）
```bash
# 方法 1：直接运行（无参数自动启动 GUI）
cargo run

# 方法 2：显式启动 GUI
cargo run -- --gui

# 或者直接运行编译后的可执行文件
target\debug\windows-port-viewer.exe
```

### CLI 模式
```bash
# 显示所有连接
cargo run -- --protocol all

# 只显示 TCP 连接
cargo run -- --protocol tcp

# 只显示 UDP 连接
cargo run -- --protocol udp

# 过滤特定端口
cargo run -- --port 443

# 组合过滤：TCP 协议的 80 端口
cargo run -- --protocol tcp --port 80

# 显示帮助
cargo run -- --help
```

## 🖥️ GUI 界面功能

### 主要特性
- **实时监控** - 每 5 秒自动刷新网络连接数据
- **智能过滤** - 支持按协议（TCP/UDP）和端口号过滤
- **统计面板** - 实时显示连接统计信息
- **现代设计** - 渐变背景、毛玻璃效果、响应式布局
- **数据表格** - 清晰的连接信息展示

### 界面布局
```
┌─────────────────────────────────────────────────────────┐
│                Port Viewer                     │
│           Monitor and analyze network connections       │
├─────────────────────────────────────────────────────────┤
│ Protocol: [All ▼] Port: [____] [🔄 Refresh]           │
├─────────────────────────────────────────────────────────┤
│ [📊 Total: 45] [📊 TCP: 38] [📊 UDP: 7]               │
├─────────────────────────────────────────────────────────┤
│ Protocol │ Local Address    │ Remote Address │ State   │
│ TCP      │ 192.168.1.4:1025│ 4.145.79.81:443│ESTABLISHED│
│ TCP      │ 192.168.1.4:1785│ 124.237.225.21:443│ESTABLISHED│
│ UDP      │ 0.0.0.0:53      │ *:0            │LISTENING│
└─────────────────────────────────────────────────────────┘
```

### 控制选项
1. **协议过滤器**
   - All Protocols - 显示所有连接
   - TCP - 仅显示 TCP 连接
   - UDP - 仅显示 UDP 连接

2. **端口过滤器**
   - 输入端口号（1-65535）
   - 匹配本地端口或远程端口
   - 留空显示所有端口

3. **刷新按钮**
   - 手动刷新连接数据
   - 除了自动刷新外的额外控制

### 数据显示
- **Protocol** - 连接协议（TCP/UDP）
- **Local Address** - 本地地址:端口
- **Remote Address** - 远程地址:端口
- **State** - 连接状态（ESTABLISHED, LISTENING, 等）
- **PID** - 进程 ID
- **Process Name** - 进程可执行文件名

### 状态颜色编码
- **TCP** - 青色背景
- **UDP** - 橙色背景
- **ESTABLISHED** - 绿色背景
- **LISTENING** - 蓝色背景
- **CLOSE_WAIT** - 橙色背景
- **TIME_WAIT** - 红色背景

## 🛠️ 技术架构

### 后端 (Rust)
- **Tauri 2.0** - 现代化桌面应用框架
- **Windows API** - 原生网络连接枚举
- **类型安全** - 枚举驱动的协议和状态管理
- **模块化设计** - 清晰的代码组织结构

### 前端 (Web Technologies)
- **HTML5** - 语义化标记
- **CSS3** - 现代样式和动画
- **JavaScript** - 交互逻辑和 Tauri API 调用
- **响应式设计** - 适配不同屏幕尺寸

### Tauri 命令
- `get_connections()` - 获取所有网络连接
- `get_filtered_connections(protocol, port)` - 获取过滤后的连接

## 🔧 开发和构建

### 开发环境要求
- Rust 1.70+
- Windows 10/11
- Visual Studio Build Tools
- WebView2 Runtime

### 构建命令
```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

### 项目结构
```
windows-port-viewer/
├── src/
│   ├── main.rs          # 主程序入口（CLI/GUI 双模式）
│   ├── models.rs        # 数据模型和枚举
│   ├── network.rs       # 网络连接获取
│   ├── process.rs       # 进程信息处理
│   ├── filter.rs        # 连接过滤逻辑
│   └── display.rs       # CLI 显示格式化
├── ui/
│   └── dist/
│       └── index.html   # GUI 前端界面
├── icons/
│   └── icon.ico         # 应用程序图标
├── tauri.conf.json      # Tauri 配置
├── build.rs             # 构建脚本
└── Cargo.toml           # 项目依赖
```

## 🎯 使用场景

### 网络管理员
- 监控服务器端口使用情况
- 识别异常网络连接
- 排查端口冲突问题

### 开发人员
- 调试网络应用程序
- 验证端口绑定状态
- 监控应用程序网络行为

### 安全分析师
- 检测可疑网络连接
- 分析恶意软件网络活动
- 监控系统网络安全状态

## 📊 性能特性

- **快速启动** - 秒级应用程序启动时间
- **低资源占用** - 最小化内存和 CPU 使用
- **实时更新** - 高效的数据刷新机制
- **响应式界面** - 流畅的用户交互体验

## 🔒 权限要求

- **管理员权限（推荐）** - 获取完整的进程名称信息
- **普通用户权限** - 基本的网络连接监控功能

## 🐛 故障排除

### 常见问题
1. **进程名显示为 `<Unknown>`**
   - 原因：权限不足
   - 解决：以管理员身份运行

2. **GUI 无法启动**
   - 检查 WebView2 Runtime 是否安装
   - 确认 Windows 版本兼容性

3. **数据不刷新**
   - 检查网络权限
   - 重启应用程序

## 🎉 总结

Port Viewer 现在是一个功能完整的现代化桌面应用程序，提供：
- 双模式支持（CLI + GUI）
- 实时网络监控
- 直观的用户界面
- 强大的过滤功能
- 专业的数据展示

无论是系统管理、开发调试还是安全分析，都能为您提供强大的网络连接监控能力！
