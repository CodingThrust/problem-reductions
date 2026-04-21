use crate::dispatch::{load_problem, read_input, ReductionBundle};
use crate::output::OutputConfig;
use anyhow::{Context, Result};
use problemreductions::rules::{ReductionGraph, ReductionPath, ReductionStep};
use std::path::Path;

/// Extract a source-space configuration from a target-space configuration and a reduction bundle.
///
/// This lets external solvers (that solved the bundle's target problem on their own)
/// recover a solution in the original source problem space without having to
/// re-solve through `pred solve`.
pub fn extract(input: &Path, config_str: &str, out: &OutputConfig) -> Result<()> {
    let content = read_input(input)?;
    let json: serde_json::Value =
        serde_json::from_str(&content).context("Input is not valid JSON")?;

    if !(json.get("source").is_some() && json.get("target").is_some() && json.get("path").is_some())
    {
        anyhow::bail!(
            "Input is not a reduction bundle.\n\
             `pred extract` requires a bundle produced by `pred reduce`.\n\
             Got a plain problem file; did you mean `pred evaluate`?"
        );
    }

    let bundle: ReductionBundle =
        serde_json::from_value(json).context("Failed to parse reduction bundle")?;

    // An empty --config means an empty target configuration (zero-variable target problem).
    let target_config: Vec<usize> = if config_str.trim().is_empty() {
        Vec::new()
    } else {
        config_str
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Invalid config value '{}': {}", s.trim(), e))
            })
            .collect::<Result<Vec<_>>>()?
    };

    // Validate bundle self-consistency before trusting it.
    if bundle.path.len() < 2 {
        anyhow::bail!(
            "Malformed bundle: `path` must contain at least two steps (source and target), got {}",
            bundle.path.len()
        );
    }
    let first = bundle.path.first().unwrap();
    let last = bundle.path.last().unwrap();
    if first.name != bundle.source.problem_type || first.variant != bundle.source.variant {
        anyhow::bail!(
            "Malformed bundle: path starts with {} but source is {}",
            format_step(&first.name, &first.variant),
            format_step(&bundle.source.problem_type, &bundle.source.variant),
        );
    }
    if last.name != bundle.target.problem_type || last.variant != bundle.target.variant {
        anyhow::bail!(
            "Malformed bundle: path ends with {} but target is {}",
            format_step(&last.name, &last.variant),
            format_step(&bundle.target.problem_type, &bundle.target.variant),
        );
    }

    let target = load_problem(
        &bundle.target.problem_type,
        &bundle.target.variant,
        bundle.target.data.clone(),
    )?;
    let target_name = target.problem_name().to_string();
    let target_dims = target.dims_dyn();
    if target_config.len() != target_dims.len() {
        anyhow::bail!(
            "Target config has {} values but target problem {} has {} variables",
            target_config.len(),
            target_name,
            target_dims.len()
        );
    }
    for (i, (val, dim)) in target_config.iter().zip(target_dims.iter()).enumerate() {
        if *val >= *dim {
            anyhow::bail!(
                "Target config value {} at position {} is out of range: variable {} has {} possible values (0..{})",
                val, i, i, dim, dim.saturating_sub(1)
            );
        }
    }
    let target_eval = target.evaluate_dyn(&target_config);

    let source = load_problem(
        &bundle.source.problem_type,
        &bundle.source.variant,
        bundle.source.data.clone(),
    )?;
    let source_name = source.problem_name().to_string();

    let graph = ReductionGraph::new();
    let reduction_path = ReductionPath {
        steps: bundle
            .path
            .iter()
            .map(|s| ReductionStep {
                name: s.name.clone(),
                variant: s.variant.clone(),
            })
            .collect(),
    };

    let chain = graph
        .reduce_along_path(&reduction_path, source.as_any())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Bundle extraction requires a witness-capable reduction path; \
                 this bundle's path cannot map a target solution back to the source."
            )
        })?;

    let source_config = chain.extract_solution(&target_config);
    let source_eval = source.evaluate_dyn(&source_config);

    let text = format!(
        "Problem: {}\nSolver: external (via {})\nSolution: {:?}\nEvaluation: {}",
        source_name, target_name, source_config, source_eval,
    );

    // Schema aligned with `pred solve` on a bundle: `problem`, `reduced_to`, `solution`,
    // `evaluation`, `intermediate { problem, solution, evaluation }`. `solver` is "external"
    // to signal that pred did not run a solver — the target config came from outside.
    let json = serde_json::json!({
        "problem": source_name,
        "solver": "external",
        "reduced_to": target_name,
        "solution": source_config,
        "evaluation": source_eval,
        "intermediate": {
            "problem": target_name,
            "solution": target_config,
            "evaluation": target_eval,
        },
    });

    out.emit_with_default_name("pred_extract.json", &text, &json)
}

fn format_step(name: &str, variant: &std::collections::BTreeMap<String, String>) -> String {
    if variant.is_empty() {
        name.to_string()
    } else {
        let parts: Vec<String> = variant
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!("{}{{{}}}", name, parts.join(", "))
    }
}
