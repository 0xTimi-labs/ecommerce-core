use async_trait::async_trait;
use shared_kernel::{OrderId, ReservationId};

use crate::domain::{InventoryError, StockReservation};

/// 库存仓储端口 (Driven Outgoing Port)
#[async_trait]
pub trait InventoryRepositoryPort: Send + Sync {
    /// 持久化库存预留聚合
    async fn save(&self, reservation: &StockReservation) -> Result<(), InventoryError>;

    /// 按预留标识查询
    async fn find_by_id(
        &self,
        id: &ReservationId,
    ) -> Result<Option<StockReservation>, InventoryError>;

    /// 按关联订单标识查询
    async fn find_by_order_id(
        &self,
        order_id: &OrderId,
    ) -> Result<Option<StockReservation>, InventoryError>;
}
