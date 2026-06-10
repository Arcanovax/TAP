use mini_redis::{Result, client};

// pub mod structures;
// pub mod enums;

#[tokio::main]
async fn main() -> Result<()> {
	let mut client = client::connect("127.0.0.1:6380").await?;
	client.set("hello", "world".into()).await?;
	let result = client.get("hello").await?;
	println!("got value from the server; result={:?}", result.unwrap());

    Ok(())

}