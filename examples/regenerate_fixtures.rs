/// Regenerate example database fixture files from builder code.
///
/// This binary recomputes all model and rule examples using BruteForce/ILP
/// and writes the results to `src/example_db/fixtures/`. Run this in release
/// mode after changing any model or rule to update the stored expected results:
///
/// ```
/// cargo run --release --example regenerate_fixtures --features example-db
/// ```
use problemreductions::example_db::{compute_model_db, compute_rule_db};
use std::fs;
use std::path::Path;

fn main() {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/example_db/fixtures");
    fs::create_dir_all(&fixtures_dir).expect("Failed to create fixtures directory");

    let rule_db = compute_rule_db().expect("Failed to compute canonical rule database");
    let model_db = compute_model_db().expect("Failed to compute canonical model database");

    let models_path = fixtures_dir.join("models.json");
    let rules_path = fixtures_dir.join("rules.json");

    let models_json = serde_json::to_string(&model_db).expect("Failed to serialize models");
    let rules_json = serde_json::to_string(&rule_db).expect("Failed to serialize rules");

    fs::write(&models_path, &models_json).expect("Failed to write models fixture");
    fs::write(&rules_path, &rules_json).expect("Failed to write rules fixture");

    println!(
        "Regenerated fixtures: {} rule examples, {} model examples",
        rule_db.rules.len(),
        model_db.models.len()
    );
    println!("  Models: {}", models_path.display());
    println!("  Rules: {}", rules_path.display());
}
