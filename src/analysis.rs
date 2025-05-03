use crate::data::Patient;

// Note: I created this Enum to represent stroke risk level based on simple rules.
#[derive(Debug, PartialEq)]
pub enum RiskLevel {
    High,
    Moderate,
    Low,
}

// This fn predicts stroke risk based on basic rules using age, heart disease, hypertension, and glucose.
pub fn predict_rule_based(patient: &Patient) -> RiskLevel {
    if patient.age > 65.0 && patient.heart_disease == 1 {
        RiskLevel::High
    } else if patient.hypertension == 1 && patient.avg_glucose_level > 150.0 {
        RiskLevel::Moderate
    } else {
        RiskLevel::Low
    }
}

// This converts RiskLevel into a binary label
pub fn risk_to_label(risk: &RiskLevel) -> u8 {
    match risk {
        RiskLevel::High | RiskLevel::Moderate => 1,
        RiskLevel::Low => 0,
    }
}

// This prints full performance metrics for the rule-based classifier in a consistent format.
pub fn print_confusion_matrix(patients: &[Patient]) {
    let mut tp = 0;
    let mut fp = 0;
    let mut tn = 0;
    let mut fn_ = 0;
    let mut correct = 0;

    for patient in patients {
        let pred = risk_to_label(&predict_rule_based(patient));
        let actual = patient.stroke;

        if pred == actual {
            correct += 1;
        }

        match (pred, actual) {
            (1, 1) => tp += 1,
            (1, 0) => fp += 1,
            (0, 0) => tn += 1,
            (0, 1) => fn_ += 1,
            _ => {}
        }
    }

    let accuracy = correct as f32 / patients.len() as f32;
    let recall = if tp + fn_ == 0 {
        0.0
    } else {
        tp as f32 / (tp + fn_) as f32
    };

    println!("\nRule-Based Classifier Results:");
    println!("Accuracy: {:.2}%", accuracy * 100.0);
    println!("Recall (TPR): {:.2}%", recall * 100.0);
    println!("Confusion Matrix:");
    println!("TP: {} | FP: {}", tp, fp);
    println!("FN: {} | TN: {}", fn_, tn);
}