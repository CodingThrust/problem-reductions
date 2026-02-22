use crate::dispatch::{
    load_problem, read_input, serialize_any_problem, PathStep, ProblemJson, ProblemJsonOutput,
    ReductionBundle,
};
use crate::output::OutputConfig;
use crate::problem_name::parse_problem_spec;
use anyhow::{Context, Result};
use problemreductions::rules::{MinimizeSteps, ReductionGraph, ReductionPath, ReductionStep};
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

    Ok(ReductionPath {
        steps,
        overheads: vec![],
    })
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
                "Path file starts with {} {} but source problem is {} {}",
                first.name,
                format_variant(&first.variant),
                source_name,
                format_variant(&source_variant),
            );
        }
        // If --to is given, validate it matches the path's target
        if let Some(target) = target {
            let dst_spec = parse_problem_spec(target)?;
            if last.name != dst_spec.name {
                anyhow::bail!(
                    "Path file ends with {} but --to specifies {}",
                    last.name,
                    dst_spec.name,
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
        let dst_spec = parse_problem_spec(target)?;
        let dst_variants = graph.variants_for(&dst_spec.name);
        if dst_variants.is_empty() {
            anyhow::bail!(
                "{}",
                crate::problem_name::unknown_problem_error(&dst_spec.name)
            );
        }

        // Auto-discover cheapest path
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
                    .is_none_or(|bp: &ReductionPath| p.len() < bp.len());
                if is_better {
                    best_path = Some(p);
                }
            }
        }

        best_path.ok_or_else(|| {
            anyhow::anyhow!(
                "No reduction path from {} to {}\n\n\
                 Hint: generate a path file first, then pass it with --via:\n\
                   pred path {} {} -o path.json\n\
                   pred reduce {} --via path.json -o reduced.json",
                source_name,
                dst_spec.name,
                source_name,
                dst_spec.name,
                input.display(),
            )
        })?
    };

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

fn format_variant(v: &BTreeMap<String, String>) -> String {
    if v.is_empty() {
        "(default)".to_string()
    } else {
        let pairs: Vec<String> = v.iter().map(|(k, val)| format!("{k}={val}")).collect();
        format!("{{{}}}", pairs.join(", "))
    }
}
