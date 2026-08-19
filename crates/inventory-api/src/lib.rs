//! 库存上下文公共契约

pub mod service;

pub use service::InventoryService;
pub use wire_contracts::inventory::v1::{
    ReleaseRequest, ReleaseResponse, ReservationLine, ReservationStatus, ReserveRequest,
    ReserveResponse,
};
