# 采用 Protobuf 作为跨端与跨服务唯一契约

前后端通信与跨服务交互以 `contracts/` 下的 `.proto` 为 Single Source of Truth，并通过 `buf` 自动生成 Rust 服务桩与 TypeScript Client，杜绝手动维护文档导致的契约漂移。
