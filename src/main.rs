mod graph_analysis;
mod predictive_modeling;

use graph_analysis::{construct_graph, analyze_centrality};
use predictive_modeling::build_predictive_model;
use plotters::prelude::*;
use std::error::Error;
use std::time::Instant;
use crate::graph_analysis::{bfs_total_distance, haversine_distance};

// This struct holds the key details of a property
#[derive(Debug)]
pub struct Property {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub rent_per_sqft: Option<f64>,
}

// Reads the dataset and processes it into a vector of Property structs
fn process_data(dataset_path: &str) -> Result<Vec<Property>, Box<dyn Error>> {
    use csv::Reader;

    let mut reader = Reader::from_path(dataset_path)?;
    let mut properties = Vec::new();

    for result in reader.records() {
        let record = result?;
        properties.push(Property {
            latitude: record.get(15).and_then(|s| s.parse().ok()), // Grab latitude
            longitude: record.get(16).and_then(|s| s.parse().ok()), // Grab longitude
            rent_per_sqft: record.get(6).and_then(|s| s.parse().ok()), // Grab rent per sqft
        });
    }

    Ok(properties)
}

// Creates a plot to visualize centrality results
fn generate_visualizations(centrality_results: &[(usize, f64)], _prediction_results: &[f64]) {
    let root = BitMapBackend::new("output/centrality.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Top 5 Central Nodes", ("sans-serif", 50))
        .build_cartesian_2d(0..centrality_results.len(), 0.0..10.0)
        .unwrap();

    chart
        .draw_series(centrality_results.iter().map(|(idx, centrality)| {
            Circle::new((*idx, *centrality), 5, BLUE.filled())
        }))
        .unwrap();
}

fn main() {
    let dataset_path = "dubai_properties.csv";

    let start_total = Instant::now(); // Timer for total execution

    // Step 1: Process the dataset to clean and extract relevant fields
    println!("Loading and processing dataset...");
    let start = Instant::now();
    let processed_data = match process_data(dataset_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error processing dataset: {}", e);
            return;
        }
    };
    println!("Time to process dataset: {:?}", start.elapsed());
    println!("Dataset Loaded: {} properties.", processed_data.len());

    // Step 2: Build a graph to represent the spatial relationships between properties
    println!("Constructing spatial graph...");
    let start = Instant::now();
    let graph = construct_graph(&processed_data[..]);
    println!("Graph constructed: {} nodes and {} edges.", graph.node_count(), graph.edge_count());
    println!("Time to construct graph: {:?}", start.elapsed());

    // Step 3: Run centrality analysis to find the most connected properties
    println!("Analyzing centrality...");
    let start = Instant::now();
    let centrality_results = analyze_centrality(&graph, 50); // Limit to a sample of 50 nodes
    let top_centrality = centrality_results.iter().take(5).collect::<Vec<_>>(); // Get top 5 nodes
    println!("Time to analyze centrality: {:?}", start.elapsed());
    println!("Top 5 Central Nodes: {:?}", top_centrality);

    // Step 4: Predict demand using a lightweight predictive model
    println!("Building predictive model...");
    let start = Instant::now();
    let prediction_results = build_predictive_model(&processed_data);
    println!("Time to build predictive model: {:?}", start.elapsed());
    println!(
        "Prediction completed. Example prediction for the first property: {:.2}",
        prediction_results.first().unwrap_or(&0.0)
    );

    // Step 5: Visualize the centrality results in a graph
    println!("Generating visualizations...");
    let start = Instant::now();
    generate_visualizations(&centrality_results, &prediction_results);
    println!("Time to generate visualizations: {:?}", start.elapsed());

    // Print the summary of all results
    println!("\nSummary of Analysis:");
    println!("---------------------");
    println!("Total Properties Analyzed: {}", processed_data.len());
    println!("Total Nodes in Graph: {}", graph.node_count());
    println!("Total Edges in Graph: {}", graph.edge_count());
    println!("Top 5 Most Connected Locations (Centrality): {:?}", top_centrality);

    println!("\nTotal runtime: {:?}", start_total.elapsed());
}
