use async_trait::async_trait;
use ordering_api::{CreateOrderDto, OrderDto};
use shared_kernel::OrderId;

use crate::domain::OrderingError;

/// 创建订单进向端口
#[async_trait]
pub trait CreateOrderPort: Send + Sync {
    /// 执行创建订单
    async fn execute(&self, req: CreateOrderDto) -> Result<OrderDto, OrderingError>;
}

/// 查询订单进向端口
#[async_trait]
pub trait GetOrderPort: Send + Sync {
    /// 查询订单详情
    async fn execute(&self, order_id: &OrderId) -> Result<Option<OrderDto>, OrderingError>;
}
