use async_trait::async_trait;
use shared_kernel::OrderId;

use crate::domain::{Order, OrderPlacedEvent, OrderingError};

/// 订单仓储出向从端口 (Driven Port)
#[async_trait]
pub trait OrderRepositoryPort: Send + Sync {
    /// 保存或更新订单聚合根
    async fn save(&self, order: &Order) -> Result<(), OrderingError>;
    /// 根据标识查询订单聚合根
    async fn find_by_id(&self, id: &OrderId) -> Result<Option<Order>, OrderingError>;
}

/// 领域事件发布出向从端口 (Driven Port)
#[async_trait]
pub trait EventPublisherPort: Send + Sync {
    /// 发布订单创建领域事件
    async fn publish_order_placed(&self, event: &OrderPlacedEvent) -> Result<(), OrderingError>;
}
