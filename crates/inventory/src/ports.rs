use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::ReserveStockRequest;

/// 库存仓储端口
#[async_trait]
pub trait InventoryRepositoryPort: Send + Sync {
    /// 预留库存
    async fn reserve(&self, req: &ReserveStockRequest) -> Result<Uuid, String>;
    /// 释放预留库存
    async fn release(&self, reservation_id: &Uuid) -> Result<(), String>;
}
