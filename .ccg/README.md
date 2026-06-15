# CCG 工作区

本目录由 CCG Engine 管理，用于任务追踪、规格文档和协作上下文。

## 目录结构

```
.ccg/
├── tasks/          # 任务工作区（每个任务一个子目录）
├── spec/           # 规格文档（OPSX 格式）
├── context/        # 全局上下文文件
└── worktrees/      # Git worktree 隔离环境
```

## 任务生命周期

1. **创建**：`/ccg:go` 或其他入口自动创建任务目录
2. **规划**：生成 `task.json` + `context.jsonl`
3. **执行**：记录 Phase 状态、决策日志
4. **归档**：完成后移动到 `tasks/.archive/`

## 文件说明

- `task.json`：任务元数据（状态、策略、当前阶段）
- `context.jsonl`：相关文件清单（每行一个 JSON 对象）
- `plan.md`：实施计划（多模型协作产出）
- `decisions.md`：关键决策记录
