use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("TAP_CONFIG").ok())
        .unwrap_or_else(|| "config.yaml".into());
    if let Err(e) = server::run(
        "0.0.0.0".to_string(),
        "8080".to_string(),
        PathBuf::from(config),
    )
    .await
    {
        eprintln!("{e}");
        std::process::exit(1);
    }
    Ok(())
}
