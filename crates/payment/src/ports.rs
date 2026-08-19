use async_trait::async_trait;
use shared_kernel::{AuthorizationId, CaptureId, Money, OrderId, PaymentId};

use crate::domain::{Payment, PaymentError};

/// 支付网关端口 (Driven Outgoing Port)
#[async_trait]
pub trait PaymentGatewayPort: Send + Sync {
    /// 向外部支付通道发起预授权
    async fn authorize(
        &self,
        order_id: &OrderId,
        amount: Money,
    ) -> Result<AuthorizationId, PaymentError>;

    /// 向外部支付通道发起请款结算
    async fn capture(
        &self,
        authorization_id: &AuthorizationId,
        amount: Money,
    ) -> Result<CaptureId, PaymentError>;

    /// 撤销未结算预授权
    async fn void(&self, authorization_id: &AuthorizationId) -> Result<(), PaymentError>;
}

/// 支付仓储端口 (Driven Outgoing Port)
#[async_trait]
pub trait PaymentRepositoryPort: Send + Sync {
    /// 持久化支付聚合
    async fn save(&self, payment: &Payment) -> Result<(), PaymentError>;

    /// 按支付标识查询
    async fn find_by_id(&self, id: &PaymentId) -> Result<Option<Payment>, PaymentError>;

    /// 按关联订单标识查询
    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Option<Payment>, PaymentError>;
}
