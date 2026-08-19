use async_trait::async_trait;
use wire_contracts::payment::v1::{
    AuthorizeRequest, AuthorizeResponse, CaptureRequest, CaptureResponse, RefundRequest,
    RefundResponse, VoidAuthorizationRequest, VoidAuthorizationResponse,
};

use crate::errors::PaymentApiError;

/// 支付上下文公共应用服务接口契约
#[async_trait]
pub trait PaymentApiService: Send + Sync {
    /// 发起预授权
    async fn authorize(&self, req: AuthorizeRequest) -> Result<AuthorizeResponse, PaymentApiError>;

    /// 发起请款结算
    async fn capture(&self, req: CaptureRequest) -> Result<CaptureResponse, PaymentApiError>;

    /// 撤销预授权
    async fn void_authorization(
        &self,
        req: VoidAuthorizationRequest,
    ) -> Result<VoidAuthorizationResponse, PaymentApiError>;

    /// 发起交易退款
    async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, PaymentApiError>;
}
