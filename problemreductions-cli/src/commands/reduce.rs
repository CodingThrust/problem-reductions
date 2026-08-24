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
    #[derive(serde::Deserialize)]
    struct RouteNode {
        name: String,
        variant: BTreeMap<String, String>,
    }

    #[derive(serde::Deserialize)]
    struct RouteEdge {
        from: RouteNode,
        to: RouteNode,
    }

    #[derive(serde::Deserialize)]
    struct ExplicitRoute {
        path: Vec<RouteEdge>,
    }

    let route: ExplicitRoute =
        serde_json::from_str(content).context("Expected one explicit route with a 'path' array")?;

    let mut steps: Vec<ReductionStep> = Vec::new();
    for (i, edge) in route.path.into_iter().enumerate() {
        let from = ReductionStep {
            name: edge.from.name,
            variant: edge.from.variant,
        };
        if let Some(previous) = steps.last() {
            if previous.name != from.name || previous.variant != from.variant {
                anyhow::bail!("Explicit route is not continuous at edge {i}");
            }
        } else {
            steps.push(from);
        }
        steps.push(ReductionStep {
            name: edge.to.name,
            variant: edge.to.variant,
        });
    }

    if steps.len() < 2 {
        anyhow::bail!("Path file must contain at least one reduction step");
    }

    Ok(ReductionPath { steps })
}

pub(crate) fn execute_route(
    problem_json: ProblemJson,
    reduction_path: ReductionPath,
) -> Result<ReductionBundle> {
    let source = load_problem(
        &problem_json.problem_type,
        &problem_json.variant,
        problem_json.data.clone(),
    )?;
    let source_name = source.problem_name();
    let source_variant = source.variant_map();
    let first = reduction_path
        .steps
        .first()
        .expect("route parser requires at least one edge");
    if first.name != source_name || first.variant != source_variant {
        anyhow::bail!(
            "Explicit route starts with {}{} but source problem is {}{}",
            first.name,
            variant_to_full_slash(&first.variant),
            source_name,
            variant_to_full_slash(&source_variant),
        );
    }

    let graph = ReductionGraph::new();
    let chain = graph
        .reduce_along_path(&reduction_path, source.as_any())
        .map_err(|error| anyhow::anyhow!("Reduction path execution failed: {error}"))?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Reduction bundles require witness-capable paths; this path cannot produce a recoverable witness."
            )
        })?;
    let target_step = reduction_path
        .steps
        .last()
        .expect("route parser requires at least one edge");
    let target_data = serialize_any_problem(
        &target_step.name,
        &target_step.variant,
        chain.target_problem_any(),
    )?;

    Ok(ReductionBundle {
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
            .into_iter()
            .map(|step| PathStep {
                name: step.name,
                variant: step.variant,
            })
            .collect(),
    })
}

pub fn reduce(input: &Path, via: &Path, out: &OutputConfig) -> Result<()> {
    let content = read_input(input)?;
    let problem_json: ProblemJson = serde_json::from_str(&content)?;
    let reduction_path = load_path_file(via)?;
    let route_len = reduction_path.len();
    let route_text = reduction_path.to_string();
    let bundle = execute_route(problem_json, reduction_path)?;

    let json = serde_json::to_value(&bundle)?;

    let mut text = format!(
        "Reduced {} to {} ({} steps)\n",
        bundle.source.problem_type, bundle.target.problem_type, route_len,
    );
    text.push_str(&format!("\nPath: {route_text}\n"));
    text.push_str(
        "\nHint: use -o to save the reduction bundle as JSON, or --json to print JSON to stdout.",
    );

    out.emit_with_default_name("", &text, &json)?;

    Ok(())
}

use super::graph::variant_to_full_slash;
