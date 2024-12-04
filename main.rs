mod data_processing;
mod graph_analysis;
mod predictive_modeling;
mod visualization;

use data_processing::process_data;
use graph_analysis::{construct_graph, analyze_centrality};
use predictive_modeling::build_predictive_model;
use visualization::generate_visualizations;

fn main() {
    let dataset_path = "dubai_properties.csv";

    // Step 1: Load and preprocess data
    println!("Loading and processing dataset...");
    let processed_data = match process_data(dataset_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error processing dataset: {}", e);
            return;
        }
    };

    println!("Dataset Loaded: {} properties.", processed_data.len());

    // Step 2: Construct spatial graph
    println!("Constructing spatial graph...");
    let graph = construct_graph(&processed_data);
    println!(
        "Graph constructed: {} nodes and {} edges.",
        graph.node_count(),
        graph.edge_count()
    );

    // Step 3: Analyze centrality measures
    println!("Analyzing centrality...");
    let centrality_results = analyze_centrality(&graph);
    let top_centrality = centrality_results.iter().take(5).collect::<Vec<_>>();
    println!("Top 5 Central Nodes: {:?}", top_centrality);

    // Step 4: Build predictive model for market saturation
    println!("Building predictive model...");
    let prediction_results = build_predictive_model(&processed_data);

    // Print prediction insights
    println!(
        "Prediction completed. Example prediction for the first property: {:.2}",
        prediction_results.first().unwrap_or(&0.0)
    );

    // Step 5: Generate visualizations
    println!("Generating visualizations...");
    generate_visualizations(&centrality_results, &prediction_results);

    // Final Summary
    println!("\nSummary of Analysis:");
    println!("---------------------");
    println!("Total Properties Analyzed: {}", processed_data.len());
    println!("Total Nodes in Graph: {}", graph.node_count());
    println!("Total Edges in Graph: {}", graph.edge_count());
    println!("Top 5 Most Connected Locations (Centrality): {:?}", top_centrality);
    //println!("Visualization saved as 'output/centrality.png'.");
}
