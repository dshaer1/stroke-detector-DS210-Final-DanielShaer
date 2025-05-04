mod data;
mod analysis;
mod model;

use std::io;

fn main() {
    let patients = data::load_data("stroke-data.csv")
        .expect("Failed to load patient data");
    println!("Loaded {} patients", patients.len());

    println!("\nChoose an option:");
    println!("1. Show evaluation results");
    println!("2. Enter your own medical info to get stroke risk");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
        "1" => {
            analysis::print_confusion_matrix(&patients);
            let (_acc, _recall, _model) = model::real_decision_tree_classifier(&patients);
        }
        "2" => {
            let patient = prompt_user_for_patient();

            // Train model only if user asks for prediction
            let (_acc, _recall, model) = model::real_decision_tree_classifier(&patients);
            let prediction = model::predict_patient(&model, &patient);
            println!(
                "\nBased on your input, the model predicts: {} stroke risk.",
                if prediction == 1 { "HIGH" } else { "LOW" }
            );
        }
        _ => println!("Invalid input."),
    }
}

// Prompt user for medical information to create a Patient
fn prompt_user_for_patient() -> data::Patient {
    fn ask<T: std::str::FromStr>(q: &str) -> T {
        loop {
            let mut input = String::new();
            println!("{}", q);
            io::stdin().read_line(&mut input).expect("Failed to read input");
            if let Ok(val) = input.trim().parse::<T>() {
                return val;
            } else {
                println!("Invalid input. Try again.");
            }
        }
    }

    let age = ask::<f32>("Enter your age:");
    let hypertension = ask::<u8>("Hypertension? (0 = No, 1 = Yes):");
    let heart_disease = ask::<u8>("Heart disease? (0 = No, 1 = Yes):");

    let mut married = String::new();
    println!("Ever married? (Yes or No):");
    io::stdin().read_line(&mut married).expect("Failed to read input");

    let glucose = ask::<f32>("Avg glucose level:");
    let bmi = ask::<f32>("BMI:");

    data::Patient {
        age,
        hypertension,
        heart_disease,
        ever_married: married.trim().to_string(),
        avg_glucose_level: glucose,
        bmi,
        stroke: 0,
    }
}