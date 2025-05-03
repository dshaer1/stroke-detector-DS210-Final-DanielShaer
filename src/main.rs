mod data;
mod analysis;

fn main() {
    let patients = data::load_data("stroke-data.csv")
        .expect("Failed to load patient data");

    println!("Loaded {} patients", patients.len());
}

