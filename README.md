# Windows Port Viewer

🚧 **项目开发中 | Project in Development** 🚧

A modern desktop application for monitoring network port usage on Windows systems, built with **Rust + Tauri 2 + Nuxt 3**.

## 🔄 Development Status

**Current Version**: `v0.1.0-dev`  
**Status**: 🟡 **Active Development**  
**Last Updated**: 2025-01-08

### ✅ Completed Features
- ✅ Rust backend with Windows API integration
- ✅ Real-time TCP/UDP connection monitoring
- ✅ Process name resolution and PID tracking
- ✅ Modern Nuxt 3 frontend with Vue.js components
- ✅ Tauri 2 desktop application framework
- ✅ Protocol filtering (TCP/UDP/All)
- ✅ Port number filtering with prefix matching
- ✅ Process name filtering
- ✅ Auto-refresh with configurable intervals
- ✅ Responsive UI with Tailwind CSS
- ✅ Connection statistics and sorting
- ✅ Anti-jitter UI improvements

### 🚧 In Progress
- 🔄 Frontend debugging and console logging improvements
- 🔄 UI/UX polish and performance optimization
- 🔄 Release mode configuration and testing

### 📋 Planned Features
- 📅 Connection history tracking
- 📅 Export functionality (CSV/JSON)
- 📅 Network traffic monitoring
- 📅 Advanced filtering and search
- 📅 System tray integration
- 📅 Dark/Light theme toggle
- 📅 Multi-language support

## 🎆 Features

### 🖥️ Desktop Application
- **Modern GUI**: Built with Tauri 2 for native desktop performance
- **Vue.js Frontend**: Responsive and interactive user interface
- **Real-time Updates**: Live connection monitoring with configurable refresh intervals
- **Cross-platform Ready**: Tauri framework supports future multi-platform deployment

### 🔍 Network Monitoring
- **Comprehensive Port Monitoring**: Display all TCP and UDP connections with detailed information
- **Process Name Resolution**: Shows the executable name for each process using network ports
- **Connection Statistics**: Real-time statistics with connection counts and states
- **Smart Filtering**: Protocol, port, and process name filtering with intelligent matching

### 🎨 User Experience
- **Clean Modern UI**: Tailwind CSS styling with responsive design
- **Anti-jitter Technology**: Smooth updates without layout shifts
- **Sortable Tables**: Click column headers to sort connections
- **Loading States**: Skeleton loading and smooth transitions

## 🛠️ Technology Stack

### Backend
- **Rust**: High-performance system programming language
- **Windows API**: Direct integration with Windows networking APIs
- **Tauri 2**: Modern desktop application framework

### Frontend
- **Nuxt 3**: Vue.js framework with SSR/SSG capabilities
- **Vue.js 3**: Composition API and reactive components
- **Tailwind CSS**: Utility-first CSS framework
- **TypeScript**: Type-safe JavaScript development

## 💻 Development Setup

### Prerequisites

- **Windows 10/11**: Required for Windows API access
- **Rust Toolchain**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Version 18+ for frontend development
- **Tauri CLI**: Install with `cargo install tauri-cli`

### Development Environment

1. **Clone the repository**:
```bash
git clone <repository-url>
cd windows-port-viewer
```

2. **Install frontend dependencies**:
```bash
npm install
```

3. **Run in development mode**:
```bash
cargo tauri dev
```

4. **Build for production**:
```bash
npm run build
cargo tauri build
```

## 🔨 编译和打包 | Build & Package

### 📦 完整构建流程 | Complete Build Process

#### 1. 环境检查 | Environment Check
```bash
# 检查 Rust 版本 | Check Rust version
rustc --version
cargo --version

# 检查 Node.js 版本 | Check Node.js version
node --version
npm --version

# 检查 Tauri CLI | Check Tauri CLI
cargo tauri --version
```

#### 2. 前端构建 | Frontend Build
```bash
# 安装依赖 | Install dependencies
npm install

# 构建 Nuxt 静态文件 | Build Nuxt static files
npm run build

# 验证构建输出 | Verify build output
ls -la dist/
```

#### 3. Rust 后端编译 | Rust Backend Compilation
```bash
# 开发模式编译 | Development build
cargo build

# 生产模式编译 | Release build
cargo build --release

# 检查编译结果 | Check build results
ls -la src-tauri/target/release/
```

#### 4. Tauri 应用打包 | Tauri App Packaging
```bash
# 完整打包流程 | Complete packaging process
cargo tauri build

# 指定目标架构 | Specify target architecture
cargo tauri build --target x86_64-pc-windows-msvc

# 调试模式打包 | Debug mode packaging
cargo tauri build --debug
```

### 🚀 运行方式 | Running Methods

