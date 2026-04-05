use crate::cli::{CreateArgs, ExampleSide};
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::{
    parse_problem_spec, resolve_catalog_problem_ref, resolve_problem_ref, unknown_problem_error,
};
use crate::util;
use anyhow::{bail, Context, Result};
use num_bigint::BigUint;
use problemreductions::export::{ModelExample, ProblemRef, ProblemSide, RuleExample};
use problemreductions::models::algebraic::{
    ClosestVectorProblem, ConsecutiveBlockMinimization, ConsecutiveOnesMatrixAugmentation,
    SparseMatrixCompression,
};
use problemreductions::models::formula::Quantifier;
use problemreductions::models::graph::{
    GeneralizedHex, HamiltonianCircuit, HamiltonianPath, HamiltonianPathBetweenTwoVertices,
    LengthBoundedDisjointPaths, LongestCircuit, MinimumCutIntoBoundedSets,
    MinimumDummyActivitiesPert, MinimumMaximalMatching, RootedTreeArrangement, SteinerTree,
    SteinerTreeInGraphs, VertexCover,
};
use problemreductions::models::misc::{
    CbqRelation, FrequencyTable, KnownValue, QueryArg, SchedulingWithIndividualDeadlines,
    ThreePartition,
};
use problemreductions::prelude::*;
use problemreductions::registry::collect_schemas;
use problemreductions::topology::{
    BipartiteGraph, DirectedGraph, Graph, KingsSubgraph, MixedGraph, SimpleGraph,
    TriangularSubgraph, UnitDiskGraph,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

mod schema_semantics;
use self::schema_semantics::validate_schema_driven_semantics;
mod schema_support;
use self::schema_support::*;

const MULTIPLE_COPY_FILE_ALLOCATION_EXAMPLE_ARGS: &str =
    "--graph 0-1,1-2,2-3 --usage 5,4,3,2 --storage 1,1,1,1";
const MULTIPLE_COPY_FILE_ALLOCATION_USAGE: &str =
    "Usage: pred create MultipleCopyFileAllocation --graph 0-1,1-2,2-3 --usage 5,4,3,2 --storage 1,1,1,1";
const EXPECTED_RETRIEVAL_COST_EXAMPLE_ARGS: &str =
    "--probabilities 0.2,0.15,0.15,0.2,0.1,0.2 --num-sectors 3";

/// Check if all data flags are None (no problem-specific input provided).
fn all_data_flags_empty(args: &CreateArgs) -> bool {
    args.graph.is_none()
        && args.weights.is_none()
        && args.edge_weights.is_none()
        && args.edge_lengths.is_none()
        && args.capacities.is_none()
        && args.demands.is_none()
        && args.setup_costs.is_none()
        && args.production_costs.is_none()
        && args.inventory_costs.is_none()
        && args.bundle_capacities.is_none()
        && args.cost_matrix.is_none()
        && args.delay_matrix.is_none()
        && args.lower_bounds.is_none()
        && args.multipliers.is_none()
        && args.source.is_none()
        && args.sink.is_none()
        && args.requirement.is_none()
        && args.num_paths_required.is_none()
        && args.paths.is_none()
        && args.couplings.is_none()
        && args.fields.is_none()
        && args.clauses.is_none()
        && args.disjuncts.is_none()
        && args.num_vars.is_none()
        && args.matrix.is_none()
        && args.k.is_none()
        && args.num_partitions.is_none()
        && args.target.is_none()
        && args.m.is_none()
        && args.n.is_none()
        && args.num_vertices.is_none()
        && args.source_vertex.is_none()
        && args.target_vertex.is_none()
        && args.edge_prob.is_none()
        && args.seed.is_none()
        && args.positions.is_none()
        && args.radius.is_none()
        && args.source_1.is_none()
        && args.sink_1.is_none()
        && args.source_2.is_none()
        && args.sink_2.is_none()
        && args.requirement_1.is_none()
        && args.requirement_2.is_none()
        && args.requirement.is_none()
        && args.sizes.is_none()
        && args.probabilities.is_none()
        && args.capacity.is_none()
        && args.sequence.is_none()
        && args.sets.is_none()
        && args.r_sets.is_none()
        && args.s_sets.is_none()
        && args.r_weights.is_none()
        && args.s_weights.is_none()
        && args.partition.is_none()
        && args.partitions.is_none()
        && args.bundles.is_none()
        && args.universe.is_none()
        && args.biedges.is_none()
        && args.left.is_none()
        && args.right.is_none()
        && args.rank.is_none()
        && args.basis.is_none()
        && args.target_vec.is_none()
        && args.bounds.is_none()
        && args.release_times.is_none()
        && args.deadlines.is_none()
        && args.lengths.is_none()
        && args.terminals.is_none()
        && args.terminal_pairs.is_none()
        && args.tree.is_none()
        && args.required_edges.is_none()
        && args.bound.is_none()
        && args.latency_bound.is_none()
        && args.length_bound.is_none()
        && args.weight_bound.is_none()
        && args.diameter_bound.is_none()
        && args.cost_bound.is_none()
        && args.delay_budget.is_none()
        && args.pattern.is_none()
        && args.strings.is_none()
        && args.string.is_none()
        && args.costs.is_none()
        && args.arc_costs.is_none()
        && args.arcs.is_none()
        && args.left_arcs.is_none()
        && args.right_arcs.is_none()
        && args.homologous_pairs.is_none()
        && args.quantifiers.is_none()
        && args.usage.is_none()
        && args.storage.is_none()
        && args.source.is_none()
        && args.sink.is_none()
        && args.size_bound.is_none()
        && args.cut_bound.is_none()
        && args.values.is_none()
        && args.precedences.is_none()
        && args.distance_matrix.is_none()
        && args.candidate_arcs.is_none()
        && args.potential_edges.is_none()
        && args.budget.is_none()
        && args.max_cycle_length.is_none()
        && args.deadlines.is_none()
        && args.lengths.is_none()
        && args.precedence_pairs.is_none()
        && args.resource_bounds.is_none()
        && args.resource_requirements.is_none()
        && args.task_lengths.is_none()
        && args.job_tasks.is_none()
        && args.deadline.is_none()
        && args.num_processors.is_none()
        && args.schedules.is_none()
        && args.requirements.is_none()
        && args.num_workers.is_none()
        && args.num_periods.is_none()
        && args.num_craftsmen.is_none()
        && args.num_tasks.is_none()
        && args.craftsman_avail.is_none()
        && args.task_avail.is_none()
        && args.alphabet_size.is_none()
        && args.num_groups.is_none()
        && args.num_sectors.is_none()
        && args.dependencies.is_none()
        && args.num_attributes.is_none()
        && args.source_string.is_none()
        && args.target_string.is_none()
        && args.pointer_cost.is_none()
        && args.capacities.is_none()
        && args.source_1.is_none()
        && args.sink_1.is_none()
        && args.source_2.is_none()
        && args.sink_2.is_none()
        && args.requirement_1.is_none()
        && args.requirement_2.is_none()
        && args.requirement.is_none()
        && args.homologous_pairs.is_none()
        && args.num_attributes.is_none()
        && args.dependencies.is_none()
        && args.relation_attrs.is_none()
        && args.known_keys.is_none()
        && args.num_objects.is_none()
        && args.attribute_domains.is_none()
        && args.frequency_tables.is_none()
        && args.known_values.is_none()
        && args.domain_size.is_none()
        && args.relations.is_none()
        && args.conjuncts_spec.is_none()
        && args.expression.is_none()
        && args.deps.is_none()
        && args.query.is_none()
        && args.equations.is_none()
        && args.coeff_a.is_none()
        && args.coeff_b.is_none()
        && args.rhs.is_none()
        && args.coeff_c.is_none()
        && args.pairs.is_none()
        && args.required_columns.is_none()
        && args.compilers.is_none()
        && args.setup_times.is_none()
        && args.w_sizes.is_none()
        && args.x_sizes.is_none()
        && args.y_sizes.is_none()
        && args.assignment.is_none()
        && args.initial_marking.is_none()
        && args.output_arcs.is_none()
        && args.gate_types.is_none()
        && args.inputs.is_none()
        && args.outputs.is_none()
        && args.true_sentences.is_none()
        && args.implications.is_none()
        && args.loop_length.is_none()
        && args.loop_variables.is_none()
        && args.assignments.is_none()
        && args.num_variables.is_none()
        && args.truth_table.is_none()
        && args.test_matrix.is_none()
        && args.num_tests.is_none()
        && args.tiles.is_none()
        && args.grid_size.is_none()
        && args.num_colors.is_none()
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

fn parse_precedence_pairs(raw: Option<&str>) -> Result<Vec<(usize, usize)>> {
    raw.filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|pair| {
                    let pair = pair.trim();
                    let (pred, succ) = pair.split_once('>').ok_or_else(|| {
                        anyhow::anyhow!(
                            "Invalid --precedences value '{}': expected 'u>v'",
                            pair
                        )
                    })?;
                    let pred = pred.trim().parse::<usize>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --precedences value '{}': expected 'u>v' with nonnegative integer indices",
                            pair
                        )
                    })?;
                    let succ = succ.trim().parse::<usize>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --precedences value '{}': expected 'u>v' with nonnegative integer indices",
                            pair
                        )
                    })?;
                    Ok((pred, succ))
                })
                .collect()
        })
        .unwrap_or_else(|| Ok(vec![]))
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

