use async_trait::async_trait;

use crate::domain::{OrderPlacedEvent, OrderingError};
use crate::ports::EventPublisherPort;

/// 事件发布器
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopEventPublisher;

#[async_trait]
impl EventPublisherPort for NoopEventPublisher {
    async fn publish_order_placed(&self, _event: &OrderPlacedEvent) -> Result<(), OrderingError> {
        Err(OrderingError::NotImplemented {
            feature: "NoopEventPublisher::publish_order_placed",
            slice: 1,
        })
    }
}
