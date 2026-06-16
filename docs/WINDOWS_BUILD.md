# RADIUS 认证客户端 - Windows 构建指南

## 问题：缺少 link.exe

**错误**：`error: linker 'link.exe' not found`

**原因**：Windows 上编译 Rust 需要 Microsoft C++ 构建工具。

---

## 解决方案

### 方法 1：安装 Visual Studio Build Tools（推荐）

#### 步骤 1：下载 Build Tools
访问：https://visualstudio.microsoft.com/downloads/
- 展开 "Tools for Visual Studio"
- 下载 "Build Tools for Visual Studio 2022"

#### 步骤 2：安装必要组件
运行安装程序，选择以下工作负载：
- ✅ **"Desktop development with C++"（使用 C++ 的桌面开发）**

必须包含的组件：
- ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools
- ✅ Windows 11 SDK (或 Windows 10 SDK)
- ✅ C++ CMake tools for Windows

#### 步骤 3：重启 PowerShell
```powershell
# 关闭当前 PowerShell 窗口，重新打开
# 验证安装
where.exe link.exe
```

预期输出：
```
C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.XX.XXXXX\bin\Hostx64\x64\link.exe
```

---

### 方法 2：安装完整的 Visual Studio（可选）

如果您需要 IDE：
1. 下载 Visual Studio Community 2022（免费）
2. 安装时选择 "Desktop development with C++"
3. 重启 PowerShell

---

## 完整的 Windows 构建环境设置

### 1. 安装 Rust（已完成）
```powershell
# 验证
rustc --version
cargo --version
```

### 2. 安装 Node.js（已完成）
```powershell
# 验证
node --version
npm --version
```

### 3. 安装 Visual Studio Build Tools（需要）
按照上述"方法 1"操作。

### 4. 安装 WebView2（Tauri 依赖）
Windows 10/11 通常已预装。如果没有：
- 下载：https://developer.microsoft.com/microsoft-edge/webview2/
- 安装 "Evergreen Standalone Installer"

---

## 安装完成后的构建流程

### 1. 清理之前的构建
```powershell
cd C:\Users\Administrator\Desktop\RADIUSetting
Remove-Item -Recurse -Force .\src-tauri\target -ErrorAction SilentlyContinue
```

### 2. 安装依赖
```powershell
# 安装前端依赖
npm install

# 更新 Rust 依赖
cd src-tauri
cargo fetch
```

### 3. 运行测试
```powershell
cargo test
```

预期输出：
```
running 10 tests
test core::auth_manager::tests::test_state_clone ... ok
test core::auth_manager::tests::test_state_transitions ... ok
test core::auth_manager::tests::test_thread_safety ... ok
test core::cert_validator::tests::test_fingerprint_calculation ... ok
test core::cert_validator::tests::test_fingerprint_consistency ... ok
test core::cert_validator::tests::test_fingerprint_different_data ... ok
test core::cert_validator::tests::test_trust_workflow ... ok
test core::cert_validator::tests::test_certificate_expiry ... ok
test core::cert_validator::tests::test_certificate_not_expired ... ok
test core::cert_validator::tests::test_parse_certificate_basic ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured
```

### 4. 编译检查
```powershell
cargo check
```

### 5. 构建 Portable 包
```powershell
cd ..
npm run tauri build
```

构建产物位置：
- **Portable .exe**：`src-tauri\target\release\radius-client.exe`
- **MSI 安装包**：`src-tauri\target\release\bundle\msi\radius-client_0.1.0_x64.msi`

---

## 验证构建结果

### 检查文件大小
```powershell
Get-Item .\src-tauri\target\release\radius-client.exe | Select-Object Name, Length
```

预期大小：约 5-10 MB

### 测试运行
```powershell
.\src-tauri\target\release\radius-client.exe
```

---

## 故障排查

### 问题 1：找不到 link.exe
```
error: linker `link.exe` not found
```
**解决**：安装 Visual Studio Build Tools（见上文）

### 问题 2：找不到 Windows SDK
```
error: failed to run custom build command for `windows-sys`
```
**解决**：在 Build Tools 安装程序中添加 Windows SDK

### 问题 3：WebView2 缺失
```
error: WebView2 runtime not found
```
**解决**：下载安装 WebView2 Runtime

### 问题 4：权限不足
```
error: Permission denied
```
**解决**：以管理员身份运行 PowerShell

---

## 加速后续构建

### 使用 sccache（可选）
```powershell
# 安装
cargo install sccache

# 配置
$env:RUSTC_WRAPPER = "sccache"
```

### 使用增量编译
```powershell
$env:CARGO_INCREMENTAL = 1
```

---

## 预估时间

| 步骤 | 首次 | 后续 |
|------|------|------|
| 安装 Build Tools | 20 分钟 | - |
| 编译依赖 | 10-15 分钟 | 1 分钟 |
| 运行测试 | 2 分钟 | 30 秒 |
| 构建 Release | 5-10 分钟 | 2 分钟 |

---

## 快速命令清单

```powershell
# 安装 Build Tools 后执行：

# 1. 进入项目目录
cd C:\Users\Administrator\Desktop\RADIUSetting

# 2. 安装依赖
npm install

# 3. 运行测试
cd src-tauri
cargo test

# 4. 构建 Portable 包
cd ..
npm run tauri build

# 5. 查看结果
Get-ChildItem .\src-tauri\target\release\*.exe
```

---

## 构建成功标志

```
    Finished `release` profile [optimized] target(s) in 8m 32s
     Running `'C:\Users\...\tauri-bundler.exe' '...'`
    Bundling radius-client.exe (C:\Users\...\target\release\radius-client.exe)
    Finished 1 bundle at:
        src-tauri\target\release\bundle\msi\radius-client_0.1.0_x64.msi
        src-tauri\target\release\radius-client.exe
```

---

**下一步**：安装 Visual Studio Build Tools，然后重新运行构建命令。
