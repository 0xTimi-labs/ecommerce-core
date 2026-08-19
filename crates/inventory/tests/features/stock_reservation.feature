Feature: 商品库存预留管理

  作为电商仓储系统
  我希望能够在下单时锁定实物库存
  以便于杜绝高并发超卖风险

  @ignore @scenario-inventory-reserve-001
  Scenario: 充足库存下成功预留实物商品
    Given 商品 "sku-apple-1" 当前可用物理库存为 10 件
    When 订单系统请求预留 2 件 "sku-apple-1"
    Then 库存预留状态应为 "Reserved"
    And 商品 "sku-apple-1" 的可用物理库存应减少为 8 件
