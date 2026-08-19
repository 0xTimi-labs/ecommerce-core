# 上下文地图

## 上下文

- [订单](./crates/ordering/context.md) — 接收并跟踪客户订单
- [支付](./crates/payment/context.md) — 处理订单支付授权、扣款结算与退款
- [库存履约](./crates/inventory/context.md) — 管理商品库存预留、拣货与出库

## 关系模式

- **订单 [U, OHS, PL] → 支付 [D, ACL]**
    - **事件**：发布 `OrderPlaced`，订阅初始化支付单
    - **事件**：发布 `OrderCancelled`，订阅释放预授权与退款
    - **查询**：gRPC API 获取订单详情与金额
- **订单 [U, OHS, PL] → 库存履约 [D, ACL]**
    - **事件**：发布 `OrderPlaced`，订阅执行预留库存
    - **事件**：发布 `OrderCancelled`，订阅释放预留库存
- **支付 [U, PL] → 订单 [D, ACL]**
    - **事件**：发布 `PaymentAuthorized`，订阅推进订单状态至已授权
    - **事件**：发布 `PaymentCaptured`，订阅推进订单状态至已支付
    - **事件**：发布 `PaymentFailed`，订阅推进订单状态至已取消
- **库存履约 [U, PL] → 订单 [D, ACL]**
    - **事件**：发布 `StockReserved`，订阅推进订单状态至待支付
    - **事件**：发布 `StockReleased`，订阅确认库存释放
    - **事件**：发布 `StockShortage`，订阅触发订单取消与客户通知
- **订单 ↔ 支付 ↔ 库存履约 [SK]**
    - **共享**：`OrderId`, `CustomerId`, `SkuId`, `Money` 类型

## 标记说明

- **U**: 上游 `Upstream`
- **D**: 下游 `Downstream`
- **OHS**: 开放服务 `Open Host Service`
- **PL**: 发布者语言 `Published Language`
- **ACL**: 防腐层 `Anticorruption Layer`
- **SK**: 共享内核 `Shared Kernel`
