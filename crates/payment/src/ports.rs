use async_trait::async_trait;
use shared_kernel::{AuthorizationId, CaptureId, Money, OrderId, PaymentId};

use crate::domain::{
    Payment, PaymentAuthorizedEvent, PaymentCapturedEvent, PaymentError, PaymentFailedEvent,
    PaymentRefundedEvent, PaymentVoidedEvent,
};

/// 支付网关出向从端口
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

    /// 发起交易退款
    async fn refund(&self, capture_id: &CaptureId, amount: Money) -> Result<String, PaymentError>;
}

/// 支付仓储出向从端口
#[async_trait]
pub trait PaymentRepositoryPort: Send + Sync {
    /// 持久化支付聚合
    async fn save(&self, payment: &Payment) -> Result<(), PaymentError>;

    /// 按支付标识查询
    async fn find_by_id(&self, id: &PaymentId) -> Result<Option<Payment>, PaymentError>;

    /// 按关联订单标识查询
    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Option<Payment>, PaymentError>;
}

/// 支付领域事件发布出向从端口
#[async_trait]
pub trait PaymentEventPublisherPort: Send + Sync {
    /// 发布支付预授权完成领域事件
    async fn publish_payment_authorized(
        &self,
        event: &PaymentAuthorizedEvent,
    ) -> Result<(), PaymentError>;

    /// 发布支付请款结算完成领域事件
    async fn publish_payment_captured(
        &self,
        event: &PaymentCapturedEvent,
    ) -> Result<(), PaymentError>;

    /// 发布预授权撤销领域事件
    async fn publish_payment_voided(&self, event: &PaymentVoidedEvent) -> Result<(), PaymentError>;

    /// 发布退款完成领域事件
    async fn publish_payment_refunded(
        &self,
        event: &PaymentRefundedEvent,
    ) -> Result<(), PaymentError>;

    /// 发布支付失败领域事件
    async fn publish_payment_failed(&self, event: &PaymentFailedEvent) -> Result<(), PaymentError>;
}
