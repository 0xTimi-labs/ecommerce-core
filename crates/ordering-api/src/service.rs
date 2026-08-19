use async_trait::async_trait;
use wire_contracts::ordering::v1::{
    CreateOrderRequest, CreateOrderResponse, GetOrderRequest, GetOrderResponse,
};

/// 订单开放服务契约（基于 Wire Protobuf 契约）
#[async_trait]
pub trait OrderingService: Send + Sync {
    /// 创建订单
    async fn create_order(&self, req: CreateOrderRequest) -> Result<CreateOrderResponse, String>;
    /// 查询订单
    async fn get_order(&self, req: GetOrderRequest) -> Result<Option<GetOrderResponse>, String>;
}
