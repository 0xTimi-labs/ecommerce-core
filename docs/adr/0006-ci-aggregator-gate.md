# CI 门禁聚合器与审查生命周期时序

CI 流水线采用单一出口门禁聚合器（All Required Checks Passed），汇总所有前置构建、测试、契约与安全检查。GitHub Ruleset 与 Merge Queue 仅锚定该聚合门禁，实现下游任务与分支保护规则对具体检查项的完全解耦。AI 审查门禁仅依赖聚合器，确保前置门禁全绿后方可准入，消除无效算力与 Token 浪费。
