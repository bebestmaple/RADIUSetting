# RADIUS 认证客户端 - 架构设计

## 系统架构概览

```
┌─────────────────────────────────────────────────────────────────┐
│                         前端层（Layer 4）                        │
│                    React + TypeScript + Tailwind CSS            │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ 连接面板     │  │ 证书对话框   │  │ 诊断面板     │         │
│  │ Connection   │  │ Certificate  │  │ Diagnostics  │         │
│  │ Panel        │  │ Dialog       │  │ Panel        │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              系统托盘（System Tray）                      │  │
│  └──────────────────────────────────────────────────────────┘  │
└───────────────────────────┬─────────────────────────────────────┘
                            │ Tauri IPC (invoke)
┌───────────────────────────▼─────────────────────────────────────┐
│                   IPC 命令层（commands.rs）                      │
│                                                                  │
│  connect_auth │ disconnect_auth │ get_auth_status │            │
│  refresh_status │ trust_certificate │ list_trusted_certs │     │
│  revoke_certificate_trust │ diagnose_system │ list_interfaces  │
└───────────────────────────┬─────────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────────┐
│                   业务逻辑层（Layer 3）                          │
│                                                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │           认证状态机（auth_manager.rs）                │    │
│  │                                                          │    │
│  │  Disconnected → Connecting → CertificatePrompt         │    │
│  │              ↓              ↓                           │    │
│  │            Failed ←─────→ Connected                     │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │           证书验证（cert_validator.rs）                │    │
│  │  - SHA-256 指纹计算                                     │    │
│  │  - 信任数据库（~/.config/radius-client/）              │    │
│  │  - 过期检查                                             │    │
│  └────────────────────────────────────────────────────────┘    │
└───────────────────────────┬─────────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────────┐
│                   证书管理层（Layer 2）                          │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Windows    │  │    macOS     │  │    Linux     │         │
│  │ cert_store   │  │  keychain    │  │ cert_store   │         │
│  │ CertOpenStore│  │  Security    │  │ /etc/ssl/    │         │
│  │ WinAPI       │  │  Framework   │  │ certs/       │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────────┐
│                   平台抽象层（Layer 1）                          │
│                                                                  │
│  ┌────────────────────────────────────────────────────────┐    │
│  │        Network802_1XManager Trait                       │    │
│  │  - enable_auth()                                        │    │
│  │  - disable_auth()                                       │    │
│  │  - get_status()                                         │    │
│  │  - diagnose()                                           │    │
│  └────────────────────────────────────────────────────────┘    │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Windows    │  │    macOS     │  │    Linux     │         │
│  │ NetworkMgr   │  │ NetworkMgr   │  │ NetworkMgr   │         │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘         │
│         │                  │                  │                 │
│  ┌──────▼───────┐  ┌──────▼───────┐  ┌──────▼───────┐         │
│  │ Service Mgr  │  │ eapolclient  │  │ NetworkMgr   │         │
│  │ netsh        │  │ networksetup │  │ D-Bus API    │         │
│  │ WLAN API     │  │ security     │  │ wpa_supplicant│        │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────────┐
│                      操作系统层                                  │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Windows    │  │    macOS     │  │    Linux     │         │
│  │  XP/7/10/11  │  │   10.12+     │  │ Ubuntu/      │         │
│  │              │  │              │  │ Debian/      │         │
│  │              │  │              │  │ CentOS       │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

---

## 层级职责

### Layer 4: 前端 UI
- **技术栈**：React + TypeScript + Tailwind CSS
- **职责**：
  - 用户交互界面
  - 调用 Tauri 命令
  - 状态展示和错误提示
- **组件**：
  - 连接面板（输入用户名/密码）
  - 证书信任对话框
  - 诊断面板（显示服务状态）
  - 系统托盘菜单

### Layer 3: 业务逻辑
- **模块**：
  - `auth_manager.rs` - 认证状态机
  - `cert_validator.rs` - 证书验证
- **职责**：
  - 状态管理（5 种状态转换）
  - 证书信任流程
  - 业务逻辑协调

### Layer 2: 证书管理
- **模块**：
  - Windows: `cert_store.rs`
  - macOS: `keychain.rs`
  - Linux: `cert_store.rs`
- **职责**：
  - 导入证书到系统存储
  - 查询证书状态
  - 删除证书

### Layer 1: 平台抽象
- **模块**：
  - Windows: `WindowsNetworkManager`
  - macOS: `MacOSNetworkManager`
  - Linux: `LinuxNetworkManager`
- **职责**：
  - 统一的 802.1X 配置接口
  - 平台特定实现
  - 服务管理和诊断

---

## 数据流

### 连接流程

```
用户点击"连接"
    ↓
