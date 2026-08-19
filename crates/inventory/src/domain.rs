pub mod errors;
pub mod events;
pub mod stock_reservation;

pub use errors::InventoryError;
pub use events::{
    ReservedItem, StockReleasedEvent, StockReservedEvent, StockShortageEvent, UnavailableItem,
};
pub use stock_reservation::{ReservationStatus, StockItem, StockReservation};
