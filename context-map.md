# 上下文地图 (Context Map)

## 上下文

- [订单 (Ordering)](./crates/ordering/context.md) — 接收并管理客户订单全生命周期与状态机
- [支付 (Payment)](./crates/payment/context.md) — 处理订单支付授权、扣款结算与退款
- [库存履约 (Inventory)](./crates/inventory/context.md) — 管理商品 SKU 库位、预留库存与出库履约

## 关系模式

- **订单 [U, OHS, PL] → 支付 [D, ACL]**
    - **事件**：发布 `OrderPlaced`，支付上下文订阅并初始化支付意向
    - **事件**：发布 `OrderCancelled`，支付上下文订阅并撤销预授权/退款
    - **RPC**：支付上下文通过 `wire_contracts::ordering::v1` gRPC API 查询订单详情与金额
- **订单 [U, OHS, PL] → 库存履约 [D, ACL]**
    - **事件**：发布 `OrderPlaced`，库存上下文订阅并执行预留（ReserveStock）
    - **事件**：发布 `OrderCancelled`，库存上下文订阅并释放预留库存
- **支付 [U, PL] → 订单 [D, ACL]**
    - **事件**：发布 `PaymentAuthorized`，订单上下文订阅推进至 `Authorized`（额度已冻结）
    - **事件**：发布 `PaymentCaptured`，订单上下文订阅推进至 `Paid`（资金已结算）
    - **事件**：发布 `PaymentFailed`，订单上下文订阅推进至 `Cancelled`（补偿释放）
- **库存履约 [U, PL] → 订单 [D, ACL]**
    - **事件**：发布 `StockReserved`，订单上下文订阅推进至 `AwaitingPayment`
    - **事件**：发布 `StockShortage`，订单上下文触发取消并通知客户
- **订单 ↔ 支付 ↔ 库存履约 [SK]**
    - **共享内核 (Shared Kernel)**：`crates/shared-kernel` 共享 `OrderId`, `CustomerId`, `SkuId`, `Money` 核心值对象与强类型标识符

## 标记说明

- **U**: 上游 `Upstream`
- **D**: 下游 `Downstream`
- **OHS**: 开放服务 `Open Host Service`
- **PL**: 发布者语言 `Published Language`
- **ACL**: 防腐层 `Anticorruption Layer`
- **SK**: 共享内核 `Shared Kernel`
