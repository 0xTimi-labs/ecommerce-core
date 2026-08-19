use async_trait::async_trait;
use wire_contracts::inventory::v1::{
    ReleaseRequest, ReleaseResponse, ReserveRequest, ReserveResponse,
};

use crate::errors::InventoryApiError;

/// 库存上下文公共应用服务接口契约
#[async_trait]
pub trait InventoryApiService: Send + Sync {
    /// 处理预留库存请求
    async fn reserve_stock(
        &self,
        req: ReserveRequest,
    ) -> Result<ReserveResponse, InventoryApiError>;

    /// 处理释放库存请求
    async fn release_stock(
        &self,
        req: ReleaseRequest,
    ) -> Result<ReleaseResponse, InventoryApiError>;
}
