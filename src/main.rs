mod structures;

use serde_yaml;
use crate::structures::global::{self, Global};

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let f = std::fs::File::open("config.yaml")?;
    let d: Global = serde_yaml::from_reader(f)?;
    println!("Read YAML string: {:?}", d);
    Ok(())
}
