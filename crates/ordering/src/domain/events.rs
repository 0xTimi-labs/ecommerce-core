use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_kernel::{CustomerId, Money, OrderId, SkuId};
use uuid::Uuid;

/// 订单商品明细快照
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderItemSnapshot {
    pub sku_id: SkuId,
    pub quantity: u32,
    pub unit_price: Money,
}

/// 订单已创建领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderPlacedEvent {
    pub event_id: Uuid,
    pub order_id: OrderId,
    pub customer_id: CustomerId,
    pub quote_id: String,
    pub quote_version: String,
    pub total_amount: Money,
    pub items: Vec<OrderItemSnapshot>,
    pub occurred_at: DateTime<Utc>,
}

/// 订单已取消领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderCancelledEvent {
    pub event_id: Uuid,
    pub order_id: OrderId,
    pub customer_id: CustomerId,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
}
