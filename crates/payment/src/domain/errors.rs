use shared_kernel::PaymentId;
use thiserror::Error;

use super::payment::PaymentStatus;

/// 支付领域业务错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PaymentError {
    /// 目标功能尚未实现
    #[error("功能尚未实现: {0}")]
    NotImplemented(&'static str),

    /// 非法支付状态流转
    #[error("支付状态流转非法: 无法从 {from:?} 流转至 {to:?}")]
    InvalidStateTransition {
        /// 当前源状态
        from: PaymentStatus,
        /// 期望流转的目标状态
        to: PaymentStatus,
    },

    /// 支付金额非法
    #[error("支付金额非法: {0}")]
    InvalidAmount(String),

    /// 支付单未找到
    #[error("未找到指定支付单: {0}")]
    NotFound(PaymentId),

    /// 支付授权失败
    #[error("支付授权失败: {0}")]
    AuthorizationFailed(String),

    /// 支付请款失败
    #[error("支付请款失败: {0}")]
    CaptureFailed(String),

    /// 仓储读写错误
    #[error("支付仓储读写错误: {0}")]
    RepositoryError(String),
}
