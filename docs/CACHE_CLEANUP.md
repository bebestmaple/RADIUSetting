# GitHub Actions 缓存清理指南

## 问题：构建使用了旧代码

即使本地已修复，GitHub Actions 可能使用了缓存的旧代码。

## 解决方案

### 方法 1：手动清理缓存（推荐）

1. **访问仓库 Actions 页面**
   https://github.com/bebestmaple/RADIUSetting/actions/caches

2. **删除所有缓存**
   - 点击每个缓存条目的"Delete"按钮
   - 或使用 GitHub CLI：
     ```bash
     gh cache delete --all
     ```

3. **重新触发构建**
   - 删除标签：`git tag -d v0.1.0 && git push origin --delete v0.1.0`
   - 重新创建：`git tag -a v0.1.0 -m "Release v0.1.0" && git push origin v0.1.0`

### 方法 2：修改 Workflow 清理缓存

在 `.github/workflows/build.yml` 中添加：

```yaml
jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
        with:
          clean: true  # 强制清理工作目录
      
      - name: 清理 Cargo 缓存
        run: |
          if (Test-Path "$env:USERPROFILE\.cargo\registry") {
            Remove-Item -Recurse -Force "$env:USERPROFILE\.cargo\registry"
          }
```

### 方法 3：在 Workflow 中禁用缓存

临时在每个 job 添加：

```yaml
- name: 禁用 Rust 缓存
  run: echo "CARGO_INCREMENTAL=0" >> $GITHUB_ENV
```

### 方法 4：使用 workflow_dispatch 手动触发

在 `.github/workflows/build.yml` 顶部已有：

```yaml
on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:  # 允许手动触发
```

访问：https://github.com/bebestmaple/RADIUSetting/actions/workflows/build.yml
点击"Run workflow"按钮手动触发。

---

## 当前状态验证

### 本地代码（正确）
```toml
tauri = { version = "1.5", features = ["shell-open"] }
```

### 远程代码
需要确认 GitHub 远程仓库是否已更新。

### Git 提交历史
```
c6acb6b docs: 添加项目交付总结文档
17b8c07 fix: 移除 custom-protocol feature（Tauri 1.x 不支持）  ← 修复提交
```

---

## 立即行动

1. **访问**：https://github.com/bebestmaple/RADIUSetting/settings/actions
2. **点击**："Clear all caches"（如果可用）
3. **重新触发构建**

或等待约 10 分钟让 GitHub 刷新缓存。
