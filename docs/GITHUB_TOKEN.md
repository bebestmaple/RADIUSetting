# GitHub Token 说明

## ✅ 无需手动配置

**GITHUB_TOKEN 是自动生成的**，GitHub Actions 会自动注入，无需任何手动操作。

---

## 🔧 已修复的配置

### 添加权限声明

在 `.github/workflows/build.yml` 中添加了：

```yaml
permissions:
  contents: write
```

这允许 GitHub Actions 创建 Release 并上传文件。

---

## 📋 无需操作的情况

以下情况 **不需要** 手动生成 Token：

✅ 使用 `${{ secrets.GITHUB_TOKEN }}` - 自动注入  
✅ 在 GitHub Actions 中创建 Release  
✅ 上传 Artifact  
✅ 提交代码  

---

## 🔑 何时需要手动生成 Token

仅在以下情况需要手动创建 Personal Access Token (PAT)：

1. **本地脚本访问 GitHub API**
   - 使用 `gh` CLI 本地推送 Release
   - 本地运行自动化脚本

2. **跨仓库操作**
   - 从一个仓库触发另一个仓库的 Workflow

3. **第三方 CI/CD**
   - 在 Jenkins、Travis 等非 GitHub 环境构建

---

## 🎯 当前状态

✅ **配置已修复**：添加了 `permissions: contents: write`  
✅ **代码已推送**：更新的配置已提交  
✅ **标签已重建**：删除旧标签并重新创建 v0.1.0  
✅ **CI/CD 已触发**：GitHub Actions 正在运行  

---

## 🔍 验证构建

访问：https://github.com/bebestmaple/RADIUSetting/actions

应该看到：
- ✅ 构建多平台包（v0.1.0）运行中
- 3 个并行任务：Windows、Ubuntu、macOS

---

## 📦 构建完成后

Release 会自动创建在：
https://github.com/bebestmaple/RADIUSetting/releases/tag/v0.1.0

包含文件：
- radius-client.exe (Windows)
- *.AppImage (Linux)
- *.deb (Ubuntu/Debian)
- *.dmg (macOS)
- *-portable.zip (macOS)

---

**总结**：已修复权限配置，无需手动生成 Token，CI/CD 正在自动构建中。
