# RADIUS 认证客户端 API 文档

## Tauri 命令接口

前端通过 Tauri 的 `invoke` API 调用后端命令。所有命令返回 `Promise<T>` 或在错误时抛出异常。

---

## 认证管理

### connect_auth

启用 802.1X 认证。

**参数**：
```typescript
{
  interface: string;    // 网络接口名（如 "eth0", "en0"）
  username: string;     // 用户名
  password: string;     // 密码
}
```

**返回**：
```typescript
AuthState  // 最新的认证状态
```

**示例**：
```javascript
import { invoke } from '@tauri-apps/api';

try {
  const state = await invoke('connect_auth', {
    interface: 'eth0',
    username: 'user@example.com',
    password: 'your_password',
  });
  console.log('认证状态:', state);
} catch (error) {
  console.error('连接失败:', error);
}
```

---

### disconnect_auth

禁用 802.1X 认证。

**参数**：
```typescript
{
  interface: string;    // 网络接口名
}
```

**返回**：
```typescript
AuthState  // 更新后的状态（通常为 Disconnected）
```

**示例**：
```javascript
const state = await invoke('disconnect_auth', {
  interface: 'eth0',
});
```

---

### get_auth_status

获取当前认证状态（不触发网络操作）。

**参数**：无

**返回**：
```typescript
AuthState
```

**示例**：
```javascript
const state = await invoke('get_auth_status');
```

---

### refresh_status

刷新认证状态（查询平台层的实时状态）。

**参数**：
```typescript
{
  interface: string;    // 网络接口名
}
```

**返回**：
```typescript
AuthState
```

**示例**：
```javascript
const state = await invoke('refresh_status', {
  interface: 'eth0',
});
```

---

## 证书管理

### trust_certificate

信任证书并继续连接。

**参数**：
```typescript
{
  cert_info: CertificateInfo;  // 证书信息
  interface: string;            // 网络接口名
  username: string;             // 用户名
  password: string;             // 密码
}
```

**返回**：
```typescript
AuthState
```

**示例**：
```javascript
const certInfo = {
  fingerprint: "AA:BB:CC:DD:...",
  issuer: "CN=Example CA",
  subject: "CN=radius.example.com",
  valid_from: "2025-01-01T00:00:00Z",
  valid_to: "2027-12-31T23:59:59Z",
  serial_number: "1234567890",
};

const state = await invoke('trust_certificate', {
  cert_info: certInfo,
  interface: 'eth0',
  username: 'user@example.com',
  password: 'your_password',
});
```

---

### list_trusted_certs

列出所有已信任的证书。

**参数**：无

**返回**：
```typescript
TrustStatus[]
```

**示例**：
```javascript
const certs = await invoke('list_trusted_certs');
console.log('已信任证书:', certs);
```

---

### revoke_certificate_trust

撤销证书信任。

**参数**：
```typescript
{
  fingerprint: string;  // 证书 SHA-256 指纹
}
```

**返回**：
```typescript
void
```

**示例**：
```javascript
await invoke('revoke_certificate_trust', {
  fingerprint: "AA:BB:CC:DD:...",
});
```

---

## 系统诊断

### diagnose_system

执行系统诊断，检查必要的服务和依赖。

**参数**：无

**返回**：
```typescript
DiagResult
```

**示例**：
```javascript
const diag = await invoke('diagnose_system');
console.log('诊断结果:', diag);

// 检查是否有问题
if (!diag.overall_status) {
  diag.services.forEach(service => {
    if (!service.running && service.suggestion) {
      console.warn(`${service.name}: ${service.suggestion}`);
    }
  });
}
```

---

### list_network_interfaces

列出可用的网络接口。

**参数**：无

**返回**：
```typescript
string[]  // 接口名数组
```

**示例**：
```javascript
const interfaces = await invoke('list_network_interfaces');
console.log('可用接口:', interfaces);
```

---

## 数据类型

### AuthState

认证状态枚举（带标签的联合类型）。

```typescript
type AuthState =
  | { status: 'Disconnected' }
  | { status: 'Connecting'; interface: string }
  | { status: 'CertificatePrompt'; cert_fingerprint: string }
  | { status: 'Connected'; interface: string; ip: string; connected_at: string }
  | { status: 'Failed'; error: string };
```

**状态说明**：
- **Disconnected**: 未连接
- **Connecting**: 连接中
- **CertificatePrompt**: 等待证书信任确认
- **Connected**: 已连接（包含 IP 地址和连接时间）
- **Failed**: 连接失败（包含错误信息）

---

### CertificateInfo

证书信息结构。

```typescript
interface CertificateInfo {
  fingerprint: string;      // SHA-256 指纹（格式：AA:BB:CC:...）
  issuer: string;           // 颁发者（如 "CN=Example CA"）
  subject: string;          // 主题（如 "CN=radius.example.com"）
  valid_from: string;       // 有效期起始（ISO 8601）
  valid_to: string;         // 有效期结束（ISO 8601）
  serial_number: string;    // 序列号
}
```

