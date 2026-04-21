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

    let target_config: Vec<usize> = config_str
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Invalid config value '{}': {}", s.trim(), e))
        })
        .collect::<Result<Vec<_>>>()?;

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
        "Source problem: {}\nSource solution: {:?}\nSource evaluation: {}\nTarget problem: {}\nTarget evaluation: {}",
        source_name, source_config, source_eval, target_name, target_eval,
    );

    let json = serde_json::json!({
        "problem": source_name,
        "solution": source_config,
        "evaluation": source_eval,
        "intermediate": {
            "problem": target_name,
            "config": target_config,
            "evaluation": target_eval,
        },
    });

    out.emit_with_default_name("pred_extract.json", &text, &json)
}
