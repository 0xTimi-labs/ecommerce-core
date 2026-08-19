use async_trait::async_trait;
use wire_contracts::inventory::v1::{
    ReleaseRequest, ReleaseResponse, ReserveRequest, ReserveResponse,
};

/// 库存开放服务契约（基于 Wire Protobuf 契约）
#[async_trait]
pub trait InventoryService: Send + Sync {
    /// 预留库存
    async fn reserve(&self, req: ReserveRequest) -> Result<ReserveResponse, String>;
    /// 释放库存
    async fn release(&self, req: ReleaseRequest) -> Result<ReleaseResponse, String>;
}
