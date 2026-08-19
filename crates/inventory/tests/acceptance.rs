use cucumber::World;

#[derive(Debug, Default, World)]
pub struct InventoryWorld;

#[tokio::main]
async fn main() {
    InventoryWorld::run("tests/features").await;
}
