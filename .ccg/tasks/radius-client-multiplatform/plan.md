# 实施计划 - RADIUS 认证客户端

## 计划概览

**技术栈选择**：Tauri（生产推荐）+ Electron（快速备选）  
**开发策略**：MVP 优先，逐步迭代  
**预估工期**：8-12 周（取决于团队规模和技术栈选择）

---

## 架构决策

### 最终技术栈（推荐）

**核心选择：Tauri + Rust**
- **前端**：React + TypeScript + Tailwind CSS
- **后端**：Rust + Tauri
- **平台层**：
  - Windows：`winapi` crate
  - macOS：Swift FFI + `eapolclient` CLI
  - Linux：`dbus-rs` + NetworkManager

**理由**：
1. 极小包体积（~15MB vs Electron 120MB）
2. 低内存占用（关键，大规模部署）
3. Rust 安全保证（网络认证是安全关键应用）
4. 良好的 WinXP/7 支持（通过旧版 SDK）

### 遗留系统策略

**Windows XP/7 处理**：
- 构建两个版本：
  - `radius-client-legacy.exe`（XP/7，使用 SDK 7.1A + `netsh`）
  - `radius-client.exe`（Win8+，使用最新 API）
- 启动器自动检测 OS 版本并调用对应二进制

---

## 实施阶段

### Layer 0: 项目基础（Week 1）

**目标**：搭建开发环境和项目骨架

#### 任务 0.1：项目初始化
- 文件：`Cargo.toml`, `package.json`, `tauri.conf.json`
- 内容：
  - Rust workspace 配置
  - Tauri 依赖（v1.x 稳定版）
  - 前端依赖（React 18, TypeScript 5, Tailwind CSS 3）
- 验证：`cargo build` 和 `npm run dev` 成功

#### 任务 0.2：目录结构
- 创建模块目录：
  ```
  src-tauri/
  ├── src/
  │   ├── main.rs
  │   ├── core/           # 业务逻辑
  │   ├── platform/       # 平台抽象
  │   │   ├── windows/
  │   │   ├── macos/
  │   │   └── linux/
  │   └── diagnostics/    # 诊断模块
  src/                    # 前端
  ├── components/
  ├── pages/
  └── utils/
  ```

#### 任务 0.3：配置文件与日志
- 文件：`src-tauri/src/core/config_loader.rs`
- 实现 JSON 配置加载（带 schema 验证）
- 集成日志库（`tracing` + 文件输出）

**验收标准**：
- ✅ 项目可编译运行
- ✅ 配置文件加载正常
- ✅ 日志输出到文件

---

### Layer 1: 平台抽象层（Week 2-3）

**目标**：实现跨平台网络配置接口

#### 任务 1.1：定义平台抽象 Trait
- 文件：`src-tauri/src/platform/mod.rs`
- 内容：
  ```rust
  pub trait Network802_1XManager {
      fn enable_auth(&self, creds: &Credentials) -> Result<()>;
      fn disable_auth(&self) -> Result<()>;
      fn get_status(&self) -> Result<AuthStatus>;
      fn diagnose(&self) -> Result<DiagResult>;
  }
  ```

#### 任务 1.2：Windows 实现
- 文件：
  - `src-tauri/src/platform/windows/mod.rs`
  - `src-tauri/src/platform/windows/netsh.rs`（XP/7 备用）
  - `src-tauri/src/platform/windows/wlanapi.rs`（Win8+ 主要）
  - `src-tauri/src/platform/windows/service_manager.rs`（服务管理）
- 依赖：`winapi = { version = "0.3", features = ["wlanapi", "wincrypt", "winsvc"] }`
- 实现：
  - **自动启动必要服务**（关键）：
    - 有线网络：`Wired AutoConfig` (dot3svc)
    - 无线网络：`WLAN AutoConfig` (WlanSvc)
    - EAP 认证：`Extensible Authentication Protocol` (EapHost)
  - 服务检查与启动逻辑：
    ```rust
    // 检查服务状态
    fn ensure_service_running(service_name: &str) -> Result<()> {
        let sc_manager = OpenSCManager(GENERIC_READ)?;
        let service = OpenService(sc_manager, service_name, SERVICE_START | SERVICE_QUERY_STATUS)?;
        
        let mut status = SERVICE_STATUS::default();
        QueryServiceStatus(service, &mut status)?;
        
        if status.dwCurrentState != SERVICE_RUNNING {
            StartService(service, &[])?;
            // 等待启动完成
            wait_for_service_start(service)?;
        }
        Ok(())
    }
    ```
  - 运行时检测 OS 版本
  - Win8+: Native WiFi API 配置 PEAP
  - XP/7: `netsh lan set eaphost` 命令

