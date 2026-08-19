/**
 * 授权支付请求
 */
export interface AuthorizePaymentRequest {
  /** 订单标识 */
  orderId: string;
  /** 客户标识 */
  customerId: string;
  /** 支付金额（分） */
  amountCents: bigint;
  /** 币种 */
  currency: string;
}

/**
 * 授权支付响应
 */
export interface AuthorizePaymentResponse {
  /** 支付标识 */
  paymentId: string;
  /** 支付状态 */
  status: string;
}

/**
 * 结算支付请求
 */
export interface CapturePaymentRequest {
  /** 支付标识 */
  paymentId: string;
}

/**
 * 结算支付响应
 */
export interface CapturePaymentResponse {
  /** 支付标识 */
  paymentId: string;
  /** 支付状态 */
  status: string;
}
