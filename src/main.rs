mod data;
mod analysis;

fn main() {
    let patients = data::load_data("stroke-data.csv")
        .expect("Failed to load patient data");

    println!("Loaded {} patients", patients.len());

    let rule_accuracy = analysis::evaluate_rule_based(&patients);
println!("Rule-based classifier accuracy: {:.2}%", rule_accuracy * 100.0);
}

