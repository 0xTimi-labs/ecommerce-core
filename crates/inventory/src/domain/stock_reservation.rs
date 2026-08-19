use serde::{Deserialize, Serialize};
use shared_kernel::{OrderId, ReservationId, SkuId};

use super::errors::InventoryError;

/// 预留商品项
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockItem {
    pub sku_id: SkuId,
    pub quantity: u32,
}

impl StockItem {
    #[must_use]
    pub fn new(sku_id: SkuId, quantity: u32) -> Self {
        Self { sku_id, quantity }
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
    /// 预留失败（缺货）
    Rejected,
}

/// 库存预留聚合根 (StockReservation Aggregate Root)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StockReservation {
    id: ReservationId,
    order_id: OrderId,
    items: Vec<StockItem>,
    status: ReservationStatus,
}

impl StockReservation {
    /// 创建新的库存预留记录
    #[must_use]
    pub fn new(order_id: OrderId, items: Vec<StockItem>) -> Self {
        Self {
            id: ReservationId::new(),
            order_id,
            items,
            status: ReservationStatus::Pending,
        }
    }

    /// 确认锁定库存
    pub fn confirm(&mut self) -> Result<(), InventoryError> {
        if self.status != ReservationStatus::Pending {
            return Err(InventoryError::InvalidStateTransition {
                current: "非 Pending 状态",
                action: "confirm",
            });
        }
        self.status = ReservationStatus::Reserved;
        Ok(())
    }

    /// 释放预留库存
    pub fn release(&mut self) -> Result<(), InventoryError> {
        if self.status != ReservationStatus::Reserved {
            return Err(InventoryError::InvalidStateTransition {
                current: "非 Reserved 状态",
                action: "release",
            });
        }
        self.status = ReservationStatus::Released;
        Ok(())
    }

    /// 拒绝预留（缺货）
    pub fn reject(&mut self) -> Result<(), InventoryError> {
        if self.status != ReservationStatus::Pending {
            return Err(InventoryError::InvalidStateTransition {
                current: "非 Pending 状态",
                action: "reject",
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