#### 任务 1.3：macOS 实现
- 文件：
  - `src-tauri/src/platform/macos/mod.rs`
  - `src-tauri/src/platform/macos/eapolclient.rs`
- 实现：
  - 执行 `eapolclient` CLI 配置 802.1X
  - 调用 `networksetup` 检查状态

#### 任务 1.4：Linux 实现
- 文件：
  - `src-tauri/src/platform/linux/mod.rs`
  - `src-tauri/src/platform/linux/networkmanager.rs`
- 依赖：`dbus = "0.9"`
- 实现：
  - D-Bus 调用 NetworkManager API
  - 生成 802.1X 连接配置（PEAP + MSCHAPv2）

**验收标准**：
- ✅ 所有平台可编译通过（条件编译）
- ✅ Windows 单元测试通过（mock 环境）

---

### Layer 2: 证书管理与特权提升（Week 4）

**目标**：实现证书信任流程和特权操作

#### 任务 2.1：证书验证与存储
- 文件：`src-tauri/src/core/cert_validator.rs`
- 实现：
  - 证书指纹计算（SHA-256）
  - 信任数据库（`~/.config/radius-client/trusted_certs.json`）
  - 首次连接检测逻辑

#### 任务 2.2：Windows 证书导入
- 文件：`src-tauri/src/platform/windows/cert_store.rs`
- 实现：
  - `CertOpenStore` 打开系统证书存储
  - `CertAddEncodedCertificateToStore` 导入 CA 证书
  - UAC 提升检测（`IsUserAnAdmin`）

#### 任务 2.3：macOS Keychain 集成
- 文件：`src-tauri/src/platform/macos/keychain.rs`
- 实现：
  - Swift 辅助代码调用 Security.framework
  - `SecItemAdd` 导入证书
  - `sudo` 提示包装器

#### 任务 2.4：Linux 证书存储
- 文件：`src-tauri/src/platform/linux/cert_store.rs`
- 实现：
  - 写入 `/etc/ssl/certs/` 或 `~/.config/radius-client/certs/`
  - PolicyKit 策略文件（`org.example.radius-client.policy`）

**验收标准**：
- ✅ 证书首次提示流程完整
- ✅ 已信任证书不重复提示
- ✅ 特权提升成功（测试环境）

---

### Layer 3: 业务逻辑层（Week 5）

**目标**：实现认证状态机和核心逻辑

#### 任务 3.1：认证状态机
- 文件：`src-tauri/src/core/auth_manager.rs`
- 状态定义：
  ```rust
  enum AuthState {
      Disconnected,
      Connecting,
      CertificatePrompt(CertInfo),
      Connected { interface: String, ip: String },
      Failed(ErrorKind),
  }
  ```
- 实现状态转换逻辑

#### 任务 3.2：Tauri 命令（IPC 接口）
- 文件：`src-tauri/src/commands.rs`
- 导出命令：
  ```rust
  #[tauri::command]
  async fn connect_auth(username: String, password: String) -> Result<()>;
  
  #[tauri::command]
  async fn disconnect_auth() -> Result<()>;
  
  #[tauri::command]
  async fn get_status() -> Result<AuthStatus>;
  
  #[tauri::command]
  async fn trust_certificate(fingerprint: String) -> Result<()>;
  ```

#### 任务 3.3：诊断逻辑
- 文件：`src-tauri/src/diagnostics/mod.rs`
- 实现：
  - Windows: 检查服务状态（**关键服务必须运行**）
    - `Wired AutoConfig` (dot3svc) - **有线 802.1X 必需**
    - `WLAN AutoConfig` (WlanSvc) - 无线 802.1X 必需
    - `Extensible Authentication Protocol` (EapHost) - EAP 认证必需
    - 诊断输出示例：
      ```
      🔴 Wired AutoConfig 服务  已停止
         → 问题：有线 802.1X 认证无法工作
         → 修复：[自动启动服务] 按钮
      ```
  - macOS: 检查 `eapolclient` 进程
  - Linux: 检查 NetworkManager 服务和包

**验收标准**：
- ✅ 前端可调用所有 Tauri 命令
- ✅ 状态机正确转换
- ✅ 诊断返回正确结果

---

### Layer 4: 前端 UI（Week 6-7）

**目标**：实现用户界面和交互流程

#### 任务 4.1：主窗口组件
- 文件：
  - `src/App.tsx`（主入口）
  - `src/components/ConnectionPanel.tsx`（连接面板）
  - `src/components/StatusDisplay.tsx`（状态显示）
- 实现：
  - 用户名/密码输入表单
  - 连接/断开按钮
  - 实时状态显示（WebSocket 或轮询）

