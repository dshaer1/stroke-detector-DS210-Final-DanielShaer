// Trains and evaluates a decision tree classifier for stroke prediction using real patient data.

use crate::data::Patient;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::model_selection::train_test_split;
use smartcore::tree::decision_tree_classifier::*;

/// Converts patient structs into feature vectors used by the model.
fn preprocess_features(patients: &[Patient]) -> Vec<Vec<f32>> {
    patients.iter().map(|p| {
        vec![
            p.age,
            p.hypertension as f32,
            p.heart_disease as f32,
            if p.ever_married == "Yes" { 1.0 } else { 0.0 },
            p.avg_glucose_level,
            p.bmi,
        ]
    }).collect()
}

/// Extracts stroke outcome labels (0 or 1) from a list of patients.
fn get_labels(patients: &[Patient]) -> Vec<u8> {
    patients.iter().map(|p| p.stroke).collect()
}

/// Trains a decision tree classifier on patient data, evaluates it,
pub fn real_decision_tree_classifier(patients: &[Patient]) -> (
    f32,
    f32,
    DecisionTreeClassifier<f32, u8, DenseMatrix<f32>, Vec<u8>>,
) {
    let mut training_data = patients.to_vec();

    // Oversample positive stroke cases to balance the dataset
    let positive_cases: Vec<Patient> = patients
        .iter()
        .filter(|p| p.stroke == 1)
        .cloned()
        .collect();

    for _ in 0..3 {
        training_data.extend(positive_cases.clone());
    }

    // Create input features and output labels
    let features = preprocess_features(&training_data);
    let labels = get_labels(&training_data);

    // Convert features into a matrix and split into train/test sets
    let x_matrix = DenseMatrix::from_2d_vec(&features);
    let (x_train, x_test, y_train, y_test) = train_test_split(&x_matrix, &labels, 0.2, true, None);

    // Train decision tree classifier
    let model = DecisionTreeClassifier::fit(&x_train, &y_train, Default::default()).unwrap();

    // Predict on test set
    let predictions = model.predict(&x_test).unwrap();

    // Calculate evaluation metrics
    let mut correct = 0;
    let mut tp = 0;
    let mut fp = 0;
    let mut tn = 0;
    let mut fn_ = 0;

    for (pred, actual) in predictions.iter().zip(y_test.iter()) {
        if pred == actual {
            correct += 1;
        }
        match (*pred, *actual) {
            (1, 1) => tp += 1,
            (1, 0) => fp += 1,
            (0, 0) => tn += 1,
            (0, 1) => fn_ += 1,
            _ => {}
        }
    }

    let accuracy = correct as f32 / y_test.len() as f32;
    let recall = if tp + fn_ == 0 {
        0.0
    } else {
        tp as f32 / (tp + fn_) as f32
    };

    // Print results
    println!("\nDecision Tree Results:");
    println!("Accuracy: {:.2}%", accuracy * 100.0);
    println!("Recall (TPR): {:.2}%", recall * 100.0);
    println!("Confusion Matrix:");
    println!("TP: {} | FP: {}", tp, fp);
    println!("FN: {} | TN: {}", fn_, tn);

    (accuracy, recall, model)
}

/// Uses a trained decision tree to predict stroke (0 or 1) for a new patient.
pub fn predict_patient(
    model: &DecisionTreeClassifier<f32, u8, DenseMatrix<f32>, Vec<u8>>,
    patient: &Patient,
) -> u8 {
    // Convert single patient into model-compatible matrix format
    let features = vec![vec![
        patient.age,
        patient.hypertension as f32,
        patient.heart_disease as f32,
        if patient.ever_married == "Yes" { 1.0 } else { 0.0 },
        patient.avg_glucose_level,
        patient.bmi,
    ]];

    let matrix = DenseMatrix::from_2d_vec(&features);

    // Predict stroke risk (0 or 1)
    model.predict(&matrix).unwrap()[0]
}