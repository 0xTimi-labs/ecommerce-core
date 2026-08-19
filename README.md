# 电商核心交易系统 (Ecommerce Core)

> 工业级领域驱动设计 (DDD)、契约优先 (Contract-First SSOT) 与垂直切片双层 TDD 的参考架构实现。

---

## 一、 架构蓝图与限界上下文

系统基于六边形架构 (Hexagonal Architecture) 与事件驱动协同式 Saga (Choreography Saga) 构建，划分为三大核心限界上下文与共享内核：

- **订单上下文 (`crates/ordering`)**：负责订单生命周期流转、金额聚合计算与状态机控制；
- **支付上下文 (`crates/payment`)**：负责两阶段支付（预授权与请款结算）及退款管理；
- **库存上下文 (`crates/inventory`)**：负责高并发实物库存原子预留与释放，彻底杜绝超卖；
- **传输契约与生成 (`crates/wire-contracts`)**：基于 Protobuf (Buf v2) 自动生成的跨语言强类型 RPC 与 DTO 事实源；
- **共享内核 (`crates/shared-kernel`)**：极简内核，仅包含强校验 `Money` 值对象与全局实体标识符。

---

## 二、 核心开发与验证命令

```bash
# 执行全量质量门禁（代码格式、Clippy 严格静态检查、全量测试、Protobuf 契约 lint 与前端 TS/Schema 检查）
make check

# 运行全量 Rust 单元与 BDD 业务验收测试
cargo test --workspace

# 前端类型检查与事件 JSON Schema 严格校验
cd apps/web && bun run check
```

---

## 三、 交付阶段与治理规范

- [交付阶段规范 (Delivery Stages)](docs/delivery-stages.md)
- [全生命周期分层测试策略 (Testing Strategy)](docs/architecture/testing-strategy.md)
- [上下文依赖地图 (Context Map)](context-map.md)
- [架构决策记录 (ADRs)](docs/adr/)
