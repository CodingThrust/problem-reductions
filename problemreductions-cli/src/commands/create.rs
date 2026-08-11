use crate::cli::{CreateArgs, ExampleSide};
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::{resolve_problem_ref, unknown_problem_error};
use crate::util;
use anyhow::{bail, Context, Result};
use num_bigint::BigUint;
use problemreductions::export::{ModelExample, ProblemRef, ProblemSide, RuleExample};
use problemreductions::models::formula::Quantifier;
use problemreductions::models::graph::{
    GeneralizedHex, HamiltonianCircuit, HamiltonianPath, HamiltonianPathBetweenTwoVertices,
    LabelledArc, LabelledDigraph, LengthBoundedDisjointPaths, LongestCircuit,
    MinimumCutIntoBoundedSets, MinimumMaximalMatching, RootedTreeArrangement, SteinerTree,
    SteinerTreeInGraphs,
};
use problemreductions::models::misc::{CbqRelation, FrequencyTable, KnownValue, QueryArg};
use problemreductions::models::Decision;
use problemreductions::prelude::*;
use problemreductions::topology::{
    DirectedGraph, Graph, KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

mod schema_support;
use self::schema_support::*;
pub(crate) use self::schema_support::{create_inputs_for, InputValueKind};

fn all_data_flags_empty(args: &CreateArgs) -> bool {
    args.is_empty()
}

fn emit_problem_output(output: &ProblemJsonOutput, out: &OutputConfig) -> Result<()> {
    let json = serde_json::to_value(output)?;
    if let Some(ref path) = out.output {
        let content = serde_json::to_string_pretty(&json).context("Failed to serialize JSON")?;
        std::fs::write(path, &content)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        out.info(&format!("Wrote {}", path.display()));
    } else {
        println!("{}", serde_json::to_string_pretty(&json)?);
    }
    Ok(())
}

fn format_problem_ref(problem: &ProblemRef) -> String {
    if problem.variant.is_empty() {
        return problem.name.clone();
    }

    let values = problem
        .variant
        .values()
        .cloned()
        .collect::<Vec<_>>()
        .join("/");
    format!("{}/{}", problem.name, values)
}

fn ensure_attribute_indices_in_range(
    indices: &[usize],
    num_attributes: usize,
    context: &str,
) -> Result<()> {
    for &attr in indices {
        anyhow::ensure!(
            attr < num_attributes,
            "{context} contains attribute index {attr}, which is out of range for --n {num_attributes}"
        );
    }
    Ok(())
}

fn parse_cdft_frequency_tables(
    raw: &str,
    attribute_domains: &[usize],
    num_objects: usize,
) -> Result<Vec<FrequencyTable>> {
    let num_attributes = attribute_domains.len();
    let mut seen_pairs = BTreeSet::new();

    raw.split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let (pair_str, counts_str) = entry.trim().split_once(':').ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid frequency table '{entry}', expected 'a,b:row0|row1|...'"
                )
            })?;
            let pair: Vec<usize> = util::parse_comma_list(pair_str.trim())?;
            anyhow::ensure!(
                pair.len() == 2,
                "Frequency table '{entry}' must start with exactly two attribute indices"
            );

            let attribute_a = pair[0];
            let attribute_b = pair[1];
            ensure_attribute_indices_in_range(
                &[attribute_a, attribute_b],
                num_attributes,
                &format!("Frequency table '{entry}'"),
            )?;
            anyhow::ensure!(
                attribute_a != attribute_b,
                "Frequency table '{entry}' must use two distinct attributes"
            );

            let pair_key = if attribute_a < attribute_b {
                (attribute_a, attribute_b)
            } else {
                (attribute_b, attribute_a)
            };
            anyhow::ensure!(
                seen_pairs.insert(pair_key),
                "Duplicate frequency table pair ({}, {})",
                pair_key.0,
                pair_key.1
            );

            let rows: Vec<Vec<usize>> = counts_str
                .split('|')
                .map(|row| util::parse_comma_list(row.trim()))
                .collect::<Result<_>>()?;

            let expected_rows = attribute_domains[attribute_a];
            anyhow::ensure!(
                rows.len() == expected_rows,
                "Frequency table '{entry}' has {} rows but attribute {attribute_a} has domain size {expected_rows}",
                rows.len()
            );

            let expected_cols = attribute_domains[attribute_b];
            for (row_index, row) in rows.iter().enumerate() {
                anyhow::ensure!(
                    row.len() == expected_cols,
                    "Frequency table '{entry}' row {row_index} has {} columns but attribute {attribute_b} has domain size {expected_cols}",
                    row.len()
                );
            }

            let total: usize = rows.iter().flatten().copied().sum();
            anyhow::ensure!(
                total == num_objects,
                "Frequency table '{entry}' sums to {total}, expected num_objects={num_objects}"
            );

            Ok(FrequencyTable::new(attribute_a, attribute_b, rows))
        })
        .collect()
}

