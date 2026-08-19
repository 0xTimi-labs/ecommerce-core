use shared_kernel::{CustomerId, Money, OrderId};

/// 发起支付请求
#[derive(Debug, Clone)]
pub struct PaymentRequest {
    /// 订单标识
    pub order_id: OrderId,
    /// 客户标识
    pub customer_id: CustomerId,
    /// 支付金额
    pub amount: Money,
}

/// 支付状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    /// 待支付
    Pending,
    /// 已授权
    Authorized,
    /// 已结算
    Captured,
    /// 支付失败
    Failed,
}