fn validate_precedence_pairs(precedences: &[(usize, usize)], num_tasks: usize) -> Result<()> {
    for &(pred, succ) in precedences {
        anyhow::ensure!(
            pred < num_tasks && succ < num_tasks,
            "precedence index out of range: ({}, {}) but num_tasks = {}",
            pred,
            succ,
            num_tasks
        );
    }
    Ok(())
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
    if args.random || !all_data_flags_empty(args) {
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
    let rgraph = problemreductions::rules::ReductionGraph::new();
    let resolved = match resolve_problem_ref(problem, &rgraph) {
        Ok(resolved) => resolved,
        Err(graph_err) => match resolve_catalog_problem_ref(problem) {
            Ok(catalog_resolved) => {
                if rgraph.variants_for(catalog_resolved.name()).is_empty() {
                    ProblemRef {
                        name: catalog_resolved.name().to_string(),
                        variant: catalog_resolved.variant().clone(),
                    }
                } else {
                    return Err(graph_err);
                }
            }
            Err(catalog_err) => {
                let spec = parse_problem_spec(problem)?;
                if rgraph.variants_for(&spec.name).is_empty() {
                    return Err(catalog_err);
                }
                return Err(graph_err);
            }
        },
    };
    let canonical = resolved.name.as_str();
    let resolved_variant = resolved.variant.clone();

    if args.random {
        return create_random(args, canonical, &resolved_variant, out);
    }

    // ILP and CircuitSAT have complex input structures not suited for CLI flags.
    // Check before the empty-flags help so they get a clear message.
    if canonical == "ILP" || canonical == "CircuitSAT" {
        bail!(
            "CLI creation is not yet supported for {canonical}.\n\n\
             {canonical} instances are typically created via reduction:\n\
               pred create MIS --graph 0-1,1-2 | pred reduce - --to {canonical}\n\n\
             Or use the Rust API for direct construction."
        );
    }

    // Show schema-driven help when no data flags are provided
    if all_data_flags_empty(args) {
        print_problem_help(canonical, &resolved_variant)?;
        std::process::exit(2);
    }

    let (data, variant) = create_schema_driven(args, canonical, &resolved_variant)?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Schema-driven creation unexpectedly returned no instance for {canonical}. This indicates a missing parser, flag mapping, derived field, or schema/factory mismatch in create.rs."
            )
        })?;

    let output = ProblemJsonOutput {
        problem_type: canonical.to_string(),
        variant,
        data,
    };

    emit_problem_output(&output, out)
}

