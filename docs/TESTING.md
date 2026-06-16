# RADIUS 认证客户端 - 测试指南

## 测试用例覆盖

### 单元测试

#### cert_validator.rs（证书验证）
- ✅ `test_fingerprint_calculation` - 验证 SHA-256 指纹格式
- ✅ `test_fingerprint_consistency` - 相同数据生成相同指纹
- ✅ `test_fingerprint_different_data` - 不同数据生成不同指纹
- ✅ `test_trust_workflow` - 完整的信任/撤销流程
- ✅ `test_certificate_expiry` - 过期证书检测
- ✅ `test_certificate_not_expired` - 未过期证书检测
- ✅ `test_parse_certificate_basic` - 基本证书解析

#### auth_manager.rs（认证状态机）
- ✅ `test_state_transitions` - 5 种状态转换测试
- ✅ `test_state_clone` - 状态克隆测试
- ✅ `test_thread_safety` - 多线程安全测试

---

## 运行测试

### 前置条件
- Rust 1.70+
- Cargo

### 运行所有测试
```bash
cd src-tauri
cargo test
```

### 运行特定模块测试
```bash
# 仅测试证书验证模块
cargo test --package radius-client --lib core::cert_validator

# 仅测试认证状态机
cargo test --package radius-client --lib core::auth_manager
```

### 显示测试输出
```bash
cargo test -- --nocapture
```

### 运行单个测试
```bash
cargo test test_fingerprint_calculation -- --exact
```

---

## 编译检查

### 检查代码（不构建二进制）
```bash
cd src-tauri
cargo check
```

### 检查所有平台代码
```bash
# Windows 特定代码
cargo check --target x86_64-pc-windows-msvc

# macOS 特定代码
cargo check --target x86_64-apple-darwin

# Linux 特定代码
cargo check --target x86_64-unknown-linux-gnu
```

---

## 代码质量检查

### Clippy（代码规范）
```bash
cd src-tauri
cargo clippy
```

### 格式化检查
```bash
cargo fmt -- --check
```

### 自动格式化
```bash
cargo fmt
```

---

## 集成测试（待实施）

### Mock RADIUS 服务器测试
```bash
# 启动 Mock 服务器
docker run -d -p 1812:1812/udp freeradius/freeradius

# 运行集成测试
cargo test --test integration_tests
```

---

## 性能测试

### 基准测试
```bash
cargo bench
```

---

## 测试覆盖率

### 使用 tarpaulin
```bash
# 安装
cargo install cargo-tarpaulin

# 运行
cargo tarpaulin --out Html --output-dir coverage
```

### 使用 llvm-cov
```bash
# 安装
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov

# 运行
cargo llvm-cov --html
```

---

## CI/CD 测试

GitHub Actions 自动运行测试：
```yaml
- name: 运行测试
  run: |
    cd src-tauri
    cargo test --all-features
```

---

## 常见问题

### 问题 1：测试失败 - 找不到配置目录
**原因**：测试环境没有 HOME 目录

**解决**：
```bash
# Linux/macOS
export HOME=/tmp

# Windows
set USERPROFILE=C:\Temp
```

### 问题 2：多线程测试冲突
**原因**：多个测试同时修改信任数据库

**解决**：使用 `--test-threads=1`
```bash
cargo test -- --test-threads=1
```

### 问题 3：条件编译警告
**原因**：在非目标平台编译平台特定代码

**解决**：这是正常现象，可以忽略警告

---

## 预期测试结果

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

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 下一步

1. **补充集成测试**：测试完整的连接流程
2. **添加 Mock 服务器**：使用 FreeRADIUS 进行真实测试
3. **性能基准测试**：测量认证延迟和吞吐量
4. **平台测试**：在真实的 Windows/macOS/Linux 环境测试
