use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_kernel::{OrderId, ReservationId, SkuId};

/// 库存预留条目
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReservedItem {
    pub sku_id: SkuId,
    pub quantity: u32,
}

/// 缺货商品条目明细
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnavailableItem {
    pub sku_id: SkuId,
    pub requested_quantity: u32,
    pub available_quantity: u32,
}

/// 库存预留成功领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockReservedEvent {
    pub reservation_id: ReservationId,
    pub order_id: OrderId,
    pub items: Vec<ReservedItem>,
    pub occurred_at: DateTime<Utc>,
}

/// 库存释放领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockReleasedEvent {
    pub reservation_id: ReservationId,
    pub order_id: OrderId,
    pub occurred_at: DateTime<Utc>,
}

/// 缺货领域事件 (自包含全量缺货条目快照)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockShortageEvent {
    pub order_id: OrderId,
    pub unavailable_items: Vec<UnavailableItem>,
    pub occurred_at: DateTime<Utc>,
}
