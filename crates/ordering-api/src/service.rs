use async_trait::async_trait;
use shared_kernel::OrderId;

use crate::dtos::{CreateOrderDto, OrderDto};

/// 订单开放服务契约
#[async_trait]
pub trait OrderingService: Send + Sync {
    /// 创建订单
    async fn create_order(&self, req: CreateOrderDto) -> Result<OrderDto, String>;
    /// 查询订单
    async fn get_order(&self, order_id: &OrderId) -> Result<Option<OrderDto>, String>;
}