fn parse_cdft_known_values(
    raw: Option<&str>,
    num_objects: usize,
    attribute_domains: &[usize],
) -> Result<Vec<KnownValue>> {
    let num_attributes = attribute_domains.len();
    match raw {
        None => Ok(vec![]),
        Some(s) if s.trim().is_empty() => Ok(vec![]),
        Some(s) => s
            .split(';')
            .filter(|entry| !entry.trim().is_empty())
            .map(|entry| {
                let triple: Vec<usize> = util::parse_comma_list(entry.trim())?;
                anyhow::ensure!(
                    triple.len() == 3,
                    "Known value '{entry}' must be an 'object,attribute,value' triple"
                );
                let object = triple[0];
                let attribute = triple[1];
                let value = triple[2];

                anyhow::ensure!(
                    object < num_objects,
                    "Known value '{entry}' has object index {object} out of range for num_objects={num_objects}"
                );
                anyhow::ensure!(
                    attribute < num_attributes,
                    "Known value '{entry}' has attribute index {attribute} out of range for {num_attributes} attributes"
                );
                let domain_size = attribute_domains[attribute];
                anyhow::ensure!(
                    value < domain_size,
                    "Known value '{entry}' has value {value} out of range for attribute {attribute} with domain size {domain_size}"
                );

                Ok(KnownValue::new(object, attribute, value))
            })
            .collect(),
    }
}

fn resolve_example_problem_ref(
    input: &str,
    rgraph: &problemreductions::rules::ReductionGraph,
) -> Result<ProblemRef> {
    let problem = resolve_problem_ref(input, rgraph)?;
    if rgraph.variants_for(&problem.name).is_empty() {
        bail!("{}", unknown_problem_error(input));
    }
    Ok(problem)
}

fn problem_output_from_side(side: ProblemSide) -> ProblemJsonOutput {
    ProblemJsonOutput {
        problem_type: side.problem,
        variant: side.variant,
        data: side.instance,
    }
}

fn problem_output_from_model(example: ModelExample) -> ProblemJsonOutput {
    ProblemJsonOutput {
        problem_type: example.problem,
        variant: example.variant,
        data: example.instance,
    }
}

fn resolve_model_example(
    example_spec: &str,
    rgraph: &problemreductions::rules::ReductionGraph,
) -> Result<ModelExample> {
    let model_db = problemreductions::example_db::build_model_db()?;
    let problem = resolve_example_problem_ref(example_spec, rgraph)?;
    model_db
        .models
        .into_iter()
        .find(|model| model.problem_ref() == problem)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No canonical model example exists for {}",
                format_problem_ref(&problem)
            )
        })
}

fn resolve_rule_example(
    example_spec: &str,
    target_spec: &str,
    rgraph: &problemreductions::rules::ReductionGraph,
) -> Result<RuleExample> {
    let rule_db = problemreductions::example_db::build_rule_db()?;
    let source = resolve_example_problem_ref(example_spec, rgraph)?;
    let target = resolve_example_problem_ref(target_spec, rgraph)?;
    rule_db
        .rules
        .into_iter()
        .find(|rule| rule.source.problem_ref() == source && rule.target.problem_ref() == target)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No canonical rule example exists for {} -> {}",
                format_problem_ref(&source),
                format_problem_ref(&target)
            )
        })
}

