use async_trait::async_trait;
use wire_contracts::ordering::v1::{
    CancelOrderRequest, CancelOrderResponse, CreateOrderRequest, CreateOrderResponse,
    GetOrderRequest, GetOrderResponse,
};

use crate::errors::OrderingApiError;

/// 订单上下文公共应用服务接口契约
#[async_trait]
pub trait OrderingApiService: Send + Sync {
    /// 处理创建订单命令
    async fn create_order(
        &self,
        req: CreateOrderRequest,
    ) -> Result<CreateOrderResponse, OrderingApiError>;

    /// 处理查询订单详情请求
    async fn get_order(&self, req: GetOrderRequest) -> Result<GetOrderResponse, OrderingApiError>;

    /// 处理取消订单命令
    async fn cancel_order(
        &self,
        req: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, OrderingApiError>;
}
