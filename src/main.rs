mod data;
mod analysis;
mod model;

fn main() {
    let patients = data::load_data("stroke-data.csv")
        .expect("Failed to load patient data");

    println!("Loaded {} patients", patients.len());

    analysis::print_confusion_matrix(&patients);

    model::real_decision_tree_classifier(&patients);
}