/// Reject non-unit weights when the resolved variant uses `weight=One`.
fn reject_nonunit_weights_for_one_variant(
    canonical: &str,
    graph_type: &str,
    variant: &BTreeMap<String, String>,
    weights: &[i32],
) -> Result<()> {
    if variant.get("weight").map(|w| w.as_str()) == Some("One") && weights.iter().any(|&w| w != 1) {
        bail!(
            "Non-unit weights are not supported for the default unit-weight variant.\n\n\
             Use the weighted variant instead:\n  \
             pred create {canonical}/{graph_type}/i32 --graph ... --weights ..."
        );
    }
    Ok(())
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

fn parse_bipartite_problem_input(
    args: &CreateArgs,
    canonical: &str,
    k_description: &str,
    usage: &str,
) -> Result<(BipartiteGraph, usize)> {
    let left = args.left.ok_or_else(|| {
        anyhow::anyhow!(
            "{canonical} requires --left, --right, --biedges, and --k\n\nUsage: {usage}"
        )
    })?;
    let right = args.right.ok_or_else(|| {
        anyhow::anyhow!("{canonical} requires --right (right partition size)\n\nUsage: {usage}")
    })?;
    let k = args.k.ok_or_else(|| {
        anyhow::anyhow!("{canonical} requires --k ({k_description})\n\nUsage: {usage}")
    })?;
    let edges_str = args.biedges.as_deref().ok_or_else(|| {
        anyhow::anyhow!("{canonical} requires --biedges (e.g., 0-0,0-1,1-1)\n\nUsage: {usage}")
    })?;
    let edges = util::parse_edge_pairs(edges_str)?;
    validate_bipartite_edges(canonical, left, right, &edges)?;
    Ok((BipartiteGraph::new(left, right, edges), k))
}

fn validate_bipartite_edges(
    canonical: &str,
    left: usize,
    right: usize,
    edges: &[(usize, usize)],
) -> Result<()> {
    for &(u, v) in edges {
        if u >= left {
            bail!("{canonical} edge {u}-{v} is out of bounds for left partition size {left}");
        }
        if v >= right {
            bail!("{canonical} edge {u}-{v} is out of bounds for right partition size {right}");
        }
    }
    Ok(())
}

/// Parse `--graph` into a SimpleGraph, optionally preserving isolated vertices
/// via `--num-vertices`.
fn parse_graph(args: &CreateArgs) -> Result<(SimpleGraph, usize)> {
    let edges_str = args
        .graph
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --graph (e.g., 0-1,1-2,2-3)"))?;

    if edges_str.trim().is_empty() {
        let num_vertices = args.num_vertices.ok_or_else(|| {
            anyhow::anyhow!(
                "Empty graph string. To create a graph with isolated vertices, pass --num-vertices N as well."
            )
        })?;
        return Ok((SimpleGraph::empty(num_vertices), num_vertices));
    }

    let edges: Vec<(usize, usize)> = edges_str
        .split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split('-').collect();
            if parts.len() != 2 {
                bail!("Invalid edge '{}': expected format u-v", pair.trim());
            }
            let u: usize = parts[0].parse()?;
            let v: usize = parts[1].parse()?;
            if u == v {
                bail!(
                    "Self-loop detected: edge {}-{}. Simple graphs do not allow self-loops",
                    u,
                    v
                );
            }
            Ok((u, v))
        })
        .collect::<Result<Vec<_>>>()?;

    let inferred_num_vertices = edges
        .iter()
        .flat_map(|(u, v)| [*u, *v])
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);
    let num_vertices = match args.num_vertices {
        Some(explicit) if explicit < inferred_num_vertices => {
            bail!(
                "--num-vertices {} is too small for the provided graph; need at least {}",
                explicit,
                inferred_num_vertices
            );
        }
        Some(explicit) => explicit,
        None => inferred_num_vertices,
    };

    Ok((SimpleGraph::new(num_vertices, edges), num_vertices))
}

/// Parse `--positions` as integer grid positions.
fn parse_int_positions(args: &CreateArgs) -> Result<Vec<(i32, i32)>> {
    let pos_str = args.positions.as_deref().ok_or_else(|| {
        anyhow::anyhow!("This variant requires --positions (e.g., \"0,0;1,0;1,1\")")
    })?;
    util::parse_positions(pos_str, "0,0")
}

/// Parse `--positions` as float positions.
fn parse_float_positions(args: &CreateArgs) -> Result<Vec<(f64, f64)>> {
    let pos_str = args.positions.as_deref().ok_or_else(|| {
        anyhow::anyhow!("This variant requires --positions (e.g., \"0.0,0.0;1.0,0.0;0.5,0.87\")")
    })?;
    util::parse_positions(pos_str, "0.0,0.0")
}

/// Parse `--weights` as vertex weights (i32), defaulting to all 1s.
fn parse_vertex_weights(args: &CreateArgs, num_vertices: usize) -> Result<Vec<i32>> {
    match &args.weights {
        Some(w) => {
            let weights: Vec<i32> = w
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if weights.len() != num_vertices {
                bail!(
                    "Expected {} weights but got {}",
                    num_vertices,
                    weights.len()
                );
            }
            Ok(weights)
        }
        None => Ok(vec![1i32; num_vertices]),
    }
}

fn parse_i32_edge_values(
    values: Option<&String>,
    num_edges: usize,
    value_label: &str,
) -> Result<Vec<i32>> {
    match values {
        Some(raw) => {
            let parsed: Vec<i32> = raw
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if parsed.len() != num_edges {
                bail!(
                    "Expected {} {} values but got {}",
                    num_edges,
                    value_label,
                    parsed.len()
                );
            }
            Ok(parsed)
        }
        None => Ok(vec![1i32; num_edges]),
    }
}

fn parse_vertex_i64_values(
    raw: Option<&str>,
    field_name: &str,
    num_vertices: usize,
    problem_name: &str,
    usage: &str,
) -> Result<Vec<i64>> {
    let raw =
        raw.ok_or_else(|| anyhow::anyhow!("{problem_name} requires --{field_name}\n\n{usage}"))?;
    let values: Vec<i64> = util::parse_comma_list(raw)
        .map_err(|e| anyhow::anyhow!("invalid {field_name} list: {e}\n\n{usage}"))?;
    if values.len() != num_vertices {
        bail!(
            "Expected {} {} values but got {}\n\n{}",
            num_vertices,
            field_name,
            values.len(),
            usage
        );
    }
    Ok(values)
}