前端调用 invoke('connect_auth', ...)
    ↓
commands::connect_auth()
    ↓
auth_manager.enable_auth()
    ↓
    ├─→ set_state(Connecting)
    ├─→ platform_manager.enable_auth()
    │       ↓
    │   Windows: service_manager.ensure_service_running("dot3svc")
    │   Windows: netsh.configure_peap_via_netsh()
    │   macOS: eapolclient.configure_peap()
    │   Linux: networkmanager.configure_via_nm()
    │       ↓
    ├─→ platform_manager.get_status()
    └─→ set_state(Connected/Failed)
    ↓
返回 AuthState 给前端
    ↓
前端更新 UI
```

### 证书信任流程

```
RADIUS 服务器返回证书
    ↓
cert_validator.is_certificate_trusted()
    ↓
   NO → auth_manager.handle_certificate_trust()
        ↓
        set_state(CertificatePrompt)
        ↓
        返回状态给前端
        ↓
        前端显示证书对话框
        ↓
        用户点击"信任"
        ↓
        前端调用 invoke('trust_certificate', ...)
        ↓
        cert_validator.trust_certificate()
        ├─→ 保存到信任数据库
        └─→ Windows: cert_store.import_ca_certificate()
            macOS: keychain.import_certificate()
            Linux: cert_store.import_ca_certificate()
        ↓
        auth_manager.trust_and_continue()
        ↓
        重新连接
```

---

## 关键设计模式

### 1. 策略模式（平台抽象）

```rust
trait Network802_1XManager {
    fn enable_auth(&self, interface: &str, creds: &Credentials) -> Result<()>;
    // ...
}

impl Network802_1XManager for WindowsNetworkManager { /* ... */ }
impl Network802_1XManager for MacOSNetworkManager { /* ... */ }
impl Network802_1XManager for LinuxNetworkManager { /* ... */ }
```

**优势**：
- 统一接口，隔离平台差异
- 易于扩展新平台
- 条件编译自动选择实现

### 2. 状态机模式（认证管理）

```
Disconnected ──enable_auth──→ Connecting
                                  │
                    ┌─────────────┴─────────────┐
                    │                           │
            证书未信任                       证书已信任
                    │                           │
                    ↓                           ↓
          CertificatePrompt              查询状态
                    │                           │
            用户信任证书                        │
                    │                           │
                    └─────────────┬─────────────┘
                                  ↓
                    ┌─────────────┴─────────────┐
                    │                           │
                成功                          失败
                    │                           │
                    ↓                           ↓
              Connected                      Failed
