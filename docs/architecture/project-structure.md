# 项目结构

> 本结构为架构模式参考规范，用于指导模块与垂直切片组织，非物理目录的 1:1 实时映射，日常开发无需维护。

## 目录结构

```text
context-map.md                          # 限界上下文映射关系
buf.yaml / buf.gen.yaml                 # Buf v2 契约治理与跨语言代码生成配置
docs/
├── vision.md                           # 产品愿景
├── architecture/                       # 架构设计与工程规范
│   ├── architecture.md
│   └── project-structure.md
└── adr/                                # 架构决策记录 (0001~0005)

contracts/                              # 跨语言契约单一事实源
├── proto/                              # Protobuf RPC 契约 (OHS)
│   ├── ordering/v1/ordering.proto
│   ├── payment/v1/payment.proto
│   └── inventory/v1/inventory.proto
└── events/                             # CloudEvents 1.0 事件 Schema (PL)
    ├── event-envelope.v1.schema.json
    ├── order-placed.v1.schema.json
    ├── order-cancelled.v1.schema.json
    ├── payment-authorized.v1.schema.json
    ├── payment-captured.v1.schema.json
    ├── payment-failed.v1.schema.json
    ├── stock-reserved.v1.schema.json
    ├── stock-released.v1.schema.json
    ├── stock-shortage.v1.schema.json
    └── fixtures/                       # valid / invalid 测试样本

crates/
├── wire-contracts/                     # Buf 自动生成的 Rust gRPC / Prost 契约绑定与事件模式集成测试
├── shared-kernel/                      # 共享内核 (OrderId, Money, CustomerId, SkuId 等值对象)
├── ordering-api/                       # 订单上下文公共应用服务契约
├── ordering/                           # 订单上下文 (DDD 六边形架构)
│   ├── context.md                      # 领域专属上下文说明
│   ├── src/
│   │   ├── domain/                     # 领域模型 (Order 充血聚合根、值对象、领域事件与错误)
│   │   ├── ports/                      # 端口 (进向 Driver 端口与出向 Driven 端口)
│   │   ├── adapters/                   # 适配器 (仓储桩、事件发布桩)
│   │   └── features/                   # 垂直切片功能实现
│   └── tests/
│       ├── features/                   # Cucumber BDD 验收规格 (@scenario-* 稳定 ID)
│       └── acceptance.rs               # BDD 集成测试入口
├── payment-api/                        # 支付上下文公共应用服务契约
├── payment/                            # 支付上下文
├── inventory-api/                      # 库存上下文公共应用服务契约
└── inventory/                          # 库存上下文

apps/
└── web/                                # 前端轻量工程 (Bun + Biome + TS)
    ├── biome.json 
    ├── package.json
    ├── tsconfig.json
    └── src/
        ├── generated/                  # Buf 自动生成的 TS 契约绑定
        └── index.ts

.github/
├── ISSUE_TEMPLATE/                     # GitHub Issue Forms (结构化表单)
├── PULL_REQUEST_TEMPLATE/              # PR 模板
└── workflows/                          # GitHub Actions CI 流水线
```

> [!NOTE]
> `*-test-support` 为跨限界上下文测试替身，仅在存在跨模块集成测试诉求时按需创建。