/// Parse `--terminals` as comma-separated vertex indices.
fn parse_terminals(args: &CreateArgs, num_vertices: usize) -> Result<Vec<usize>> {
    let s = args
        .terminals
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("--terminals required (e.g., \"0,2,4\")"))?;
    let terminals: Vec<usize> = s
        .split(',')
        .map(|t| t.trim().parse::<usize>())
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("invalid terminal index")?;
    for &t in &terminals {
        anyhow::ensure!(
            t < num_vertices,
            "terminal {t} >= num_vertices ({num_vertices})"
        );
    }
    let distinct_terminals: BTreeSet<_> = terminals.iter().copied().collect();
    anyhow::ensure!(
        distinct_terminals.len() == terminals.len(),
        "terminals must be distinct"
    );
    anyhow::ensure!(terminals.len() >= 2, "at least 2 terminals required");
    Ok(terminals)
}

/// Parse `--terminal-pairs` as comma-separated `u-v` vertex pairs.
fn parse_terminal_pairs(args: &CreateArgs, num_vertices: usize) -> Result<Vec<(usize, usize)>> {
    let raw = args
        .terminal_pairs
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("--terminal-pairs required (e.g., \"0-3,2-5\")"))?;
    let terminal_pairs = util::parse_edge_pairs(raw)?;
    anyhow::ensure!(
        !terminal_pairs.is_empty(),
        "at least 1 terminal pair required"
    );

    let mut used = BTreeSet::new();
    for &(source, sink) in &terminal_pairs {
        anyhow::ensure!(
            source < num_vertices,
            "terminal pair source {source} >= num_vertices ({num_vertices})"
        );
        anyhow::ensure!(
            sink < num_vertices,
            "terminal pair sink {sink} >= num_vertices ({num_vertices})"
        );
        anyhow::ensure!(source != sink, "terminal pair endpoints must be distinct");
        anyhow::ensure!(
            used.insert(source) && used.insert(sink),
            "terminal vertices must be pairwise disjoint across terminal pairs"
        );
    }

    Ok(terminal_pairs)
}

fn ensure_positive_i32_values(values: &[i32], label: &str) -> Result<()> {
    if values.iter().any(|&value| value <= 0) {
        bail!("All {label} must be positive (> 0)");
    }
    Ok(())
}

fn ensure_positive_i32(value: i32, label: &str) -> Result<()> {
    if value <= 0 {
        bail!("{label} must be positive (> 0)");
    }
    Ok(())
}

fn ensure_vertex_in_bounds(vertex: usize, num_vertices: usize, label: &str) -> Result<()> {
    if vertex >= num_vertices {
        bail!("{label} {vertex} out of bounds (graph has {num_vertices} vertices)");
    }
    Ok(())
}

/// Parse `--edge-weights` as per-edge numeric values (i32), defaulting to all 1s.
fn parse_edge_weights(args: &CreateArgs, num_edges: usize) -> Result<Vec<i32>> {
    parse_i32_edge_values(args.edge_weights.as_ref(), num_edges, "edge weight")
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

/// Parse `--capacities` as edge capacities (u64).
fn parse_capacities(args: &CreateArgs, num_edges: usize, usage: &str) -> Result<Vec<u64>> {
    let capacities = args
        .capacities
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --capacities\n\n{usage}"))?;
    let capacities: Vec<u64> = capacities
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            trimmed
                .parse::<u64>()
                .with_context(|| format!("Invalid capacity `{trimmed}`\n\n{usage}"))
        })
        .collect::<Result<Vec<_>>>()?;
    if capacities.len() != num_edges {
        bail!(
            "Expected {} capacities but got {}\n\n{}",
            num_edges,
            capacities.len(),
            usage
        );
    }
    Ok(capacities)
}

/// Parse `--lower-bounds` as edge lower bounds (u64).
fn parse_lower_bounds(args: &CreateArgs, num_edges: usize, usage: &str) -> Result<Vec<u64>> {
    let lower_bounds = args.lower_bounds.as_deref().ok_or_else(|| {
        anyhow::anyhow!("UndirectedFlowLowerBounds requires --lower-bounds\n\n{usage}")
    })?;
    let lower_bounds: Vec<u64> = lower_bounds
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            trimmed
                .parse::<u64>()
                .with_context(|| format!("Invalid lower bound `{trimmed}`\n\n{usage}"))
        })
        .collect::<Result<Vec<_>>>()?;
    if lower_bounds.len() != num_edges {
        bail!(
            "Expected {} lower bounds but got {}\n\n{}",
            num_edges,
            lower_bounds.len(),
            usage
        );
    }
    Ok(lower_bounds)
}

fn parse_bundle_capacities(args: &CreateArgs, num_bundles: usize, usage: &str) -> Result<Vec<u64>> {
    let capacities = args.bundle_capacities.as_deref().ok_or_else(|| {
        anyhow::anyhow!("IntegralFlowBundles requires --bundle-capacities\n\n{usage}")
    })?;
    let capacities: Vec<u64> = capacities
        .split(',')
        .map(|s| {
            let trimmed = s.trim();
            trimmed
                .parse::<u64>()
                .with_context(|| format!("Invalid bundle capacity `{trimmed}`\n\n{usage}"))
        })
        .collect::<Result<Vec<_>>>()?;
    anyhow::ensure!(
        capacities.len() == num_bundles,
        "Expected {} bundle capacities but got {}\n\n{}",
        num_bundles,
        capacities.len(),
        usage
    );
    for (bundle_index, &capacity) in capacities.iter().enumerate() {
        let fits = usize::try_from(capacity)
            .ok()
            .and_then(|value| value.checked_add(1))
            .is_some();
        anyhow::ensure!(
            fits,
            "bundle capacity {} at bundle index {} is too large for this platform\n\n{}",
            capacity,
            bundle_index,
            usage
        );
        anyhow::ensure!(
            capacity > 0,
            "bundle capacity at bundle index {} must be positive\n\n{}",
            bundle_index,
            usage
        );
    }
    Ok(capacities)
}

