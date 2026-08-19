//! 库存履约上下文公共契约

pub mod dtos;
pub mod service;

pub use dtos::{ReserveStockDto, ReserveStockItemDto, StockReservationDto};
pub use service::InventoryService;
