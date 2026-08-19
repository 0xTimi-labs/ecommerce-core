use cucumber::World;

/// 订单上下文 BDD 验收测试世界上下文
#[derive(Debug, Default, World)]
pub struct OrderingWorld;

#[tokio::main]
async fn main() {
    OrderingWorld::run("tests/features").await;
}
