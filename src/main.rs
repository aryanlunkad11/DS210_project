mod graph_analysis;
mod predictive_modeling;

use graph_analysis::{construct_graph, analyze_centrality};
use predictive_modeling::build_predictive_model;
use plotters::prelude::*;
use std::error::Error;
use std::time::Instant;
use crate::graph_analysis::{bfs_total_distance, haversine_distance};

#[derive(Debug)]
pub struct Property {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub rent_per_sqft: Option<f64>,
}

fn process_data(dataset_path: &str) -> Result<Vec<Property>, Box<dyn Error>> {
    use csv::Reader;

    let mut reader = Reader::from_path(dataset_path)?;
    let mut properties = Vec::new();

    for result in reader.records() {
        let record = result?;
        properties.push(Property {
            latitude: record.get(15).and_then(|s| s.parse().ok()),
            longitude: record.get(16).and_then(|s| s.parse().ok()),
            rent_per_sqft: record.get(6).and_then(|s| s.parse().ok()),
        });
    }

    Ok(properties)
}

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

    println!("Visualization saved to output/centrality.png");
}

fn main() {
    let dataset_path = "dubai_properties.csv";

    let start_total = Instant::now();

    // Step 1: Load and preprocess data
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

    // Step 2: Construct spatial graph
    println!("Constructing spatial graph...");
    let start = Instant::now();
    let graph = construct_graph(&processed_data[..]);
    println!("Graph constructed: {} nodes and {} edges.", graph.node_count(), graph.edge_count());
    println!("Time to construct graph: {:?}", start.elapsed());

    // Step 3: Analyze centrality
    println!("Analyzing centrality...");
    let start = Instant::now();
    let centrality_results = analyze_centrality(&graph, 50); // Analyze for 50 sampled nodes
    let top_centrality = centrality_results.iter().take(5).collect::<Vec<_>>();
    println!("Time to analyze centrality: {:?}", start.elapsed());
    println!("Top 5 Central Nodes: {:?}", top_centrality);

    // Step 4: Build predictive model
    println!("Building predictive model...");
    let start = Instant::now();
    let prediction_results = build_predictive_model(&processed_data);
    println!("Time to build predictive model: {:?}", start.elapsed());
    println!(
        "Prediction completed. Example prediction for the first property: {:.2}",
        prediction_results.first().unwrap_or(&0.0)
    );

    // Step 5: Generate visualizations
    println!("Generating visualizations...");
    let start = Instant::now();
    generate_visualizations(&centrality_results, &prediction_results);
    println!("Time to generate visualizations: {:?}", start.elapsed());

    // Final Summary
    println!("\nSummary of Analysis:");
    println!("---------------------");
    println!("Total Properties Analyzed: {}", processed_data.len());
    println!("Total Nodes in Graph: {}", graph.node_count());
    println!("Total Edges in Graph: {}", graph.edge_count());
    println!("Top 5 Most Connected Locations (Centrality): {:?}", top_centrality);

    println!("\nTotal runtime: {:?}", start_total.elapsed());
}



#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::Graph;

    #[test]
    fn test_construct_graph() {
        let properties = vec![
            Property {
                latitude: Some(25.276987),
                longitude: Some(55.296249),
                rent_per_sqft: None,
            },
            Property {
                latitude: Some(25.204849),
                longitude: Some(55.270783),
                rent_per_sqft: None,
            },
            Property {
                latitude: Some(25.171356),
                longitude: Some(55.212227),
                rent_per_sqft: None,
            },
        ];

        let graph = construct_graph(&properties);
        assert_eq!(graph.node_count(), 3);
        assert!(graph.edge_count() > 0); // At least one edge if within the 10 km threshold
    }

    #[test]
    fn test_analyze_centrality() {
        let mut graph = Graph::new();
        let node_a = graph.add_node((25.276987, 55.296249));
        let node_b = graph.add_node((25.204849, 55.270783));
        let node_c = graph.add_node((25.171356, 55.212227));

        graph.add_edge(node_a, node_b, 5.0);
        graph.add_edge(node_b, node_c, 7.0);

        let centrality_results = analyze_centrality(&graph, 3);
        assert_eq!(centrality_results.len(), 3);
        assert!(centrality_results[0].1 > 0.0); // Centrality score should be positive
    }

    #[test]
    fn test_bfs_total_distance() {
        let mut graph = Graph::new();
        let node_a = graph.add_node((25.276987, 55.296249));
        let node_b = graph.add_node((25.204849, 55.270783));
        let node_c = graph.add_node((25.171356, 55.212227));

        graph.add_edge(node_a, node_b, 5.0);
        graph.add_edge(node_b, node_c, 7.0);

        let total_distance = bfs_total_distance(&graph, node_a);
        assert!(total_distance > 0.0);
    }

    #[test]
    fn test_haversine_distance() {
        let coord1 = (25.276987, 55.296249);
        let coord2 = (25.204849, 55.270783);

        let distance = haversine_distance(coord1, coord2);
        assert!(distance > 0.0);
        assert!(distance < 10.0); // Should be within 10 km
    }
}
