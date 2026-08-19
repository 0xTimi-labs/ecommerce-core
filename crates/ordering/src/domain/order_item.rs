use serde::{Deserialize, Serialize};
use shared_kernel::{Money, SkuId};

use super::errors::OrderingError;

/// 订单商品行项实体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderItem {
    sku_id: SkuId,
    quantity: u32,
    unit_price: Money,
}

impl OrderItem {
    /// 构造新的订单商品行项
    pub fn new(sku_id: SkuId, quantity: u32, unit_price: Money) -> Result<Self, OrderingError> {
        if quantity == 0 {
            return Err(OrderingError::ValidationError(
                "商品行项数量必须大于 0".to_string(),
            ));
        }
        Ok(Self {
            sku_id,
            quantity,
            unit_price,
        })
    }

    #[must_use]
    pub const fn sku_id(&self) -> &SkuId {
        &self.sku_id
    }

    #[must_use]
    pub const fn quantity(&self) -> u32 {
        self.quantity
    }

    #[must_use]
    pub const fn unit_price(&self) -> Money {
        self.unit_price
    }

    /// 计算当前行项小计金额
    pub fn subtotal(&self) -> Result<Money, OrderingError> {
        self.unit_price
            .checked_mul(self.quantity)
            .map_err(|e| OrderingError::ValidationError(format!("计算行项金额溢出: {e}")))
    }
}