fn parse_job_shop_jobs(raw: &str) -> Result<Vec<Vec<(usize, u64)>>> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(vec![]);
    }

    raw.split(';')
        .enumerate()
        .map(|(job_index, job_str)| {
            let job_str = job_str.trim();
            anyhow::ensure!(
                !job_str.is_empty(),
                "Invalid --jobs value: empty job at position {}",
                job_index
            );

            job_str
                .split(',')
                .map(|task_str| {
                    let task_str = task_str.trim();
                    let (processor, length) = task_str.split_once(':').ok_or_else(|| {
                        anyhow::anyhow!(
                            "Invalid --jobs operation '{}': expected 'processor:length'",
                            task_str
                        )
                    })?;
                    let processor = processor.trim().parse::<usize>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --jobs operation '{}': processor must be a nonnegative integer",
                            task_str
                        )
                    })?;
                    let length = length.trim().parse::<u64>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --jobs operation '{}': length must be a nonnegative integer",
                            task_str
                        )
                    })?;
                    Ok((processor, length))
                })
                .collect()
        })
        .collect()
}

fn create_from_example(args: &CreateArgs, out: &OutputConfig) -> Result<()> {
    let example_spec = args
        .example
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("Missing --example problem spec"))?;

    if args.problem.is_some() {
        bail!(
            "Use either `pred create <PROBLEM>` or `pred create --example <PROBLEM_SPEC>`, not both"
        );
    }
    if args.has("random") || !all_data_flags_empty(args) {
        bail!("`pred create --example` does not accept problem-construction flags");
    }
    let rgraph = problemreductions::rules::ReductionGraph::new();

    let output = if let Some(target_spec) = args.example_target.as_deref() {
        let example = resolve_rule_example(example_spec, target_spec, &rgraph)?;
        match args.example_side {
            ExampleSide::Source => problem_output_from_side(example.source),
            ExampleSide::Target => problem_output_from_side(example.target),
        }
    } else {
        if matches!(args.example_side, ExampleSide::Target) {
            bail!("`--example-side target` requires `--to <TARGET_SPEC>`");
        }

        problem_output_from_model(resolve_model_example(example_spec, &rgraph)?)
    };

    emit_problem_output(&output, out)
}

fn resolved_graph_type(variant: &BTreeMap<String, String>) -> &str {
    variant
        .get("graph")
        .map(|s| s.as_str())
        .unwrap_or("SimpleGraph")
}

pub fn create(args: &CreateArgs, out: &OutputConfig) -> Result<()> {
    if args.example.is_some() {
        return create_from_example(args, out);
    }

    let problem = args.problem.as_ref().ok_or_else(|| {
        anyhow::anyhow!("Missing problem type.\n\nUsage: pred create <PROBLEM> [FLAGS]")
    })?;
    let (canonical, resolved_variant) =
        crate::create_args::resolve_registered_create_variant(problem);

    if args.has("random") {
        return create_random(args, canonical, &resolved_variant, out);
    }

    // ILP and CircuitSAT have complex input structures
    // not suited for CLI flags. Check before the empty-flags help so they get a
    // clear message.
    if canonical == "ILP" || canonical == "CircuitSAT" {
        bail!(
            "CLI creation is not yet supported for {canonical}.\n\n\
             {canonical} instances are typically created via reduction:\n\
               pred create MIS --graph 0-1,1-2 | pred reduce - --via route.json\n\n\
             Or use the Rust API for direct construction."
        );
    }

    let (data, variant) = create_schema_driven(args, canonical, &resolved_variant)?;

    let output = ProblemJsonOutput {
        problem_type: canonical.to_string(),
        variant,
        data,
    };

    emit_problem_output(&output, out)
}

/// Create a vertex-weight problem dispatching on geometry graph type.
/// Serialize a vertex-weight problem with a generic graph type.
fn ser_vertex_weight_problem_with<G: Graph + Serialize>(
    canonical: &str,
    graph: G,
    weights: Vec<i32>,
) -> Result<serde_json::Value> {
    match canonical {
        "MaximumIndependentSet" => ser(MaximumIndependentSet::new(graph, weights)),
        "MinimumVertexCover" => ser(MinimumVertexCover::new(graph, weights)),
        "MaximumClique" => ser(MaximumClique::new(graph, weights)),
        "MinimumDominatingSet" => ser(MinimumDominatingSet::new(graph, weights)),
        "MaximalIS" => ser(MaximalIS::new(graph, weights)),
        _ => unreachable!(),
    }
}

