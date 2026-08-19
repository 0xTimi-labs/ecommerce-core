use async_trait::async_trait;
use wire_contracts::payment::v1::{
    AuthorizeRequest, AuthorizeResponse, CaptureRequest, CaptureResponse, RefundRequest,
    RefundResponse, VoidAuthorizationRequest, VoidAuthorizationResponse,
};

/// 支付开放服务契约（基于 Wire Protobuf 契约）
#[async_trait]
pub trait PaymentService: Send + Sync {
    /// 支付授权
    async fn authorize(&self, req: AuthorizeRequest) -> Result<AuthorizeResponse, String>;
    /// 支付请款
    async fn capture(&self, req: CaptureRequest) -> Result<CaptureResponse, String>;
    /// 撤销授权
    async fn void_authorization(
        &self,
        req: VoidAuthorizationRequest,
    ) -> Result<VoidAuthorizationResponse, String>;
    /// 退款
    async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, String>;
}
