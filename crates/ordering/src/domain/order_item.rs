use serde::{Deserialize, Serialize};
use shared_kernel::{Money, SkuId};

/// 订单项实体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderItem {
    /// SKU 标识
    pub sku_id: SkuId,
    /// 数量
    pub quantity: u32,
    /// 单价
    pub unit_price: Money,
}
