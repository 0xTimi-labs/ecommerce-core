pub mod errors;
pub mod service;

pub use errors::OrderingApiError;
pub use service::OrderingApiService;
pub use wire_contracts::ordering::v1::{
    CreateOrderRequest, CreateOrderResponse, GetOrderRequest, GetOrderResponse, OrderLineInput,
    OrderStatus, QuoteReference,
};
