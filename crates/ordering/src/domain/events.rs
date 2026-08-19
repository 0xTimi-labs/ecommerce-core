use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_kernel::{CustomerId, Money, OrderId};
use uuid::Uuid;

use super::order_item::OrderItem;

/// 订单已创建领域事件 (Published Language)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderPlacedEvent {
    /// 事件标识
    pub event_id: Uuid,
    /// 订单标识
    pub order_id: OrderId,
    /// 客户标识
    pub customer_id: CustomerId,
    /// 报价单标识
    pub quote_id: String,
    /// 报价版本号
    pub quote_version: String,
    /// 订单总金额
    pub total_amount: Money,
    /// 订单包含的商品行项快照
    pub items: Vec<OrderItem>,
    /// 发生时间
    pub occurred_at: DateTime<Utc>,
}

/// 订单已取消领域事件 (Published Language)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderCancelledEvent {
    /// 事件标识
    pub event_id: Uuid,
    /// 订单标识
    pub order_id: OrderId,
    /// 客户标识
    pub customer_id: CustomerId,
    /// 取消原因
    pub reason: String,
    /// 发生时间
    pub occurred_at: DateTime<Utc>,
}
