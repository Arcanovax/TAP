use mini_redis::{Result, client};

// pub mod structures;
// pub mod enums;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run("127.0.0.1".to_string(), "8080".to_string()).await
}
