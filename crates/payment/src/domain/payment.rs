use serde::{Deserialize, Serialize};
use shared_kernel::{AuthorizationId, CaptureId, Money, OrderId, PaymentId};

use super::errors::PaymentError;

/// 支付流转状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    /// 待授权
    Pending,
    /// 已授权（额度冻结）
    Authorized,
    /// 已请款（结算完成）
    Captured,
    /// 预授权已撤销
    Voided,
    /// 已退款
    Refunded,
    /// 支付失败
    Failed,
}

/// 支付聚合根
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Payment {
    id: PaymentId,
    order_id: OrderId,
    amount: Money,
    authorization_id: Option<AuthorizationId>,
    capture_id: Option<CaptureId>,
    status: PaymentStatus,
}

impl Payment {
    /// 创建新的支付意向并校验不变量
    pub fn new(order_id: OrderId, amount: Money) -> Result<Self, PaymentError> {
        if amount.is_zero() {
            return Err(PaymentError::InvalidAmount(
                "支付意向金额必须大于 0".to_string(),
            ));
        }
        Ok(Self {
            id: PaymentId::new(),
            order_id,
            amount,
            authorization_id: None,
            capture_id: None,
            status: PaymentStatus::Pending,
        })
    }

    /// 执行预授权
    pub fn authorize(&mut self, authorization_id: AuthorizationId) -> Result<(), PaymentError> {
        if self.status != PaymentStatus::Pending {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Authorized,
            });
        }
        self.authorization_id = Some(authorization_id);
        self.status = PaymentStatus::Authorized;
        Ok(())
    }

    /// 执行请款结算
    pub fn capture(&mut self, capture_id: CaptureId) -> Result<(), PaymentError> {
        if self.status != PaymentStatus::Authorized {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Captured,
            });
        }
        self.capture_id = Some(capture_id);
        self.status = PaymentStatus::Captured;
        Ok(())
    }

    /// 撤销预授权
    pub fn void(&mut self) -> Result<(), PaymentError> {
        if self.status != PaymentStatus::Authorized {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Voided,
            });
        }
        self.status = PaymentStatus::Voided;
        Ok(())
    }

    /// 标记支付失败
    pub fn fail(&mut self) -> Result<(), PaymentError> {
        if self.status == PaymentStatus::Captured {
            return Err(PaymentError::InvalidStateTransition {
                from: self.status,
                to: PaymentStatus::Failed,
            });
        }
        self.status = PaymentStatus::Failed;
        Ok(())
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
