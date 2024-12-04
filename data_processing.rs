use csv::Reader;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Rent")]
    pub rent: f64,
    #[serde(rename = "Beds")]
    pub beds: u32,
    #[serde(rename = "Baths")]
    pub baths: u32,
    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,
    #[serde(rename = "Rent_per_sqft")]
    pub rent_per_sqft: Option<f64>,
    #[serde(rename = "Age_of_listing_in_days")]
    pub age_of_listing_in_days: Option<u32>,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "City")]
    pub city: String,
}

pub fn process_data(file_path: &str) -> Result<Vec<Property>, Box<dyn Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut properties = Vec::new();

    for result in reader.deserialize() {
        let record: Property = result?;
        properties.push(record);
    }

    Ok(properties)
}
