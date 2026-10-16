//! Generate or check the response schema derived from the actual browser wire structs.
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let mode = args.next();
    if mode.as_deref().is_some_and(|mode| mode != "--check") {
        return Err("Usage: export_frontend_schema [--check [PATH]]".into());
    }
    let check = mode.is_some();
    let path = args.next().map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../docs/website/wasm-paths.schema.json")
    });
    if args.next().is_some() {
        return Err("Usage: export_frontend_schema [--check [PATH]]".into());
    }
    let schema =
        serde_json::to_string_pretty(&problemreductions_web::path_response_schema())? + "\n";
    if check {
        if fs::read_to_string(&path)? != schema {
            return Err(format!("Rust contract drift: {}", path.display()).into());
        }
    } else {
        fs::write(path, schema)?;
    }
    Ok(())
}