/// Parse `--couplings` as SpinGlass pairwise couplings (i32), defaulting to all 1s.
/// Parse `--fields` as SpinGlass on-site fields (i32), defaulting to all 0s.
/// Check if a CLI string value contains float syntax (a decimal point).
/// Parse `--couplings` as SpinGlass pairwise couplings (f64), defaulting to all 1.0.
/// Parse `--fields` as SpinGlass on-site fields (f64), defaulting to all 0.0.
/// Parse `--clauses` as semicolon-separated clauses of comma-separated literals.
/// E.g., "1,2;-1,3;2,-3"
/// Parse `--subsets` as semicolon-separated sets of comma-separated usize.
/// E.g., "0,1;1,2;0,2"
fn parse_sets(args: &CreateArgs) -> Result<Vec<Vec<usize>>> {
    parse_named_sets(args.sets.as_deref(), "--subsets")
}

fn parse_named_sets(sets_str: Option<&str>, flag: &str) -> Result<Vec<Vec<usize>>> {
    let sets_str = sets_str
        .ok_or_else(|| anyhow::anyhow!("This problem requires {flag} (e.g., \"0,1;1,2;0,2\")"))?;
    sets_str
        .split(';')
        .map(|set| {
            set.trim()
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse::<usize>()
                        .map_err(|e| anyhow::anyhow!("Invalid set element: {}", e))
                })
                .collect()
        })
        .collect()
}

fn parse_homologous_pairs(args: &CreateArgs) -> Result<Vec<(usize, usize)>> {
    let pairs = args.homologous_pairs.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "IntegralFlowHomologousArcs requires --homologous-pairs (e.g., \"2=5;4=3\")"
        )
    })?;

    pairs
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let entry = entry.trim();
            let (left, right) = entry.split_once('=').ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid homologous pair '{}': expected format u=v (e.g., 2=5)",
                    entry
                )
            })?;
            let left = left.trim().parse::<usize>().with_context(|| {
                format!("Invalid homologous pair '{}': expected format u=v", entry)
            })?;
            let right = right.trim().parse::<usize>().with_context(|| {
                format!("Invalid homologous pair '{}': expected format u=v", entry)
            })?;
            Ok((left, right))
        })
        .collect()
}

/// Parse a dependency string as semicolon-separated `lhs>rhs` pairs.
/// E.g., "0,1>2,3;2,3>0,1"
/// Parse a comma-separated list of usize indices.
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

fn validate_comparative_containment_sets(
    family_name: &str,
    flag: &str,
    universe_size: usize,
    sets: &[Vec<usize>],
) -> Result<()> {
    for (set_index, set) in sets.iter().enumerate() {
        for &element in set {
            anyhow::ensure!(
                element < universe_size,
                "{family_name} set {set_index} from {flag} contains element {element} outside universe of size {universe_size}"
            );
        }
    }
    Ok(())
}

/// Parse `--partition` as semicolon-separated groups of comma-separated arc indices.
/// E.g., "0,1;2,3;4,7;5,6"
fn parse_partition_groups(args: &CreateArgs, num_arcs: usize) -> Result<Vec<Vec<usize>>> {
    let partition_str = args.partition.as_deref().ok_or_else(|| {
        anyhow::anyhow!("MultipleChoiceBranching requires --partition (e.g., \"0,1;2,3;4,7;5,6\")")
    })?;

    let partition: Vec<Vec<usize>> = partition_str
        .split(';')
        .map(|group| {
            group
                .trim()
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse::<usize>()
                        .map_err(|e| anyhow::anyhow!("Invalid partition index: {}", e))
                })
                .collect()
        })
        .collect::<Result<_>>()?;

    let mut seen = vec![false; num_arcs];
    for group in &partition {
        for &arc_index in group {
            anyhow::ensure!(
                arc_index < num_arcs,
                "partition arc index {} out of range for {} arcs",
                arc_index,
                num_arcs
            );
            anyhow::ensure!(
                !seen[arc_index],
                "partition arc index {} appears more than once",
                arc_index
            );
            seen[arc_index] = true;
        }
    }
    anyhow::ensure!(
        seen.iter().all(|present| *present),
        "partition must cover every arc exactly once"
    );

    Ok(partition)
}

