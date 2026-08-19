use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared_kernel::{CustomerId, Money, OrderId};
use uuid::Uuid;

use super::errors::OrderingError;
use super::events::OrderCancelledEvent;
use super::order_item::OrderItem;

/// 订单流转状态 (Saga 状态机与统一语言对齐)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 已下单（初始状态）
    Placed,
    /// 已支付
    Paid,
    /// 已履约出库
    Fulfilled,
    /// 已取消
    Cancelled,
}

/// 订单聚合根 (Order Aggregate Root)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    id: OrderId,
    customer_id: CustomerId,
    quote_id: String,
    quote_version: String,
    items: Vec<OrderItem>,
    total_amount: Money,
    status: OrderStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Order {
    /// 创建新订单并校验不变量（必须包含至少一项商品，自动汇总权威总金额）
    pub fn new(
        customer_id: CustomerId,
        quote_id: impl Into<String>,
        quote_version: impl Into<String>,
        items: Vec<OrderItem>,
    ) -> Result<Self, OrderingError> {
        let first_item = items.first().ok_or(OrderingError::EmptyOrder)?;
        let mut total = Money::zero(first_item.unit_price().currency());
        for item in &items {
            let subtotal = item.subtotal()?;
            total = total
                .checked_add(subtotal)
                .map_err(|e| OrderingError::ValidationError(format!("计算订单总金额失败: {e}")))?;
        }

        let now = Utc::now();
        Ok(Self {
            id: OrderId::new(),
            customer_id,
            quote_id: quote_id.into(),
            quote_version: quote_version.into(),
            items,
            total_amount: total,
            status: OrderStatus::Placed,
            created_at: now,
            updated_at: now,
        })
    }

    /// 推进订单状态至已支付
    pub fn pay(&mut self) -> Result<(), OrderingError> {
        if self.status != OrderStatus::Placed {
            return Err(OrderingError::InvalidStateTransition {
                from: self.status,
                to: OrderStatus::Paid,
            });
        }
        self.status = OrderStatus::Paid;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 推进订单状态至已履约
    pub fn fulfill(&mut self) -> Result<(), OrderingError> {
        if self.status != OrderStatus::Paid {
            return Err(OrderingError::InvalidStateTransition {
                from: self.status,
                to: OrderStatus::Fulfilled,
            });
        }
        self.status = OrderStatus::Fulfilled;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 取消订单并产生取消事件
    pub fn cancel(
        &mut self,
        reason: impl Into<String>,
    ) -> Result<OrderCancelledEvent, OrderingError> {
        if self.status == OrderStatus::Fulfilled || self.status == OrderStatus::Cancelled {
            return Err(OrderingError::InvalidStateTransition {
                from: self.status,
                to: OrderStatus::Cancelled,
            });
        }

        let reason_str = reason.into();
        self.status = OrderStatus::Cancelled;
        let now = Utc::now();
        self.updated_at = now;

        Ok(OrderCancelledEvent {
            event_id: Uuid::new_v4(),
            order_id: self.id,
            customer_id: self.customer_id.clone(),
            reason: reason_str,
            occurred_at: now,
        })
    }

    #[must_use]
    pub const fn id(&self) -> &OrderId {
        &self.id
    }

    #[must_use]
    pub const fn customer_id(&self) -> &CustomerId {
        &self.customer_id
    }

    #[must_use]
    pub fn quote_id(&self) -> &str {
        &self.quote_id
    }

    #[must_use]
    pub fn quote_version(&self) -> &str {
        &self.quote_version
    }

    #[must_use]
    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    #[must_use]
    pub const fn total_amount(&self) -> Money {
        self.total_amount
    }

    #[must_use]
    pub const fn status(&self) -> OrderStatus {
        self.status
    }

    #[must_use]
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    #[must_use]
    pub const fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
