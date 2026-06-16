# RADIUS 认证客户端

跨平台的 802.1X PEAP 认证客户端，支持 Windows、macOS 和 Linux。

[![构建状态](https://img.shields.io/github/actions/workflow/status/your-repo/radius-client/build.yml?branch=main)](https://github.com/your-repo/radius-client/actions)
[![许可证](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## ✨ 功能特性

### 核心功能
- ✅ **一键认证** - 输入用户名/密码即可连接
- ✅ **证书管理** - 首次信任 RADIUS 证书，后续自动连接
- ✅ **系统诊断** - 自动检查必要的服务和依赖
- ✅ **状态监控** - 实时显示连接状态和 IP 地址
- ✅ **多平台支持** - Windows XP/7/10/11、macOS 10.12+、Ubuntu/Debian/CentOS

### 平台特性

#### Windows
- 自动启动必要服务（`Wired AutoConfig`、`EapHost`）
- 支持 Windows XP/7（netsh）和 Windows 8+（Native WiFi API）
- 证书导入到系统证书存储

#### macOS
- 使用 eapolclient 和 Security.framework
- 证书导入到系统 Keychain
- 支持 macOS 10.12+

#### Linux
- 优先使用 NetworkManager，降级到 wpa_supplicant
- 支持 Ubuntu/Debian/CentOS
- 证书可导入系统存储（需 root）或用户目录（无需 root）
- 额外提供终端脚本（无 GUI 环境）

---

## 🚀 快速开始

### Windows

#### 下载并运行（推荐）
1. 从 [Releases](https://github.com/your-repo/radius-client/releases) 下载 `radius-client-win64.exe`
2. 双击运行（无需安装）
3. 输入用户名和密码，点击"连接"

#### 首次运行提示
如果提示"Wired AutoConfig 服务未运行"，点击"自动启动服务"按钮。

---

### macOS

#### 下载并运行
1. 从 [Releases](https://github.com/your-repo/radius-client/releases) 下载 `radius-client-macos.zip`
2. 解压得到 `RADIUS Client.app`
3. 首次运行：右键点击 → 打开 → 确认
4. 输入管理员密码（用于配置网络）

---

### Linux（桌面版）

#### AppImage（推荐，所有发行版通用）
```bash
# 1. 下载
wget https://github.com/your-repo/radius-client/releases/download/v0.1.0/radius-client-x86_64.AppImage

# 2. 赋予执行权限
chmod +x radius-client-x86_64.AppImage

# 3. 运行
./radius-client-x86_64.AppImage
```

#### .deb 包（Ubuntu/Debian）
```bash
# 1. 下载
wget https://github.com/your-repo/radius-client/releases/download/v0.1.0/radius-client_0.1.0_amd64.deb

# 2. 安装
sudo dpkg -i radius-client_0.1.0_amd64.deb

# 3. 运行
radius-client
```

#### .rpm 包（CentOS/RHEL）
```bash
# 1. 下载
wget https://github.com/your-repo/radius-client/releases/download/v0.1.0/radius-client-0.1.0.x86_64.rpm

# 2. 安装
sudo rpm -i radius-client-0.1.0.x86_64.rpm

# 3. 运行
radius-client
```

---

### Linux（终端版，无 GUI）

适用于服务器或无桌面环境的 Linux 系统。

```bash
# 1. 下载脚本
wget https://github.com/your-repo/radius-client/releases/download/v0.1.0/radius-setup.sh

# 2. 赋予执行权限
chmod +x radius-setup.sh

# 3. 运行
sudo ./radius-setup.sh
```

**交互式菜单**：
```
=========================================
 RADIUS 认证客户端 - Linux 终端版
=========================================
1. 启用 802.1X 认证
2. 禁用 802.1X 认证
3. 查看状态
4. 系统诊断
5. 列出网络接口
0. 退出
=========================================
```

---

## 📖 使用指南

### 基本操作

#### 1. 连接认证
1. 选择网络接口（如 `eth0`、`en0`、`wlan0`）
2. 输入用户名（通常是邮箱地址）
3. 输入密码
4. 点击"连接"

#### 2. 首次连接（证书信任）
首次连接时，会显示 RADIUS 服务器证书：

```
证书信息
━━━━━━━━━━━━━━━━━━━━━━
颁发者：CN=Example RADIUS CA
主题：  CN=radius.example.com
指纹：  AA:BB:CC:DD:EE:...
有效期：2025-01-01 至 2027-12-31

[ 信任并继续 ]  [ 取消 ]
```

点击"信任并继续"后，证书会保存到系统，后续自动连接。

#### 3. 查看状态
连接成功后，主界面显示：
```
✅ 已连接
接口：eth0
IP：  192.168.1.100
时间：2026-06-15 10:30:00
```

#### 4. 断开连接
点击"断开"按钮即可禁用认证。

---

### 系统诊断

点击"诊断"按钮，自动检查：

#### Windows
- ✅ Wired AutoConfig 服务（有线 802.1X）
- ✅ EapHost 服务（EAP 认证）
- ✅ 网络适配器状态

**修复建议**：如果服务未运行，点击"自动启动服务"按钮。

#### macOS
- ✅ eapolclient 进程
- ✅ 网络接口可用性

#### Linux
- ✅ NetworkManager 服务
- ✅ wpa_supplicant 包
- ✅ 网络接口可用性

**修复建议**：
```bash
# Ubuntu/Debian
sudo apt install network-manager wpasupplicant

# CentOS/RHEL
sudo yum install NetworkManager wpa_supplicant
```

---

### 证书管理

#### 查看已信任证书
点击"证书管理"查看所有已信任的 RADIUS 证书：
```
已信任证书列表
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
radius.example.com
  指纹：AA:BB:CC:DD:...
  信任时间：2026-06-15 10:00:00
  过期时间：2027-12-31 23:59:59

[ 撤销信任 ]
```

#### 撤销信任
选择证书 → 点击"撤销信任" → 下次连接时会重新提示。

---

## 🛠️ 故障排查

### Windows

#### 问题 1：服务未运行
**错误**：`无法打开服务: dot3svc`

**解决方案**：
1. 运行诊断 → 点击"自动启动服务"
2. 或手动启动：
   ```cmd
   sc start dot3svc
   sc start EapHost
   ```

#### 问题 2：权限不足
**错误**：`拒绝访问`

**解决方案**：右键点击程序 → "以管理员身份运行"

---

### macOS

#### 问题 1：无法打开应用
**错误**：`"RADIUS Client"已损坏，无法打开`

**解决方案**：
```bash
# 移除隔离属性
xattr -cr "/Applications/RADIUS Client.app"
```

#### 问题 2：需要管理员密码
这是正常现象。配置网络需要 sudo 权限。

---

### Linux

#### 问题 1：NetworkManager 未运行
**错误**：`NetworkManager 配置失败`

**解决方案**：
```bash
# 启动服务
sudo systemctl start NetworkManager
sudo systemctl enable NetworkManager
```

#### 问题 2：wpa_supplicant 缺失
**错误**：`降级到 wpa_supplicant 失败`

**解决方案**：
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install wpasupplicant

# CentOS/RHEL
sudo yum install wpa_supplicant
```

#### 问题 3：权限不足
某些操作需要 root 权限：
```bash
sudo radius-client
# 或
sudo ./radius-setup.sh
```

---

## 📚 文档

- [API 文档](docs/API.md) - Tauri 命令接口详解
- [架构设计](docs/ARCHITECTURE.md) - 系统架构和设计模式
- [构建指南](docs/BUILD.md) - 本地构建和交叉编译

---

## 🏗️ 本地构建

### 前置条件

#### 所有平台
- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (18+)

#### Windows
- Visual Studio Build Tools

#### macOS
- Xcode Command Line Tools: `xcode-select --install`

#### Linux
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# CentOS/RHEL
sudo dnf install gcc make webkit2gtk3-devel openssl-devel gtk3-devel
```

### 构建步骤

```bash
# 1. 克隆仓库
git clone https://github.com/your-repo/radius-client.git
cd radius-client

# 2. 安装前端依赖
npm install

# 3. 构建应用
npm run tauri build

# 输出位置：
# Windows: src-tauri\target\release\radius-client.exe
# macOS:   src-tauri/target/release/bundle/dmg/
# Linux:   src-tauri/target/release/bundle/deb/ 或 appimage/
```

---

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

### 报告问题
请在 [Issues](https://github.com/your-repo/radius-client/issues) 中描述：
- 操作系统和版本
- 问题描述和复现步骤
- 错误日志（位于 `~/.config/radius-client/logs/`）

### 提交代码
1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/your-feature`
3. 提交更改：`git commit -m "Add: your feature"`
4. 推送分支：`git push origin feature/your-feature`
5. 创建 Pull Request

---

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。

---

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [React](https://reactjs.org/) - 前端框架
- [FreeRADIUS](https://freeradius.org/) - 开源 RADIUS 服务器

---

## 📞 联系方式

- 问题反馈：[GitHub Issues](https://github.com/your-repo/radius-client/issues)
- 讨论：[GitHub Discussions](https://github.com/your-repo/radius-client/discussions)
- 邮箱：support@example.com

---

## 🗺️ 路线图

### v0.1.0（当前版本）
- ✅ 基本 PEAP 认证
- ✅ 证书信任管理
- ✅ 系统诊断
- ✅ Windows/macOS/Linux 支持

### v0.2.0（计划中）
- ⏳ 多配置文件支持
- ⏳ 自动重连
- ⏳ 日志查看器
- ⏳ 多语言支持

### v0.3.0（计划中）
- ⏳ 企业策略部署（GPO/MDM）
- ⏳ 其他 EAP 方法（TLS、TTLS）
- ⏳ 批量部署工具

---

**开始使用 RADIUS 认证客户端，一键连接企业网络！**
