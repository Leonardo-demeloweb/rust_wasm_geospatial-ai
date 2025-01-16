use std::fs;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry};

/// Normalize a GeoJSON file and save the result to a new file
pub fn normalize_geojson(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the input GeoJSON file
    let geojson_str = fs::read_to_string(input_path)?;
    let geojson: GeoJson = geojson_str.parse()?;

    // Ensure it's a FeatureCollection
    if let GeoJson::FeatureCollection(collection) = geojson {
        let normalized_collection = normalize_feature_collection(collection)?;

        // Write the normalized GeoJSON to the output file
        let normalized_geojson = GeoJson::FeatureCollection(normalized_collection);
        fs::write(output_path, normalized_geojson.to_string())?;
        println!("Normalized GeoJSON written to {}", output_path);
    } else {
        println!("The input file is not a FeatureCollection");
    }

    Ok(())
}

/// Normalize a FeatureCollection
fn normalize_feature_collection(
    collection: FeatureCollection,
) -> Result<FeatureCollection, Box<dyn std::error::Error>> {
    let mut normalized_features = Vec::new();

    for feature in collection.features {
        // Normalize each feature's geometry and properties
        if let Some(geometry) = feature.geometry {
            let normalized_geometry = normalize_geometry(&geometry)?;
            let normalized_feature = Feature {
                geometry: Some(normalized_geometry),
                properties: feature.properties,
                bbox: feature.bbox,
                id: feature.id,
                foreign_members: feature.foreign_members,
            };
            normalized_features.push(normalized_feature);
        } else {
            println!("Feature skipped due to missing geometry");
        }
    }

    Ok(FeatureCollection {
        features: normalized_features,
        bbox: collection.bbox,
        foreign_members: collection.foreign_members,
    })
}

/// Normalize a Geometry (e.g., ensure polygons are closed)
fn normalize_geometry(geometry: &Geometry) -> Result<Geometry, Box<dyn std::error::Error>> {
    if let geojson::Value::Polygon(coords) = &geometry.value {
        let normalized_coords = coords
            .iter()
            .map(|ring| {
                let mut normalized_ring = ring.clone();
                if !ring.is_empty() && ring[0] != *ring.last().unwrap() {
                    // Ensure the ring is closed
                    normalized_ring.push(ring[0].clone());
                }
                normalized_ring
            })
            .collect();
        return Ok(Geometry::new(geojson::Value::Polygon(normalized_coords)));
    }

    // For other geometry types, return as-is
    Ok(geometry.clone())
}
