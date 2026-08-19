use async_trait::async_trait;
use shared_kernel::{OrderId, ReservationId};

use crate::domain::{InventoryError, StockReservation};
use crate::ports::InventoryRepositoryPort;

/// 内存库存仓储适配器
#[derive(Debug, Default, Clone)]
pub struct InMemoryInventoryRepository;

impl InMemoryInventoryRepository {
    /// 构造新的内存仓储实例
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl InventoryRepositoryPort for InMemoryInventoryRepository {
    async fn save(&self, _reservation: &StockReservation) -> Result<(), InventoryError> {
        Err(InventoryError::NotImplemented(
            "InMemoryInventoryRepository::save",
        ))
    }

    async fn find_by_id(
        &self,
        _id: &ReservationId,
    ) -> Result<Option<StockReservation>, InventoryError> {
        Err(InventoryError::NotImplemented(
            "InMemoryInventoryRepository::find_by_id",
        ))
    }

    async fn find_by_order_id(
        &self,
        _order_id: &OrderId,
    ) -> Result<Option<StockReservation>, InventoryError> {
        Err(InventoryError::NotImplemented(
            "InMemoryInventoryRepository::find_by_order_id",
        ))
    }
}
