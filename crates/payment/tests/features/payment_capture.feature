Feature: 支付请款结算管理

  作为电商财务与结算系统
  我希望能够对已预授权的款项进行正式请款结算
  以便于完成实际资金转移并推进订单完成

  @ignore @scenario-payment-capture-001
  Scenario: 对已授权支付成功进行请款
    Given 存在已授权凭据 "auth-3001" 且金额为 10000 分
    When 履约系统发起请款请求
    Then 支付状态应流转为 "Captured"
    And 产生财务请款结算流水 "cap-3001"
