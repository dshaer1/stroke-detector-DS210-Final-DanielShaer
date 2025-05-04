// Loads patient medical data from a CSV file and defines the Patient struct.

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;
use serde::Deserialize;

/// Represents one patient’s medical information and stroke outcome.
#[derive(Debug, Deserialize, Clone)]
pub struct Patient {
    pub age: f32,
    pub hypertension: u8,
    pub heart_disease: u8,
    pub ever_married: String,
    pub avg_glucose_level: f32,
    pub bmi: f32,
    pub stroke: u8,
}

/// Loads patient data from a CSV file into a vector of `Patient`.
pub fn load_data(path: &str) -> Result<Vec<Patient>, Box<dyn Error>> {
    // Open the file and wrap it in a buffered reader for efficiency
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Set up the CSV reader with headers enabled
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    let mut patients = Vec::new();

    // Deserialize each row into a Patient struct and add to the list
    for result in rdr.deserialize() {
        let patient: Patient = result?;
        patients.push(patient);
    }

    Ok(patients)
}