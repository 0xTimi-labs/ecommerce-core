Feature: 支付预授权管理

  作为电商支付系统
  我希望能够安全冻结客户支付额度
  以便于在履约前确保交易具备履约资金保障

  @ignore @scenario-payment-auth-001
  Scenario: 客户成功进行订单支付预授权
    Given 存在待支付的有效订单 "order-3001"，金额为 10000 分
    When 客户发起 10000 分的支付授权请求
    Then 支付状态应流转为 "Authorized"
    And 系统应记录对应的授权凭据 "auth-3001"
