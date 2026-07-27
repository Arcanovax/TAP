#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = server::run("0.0.0.0".to_string(), "8080".to_string()).await {
        eprintln!("{e}");
        std::process::exit(1);
    }
    Ok(())
}
