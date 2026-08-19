Feature: 订单创建与校验管理

  作为电商客户
  我希望能够提交包含有效商品的订单并由服务端计算金额
  以便于安全可靠地开启交易流程

  @ignore @scenario-order-create-valid
  Scenario: 客户成功创建包含有效商品的订单
    Given 客户 "cust-001" 挑选了商品 "sku-apple-1"，数量为 2
    And 服务端权威价格引用报价快照为单价 5000 分
    When 客户提交创建订单请求
    Then 订单应成功创建，初始状态为 "Placed"
    And 订单总金额应为 10000 分

  @ignore @scenario-order-create-empty-items
  Scenario: 客户提交空商品列表时创建订单失败
    Given 客户 "cust-001" 提交了空的商品列表
    When 客户尝试创建订单
    Then 系统应拒绝创建订单，并提示空商品列表错误
