use async_trait::async_trait;
use shared_kernel::{OrderId, PaymentId};

use crate::domain::{Payment, PaymentError};
use crate::ports::PaymentRepositoryPort;

/// 内存支付仓储适配器
#[derive(Debug, Default, Clone)]
pub struct InMemoryPaymentRepository;

impl InMemoryPaymentRepository {
    /// 构造新的内存仓储实例
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PaymentRepositoryPort for InMemoryPaymentRepository {
    async fn save(&self, _payment: &Payment) -> Result<(), PaymentError> {
        Err(PaymentError::NotImplemented(
            "InMemoryPaymentRepository::save",
        ))
    }

    async fn find_by_id(&self, _id: &PaymentId) -> Result<Option<Payment>, PaymentError> {
        Err(PaymentError::NotImplemented(
            "InMemoryPaymentRepository::find_by_id",
        ))
    }

    async fn find_by_order_id(&self, _order_id: &OrderId) -> Result<Option<Payment>, PaymentError> {
        Err(PaymentError::NotImplemented(
            "InMemoryPaymentRepository::find_by_order_id",
        ))
    }
}