fn parse_bundles(args: &CreateArgs, num_arcs: usize, usage: &str) -> Result<Vec<Vec<usize>>> {
    let bundles_str = args
        .bundles
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --bundles\n\n{usage}"))?;

    let bundles: Vec<Vec<usize>> = bundles_str
        .split(';')
        .map(|bundle| {
            let bundle = bundle.trim();
            anyhow::ensure!(
                !bundle.is_empty(),
                "IntegralFlowBundles does not allow empty bundle entries\n\n{usage}"
            );
            bundle
                .split(',')
                .map(|s| {
                    s.trim().parse::<usize>().with_context(|| {
                        format!("Invalid bundle arc index `{}`\n\n{usage}", s.trim())
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<_>>()?;

    let mut seen_overall = vec![false; num_arcs];
    for (bundle_index, bundle) in bundles.iter().enumerate() {
        let mut seen_in_bundle = BTreeSet::new();
        for &arc_index in bundle {
            anyhow::ensure!(
                arc_index < num_arcs,
                "bundle {bundle_index} references arc {arc_index}, but num_arcs is {num_arcs}\n\n{usage}"
            );
            anyhow::ensure!(
                seen_in_bundle.insert(arc_index),
                "bundle {bundle_index} contains duplicate arc index {arc_index}\n\n{usage}"
            );
            seen_overall[arc_index] = true;
        }
    }
    anyhow::ensure!(
        seen_overall.iter().all(|covered| *covered),
        "bundles must cover every arc at least once\n\n{usage}"
    );

    Ok(bundles)
}

fn parse_multiple_choice_branching_threshold(args: &CreateArgs, usage: &str) -> Result<i32> {
    let raw_bound = args.bound.ok_or_else(|| {
        anyhow::anyhow!("MultipleChoiceBranching requires --threshold\n\n{usage}")
    })?;
    anyhow::ensure!(
        raw_bound >= 0,
        "MultipleChoiceBranching threshold must be non-negative, got {raw_bound}"
    );
    i32::try_from(raw_bound).map_err(|_| {
        anyhow::anyhow!(
            "MultipleChoiceBranching threshold must fit in a 32-bit signed integer, got {raw_bound}"
        )
    })
}

/// Parse `--weights` for set-based problems (i32), defaulting to all 1s.
fn parse_named_set_weights(
    weights_str: Option<&str>,
    num_sets: usize,
    flag: &str,
) -> Result<Vec<i32>> {
    match weights_str {
        Some(w) => {
            let weights: Vec<i32> = util::parse_comma_list(w)?;
            if weights.len() != num_sets {
                bail!(
                    "Expected {} values for {} but got {}",
                    num_sets,
                    flag,
                    weights.len()
                );
            }
            Ok(weights)
        }
        None => Ok(vec![1i32; num_sets]),
    }
}

fn parse_named_set_weights_f64(
    weights_str: Option<&str>,
    num_sets: usize,
    flag: &str,
) -> Result<Vec<f64>> {
    match weights_str {
        Some(w) => {
            let weights: Vec<f64> = util::parse_comma_list(w)?;
            if weights.len() != num_sets {
                bail!(
                    "Expected {} values for {} but got {}",
                    num_sets,
                    flag,
                    weights.len()
                );
            }
            Ok(weights)
        }
        None => Ok(vec![1.0f64; num_sets]),
    }
}

fn validate_comparative_containment_i32_weights(
    family_name: &str,
    flag: &str,
    weights: &[i32],
) -> Result<()> {
    for (index, weight) in weights.iter().enumerate() {
        anyhow::ensure!(
            *weight > 0,
            "{family_name} weights from {flag} must be positive; found {weight} at index {index}"
        );
    }
    Ok(())
}

fn validate_comparative_containment_f64_weights(
    family_name: &str,
    flag: &str,
    weights: &[f64],
) -> Result<()> {
    for (index, weight) in weights.iter().enumerate() {
        anyhow::ensure!(
            weight.is_finite() && *weight > 0.0,
            "{family_name} weights from {flag} must be finite and positive; found {weight} at index {index}"
        );
    }
    Ok(())
}

/// Parse `--matrix` as semicolon-separated rows of comma-separated bool values (0/1).
/// E.g., "1,0;0,1;1,1"
fn parse_bool_matrix(args: &CreateArgs) -> Result<Vec<Vec<bool>>> {
    let matrix_str = args
        .matrix
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --matrix (e.g., \"1,0;0,1;1,1\")"))?;
    parse_bool_rows(matrix_str)
}

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

fn parse_named_u64_list(
    raw: Option<&str>,
    problem: &str,
    flag: &str,
    usage: &str,
) -> Result<Vec<u64>> {
    let raw = raw.ok_or_else(|| anyhow::anyhow!("{problem} requires {flag}\n\n{usage}"))?;
    util::parse_comma_list(raw).map_err(|err| anyhow::anyhow!("{err}\n\n{usage}"))
}

fn ensure_named_len(len: usize, expected: usize, flag: &str, usage: &str) -> Result<()> {
    anyhow::ensure!(
        len == expected,
        "{flag} must contain exactly {expected} entries\n\n{usage}"
    );
    Ok(())
}

fn parse_named_bool_rows(rows: Option<&str>, flag: &str, usage: &str) -> Result<Vec<Vec<bool>>> {
    let rows = rows.ok_or_else(|| anyhow::anyhow!("TimetableDesign requires {flag}\n\n{usage}"))?;
    parse_bool_rows(rows).map_err(|err| {
        let message = err.to_string().replace("--matrix", flag);
        anyhow::anyhow!("{message}\n\n{usage}")
    })
}

fn parse_timetable_requirements(requirements: Option<&str>, usage: &str) -> Result<Vec<Vec<u64>>> {
    let requirements = requirements
        .ok_or_else(|| anyhow::anyhow!("TimetableDesign requires --requirements\n\n{usage}"))?;
    let matrix: Vec<Vec<u64>> = requirements
        .split(';')
        .map(|row| util::parse_comma_list(row.trim()))
        .collect::<Result<_>>()?;

    if let Some(expected_width) = matrix.first().map(Vec::len) {
        anyhow::ensure!(
            matrix.iter().all(|row| row.len() == expected_width),
            "All rows in --requirements must have the same length"
        );
    }

    Ok(matrix)
}

fn validate_timetable_design_args(
    num_periods: usize,
    num_craftsmen: usize,
    num_tasks: usize,
    craftsman_avail: &[Vec<bool>],
    task_avail: &[Vec<bool>],
    requirements: &[Vec<u64>],
    usage: &str,
) -> Result<()> {
    anyhow::ensure!(
        craftsman_avail.len() == num_craftsmen,
        "craftsman availability row count ({}) must equal num_craftsmen ({})\n\n{}",
        craftsman_avail.len(),
        num_craftsmen,
        usage
    );
    anyhow::ensure!(
        task_avail.len() == num_tasks,
        "task availability row count ({}) must equal num_tasks ({})\n\n{}",
        task_avail.len(),
        num_tasks,
        usage
    );
    anyhow::ensure!(
        requirements.len() == num_craftsmen,
        "requirements row count ({}) must equal num_craftsmen ({})\n\n{}",
        requirements.len(),
        num_craftsmen,
        usage
    );

    for (index, row) in craftsman_avail.iter().enumerate() {
        anyhow::ensure!(
            row.len() == num_periods,
            "craftsman availability row {} has {} periods, expected {}\n\n{}",
            index,
            row.len(),
            num_periods,
            usage
        );
    }
    for (index, row) in task_avail.iter().enumerate() {
        anyhow::ensure!(
            row.len() == num_periods,
            "task availability row {} has {} periods, expected {}\n\n{}",
            index,
            row.len(),
            num_periods,
            usage
        );
    }
    for (index, row) in requirements.iter().enumerate() {
        anyhow::ensure!(
            row.len() == num_tasks,
            "requirements row {} has {} tasks, expected {}\n\n{}",
            index,
            row.len(),
            num_tasks,
            usage
        );
    }

    Ok(())
}

/// Parse `--matrix` as semicolon-separated rows of comma-separated f64 values.
/// E.g., "1,0.5;0.5,2"
fn parse_matrix(args: &CreateArgs) -> Result<Vec<Vec<f64>>> {
    let matrix_str = args
        .matrix
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("QUBO requires --matrix (e.g., \"1,0.5;0.5,2\")"))?;

    matrix_str
        .split(';')
        .map(|row| {
            row.trim()
                .split(',')
                .map(|s| {
                    s.trim()
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!("Invalid matrix value: {}", e))
                })
                .collect()
        })
        .collect()
}

fn parse_u64_matrix_rows(matrix_str: &str, matrix_name: &str) -> Result<Vec<Vec<u64>>> {
    matrix_str
        .split(';')
        .enumerate()
        .map(|(row_index, row)| {
            let row = row.trim();
            anyhow::ensure!(
                !row.is_empty(),
                "{matrix_name} row {row_index} must not be empty"
            );
            row.split(',')
                .map(|value| {
                    value.trim().parse::<u64>().map_err(|error| {
                        anyhow::anyhow!(
                            "Invalid {matrix_name} row {row_index} value {:?}: {}",
                            value.trim(),
                            error
                        )
                    })
                })
                .collect()
        })
        .collect()
}

/// Parse `--quantifiers` as comma-separated quantifier labels (E/A or Exists/ForAll).
/// E.g., "E,A,E" or "Exists,ForAll,Exists"
/// Parse a semicolon-separated matrix of i64 values.
/// E.g., "0,5;5,0"
fn parse_potential_edges(args: &CreateArgs) -> Result<Vec<(usize, usize, i32)>> {
    let edges_str = args.potential_edges.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "BiconnectivityAugmentation requires --potential-weights (e.g., 0-2:3,1-3:5)"
        )
    })?;

    edges_str
        .split(',')
        .map(|entry| {
            let entry = entry.trim();
            let (edge_part, weight_part) = entry.split_once(':').ok_or_else(|| {
                anyhow::anyhow!("Invalid potential edge '{entry}': expected u-v:w")
            })?;
            let (u_str, v_str) = edge_part.split_once('-').ok_or_else(|| {
                anyhow::anyhow!("Invalid potential edge '{entry}': expected u-v:w")
            })?;
            let u = u_str.trim().parse::<usize>()?;
            let v = v_str.trim().parse::<usize>()?;
            if u == v {
                bail!("Self-loop detected in potential edge {u}-{v}");
            }
            let weight = weight_part.trim().parse::<i32>()?;
            Ok((u, v, weight))
        })
        .collect()
}

