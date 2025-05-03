mod data;
mod analysis;

fn main() {
    let patients = data::load_data("stroke-data.csv")
        .expect("Failed to load patient data");

    println!("Loaded {} patients", patients.len());

    let rule_accuracy = analysis::evaluate_rule_based(&patients);
println!("Rule-based classifier accuracy: {:.2}%", rule_accuracy * 100.0);

let recall = analysis::rule_based_recall(&patients);
println!("Rule-based recall: {:.2}%", recall * 100.0);

analysis::print_confusion_matrix(&patients);
}