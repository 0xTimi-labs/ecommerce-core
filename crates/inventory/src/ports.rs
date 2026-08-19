use async_trait::async_trait;
use shared_kernel::{OrderId, ReservationId};

use crate::domain::{
    InventoryError, StockReleasedEvent, StockReservation, StockReservedEvent, StockShortageEvent,
};

/// 库存仓储出向从端口
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

/// 库存领域事件发布出向从端口
#[async_trait]
pub trait InventoryEventPublisherPort: Send + Sync {
    /// 发布库存预留成功领域事件
    async fn publish_stock_reserved(
        &self,
        event: &StockReservedEvent,
    ) -> Result<(), InventoryError>;

    /// 发布库存释放领域事件
    async fn publish_stock_released(
        &self,
        event: &StockReleasedEvent,
    ) -> Result<(), InventoryError>;

    /// 发布缺货领域事件
    async fn publish_stock_shortage(
        &self,
        event: &StockShortageEvent,
    ) -> Result<(), InventoryError>;
}
