pub mod errors;
pub mod service;

pub use errors::InventoryApiError;
pub use service::InventoryApiService;
pub use wire_contracts::inventory::v1::{
    ReleaseRequest, ReleaseResponse, ReservationLine, ReservationStatus, ReserveRequest,
    ReserveResponse,
};
