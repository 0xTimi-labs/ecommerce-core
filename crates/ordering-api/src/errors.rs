use thiserror::Error;

/// 订单 API 边界业务错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OrderingApiError {
    #[error("业务参数校验失败: {0}")]
    InvalidArgument(String),

    #[error("订单未找到: {0}")]
    NotFound(String),

    #[error("内部服务异常: {0}")]
    Internal(String),
}
