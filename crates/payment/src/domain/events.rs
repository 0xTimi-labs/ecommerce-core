use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_kernel::{AuthorizationId, CaptureId, Money, OrderId, PaymentId};

/// 支付预授权完成领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentAuthorizedEvent {
    pub payment_id: PaymentId,
    pub authorization_id: AuthorizationId,
    pub order_id: OrderId,
    pub amount: Money,
    pub occurred_at: DateTime<Utc>,
}

/// 支付请款结算完成领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentCapturedEvent {
    pub payment_id: PaymentId,
    pub capture_id: CaptureId,
    pub order_id: OrderId,
    pub amount: Money,
    pub occurred_at: DateTime<Utc>,
}

/// 支付失败领域事件
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentFailedEvent {
    pub payment_id: PaymentId,
    pub order_id: OrderId,
    pub amount: Money,
    pub error_code: String,
    pub reason: String,
    pub occurred_at: DateTime<Utc>,
}
