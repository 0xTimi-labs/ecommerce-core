use async_trait::async_trait;

use crate::domain::{
    PaymentAuthorizedEvent, PaymentCapturedEvent, PaymentError, PaymentFailedEvent,
    PaymentRefundedEvent, PaymentVoidedEvent,
};
use crate::ports::PaymentEventPublisherPort;

/// 空支付事件发布者适配器
#[derive(Debug, Default, Clone)]
pub struct NoopPaymentEventPublisher;

#[async_trait]
impl PaymentEventPublisherPort for NoopPaymentEventPublisher {
    async fn publish_payment_authorized(
        &self,
        _event: &PaymentAuthorizedEvent,
    ) -> Result<(), PaymentError> {
        Err(PaymentError::NotImplemented(
            "NoopPaymentEventPublisher::publish_payment_authorized",
        ))
    }

    async fn publish_payment_captured(
        &self,
        _event: &PaymentCapturedEvent,
    ) -> Result<(), PaymentError> {
        Err(PaymentError::NotImplemented(
            "NoopPaymentEventPublisher::publish_payment_captured",
        ))
    }

    async fn publish_payment_voided(
        &self,
        _event: &PaymentVoidedEvent,
    ) -> Result<(), PaymentError> {
        Err(PaymentError::NotImplemented(
            "NoopPaymentEventPublisher::publish_payment_voided",
        ))
    }

    async fn publish_payment_refunded(
        &self,
        _event: &PaymentRefundedEvent,
    ) -> Result<(), PaymentError> {
        Err(PaymentError::NotImplemented(
            "NoopPaymentEventPublisher::publish_payment_refunded",
        ))
    }

    async fn publish_payment_failed(
        &self,
        _event: &PaymentFailedEvent,
    ) -> Result<(), PaymentError> {
        Err(PaymentError::NotImplemented(
            "NoopPaymentEventPublisher::publish_payment_failed",
        ))
    }
}