#### 任务 4.2：证书信任对话框
- 文件：`src/components/CertificateTrustDialog.tsx`
- 实现：
  - 显示证书详情（颁发者、有效期、指纹）
  - "信任" / "取消" 按钮
  - 调用 `trust_certificate` 命令

#### 任务 4.3：诊断面板
- 文件：`src/components/DiagnosticsPanel.tsx`
- 实现：
  - 服务/包状态列表（🟢🟡🔴 指示灯）
  - "重新检测" 按钮
  - 修复建议提示

#### 任务 4.4：系统托盘集成
- 文件：`src-tauri/src/tray.rs`
- 实现：
  - 托盘图标（根据状态变化）
  - 右键菜单（连接/断开/诊断/退出）
  - 最小化到托盘

**验收标准**：
- ✅ UI 完整可交互
- ✅ Tauri 命令正确调用
- ✅ 状态实时更新

---

### Layer 5: 测试与优化（Week 8-9）

**目标**：确保质量和性能

#### 任务 5.1：单元测试
- 覆盖率目标：>70%
- 测试文件：
  - `src-tauri/src/core/auth_manager_test.rs`
  - `src-tauri/src/platform/windows/netsh_test.rs`
- Mock 外部依赖（系统 API、网络调用）

#### 任务 5.2：集成测试
- 测试环境：
  - Windows 10 VM + Mock RADIUS 服务器
  - Ubuntu 22.04 VM + FreeRADIUS
- 测试场景：
  - 首次连接 → 证书提示 → 信任 → 连接成功
  - 重新连接 → 自动使用已信任证书
  - 证书过期 → 警告提示

#### 任务 5.3：性能优化
- 内存占用 < 50MB（Tauri 应用）
- 启动时间 < 2 秒
- 认证触发响应 < 3 秒

#### 任务 5.4：错误处理增强
- 所有 `Result` 类型正确传播
- 用户友好的错误消息
- 日志记录关键操作

**验收标准**：
- ✅ 单元测试通过率 100%
- ✅ 集成测试覆盖核心流程
- ✅ 性能指标达标

---

### Layer 6: 打包与分发（Week 10-11）

**目标**：构建 9 个平台的可分发包

#### 任务 6.1：CI/CD 管道搭建
- 文件：`.github/workflows/build.yml`（或 GitLab CI）
- 矩阵构建：
  - Windows: XP/7 (x86+x64), 10/11 (x64)
  - macOS: Intel (x64), Apple Silicon (ARM64)
  - Linux: Ubuntu/Debian/CentOS7 (x64)

#### 任务 6.2：Windows 打包
- 工具：`tauri-bundler` + NSIS（可选）
- 输出：
  - `radius-client-legacy-x86.exe`（XP 32位）
  - `radius-client-legacy-x64.exe`（7 64位）
  - `radius-client-x64.exe`（10/11）
  - 自动检测启动器：`radius-client-launcher.exe`
- 依赖：打包 VC++ Redistributable

#### 任务 6.3：macOS 打包
- 工具：`tauri-bundler` + DMG 打包
- 输出：
  - `RADIUSClient-x64.dmg`（Intel）
  - `RADIUSClient-arm64.dmg`（Apple Silicon）
  - 通用二进制：`RADIUSClient-universal.dmg`
- 代码签名：需要 Apple Developer 证书

#### 任务 6.4：Linux 打包
- 工具：`cargo-deb`, `cargo-rpm`, AppImage
- 输出：
  - `radius-client_amd64.deb`（Ubuntu/Debian）
  - `radius-client-x86_64.rpm`（CentOS7）
  - `RADIUSClient-x86_64.AppImage`（通用）

**验收标准**：
- ✅ 所有 9 个包成功构建
- ✅ 在目标系统上安装运行正常
- ✅ 包体积符合预期（Tauri < 20MB）

---

### Layer 7: 文档与部署（Week 12）

**目标**：完善文档和部署指南

#### 任务 7.1：用户手册
- 文件：`docs/user-guide.md`
- 内容：
  - 安装步骤（每个平台截图）
  - 使用流程（3 步连接指南）
  - 常见问题（证书警告、权限问题）

#### 任务 7.2：管理员部署指南
- 文件：`docs/deployment-guide.md`
- 内容：
  - 配置文件模板
  - 批量部署脚本（GPO/脚本分发）
  - 依赖项检查清单（XP 的 KB893357 等）

#### 任务 7.3：开发者文档
- 文件：`docs/developer-guide.md`
- 内容：
  - 架构设计图
  - 模块职责说明
  - 如何添加新平台支持

**验收标准**：
- ✅ 文档完整，非技术用户可理解
- ✅ 部署指南经过 IT 管理员验证

---

## 风险缓解措施

