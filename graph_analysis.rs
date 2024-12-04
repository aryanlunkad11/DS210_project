use petgraph::graph::{Graph, NodeIndex};
use crate::data_processing::Property;

pub fn construct_graph(data: &[Property]) -> Graph<(f64, f64), f64> {
    let mut graph = Graph::new();
    let mut node_indices = Vec::new();

    for property in data.iter().filter(|p| p.latitude.is_some() && p.longitude.is_some()) {
        let node = (property.latitude.unwrap(), property.longitude.unwrap());
        node_indices.push(graph.add_node(node));
    }

    for i in 0..node_indices.len() {
        for j in i + 1..node_indices.len() {
            let distance = haversine_distance(
                graph[node_indices[i]],
                graph[node_indices[j]],
            );
            if distance < 50.0 { // Only connect nearby nodes
                graph.add_edge(node_indices[i], node_indices[j], distance);
            }
        }
    }

    graph
}

pub fn analyze_centrality(graph: &Graph<(f64, f64), f64>) -> Vec<(usize, f64)> {
    graph.node_indices()
        .map(|node| {
            let degree = graph.edges(node).count();
            (node.index(), degree as f64)
        })
        .collect::<Vec<_>>()
}

fn haversine_distance(node1: (f64, f64), node2: (f64, f64)) -> f64 {
    let (lat1, lon1) = node1;
    let (lat2, lon2) = node2;

    let r = 6371.0; // Earth's radius in kilometers
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}
