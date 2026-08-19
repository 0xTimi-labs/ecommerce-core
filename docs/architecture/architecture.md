# 架构原则

## 分层与边界
- **Domain**：聚合根、实体、值对象、领域事件与业务不变量。禁止依赖 adapters、数据库或网络库
- **Ports**：输入端口（Command/Query traits）与输出端口（Repository/Publisher traits）
- **Adapters**：外部技术实现（Tonic gRPC、SQLx、In-Memory、EventBus）
- **Features**：按切片自包含 `command.rs`、`handler.rs`、`validator.rs`、`response.rs`、`persistence.rs`、`tests.rs`

## 通信与依赖
- **单向依赖**：`adapters` -> `ports` <- `domain`
- **Single Source of Truth**：跨进程与跨端通信严格使用 `contracts/` 定义
- **隔离规则**：模块间测试仅使用 `{context}-test-support` 暴露的 Test Double，禁止跨上下文直接引用实现
