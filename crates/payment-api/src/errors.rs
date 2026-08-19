use thiserror::Error;

/// 支付 API 边界业务错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PaymentApiError {
    #[error("业务参数校验失败: {0}")]
    InvalidArgument(String),

    #[error("支付单未找到: {0}")]
    NotFound(String),

    #[error("支付处理被通道拒绝: {0}")]
    PaymentDeclined(String),

    #[error("内部服务异常: {0}")]
    Internal(String),
}