fn ser_decision_minimum_vertex_cover_with<
    G: Graph + Serialize + problemreductions::variant::VariantParam,
>(
    graph: G,
    weights: Vec<i32>,
    bound: i64,
) -> Result<serde_json::Value> {
    ser(Decision::new(
        MinimumVertexCover::new(graph, weights),
        bound,
    ))
}

fn ser<T: Serialize>(problem: T) -> Result<serde_json::Value> {
    util::ser(problem)
}

fn parse_kclique_threshold(
    k_flag: Option<usize>,
    num_vertices: usize,
    usage: &str,
) -> Result<usize> {
    let k = k_flag.ok_or_else(|| anyhow::anyhow!("KClique requires --k\n\n{usage}"))?;
    if k == 0 {
        bail!("KClique: --k must be positive");
    }
    if k > num_vertices {
        bail!("KClique: k must be <= graph num_vertices");
    }
    Ok(k)
}

fn variant_map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    util::variant_map(pairs)
}

fn ensure_vertex_in_bounds(vertex: usize, num_vertices: usize, label: &str) -> Result<()> {
    if vertex >= num_vertices {
        bail!("{label} {vertex} out of bounds (graph has {num_vertices} vertices)");
    }
    Ok(())
}

fn validate_vertex_index(
    label: &str,
    vertex: usize,
    num_vertices: usize,
    usage: &str,
) -> Result<()> {
    if vertex < num_vertices {
        return Ok(());
    }

    bail!("{label} must be less than num_vertices ({num_vertices})\n\n{usage}");
}

/// Parse `--dependencies` as semicolon-separated "lhs>rhs" pairs.
/// E.g., "0,1>2;0,2>3;1,3>4;2,4>5" means {0,1}->{2}, {0,2}->{3}, etc.
fn parse_dependencies(input: &str) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
    fn parse_dependency_side(side: &str) -> Result<Vec<usize>> {
        if side.trim().is_empty() {
            return Ok(vec![]);
        }
        side.split(',')
            .map(|s| {
                s.trim()
                    .parse::<usize>()
                    .map_err(|e| anyhow::anyhow!("Invalid attribute index: {}", e))
            })
            .collect()
    }

    input
        .split(';')
        .map(|dep| {
            let parts: Vec<&str> = dep.trim().split('>').collect();
            if parts.len() != 2 {
                bail!(
                    "Invalid dependency format: expected 'lhs>rhs', got '{}'",
                    dep.trim()
                );
            }
            let lhs = parse_dependency_side(parts[0])?;
            let rhs = parse_dependency_side(parts[1])?;
            Ok((lhs, rhs))
        })
        .collect()
}

/// Parse `--matrix` as semicolon-separated rows of comma-separated bool values (0/1).
/// E.g., "1,0;0,1;1,1"
fn parse_bool_rows(rows_str: &str) -> Result<Vec<Vec<bool>>> {
    let matrix: Vec<Vec<bool>> = rows_str
        .split(';')
        .map(|row| {
            row.trim()
                .split(',')
                .map(|entry| match entry.trim() {
                    "1" | "true" => Ok(true),
                    "0" | "false" => Ok(false),
                    other => Err(anyhow::anyhow!(
                        "Invalid boolean entry '{other}': expected 0/1 or true/false"
                    )),
                })
                .collect()
        })
        .collect::<Result<_>>()?;

    if let Some(expected_width) = matrix.first().map(Vec::len) {
        anyhow::ensure!(
            matrix.iter().all(|row| row.len() == expected_width),
            "All rows in --matrix must have the same length"
        );
    }

    Ok(matrix)
}