```

### 3. 命令模式（Tauri IPC）

```rust
#[tauri::command]
pub async fn connect_auth(
    interface: String,
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<AuthState, String>
```

**优势**：
- 前端通过字符串调用命令
- 统一的错误处理
- 类型安全（Serde 自动序列化）

---

## 线程安全

### 全局状态管理

```rust
pub struct AppState {
    pub auth_manager: Mutex<AuthManager>,
}

pub struct AuthManager {
    state: Arc<Mutex<AuthState>>,
    // ...
}
```

- **外层 Mutex**：保护整个 AuthManager（Tauri 要求）
- **内层 Arc<Mutex>**：保护 AuthState（多线程访问）

---

## 错误处理策略

### 层级传播

```
平台层错误（anyhow::Error）
    ↓
业务逻辑层（anyhow::Error）
    ↓
命令层（Result<T, String>）
    ↓
前端（JavaScript Error）
```

### 错误转换

```rust
manager.enable_auth(&interface, &username, &password)
    .map_err(|e| e.to_string())?;
```

**原因**：
- Tauri 只能传输可序列化的类型
- String 是最简单的跨语言错误表示

---

## 性能考虑

### 1. 异步命令

所有 Tauri 命令使用 `async fn`，避免阻塞主线程。

### 2. 轮询间隔

建议前端每 3-5 秒调用 `refresh_status`，平衡实时性和性能。

### 3. 状态缓存

`get_auth_status` 返回缓存状态，不触发网络操作。

---

## 安全考虑

### 1. 密码不持久化

密码仅在内存中传递，不写入配置文件。

### 2. 证书验证

首次连接时强制用户确认证书，防止中间人攻击。

### 3. 最小权限

- Windows: 仅在必要时请求 UAC 提升
- macOS: 使用 SMJobBless（推荐）或 sudo 包装器
- Linux: 优先使用用户目录证书存储

---

## 平台差异处理

### Windows

- **遗留支持**：独立构建 XP/7 版本（使用 netsh）
- **服务管理**：自动启动 `dot3svc` 和 `EapHost`
- **证书存储**：使用 WinAPI 的 CertOpenStore

### macOS

- **权限**：首次运行需要管理员密码
- **命令工具**：使用 `eapolclient` 和 `security`
- **证书**：导入到系统 Keychain

### Linux

- **发行版兼容**：支持 Debian/Ubuntu/CentOS
- **网络管理器**：优先 NetworkManager，降级到 wpa_supplicant
- **证书路径**：系统级（需 root）或用户级（无需 root）

---

## 扩展性

### 添加新认证方法

1. 在 `platform/mod.rs` 的 Trait 中添加方法
2. 在各平台的实现中添加对应逻辑
3. 在 `auth_manager.rs` 中集成
4. 在 `commands.rs` 中导出命令

### 添加新平台

1. 创建 `platform/{platform}/` 目录
2. 实现 `Network802_1XManager` trait
3. 在 `platform/mod.rs` 中添加条件编译
4. 更新 CI/CD 配置

---

## 依赖关系

```
前端 (React)
    │
    └─→ Tauri Runtime
            │
            └─→ commands.rs
                    │
                    ├─→ auth_manager.rs
                    │       └─→ cert_validator.rs
                    │       └─→ platform_manager
                    │               ├─→ Windows
                    │               ├─→ macOS
                    │               └─→ Linux
                    │
                    └─→ cert_validator.rs
                            └─→ sha2, chrono, dirs
```

---

## 配置文件

### config.json

```json
{
  "version": "0.1.0",
  "authentication": {
    "protocol": "PEAP",
    "inner_method": "MSCHAPv2",
    "anonymous_identity": "anonymous"
  },
  "logging": {
    "level": "info",
    "file_path": "./logs/radius-client.log",
    "max_size_mb": 10
  }
}
```

### 信任数据库

```json
// ~/.config/radius-client/trusted_certs.json
{
  "trusted_certs": [
    {
      "fingerprint": "AA:BB:CC:DD:...",
      "trusted": true,
      "trusted_at": "2026-06-15T10:00:00Z",
      "expires_at": "2027-12-31T23:59:59Z"
    }
  ]
}
```

---

## 测试策略

### 单元测试

- `cert_validator.rs` - 指纹计算、信任逻辑
- `auth_manager.rs` - 状态转换、线程安全

### 集成测试

- Mock RADIUS 服务器 + FreeRADIUS
- 测试完整连接流程

### 平台测试

- VM 农场（Windows XP/7/10/11, macOS 12+, Ubuntu/CentOS）
- 真实 RADIUS 服务器环境

---

## 部署架构

### 分发格式

- **Windows**: `.exe` (portable) + `.msi` (安装包)
- **macOS**: `.zip` (portable) + `.dmg` (镜像)
- **Linux**: `.AppImage` (portable) + `.deb`/`.rpm` (安装包)

### CI/CD

GitHub Actions 自动构建所有平台：

```
git tag v0.1.0
    ↓
GitHub Actions 触发
    ↓
并行构建 Windows/macOS/Linux
    ↓
上传到 GitHub Releases
```
