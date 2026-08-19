use async_trait::async_trait;

use crate::domain::{OrderCancelledEvent, OrderPlacedEvent, OrderingError};
use crate::ports::EventPublisherPort;

/// 空事件发布者适配器
#[derive(Debug, Default, Clone)]
pub struct NoopEventPublisher;

#[async_trait]
impl EventPublisherPort for NoopEventPublisher {
    async fn publish_order_placed(&self, _event: &OrderPlacedEvent) -> Result<(), OrderingError> {
        Err(OrderingError::NotImplemented(
            "NoopEventPublisher::publish_order_placed",
        ))
    }

    async fn publish_order_cancelled(
        &self,
        _event: &OrderCancelledEvent,
    ) -> Result<(), OrderingError> {
        Err(OrderingError::NotImplemented(
            "NoopEventPublisher::publish_order_cancelled",
        ))
    }
}
