use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the YAML file
    let yaml_content = fs::read_to_string("schema.yaml")?;

    // Parse YAML and convert to JSON
    let value: Value = serde_yaml::from_str(&yaml_content)?;
    let json_content = serde_json::to_string_pretty(&value)?;

    // Write to JSON file
    fs::write("schema.json", json_content)?;

    println!("Schema converted from YAML to JSON successfully!");
    Ok(())
}