#### 开发模式 | Development Mode
```bash
# 标准开发模式（推荐）| Standard dev mode (recommended)
cargo tauri dev

# 带调试信息的开发模式 | Dev mode with debug info
RUST_LOG=debug cargo tauri dev

# 仅启动前端开发服务器 | Frontend dev server only
npm run dev
```

#### 生产模式 | Production Mode
```bash
# 运行构建后的可执行文件 | Run built executable
.\src-tauri\target\release\windows-port-viewer.exe

# 带控制台输出运行 | Run with console output
.\src-tauri\target\release\windows-port-viewer.exe > output.log 2>&1

# 后台运行 | Run in background
start "" ".\src-tauri\target\release\windows-port-viewer.exe"
```

### 📁 构建输出说明 | Build Output Description

#### 目录结构 | Directory Structure
```
src-tauri/target/
├── debug/                          # 调试版本 | Debug builds
│   ├── windows-port-viewer.exe     # 调试可执行文件 | Debug executable
│   └── deps/                       # 依赖文件 | Dependencies
├── release/                        # 发布版本 | Release builds
│   ├── windows-port-viewer.exe     # 发布可执行文件 | Release executable
│   ├── bundle/                     # 打包文件 | Bundle files
│   │   ├── msi/                    # MSI 安装包 | MSI installer
│   │   └── nsis/                   # NSIS 安装包 | NSIS installer
│   └── deps/                       # 依赖文件 | Dependencies
└── build/                          # 构建缓存 | Build cache
```

#### 文件说明 | File Description
- **`windows-port-viewer.exe`**: 主程序可执行文件 | Main executable
- **`bundle/msi/`**: Windows MSI 安装包 | Windows MSI installer
- **`bundle/nsis/`**: NSIS 安装程序 | NSIS installer
- **`deps/`**: 编译依赖和中间文件 | Compilation dependencies

### ⚡ 快速命令 | Quick Commands

```bash
# 一键开发 | One-click development
npm run tauri:dev

# 一键构建 | One-click build
npm run tauri:build

# 清理构建缓存 | Clean build cache
cargo clean
npm run clean

# 重新构建 | Rebuild from scratch
cargo clean && npm run build && cargo tauri build
```

### 📝 Development Notes

- **Frontend Dev Server**: Runs on `http://localhost:1420`
- **Hot Reload**: Both Rust backend and Nuxt frontend support hot reload
- **Debug Console**: Use `F12` in dev mode to access browser developer tools
- **Build Output**: Production builds are located in `src-tauri/target/release/`
- **Bundle Size**: Release executable is typically 8-15MB
- **Build Time**: Full release build takes 2-5 minutes depending on hardware

## 🚀 Usage

### Running the Application

**Development Mode** (with hot reload):
```bash
cargo tauri dev
```

**Production Build**:
```bash
# Build the application
npm run build
cargo tauri build

# Run the built executable
.\src-tauri\target\release\windows-port-viewer.exe
```

### 🎮 User Interface

#### Main Features
1. **Connection Table**: View all active TCP/UDP connections
2. **Filter Controls**: 
   - Protocol dropdown (All/TCP/UDP)
   - Port search box (supports prefix matching)
   - Process name search
3. **Auto Refresh**: Configurable intervals (2s, 5s, 10s, 30s, 1m)
4. **Statistics Panel**: Real-time connection counts
5. **Sorting**: Click column headers to sort data

#### Keyboard Shortcuts
- `F12`: Open developer tools (development mode)
- `Ctrl+R`: Manual refresh
- `Esc`: Clear filters

### 🔧 Advanced Usage

#### Debug Mode
For troubleshooting and development:
```bash
# Enable debug logging
RUST_LOG=debug cargo tauri dev

# View console output in release mode
.\src-tauri\target\release\windows-port-viewer.exe > debug.log 2>&1
```

#### Custom Configuration
The application supports various configuration options through the Tauri config file (`src-tauri/tauri.conf.json`).

## Output Format

The tool displays network connections in a table format with the following columns:

| Column | Description |
|--------|-------------|
| Protocol | Connection protocol (TCP/UDP) |
| Local Address | Local IP address and port |
| Remote Address | Remote IP address and port (or *:* for listening ports) |
| State | Connection state (LISTEN, ESTABLISHED, etc.) |
| PID | Process ID using the port |
| Process Name | Executable name of the process |

### Example Output

```
Windows Port Usage:
Protocol Local Address          Remote Address         State        PID      Process Name        
--------------------------------------------------------------------------------------------------------------
TCP      0.0.0.0:135            *:*                    LISTEN       1908     svchost.exe         
TCP      127.0.0.1:3306         *:*                    LISTEN       9684     mysqld.exe          
TCP      192.168.1.100:443      74.125.224.108:443     ESTABLISHED  5432     chrome.exe          
UDP      0.0.0.0:53             *:*                    LISTENING    5040     dns.exe             
UDP      127.0.0.1:1900         *:*                    LISTENING    20704    svchost.exe         
```

