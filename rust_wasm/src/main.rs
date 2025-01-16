mod normalization;

use normalization::normalize_geojson;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "/Users/leonardodemelo/rust-wasm-ai-predictive/backend/data/aquifer.geojson";
    let output_path = "/Users/leonardodemelo/rust-wasm-ai-predictive/backend/data/aquifer_normalized.geojson";

    println!("Starting normalization process...");
    normalize_geojson(input_path, output_path)?;
    println!("Normalization process completed.");

    Ok(())
}
