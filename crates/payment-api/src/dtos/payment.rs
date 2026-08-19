use serde::{Deserialize, Serialize};
use shared_kernel::{CustomerId, Money, OrderId};
use uuid::Uuid;

/// 授权支付请求 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizePaymentDto {
    /// 订单标识
    pub order_id: OrderId,
    /// 客户标识
    pub customer_id: CustomerId,
    /// 授权金额
    pub amount: Money,
}

/// 支付响应 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDto {
    /// 支付标识
    pub payment_id: Uuid,
    /// 订单标识
    pub order_id: OrderId,
    /// 支付状态
    pub status: String,
    /// 支付金额
    pub amount: Money,
}
