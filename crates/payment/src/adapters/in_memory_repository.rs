use async_trait::async_trait;
use shared_kernel::{OrderId, PaymentId};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::{Payment, PaymentError};
use crate::ports::PaymentRepositoryPort;

/// 内存支付仓储适配器 (测试与本地运行双体)
#[derive(Debug, Default)]
pub struct InMemoryPaymentRepository {
    payments: RwLock<HashMap<PaymentId, Payment>>,
    order_index: RwLock<HashMap<OrderId, PaymentId>>,
}

impl InMemoryPaymentRepository {
    /// 构造新的内存仓储实例
    #[must_use]
    pub fn new() -> Self {
        Self {
            payments: RwLock::new(HashMap::new()),
            order_index: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PaymentRepositoryPort for InMemoryPaymentRepository {
    async fn save(&self, payment: &Payment) -> Result<(), PaymentError> {
        let mut payments = self
            .payments
            .write()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;
        let mut order_index = self
            .order_index
            .write()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        payments.insert(*payment.id(), payment.clone());
        order_index.insert(*payment.order_id(), *payment.id());
        Ok(())
    }

    async fn find_by_id(&self, id: &PaymentId) -> Result<Option<Payment>, PaymentError> {
        let payments = self
            .payments
            .read()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;
        Ok(payments.get(id).cloned())
    }

    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Option<Payment>, PaymentError> {
        let payment_id = {
            let order_index = self
                .order_index
                .read()
                .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;
            order_index.get(order_id).copied()
        };
        match payment_id {
            Some(id) => self.find_by_id(&id).await,
            None => Ok(None),
        }
    }
}
