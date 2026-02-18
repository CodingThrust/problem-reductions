use crate::dispatch::{
    load_problem, serialize_any_problem, PathStep, ProblemJson, ProblemJsonOutput, ReductionBundle,
};
use crate::output::OutputConfig;
use crate::problem_name::parse_problem_spec;
use anyhow::Result;
use problemreductions::rules::{MinimizeSteps, ReductionGraph};
use problemreductions::types::ProblemSize;
use std::collections::BTreeMap;
use std::path::Path;

pub fn reduce(input: &Path, target: &str, out: &OutputConfig) -> Result<()> {
    // 1. Load source problem
    let content = std::fs::read_to_string(input)?;
    let problem_json: ProblemJson = serde_json::from_str(&content)?;

    let source = load_problem(
        &problem_json.problem_type,
        &problem_json.variant,
        problem_json.data.clone(),
    )?;

    let source_name = source.problem_name();
    let source_variant = source.variant_map();

    // 2. Parse target spec
    let dst_spec = parse_problem_spec(target)?;
    let graph = ReductionGraph::new();

    // Resolve target variant (use default if not specified)
    let graph_json_str = graph.to_json_string()?;
    let graph_json: serde_json::Value = serde_json::from_str(&graph_json_str)?;
    let nodes = graph_json["nodes"].as_array().unwrap();

    let dst_variants: Vec<BTreeMap<String, String>> = nodes
        .iter()
        .filter(|n| n["name"].as_str() == Some(&dst_spec.name))
        .map(|n| {
            n["variant"]
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect()
                })
                .unwrap_or_default()
        })
        .collect();

    if dst_variants.is_empty() {
        anyhow::bail!("Unknown target problem: {}", dst_spec.name);
    }

    // 3. Find reduction path
    let input_size = ProblemSize::new(vec![]);
    let mut best_path = None;

    for dv in &dst_variants {
        if let Some(p) = graph.find_cheapest_path(
            source_name,
            &source_variant,
            &dst_spec.name,
            dv,
            &input_size,
            &MinimizeSteps,
        ) {
            let is_better = best_path
                .as_ref()
                .is_none_or(|bp: &problemreductions::rules::ReductionPath| p.len() < bp.len());
            if is_better {
                best_path = Some(p);
            }
        }
    }

    let reduction_path = best_path.ok_or_else(|| {
        anyhow::anyhow!(
            "No reduction path from {} to {}",
            source_name,
            dst_spec.name
        )
    })?;

    // 4. Execute reduction chain via reduce_along_path
    let chain = graph
        .reduce_along_path(&reduction_path, source.as_any())
        .ok_or_else(|| anyhow::anyhow!("Failed to execute reduction chain"))?;

    // 5. Serialize target
    let target_step = reduction_path.steps.last().unwrap();
    let target_data = serialize_any_problem(
        &target_step.name,
        &target_step.variant,
        chain.target_problem_any(),
    )?;

    // 6. Build full reduction bundle
    let bundle = ReductionBundle {
        source: ProblemJsonOutput {
            problem_type: source_name.to_string(),
            variant: source_variant,
            data: problem_json.data,
        },
        target: ProblemJsonOutput {
            problem_type: target_step.name.clone(),
            variant: target_step.variant.clone(),
            data: target_data,
        },
        path: reduction_path
            .steps
            .iter()
            .map(|s| PathStep {
                name: s.name.clone(),
                variant: s.variant.clone(),
            })
            .collect(),
    };

    let text = format!(
        "Reduced {} to {} ({} steps)\nBundle written with source + target + path.",
        source_name,
        target_step.name,
        reduction_path.len(),
    );

    let json = serde_json::to_value(&bundle)?;
    let default_name = format!("pred_reduce_{}_to_{}.json", source_name, dst_spec.name);
    out.emit_with_default_name(&default_name, &text, &json)
}
