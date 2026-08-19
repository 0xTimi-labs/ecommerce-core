use thiserror::Error;

/// 跨上下文共享领域基础错误
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    /// 基础数据校验失败
    #[error("数据校验错误: {0}")]
    ValidationError(String),

    /// 目标实体不存在
    #[error("未找到对应实体: {0}")]
    NotFound(String),

    /// 非法状态机流转
    #[error("非法状态流转: 无法从 {from} 流转至 {to}")]
    InvalidStateTransition {
        /// 当前源状态
        from: &'static str,
        /// 目标状态
        to: &'static str,
    },

    /// 领域不变量冲突
    #[error("领域不变量违背: {0}")]
    InvariantViolation(String),
}
