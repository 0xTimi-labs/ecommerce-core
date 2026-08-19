---
name: 垂直切片实现
about: 单一垂直切片实现与 TDD 交付任务
title: "feat(slice-{context}): {切片动作}"
---

#{feature_issue_id}

## 切片目标

- 核心动作: `{切片动作名称，如 create_order}`
- 交付范围: 命令/查询定义、输入校验器、领域聚合持久化、适配器实现与单元测试

## 验收场景

- `crates/{context}/tests/features/{feature_name}.feature`: `{Scenario 名称，移除 @ignore}`

## TDD 测试用例

1. 成功场景用例:
   - {说明测试输入与预期断言}
2. 异常与边界用例:
   - {说明边界校验与错误断言}
