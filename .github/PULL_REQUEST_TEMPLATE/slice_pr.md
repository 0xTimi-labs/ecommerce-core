## 关联切片

- Sub-Issue: fixes #{sub_issue_id}
- 解锁场景: `{feature_path}#L{line}` 中的 `{Scenario 标题}`

## 自检清单

- [ ] 仅解锁本切片指定的单个 Scenario
- [ ] `src/features/{action}/tests.rs` 包含实质业务断言
- [ ] 未修改 `contracts/` 下的契约定义
- [ ] `domain/` 未引入外部库或适配器实现
- [ ] 跨模块调用严格使用测试替身（若涉及）
- [ ] 本地 `make check` 全绿

## 说明

{简要说明本切片的实现重点或设计权衡}