### 1. Windows XP/7 API 兼容性（关键）

**策略**：
- 优先实现 Win10/11 版本（验证技术可行性）
- XP/7 版本作为独立分支开发
- 提供详细的前提条件检查工具

**应急**：
- 如果 XP 不可行 → 提供 netsh 脚本作为变通
- Win7 最低要求：SP1 + TLS 1.2 补丁

### 2. 证书信任流程

**测试覆盖**：
- 单元测试：证书指纹计算、信任数据库 CRUD
- 集成测试：Mock RADIUS + 自签名证书
- 用户测试：非技术用户完成首次连接

### 3. 跨平台一致性

**验证方法**：
- 每周在 3 个主要平台（Win10/macOS/Ubuntu）测试
- 自动化 E2E 测试（Playwright/Selenium）
- 用户反馈快速迭代

---

## 测试策略

### 单元测试
- **工具**：Rust `cargo test` + JS `vitest`
- **覆盖率**：>70%
- **重点**：状态机逻辑、证书验证、平台抽象层

### 集成测试
- **环境**：Docker + FreeRADIUS Mock 服务器
- **场景**：
  - 成功认证流程
  - 证书首次信任
  - 网络断开恢复
  - 密码错误处理

### 端到端测试
- **工具**：VM 农场（VirtualBox/Hyper-V）
- **平台矩阵**：
  | OS | 版本 | 测试重点 |
  |---|---|---|
  | Windows XP | SP3 | 遗留 API、依赖检查 |
  | Windows 7 | SP1 | TLS 1.2、UAC |
  | Windows 10 | 21H2 | 现代 API、性能 |
  | macOS | 12+ | Keychain、sudo |
  | Ubuntu | 22.04 | NetworkManager |

---

## 部署计划

### 阶段 1：内部测试（Week 9-10）
- 在少量测试机上部署（每平台 2-3 台）
- 收集日志和用户反馈
- 修复关键 bug

### 阶段 2：灰度发布（Week 11）
- 扩大到 10% 用户
- 监控认证成功率
- 优化诊断建议

### 阶段 3：全量发布（Week 12）
- 提供所有 9 个平台的包
- 发布用户手册和部署指南
- 建立技术支持渠道

---

## 资源需求

### 开发团队
- **Rust 后端开发**：1-2 人（必须）
- **前端开发**：1 人（React/TypeScript）
- **测试工程师**：1 人（跨平台测试）
- **技术写作**：0.5 人（文档）

### 基础设施
- **CI/CD**：GitHub Actions 或 GitLab CI（免费额度足够）
- **测试环境**：VirtualBox/Hyper-V VM（本地）
- **Mock RADIUS**：FreeRADIUS Docker 容器

### 外部依赖
- **代码签名证书**：
  - Windows：Sectigo/DigiCert EV 证书（~$300/年）
  - macOS：Apple Developer Program（$99/年）
- **VPN 测试环境**（如果没有真实 RADIUS 服务器）

---

## 交付物清单

### 代码仓库
- [ ] 源代码（Rust + TypeScript）
- [ ] 单元测试 + 集成测试
- [ ] CI/CD 配置

### 可执行文件
- [ ] Windows: 4 个 .exe（XP x86, 7 x64, 10/11 x64, Launcher）
- [ ] macOS: 2-3 个 .dmg（Intel, ARM64, Universal）
- [ ] Linux: 3 个包（.deb, .rpm, .AppImage）

### 文档
- [ ] 用户手册（中文 + 英文）
- [ ] 管理员部署指南
- [ ] 开发者文档
- [ ] API 参考（如果开放扩展）

### 配置文件
- [ ] 默认 config.json 模板
- [ ] PolicyKit 策略文件（Linux）
- [ ] UAC 清单（Windows）

---

## 成功标准

1. ✅ **功能完整性**：所有需求文档中的功能实现
2. ✅ **平台覆盖**：9 个目标平台均可运行
3. ✅ **用户体验**：非技术用户 3 步内完成连接
4. ✅ **性能指标**：
   - 包体积 < 20MB（Tauri）
   - 内存占用 < 50MB
   - 认证响应 < 3 秒
5. ✅ **可维护性**：
   - 单元测试覆盖率 > 70%
   - 代码审查通过（无 Critical 问题）
   - 文档完整

---

## 后续迭代方向

### v1.1（可选功能）
- 多语言支持（i18n）
- 自动更新检查
- 统计仪表板（连接时长、流量）

### v2.0（高级功能）
- 支持 EAP-TLS（证书认证）
- 多网络配置文件（家庭/办公室切换）
- 集中配置管理（从服务器拉取配置）

---

**实施计划生成完毕。请审阅上述计划。**
