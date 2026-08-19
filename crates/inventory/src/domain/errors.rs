use shared_kernel::ReservationId;
use thiserror::Error;

use super::stock_reservation::ReservationStatus;

/// 库存领域业务错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum InventoryError {
    /// 目标功能尚未实现
    #[error("功能尚未实现: {0}")]
    NotImplemented(&'static str),

    /// 非法库存状态流转
    #[error("库存状态流转非法: 无法从 {from:?} 流转至 {to:?}")]
    InvalidStateTransition {
        /// 当前源状态
        from: ReservationStatus,
        /// 期望流转的目标状态
        to: ReservationStatus,
    },

    /// 校验失败
    #[error("库存业务校验失败: {0}")]
    ValidationError(String),

    /// 预留商品列表为空
    #[error("库存预留项不能为空")]
    EmptyReservation,

    /// 商品库存不足
    #[error("商品 [{sku_id}] 库存不足: 请求 [{requested}], 当前可用 [{available}]")]
    InsufficientStock {
        sku_id: String,
        requested: u32,
        available: u32,
    },

    /// 库存预留记录未找到
    #[error("库存预留记录未找到: {0}")]
    NotFound(ReservationId),

    /// 仓储读写错误
    #[error("库存仓储读写错误: {0}")]
    RepositoryError(String),
}
