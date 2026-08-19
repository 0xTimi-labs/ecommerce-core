use shared_kernel::PaymentId;
use thiserror::Error;

/// 支付领域业务错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PaymentError {
    /// 目标功能尚未实现
    #[error("功能尚未实现: {0}")]
    NotImplemented(&'static str),

    #[error("非法状态流转：当前状态 [{current}] 不允许执行 [{action}]")]
    InvalidStateTransition {
        current: &'static str,
        action: &'static str,
    },

    #[error("支付单未找到: {0}")]
    NotFound(PaymentId),

    #[error("支付授权失败: {0}")]
    AuthorizationFailed(String),

    #[error("支付请款失败: {0}")]
    CaptureFailed(String),

    #[error("支付仓储读写错误: {0}")]
    RepositoryError(String),
}
