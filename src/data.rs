#[derive(Debug, Deserialize)]
pub struct Patient {
    pub age: f32,
    pub hypertension: u8,
    pub heart_disease: u8,
    pub ever_married: String,
    pub avg_glucose_level: f32,
    pub bmi: f32,
    pub stroke: u8,
}