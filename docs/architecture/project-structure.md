# 项目结构

> 本结构为架构模式参考规范，用于指导模块与垂直切片组织，非物理目录的 1:1 实时映射，日常开发无需维护。

## 目录结构

```text
context-map.md
docs/
├── VISION.md                           # 产品愿景
├── architecture/                       # 架构设计与工程规范
│   ├── ARCHITECTURE.md
│   └── project-structure.md
└── adr/

contracts/                              # 契约
├── buf.yaml / buf.gen.yaml             # Buf 规范与代码生成配置
├── ordering/                           # Protobuf 契约
│   └── v1/order_service.proto
└── events/                             # Event Schema
    ├── order_placed.v1.json
    └── stock_reserved.v1.json 

crates/
├── shared-kernel/                      # 共享内核
├── ordering-api/
├── ordering/
│   ├── context.md                      # 领域专属上下文
│   ├── src/
│   │   ├── domain/                     # 领域模型
│   │   ├── ports/                      # 端口
│   │   ├── adapters/                   # 适配器
│   │   └── features/                   # 垂直切片
│   │       ├── <command_feature>/ 
│   │       │   ├── command.rs          # 命令
│   │       │   ├── handler.rs          # 处理器
│   │       │   ├── validator.rs        # 校验规则
│   │       │   ├── response.rs         # 响应
│   │       │   └── tests.rs            # 单元测试
│   │       └── <query_feature>/ 
│   │           ├── query.rs            # 查询
│   │           ├── handler.rs          # 处理器
│   │           ├── response.rs         # 响应
│   │           └── tests.rs            # 单元测试
│   └── tests/
│       ├── features/                   # 验收测试规格
│       └── acceptance.rs               # 验收测试
├── ordering-test-support/
├── payment/
├── payment-api/

apps/
└── web/
    ├── biome.json 
    ├── package.json
    ├── tsconfig.json
    └── src/
        ├── contracts/                  # Web 契约
        └── index.ts

.github/
├── ISSUE_TEMPLATE/                     # Issue 模板
├── PULL_REQUEST_TEMPLATE/              # PR 模板
└── workflows/
```

> [!NOTE]
> `*-test-support` 为跨限界上下文测试替身，仅在存在跨模块集成测试诉求时按需创建。
