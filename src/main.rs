use rust_yaml::Yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = Yaml::new();

    // Parse YAML from a string
    let yaml_content = r#"
        name: "rust-yaml"
        version: "1.1.0"
        features:
          - fast
          - safe
          - reliable
        config:
          debug: true
          max_depth: 100
    "#;

    let parsed = yaml.load_str(yaml_content)?;
    println!("Parsed: {:#?}", parsed);

    // Dump back to YAML
    let output = yaml.dump_str(&parsed)?;
    println!("Output:\n{}", output);

    Ok(())
}
