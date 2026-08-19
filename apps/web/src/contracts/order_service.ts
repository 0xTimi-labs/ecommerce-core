/**
 * 订单项输入
 */
export interface OrderItemInput {
  /** SKU 标识 */
  skuId: string;
  /** 数量 */
  quantity: number;
  /** 单价（分） */
  unitPriceCents: bigint;
  /** 币种 */
  currency: string;
}

/**
 * 创建订单请求
 */
export interface CreateOrderRequest {
  /** 客户标识 */
  customerId: string;
  /** 订单项列表 */
  items: OrderItemInput[];
}

/**
 * 创建订单响应
 */
export interface CreateOrderResponse {
  /** 订单标识 */
  orderId: string;
  /** 订单状态 */
  status: string;
}

/**
 * 查询订单请求
 */
export interface GetOrderRequest {
  /** 订单标识 */
  orderId: string;
}

/**
 * 订单响应
 */
export interface GetOrderResponse {
  /** 订单标识 */
  orderId: string;
  /** 客户标识 */
  customerId: string;
  /** 订单状态 */
  status: string;
  /** 订单总金额（分） */
  totalAmountCents: bigint;
  /** 币种 */
  currency: string;
  /** 订单项列表 */
  items: OrderItemInput[];
}
