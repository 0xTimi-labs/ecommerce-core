use thiserror::Error;

/// 订单上下文领域业务错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OrderingError {
    /// 目标功能尚未实现
    #[error("功能尚未实现: {0}")]
    NotImplemented(&'static str),

    /// 订单业务校验失败
    #[error("订单业务校验失败: {0}")]
    ValidationError(String),

    /// 目标订单不存在
    #[error("未找到指定订单: {0}")]
    NotFound(String),

    /// 非法订单状态流转
    #[error("订单状态流转非法: 无法从 {from:?} 流转至 {to:?}")]
    InvalidStateTransition {
        /// 当前源状态
        from: super::order::OrderStatus,
        /// 期望流转的目标状态
        to: super::order::OrderStatus,
    },

    /// 订单商品项为空
    #[error("订单不能为空: 必须包含至少一件有效商品")]
    EmptyOrder,
}
