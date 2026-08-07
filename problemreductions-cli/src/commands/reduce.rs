use crate::dispatch::{
    load_problem, read_input, serialize_any_problem, PathStep, ProblemJson, ProblemJsonOutput,
    ReductionBundle,
};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::rules::{ReductionGraph, ReductionPath, ReductionStep};
use std::collections::BTreeMap;
use std::path::Path;

/// Parse a path JSON file (produced by `pred path ... -o`) into a ReductionPath.
fn load_path_file(path_file: &Path) -> Result<ReductionPath> {
    let content = std::fs::read_to_string(path_file).context("Failed to read path file")?;
    parse_path_json(&content)
}

pub(crate) fn parse_path_json(content: &str) -> Result<ReductionPath> {
    let json: serde_json::Value = serde_json::from_str(content).context("Failed to parse path")?;

    let path_array = json["path"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Expected one explicit route with a 'path' array"))?;

    let mut steps: Vec<ReductionStep> = Vec::new();
    for (i, entry) in path_array.iter().enumerate() {
        let from = parse_path_node(&entry["from"])?;
        if let Some(previous) = steps.last() {
            if previous.name != from.name || previous.variant != from.variant {
                anyhow::bail!("Explicit route is not continuous at edge {i}");
            }
        } else {
            steps.push(from);
        }
        steps.push(parse_path_node(&entry["to"])?);
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
    let variant = serde_json::from_value::<BTreeMap<String, String>>(
        node.get("variant")
            .ok_or_else(|| anyhow::anyhow!("Path node missing 'variant'"))?
            .clone(),
    )
    .context("Path node has invalid 'variant'")?;
    Ok(ReductionStep { name, variant })
}

pub fn reduce(input: &Path, via: &Path, out: &OutputConfig) -> Result<()> {
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

    let reduction_path = load_path_file(via)?;
    let first = reduction_path.steps.first().unwrap();
    if first.name != source_name || first.variant != source_variant {
        anyhow::bail!(
            "Path file starts with {}{} but source problem is {}{}",
            first.name,
            variant_to_full_slash(&first.variant),
            source_name,
            variant_to_full_slash(&source_variant),
        );
    }

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

use super::graph::variant_to_full_slash;