## Technical Details

### Windows APIs Used

- **GetExtendedTcpTable**: Retrieves TCP connection information with process IDs
- **GetExtendedUdpTable**: Retrieves UDP connection information with process IDs
- **OpenProcess**: Opens process handles for querying process information
- **QueryFullProcessImageNameW**: Gets the full path of process executables

### Dependencies

- `windows`: Windows API bindings for Rust
- `clap`: Command-line argument parsing

### Connection States

The tool displays various TCP connection states:

- **LISTEN**: Port is listening for incoming connections
- **ESTABLISHED**: Active connection established
- **SYN_SENT**: Connection request sent
- **SYN_RCVD**: Connection request received
- **FIN_WAIT1/FIN_WAIT2**: Connection termination in progress
- **CLOSE_WAIT**: Waiting for connection termination
- **TIME_WAIT**: Waiting for network to clear duplicate packets
- **CLOSED**: Connection closed

## Permissions

Some processes may show as `<Unknown>` in the Process Name column. This typically occurs when:

- The process requires elevated privileges to access
- The process has terminated but the connection is still being cleaned up
- System-level processes that restrict access

To see more process names, consider running the tool with administrator privileges.

## Use Cases

- **Network Debugging**: Identify which processes are using specific ports
- **Security Monitoring**: Monitor for unexpected network connections
- **System Administration**: Manage services and applications using network resources
- **Development**: Debug network-related issues in applications
- **Performance Analysis**: Identify network-intensive processes

## Troubleshooting

### Common Issues

**"Access Denied" or `<Unknown>` Process Names:**
- Run the tool as Administrator for better process name resolution

**No Output or Empty Results:**
- Ensure Windows Firewall is not blocking the application
- Check if you have necessary permissions to query network information

**Build Errors:**
- Ensure you have the latest Rust toolchain installed
- Verify Windows SDK is available for the `windows` crate

## 🤝 Contributing

**We welcome contributions!** This project is actively being developed and we'd love your help.

### 🐛 Reporting Issues
- Use GitHub Issues to report bugs or request features
- Include system information (Windows version, Rust version)
- Provide steps to reproduce any issues
- Screenshots are helpful for UI-related issues

### 📝 Development Contributions
1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** with proper testing
4. **Follow code style**: Run `cargo fmt` and `cargo clippy`
5. **Test thoroughly**: Both dev and release modes
6. **Submit a Pull Request** with a clear description

### 📊 Areas for Contribution
- 🐛 Bug fixes and stability improvements
- 🎨 UI/UX enhancements
- 🚀 Performance optimizations
- 📝 Documentation improvements
- 🌍 Internationalization (i18n)
- ✨ New features from the roadmap

## 🗺️ Project Roadmap

### 🎯 Short Term (v0.2.0)
- 🔧 Complete debugging and logging system
- 🎨 UI polish and performance optimization
- 📊 Export functionality (CSV/JSON)
- 🔍 Advanced search and filtering

### 🎆 Medium Term (v0.3.0)
- 📈 Connection history and analytics
- 🌙 Dark/Light theme support
- 🗺️ Network traffic visualization
- 📧 System tray integration

### 🚀 Long Term (v1.0.0)
- 🌍 Multi-language support
- 🛡️ Security monitoring features
- 📊 Advanced analytics dashboard
- 🔌 Cross-platform support (macOS, Linux)

## 📜 License

This project is open source. Please refer to the license file for details.

## 📅 Changelog

### Version 0.1.0-dev (Current)
- ✨ **NEW**: Modern Tauri 2 + Nuxt 3 architecture
- ✨ **NEW**: Real-time GUI with Vue.js components
- ✨ **NEW**: Auto-refresh with configurable intervals
- ✨ **NEW**: Advanced filtering (protocol, port, process)
- ✨ **NEW**: Anti-jitter UI improvements
- ✨ **NEW**: Responsive design with Tailwind CSS
- ✨ **NEW**: Connection statistics and sorting
- 🔄 **MIGRATED**: From CLI to desktop GUI application
- 🔄 **IMPROVED**: Better process name resolution
- 🔄 **ENHANCED**: Windows API integration

### Legacy Version 1.0.0 (CLI)
- ✅ Initial CLI release with TCP/UDP monitoring
- ✅ Process name resolution
- ✅ Protocol and port filtering
- ✅ Clean table output format

---

**💬 Questions or suggestions?** Feel free to open an issue or start a discussion!

**⭐ Like this project?** Give it a star on GitHub to show your support!

**Built with ❤️ by developers, for developers.**
