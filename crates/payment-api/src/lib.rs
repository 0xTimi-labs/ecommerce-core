//! 支付上下文公共契约

pub mod service;

pub use service::PaymentService;
pub use wire_contracts::payment::v1::{
    AuthorizationStatus, AuthorizeRequest, AuthorizeResponse, CaptureRequest, CaptureResponse,
    CaptureStatus, Money, RefundRequest, RefundResponse, RefundStatus, VoidAuthorizationRequest,
    VoidAuthorizationResponse,
};
