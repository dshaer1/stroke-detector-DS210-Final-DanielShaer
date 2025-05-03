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