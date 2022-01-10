use mini_redis::{client, Result};
use std::time::Duration;

async fn hello() {
    println!("world");
    std::thread::sleep(Duration::from_secs(1));
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let op = hello();
    println!("hello");
    op.await;
    let mut c = client::connect("127.0.0.1:6379").await?;
    c.set("hello", "world".into()).await?;
    let result = c.get("hello").await?;
    println!("got value from the server result={:?}", result);

    Ok(())
}
