use plotters::prelude::*;

pub fn generate_visualizations(
    centrality_results: &[(usize, f64)],
    _prediction_results: &[f64],
) {
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