/// Parse `--arcs` as directed arc pairs and build a `DirectedGraph`.
///
/// Returns `(graph, num_arcs)`. Infers vertex count from arc endpoints
/// unless `num_vertices` is provided (which must be >= inferred count).
/// E.g., "0>1,1>2,2>0"
fn parse_directed_graph(
    arcs_str: &str,
    num_vertices: Option<usize>,
) -> Result<(DirectedGraph, usize)> {
    let arcs: Vec<(usize, usize)> = arcs_str
        .split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split('>').collect();
            if parts.len() != 2 {
                bail!(
                    "Invalid arc '{}': expected format u>v (e.g., 0>1)",
                    pair.trim()
                );
            }
            let u: usize = parts[0].parse()?;
            let v: usize = parts[1].parse()?;
            Ok((u, v))
        })
        .collect::<Result<Vec<_>>>()?;
    let inferred_num_v = arcs
        .iter()
        .flat_map(|&(u, v)| [u, v])
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    let num_v = match num_vertices {
        Some(user_num_v) => {
            anyhow::ensure!(
                user_num_v >= inferred_num_v,
                "--num-vertices ({}) is too small for the arcs: need at least {} to cover vertices up to {}",
                user_num_v,
                inferred_num_v,
                inferred_num_v.saturating_sub(1),
            );
            user_num_v
        }
        None => inferred_num_v,
    };
    let num_arcs = arcs.len();
    Ok((DirectedGraph::new(num_v, arcs), num_arcs))
}

/// Parse `--candidate-arcs` as `u>v:w` entries for StrongConnectivityAugmentation.
pub(super) fn supports_random(name: &str) -> bool {
    matches!(
        name,
        "DecisionMinimumVertexCover"
            | "MaximumIndependentSet"
            | "MinimumVertexCover"
            | "MaximumClique"
            | "MinimumDominatingSet"
            | "MaximalIS"
            | "KClique"
            | "MinimumCutIntoBoundedSets"
            | "HamiltonianCircuit"
            | "HamiltonianPath"
            | "HamiltonianPathBetweenTwoVertices"
            | "LongestCircuit"
            | "MinimumMaximalMatching"
            | "RootedTreeArrangement"
            | "SteinerTree"
            | "SteinerTreeInGraphs"
            | "LengthBoundedDisjointPaths"
            | "MaximumAchromaticNumber"
            | "MaximumDomaticNumber"
            | "MinimumCoveringByCliques"
            | "MinimumIntersectionGraphBasis"
            | "MaximumLeafSpanningTree"
            | "GeneralizedHex"
            | "BottleneckTravelingSalesman"
            | "MaxCut"
            | "MaximumMatching"
            | "TravelingSalesman"
            | "SpinGlass"
            | "KColoring"
            | "OptimalLinearArrangement"
    )
}

