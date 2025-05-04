// Runs the program: lets the user choose between model evaluation or interactive stroke risk prediction.
mod data;
mod analysis;
mod model;

use std::io;

fn main() {
    // Load dataset from CSV file
    let patients = data::load_data("stroke-data.csv")
        .expect("Failed to load patient data");
    println!("Loaded {} patients", patients.len());

    // Present mode options to the user
    println!("\nChoose an option:");
    println!("1. Show evaluation results");
    println!("2. Enter your own medical info to get stroke risk");

    // Read user input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim() {
        "1" => {
            // Print rule-based classifier results and confusion matrix
            analysis::print_confusion_matrix(&patients);

            // Train decision tree classifier and print results
            let (_acc, _recall, _model) = model::real_decision_tree_classifier(&patients);
        }
        "2" => {
            // Gather patient info from user input
            let patient = prompt_user_for_patient();

            // Train model only if user asks for prediction
            let (_acc, _recall, model) = model::real_decision_tree_classifier(&patients);

            // Use trained model to predict stroke risk from input
            let prediction = model::predict_patient(&model, &patient);
            println!(
                "\nBased on your input, the model predicts: {} stroke risk.",
                if prediction == 1 { "HIGH" } else { "LOW" }
            );
        }
        _ => println!("Invalid input."),
    }
}

/// Prompt user for medical information to create a Patient struct.
fn prompt_user_for_patient() -> data::Patient {
    /// Helper function to ask and parse typed input with retry loop
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

    // Collect each medical attribute from the user
    let age = ask::<f32>("Enter your age:");
    let hypertension = ask::<u8>("Hypertension? (0 = No, 1 = Yes):");
    let heart_disease = ask::<u8>("Heart disease? (0 = No, 1 = Yes):");

    let mut married = String::new();
    println!("Ever married? (Yes or No):");
    io::stdin().read_line(&mut married).expect("Failed to read input");

    let glucose = ask::<f32>("Avg glucose level:");
    let bmi = ask::<f32>("BMI:");

    // Return the Patient struct to be used in prediction
    data::Patient {
        age,
        hypertension,
        heart_disease,
        ever_married: married.trim().to_string(),
        avg_glucose_level: glucose,
        bmi,
        stroke: 0, // Placeholder, since actual outcome is unknown
    }
}