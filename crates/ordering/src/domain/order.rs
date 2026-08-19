use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_kernel::{CustomerId, Money, OrderId};

use super::order_item::OrderItem;

/// 订单状态（Saga 状态机）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 草稿
    Draft,
    /// 待库存预留
    PendingInventory,
    /// 待支付
    AwaitingPayment,
    /// 已预授权
    Authorized,
    /// 已支付
    Paid,
    /// 已履约
    Fulfilled,
    /// 已取消
    Cancelled,
}

/// 订单聚合根
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    /// 订单标识
    pub id: OrderId,
    /// 客户标识
    pub customer_id: CustomerId,
    /// 订单项列表
    pub items: Vec<OrderItem>,
    /// 订单状态
    pub status: OrderStatus,
    /// 订单总金额
    pub total_amount: Money,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}
