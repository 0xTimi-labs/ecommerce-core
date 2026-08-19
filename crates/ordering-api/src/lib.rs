//! 订单上下文公共契约

pub mod service;

pub use service::OrderingService;
pub use wire_contracts::ordering::v1::{
    CreateOrderRequest, CreateOrderResponse, GetOrderRequest, GetOrderResponse, OrderLineInput,
    OrderStatus, QuoteReference,
};
