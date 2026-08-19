use thiserror::Error;

/// 库存 API 边界业务错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum InventoryApiError {
    #[error("业务参数校验失败: {0}")]
    InvalidArgument(String),

    #[error("库存预留记录未找到: {0}")]
    NotFound(String),

    #[error("商品库存不足: {0}")]
    InsufficientStock(String),

    #[error("内部服务异常: {0}")]
    Internal(String),
}
