# 构建与打包指南

## 自动化构建（推荐）

### GitHub Actions

项目已配置自动化构建管道（`.github/workflows/build.yml`）。

**触发构建**：
```bash
# 方式 1：推送 tag
git tag v0.1.0
git push origin v0.1.0

# 方式 2：手动触发（GitHub 网页端）
# Actions → 构建多平台包 → Run workflow
```

**产出物**：
- Windows: `radius-client.exe`
- Ubuntu: `.deb` 包 + `.AppImage`
- macOS: `.dmg` 镜像

---

## 本地构建

### Windows

**前置条件**：
- Rust（[rustup.rs](https://rustup.rs/)）
- Node.js 18+
- Visual Studio Build Tools

**构建命令**：
```powershell
npm install
npm run tauri build
```

**输出位置**：
- `src-tauri\target\release\radius-client.exe`

---

### Linux (Ubuntu/Debian)

**前置条件**：
```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js (通过 nvm 或包管理器)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
```

**构建命令**：
```bash
npm install
npm run tauri build
```

**输出位置**：
- `.deb`: `src-tauri/target/release/bundle/deb/radius-client_*.deb`
- AppImage: `src-tauri/target/release/bundle/appimage/radius-client_*.AppImage`

---

### macOS

**前置条件**：
- Xcode Command Line Tools: `xcode-select --install`
- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js 18+ (通过 Homebrew: `brew install node`)

**构建命令**：
```bash
npm install
npm run tauri build
```

**输出位置**：
- `src-tauri/target/release/bundle/dmg/RADIUS Client_*.dmg`

---

## Docker 构建（Linux）

### 构建 Ubuntu .deb 包

```bash
docker run --rm -v ${PWD}:/workspace -w /workspace ubuntu:22.04 bash -c "
  apt update && 
  apt install -y curl build-essential libwebkit2gtk-4.0-dev \
    libgtk-3-dev libssl-dev librsvg2-dev wget nodejs npm && 
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && 
  source \$HOME/.cargo/env && 
  npm install && 
  npm run tauri build
"
```

**输出**：容器内 `src-tauri/target/release/bundle/deb/` 目录（映射到宿主机）

### 构建 CentOS .rpm 包

```bash
docker run --rm -v ${PWD}:/workspace -w /workspace rockylinux:8 bash -c "
  dnf install -y gcc make wget gtk3-devel webkit2gtk3-devel openssl-devel && 
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && 
  source \$HOME/.cargo/env && 
  curl -fsSL https://rpm.nodesource.com/setup_18.x | bash - && 
  dnf install -y nodejs && 
  npm install && 
  npm run tauri build
"
```

---

## 交叉编译（高级）

### 从 Windows 交叉编译到 Linux

**⚠️ 不推荐**：Tauri 依赖目标平台的系统库，交叉编译配置复杂且容易出错。

如果必须尝试：
1. 安装 `cross`：`cargo install cross`
2. 配置 Docker（WSL2 后端）
3. 修改 `Cargo.toml` 添加目标平台
4. 运行：`cross build --target x86_64-unknown-linux-gnu --release`

**限制**：
- 只能编译 Rust 代码，无法打包 `.deb` / `.rpm`
- 无法处理 WebView 等系统依赖
- 需要手动解决动态链接库问题

**结论**：使用 GitHub Actions 或 Docker 更可靠。

---

## 分发建议

### Windows
- 提供 `.exe` 可执行文件（绿色版）
- 可选：使用 Inno Setup / WiX 制作安装包

### Linux
- **Ubuntu/Debian**: `.deb` 包（双击安装）+ `.AppImage`（免安装）
- **CentOS/RHEL**: `.rpm` 包 + `.AppImage`
- **通用**: `scripts/linux/radius-setup.sh` 终端脚本

### macOS
- `.dmg` 磁盘镜像（拖拽安装）
- 需要代码签名和公证（Apple Developer Program）

---

## 故障排查

### 编译错误

**问题**：`webkit2gtk` 找不到
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-dev

# CentOS/RHEL
sudo dnf install webkit2gtk3-devel
```

**问题**：Rust 版本过旧
```bash
rustup update stable
```

### 运行错误

**Linux**：`error while loading shared libraries`
```bash
# 安装运行时依赖
sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0
```

**macOS**：应用无法打开（未签名）
```bash
# 临时绕过 Gatekeeper
xattr -cr "RADIUS Client.app"
```
