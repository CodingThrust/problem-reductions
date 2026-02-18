use crate::output::OutputConfig;
use crate::problem_name::parse_problem_spec;
use anyhow::Result;
use problemreductions::registry::collect_schemas;

pub fn schema(problem: &str, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(problem)?;
    let schemas = collect_schemas();

    let schema = schemas
        .iter()
        .find(|s| s.name == spec.name)
        .ok_or_else(|| anyhow::anyhow!("No schema found for: {}", spec.name))?;

    let mut text = format!("{}\n", schema.name);
    if !schema.description.is_empty() {
        text.push_str(&format!("  {}\n", schema.description));
    }
    text.push_str("\nFields:\n");
    for field in &schema.fields {
        text.push_str(&format!("  {} ({})", field.name, field.type_name));
        if !field.description.is_empty() {
            text.push_str(&format!(" -- {}", field.description));
        }
        text.push('\n');
    }

    let json = serde_json::to_value(schema)?;
    let default_name = format!("pred_schema_{}.json", spec.name);
    out.emit_with_default_name(&default_name, &text, &json)
}
