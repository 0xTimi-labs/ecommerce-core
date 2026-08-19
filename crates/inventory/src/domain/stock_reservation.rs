use shared_kernel::{OrderId, SkuId};

/// 库存预留请求
#[derive(Debug, Clone)]
pub struct ReserveStockRequest {
    /// 订单标识
    pub order_id: OrderId,
    /// SKU 标识
    pub sku_id: SkuId,
    /// 预留数量
    pub quantity: u32,
}
