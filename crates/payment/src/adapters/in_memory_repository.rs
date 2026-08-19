use async_trait::async_trait;
use shared_kernel::OrderId;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::{PaymentRequest, PaymentStatus};
use crate::ports::PaymentRepositoryPort;

/// 内存支付仓储适配器 (测试与本地运行适配器)
#[derive(Debug, Default)]
pub struct InMemoryPaymentRepository {
    storage: RwLock<HashMap<OrderId, (PaymentRequest, PaymentStatus)>>,
}

impl InMemoryPaymentRepository {
    /// 构造新的内存仓储实例
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PaymentRepositoryPort for InMemoryPaymentRepository {
    async fn save(&self, req: &PaymentRequest, status: PaymentStatus) -> Result<(), String> {
        let mut storage = self
            .storage
            .write()
            .map_err(|_| "获取写锁失败".to_string())?;
        storage.insert(req.order_id, (req.clone(), status));
        Ok(())
    }

    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Option<PaymentStatus>, String> {
        let storage = self
            .storage
            .read()
            .map_err(|_| "获取读锁失败".to_string())?;
        Ok(storage.get(order_id).map(|(_, status)| *status))
    }
}
