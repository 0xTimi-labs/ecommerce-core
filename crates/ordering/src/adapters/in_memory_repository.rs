use async_trait::async_trait;
use shared_kernel::OrderId;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::{Order, OrderingError};
use crate::ports::OrderRepositoryPort;

/// 内存订单仓储 (测试与本地运行双体)
#[derive(Debug, Default)]
pub struct InMemoryOrderRepository {
    orders: RwLock<HashMap<OrderId, Order>>,
}

impl InMemoryOrderRepository {
    /// 构造内存订单仓储
    #[must_use]
    pub fn new() -> Self {
        Self {
            orders: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl OrderRepositoryPort for InMemoryOrderRepository {
    async fn save(&self, order: &Order) -> Result<(), OrderingError> {
        let mut orders = self
            .orders
            .write()
            .map_err(|e| OrderingError::ValidationError(format!("获取写锁失败: {e}")))?;
        orders.insert(order.id, order.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &OrderId) -> Result<Option<Order>, OrderingError> {
        let orders = self
            .orders
            .read()
            .map_err(|e| OrderingError::ValidationError(format!("获取读锁失败: {e}")))?;
        Ok(orders.get(id).cloned())
    }
}
