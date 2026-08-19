use serde::{Deserialize, Serialize};
use shared_kernel::{OrderId, ReservationId, SkuId};

use super::errors::InventoryError;

/// 预留商品项实体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockItem {
    sku_id: SkuId,
    quantity: u32,
}

impl StockItem {
    /// 构造新的预留商品项
    pub fn new(sku_id: SkuId, quantity: u32) -> Result<Self, InventoryError> {
        if quantity == 0 {
            return Err(InventoryError::ValidationError(
                "预留商品数量必须大于 0".to_string(),
            ));
        }
        Ok(Self { sku_id, quantity })
    }

    #[must_use]
    pub const fn sku_id(&self) -> &SkuId {
        &self.sku_id
    }

    #[must_use]
    pub const fn quantity(&self) -> u32 {
        self.quantity
    }
}

/// 预留流转状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReservationStatus {
    /// 待锁定
    Pending,
    /// 已锁定预留
    Reserved,
    /// 已释放
    Released,
    /// 预留失败
    Rejected,
}

/// 库存预留聚合根
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockReservation {
    id: ReservationId,
    order_id: OrderId,
    items: Vec<StockItem>,
    status: ReservationStatus,
}

impl StockReservation {
    /// 创建新的库存预留记录并校验不变量
    pub fn new(order_id: OrderId, items: Vec<StockItem>) -> Result<Self, InventoryError> {
        if items.is_empty() {
            return Err(InventoryError::EmptyReservation);
        }
        Ok(Self {
            id: ReservationId::new(),
            order_id,
            items,
            status: ReservationStatus::Pending,
        })
    }

    /// 确认锁定库存
    pub fn confirm(&mut self) -> Result<(), InventoryError> {
        if self.status != ReservationStatus::Pending {
            return Err(InventoryError::InvalidStateTransition {
                from: self.status,
                to: ReservationStatus::Reserved,
            });
        }
        self.status = ReservationStatus::Reserved;
        Ok(())
    }

    /// 释放预留库存
    pub fn release(&mut self) -> Result<(), InventoryError> {
        if self.status != ReservationStatus::Reserved {
            return Err(InventoryError::InvalidStateTransition {
                from: self.status,
                to: ReservationStatus::Released,
            });
        }
        self.status = ReservationStatus::Released;
        Ok(())
    }

    /// 拒绝预留
    pub fn reject(&mut self) -> Result<(), InventoryError> {
        if self.status != ReservationStatus::Pending {
            return Err(InventoryError::InvalidStateTransition {
                from: self.status,
                to: ReservationStatus::Rejected,
            });
        }
        self.status = ReservationStatus::Rejected;
        Ok(())
    }

    #[must_use]
    pub const fn id(&self) -> &ReservationId {
        &self.id
    }

    #[must_use]
    pub const fn order_id(&self) -> &OrderId {
        &self.order_id
    }

    #[must_use]
    pub fn items(&self) -> &[StockItem] {
        &self.items
    }

    #[must_use]
    pub const fn status(&self) -> ReservationStatus {
        self.status
    }
}
