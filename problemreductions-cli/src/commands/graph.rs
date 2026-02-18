use crate::output::OutputConfig;
use anyhow::Result;
use problemreductions::rules::ReductionGraph;

pub fn list(out: &OutputConfig) -> Result<()> {
    let graph = ReductionGraph::new();

    let mut types = graph.problem_types();
    types.sort();

    let mut text = format!(
        "Registered problems: {} types, {} reductions, {} variant nodes\n\n",
        graph.num_types(),
        graph.num_reductions(),
        graph.num_variant_nodes(),
    );

    for name in &types {
        text.push_str(&format!("  {name}\n"));
    }

    let json = serde_json::json!({
        "num_types": graph.num_types(),
        "num_reductions": graph.num_reductions(),
        "num_variant_nodes": graph.num_variant_nodes(),
        "problems": types,
    });

    out.emit_with_default_name("pred_graph_list.json", &text, &json)
}
