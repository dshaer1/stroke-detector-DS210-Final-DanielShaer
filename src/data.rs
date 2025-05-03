use csv::ReaderBuilder;
use std::error::Error;

#[derive(Debug, serde::Deserialize)]
pub struct Patient {
    pub age: f32,
    pub hypertension: u8,
    pub heart_disease: u8,
    pub ever_married: String,
    pub avg_glucose_level: f32,
    pub bmi: f32,
    pub stroke: u8,
}

pub fn load_data(path: &str) -> Result<Vec<Patient>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut patients = Vec::new();

    for result in rdr.deserialize() {
        let record: Patient = result?;
        patients.push(record);
    }

    Ok(patients)
}