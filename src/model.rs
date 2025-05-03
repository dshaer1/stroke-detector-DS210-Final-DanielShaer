use crate::data::Patient;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::model_selection::train_test_split;
use smartcore::tree::decision_tree_classifier::*;

// this takes numerical features from each patient and returns them as Vec<Vec<f32>>
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

// this takes stroke labels from patient data
fn get_labels(patients: &[Patient]) -> Vec<u8> {
    patients.iter().map(|p| p.stroke).collect()
}

// this fn trains decision tree and returns its accuracy on test data
pub fn real_decision_tree_classifier(patients: &[Patient]) -> (f32, f32) {
    let mut training_data = patients.to_vec();

    let positive_cases: Vec<Patient> = patients
        .iter()
        .filter(|p| p.stroke == 1)
        .cloned()
        .collect();

    // To oversample the far rarer stroke occurrence, each positive case appears 3 extra times
    for _ in 0..3 {
        training_data.extend(positive_cases.clone());
    }

    let features = preprocess_features(&training_data);
    let labels = get_labels(&training_data);

    let x_matrix = DenseMatrix::from_2d_vec(&features);
    let (x_train, x_test, y_train, y_test) = train_test_split(&x_matrix, &labels, 0.2, true, None);

    let model = DecisionTreeClassifier::fit(&x_train, &y_train, Default::default()).unwrap();
    let predictions = model.predict(&x_test).unwrap();

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

    println!("\nDecision Tree Results:");
    println!("Accuracy: {:.2}%", accuracy * 100.0);
    println!("Recall (TPR): {:.2}%", recall * 100.0);
    println!("Confusion Matrix:");
    println!("TP: {} | FP: {}", tp, fp);
    println!("FN: {} | TN: {}", fn_, tn);

    (accuracy, recall)
}