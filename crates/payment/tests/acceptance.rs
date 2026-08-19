use cucumber::World;

#[derive(Debug, Default, World)]
pub struct PaymentWorld;

#[tokio::main]
async fn main() {
    PaymentWorld::run("tests/features").await;
}