---

### TrustStatus

证书信任状态。

```typescript
interface TrustStatus {
  fingerprint: string;      // 证书指纹
  trusted: boolean;         // 是否信任
  trusted_at: string | null;  // 信任时间（ISO 8601）
  expires_at: string | null;  // 过期时间（ISO 8601）
}
```

---

### DiagResult

系统诊断结果。

```typescript
interface DiagResult {
  services: ServiceStatus[];  // 服务状态列表
  overall_status: boolean;    // 总体状态（true = 所有服务正常）
}

interface ServiceStatus {
  name: string;              // 服务名称
  running: boolean;          // 是否运行
  suggestion: string | null; // 修复建议（如果有问题）
}
```

---

## 错误处理

所有命令在失败时会抛出字符串类型的错误。建议使用 try-catch 捕获：

```javascript
try {
  const state = await invoke('connect_auth', { /* ... */ });
} catch (error) {
  // error 是字符串类型
  if (error.includes('服务')) {
    console.error('服务未运行:', error);
  } else if (error.includes('密码')) {
    console.error('认证失败:', error);
  } else {
    console.error('未知错误:', error);
  }
}
```

---

## 完整示例：连接流程

```javascript
import { invoke } from '@tauri-apps/api';

async function connectToRadius(interface, username, password) {
  try {
    // 1. 尝试连接
    let state = await invoke('connect_auth', {
      interface,
      username,
      password,
    });

    // 2. 检查状态
    if (state.status === 'CertificatePrompt') {
      // 需要用户确认证书
      const certFingerprint = state.cert_fingerprint;
      
      // 显示证书对话框，用户确认后：
      const certInfo = await fetchCertificateInfo(certFingerprint);
      
      state = await invoke('trust_certificate', {
        cert_info: certInfo,
        interface,
        username,
        password,
      });
    }

    // 3. 检查最终状态
    if (state.status === 'Connected') {
      console.log(`✅ 已连接到 ${state.interface}`);
      console.log(`   IP: ${state.ip}`);
      console.log(`   时间: ${state.connected_at}`);
    } else if (state.status === 'Failed') {
      console.error(`❌ 连接失败: ${state.error}`);
    }

  } catch (error) {
    console.error('连接过程出错:', error);
  }
}

// 使用示例
connectToRadius('eth0', 'user@example.com', 'password123');
```

---

## 轮询状态更新

由于 Tauri 不支持服务器推送，建议使用轮询：

```javascript
let statusInterval;

function startStatusPolling(interface) {
  statusInterval = setInterval(async () => {
    try {
      const state = await invoke('refresh_status', { interface });
      updateUI(state);
    } catch (error) {
      console.error('刷新状态失败:', error);
    }
  }, 3000);  // 每 3 秒刷新
}

function stopStatusPolling() {
  if (statusInterval) {
    clearInterval(statusInterval);
  }
}
```

---

## 平台差异

### Windows
- 首次连接可能需要启动服务（`Wired AutoConfig`, `EapHost`）
- 诊断会检查服务状态并提供自动修复建议

### macOS
- 需要管理员权限（首次运行会提示输入密码）
- 证书导入到系统 Keychain

### Linux
- 优先使用 NetworkManager，失败时降级到 wpa_supplicant
- 证书可导入系统存储（需 root）或用户目录（无需 root）
- 诊断会检查 NetworkManager 服务和 wpa_supplicant 包

---

## TypeScript 类型定义

建议在前端项目中创建 `src/types/tauri-commands.d.ts`：

```typescript
declare module '@tauri-apps/api' {
  export function invoke<T = any>(
    cmd: 'connect_auth',
    args: { interface: string; username: string; password: string }
  ): Promise<AuthState>;
  
  export function invoke<T = any>(
    cmd: 'disconnect_auth',
    args: { interface: string }
  ): Promise<AuthState>;
  
  export function invoke<T = any>(
    cmd: 'get_auth_status'
  ): Promise<AuthState>;
  
  export function invoke<T = any>(
    cmd: 'refresh_status',
    args: { interface: string }
  ): Promise<AuthState>;
  
  export function invoke<T = any>(
    cmd: 'trust_certificate',
    args: {
      cert_info: CertificateInfo;
      interface: string;
      username: string;
      password: string;
    }
  ): Promise<AuthState>;
  
  export function invoke<T = any>(
    cmd: 'list_trusted_certs'
  ): Promise<TrustStatus[]>;
  
  export function invoke<T = any>(
    cmd: 'revoke_certificate_trust',
    args: { fingerprint: string }
  ): Promise<void>;
  
  export function invoke<T = any>(
    cmd: 'diagnose_system'
  ): Promise<DiagResult>;
  
  export function invoke<T = any>(
    cmd: 'list_network_interfaces'
  ): Promise<string[]>;
}
```
