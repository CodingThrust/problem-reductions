/// Regenerate example database fixture files from builder code.
///
/// This binary recomputes all model and rule examples using BruteForce/ILP
/// and writes them to `src/example_db/fixtures/` as wrapped JSON objects. Run
/// this in release mode after changing any model or rule to update the stored
/// expected results:
///
/// ```
/// cargo run --release --example regenerate_fixtures --features "ilp-highs example-db"
/// ```
use problemreductions::example_db::compute_example_db;
use problemreductions::export::write_example_db_to;
use std::fs;
use std::path::Path;

fn main() {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/example_db/fixtures");
    fs::create_dir_all(&fixtures_dir).expect("Failed to create fixtures directory");

    let example_db = compute_example_db().expect("Failed to compute canonical example database");
    let examples_path = fixtures_dir.join("examples.json");

    write_example_db_to(&fixtures_dir, &example_db);

    println!(
        "Regenerated fixtures: {} rule examples, {} model examples",
        example_db.rules.len(),
        example_db.models.len()
    );
    println!("  Examples: {}", examples_path.display());
}
