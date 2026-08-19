use shared_kernel::{OrderId, SkuId};

/// 库存项
#[derive(Debug, Clone)]
pub struct StockItem {
    /// SKU 标识
    pub sku_id: SkuId,
    /// 数量
    pub quantity: u32,
}

/// 库存预留请求
#[derive(Debug, Clone)]
pub struct ReserveStockRequest {
    /// 订单标识
    pub order_id: OrderId,
    /// 预留库存项列表
    pub items: Vec<StockItem>,
}
