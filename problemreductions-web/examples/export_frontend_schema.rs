//! Emit request/response schemas derived from the browser wire structs.
use problemreductions_web::{
    comparison_request_schema, comparison_response_schema, path_request_schema,
    path_response_schema,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas = serde_json::json!({
        "wasm-path-request": path_request_schema(),
        "wasm-comparison-request": comparison_request_schema(),
        "wasm-paths": path_response_schema(),
        "wasm-comparison": comparison_response_schema(),
    });
    serde_json::to_writer(std::io::stdout().lock(), &schemas)?;
    Ok(())
}
