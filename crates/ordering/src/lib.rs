pub mod adapters;
pub mod domain;
pub mod ports;

pub use domain::{Order, OrderItem, OrderPlacedEvent, OrderStatus, OrderingError};
pub use ports::{CreateOrderPort, EventPublisherPort, GetOrderPort, OrderRepositoryPort};
