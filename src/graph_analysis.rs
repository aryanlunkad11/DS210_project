use crate::Property;
use petgraph::visit::EdgeRef; 
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;


// Haversine distance calculation
pub fn haversine_distance(coord1: (f64, f64), coord2: (f64, f64)) -> f64 {
    let (lat1, lon1) = coord1;
    let (lat2, lon2) = coord2;

    let r = 6371.0; // Earth's radius in kilometers
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}

pub fn construct_graph(data: &[Property]) -> Graph<(f64, f64), f64> {
    let mut graph = Graph::new();
    let mut nodes = Vec::new();

    for property in data.iter().filter(|p| p.latitude.is_some() && p.longitude.is_some()) {
        let coordinates = (property.latitude.unwrap(), property.longitude.unwrap());
        nodes.push(graph.add_node(coordinates));
    }

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let coord1 = graph[nodes[i]];
            let coord2 = graph[nodes[j]];
            let distance = haversine_distance(coord1, coord2);
            if distance <= 10.0 { // Threshold in kilometers
                graph.add_edge(nodes[i], nodes[j], distance);
            }
        }
    }

    graph
}

pub fn analyze_centrality(
    graph: &Graph<(f64, f64), f64>,
    sample_size: usize,
) -> Vec<(usize, f64)> {
    let mut centrality_scores = Vec::new();
    let nodes: Vec<_> = graph.node_indices().collect();

    for node in nodes.iter().take(sample_size) {
        let total_distance = bfs_total_distance(graph, *node);
        let centrality = if total_distance > 0.0 {
            1.0 / total_distance
        } else {
            0.0
        };
        centrality_scores.push((node.index(), centrality));
    }

    centrality_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    centrality_scores
}

pub fn bfs_total_distance(graph: &Graph<(f64, f64), f64>, start_node: NodeIndex) -> f64 {
    let mut visited = HashMap::new();
    let mut queue = std::collections::VecDeque::new();
    let mut total_distance = 0.0;

    queue.push_back((start_node, 0.0));

    while let Some((current_node, distance)) = queue.pop_front() {
        if visited.contains_key(&current_node) {
            continue;
        }

        visited.insert(current_node, true);
        total_distance += distance;

        for edge in graph.edges(current_node) {
            let neighbor = edge.target();
            if !visited.contains_key(&neighbor) {
                queue.push_back((neighbor, distance + *edge.weight()));
            }
        }
    }

    total_distance
}

