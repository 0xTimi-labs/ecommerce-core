use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::domain::ReserveStockRequest;
use crate::ports::InventoryRepositoryPort;

/// 内存库存仓储适配器 (测试与本地运行适配器)
#[derive(Debug, Default)]
pub struct InMemoryInventoryRepository {
    storage: RwLock<HashMap<Uuid, ReserveStockRequest>>,
}

impl InMemoryInventoryRepository {
    /// 构造新的内存仓储实例
    #[must_use]
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl InventoryRepositoryPort for InMemoryInventoryRepository {
    async fn reserve(&self, req: &ReserveStockRequest) -> Result<Uuid, String> {
        let mut storage = self
            .storage
            .write()
            .map_err(|_| "获取写锁失败".to_string())?;
        let reservation_id = Uuid::new_v4();
        storage.insert(reservation_id, req.clone());
        Ok(reservation_id)
    }

    async fn release(&self, reservation_id: &Uuid) -> Result<(), String> {
        let mut storage = self
            .storage
            .write()
            .map_err(|_| "获取写锁失败".to_string())?;
        storage.remove(reservation_id);
        Ok(())
    }
}
