#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tap::run("127.0.0.1".to_string(), "8080".to_string()).await
}
