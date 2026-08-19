Feature: 订单取消与状态保护

  作为电商系统与客户
  我希望能够合理取消未支付订单并严格保护已完成订单
  以便于保证交易状态机的一致性

  @ignore @scenario-order-cancel-001
  Scenario: 待支付订单客户成功取消
    Given 已存在一个处于 "Placed" 状态的订单 "order-1001"
    When 客户请求取消该订单
    Then 订单状态应流转为 "Cancelled"

  @ignore @scenario-order-cancel-002
  Scenario: 处于已履约状态的订单不可取消
    Given 已存在一个处于 "Fulfilled" 状态的订单 "order-2001"
    When 客户尝试取消该订单
    Then 系统应拒绝取消操作，并提示状态流转非法
