use async_trait::async_trait;
use shared_kernel::{Money, OrderId};
use uuid::Uuid;

use crate::domain::{PaymentRequest, PaymentStatus};

/// 支付网关端口
#[async_trait]
pub trait PaymentGatewayPort: Send + Sync {
    /// 发起支付授权
    async fn authorize(&self, order_id: &OrderId, amount: Money) -> Result<Uuid, String>;
    /// 结算支付
    async fn capture(&self, payment_id: &Uuid) -> Result<(), String>;
}

/// 支付仓储端口
#[async_trait]
pub trait PaymentRepositoryPort: Send + Sync {
    /// 保存支付记录
    async fn save(&self, req: &PaymentRequest, status: PaymentStatus) -> Result<(), String>;
    /// 查询支付状态
    async fn find_by_order_id(&self, order_id: &OrderId) -> Result<Option<PaymentStatus>, String>;
}
