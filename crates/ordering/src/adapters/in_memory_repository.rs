use async_trait::async_trait;
use shared_kernel::OrderId;

use crate::domain::{Order, OrderingError};
use crate::ports::OrderRepositoryPort;

/// 内存订单仓储适配器
#[derive(Debug, Default, Clone)]
pub struct InMemoryOrderRepository;

impl InMemoryOrderRepository {
    /// 构造新的内存仓储实例
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OrderRepositoryPort for InMemoryOrderRepository {
    async fn save(&self, _order: &Order) -> Result<(), OrderingError> {
        Err(OrderingError::NotImplemented {
            feature: "InMemoryOrderRepository::save",
            slice: 1,
        })
    }

    async fn find_by_id(&self, _id: &OrderId) -> Result<Option<Order>, OrderingError> {
        Err(OrderingError::NotImplemented {
            feature: "InMemoryOrderRepository::find_by_id",
            slice: 1,
        })
    }
}
