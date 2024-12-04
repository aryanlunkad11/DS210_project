use crate::data_processing::Property;

pub fn build_predictive_model(data: &[Property]) -> Vec<f64> {
    data.iter()
        .map(|p| p.rent_per_sqft.unwrap_or(0.0) * 1.1) // Example prediction
        .collect()
}
