/// Export the example database as JSON for the Typst paper.
///
/// Writes to `docs/paper/data/examples.json` (gitignored build artifact).
///
/// ```
/// cargo run --features "example-db" --example export_examples
/// ```
use problemreductions::example_db::build_example_db;
use problemreductions::export::write_example_db_to;
use std::path::Path;

fn main() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/paper/data");
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let db = build_example_db().expect("Failed to build example database");
    write_example_db_to(&data_dir, &db);

    println!(
        "Exported {} models, {} rules",
        db.models.len(),
        db.rules.len()
    );
}