fn validate_potential_edges(
    graph: &SimpleGraph,
    potential_edges: &[(usize, usize, i32)],
) -> Result<()> {
    let num_vertices = graph.num_vertices();
    let mut seen_potential_edges = BTreeSet::new();
    for &(u, v, _) in potential_edges {
        if u >= num_vertices || v >= num_vertices {
            bail!(
                "Potential edge {u}-{v} references a vertex outside the graph (num_vertices = {num_vertices})"
            );
        }
        let edge = if u <= v { (u, v) } else { (v, u) };
        if graph.has_edge(edge.0, edge.1) {
            bail!(
                "Potential edge {}-{} already exists in the graph",
                edge.0,
                edge.1
            );
        }
        if !seen_potential_edges.insert(edge) {
            bail!(
                "Duplicate potential edge {}-{} is not allowed",
                edge.0,
                edge.1
            );
        }
    }
    Ok(())
}

fn parse_budget(args: &CreateArgs) -> Result<i32> {
    let budget = args
        .budget
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("BiconnectivityAugmentation requires --budget (e.g., 5)"))?;
    budget
        .parse::<i32>()
        .map_err(|e| anyhow::anyhow!("Invalid budget '{budget}': {e}"))
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

fn parse_prescribed_paths(
    args: &CreateArgs,
    num_arcs: usize,
    usage: &str,
) -> Result<Vec<Vec<usize>>> {
    let paths_str = args
        .paths
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("PathConstrainedNetworkFlow requires --paths\n\n{usage}"))?;

    paths_str
        .split(';')
        .map(|path_str| {
            let trimmed = path_str.trim();
            anyhow::ensure!(
                !trimmed.is_empty(),
                "PathConstrainedNetworkFlow paths must be non-empty\n\n{usage}"
            );
            let path: Vec<usize> = util::parse_comma_list(trimmed)?;
            anyhow::ensure!(
                !path.is_empty(),
                "PathConstrainedNetworkFlow paths must be non-empty\n\n{usage}"
            );
            for &arc_idx in &path {
                anyhow::ensure!(
                    arc_idx < num_arcs,
                    "Path arc index {arc_idx} out of bounds for {num_arcs} arcs\n\n{usage}"
                );
            }
            Ok(path)
        })
        .collect()
}

fn parse_mixed_graph(args: &CreateArgs, usage: &str) -> Result<MixedGraph> {
    let (undirected_graph, num_vertices) =
        parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
    let arcs_str = args
        .arcs
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("MixedChinesePostman requires --arcs\n\n{usage}"))?;
    let (directed_graph, _) = parse_directed_graph(arcs_str, Some(num_vertices))
        .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
    Ok(MixedGraph::new(
        num_vertices,
        directed_graph.arcs(),
        undirected_graph.edges(),
    ))
}

/// Parse `--weights` as arc weights (i32), defaulting to all 1s.
fn parse_arc_weights(args: &CreateArgs, num_arcs: usize) -> Result<Vec<i32>> {
    match &args.weights {
        Some(w) => {
            let weights: Vec<i32> = w
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if weights.len() != num_arcs {
                bail!(
                    "Expected {} arc weights but got {}",
                    num_arcs,
                    weights.len()
                );
            }
            Ok(weights)
        }
        None => Ok(vec![1i32; num_arcs]),
    }
}

/// Parse `--arc-weights` / `--arc-lengths` as per-arc costs (i32), defaulting to all 1s.
fn parse_arc_costs(args: &CreateArgs, num_arcs: usize) -> Result<Vec<i32>> {
    match &args.arc_costs {
        Some(costs) => {
            let parsed: Vec<i32> = costs
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if parsed.len() != num_arcs {
                bail!("Expected {} arc costs but got {}", num_arcs, parsed.len());
            }
            Ok(parsed)
        }
        None => Ok(vec![1i32; num_arcs]),
    }
}

