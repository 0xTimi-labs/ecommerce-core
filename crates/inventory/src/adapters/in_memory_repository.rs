use async_trait::async_trait;
use shared_kernel::{OrderId, ReservationId};
use std::collections::HashMap;
use std::sync::RwLock;

use crate::domain::{InventoryError, StockReservation};
use crate::ports::InventoryRepositoryPort;

/// 内存库存仓储适配器 (测试与本地运行双体)
#[derive(Debug, Default)]
pub struct InMemoryInventoryRepository {
    reservations: RwLock<HashMap<ReservationId, StockReservation>>,
    order_index: RwLock<HashMap<OrderId, ReservationId>>,
}

impl InMemoryInventoryRepository {
    /// 构造新的内存仓储实例
    #[must_use]
    pub fn new() -> Self {
        Self {
            reservations: RwLock::new(HashMap::new()),
            order_index: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl InventoryRepositoryPort for InMemoryInventoryRepository {
    async fn save(&self, reservation: &StockReservation) -> Result<(), InventoryError> {
        let mut reservations = self
            .reservations
            .write()
            .map_err(|e| InventoryError::RepositoryError(e.to_string()))?;
        let mut order_index = self
            .order_index
            .write()
            .map_err(|e| InventoryError::RepositoryError(e.to_string()))?;

        reservations.insert(*reservation.id(), reservation.clone());
        order_index.insert(*reservation.order_id(), *reservation.id());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ReservationId,
    ) -> Result<Option<StockReservation>, InventoryError> {
        let reservations = self
            .reservations
            .read()
            .map_err(|e| InventoryError::RepositoryError(e.to_string()))?;
        Ok(reservations.get(id).cloned())
    }

    async fn find_by_order_id(
        &self,
        order_id: &OrderId,
    ) -> Result<Option<StockReservation>, InventoryError> {
        let reservation_id = {
            let order_index = self
                .order_index
                .read()
                .map_err(|e| InventoryError::RepositoryError(e.to_string()))?;
            order_index.get(order_id).copied()
        };
        match reservation_id {
            Some(id) => self.find_by_id(&id).await,
            None => Ok(None),
        }
    }
}
