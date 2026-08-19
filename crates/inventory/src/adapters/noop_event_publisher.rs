use async_trait::async_trait;

use crate::domain::{InventoryError, StockReleasedEvent, StockReservedEvent, StockShortageEvent};
use crate::ports::InventoryEventPublisherPort;

/// 空库存事件发布者适配器
#[derive(Debug, Default, Clone)]
pub struct NoopInventoryEventPublisher;

#[async_trait]
impl InventoryEventPublisherPort for NoopInventoryEventPublisher {
    async fn publish_stock_reserved(
        &self,
        _event: &StockReservedEvent,
    ) -> Result<(), InventoryError> {
        Err(InventoryError::NotImplemented(
            "NoopInventoryEventPublisher::publish_stock_reserved",
        ))
    }

    async fn publish_stock_released(
        &self,
        _event: &StockReleasedEvent,
    ) -> Result<(), InventoryError> {
        Err(InventoryError::NotImplemented(
            "NoopInventoryEventPublisher::publish_stock_released",
        ))
    }

    async fn publish_stock_shortage(
        &self,
        _event: &StockShortageEvent,
    ) -> Result<(), InventoryError> {
        Err(InventoryError::NotImplemented(
            "NoopInventoryEventPublisher::publish_stock_shortage",
        ))
    }
}
