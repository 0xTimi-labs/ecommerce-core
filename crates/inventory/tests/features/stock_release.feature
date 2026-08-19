Feature: 商品库存释放与补偿管理

  作为电商仓储系统
  我希望能够对取消订单或超时的库存预留进行原子释放
  以便于保证物理可用库存的准确回归

  @ignore @scenario-inventory-release-success
  Scenario: 订单取消后成功释放已预留库存
    Given 存在已预留凭据 "res-5001"，包含 2 件 "sku-apple-1"
    When 交易补偿系统发起库存释放请求
    Then 库存预留状态应流转为 "Released"
    And 商品 "sku-apple-1" 的可用物理库存应恢复 2 件
