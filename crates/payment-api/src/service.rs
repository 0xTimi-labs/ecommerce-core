use async_trait::async_trait;
use uuid::Uuid;

use crate::dtos::{AuthorizePaymentDto, PaymentDto};

/// 支付开放服务契约
#[async_trait]
pub trait PaymentService: Send + Sync {
    /// 授权支付
    async fn authorize(&self, req: AuthorizePaymentDto) -> Result<PaymentDto, String>;
    /// 结算支付
    async fn capture(&self, payment_id: &Uuid) -> Result<PaymentDto, String>;
}
