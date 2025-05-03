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

/// Compares predicted risk labels to actual stroke outcomes and returns accuracy.
pub fn evaluate_rule_based(patients: &[Patient]) -> f32 {
    let mut correct = 0;

    for patient in patients {
        let predicted = risk_to_label(&predict_rule_based(patient));
        if predicted == patient.stroke {
            correct += 1;
        }
    }

    correct as f32 / patients.len() as f32
}

// This fn returns the recall (true positive rate) for the rule-based classifier, as just overall accuracy is qute misleading because there are so many moe negs than pos.
pub fn rule_based_recall(patients: &[Patient]) -> f32 {
    let mut tp = 0;
    let mut fn_ = 0;

    for patient in patients {
        let pred = risk_to_label(&predict_rule_based(patient));
        let actual = patient.stroke;

        match (pred, actual) {
            (1, 1) => tp += 1,
            (0, 1) => fn_ += 1,
            _ => {}
        }
    }

    if tp + fn_ == 0 {
        0.0
    } else {
        tp as f32 / (tp + fn_) as f32
    }
}

// This creates a confusion matrix comparing predicted vs actual stroke outcomes.
pub fn print_confusion_matrix(patients: &[Patient]) {
    let mut tp = 0;
    let mut fp = 0;
    let mut tn = 0;
    let mut fn_ = 0;

    for patient in patients {
        let pred = risk_to_label(&predict_rule_based(patient));
        let actual = patient.stroke;

        match (pred, actual) {
            (1, 1) => tp += 1,
            (1, 0) => fp += 1,
            (0, 0) => tn += 1,
            (0, 1) => fn_ += 1,
            _ => {}
        }
    }

    println!("\nConfusion Matrix (Rule-Based Classifier):");
    println!("TP: {} | FP: {}", tp, fp);
    println!("FN: {} | TN: {}", fn_, tn);
}