/// Parse `--candidate-arcs` as `u>v:w` entries for StrongConnectivityAugmentation.
/// Handle `pred create <PROBLEM> --random ...`
fn create_random(
    args: &CreateArgs,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
    out: &OutputConfig,
) -> Result<()> {
    let num_vertices = args.num_vertices.ok_or_else(|| {
        anyhow::anyhow!(
            "--random requires --num-vertices\n\n\
             Usage: pred create {} --random --num-vertices 10 [--edge-prob 0.3] [--seed 42]",
            canonical
        )
    })?;

    let graph_type = resolved_graph_type(resolved_variant);

    let (data, variant) = match canonical {
        // Graph problems with vertex weights
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet"
        | "MaximalIS" => {
            let weights = vec![1i32; num_vertices];
            match graph_type {
                "KingsSubgraph" => {
                    let positions = util::create_random_int_positions(num_vertices, args.seed);
                    let graph = KingsSubgraph::new(positions);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                "TriangularSubgraph" => {
                    let positions = util::create_random_int_positions(num_vertices, args.seed);
                    let graph = TriangularSubgraph::new(positions);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                "UnitDiskGraph" => {
                    let radius = args.radius.unwrap_or(1.0);
                    let positions = util::create_random_float_positions(num_vertices, args.seed);
                    let graph = UnitDiskGraph::new(positions, radius);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                _ => {
                    let edge_prob = args.edge_prob.unwrap_or(0.5);
                    if !(0.0..=1.0).contains(&edge_prob) {
                        bail!("--edge-prob must be between 0.0 and 1.0");
                    }
                    let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
                    let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                    let data = ser_vertex_weight_problem_with(canonical, graph, weights)?;
                    (data, variant)
                }
            }
        }

        "KClique" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let usage =
                "Usage: pred create KClique --random --num-vertices 5 [--edge-prob 0.5] [--seed 42] --k 3";
            let k = parse_kclique_threshold(args.k, graph.num_vertices(), usage)?;
            (
                ser(KClique::new(graph, k))?,
                variant_map(&[("graph", "SimpleGraph")]),
            )
        }

        "VertexCover" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let usage =
                "Usage: pred create VertexCover --random --num-vertices 5 [--edge-prob 0.5] [--seed 42] --k 3";
            let k = args
                .k
                .ok_or_else(|| anyhow::anyhow!("VertexCover requires --k\n\n{usage}"))?;
            if k == 0 {
                bail!("VertexCover: --k must be positive");
            }
            if k > graph.num_vertices() {
                bail!("VertexCover: k must be <= graph num_vertices");
            }
            (
                ser(VertexCover::new(graph, k))?,
                variant_map(&[("graph", "SimpleGraph")]),
            )
        }

        // MinimumCutIntoBoundedSets (graph + edge weights + s/t/B/K)
        "MinimumCutIntoBoundedSets" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MaximumAchromaticNumber::new(graph))?,
                variant,
            )
        }

        // MaximumDomaticNumber (graph only, no weights)
        "MaximumDomaticNumber" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MaximumDomaticNumber::new(graph))?,
                variant,
            )
        }

        // MinimumCoveringByCliques (graph only, no weights)
        "MinimumCoveringByCliques" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MinimumCoveringByCliques::new(graph))?,
                variant,
            )
        }

        // MinimumIntersectionGraphBasis (graph only, no weights)
        "MinimumIntersectionGraphBasis" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MinimumIntersectionGraphBasis::new(graph))?,
                variant,
            )
        }

        // MinimumMaximalMatching (graph only, no weights)
        "MinimumMaximalMatching" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(MinimumMaximalMatching::new(graph))?, variant)
        }

        // Hamiltonian Circuit (graph only, no weights)
        "HamiltonianCircuit" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(HamiltonianCircuit::new(graph))?, variant)
        }

        // Maximum Leaf Spanning Tree (graph only, no weights)
        "MaximumLeafSpanningTree" => {
            let num_vertices = num_vertices.max(2);
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(problemreductions::models::graph::MaximumLeafSpanningTree::new(graph))?,
                variant,
            )
        }

        // HamiltonianPath (graph only, no weights)
        "HamiltonianPath" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(HamiltonianPath::new(graph))?, variant)
        }

        // HamiltonianPathBetweenTwoVertices (graph + source/target)
        "HamiltonianPathBetweenTwoVertices" => {
            let num_vertices = num_vertices.max(2);
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let source_vertex = args.source_vertex.unwrap_or(0);
            let target_vertex = args
                .target_vertex
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let edge_lengths = vec![1i32; graph.num_edges()];
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            (ser(LongestCircuit::new(graph, edge_lengths))?, variant)
        }

        // GeneralizedHex (graph only, with source/sink defaults)
        "GeneralizedHex" => {
            let num_vertices = num_vertices.max(2);
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let source = args.source.unwrap_or(0);
            let sink = args.sink.unwrap_or(num_vertices - 1);
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let source = args.source.unwrap_or(0);
            let sink = args.sink.unwrap_or(num_vertices - 1);
            let bound = args.bound.unwrap_or((num_vertices - 1) as i64);
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let mut state = util::lcg_init(args.seed);
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
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
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let (k, _variant) =
                util::validate_k_param(resolved_variant, args.k, Some(3), "KColoring")?;
            util::ser_kcoloring(graph, k)?
        }

        // OptimalLinearArrangement — graph only (optimization)
        "OptimalLinearArrangement" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(OptimalLinearArrangement::new(graph))?, variant)
        }

        // RootedTreeArrangement — graph + bound
        "RootedTreeArrangement" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let n = graph.num_vertices();
            let usage = "Usage: pred create RootedTreeArrangement --random --num-vertices 5 [--edge-prob 0.5] [--seed 42] [--bound 10]";
            let bound = args
                .bound
                .map(|b| parse_nonnegative_usize_bound(b, "RootedTreeArrangement", usage))
                .transpose()?
                .unwrap_or((n.saturating_sub(1)) * graph.num_edges());
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(RootedTreeArrangement::new(graph, bound))?, variant)
        }

        _ => bail!(
            "Random generation is not supported for {canonical}. \
             Supported: graph-based problems (MIS, MVC, MaxCut, MaxClique, \
             MaximumMatching, MinimumDominatingSet, SpinGlass, KColoring, KClique, VertexCover, TravelingSalesman, \
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
