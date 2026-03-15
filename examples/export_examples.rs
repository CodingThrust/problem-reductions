use problemreductions::example_db::{build_model_db, build_rule_db, default_generated_dir};
use problemreductions::export::{write_model_db_to, write_rule_db_to};
use std::fs;

fn main() {
    let output_dir = default_generated_dir();
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("Failed to clear generated examples directory");
    }
    fs::create_dir_all(&output_dir).expect("Failed to create generated examples directory");

    let rule_db = build_rule_db().expect("Failed to build canonical rule database");
    let model_db = build_model_db().expect("Failed to build canonical model database");

    write_rule_db_to(&output_dir, &rule_db);
    write_model_db_to(&output_dir, &model_db);

    println!(
        "Exported {} rule examples and {} model examples",
        rule_db.rules.len(),
        model_db.models.len()
    );
}
