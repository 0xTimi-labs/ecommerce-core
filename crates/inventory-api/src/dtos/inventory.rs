use serde::{Deserialize, Serialize};
use shared_kernel::{OrderId, SkuId};
use uuid::Uuid;

/// 预留库存项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveStockItemDto {
    /// SKU 标识
    pub sku_id: SkuId,
    /// 预留数量
    pub quantity: u32,
}

/// 预留库存请求 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReserveStockDto {
    /// 订单标识
    pub order_id: OrderId,
    /// 预留商品项列表
    pub items: Vec<ReserveStockItemDto>,
}

/// 预留库存响应 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockReservationDto {
    /// 预留标识
    pub reservation_id: Uuid,
    /// 订单标识
    pub order_id: OrderId,
    /// 预留状态
    pub status: String,
}