/// Handle `pred create <PROBLEM> --random ...`
fn create_random(
    args: &CreateArgs,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
    out: &OutputConfig,
) -> Result<()> {
    let num_vertices = args.value::<usize>("num-vertices").ok_or_else(|| {
        anyhow::anyhow!(
            "--random requires --num-vertices\n\n\
             Usage: pred create {} --random --num-vertices 10 [--edge-prob 0.3] [--seed 42]",
            canonical
        )
    })?;

    let graph_type = resolved_graph_type(resolved_variant);

    let (data, variant) = match canonical {
        "DecisionMinimumVertexCover" => {
            let raw_bound = args.value::<i64>("bound").ok_or_else(|| {
                anyhow::anyhow!(
                    "DecisionMinimumVertexCover requires --bound\n\n\
                     Usage: pred create DecisionMinimumVertexCover --random --num-vertices 5 [--edge-prob 0.5] [--seed 42] --bound 3"
                )
            })?;
            anyhow::ensure!(
                raw_bound >= 0,
                "DecisionMinimumVertexCover: --bound must be non-negative"
            );
            let bound = raw_bound;
            let weights = vec![1i32; num_vertices];
            match graph_type {
                "KingsSubgraph" => {
                    let positions = util::create_random_int_positions(num_vertices, args.value::<u64>("seed"));
                    let graph = KingsSubgraph::new(positions);
                    (
                        ser_decision_minimum_vertex_cover_with(graph, weights, bound)?,
                        resolved_variant.clone(),
                    )
                }
                "TriangularSubgraph" => {
                    let positions = util::create_random_int_positions(num_vertices, args.value::<u64>("seed"));
                    let graph = TriangularSubgraph::new(positions);
                    (
                        ser_decision_minimum_vertex_cover_with(graph, weights, bound)?,
                        resolved_variant.clone(),
                    )
                }
                "UnitDiskGraph" => {
                    let positions = util::create_random_float_positions(num_vertices, args.value::<u64>("seed"));
                    let radius = args.value::<f64>("radius").unwrap_or(1.5);
                    let graph = UnitDiskGraph::new(positions, radius);
                    (
                        ser_decision_minimum_vertex_cover_with(graph, weights, bound)?,
                        resolved_variant.clone(),
                    )
                }
                _ => {
                    let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
                    if !(0.0..=1.0).contains(&edge_prob) {
                        bail!("--edge-prob must be between 0.0 and 1.0");
                    }
                    let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
                    (
                        ser_decision_minimum_vertex_cover_with(graph, weights, bound)?,
                        resolved_variant.clone(),
                    )
                }
            }
        }

        // Graph problems with vertex weights
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet"
        | "MaximalIS" => {
            let weights = vec![1i32; num_vertices];
            match graph_type {
                "KingsSubgraph" => {
                    let positions = util::create_random_int_positions(num_vertices, args.value::<u64>("seed"));
                    let graph = KingsSubgraph::new(positions);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                "TriangularSubgraph" => {
                    let positions = util::create_random_int_positions(num_vertices, args.value::<u64>("seed"));
                    let graph = TriangularSubgraph::new(positions);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                "UnitDiskGraph" => {
                    let radius = args.value::<f64>("radius").unwrap_or(1.0);
                    let positions = util::create_random_float_positions(num_vertices, args.value::<u64>("seed"));
                    let graph = UnitDiskGraph::new(positions, radius);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                _ => {
                    let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
                    if !(0.0..=1.0).contains(&edge_prob) {
                        bail!("--edge-prob must be between 0.0 and 1.0");
                    }
                    let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
                    let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                    let data = ser_vertex_weight_problem_with(canonical, graph, weights)?;
                    (data, variant)
                }
            }
        }

        "KClique" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let usage =
                "Usage: pred create KClique --random --num-vertices 5 [--edge-prob 0.5] [--seed 42] --k 3";
            let k = parse_kclique_threshold(args.value::<usize>("k"), graph.num_vertices(), usage)?;
            (
                ser(KClique::new(graph, k))?,
                variant_map(&[("graph", "SimpleGraph")]),
            )
        }

        // MinimumCutIntoBoundedSets (graph + edge weights + s/t/B/K)
        "MinimumCutIntoBoundedSets" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let num_edges = graph.num_edges();
            let edge_weights = vec![1i32; num_edges];
            let source = 0;
            let sink = num_vertices.saturating_sub(1);
            let size_bound = num_vertices; // no effective size constraint
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            (
                ser(MinimumCutIntoBoundedSets::new(
                    graph,
                    edge_weights,
                    source,
                    sink,
                    size_bound,
                ))?,
                variant,
            )
        }

        // MaximumAchromaticNumber (graph only, no weights)
        "MaximumAchromaticNumber" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MaximumAchromaticNumber::new(graph))?,
                variant,
            )
        }

        // MaximumDomaticNumber (graph only, no weights)
        "MaximumDomaticNumber" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MaximumDomaticNumber::new(graph))?,
                variant,
            )
        }

        // MinimumCoveringByCliques (graph only, no weights)
        "MinimumCoveringByCliques" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MinimumCoveringByCliques::new(graph))?,
                variant,
            )
        }

        // MinimumIntersectionGraphBasis (graph only, no weights)
        "MinimumIntersectionGraphBasis" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MinimumIntersectionGraphBasis::new(graph))?,
                variant,
            )
        }

        // MinimumMaximalMatching (graph only, no weights)
        "MinimumMaximalMatching" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(MinimumMaximalMatching::new(graph))?, variant)
        }

        // Hamiltonian Circuit (graph only, no weights)
        "HamiltonianCircuit" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(HamiltonianCircuit::new(graph))?, variant)
        }

        // Maximum Leaf Spanning Tree (graph only, no weights)
        "MaximumLeafSpanningTree" => {
            let num_vertices = num_vertices.max(2);
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MaximumLeafSpanningTree::new(graph))?,
                variant,
            )
        }

        // HamiltonianPath (graph only, no weights)
        "HamiltonianPath" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(HamiltonianPath::new(graph))?, variant)
        }

        // HamiltonianPathBetweenTwoVertices (graph + source/target)
        "HamiltonianPathBetweenTwoVertices" => {
            let num_vertices = num_vertices.max(2);
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let source_vertex = args.value::<usize>("source-vertex").unwrap_or(0);
            let target_vertex = args.value::<usize>("target-vertex")
                .unwrap_or_else(|| num_vertices.saturating_sub(1));
            ensure_vertex_in_bounds(source_vertex, graph.num_vertices(), "source_vertex")?;
            ensure_vertex_in_bounds(target_vertex, graph.num_vertices(), "target_vertex")?;
            anyhow::ensure!(
                source_vertex != target_vertex,
                "source_vertex and target_vertex must be distinct"
            );
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(HamiltonianPathBetweenTwoVertices::new(
                    graph,
                    source_vertex,
                    target_vertex,
                ))?,
                variant,
            )
        }

        // LongestCircuit (graph + unit edge lengths)
        "LongestCircuit" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let edge_lengths = vec![1i32; graph.num_edges()];
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            (ser(LongestCircuit::new(graph, edge_lengths))?, variant)
        }

        // GeneralizedHex (graph only, with source/sink defaults)
        "GeneralizedHex" => {
            let num_vertices = num_vertices.max(2);
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let source = args.value::<usize>("source").unwrap_or(0);
            let sink = args.value::<usize>("sink").unwrap_or(num_vertices - 1);
            let usage = "Usage: pred create GeneralizedHex --random --num-vertices 6 [--edge-prob 0.5] [--seed 42] [--source 0] [--sink 5]";
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            if source == sink {
                bail!("GeneralizedHex requires distinct --source and --sink\n\n{usage}");
            }
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(GeneralizedHex::new(graph, source, sink))?, variant)
        }

        // LengthBoundedDisjointPaths (graph only, with path defaults)
        "LengthBoundedDisjointPaths" => {
            let num_vertices = if num_vertices < 2 {
                eprintln!(
                    "Warning: LengthBoundedDisjointPaths requires at least 2 vertices; rounding {} up to 2",
                    num_vertices
                );
                2
            } else {
                num_vertices
            };
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let source = args.value::<usize>("source").unwrap_or(0);
            let sink = args.value::<usize>("sink").unwrap_or(num_vertices - 1);
            let bound = args
                .value::<i64>("max-length")
                .unwrap_or((num_vertices - 1) as i64);
            let max_length = validate_length_bounded_disjoint_paths_args(
                num_vertices,
                source,
                sink,
                bound,
                None,
            )?;
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(LengthBoundedDisjointPaths::new(
                    graph,
                    source,
                    sink,
                    max_length,
                ))?,
                variant,
            )
        }

        // Graph problems with edge weights
        "BottleneckTravelingSalesman" | "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let num_edges = graph.num_edges();
            let edge_weights = vec![1i32; num_edges];
            let variant = match canonical {
                "BottleneckTravelingSalesman" => variant_map(&[]),
                _ => variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]),
            };
            let data = match canonical {
                "BottleneckTravelingSalesman" => {
                    ser(BottleneckTravelingSalesman::new(graph, edge_weights))?
                }
                "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                _ => unreachable!(),
            };
            (data, variant)
        }

        // SteinerTreeInGraphs
        "SteinerTreeInGraphs" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let num_edges = graph.num_edges();
            let edge_weights = vec![1i32; num_edges];
            // Use first half of vertices as terminals (at least 2)
            let num_terminals = std::cmp::max(2, num_vertices / 2);
            let terminals: Vec<usize> = (0..num_terminals).collect();
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            (
                ser(SteinerTreeInGraphs::new(graph, terminals, edge_weights))?,
                variant,
            )
        }

        // SteinerTree
        "SteinerTree" => {
            anyhow::ensure!(
                num_vertices >= 2,
                "SteinerTree random generation requires --num-vertices >= 2"
            );
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let mut state = util::lcg_init(args.value::<u64>("seed"));
            let graph = util::create_random_graph(num_vertices, edge_prob, Some(state));
            // Advance state past the graph generation
            for _ in 0..num_vertices * num_vertices {
                util::lcg_step(&mut state);
            }
            let edge_weights: Vec<i32> = (0..graph.num_edges())
                .map(|_| (util::lcg_step(&mut state) * 9.0) as i32 + 1)
                .collect();
            let num_terminals = std::cmp::max(2, num_vertices * 2 / 5);
            let terminals = util::lcg_choose(&mut state, num_vertices, num_terminals);
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            (
                ser(SteinerTree::new(graph, edge_weights, terminals))?,
                variant,
            )
        }

        // SpinGlass
        "SpinGlass" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let num_edges = graph.num_edges();
            let couplings = vec![1i32; num_edges];
            let fields = vec![0i32; num_vertices];
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            (
                ser(SpinGlass::from_graph(graph, couplings, fields))?,
                variant,
            )
        }

        // KColoring
        "KColoring" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let (k, _variant) =
                util::validate_k_param(resolved_variant, args.value::<usize>("k"), Some(3), "KColoring")?;
            util::ser_kcoloring(graph, k)?
        }

        // OptimalLinearArrangement — graph only (optimization)
        "OptimalLinearArrangement" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(OptimalLinearArrangement::new(graph))?, variant)
        }

        // RootedTreeArrangement — graph + bound
        "RootedTreeArrangement" => {
            let edge_prob = args.value::<f64>("edge-prob").unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.value::<u64>("seed"));
            let n = graph.num_vertices();
            let usage = "Usage: pred create RootedTreeArrangement --random --num-vertices 5 [--edge-prob 0.5] [--seed 42] [--bound 10]";
            let bound = args.value::<i64>("bound")
                .map(|b| parse_nonnegative_usize_bound(b, "RootedTreeArrangement", usage))
                .transpose()?
                .unwrap_or((n.saturating_sub(1)) * graph.num_edges());
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(RootedTreeArrangement::new(graph, bound))?, variant)
        }

        _ => bail!(
            "Random generation is not supported for {canonical}. \
             Supported: graph-based problems (MIS, MVC, MaxCut, MaxClique, \
             MaximumMatching, MinimumDominatingSet, SpinGlass, KColoring, KClique, DecisionMinimumVertexCover, TravelingSalesman, \
             BottleneckTravelingSalesman, SteinerTreeInGraphs, HamiltonianCircuit, MaximumLeafSpanningTree, SteinerTree, \
             OptimalLinearArrangement, RootedTreeArrangement, HamiltonianPath, LongestCircuit, GeneralizedHex)"
        ),
    };

    let output = ProblemJsonOutput {
        problem_type: canonical.to_string(),
        variant,
        data,
    };

    emit_problem_output(&output, out)
}

/// Parse implication rules from semicolon-separated "antecedents>consequent" strings.
///
/// Format: "0,1>2;3>4;5,6,7>0" where antecedents are comma-separated indices
/// before the `>` and the consequent is the single index after.
fn parse_implications(s: &str) -> Result<Vec<(Vec<usize>, usize)>> {
    let mut implications = Vec::new();
    for part in s.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (lhs, rhs) = part.split_once('>').ok_or_else(|| {
            anyhow::anyhow!("Each implication must contain '>' separator: {part}")
        })?;
        let antecedents: Vec<usize> = lhs
            .split(',')
            .map(|x| x.trim().parse::<usize>())
            .collect::<Result<_, _>>()
            .context(format!("Invalid antecedent index in implication: {part}"))?;
        let consequent: usize = rhs
            .trim()
            .parse()
            .context(format!("Invalid consequent index in implication: {part}"))?;
        implications.push((antecedents, consequent));
    }
    Ok(implications)
}

#[cfg(test)]
mod tests;
