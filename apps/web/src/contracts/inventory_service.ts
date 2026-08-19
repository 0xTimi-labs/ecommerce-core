/**
 * 预留库存项
 */
export interface StockItemInput {
  /** SKU 标识 */
  skuId: string;
  /** 数量 */
  quantity: number;
}

/**
 * 预留库存请求
 */
export interface ReserveStockRequest {
  /** 订单标识 */
  orderId: string;
  /** 预留商品列表 */
  items: StockItemInput[];
}

/**
 * 预留库存响应
 */
export interface ReserveStockResponse {
  /** 预留标识 */
  reservationId: string;
  /** 预留状态 */
  status: string;
}

/**
 * 释放库存预留请求
 */
export interface ReleaseStockRequest {
  /** 预留标识 */
  reservationId: string;
}

/**
 * 释放库存预留响应
 */
export interface ReleaseStockResponse {
  /** 预留标识 */
  reservationId: string;
  /** 预留状态 */
  status: string;
}
