use shared_kernel::{AuthorizationId, CaptureId, Money, OrderId, PaymentId};

use super::errors::PaymentError;
use super::events::{
    PaymentAuthorizedEvent, PaymentCapturedEvent, PaymentFailedEvent, PaymentRefundedEvent,
    PaymentVoidedEvent,
};
use chrono::Utc;

/// 支付聚合流转状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    /// 待支付处理
    Pending,
    /// 已预授权冻结额度
    Authorized,
    /// 已请款结清
    Captured,
    /// 预授权已撤销
    Voided,
    /// 款项已退还
    Refunded,
    /// 支付失败
    Failed,
}

/// 支付聚合根
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    id: PaymentId,
    order_id: OrderId,
    amount: Money,
    authorization_id: Option<AuthorizationId>,
    capture_id: Option<CaptureId>,
    status: PaymentStatus,
}

impl Payment {
    /// 创建待处理支付单
    #[must_use]
    pub const fn new(id: PaymentId, order_id: OrderId, amount: Money) -> Self {
        Self {
            id,
            order_id,
            amount,
            authorization_id: None,
            capture_id: None,
            status: PaymentStatus::Pending,
        }
    }

    /// 执行预授权冻结
    pub fn authorize(
        &mut self,
        authorization_id: AuthorizationId,
    ) -> Result<PaymentAuthorizedEvent, PaymentError> {
        if self.status != PaymentStatus::Pending {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Authorized,
            });
        }
        self.authorization_id = Some(authorization_id);
        self.status = PaymentStatus::Authorized;
        Ok(PaymentAuthorizedEvent {
            payment_id: self.id,
            authorization_id,
            order_id: self.order_id,
            amount: self.amount,
            occurred_at: Utc::now(),
        })
    }

    /// 执行请款结算
    pub fn capture(&mut self, capture_id: CaptureId) -> Result<PaymentCapturedEvent, PaymentError> {
        if self.status != PaymentStatus::Authorized {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Captured,
            });
        }
        self.capture_id = Some(capture_id);
        self.status = PaymentStatus::Captured;
        Ok(PaymentCapturedEvent {
            payment_id: self.id,
            capture_id,
            order_id: self.order_id,
            amount: self.amount,
            occurred_at: Utc::now(),
        })
    }

    /// 撤销预授权
    pub fn void(&mut self) -> Result<PaymentVoidedEvent, PaymentError> {
        if self.status != PaymentStatus::Authorized {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Voided,
            });
        }
        let authorization_id =
            self.authorization_id
                .ok_or(PaymentError::InvalidStateTransition {
                    from: self.status,
                    to: PaymentStatus::Voided,
                })?;
        self.status = PaymentStatus::Voided;
        Ok(PaymentVoidedEvent {
            payment_id: self.id,
            authorization_id,
            order_id: self.order_id,
            occurred_at: Utc::now(),
        })
    }

    /// 执行退款流转
    pub fn refund(&mut self, refund_id: String) -> Result<PaymentRefundedEvent, PaymentError> {
        if self.status != PaymentStatus::Captured {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Refunded,
            });
        }
        let capture_id = self
            .capture_id
            .ok_or(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Refunded,
            })?;
        self.status = PaymentStatus::Refunded;
        Ok(PaymentRefundedEvent {
            payment_id: self.id,
            refund_id,
            capture_id,
            order_id: self.order_id,
            amount: self.amount,
            occurred_at: Utc::now(),
        })
    }

    /// 标记支付失败（仅允许从待处理或已授权状态流转）
    pub fn fail(
        &mut self,
        error_code: String,
        reason: String,
    ) -> Result<PaymentFailedEvent, PaymentError> {
        if self.status != PaymentStatus::Pending && self.status != PaymentStatus::Authorized {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Failed,
            });
        }
        self.status = PaymentStatus::Failed;
        Ok(PaymentFailedEvent {
            payment_id: self.id,
            order_id: self.order_id,
            amount: self.amount,
            error_code,
            reason,
            occurred_at: Utc::now(),
        })
    }

    #[must_use]
    pub const fn id(&self) -> &PaymentId {
        &self.id
    }

    #[must_use]
    pub const fn order_id(&self) -> &OrderId {
        &self.order_id
    }

    #[must_use]
    pub const fn amount(&self) -> Money {
        self.amount
    }

    #[must_use]
    pub const fn authorization_id(&self) -> Option<&AuthorizationId> {
        self.authorization_id.as_ref()
    }

    #[must_use]
    pub const fn capture_id(&self) -> Option<&CaptureId> {
        self.capture_id.as_ref()
    }

    #[must_use]
    pub const fn status(&self) -> PaymentStatus {
        self.status
    }
}
