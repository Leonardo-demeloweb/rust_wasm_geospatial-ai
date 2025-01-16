use geo::algorithm::contains::Contains; // Import the Contains trait
use geo::{point, Polygon};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

#[derive(Deserialize)]
pub struct RealWorldData {
    points: Vec<Point>,
    polygons: Vec<PolygonData>,
}

#[derive(Deserialize)]
pub struct Point {
    id: u32,
    longitude: f64,
    latitude: f64,
}

#[derive(Deserialize)]
pub struct PolygonData {
    id: u32,
    coordinates: Vec<PointData>,
}

#[derive(Deserialize)]
pub struct PointData {
    longitude: f64,
    latitude: f64,
}

#[derive(Serialize)]
pub struct PIPResult {
    point_id: u32,
    polygon_id: u32,
    inside: bool,
}

/// WASI-compatible entry point
#[no_mangle]
pub extern "C" fn point_in_polygon() {
    // Read input JSON from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    // Process the input and output the result
    let result = process_point_in_polygon(&input);
    println!("{}", result);
}

/// Core logic for point-in-polygon determination
pub fn process_point_in_polygon(json_data: &str) -> String {
    let data: RealWorldData = serde_json::from_str(json_data).expect("Invalid input format");

    let mut results = Vec::new();

    for polygon_data in &data.polygons {
        // Debugging: Print the polygon ID and coordinates
        eprintln!(
            "Processing Polygon ID {} with {} coordinates",
            polygon_data.id,
            polygon_data.coordinates.len()
        );

        let polygon_coords: Vec<_> = polygon_data
            .coordinates
            .iter()
            .map(|coord| (coord.longitude, coord.latitude))
            .collect();
        let polygon = Polygon::new(polygon_coords.into(), vec![]);

        for point in &data.points {
            let test_point = point!(x: point.longitude, y: point.latitude);

            // Debugging: Print the point being tested
            eprintln!(
                "Testing Point ID {} at ({}, {})",
                point.id, point.longitude, point.latitude
            );

            results.push(PIPResult {
                point_id: point.id,
                polygon_id: polygon_data.id,
                inside: polygon.contains(&test_point),
            });
        }
    }

    serde_json::to_string(&results).expect("Failed to serialize output")
}
