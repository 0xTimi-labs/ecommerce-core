use async_trait::async_trait;
use uuid::Uuid;

use crate::dtos::{ReserveStockDto, StockReservationDto};

/// 库存开放服务契约
#[async_trait]
pub trait InventoryService: Send + Sync {
    /// 预留库存
    async fn reserve_stock(&self, req: ReserveStockDto) -> Result<StockReservationDto, String>;
    /// 释放库存预留
    async fn release_stock(&self, reservation_id: &Uuid) -> Result<(), String>;
}
