use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_kernel::{CustomerId, OrderId};
use uuid::Uuid;

use super::order_item::OrderItem;

/// 订单已创建领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderPlacedEvent {
    /// 事件标识
    pub event_id: Uuid,
    /// 订单标识
    pub order_id: OrderId,
    /// 客户标识
    pub customer_id: CustomerId,
    /// 订单项快照
    pub items: Vec<OrderItem>,
    /// 发生时间
    pub occurred_at: DateTime<Utc>,
}
