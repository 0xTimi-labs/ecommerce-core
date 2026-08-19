//! 由 Buf 从 `contracts/proto` 生成的 wire 类型与 Tonic gRPC 服务绑定；不得手写。
//!
//! Buf 生成的公开 API 自带上游 lint 选择；仅在生成模块局部豁免 Clippy，手写骨架仍由 workspace lint 检查。

pub mod inventory {
    #[allow(clippy::all, clippy::pedantic, clippy::nursery, unused_qualifications)]
    pub mod v1 {
        include!("generated/inventory/v1/inventory.v1.rs");
    }
}

pub mod ordering {
    #[allow(clippy::all, clippy::pedantic, clippy::nursery, unused_qualifications)]
    pub mod v1 {
        include!("generated/ordering/v1/ordering.v1.rs");
    }
}

pub mod payment {
    #[allow(clippy::all, clippy::pedantic, clippy::nursery, unused_qualifications)]
    pub mod v1 {
        include!("generated/payment/v1/payment.v1.rs");
    }
}
