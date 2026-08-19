Feature: 订单全生命周期管理

  作为电商客户
  我希望能够创建订单、完成支付、履约并跟踪状态
  以便于安全可靠地购买所需商品

  @ignore
  Scenario: 客户成功创建包含有效商品的订单
    Given 客户 "cust-001" 挑选了商品 "sku-apple-1"，单价为 5000 分，数量为 2
    When 客户提交创建订单请求
    Then 订单应成功创建，初始状态为 "Placed"
    And 订单总金额应为 10000 分

  @ignore
  Scenario: 客户提交空商品列表时创建订单失败
    Given 客户 "cust-001" 提交了空的商品列表
    When 客户尝试创建订单
    Then 系统应拒绝创建订单，并提示 "EmptyOrder" 错误

  @ignore
  Scenario: 订单支付成功后状态流转为已支付
    Given 已存在一个处于 "Placed" 状态的订单 "order-1001"
    When 收到支付成功的通知
    Then 订单状态应更新为 "Paid"

  @ignore
  Scenario: 处于已履约状态的订单不可取消
    Given 已存在一个处于 "Fulfilled" 状态的订单 "order-2001"
    When 客户尝试取消该订单
    Then 系统应拒绝取消操作，并提示状态流转非法
