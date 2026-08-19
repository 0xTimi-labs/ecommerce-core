use serde::{Deserialize, Serialize};
use shared_kernel::{CustomerId, Money, OrderId, SkuId};

/// 创建订单请求 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderDto {
    /// 客户标识
    pub customer_id: CustomerId,
    /// 订单项列表
    pub items: Vec<OrderItemDto>,
}

/// 订单项 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemDto {
    /// SKU 标识
    pub sku_id: SkuId,
    /// 数量
    pub quantity: u32,
    /// 单价
    pub unit_price: Money,
}

/// 订单响应 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDto {
    /// 订单标识
    pub order_id: OrderId,
    /// 客户标识
    pub customer_id: CustomerId,
    /// 订单状态
    pub status: String,
    /// 订单项列表
    pub items: Vec<OrderItemDto>,
    /// 订单总金额
    pub total_amount: Money,
}
