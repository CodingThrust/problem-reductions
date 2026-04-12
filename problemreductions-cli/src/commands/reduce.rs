use crate::dispatch::{
    load_problem, read_input, serialize_any_problem, PathStep, ProblemJson, ProblemJsonOutput,
    ReductionBundle,
};
use crate::output::OutputConfig;
use crate::problem_name::resolve_problem_ref;
use anyhow::{Context, Result};
use problemreductions::rules::{
    MinimizeSteps, ReductionGraph, ReductionMode, ReductionPath, ReductionStep,
};
use problemreductions::types::ProblemSize;
use std::collections::BTreeMap;
use std::path::Path;

/// Parse a path JSON file (produced by `pred path ... -o`) into a ReductionPath.
fn load_path_file(path_file: &Path) -> Result<ReductionPath> {
    let content = std::fs::read_to_string(path_file).context("Failed to read path file")?;
    let json: serde_json::Value =
        serde_json::from_str(&content).context("Failed to parse path file")?;

    let path_array = json["path"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Path file missing 'path' array"))?;

    let mut steps: Vec<ReductionStep> = Vec::new();
    for (i, entry) in path_array.iter().enumerate() {
        if i == 0 {
            let from = &entry["from"];
            steps.push(parse_path_node(from)?);
        }
        let to = &entry["to"];
        steps.push(parse_path_node(to)?);
    }

    if steps.len() < 2 {
        anyhow::bail!("Path file must contain at least one reduction step");
    }

    Ok(ReductionPath { steps })
}

fn parse_path_node(node: &serde_json::Value) -> Result<ReductionStep> {
    let name = node["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Path node missing 'name'"))?
        .to_string();
    let variant: BTreeMap<String, String> = node
        .get("variant")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    Ok(ReductionStep { name, variant })
}

pub fn reduce(
    input: &Path,
    target: Option<&str>,
    via: Option<&Path>,
    out: &OutputConfig,
) -> Result<()> {
    // 1. Load source problem
    let content = read_input(input)?;
    let problem_json: ProblemJson = serde_json::from_str(&content)?;

    let source = load_problem(
        &problem_json.problem_type,
        &problem_json.variant,
        problem_json.data.clone(),
    )?;

    let source_name = source.problem_name();
    let source_variant = source.variant_map();
    let graph = ReductionGraph::new();

    // 3. Get reduction path: from --via file or auto-discover
    let reduction_path = if let Some(path_file) = via {
        let path = load_path_file(path_file)?;
        // Validate that the path starts with the source
        let first = path.steps.first().unwrap();
        let last = path.steps.last().unwrap();
        if first.name != source_name || first.variant != source_variant {
            anyhow::bail!(
                "Path file starts with {}{} but source problem is {}{}",
                first.name,
                variant_to_full_slash(&first.variant),
                source_name,
                variant_to_full_slash(&source_variant),
            );
        }
        // If --to is given, validate it matches the path's target
        if let Some(target) = target {
            let dst_ref = resolve_problem_ref(target, &graph)?;
            if last.name != dst_ref.name || last.variant != dst_ref.variant {
                anyhow::bail!(
                    "Path file ends with {}{} but --to specifies {}{}",
                    last.name,
                    variant_to_full_slash(&last.variant),
                    dst_ref.name,
                    variant_to_full_slash(&dst_ref.variant),
                );
            }
        }
        path
    } else {
        // --to is required when --via is not given
        let target = target.ok_or_else(|| {
            anyhow::anyhow!(
                "Either --to or --via is required.\n\n\
                 Usage:\n\
                   pred reduce problem.json --to QUBO\n\
                   pred reduce problem.json --via path.json"
            )
        })?;
        let dst_ref = resolve_problem_ref(target, &graph)?;

        // Auto-discover cheapest path
        let input_size = ProblemSize::new(vec![]);
        let best_path = graph.find_cheapest_path_mode(
            source_name,
            &source_variant,
            &dst_ref.name,
            &dst_ref.variant,
            ReductionMode::Witness,
            &input_size,
            &MinimizeSteps,
        );

        best_path.ok_or_else(|| {
            let variant_hint = variant_hint_for(&graph, &dst_ref.name);
            anyhow::anyhow!(
                "No witness-capable reduction path from {} to {}\n\
                 {variant_hint}\n\
                 Hint: generate a path file first, then pass it with --via:\n\
                   pred path {} {} -o path.json\n\
                   pred reduce {} --via path.json -o reduced.json",
                source_name,
                dst_ref.name,
                source_name,
                dst_ref.name,
                input.display(),
            )
        })?
    };

    // 4. Execute reduction chain via reduce_along_path
    let chain = graph
        .reduce_along_path(&reduction_path, source.as_any())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Reduction bundles require witness-capable paths; this path cannot produce a recoverable witness."
            )
        })?;

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

    let json = serde_json::to_value(&bundle)?;

    let mut text = format!(
        "Reduced {} to {} ({} steps)\n",
        source_name,
        target_step.name,
        reduction_path.len(),
    );
    text.push_str(&format!("\nPath: {}\n", reduction_path));
    text.push_str(
        "\nHint: use -o to save the reduction bundle as JSON, or --json to print JSON to stdout.",
    );

    out.emit_with_default_name("", &text, &json)?;

    Ok(())
}

use super::graph::{variant_hint_for, variant_to_full_slash};
