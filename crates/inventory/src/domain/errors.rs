use shared_kernel::ReservationId;
use thiserror::Error;

/// 库存领域错误枚举
#[derive(Debug, Error, PartialEq, Eq)]
pub enum InventoryError {
    #[error("非法状态流转：当前状态 [{current}] 不允许执行 [{action}]")]
    InvalidStateTransition {
        current: &'static str,
        action: &'static str,
    },

    #[error("商品 [{sku_id}] 库存不足: 请求 [{requested}], 当前可用 [{available}]")]
    InsufficientStock {
        sku_id: String,
        requested: u32,
        available: u32,
    },

    #[error("库存预留记录未找到: {0}")]
    NotFound(ReservationId),

    #[error("库存仓储读写错误: {0}")]
    RepositoryError(String),
}
