use crate::cli::{CreateArgs, ExampleSide};
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::{
    parse_problem_spec, resolve_catalog_problem_ref, resolve_problem_ref, unknown_problem_error,
};
use crate::util;
use anyhow::{bail, Context, Result};
use problemreductions::export::{ModelExample, ProblemRef, ProblemSide, RuleExample};
use problemreductions::models::algebraic::{
    ClosestVectorProblem, ConsecutiveBlockMinimization, ConsecutiveOnesMatrixAugmentation,
    ConsecutiveOnesSubmatrix, FeasibleBasisExtension, QuadraticDiophantineEquations,
    SparseMatrixCompression, BMF,
};
use problemreductions::models::formula::Quantifier;
use problemreductions::models::graph::{
    DisjointConnectingPaths, GeneralizedHex, HamiltonianCircuit, HamiltonianPath,
    IntegralFlowBundles, LengthBoundedDisjointPaths, LongestCircuit, LongestPath,
    MinimumCutIntoBoundedSets, MinimumDummyActivitiesPert, MinimumMultiwayCut, MixedChinesePostman,
    MultipleChoiceBranching, PathConstrainedNetworkFlow, RootedTreeArrangement, SteinerTree,
    SteinerTreeInGraphs, StrongConnectivityAugmentation,
};
use problemreductions::models::misc::{
    AdditionalKey, BinPacking, BoyceCoddNormalFormViolation, CapacityAssignment, CbqRelation,
    ConjunctiveBooleanQuery, ConsistencyOfDatabaseFrequencyTables, EnsembleComputation,
    ExpectedRetrievalCost, FlowShopScheduling, FrequencyTable, GroupingBySwapping,
    JobShopScheduling, KnownValue, LongestCommonSubsequence, MinimumTardinessSequencing,
    MultiprocessorScheduling, PaintShop, PartiallyOrderedKnapsack, ProductionPlanning, QueryArg,
    RectilinearPictureCompression, ResourceConstrainedScheduling,
    SchedulingWithIndividualDeadlines, SequencingToMinimizeMaximumCumulativeCost,
    SequencingToMinimizeWeightedCompletionTime, SequencingToMinimizeWeightedTardiness,
    SequencingWithReleaseTimesAndDeadlines, SequencingWithinIntervals, ShortestCommonSupersequence,
    StringToStringCorrection, SubsetSum, SumOfSquaresPartition, ThreePartition, TimetableDesign,
};
use problemreductions::models::BiconnectivityAugmentation;
use problemreductions::prelude::*;
use problemreductions::registry::collect_schemas;
use problemreductions::topology::{
    BipartiteGraph, DirectedGraph, Graph, KingsSubgraph, MixedGraph, SimpleGraph,
    TriangularSubgraph, UnitDiskGraph,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

const MULTIPLE_COPY_FILE_ALLOCATION_EXAMPLE_ARGS: &str =
    "--graph 0-1,1-2,2-3 --usage 5,4,3,2 --storage 1,1,1,1";
const MULTIPLE_COPY_FILE_ALLOCATION_USAGE: &str =
    "Usage: pred create MultipleCopyFileAllocation --graph 0-1,1-2,2-3 --usage 5,4,3,2 --storage 1,1,1,1";
const EXPECTED_RETRIEVAL_COST_EXAMPLE_ARGS: &str =
    "--probabilities 0.2,0.15,0.15,0.2,0.1,0.2 --num-sectors 3";
const EXPECTED_RETRIEVAL_COST_USAGE: &str =
    "Usage: pred create ExpectedRetrievalCost --probabilities 0.2,0.15,0.15,0.2,0.1,0.2 --num-sectors 3";

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
        && args.num_vars.is_none()
        && args.matrix.is_none()
        && args.k.is_none()
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
        && args.cost_bound.is_none()
        && args.delay_budget.is_none()
        && args.pattern.is_none()
        && args.strings.is_none()
        && args.string.is_none()
        && args.costs.is_none()
        && args.arc_costs.is_none()
        && args.arcs.is_none()
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
        && args.deps.is_none()
        && args.query.is_none()
        && args.coeff_a.is_none()
        && args.coeff_b.is_none()
        && args.rhs.is_none()
        && args.qde_rhs.is_none()
        && args.required_columns.is_none()
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
                            "Invalid --precedence-pairs value '{}': expected 'u>v'",
                            pair
                        )
                    })?;
                    let pred = pred.trim().parse::<usize>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --precedence-pairs value '{}': expected 'u>v' with nonnegative integer indices",
                            pair
                        )
                    })?;
                    let succ = succ.trim().parse::<usize>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --precedence-pairs value '{}': expected 'u>v' with nonnegative integer indices",
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
                "Invalid --job-tasks value: empty job at position {}",
                job_index
            );

            job_str
                .split(',')
                .map(|task_str| {
                    let task_str = task_str.trim();
                    let (processor, length) = task_str.split_once(':').ok_or_else(|| {
                        anyhow::anyhow!(
                            "Invalid --job-tasks operation '{}': expected 'processor:length'",
                            task_str
                        )
                    })?;
                    let processor = processor.trim().parse::<usize>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --job-tasks operation '{}': processor must be a nonnegative integer",
                            task_str
                        )
                    })?;
                    let length = length.trim().parse::<u64>().map_err(|_| {
                        anyhow::anyhow!(
                            "Invalid --job-tasks operation '{}': length must be a nonnegative integer",
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

fn type_format_hint(type_name: &str, graph_type: Option<&str>) -> &'static str {
    match type_name {
        "SimpleGraph" => "edge list: 0-1,1-2,2-3",
        "G" => match graph_type {
            Some("KingsSubgraph" | "TriangularSubgraph") => "integer positions: \"0,0;1,0;1,1\"",
            Some("UnitDiskGraph") => "float positions: \"0.0,0.0;1.0,0.0\"",
            _ => "edge list: 0-1,1-2,2-3",
        },
        "Vec<(Vec<usize>, Vec<usize>)>" => "semicolon-separated dependencies: \"0,1>2;0,2>3\"",
        "Vec<u64>" => "comma-separated integers: 4,5,3,2,6",
        "Vec<W>" => "comma-separated: 1,2,3",
        "W" | "N" | "W::Sum" | "N::Sum" => "numeric value: 10",
        "Vec<usize>" => "comma-separated indices: 0,2,4",
        "Vec<(usize, usize, W)>" | "Vec<(usize,usize,W)>" => {
            "comma-separated weighted edges: 0-2:3,1-3:5"
        }
        "Vec<Vec<usize>>" => "semicolon-separated sets: \"0,1;1,2;0,2\"",
        "Vec<CNFClause>" => "semicolon-separated clauses: \"1,2;-1,3\"",
        "Vec<Vec<bool>>" => "JSON 2D bool array: '[[true,false],[false,true]]'",
        "Vec<Vec<W>>" => "semicolon-separated rows: \"1,0.5;0.5,2\"",
        "usize" => "integer",
        "u64" => "integer",
        "i64" => "integer",
        "BigUint" => "nonnegative decimal integer",
        "Vec<BigUint>" => "comma-separated nonnegative decimal integers: 3,7,1,8",
        "Vec<i64>" => "comma-separated integers: 3,7,1,8",
        "DirectedGraph" => "directed arcs: 0>1,1>2,2>0",
        _ => "value",
    }
}

fn example_for(canonical: &str, graph_type: Option<&str>) -> &'static str {
    match canonical {
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet" => match graph_type {
            Some("KingsSubgraph") => "--positions \"0,0;1,0;1,1;0,1\"",
            Some("TriangularSubgraph") => "--positions \"0,0;0,1;1,0;1,1\"",
            Some("UnitDiskGraph") => "--positions \"0,0;1,0;0.5,0.8\" --radius 1.5",
            _ => "--graph 0-1,1-2,2-3 --weights 1,1,1,1",
        },
        "KClique" => "--graph 0-1,0-2,1-3,2-3,2-4,3-4 --k 3",
        "GeneralizedHex" => "--graph 0-1,0-2,0-3,1-4,2-4,3-4,4-5 --source 0 --sink 5",
        "IntegralFlowBundles" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>2,2>1\" --bundles \"0,1;2,5;3,4\" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4"
        }
        "IntegralFlowWithMultipliers" => {
            "--arcs \"0>1,0>2,1>3,2>3\" --capacities 1,1,2,2 --source 0 --sink 3 --multipliers 1,2,3,1 --requirement 2"
        }
        "MinimumCutIntoBoundedSets" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1 --source 0 --sink 3 --size-bound 3"
        }
        "BoundedComponentSpanningForest" => {
            "--graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --bound 6"
        }
        "HamiltonianPath" => "--graph 0-1,1-2,2-3",
        "LongestPath" => {
            "--graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,4-6,5-6,1-6 --edge-lengths 3,2,4,1,5,2,3,2,4,1 --source-vertex 0 --target-vertex 6"
        }
        "UndirectedFlowLowerBounds" => {
            "--graph 0-1,0-2,1-3,2-3,1-4,3-5,4-5 --capacities 2,2,2,2,1,3,2 --lower-bounds 1,1,0,0,1,0,1 --source 0 --sink 5 --requirement 3"
        }
        "UndirectedTwoCommodityIntegralFlow" => {
            "--graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1"
        },
        "DisjointConnectingPaths" => {
            "--graph 0-1,1-3,0-2,1-4,2-4,3-5,4-5 --terminal-pairs 0-3,2-5"
        }
        "IntegralFlowHomologousArcs" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>4,2>4,3>5,4>5\" --capacities 1,1,1,1,1,1,1,1 --source 0 --sink 5 --requirement 2 --homologous-pairs \"2=5;4=3\""
        }
        "LengthBoundedDisjointPaths" => {
            "--graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --bound 4"
        }
        "PathConstrainedNetworkFlow" => {
            "--arcs \"0>1,0>2,1>3,1>4,2>4,3>5,4>5,4>6,5>7,6>7\" --capacities 2,1,1,1,1,1,1,1,2,1 --source 0 --sink 7 --paths \"0,2,5,8;0,3,6,8;0,3,7,9;1,4,6,8;1,4,7,9\" --requirement 3"
        }
        "IsomorphicSpanningTree" => "--graph 0-1,1-2,0-2 --tree 0-1,1-2",
        "KthBestSpanningTree" => "--graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3",
        "LongestCircuit" => {
            "--graph 0-1,1-2,2-3,3-4,4-5,5-0,0-3,1-4,2-5,3-5 --edge-weights 3,2,4,1,5,2,3,2,1,2"
        }
        "BottleneckTravelingSalesman" | "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1"
        }
        "ShortestWeightConstrainedPath" => {
            "--graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4 --edge-lengths 2,4,3,1,5,4,2,6 --edge-weights 5,1,2,3,2,3,1,1 --source-vertex 0 --target-vertex 5 --weight-bound 8"
        }
        "SteinerTreeInGraphs" => "--graph 0-1,1-2,2-3 --edge-weights 1,1,1 --terminals 0,3",
        "BiconnectivityAugmentation" => {
            "--graph 0-1,1-2,2-3 --potential-edges 0-2:3,0-3:4,1-3:2 --budget 5"
        }
        "PartialFeedbackEdgeSet" => {
            "--graph 0-1,1-2,2-0,2-3,3-4,4-2,3-5,5-4,0-3 --budget 3 --max-cycle-length 4"
        }
        "Satisfiability" => "--num-vars 3 --clauses \"1,2;-1,3\"",
        "NAESatisfiability" => "--num-vars 3 --clauses \"1,2,-3;-1,2,3\"",
        "QuantifiedBooleanFormulas" => {
            "--num-vars 3 --clauses \"1,2;-1,3\" --quantifiers \"E,A,E\""
        }
        "KSatisfiability" => "--num-vars 3 --clauses \"1,2,3;-1,2,-3\" --k 3",
        "QUBO" => "--matrix \"1,0.5;0.5,2\"",
        "QuadraticAssignment" => "--matrix \"0,5;5,0\" --distance-matrix \"0,1;1,0\"",
        "SpinGlass" => "--graph 0-1,1-2 --couplings 1,1",
        "KColoring" => "--graph 0-1,1-2,2-0 --k 3",
        "HamiltonianCircuit" => "--graph 0-1,1-2,2-3,3-0",
        "EnsembleComputation" => "--universe 4 --sets \"0,1,2;0,1,3\" --budget 4",
        "RootedTreeStorageAssignment" => "--universe 5 --sets \"0,2;1,3;0,4;2,4\" --bound 1",
        "MinMaxMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "MinimumSumMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "BalancedCompleteBipartiteSubgraph" => {
            "--left 4 --right 4 --biedges 0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3 --k 3"
        }
        "PartitionIntoTriangles" => "--graph 0-1,1-2,0-2",
        "Factoring" => "--target 15 --m 4 --n 4",
        "CapacityAssignment" => {
            "--capacities 1,2,3 --cost-matrix \"1,3,6;2,4,7;1,2,5\" --delay-matrix \"8,4,1;7,3,1;6,3,1\" --delay-budget 12"
        }
        "ProductionPlanning" => {
            "--num-periods 6 --demands 5,3,7,2,8,5 --capacities 12,12,12,12,12,12 --setup-costs 10,10,10,10,10,10 --production-costs 1,1,1,1,1,1 --inventory-costs 1,1,1,1,1,1 --cost-bound 80"
        }
        "MultiprocessorScheduling" => "--lengths 4,5,3,2,6 --num-processors 2 --deadline 10",
        "JobShopScheduling" => {
            "--job-tasks \"0:3,1:4;1:2,0:3,1:2;0:4,1:3;1:5,0:2;0:2,1:3,0:1\" --num-processors 2"
        }
        "MinimumMultiwayCut" => "--graph 0-1,1-2,2-3 --terminals 0,2 --edge-weights 1,1,1",
        "ExpectedRetrievalCost" => EXPECTED_RETRIEVAL_COST_EXAMPLE_ARGS,
        "SequencingWithinIntervals" => "--release-times 0,0,5 --deadlines 11,11,6 --lengths 3,1,1",
        "StaffScheduling" => {
            "--schedules \"1,1,1,1,1,0,0;0,1,1,1,1,1,0;0,0,1,1,1,1,1;1,0,0,1,1,1,1;1,1,0,0,1,1,1\" --requirements 2,2,2,3,3,2,1 --num-workers 4 --k 5"
        }
        "TimetableDesign" => {
            "--num-periods 3 --num-craftsmen 5 --num-tasks 5 --craftsman-avail \"1,1,1;1,1,0;0,1,1;1,0,1;1,1,1\" --task-avail \"1,1,0;0,1,1;1,0,1;1,1,1;1,1,1\" --requirements \"1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0\""
        }
        "SteinerTree" => "--graph 0-1,1-2,1-3,3-4 --edge-weights 2,2,1,1 --terminals 0,2,4",
        "MultipleCopyFileAllocation" => {
            MULTIPLE_COPY_FILE_ALLOCATION_EXAMPLE_ARGS
        }
        "AcyclicPartition" => {
            "--arcs \"0>1,0>2,1>3,1>4,2>4,2>5,3>5,4>5\" --weights 2,3,2,1,3,1 --arc-costs 1,1,1,1,1,1,1,1 --weight-bound 5 --cost-bound 5"
        }
        "OptimalLinearArrangement" => "--graph 0-1,1-2,2-3",
        "RootedTreeArrangement" => "--graph 0-1,0-2,1-2,2-3,3-4 --bound 7",
        "DirectedTwoCommodityIntegralFlow" => {
            "--arcs \"0>2,0>3,1>2,1>3,2>4,2>5,3>4,3>5\" --capacities 1,1,1,1,1,1,1,1 --source-1 0 --sink-1 4 --source-2 1 --sink-2 5 --requirement-1 1 --requirement-2 1"
        }
        "MinimumFeedbackArcSet" => "--arcs \"0>1,1>2,2>0\"",
        "MinimumDummyActivitiesPert" => "--arcs \"0>2,0>3,1>3,1>4,2>5\" --num-vertices 6",
        "StrongConnectivityAugmentation" => {
            "--arcs \"0>1,1>2\" --candidate-arcs \"2>0:1\" --bound 1"
        }
        "MixedChinesePostman" => {
            "--graph 0-2,1-3,0-4,4-2 --arcs \"0>1,1>2,2>3,3>0\" --edge-weights 2,3,1,2 --arc-costs 2,3,1,4"
        }
        "RuralPostman" => {
            "--graph 0-1,1-2,2-3,3-0 --edge-weights 1,1,1,1 --required-edges 0,2"
        }
        "StackerCrane" => {
            "--arcs \"0>4,2>5,5>1,3>0,4>3\" --graph \"0-1,1-2,2-3,3-5,4-5,0-3,1-5\" --arc-costs 3,4,2,5,3 --edge-lengths 2,1,3,2,1,4,3 --num-vertices 6"
        }
        "MultipleChoiceBranching" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --bound 10"
        }
        "AdditionalKey" => "--num-attributes 6 --dependencies \"0,1:2,3;2,3:4,5;4,5:0,1\" --relation-attrs 0,1,2,3,4,5 --known-keys \"0,1;2,3;4,5\"",
        "ConsistencyOfDatabaseFrequencyTables" => {
            "--num-objects 6 --attribute-domains \"2,3,2\" --frequency-tables \"0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1\" --known-values \"0,0,0;3,0,1;1,2,1\""
        }
        "SubgraphIsomorphism" => "--graph 0-1,1-2,2-0 --pattern 0-1",
        "RectilinearPictureCompression" => {
            "--matrix \"1,1,0,0;1,1,0,0;0,0,1,1;0,0,1,1\" --bound 2"
        }
        "SequencingToMinimizeWeightedTardiness" => {
            "--sizes 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
        }
        "IntegerKnapsack" => "--sizes 3,4,5,2,7 --values 4,5,7,3,9 --capacity 15",
        "SubsetSum" => "--sizes 3,7,1,8,2,4 --target 11",
        "ThreePartition" => "--sizes 4,5,6,4,6,5 --bound 15",
        "QuadraticDiophantineEquations" => "--coeff-a 3 --coeff-b 5 --qde-rhs 53",
        "BoyceCoddNormalFormViolation" => {
            "--n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
        }
        "SumOfSquaresPartition" => "--sizes 5,3,8,2,7,1 --num-groups 3",
        "ComparativeContainment" => {
            "--universe 4 --r-sets \"0,1,2,3;0,1\" --s-sets \"0,1,2,3;2,3\" --r-weights 2,5 --s-weights 3,6"
        }
        "SetBasis" => "--universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3",
        "LongestCommonSubsequence" => {
            "--strings \"010110;100101;001011\" --alphabet-size 2"
        }
        "GroupingBySwapping" => "--string \"0,1,2,0,1,2\" --bound 5",
        "MinimumCardinalityKey" => {
            "--num-attributes 6 --dependencies \"0,1>2;0,2>3;1,3>4;2,4>5\""
        }
        "PrimeAttributeName" => {
            "--universe 6 --deps \"0,1>2,3,4,5;2,3>0,1,4,5\" --query 3"
        }
        "TwoDimensionalConsecutiveSets" => {
            "--alphabet-size 6 --sets \"0,1,2;3,4,5;1,3;2,4;0,5\""
        }
        "ShortestCommonSupersequence" => "--strings \"0,1,2;1,2,0\"",
        "ConsecutiveBlockMinimization" => {
            "--matrix '[[true,false,true],[false,true,true]]' --bound 2"
        }
        "ConsecutiveOnesMatrixAugmentation" => {
            "--matrix \"1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0\" --bound 2"
        }
        "SparseMatrixCompression" => {
            "--matrix \"1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0\" --bound 2"
        }
        "ConjunctiveBooleanQuery" => {
            "--domain-size 6 --relations \"2:0,3|1,3|2,4;3:0,1,5|1,2,5\" --conjuncts-spec \"0:v0,c3;0:v1,c3;1:v0,v1,c5\""
        }
        "ConjunctiveQueryFoldability" => "(use --example ConjunctiveQueryFoldability)",
        "SequencingToMinimizeMaximumCumulativeCost" => {
            "--costs 2,-1,3,-2,1,-3 --precedence-pairs \"0>2,1>2,1>3,2>4,3>5,4>5\""
        }
        "StringToStringCorrection" => {
            "--source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2"
        }
        "FeasibleBasisExtension" => {
            "--matrix '[[1,0,1,2,-1,0],[0,1,0,1,1,2],[0,0,1,1,0,1]]' --rhs '7,5,3' --required-columns '0,1'"
        }
        _ => "",
    }
}

fn uses_edge_weights_flag(canonical: &str) -> bool {
    matches!(
        canonical,
        "BottleneckTravelingSalesman"
            | "KthBestSpanningTree"
            | "LongestCircuit"
            | "MaxCut"
            | "MaximumMatching"
            | "MixedChinesePostman"
            | "RuralPostman"
            | "TravelingSalesman"
    )
}

fn help_flag_name(canonical: &str, field_name: &str) -> String {
    // Problem-specific overrides first
    match (canonical, field_name) {
        ("BoundedComponentSpanningForest", "max_components") => return "k".to_string(),
        ("BoundedComponentSpanningForest", "max_weight") => return "bound".to_string(),
        ("FlowShopScheduling", "num_processors")
        | ("JobShopScheduling", "num_processors")
        | ("SchedulingWithIndividualDeadlines", "num_processors") => {
            return "num-processors/--m".to_string();
        }
        ("JobShopScheduling", "jobs") => return "job-tasks".to_string(),
        ("LengthBoundedDisjointPaths", "max_length") => return "bound".to_string(),
        ("RectilinearPictureCompression", "bound") => return "bound".to_string(),
        ("PrimeAttributeName", "num_attributes") => return "universe".to_string(),
        ("PrimeAttributeName", "dependencies") => return "deps".to_string(),
        ("PrimeAttributeName", "query_attribute") => return "query".to_string(),
        ("MixedChinesePostman", "arc_weights") => return "arc-costs".to_string(),
        ("ConsecutiveOnesMatrixAugmentation", "bound") => return "bound".to_string(),
        ("ConsecutiveOnesSubmatrix", "bound") => return "bound".to_string(),
        ("SparseMatrixCompression", "bound_k") => return "bound".to_string(),
        ("StackerCrane", "edges") => return "graph".to_string(),
        ("StackerCrane", "arc_lengths") => return "arc-costs".to_string(),
        ("StackerCrane", "edge_lengths") => return "edge-lengths".to_string(),
        ("StaffScheduling", "shifts_per_schedule") => return "k".to_string(),
        ("TimetableDesign", "num_tasks") => return "num-tasks".to_string(),
        _ => {}
    }
    // Edge-weight problems use --edge-weights instead of --weights
    if field_name == "weights" && uses_edge_weights_flag(canonical) {
        return "edge-weights".to_string();
    }
    // General field-name overrides (previously in cli_flag_name)
    match field_name {
        "universe_size" => "universe".to_string(),
        "collection" | "subsets" => "sets".to_string(),
        "left_size" => "left".to_string(),
        "right_size" => "right".to_string(),
        "edges" => "biedges".to_string(),
        "vertex_weights" => "weights".to_string(),
        "potential_weights" => "potential-edges".to_string(),
        "edge_lengths" => "edge-weights".to_string(),
        "num_tasks" => "n".to_string(),
        "precedences" => "precedence-pairs".to_string(),
        "threshold" => "bound".to_string(),
        "lengths" => "sizes".to_string(),
        _ => field_name.replace('_', "-"),
    }
}

fn reject_vertex_weights_for_edge_weight_problem(
    args: &CreateArgs,
    canonical: &str,
    graph_type: Option<&str>,
) -> Result<()> {
    if args.weights.is_some() && uses_edge_weights_flag(canonical) {
        bail!(
            "{canonical} uses --edge-weights, not --weights.\n\n\
             Usage: pred create {} {}",
            match graph_type {
                Some(g) => format!("{canonical}/{g}"),
                None => canonical.to_string(),
            },
            example_for(canonical, graph_type)
        );
    }
    Ok(())
}

fn help_flag_hint(
    canonical: &str,
    field_name: &str,
    type_name: &str,
    graph_type: Option<&str>,
) -> &'static str {
    match (canonical, field_name) {
        ("BoundedComponentSpanningForest", "max_weight") => "integer",
        ("SequencingWithinIntervals", "release_times") => "comma-separated integers: 0,0,5",
        ("DisjointConnectingPaths", "terminal_pairs") => "comma-separated pairs: 0-3,2-5",
        ("PrimeAttributeName", "dependencies") => {
            "semicolon-separated dependencies: \"0,1>2,3;2,3>0,1\""
        }
        ("LongestCommonSubsequence", "strings") => {
            "raw strings: \"ABAC;BACA\" or symbol lists: \"0,1,0;1,0,1\""
        }
        ("GroupingBySwapping", "string") => "symbol list: \"0,1,2,0,1,2\"",
        ("ShortestCommonSupersequence", "strings") => "symbol lists: \"0,1,2;1,2,0\"",
        ("MultipleChoiceBranching", "partition") => "semicolon-separated groups: \"0,1;2,3\"",
        ("IntegralFlowHomologousArcs", "homologous_pairs") => {
            "semicolon-separated arc-index equalities: \"2=5;4=3\""
        }
        ("ConsistencyOfDatabaseFrequencyTables", "attribute_domains") => {
            "comma-separated domain sizes: 2,3,2"
        }
        ("ConsistencyOfDatabaseFrequencyTables", "frequency_tables") => {
            "semicolon-separated tables: \"0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1\""
        }
        ("ConsistencyOfDatabaseFrequencyTables", "known_values") => {
            "semicolon-separated triples: \"0,0,0;3,0,1;1,2,1\""
        }
        ("IntegralFlowBundles", "bundles") => "semicolon-separated groups: \"0,1;2,5;3,4\"",
        ("IntegralFlowBundles", "bundle_capacities") => "comma-separated capacities: 1,1,1",
        ("PathConstrainedNetworkFlow", "paths") => {
            "semicolon-separated arc-index paths: \"0,2,5,8;1,4,7,9\""
        }
        ("ConsecutiveOnesMatrixAugmentation", "matrix") => {
            "semicolon-separated 0/1 rows: \"1,0;0,1\""
        }
        ("ConsecutiveOnesSubmatrix", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("SparseMatrixCompression", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("FeasibleBasisExtension", "matrix") => "JSON 2D integer array: '[[1,0,1],[0,1,0]]'",
        ("FeasibleBasisExtension", "rhs") => "comma-separated integers: \"7,5,3\"",
        ("FeasibleBasisExtension", "required_columns") => "comma-separated column indices: \"0,1\"",
        ("TimetableDesign", "craftsman_avail") | ("TimetableDesign", "task_avail") => {
            "semicolon-separated 0/1 rows: \"1,1,0;0,1,1\""
        }
        ("TimetableDesign", "requirements") => "semicolon-separated rows: \"1,0,1;0,1,0\"",
        _ => type_format_hint(type_name, graph_type),
    }
}

fn parse_nonnegative_usize_bound(bound: i64, problem_name: &str, usage: &str) -> Result<usize> {
    usize::try_from(bound)
        .map_err(|_| anyhow::anyhow!("{problem_name} requires nonnegative --bound\n\n{usage}"))
}

fn resolve_processor_count_flags(
    problem_name: &str,
    usage: &str,
    num_processors: Option<usize>,
    m_alias: Option<usize>,
) -> Result<Option<usize>> {
    match (num_processors, m_alias) {
        (Some(num_processors), Some(m_alias)) => {
            anyhow::ensure!(
                num_processors == m_alias,
                "{problem_name} received conflicting processor counts: --num-processors={num_processors} but --m={m_alias}\n\n{usage}"
            );
            Ok(Some(num_processors))
        }
        (Some(num_processors), None) => Ok(Some(num_processors)),
        (None, Some(m_alias)) => Ok(Some(m_alias)),
        (None, None) => Ok(None),
    }
}

fn validate_sequencing_within_intervals_inputs(
    release_times: &[u64],
    deadlines: &[u64],
    lengths: &[u64],
    usage: &str,
) -> Result<()> {
    if release_times.len() != deadlines.len() {
        bail!("release_times and deadlines must have the same length\n\n{usage}");
    }
    if release_times.len() != lengths.len() {
        bail!("release_times and lengths must have the same length\n\n{usage}");
    }

    for (i, ((&release_time, &deadline), &length)) in release_times
        .iter()
        .zip(deadlines.iter())
        .zip(lengths.iter())
        .enumerate()
    {
        let end = release_time.checked_add(length).ok_or_else(|| {
            anyhow::anyhow!("Task {i}: overflow computing r(i) + l(i)\n\n{usage}")
        })?;
        if end > deadline {
            bail!(
                "Task {i}: r({}) + l({}) > d({}), time window is empty\n\n{usage}",
                release_time,
                length,
                deadline
            );
        }
    }

    Ok(())
}

fn print_problem_help(canonical: &str, graph_type: Option<&str>) -> Result<()> {
    let is_geometry = matches!(
        graph_type,
        Some("KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph")
    );
    let schemas = collect_schemas();
    let schema = schemas.iter().find(|s| s.name == canonical);

    if let Some(s) = schema {
        eprintln!("{}\n  {}\n", canonical, s.description);
        eprintln!("Parameters:");
        for field in &s.fields {
            let flag_name =
                problem_help_flag_name(canonical, &field.name, &field.type_name, is_geometry);
            // For geometry variants, show --positions instead of --graph
            if field.type_name == "G" && is_geometry {
                let hint = type_format_hint(&field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({hint})", flag_name, field.description);
                if graph_type == Some("UnitDiskGraph") {
                    eprintln!("  --{:<16} Distance threshold [default: 1.0]", "radius");
                }
            } else if field.type_name == "DirectedGraph" {
                // DirectedGraph fields use --arcs, not --graph
                let hint = type_format_hint(&field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({})", "arcs", field.description, hint);
            } else if field.type_name == "MixedGraph" {
                eprintln!(
                    "  --{:<16} Undirected edges E of the mixed graph (edge list: 0-1,1-2,2-3)",
                    "graph"
                );
                eprintln!(
                    "  --{:<16} Directed arcs A of the mixed graph (directed arcs: 0>1,1>2,2>0)",
                    "arcs"
                );
            } else if field.type_name == "BipartiteGraph" {
                eprintln!(
                    "  --{:<16} Vertices in the left partition (integer)",
                    "left"
                );
                eprintln!(
                    "  --{:<16} Vertices in the right partition (integer)",
                    "right"
                );
                eprintln!(
                    "  --{:<16} Bipartite edges as left-right pairs (edge list: 0-0,0-1,1-2)",
                    "biedges"
                );
            } else {
                let hint = help_flag_hint(canonical, &field.name, &field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({})", flag_name, field.description, hint);
            }
        }
    } else {
        bail!("{}", crate::problem_name::unknown_problem_error(canonical));
    }

    let example = example_for(canonical, graph_type);
    if !example.is_empty() {
        eprintln!("\nExample:");
        eprintln!(
            "  pred create {} {}",
            match graph_type {
                Some(g) => format!("{canonical}/{g}"),
                None => canonical.to_string(),
            },
            example
        );
    }
    Ok(())
}

fn problem_help_flag_name(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> String {
    if field_type == "G" && is_geometry {
        return "positions".to_string();
    }
    if field_type == "DirectedGraph" {
        return "arcs".to_string();
    }
    if field_type == "MixedGraph" {
        return "graph".to_string();
    }
    if canonical == "LengthBoundedDisjointPaths" && field_name == "max_length" {
        return "bound".to_string();
    }
    if canonical == "GeneralizedHex" && field_name == "target" {
        return "sink".to_string();
    }
    if canonical == "StringToStringCorrection" {
        return match field_name {
            "source" => "source-string".to_string(),
            "target" => "target-string".to_string(),
            "bound" => "bound".to_string(),
            _ => help_flag_name(canonical, field_name),
        };
    }
    help_flag_name(canonical, field_name)
}

fn lbdp_validation_error(message: &str, usage: Option<&str>) -> anyhow::Error {
    match usage {
        Some(usage) => anyhow::anyhow!("{message}\n\n{usage}"),
        None => anyhow::anyhow!("{message}"),
    }
}

fn validate_length_bounded_disjoint_paths_args(
    num_vertices: usize,
    source: usize,
    sink: usize,
    bound: i64,
    usage: Option<&str>,
) -> Result<usize> {
    let max_length = usize::try_from(bound).map_err(|_| {
        lbdp_validation_error(
            "--bound must be a nonnegative integer for LengthBoundedDisjointPaths",
            usage,
        )
    })?;
    if source >= num_vertices || sink >= num_vertices {
        return Err(lbdp_validation_error(
            "--source and --sink must be valid graph vertices",
            usage,
        ));
    }
    if source == sink {
        return Err(lbdp_validation_error(
            "--source and --sink must be distinct",
            usage,
        ));
    }
    if max_length == 0 {
        return Err(lbdp_validation_error("--bound must be positive", usage));
    }
    Ok(max_length)
}

/// Resolve the graph type from the variant map (e.g., "KingsSubgraph", "UnitDiskGraph", or "SimpleGraph").
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
    let graph_type = resolved_graph_type(&resolved_variant);

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
        let gt = if graph_type != "SimpleGraph" {
            Some(graph_type)
        } else {
            None
        };
        print_problem_help(canonical, gt)?;
        std::process::exit(2);
    }

    let (data, variant) = match canonical {
        // Graph problems with vertex weights
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet" => {
            create_vertex_weight_problem(args, canonical, graph_type, &resolved_variant)?
        }

        // SteinerTree (graph + edge weights + terminals)
        "SteinerTree" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create SteinerTree --graph 0-1,1-2,1-3,3-4 --edge-weights 2,2,1,1 --terminals 0,2,4"
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let terminals = parse_terminals(args, graph.num_vertices())?;
            let data = ser(SteinerTree::new(graph, edge_weights, terminals))?;
            (data, resolved_variant.clone())
        }

        // Generalized Hex (graph + source + sink)
        "GeneralizedHex" => {
            let usage =
                "Usage: pred create GeneralizedHex --graph 0-1,0-2,0-3,1-4,2-4,3-4,4-5 --source 0 --sink 5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let num_vertices = graph.num_vertices();
            let source = args
                .source
                .ok_or_else(|| anyhow::anyhow!("GeneralizedHex requires --source\n\n{usage}"))?;
            let sink = args
                .sink
                .ok_or_else(|| anyhow::anyhow!("GeneralizedHex requires --sink\n\n{usage}"))?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            if source == sink {
                bail!("GeneralizedHex requires distinct --source and --sink\n\n{usage}");
            }
            (
                ser(GeneralizedHex::new(graph, source, sink))?,
                resolved_variant.clone(),
            )
        }

        // DisjointConnectingPaths (graph + terminal pairs)
        "DisjointConnectingPaths" => {
            let usage =
                "Usage: pred create DisjointConnectingPaths --graph 0-1,1-3,0-2,1-4,2-4,3-5,4-5 --terminal-pairs 0-3,2-5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let terminal_pairs = parse_terminal_pairs(args, graph.num_vertices())
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            (
                ser(DisjointConnectingPaths::new(graph, terminal_pairs))?,
                resolved_variant.clone(),
            )
        }

        // IntegralFlowWithMultipliers (directed arcs + capacities + source/sink + multipliers + requirement)
        "IntegralFlowWithMultipliers" => {
            let usage = "Usage: pred create IntegralFlowWithMultipliers --arcs \"0>1,0>2,1>3,2>3\" --capacities 1,1,2,2 --source 0 --sink 3 --multipliers 1,2,3,1 --requirement 2";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --arcs\n\n{usage}")
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities_str = args.capacities.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --capacities\n\n{usage}")
            })?;
            let capacities: Vec<u64> = util::parse_comma_list(capacities_str)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            if capacities.len() != num_arcs {
                bail!(
                    "Expected {} capacities but got {}\n\n{}",
                    num_arcs,
                    capacities.len(),
                    usage
                );
            }
            for (arc_index, &capacity) in capacities.iter().enumerate() {
                let fits = usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some();
                if !fits {
                    bail!(
                        "capacity {} at arc index {} is too large for this platform\n\n{}",
                        capacity,
                        arc_index,
                        usage
                    );
                }
            }

            let num_vertices = graph.num_vertices();
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --sink\n\n{usage}")
            })?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            if source == sink {
                bail!(
                    "IntegralFlowWithMultipliers requires distinct --source and --sink\n\n{}",
                    usage
                );
            }

            let multipliers_str = args.multipliers.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --multipliers\n\n{usage}")
            })?;
            let multipliers: Vec<u64> = util::parse_comma_list(multipliers_str)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            if multipliers.len() != num_vertices {
                bail!(
                    "Expected {} multipliers but got {}\n\n{}",
                    num_vertices,
                    multipliers.len(),
                    usage
                );
            }
            if multipliers
                .iter()
                .enumerate()
                .any(|(vertex, &multiplier)| vertex != source && vertex != sink && multiplier == 0)
            {
                bail!("non-terminal multipliers must be positive\n\n{usage}");
            }

            let requirement = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --requirement\n\n{usage}")
            })?;
            (
                ser(IntegralFlowWithMultipliers::new(
                    graph,
                    source,
                    sink,
                    multipliers,
                    capacities,
                    requirement,
                ))?,
                resolved_variant.clone(),
            )
        }

        // Minimum cut into bounded sets (graph + edge weights + s/t/B/K)
        "MinimumCutIntoBoundedSets" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create MinimumCutIntoBoundedSets --graph 0-1,1-2,2-3 --edge-weights 1,1,1 --source 0 --sink 2 --size-bound 2"
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let source = args
                .source
                .context("--source is required for MinimumCutIntoBoundedSets")?;
            let sink = args
                .sink
                .context("--sink is required for MinimumCutIntoBoundedSets")?;
            let size_bound = args
                .size_bound
                .context("--size-bound is required for MinimumCutIntoBoundedSets")?;
            (
                ser(MinimumCutIntoBoundedSets::new(
                    graph,
                    edge_weights,
                    source,
                    sink,
                    size_bound,
                ))?,
                resolved_variant.clone(),
            )
        }

        // Hamiltonian Circuit (graph only, no weights)
        "HamiltonianCircuit" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create HamiltonianCircuit --graph 0-1,1-2,2-3,3-0"
                )
            })?;
            (
                ser(HamiltonianCircuit::new(graph))?,
                resolved_variant.clone(),
            )
        }

        // Biconnectivity augmentation
        "BiconnectivityAugmentation" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create BiconnectivityAugmentation --graph 0-1,1-2,2-3 --potential-edges 0-2:3,0-3:4,1-3:2 --budget 5"
                )
            })?;
            let potential_edges = parse_potential_edges(args)?;
            validate_potential_edges(&graph, &potential_edges)?;
            let budget = parse_budget(args)?;
            (
                ser(BiconnectivityAugmentation::new(
                    graph,
                    potential_edges,
                    budget,
                ))?,
                resolved_variant.clone(),
            )
        }

        // Partial Feedback Edge Set
        "PartialFeedbackEdgeSet" => {
            let usage = "Usage: pred create PartialFeedbackEdgeSet --graph 0-1,1-2,2-0,2-3,3-4,4-2,3-5,5-4,0-3 --budget 3 --max-cycle-length 4";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let budget = args
                .budget
                .as_deref()
                .ok_or_else(|| {
                    anyhow::anyhow!("PartialFeedbackEdgeSet requires --budget\n\n{usage}")
                })?
                .parse::<usize>()
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Invalid --budget value for PartialFeedbackEdgeSet: {e}\n\n{usage}"
                    )
                })?;
            let max_cycle_length = args.max_cycle_length.ok_or_else(|| {
                anyhow::anyhow!("PartialFeedbackEdgeSet requires --max-cycle-length\n\n{usage}")
            })?;
            (
                ser(PartialFeedbackEdgeSet::new(graph, budget, max_cycle_length))?,
                resolved_variant.clone(),
            )
        }

        // Bounded Component Spanning Forest
        "BoundedComponentSpanningForest" => {
            let usage = "Usage: pred create BoundedComponentSpanningForest --graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --bound 6";
            let (graph, n) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            args.weights.as_deref().ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --weights\n\n{usage}")
            })?;
            let weights = parse_vertex_weights(args, n)?;
            if weights.iter().any(|&weight| weight < 0) {
                bail!("BoundedComponentSpanningForest requires nonnegative --weights\n\n{usage}");
            }
            let max_components = args.k.ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --k\n\n{usage}")
            })?;
            if max_components == 0 {
                bail!("BoundedComponentSpanningForest requires --k >= 1\n\n{usage}");
            }
            let bound_raw = args.bound.ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --bound\n\n{usage}")
            })?;
            if bound_raw <= 0 {
                bail!("BoundedComponentSpanningForest requires positive --bound\n\n{usage}");
            }
            let max_weight = i32::try_from(bound_raw).map_err(|_| {
                anyhow::anyhow!(
                    "BoundedComponentSpanningForest requires --bound within i32 range\n\n{usage}"
                )
            })?;
            (
                ser(BoundedComponentSpanningForest::new(
                    graph,
                    weights,
                    max_components,
                    max_weight,
                ))?,
                resolved_variant.clone(),
            )
        }

        // Hamiltonian path (graph only, no weights)
        "HamiltonianPath" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!("{e}\n\nUsage: pred create HamiltonianPath --graph 0-1,1-2,2-3")
            })?;
            (ser(HamiltonianPath::new(graph))?, resolved_variant.clone())
        }

        // LongestPath
        "LongestPath" => {
            let usage = "pred create LongestPath --graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,4-6,5-6,1-6 --edge-lengths 3,2,4,1,5,2,3,2,4,1 --source-vertex 0 --target-vertex 6";
            let (graph, _) =
                parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\nUsage: {usage}"))?;
            if args.weights.is_some() {
                bail!("LongestPath uses --edge-lengths, not --weights\n\nUsage: {usage}");
            }
            let edge_lengths_raw = args.edge_lengths.as_ref().ok_or_else(|| {
                anyhow::anyhow!("LongestPath requires --edge-lengths\n\nUsage: {usage}")
            })?;
            let edge_lengths =
                parse_i32_edge_values(Some(edge_lengths_raw), graph.num_edges(), "edge length")?;
            ensure_positive_i32_values(&edge_lengths, "edge lengths")?;
            let source_vertex = args.source_vertex.ok_or_else(|| {
                anyhow::anyhow!("LongestPath requires --source-vertex\n\nUsage: {usage}")
            })?;
            let target_vertex = args.target_vertex.ok_or_else(|| {
                anyhow::anyhow!("LongestPath requires --target-vertex\n\nUsage: {usage}")
            })?;
            ensure_vertex_in_bounds(source_vertex, graph.num_vertices(), "source_vertex")?;
            ensure_vertex_in_bounds(target_vertex, graph.num_vertices(), "target_vertex")?;
            (
                ser(LongestPath::new(
                    graph,
                    edge_lengths,
                    source_vertex,
                    target_vertex,
                ))?,
                resolved_variant.clone(),
            )
        }

        // ShortestWeightConstrainedPath
        "ShortestWeightConstrainedPath" => {
            let usage = "pred create ShortestWeightConstrainedPath --graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4 --edge-lengths 2,4,3,1,5,4,2,6 --edge-weights 5,1,2,3,2,3,1,1 --source-vertex 0 --target-vertex 5 --weight-bound 8";
            let (graph, _) =
                parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\nUsage: {usage}"))?;
            if args.weights.is_some() {
                bail!(
                    "ShortestWeightConstrainedPath uses --edge-weights, not --weights\n\nUsage: {usage}"
                );
            }
            let edge_lengths_raw = args.edge_lengths.as_ref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --edge-lengths\n\nUsage: {usage}"
                )
            })?;
            let edge_weights_raw = args.edge_weights.as_ref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --edge-weights\n\nUsage: {usage}"
                )
            })?;
            let edge_lengths =
                parse_i32_edge_values(Some(edge_lengths_raw), graph.num_edges(), "edge length")?;
            let edge_weights =
                parse_i32_edge_values(Some(edge_weights_raw), graph.num_edges(), "edge weight")?;
            ensure_positive_i32_values(&edge_lengths, "edge lengths")?;
            ensure_positive_i32_values(&edge_weights, "edge weights")?;
            let source_vertex = args.source_vertex.ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --source-vertex\n\nUsage: {usage}"
                )
            })?;
            let target_vertex = args.target_vertex.ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --target-vertex\n\nUsage: {usage}"
                )
            })?;
            let weight_bound = args.weight_bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ShortestWeightConstrainedPath requires --weight-bound\n\nUsage: {usage}"
                )
            })?;
            ensure_vertex_in_bounds(source_vertex, graph.num_vertices(), "source_vertex")?;
            ensure_vertex_in_bounds(target_vertex, graph.num_vertices(), "target_vertex")?;
            ensure_positive_i32(weight_bound, "weight_bound")?;
            (
                ser(ShortestWeightConstrainedPath::new(
                    graph,
                    edge_lengths,
                    edge_weights,
                    source_vertex,
                    target_vertex,
                    weight_bound,
                ))?,
                resolved_variant.clone(),
            )
        }

        // MultipleCopyFileAllocation (graph + usage + storage)
        "MultipleCopyFileAllocation" => {
            let (graph, num_vertices) = parse_graph(args)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{MULTIPLE_COPY_FILE_ALLOCATION_USAGE}"))?;
            let usage = parse_vertex_i64_values(
                args.usage.as_deref(),
                "usage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?;
            let storage = parse_vertex_i64_values(
                args.storage.as_deref(),
                "storage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?;
            (
                ser(MultipleCopyFileAllocation::new(graph, usage, storage))?,
                resolved_variant.clone(),
            )
        }

        // ExpectedRetrievalCost (probabilities + sectors)
        "ExpectedRetrievalCost" => {
            let probabilities_str = args.probabilities.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ExpectedRetrievalCost requires --probabilities\n\n{EXPECTED_RETRIEVAL_COST_USAGE}"
                )
            })?;
            let probabilities: Vec<f64> = util::parse_comma_list(probabilities_str)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{EXPECTED_RETRIEVAL_COST_USAGE}"))?;
            anyhow::ensure!(
                !probabilities.is_empty(),
                "ExpectedRetrievalCost requires at least one probability\n\n{EXPECTED_RETRIEVAL_COST_USAGE}"
            );
            anyhow::ensure!(
                probabilities.iter().all(|p| p.is_finite() && (0.0..=1.0).contains(p)),
                "ExpectedRetrievalCost probabilities must be finite values in [0, 1]\n\n{EXPECTED_RETRIEVAL_COST_USAGE}"
            );
            let total_probability: f64 = probabilities.iter().sum();
            anyhow::ensure!(
                (total_probability - 1.0).abs() <= 1e-9,
                "ExpectedRetrievalCost probabilities must sum to 1.0\n\n{EXPECTED_RETRIEVAL_COST_USAGE}"
            );

            let num_sectors = args.num_sectors.ok_or_else(|| {
                anyhow::anyhow!(
                    "ExpectedRetrievalCost requires --num-sectors\n\n{EXPECTED_RETRIEVAL_COST_USAGE}"
                )
            })?;
            anyhow::ensure!(
                num_sectors >= 2,
                "ExpectedRetrievalCost requires at least two sectors\n\n{EXPECTED_RETRIEVAL_COST_USAGE}"
            );

            (
                ser(ExpectedRetrievalCost::new(probabilities, num_sectors))?,
                resolved_variant.clone(),
            )
        }

        // UndirectedFlowLowerBounds (graph + capacities + lower bounds + terminals + requirement)
        "UndirectedFlowLowerBounds" => {
            let usage = "Usage: pred create UndirectedFlowLowerBounds --graph 0-1,0-2,1-3,2-3,1-4,3-5,4-5 --capacities 2,2,2,2,1,3,2 --lower-bounds 1,1,0,0,1,0,1 --source 0 --sink 5 --requirement 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities = parse_capacities(args, graph.num_edges(), usage)?;
            let lower_bounds = parse_lower_bounds(args, graph.num_edges(), usage)?;
            let num_vertices = graph.num_vertices();
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("UndirectedFlowLowerBounds requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("UndirectedFlowLowerBounds requires --sink\n\n{usage}")
            })?;
            let requirement = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("UndirectedFlowLowerBounds requires --requirement\n\n{usage}")
            })?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            (
                ser(UndirectedFlowLowerBounds::new(
                    graph,
                    capacities,
                    lower_bounds,
                    source,
                    sink,
                    requirement,
                ))?,
                resolved_variant.clone(),
            )
        }

        // UndirectedTwoCommodityIntegralFlow (graph + capacities + terminals + requirements)
        "UndirectedTwoCommodityIntegralFlow" => {
            let usage = "Usage: pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities = parse_capacities(args, graph.num_edges(), usage)?;
            for (edge_index, &capacity) in capacities.iter().enumerate() {
                let fits = usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some();
                if !fits {
                    bail!(
                        "capacity {} at edge index {} is too large for this platform\n\n{}",
                        capacity,
                        edge_index,
                        usage
                    );
                }
            }
            let num_vertices = graph.num_vertices();
            let source_1 = args.source_1.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --source-1\n\n{usage}")
            })?;
            let sink_1 = args.sink_1.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --sink-1\n\n{usage}")
            })?;
            let source_2 = args.source_2.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --source-2\n\n{usage}")
            })?;
            let sink_2 = args.sink_2.ok_or_else(|| {
                anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --sink-2\n\n{usage}")
            })?;
            let requirement_1 = args.requirement_1.ok_or_else(|| {
                anyhow::anyhow!(
                    "UndirectedTwoCommodityIntegralFlow requires --requirement-1\n\n{usage}"
                )
            })?;
            let requirement_2 = args.requirement_2.ok_or_else(|| {
                anyhow::anyhow!(
                    "UndirectedTwoCommodityIntegralFlow requires --requirement-2\n\n{usage}"
                )
            })?;
            for (label, vertex) in [
                ("source-1", source_1),
                ("sink-1", sink_1),
                ("source-2", source_2),
                ("sink-2", sink_2),
            ] {
                validate_vertex_index(label, vertex, num_vertices, usage)?;
            }
            (
                ser(UndirectedTwoCommodityIntegralFlow::new(
                    graph,
                    capacities,
                    source_1,
                    sink_1,
                    source_2,
                    sink_2,
                    requirement_1,
                    requirement_2,
                ))?,
                resolved_variant.clone(),
            )
        }

        // IntegralFlowBundles (directed graph + bundles + source/sink + requirement)
        "IntegralFlowBundles" => {
            let usage = "Usage: pred create IntegralFlowBundles --arcs \"0>1,0>2,1>3,2>3,1>2,2>1\" --bundles \"0,1;2,5;3,4\" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --arcs\n\n{usage}"))?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let bundles = parse_bundles(args, num_arcs, usage)?;
            let bundle_capacities = parse_bundle_capacities(args, bundles.len(), usage)?;
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowBundles requires --source\n\n{usage}")
            })?;
            let sink = args
                .sink
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --sink\n\n{usage}"))?;
            let requirement = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowBundles requires --requirement\n\n{usage}")
            })?;
            validate_vertex_index("source", source, graph.num_vertices(), usage)?;
            validate_vertex_index("sink", sink, graph.num_vertices(), usage)?;
            anyhow::ensure!(
                source != sink,
                "IntegralFlowBundles requires distinct --source and --sink\n\n{usage}"
            );

            (
                ser(IntegralFlowBundles::new(
                    graph,
                    source,
                    sink,
                    bundles,
                    bundle_capacities,
                    requirement,
                ))?,
                resolved_variant.clone(),
            )
        }

        // LengthBoundedDisjointPaths (graph + source + sink + bound)
        "LengthBoundedDisjointPaths" => {
            let usage = "Usage: pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --bound 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --sink\n\n{usage}")
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --bound\n\n{usage}")
            })?;
            let max_length = validate_length_bounded_disjoint_paths_args(
                graph.num_vertices(),
                source,
                sink,
                bound,
                Some(usage),
            )?;

            (
                ser(LengthBoundedDisjointPaths::new(
                    graph, source, sink, max_length,
                ))?,
                resolved_variant.clone(),
            )
        }

        // IsomorphicSpanningTree (graph + tree)
        "IsomorphicSpanningTree" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create IsomorphicSpanningTree --graph 0-1,1-2,0-2 --tree 0-1,1-2"
                )
            })?;
            let tree_str = args.tree.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "IsomorphicSpanningTree requires --tree\n\n\
                     Usage: pred create IsomorphicSpanningTree --graph 0-1,1-2,0-2 --tree 0-1,1-2"
                )
            })?;
            let tree_edges: Vec<(usize, usize)> = tree_str
                .split(',')
                .map(|pair| {
                    let parts: Vec<&str> = pair.trim().split('-').collect();
                    if parts.len() != 2 {
                        bail!("Invalid tree edge '{}': expected format u-v", pair.trim());
                    }
                    let u: usize = parts[0].parse()?;
                    let v: usize = parts[1].parse()?;
                    Ok((u, v))
                })
                .collect::<Result<Vec<_>>>()?;
            let tree_num_vertices = tree_edges
                .iter()
                .flat_map(|(u, v)| [*u, *v])
                .max()
                .map(|m| m + 1)
                .unwrap_or(0)
                .max(graph.num_vertices());
            let tree = SimpleGraph::new(tree_num_vertices, tree_edges);
            (
                ser(problemreductions::models::graph::IsomorphicSpanningTree::new(graph, tree))?,
                resolved_variant.clone(),
            )
        }

        // KthBestSpanningTree (weighted graph + k + bound)
        "KthBestSpanningTree" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create KthBestSpanningTree --graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3"
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let (k, _variant) =
                util::validate_k_param(&resolved_variant, args.k, None, "KthBestSpanningTree")?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "KthBestSpanningTree requires --bound\n\n\
                     Usage: pred create KthBestSpanningTree --graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3"
                )
            })? as i32;
            (
                ser(problemreductions::models::graph::KthBestSpanningTree::new(
                    graph,
                    edge_weights,
                    k,
                    bound,
                ))?,
                resolved_variant.clone(),
            )
        }

        // Graph problems with edge weights
        "BottleneckTravelingSalesman" | "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create {} --graph 0-1,1-2,2-3 [--edge-weights 1,1,1]",
                    problem
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let data = match canonical {
                "BottleneckTravelingSalesman" => {
                    ser(BottleneckTravelingSalesman::new(graph, edge_weights))?
                }
                "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                _ => unreachable!(),
            };
            (data, resolved_variant.clone())
        }

        // SteinerTreeInGraphs (graph + edge weights + terminals)
        "SteinerTreeInGraphs" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create SteinerTreeInGraphs --graph 0-1,1-2,2-3 --terminals 0,3 [--edge-weights 1,1,1]"
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let terminals = parse_terminals(args, graph.num_vertices())?;
            (
                ser(SteinerTreeInGraphs::new(graph, terminals, edge_weights))?,
                resolved_variant.clone(),
            )
        }

        // RuralPostman
        "RuralPostman" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create RuralPostman --graph 0-1,1-2,2-3 --edge-weights 1,1,1 --required-edges 0,2"
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let required_edges_str = args.required_edges.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "RuralPostman requires --required-edges\n\n\
                     Usage: pred create RuralPostman --graph 0-1,1-2,2-3 --edge-weights 1,1,1 --required-edges 0,2"
                )
            })?;
            let required_edges: Vec<usize> = util::parse_comma_list(required_edges_str)?;
            (
                ser(RuralPostman::new(graph, edge_weights, required_edges))?,
                resolved_variant.clone(),
            )
        }

        // LongestCircuit
        "LongestCircuit" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let usage = "pred create LongestCircuit --graph 0-1,1-2,2-3,3-4,4-5,5-0,0-3,1-4,2-5,3-5 --edge-weights 3,2,4,1,5,2,3,2,1,2";
            let (graph, _) =
                parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\nUsage: {usage}"))?;
            let edge_lengths = parse_edge_weights(args, graph.num_edges())?;
            if edge_lengths.iter().any(|&length| length <= 0) {
                bail!("LongestCircuit --edge-weights must be positive (> 0)");
            }
            (
                ser(LongestCircuit::new(graph, edge_lengths))?,
                resolved_variant.clone(),
            )
        }

        // StackerCrane
        "StackerCrane" => {
            let usage = "Usage: pred create StackerCrane --arcs \"0>4,2>5,5>1,3>0,4>3\" --graph \"0-1,1-2,2-3,3-5,4-5,0-3,1-5\" --arc-costs 3,4,2,5,3 --edge-lengths 2,1,3,2,1,4,3 --num-vertices 6";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("StackerCrane requires --arcs\n\n{usage}"))?;
            let (arcs_graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let (edges_graph, num_vertices) =
                parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            anyhow::ensure!(
                edges_graph.num_vertices() == num_vertices,
                "internal error: inconsistent graph vertex count"
            );
            anyhow::ensure!(
                num_vertices == arcs_graph.num_vertices(),
                "StackerCrane requires the directed and undirected inputs to agree on --num-vertices\n\n{usage}"
            );
            let arc_lengths = parse_arc_costs(args, num_arcs)?;
            let edge_lengths = parse_i32_edge_values(
                args.edge_lengths.as_ref(),
                edges_graph.num_edges(),
                "edge length",
            )?;
            (
                ser(StackerCrane::try_new(
                    num_vertices,
                    arcs_graph.arcs(),
                    edges_graph.edges(),
                    arc_lengths,
                    edge_lengths,
                )
                .map_err(|e| anyhow::anyhow!(e))?)?,
                resolved_variant.clone(),
            )
        }

        // MultipleChoiceBranching
        "MultipleChoiceBranching" => {
            let usage = "Usage: pred create MultipleChoiceBranching/i32 --arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --bound 10";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("MultipleChoiceBranching requires --arcs\n\n{usage}")
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let weights = parse_arc_weights(args, num_arcs)?;
            let partition = parse_partition_groups(args, num_arcs)?;
            let threshold = parse_multiple_choice_branching_threshold(args, usage)?;
            (
                ser(MultipleChoiceBranching::new(
                    graph, weights, partition, threshold,
                ))?,
                resolved_variant.clone(),
            )
        }

        // KColoring
        "KColoring" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!("{e}\n\nUsage: pred create KColoring --graph 0-1,1-2,2-0 --k 3")
            })?;
            let (k, _variant) =
                util::validate_k_param(&resolved_variant, args.k, None, "KColoring")?;
            util::ser_kcoloring(graph, k)?
        }

        "KClique" => {
            let usage = "Usage: pred create KClique --graph 0-1,0-2,1-3,2-3,2-4,3-4 --k 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let k = parse_kclique_threshold(args.k, graph.num_vertices(), usage)?;
            (ser(KClique::new(graph, k))?, resolved_variant.clone())
        }

        // SAT
        "Satisfiability" => {
            let num_vars = args.num_vars.ok_or_else(|| {
                anyhow::anyhow!(
                    "Satisfiability requires --num-vars\n\n\
                     Usage: pred create SAT --num-vars 3 --clauses \"1,2;-1,3\""
                )
            })?;
            let clauses = parse_clauses(args)?;
            (
                ser(Satisfiability::new(num_vars, clauses))?,
                resolved_variant.clone(),
            )
        }
        "NAESatisfiability" => {
            let num_vars = args.num_vars.ok_or_else(|| {
                anyhow::anyhow!(
                    "NAESatisfiability requires --num-vars\n\n\
                     Usage: pred create NAESAT --num-vars 3 --clauses \"1,2,-3;-1,2,3\""
                )
            })?;
            let clauses = parse_clauses(args)?;
            (
                ser(NAESatisfiability::try_new(num_vars, clauses).map_err(anyhow::Error::msg)?)?,
                resolved_variant.clone(),
            )
        }
        "KSatisfiability" => {
            let num_vars = args.num_vars.ok_or_else(|| {
                anyhow::anyhow!(
                    "KSatisfiability requires --num-vars\n\n\
                     Usage: pred create KSAT --num-vars 3 --clauses \"1,2,3;-1,2,-3\""
                )
            })?;
            let clauses = parse_clauses(args)?;
            let (k, _variant) =
                util::validate_k_param(&resolved_variant, args.k, Some(3), "KSatisfiability")?;
            util::ser_ksat(num_vars, clauses, k)?
        }

        // QBF
        "QuantifiedBooleanFormulas" => {
            let num_vars = args.num_vars.ok_or_else(|| {
                anyhow::anyhow!(
                    "QuantifiedBooleanFormulas requires --num-vars, --clauses, and --quantifiers\n\n\
                     Usage: pred create QBF --num-vars 3 --clauses \"1,2;-1,3\" --quantifiers \"E,A,E\""
                )
            })?;
            let clauses = parse_clauses(args)?;
            let quantifiers = parse_quantifiers(args, num_vars)?;
            (
                ser(QuantifiedBooleanFormulas::new(
                    num_vars,
                    quantifiers,
                    clauses,
                ))?,
                resolved_variant.clone(),
            )
        }

        // QuadraticAssignment
        "QuadraticAssignment" => {
            let cost_str = args.matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "QuadraticAssignment requires --matrix (cost) and --distance-matrix\n\n\
                     Usage: pred create QAP --matrix \"0,5;5,0\" --distance-matrix \"0,1;1,0\""
                )
            })?;
            let dist_str = args.distance_matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "QuadraticAssignment requires --distance-matrix\n\n\
                     Usage: pred create QAP --matrix \"0,5;5,0\" --distance-matrix \"0,1;1,0\""
                )
            })?;
            let cost_matrix = parse_i64_matrix(cost_str).context("Invalid cost matrix")?;
            let distance_matrix = parse_i64_matrix(dist_str).context("Invalid distance matrix")?;
            let n = cost_matrix.len();
            for (i, row) in cost_matrix.iter().enumerate() {
                if row.len() != n {
                    bail!(
                        "cost matrix must be square: row {i} has {} columns, expected {n}",
                        row.len()
                    );
                }
            }
            let m = distance_matrix.len();
            for (i, row) in distance_matrix.iter().enumerate() {
                if row.len() != m {
                    bail!(
                        "distance matrix must be square: row {i} has {} columns, expected {m}",
                        row.len()
                    );
                }
            }
            if n > m {
                bail!("num_facilities ({n}) must be <= num_locations ({m})");
            }
            (
                ser(
                    problemreductions::models::algebraic::QuadraticAssignment::new(
                        cost_matrix,
                        distance_matrix,
                    ),
                )?,
                resolved_variant.clone(),
            )
        }

        // QUBO
        "QUBO" => {
            let matrix = parse_matrix(args)?;
            (ser(QUBO::from_matrix(matrix))?, resolved_variant.clone())
        }

        // SpinGlass
        "SpinGlass" => {
            let (graph, n) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create SpinGlass --graph 0-1,1-2 [--couplings 1,1] [--fields 0,0,0]"
                )
            })?;
            let use_f64 = resolved_variant.get("weight").is_some_and(|w| w == "f64")
                || has_float_syntax(&args.couplings)
                || has_float_syntax(&args.fields);
            if use_f64 {
                let couplings = parse_couplings_f64(args, graph.num_edges())?;
                let fields = parse_fields_f64(args, n)?;
                let mut variant = resolved_variant.clone();
                variant.insert("weight".to_string(), "f64".to_string());
                (
                    ser(SpinGlass::from_graph(graph, couplings, fields))?,
                    variant,
                )
            } else {
                let couplings = parse_couplings(args, graph.num_edges())?;
                let fields = parse_fields(args, n)?;
                (
                    ser(SpinGlass::from_graph(graph, couplings, fields))?,
                    resolved_variant.clone(),
                )
            }
        }

        // Factoring
        "Factoring" => {
            let usage = "Usage: pred create Factoring --target 15 --m 4 --n 4";
            let target = args
                .target
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("Factoring requires --target\n\n{usage}"))?;
            let target: u64 = target
                .parse()
                .context("Factoring --target must fit in u64")?;
            let m = args
                .m
                .ok_or_else(|| anyhow::anyhow!("Factoring requires --m\n\n{usage}"))?;
            let n = args
                .n
                .ok_or_else(|| anyhow::anyhow!("Factoring requires --n\n\n{usage}"))?;
            (ser(Factoring::new(m, n, target))?, resolved_variant.clone())
        }

        // MaximalIS — same as MIS (graph + vertex weights)
        "MaximalIS" => {
            create_vertex_weight_problem(args, canonical, graph_type, &resolved_variant)?
        }

        // BoyceCoddNormalFormViolation
        "BoyceCoddNormalFormViolation" => {
            let n = args.n.ok_or_else(|| {
                anyhow::anyhow!(
                    "BoyceCoddNormalFormViolation requires --n, --sets, and --target\n\n\
                     Usage: pred create BoyceCoddNormalFormViolation --n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
                )
            })?;
            let sets_str = args.sets.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "BoyceCoddNormalFormViolation requires --sets (functional deps as lhs:rhs;...)\n\n\
                     Usage: pred create BoyceCoddNormalFormViolation --n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
                )
            })?;
            let target_str = args.target.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "BoyceCoddNormalFormViolation requires --target (comma-separated attribute indices)\n\n\
                     Usage: pred create BoyceCoddNormalFormViolation --n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
                )
            })?;
            let fds: Vec<(Vec<usize>, Vec<usize>)> = sets_str
                .split(';')
                .map(|fd_str| {
                    let parts: Vec<&str> = fd_str.split(':').collect();
                    anyhow::ensure!(
                        parts.len() == 2,
                        "Each FD must be lhs:rhs, got '{}'",
                        fd_str
                    );
                    let lhs: Vec<usize> = util::parse_comma_list(parts[0])?;
                    let rhs: Vec<usize> = util::parse_comma_list(parts[1])?;
                    ensure_attribute_indices_in_range(
                        &lhs,
                        n,
                        &format!("Functional dependency '{fd_str}' lhs"),
                    )?;
                    ensure_attribute_indices_in_range(
                        &rhs,
                        n,
                        &format!("Functional dependency '{fd_str}' rhs"),
                    )?;
                    Ok((lhs, rhs))
                })
                .collect::<Result<_>>()?;
            let target: Vec<usize> = util::parse_comma_list(target_str)?;
            ensure_attribute_indices_in_range(&target, n, "Target subset")?;
            (
                ser(BoyceCoddNormalFormViolation::new(n, fds, target))?,
                resolved_variant.clone(),
            )
        }

        // BinPacking
        "BinPacking" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "BinPacking requires --sizes and --capacity\n\n\
                     Usage: pred create BinPacking --sizes 3,3,2,2 --capacity 5"
                )
            })?;
            let cap_str = args.capacity.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "BinPacking requires --capacity\n\n\
                     Usage: pred create BinPacking --sizes 3,3,2,2 --capacity 5"
                )
            })?;
            let use_f64 = sizes_str.contains('.') || cap_str.contains('.');
            if use_f64 {
                let sizes: Vec<f64> = util::parse_comma_list(sizes_str)?;
                let capacity: f64 = cap_str.parse()?;
                let mut variant = resolved_variant.clone();
                variant.insert("weight".to_string(), "f64".to_string());
                (ser(BinPacking::new(sizes, capacity))?, variant)
            } else {
                let sizes: Vec<i32> = util::parse_comma_list(sizes_str)?;
                let capacity: i32 = cap_str.parse()?;
                (
                    ser(BinPacking::new(sizes, capacity))?,
                    resolved_variant.clone(),
                )
            }
        }

        // AdditionalKey
        "AdditionalKey" => {
            let usage = "Usage: pred create AdditionalKey --num-attributes 6 --dependencies \"0,1:2,3;2,3:4,5\" --relation-attrs \"0,1,2,3,4,5\" --known-keys \"0,1;2,3\"";
            let num_attributes = args.num_attributes.ok_or_else(|| {
                anyhow::anyhow!("AdditionalKey requires --num-attributes\n\n{usage}")
            })?;
            let deps_str = args.dependencies.as_deref().ok_or_else(|| {
                anyhow::anyhow!("AdditionalKey requires --dependencies\n\n{usage}")
            })?;
            let ra_str = args.relation_attrs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("AdditionalKey requires --relation-attrs\n\n{usage}")
            })?;
            let dependencies: Vec<(Vec<usize>, Vec<usize>)> = deps_str
                .split(';')
                .map(|dep| {
                    let parts: Vec<&str> = dep.trim().split(':').collect();
                    anyhow::ensure!(
                        parts.len() == 2,
                        "Invalid dependency format '{}', expected 'lhs:rhs' (e.g., '0,1:2,3')",
                        dep.trim()
                    );
                    let lhs: Vec<usize> = util::parse_comma_list(parts[0].trim())?;
                    let rhs: Vec<usize> = util::parse_comma_list(parts[1].trim())?;
                    Ok((lhs, rhs))
                })
                .collect::<Result<Vec<_>>>()?;
            let relation_attrs: Vec<usize> = util::parse_comma_list(ra_str)?;
            let known_keys: Vec<Vec<usize>> = match args.known_keys.as_deref() {
                Some(s) if !s.is_empty() => s
                    .split(';')
                    .map(|k| util::parse_comma_list(k.trim()))
                    .collect::<Result<Vec<_>>>()?,
                _ => vec![],
            };
            (
                ser(AdditionalKey::new(
                    num_attributes,
                    dependencies,
                    relation_attrs,
                    known_keys,
                ))?,
                resolved_variant.clone(),
            )
        }

        "ConsistencyOfDatabaseFrequencyTables" => {
            let usage = "Usage: pred create ConsistencyOfDatabaseFrequencyTables --num-objects 6 --attribute-domains \"2,3,2\" --frequency-tables \"0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1\" --known-values \"0,0,0;3,0,1;1,2,1\"";
            let num_objects = args.num_objects.ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsistencyOfDatabaseFrequencyTables requires --num-objects\n\n{usage}"
                )
            })?;
            let attribute_domains_str = args.attribute_domains.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsistencyOfDatabaseFrequencyTables requires --attribute-domains\n\n{usage}"
                )
            })?;
            let frequency_tables_str = args.frequency_tables.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsistencyOfDatabaseFrequencyTables requires --frequency-tables\n\n{usage}"
                )
            })?;

            let attribute_domains: Vec<usize> = util::parse_comma_list(attribute_domains_str)?;
            for (index, &domain_size) in attribute_domains.iter().enumerate() {
                anyhow::ensure!(
                    domain_size > 0,
                    "attribute domain at index {index} must be positive\n\n{usage}"
                );
            }
            let frequency_tables =
                parse_cdft_frequency_tables(frequency_tables_str, &attribute_domains, num_objects)
                    .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let known_values = parse_cdft_known_values(
                args.known_values.as_deref(),
                num_objects,
                &attribute_domains,
            )
            .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;

            (
                ser(ConsistencyOfDatabaseFrequencyTables::new(
                    num_objects,
                    attribute_domains,
                    frequency_tables,
                    known_values,
                ))?,
                resolved_variant.clone(),
            )
        }

        // IntegerKnapsack
        "IntegerKnapsack" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "IntegerKnapsack requires --sizes, --values, and --capacity\n\n\
                     Usage: pred create IntegerKnapsack --sizes 3,4,5,2,7 --values 4,5,7,3,9 --capacity 15"
                )
            })?;
            let values_str = args.values.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegerKnapsack requires --values (e.g., 4,5,7,3,9)")
            })?;
            let cap_str = args
                .capacity
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("IntegerKnapsack requires --capacity (e.g., 15)"))?;
            let sizes: Vec<i64> = util::parse_comma_list(sizes_str)?;
            let values: Vec<i64> = util::parse_comma_list(values_str)?;
            let capacity: i64 = cap_str.parse()?;
            anyhow::ensure!(
                sizes.len() == values.len(),
                "sizes and values must have the same length, got {} and {}",
                sizes.len(),
                values.len()
            );
            anyhow::ensure!(sizes.iter().all(|&s| s > 0), "all sizes must be positive");
            anyhow::ensure!(values.iter().all(|&v| v > 0), "all values must be positive");
            anyhow::ensure!(capacity >= 0, "capacity must be nonnegative");
            (
                ser(problemreductions::models::set::IntegerKnapsack::new(
                    sizes, values, capacity,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SubsetSum
        "SubsetSum" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SubsetSum requires --sizes and --target\n\n\
                     Usage: pred create SubsetSum --sizes 3,7,1,8,2,4 --target 11"
                )
            })?;
            let target = args.target.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SubsetSum requires --target\n\n\
                     Usage: pred create SubsetSum --sizes 3,7,1,8,2,4 --target 11"
                )
            })?;
            let sizes = util::parse_biguint_list(sizes_str)?;
            let target = util::parse_decimal_biguint(target)?;
            (
                ser(SubsetSum::new(sizes, target))?,
                resolved_variant.clone(),
            )
        }

        // ThreePartition
        "ThreePartition" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ThreePartition requires --sizes and --bound\n\n\
                     Usage: pred create ThreePartition --sizes 4,5,6,4,6,5 --bound 15"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ThreePartition requires --bound\n\n\
                     Usage: pred create ThreePartition --sizes 4,5,6,4,6,5 --bound 15"
                )
            })?;
            let bound = u64::try_from(bound).map_err(|_| {
                anyhow::anyhow!(
                    "ThreePartition requires a positive integer --bound\n\n\
                     Usage: pred create ThreePartition --sizes 4,5,6,4,6,5 --bound 15"
                )
            })?;
            let sizes: Vec<u64> = util::parse_comma_list(sizes_str)?;
            (
                ser(ThreePartition::try_new(sizes, bound).map_err(anyhow::Error::msg)?)?,
                resolved_variant.clone(),
            )
        }

        // QuadraticDiophantineEquations
        "QuadraticDiophantineEquations" => {
            let a = args.coeff_a.ok_or_else(|| {
                anyhow::anyhow!(
                    "QuadraticDiophantineEquations requires --coeff-a, --coeff-b, and --qde-rhs\n\n\
                     Usage: pred create QuadraticDiophantineEquations --coeff-a 3 --coeff-b 5 --qde-rhs 53"
                )
            })?;
            let b = args.coeff_b.ok_or_else(|| {
                anyhow::anyhow!(
                    "QuadraticDiophantineEquations requires --coeff-b\n\n\
                     Usage: pred create QuadraticDiophantineEquations --coeff-a 3 --coeff-b 5 --qde-rhs 53"
                )
            })?;
            let c = args.qde_rhs.ok_or_else(|| {
                anyhow::anyhow!(
                    "QuadraticDiophantineEquations requires --qde-rhs\n\n\
                     Usage: pred create QuadraticDiophantineEquations --coeff-a 3 --coeff-b 5 --qde-rhs 53"
                )
            })?;
            (
                ser(QuadraticDiophantineEquations::try_new(a, b, c).map_err(anyhow::Error::msg)?)?,
                resolved_variant.clone(),
            )
        }

        // SumOfSquaresPartition
        "SumOfSquaresPartition" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SumOfSquaresPartition requires --sizes and --num-groups\n\n\
                     Usage: pred create SumOfSquaresPartition --sizes 5,3,8,2,7,1 --num-groups 3"
                )
            })?;
            let num_groups = args.num_groups.ok_or_else(|| {
                anyhow::anyhow!(
                    "SumOfSquaresPartition requires --num-groups\n\n\
                     Usage: pred create SumOfSquaresPartition --sizes 5,3,8,2,7,1 --num-groups 3"
                )
            })?;
            let sizes: Vec<i64> = util::parse_comma_list(sizes_str)?;
            (
                ser(SumOfSquaresPartition::try_new(sizes, num_groups)
                    .map_err(anyhow::Error::msg)?)?,
                resolved_variant.clone(),
            )
        }

        // PaintShop
        "PaintShop" => {
            let seq_str = args.sequence.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "PaintShop requires --sequence\n\n\
                     Usage: pred create PaintShop --sequence a,b,a,c,c,b"
                )
            })?;
            let sequence: Vec<String> = seq_str.split(',').map(|s| s.trim().to_string()).collect();
            (ser(PaintShop::new(sequence))?, resolved_variant.clone())
        }

        // MaximumSetPacking
        "MaximumSetPacking" => {
            let sets = parse_sets(args)?;
            let num_sets = sets.len();
            let weights = parse_set_weights(args, num_sets)?;
            (
                ser(MaximumSetPacking::with_weights(sets, weights))?,
                resolved_variant.clone(),
            )
        }

        // MinimumHittingSet
        "MinimumHittingSet" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumHittingSet requires --universe and --sets\n\n\
                     Usage: pred create MinimumHittingSet --universe 6 --sets \"0,1,2;0,3,4;1,3,5;2,4,5;0,1,5;2,3;1,4\""
                )
            })?;
            let sets = parse_sets(args)?;
            for (i, set) in sets.iter().enumerate() {
                for &element in set {
                    if element >= universe {
                        bail!(
                            "Set {} contains element {} which is outside universe of size {}",
                            i,
                            element,
                            universe
                        );
                    }
                }
            }
            (
                ser(MinimumHittingSet::new(universe, sets))?,
                resolved_variant.clone(),
            )
        }

        // MinimumSetCovering
        "MinimumSetCovering" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumSetCovering requires --universe and --sets\n\n\
                     Usage: pred create MinimumSetCovering --universe 4 --sets \"0,1;1,2;2,3;0,3\""
                )
            })?;
            let sets = parse_sets(args)?;
            let num_sets = sets.len();
            let weights = parse_set_weights(args, num_sets)?;
            (
                ser(MinimumSetCovering::with_weights(universe, sets, weights))?,
                resolved_variant.clone(),
            )
        }

        // EnsembleComputation
        "EnsembleComputation" => {
            let usage =
                "Usage: pred create EnsembleComputation --universe 4 --sets \"0,1,2;0,1,3\" --budget 4";
            let universe_size = args.universe.ok_or_else(|| {
                anyhow::anyhow!("EnsembleComputation requires --universe\n\n{usage}")
            })?;
            let subsets = parse_sets(args)?;
            let budget = args
                .budget
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("EnsembleComputation requires --budget\n\n{usage}"))?
                .parse::<usize>()
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Invalid --budget value for EnsembleComputation: {e}\n\n{usage}"
                    )
                })?;
            (
                ser(EnsembleComputation::try_new(universe_size, subsets, budget)
                    .map_err(anyhow::Error::msg)?)?,
                resolved_variant.clone(),
            )
        }

        // ComparativeContainment
        "ComparativeContainment" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "ComparativeContainment requires --universe, --r-sets, and --s-sets\n\n\
                     Usage: pred create ComparativeContainment --universe 4 --r-sets \"0,1,2,3;0,1\" --s-sets \"0,1,2,3;2,3\" [--r-weights 2,5] [--s-weights 3,6]"
                )
            })?;
            let r_sets = parse_named_sets(args.r_sets.as_deref(), "--r-sets")?;
            let s_sets = parse_named_sets(args.s_sets.as_deref(), "--s-sets")?;
            validate_comparative_containment_sets("R", "--r-sets", universe, &r_sets)?;
            validate_comparative_containment_sets("S", "--s-sets", universe, &s_sets)?;
            let data = match resolved_variant.get("weight").map(|value| value.as_str()) {
                Some("One") => {
                    let r_weights = parse_named_set_weights(
                        args.r_weights.as_deref(),
                        r_sets.len(),
                        "--r-weights",
                    )?;
                    let s_weights = parse_named_set_weights(
                        args.s_weights.as_deref(),
                        s_sets.len(),
                        "--s-weights",
                    )?;
                    if r_weights.iter().any(|&w| w != 1) || s_weights.iter().any(|&w| w != 1) {
                        bail!(
                            "Non-unit weights are not supported for ComparativeContainment/One.\n\n\
                             Use `pred create ComparativeContainment/i32 ... --r-weights ... --s-weights ...` for weighted instances."
                        );
                    }
                    ser(ComparativeContainment::<One>::new(universe, r_sets, s_sets))?
                }
                Some("f64") => {
                    let r_weights = parse_named_set_weights_f64(
                        args.r_weights.as_deref(),
                        r_sets.len(),
                        "--r-weights",
                    )?;
                    validate_comparative_containment_f64_weights("R", "--r-weights", &r_weights)?;
                    let s_weights = parse_named_set_weights_f64(
                        args.s_weights.as_deref(),
                        s_sets.len(),
                        "--s-weights",
                    )?;
                    validate_comparative_containment_f64_weights("S", "--s-weights", &s_weights)?;
                    ser(ComparativeContainment::<f64>::with_weights(
                        universe, r_sets, s_sets, r_weights, s_weights,
                    ))?
                }
                Some("i32") | None => {
                    let r_weights = parse_named_set_weights(
                        args.r_weights.as_deref(),
                        r_sets.len(),
                        "--r-weights",
                    )?;
                    validate_comparative_containment_i32_weights("R", "--r-weights", &r_weights)?;
                    let s_weights = parse_named_set_weights(
                        args.s_weights.as_deref(),
                        s_sets.len(),
                        "--s-weights",
                    )?;
                    validate_comparative_containment_i32_weights("S", "--s-weights", &s_weights)?;
                    ser(ComparativeContainment::with_weights(
                        universe, r_sets, s_sets, r_weights, s_weights,
                    ))?
                }
                Some(other) => bail!(
                    "Unsupported ComparativeContainment weight variant: {}",
                    other
                ),
            };
            (data, resolved_variant.clone())
        }

        // ExactCoverBy3Sets
        "ExactCoverBy3Sets" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "ExactCoverBy3Sets requires --universe and --sets\n\n\
                     Usage: pred create X3C --universe 6 --sets \"0,1,2;3,4,5\""
                )
            })?;
            if universe % 3 != 0 {
                bail!("Universe size must be divisible by 3, got {}", universe);
            }
            let sets = parse_sets(args)?;
            // Validate each set has exactly 3 distinct elements within the universe
            for (i, set) in sets.iter().enumerate() {
                if set.len() != 3 {
                    bail!(
                        "Subset {} has {} elements, but X3C requires exactly 3 elements per subset",
                        i,
                        set.len()
                    );
                }
                if set[0] == set[1] || set[0] == set[2] || set[1] == set[2] {
                    bail!("Subset {} contains duplicate elements: {:?}", i, set);
                }
                for &elem in set {
                    if elem >= universe {
                        bail!(
                            "Subset {} contains element {} which is outside universe of size {}",
                            i,
                            elem,
                            universe
                        );
                    }
                }
            }
            let subsets: Vec<[usize; 3]> = sets.into_iter().map(|s| [s[0], s[1], s[2]]).collect();
            (
                ser(problemreductions::models::set::ExactCoverBy3Sets::new(
                    universe, subsets,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SetBasis
        "SetBasis" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "SetBasis requires --universe, --sets, and --k\n\n\
                     Usage: pred create SetBasis --universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3"
                )
            })?;
            let k = args.k.ok_or_else(|| {
                anyhow::anyhow!(
                    "SetBasis requires --k\n\n\
                     Usage: pred create SetBasis --universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3"
                )
            })?;
            let sets = parse_sets(args)?;
            for (i, set) in sets.iter().enumerate() {
                for &element in set {
                    if element >= universe {
                        bail!(
                            "Set {} contains element {} which is outside universe of size {}",
                            i,
                            element,
                            universe
                        );
                    }
                }
            }
            (
                ser(problemreductions::models::set::SetBasis::new(
                    universe, sets, k,
                ))?,
                resolved_variant.clone(),
            )
        }

        // MinimumCardinalityKey
        "MinimumCardinalityKey" => {
            let num_attributes = args.num_attributes.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumCardinalityKey requires --num-attributes and --dependencies\n\n\
                     Usage: pred create MinimumCardinalityKey --num-attributes 6 --dependencies \"0,1>2;0,2>3;1,3>4;2,4>5\""
                )
            })?;
            let deps_str = args.dependencies.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumCardinalityKey requires --dependencies (e.g., \"0,1>2;0,2>3\")"
                )
            })?;
            let dependencies = parse_dependencies(deps_str)?;
            (
                ser(problemreductions::models::set::MinimumCardinalityKey::new(
                    num_attributes,
                    dependencies,
                ))?,
                resolved_variant.clone(),
            )
        }

        // TwoDimensionalConsecutiveSets
        "TwoDimensionalConsecutiveSets" => {
            let alphabet_size = args.alphabet_size.or(args.universe).ok_or_else(|| {
                anyhow::anyhow!(
                    "TwoDimensionalConsecutiveSets requires --alphabet-size (or --universe) and --sets\n\n\
                     Usage: pred create TwoDimensionalConsecutiveSets --alphabet-size 6 --sets \"0,1,2;3,4,5;1,3;2,4;0,5\""
                )
            })?;
            let sets = parse_sets(args)?;
            (
                ser(
                    problemreductions::models::set::TwoDimensionalConsecutiveSets::try_new(
                        alphabet_size,
                        sets,
                    )
                    .map_err(anyhow::Error::msg)?,
                )?,
                resolved_variant.clone(),
            )
        }

        // RootedTreeStorageAssignment
        "RootedTreeStorageAssignment" => {
            let usage =
                "Usage: pred create RootedTreeStorageAssignment --universe 5 --sets \"0,2;1,3;0,4;2,4\" --bound 1";
            let universe_size = args.universe.ok_or_else(|| {
                anyhow::anyhow!("RootedTreeStorageAssignment requires --universe\n\n{usage}")
            })?;
            let subsets = parse_sets(args)?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("RootedTreeStorageAssignment requires --bound\n\n{usage}")
            })?;
            let bound = parse_nonnegative_usize_bound(bound, "RootedTreeStorageAssignment", usage)?;
            (
                ser(
                    problemreductions::models::set::RootedTreeStorageAssignment::try_new(
                        universe_size,
                        subsets,
                        bound,
                    )
                    .map_err(anyhow::Error::msg)?,
                )?,
                resolved_variant.clone(),
            )
        }

        // BicliqueCover
        "BicliqueCover" => {
            let usage = "pred create BicliqueCover --left 2 --right 2 --biedges 0-0,0-1,1-1 --k 2";
            let (graph, k) =
                parse_bipartite_problem_input(args, "BicliqueCover", "number of bicliques", usage)?;
            (ser(BicliqueCover::new(graph, k))?, resolved_variant.clone())
        }

        // BalancedCompleteBipartiteSubgraph
        "BalancedCompleteBipartiteSubgraph" => {
            let usage = "pred create BalancedCompleteBipartiteSubgraph --left 4 --right 4 --biedges 0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3 --k 3";
            let (graph, k) = parse_bipartite_problem_input(
                args,
                "BalancedCompleteBipartiteSubgraph",
                "balanced biclique size",
                usage,
            )?;
            (
                ser(BalancedCompleteBipartiteSubgraph::new(graph, k))?,
                resolved_variant.clone(),
            )
        }

        // BMF
        "BMF" => {
            let matrix = parse_bool_matrix(args)?;
            let rank = args.rank.ok_or_else(|| {
                anyhow::anyhow!(
                    "BMF requires --matrix and --rank\n\n\
                     Usage: pred create BMF --matrix \"1,0;0,1;1,1\" --rank 2"
                )
            })?;
            (ser(BMF::new(matrix, rank))?, resolved_variant.clone())
        }

        // ConsecutiveBlockMinimization
        "ConsecutiveBlockMinimization" => {
            let usage = "Usage: pred create ConsecutiveBlockMinimization --matrix '[[true,false,true],[false,true,true]]' --bound 2";
            let matrix_str = args.matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array and --bound\n\n{usage}"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("ConsecutiveBlockMinimization requires --bound\n\n{usage}")
            })?;
            let matrix: Vec<Vec<bool>> = serde_json::from_str(matrix_str).map_err(|err| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array (e.g., '[[true,false,true],[false,true,true]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            (
                ser(ConsecutiveBlockMinimization::try_new(matrix, bound)
                    .map_err(|err| anyhow::anyhow!("{err}\n\n{usage}"))?)?,
                resolved_variant.clone(),
            )
        }

        // RectilinearPictureCompression
        "RectilinearPictureCompression" => {
            let matrix = parse_bool_matrix(args)?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "RectilinearPictureCompression requires --matrix and --bound\n\n\
                     Usage: pred create RectilinearPictureCompression --matrix \"1,1,0,0;1,1,0,0;0,0,1,1;0,0,1,1\" --bound 2"
                )
            })?;
            (
                ser(RectilinearPictureCompression::new(matrix, bound))?,
                resolved_variant.clone(),
            )
        }

        // ConsecutiveOnesSubmatrix
        "ConsecutiveOnesSubmatrix" => {
            let matrix = parse_bool_matrix(args)?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveOnesSubmatrix requires --matrix and --bound\n\n\
                     Usage: pred create ConsecutiveOnesSubmatrix --matrix \"1,1,0,1;1,0,1,1;0,1,1,0\" --bound 3"
                )
            })?;
            (
                ser(ConsecutiveOnesSubmatrix::new(matrix, bound))?,
                resolved_variant.clone(),
            )
        }

        // ConsecutiveOnesMatrixAugmentation
        "ConsecutiveOnesMatrixAugmentation" => {
            let matrix = parse_bool_matrix(args)?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveOnesMatrixAugmentation requires --matrix and --bound\n\n\
                     Usage: pred create ConsecutiveOnesMatrixAugmentation --matrix \"1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0\" --bound 2"
                )
            })?;
            (
                ser(ConsecutiveOnesMatrixAugmentation::try_new(matrix, bound)
                    .map_err(|e| anyhow::anyhow!(e))?)?,
                resolved_variant.clone(),
            )
        }

        // SparseMatrixCompression
        "SparseMatrixCompression" => {
            let matrix = parse_bool_matrix(args)?;
            let usage = "Usage: pred create SparseMatrixCompression --matrix \"1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0\" --bound 2";
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("SparseMatrixCompression requires --matrix and --bound\n\n{usage}")
            })?;
            let bound = parse_nonnegative_usize_bound(bound, "SparseMatrixCompression", usage)?;
            if bound == 0 {
                anyhow::bail!("SparseMatrixCompression requires bound >= 1\n\n{usage}");
            }
            (
                ser(SparseMatrixCompression::new(matrix, bound))?,
                resolved_variant.clone(),
            )
        }

        // FeasibleBasisExtension
        "FeasibleBasisExtension" => {
            let usage = "Usage: pred create FeasibleBasisExtension --matrix '[[1,0,1],[0,1,0]]' --rhs '7,5' --required-columns '0'";
            let matrix_str = args.matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "FeasibleBasisExtension requires --matrix (JSON 2D i64 array), --rhs, and --required-columns\n\n{usage}"
                )
            })?;
            let matrix: Vec<Vec<i64>> = serde_json::from_str(matrix_str).map_err(|err| {
                anyhow::anyhow!(
                    "FeasibleBasisExtension requires --matrix as a JSON 2D integer array (e.g., '[[1,0,1],[0,1,0]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            let rhs_str = args.rhs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "FeasibleBasisExtension requires --rhs (comma-separated integers)\n\n{usage}"
                )
            })?;
            let rhs: Vec<i64> = rhs_str
                .split(',')
                .map(|s| s.trim().parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| {
                    anyhow::anyhow!(
                        "Failed to parse --rhs as comma-separated integers: {err}\n\n{usage}"
                    )
                })?;
            let required_str = args.required_columns.as_deref().unwrap_or("");
            let required_columns: Vec<usize> = if required_str.is_empty() {
                vec![]
            } else {
                required_str
                    .split(',')
                    .map(|s| s.trim().parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| {
                        anyhow::anyhow!(
                            "Failed to parse --required-columns as comma-separated indices: {err}\n\n{usage}"
                        )
                    })?
            };
            (
                ser(FeasibleBasisExtension::new(matrix, rhs, required_columns))?,
                resolved_variant.clone(),
            )
        }

        // LongestCommonSubsequence
        "LongestCommonSubsequence" => {
            let usage =
                "Usage: pred create LCS --strings \"010110;100101;001011\" [--alphabet-size 2]";
            let strings_str = args.strings.as_deref().ok_or_else(|| {
                anyhow::anyhow!("LongestCommonSubsequence requires --strings\n\n{usage}")
            })?;

            let segments: Vec<&str> = strings_str.split(';').map(str::trim).collect();
            let comma_mode = segments.iter().any(|segment| segment.contains(','));

            let (strings, inferred_alphabet_size): (Vec<Vec<usize>>, usize) = if comma_mode {
                let strings = segments
                    .iter()
                    .map(|segment| {
                        if segment.is_empty() {
                            return Ok(Vec::new());
                        }
                        segment
                            .split(',')
                            .map(|value| {
                                value.trim().parse::<usize>().map_err(|e| {
                                    anyhow::anyhow!("Invalid LCS alphabet index: {}", e)
                                })
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                    .collect::<Result<Vec<_>>>()?;
                let inferred = strings
                    .iter()
                    .flat_map(|string| string.iter())
                    .copied()
                    .max()
                    .map(|value| value + 1)
                    .unwrap_or(0);
                (strings, inferred)
            } else {
                let mut encoding = BTreeMap::new();
                let mut next_symbol = 0usize;
                let strings = segments
                    .iter()
                    .map(|segment| {
                        segment
                            .as_bytes()
                            .iter()
                            .map(|byte| {
                                let entry = encoding.entry(*byte).or_insert_with(|| {
                                    let current = next_symbol;
                                    next_symbol += 1;
                                    current
                                });
                                *entry
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                (strings, next_symbol)
            };

            let alphabet_size = args.alphabet_size.unwrap_or(inferred_alphabet_size);
            anyhow::ensure!(
                alphabet_size >= inferred_alphabet_size,
                "--alphabet-size {} is smaller than the inferred alphabet size ({})",
                alphabet_size,
                inferred_alphabet_size
            );
            anyhow::ensure!(
                strings.iter().any(|string| !string.is_empty()),
                "LongestCommonSubsequence requires at least one non-empty string.\n\n{usage}"
            );
            anyhow::ensure!(
                alphabet_size > 0,
                "LongestCommonSubsequence requires a positive alphabet. Provide --alphabet-size when all strings are empty.\n\n{usage}"
            );
            (
                ser(LongestCommonSubsequence::new(alphabet_size, strings))?,
                resolved_variant.clone(),
            )
        }

        // GroupingBySwapping
        "GroupingBySwapping" => {
            let usage =
                "Usage: pred create GroupingBySwapping --string \"0,1,2,0,1,2\" --bound 5 [--alphabet-size 3]";
            let string_str = args.string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("GroupingBySwapping requires --string\n\n{usage}")
            })?;
            let bound = parse_nonnegative_usize_bound(
                args.bound.ok_or_else(|| {
                    anyhow::anyhow!("GroupingBySwapping requires --bound\n\n{usage}")
                })?,
                "GroupingBySwapping",
                usage,
            )?;

            let string = if string_str.trim().is_empty() {
                Vec::new()
            } else {
                string_str
                    .split(',')
                    .map(|value| {
                        value
                            .trim()
                            .parse::<usize>()
                            .context("invalid symbol index")
                    })
                    .collect::<Result<Vec<_>>>()?
            };
            let inferred = string.iter().copied().max().map_or(0, |value| value + 1);
            let alphabet_size = args.alphabet_size.unwrap_or(inferred);
            anyhow::ensure!(
                alphabet_size >= inferred,
                "--alphabet-size {} is smaller than max symbol + 1 ({}) in the input string",
                alphabet_size,
                inferred
            );
            anyhow::ensure!(
                alphabet_size > 0 || string.is_empty(),
                "GroupingBySwapping requires a positive alphabet for non-empty strings.\n\n{usage}"
            );
            anyhow::ensure!(
                !string.is_empty() || bound == 0,
                "GroupingBySwapping requires --bound 0 when --string is empty.\n\n{usage}"
            );
            (
                ser(GroupingBySwapping::new(alphabet_size, string, bound))?,
                resolved_variant.clone(),
            )
        }

        // ClosestVectorProblem
        "ClosestVectorProblem" => {
            let basis_str = args.basis.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "CVP requires --basis, --target-vec\n\n\
                     Usage: pred create CVP --basis \"1,0;0,1\" --target-vec \"0.5,0.5\""
                )
            })?;
            let target_str = args
                .target_vec
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("CVP requires --target-vec (e.g., \"0.5,0.5\")"))?;
            let basis: Vec<Vec<i32>> = basis_str
                .split(';')
                .map(|row| util::parse_comma_list(row.trim()))
                .collect::<Result<Vec<_>>>()?;
            let target: Vec<f64> = util::parse_comma_list(target_str)?;
            let n = basis.len();
            let (lo, hi) = match args.bounds.as_deref() {
                Some(s) => {
                    let parts: Vec<i64> = util::parse_comma_list(s)?;
                    if parts.len() != 2 {
                        bail!("--bounds expects \"lower,upper\" (e.g., \"-10,10\")");
                    }
                    (parts[0], parts[1])
                }
                None => (-10, 10),
            };
            let bounds = vec![problemreductions::models::algebraic::VarBounds::bounded(lo, hi); n];
            (
                ser(ClosestVectorProblem::new(basis, target, bounds))?,
                resolved_variant.clone(),
            )
        }

        // ResourceConstrainedScheduling
        "ResourceConstrainedScheduling" => {
            let usage = "Usage: pred create ResourceConstrainedScheduling --num-processors 3 --resource-bounds \"20\" --resource-requirements \"6;7;7;6;8;6\" --deadline 2";
            let num_processors = args.num_processors.ok_or_else(|| {
                anyhow::anyhow!(
                    "ResourceConstrainedScheduling requires --num-processors\n\n{usage}"
                )
            })?;
            let bounds_str = args.resource_bounds.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ResourceConstrainedScheduling requires --resource-bounds\n\n{usage}"
                )
            })?;
            let reqs_str = args.resource_requirements.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ResourceConstrainedScheduling requires --resource-requirements\n\n{usage}"
                )
            })?;
            let deadline = args.deadline.ok_or_else(|| {
                anyhow::anyhow!("ResourceConstrainedScheduling requires --deadline\n\n{usage}")
            })?;

            let resource_bounds: Vec<u64> = util::parse_comma_list(bounds_str)?;
            let resource_requirements: Vec<Vec<u64>> = reqs_str
                .split(';')
                .map(|row| util::parse_comma_list(row.trim()))
                .collect::<Result<Vec<_>>>()?;

            (
                ser(ResourceConstrainedScheduling::new(
                    num_processors,
                    resource_bounds,
                    resource_requirements,
                    deadline,
                ))?,
                resolved_variant.clone(),
            )
        }

        // MultiprocessorScheduling
        "MultiprocessorScheduling" => {
            let usage = "Usage: pred create MultiprocessorScheduling --lengths 4,5,3,2,6 --num-processors 2 --deadline 10";
            let lengths_str = args.lengths.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MultiprocessorScheduling requires --lengths, --num-processors, and --deadline\n\n{usage}"
                )
            })?;
            let num_processors = args.num_processors.ok_or_else(|| {
                anyhow::anyhow!("MultiprocessorScheduling requires --num-processors\n\n{usage}")
            })?;
            if num_processors == 0 {
                bail!("MultiprocessorScheduling requires --num-processors > 0\n\n{usage}");
            }
            let deadline = args.deadline.ok_or_else(|| {
                anyhow::anyhow!("MultiprocessorScheduling requires --deadline\n\n{usage}")
            })?;
            let lengths: Vec<u64> = util::parse_comma_list(lengths_str)?;
            (
                ser(MultiprocessorScheduling::new(
                    lengths,
                    num_processors,
                    deadline,
                ))?,
                resolved_variant.clone(),
            )
        }

        "ProductionPlanning" => {
            let usage = "Usage: pred create ProductionPlanning --num-periods 6 --demands 5,3,7,2,8,5 --capacities 12,12,12,12,12,12 --setup-costs 10,10,10,10,10,10 --production-costs 1,1,1,1,1,1 --inventory-costs 1,1,1,1,1,1 --cost-bound 80";
            let num_periods = args.num_periods.ok_or_else(|| {
                anyhow::anyhow!("ProductionPlanning requires --num-periods\n\n{usage}")
            })?;
            let demands = parse_named_u64_list(
                args.demands.as_deref(),
                "ProductionPlanning",
                "--demands",
                usage,
            )?;
            let capacities = parse_named_u64_list(
                args.capacities.as_deref(),
                "ProductionPlanning",
                "--capacities",
                usage,
            )?;
            let setup_costs = parse_named_u64_list(
                args.setup_costs.as_deref(),
                "ProductionPlanning",
                "--setup-costs",
                usage,
            )?;
            let production_costs = parse_named_u64_list(
                args.production_costs.as_deref(),
                "ProductionPlanning",
                "--production-costs",
                usage,
            )?;
            let inventory_costs = parse_named_u64_list(
                args.inventory_costs.as_deref(),
                "ProductionPlanning",
                "--inventory-costs",
                usage,
            )?;
            let cost_bound = args.cost_bound.ok_or_else(|| {
                anyhow::anyhow!("ProductionPlanning requires --cost-bound\n\n{usage}")
            })? as u64;

            for (flag, len) in [
                ("--demands", demands.len()),
                ("--capacities", capacities.len()),
                ("--setup-costs", setup_costs.len()),
                ("--production-costs", production_costs.len()),
                ("--inventory-costs", inventory_costs.len()),
            ] {
                ensure_named_len(len, num_periods, flag, usage)?;
            }

            (
                ser(ProductionPlanning::new(
                    num_periods,
                    demands,
                    capacities,
                    setup_costs,
                    production_costs,
                    inventory_costs,
                    cost_bound,
                ))?,
                resolved_variant.clone(),
            )
        }

        "CapacityAssignment" => {
            let usage = "Usage: pred create CapacityAssignment --capacities 1,2,3 --cost-matrix \"1,3,6;2,4,7;1,2,5\" --delay-matrix \"8,4,1;7,3,1;6,3,1\" --delay-budget 12";
            let capacities_str = args.capacities.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "CapacityAssignment requires --capacities, --cost-matrix, --delay-matrix, and --delay-budget\n\n{usage}"
                )
            })?;
            let cost_matrix_str = args.cost_matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!("CapacityAssignment requires --cost-matrix\n\n{usage}")
            })?;
            let delay_matrix_str = args.delay_matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!("CapacityAssignment requires --delay-matrix\n\n{usage}")
            })?;
            let delay_budget = args.delay_budget.ok_or_else(|| {
                anyhow::anyhow!("CapacityAssignment requires --delay-budget\n\n{usage}")
            })?;

            let capacities: Vec<u64> = util::parse_comma_list(capacities_str)?;
            anyhow::ensure!(
                !capacities.is_empty(),
                "CapacityAssignment requires at least one capacity value\n\n{usage}"
            );
            anyhow::ensure!(
                capacities.iter().all(|&capacity| capacity > 0),
                "CapacityAssignment capacities must be positive\n\n{usage}"
            );
            anyhow::ensure!(
                capacities.windows(2).all(|w| w[0] < w[1]),
                "CapacityAssignment capacities must be strictly increasing\n\n{usage}"
            );

            let cost = parse_u64_matrix_rows(cost_matrix_str, "cost")?;
            let delay = parse_u64_matrix_rows(delay_matrix_str, "delay")?;
            anyhow::ensure!(
                cost.len() == delay.len(),
                "cost matrix row count ({}) must match delay matrix row count ({})\n\n{usage}",
                cost.len(),
                delay.len()
            );

            for (index, row) in cost.iter().enumerate() {
                anyhow::ensure!(
                    row.len() == capacities.len(),
                    "cost row {} length ({}) must match capacities length ({})\n\n{usage}",
                    index,
                    row.len(),
                    capacities.len()
                );
                anyhow::ensure!(
                    row.windows(2).all(|w| w[0] <= w[1]),
                    "cost row {} must be non-decreasing\n\n{usage}",
                    index
                );
            }
            for (index, row) in delay.iter().enumerate() {
                anyhow::ensure!(
                    row.len() == capacities.len(),
                    "delay row {} length ({}) must match capacities length ({})\n\n{usage}",
                    index,
                    row.len(),
                    capacities.len()
                );
                anyhow::ensure!(
                    row.windows(2).all(|w| w[0] >= w[1]),
                    "delay row {} must be non-increasing\n\n{usage}",
                    index
                );
            }

            (
                ser(CapacityAssignment::new(
                    capacities,
                    cost,
                    delay,
                    delay_budget,
                ))?,
                resolved_variant.clone(),
            )
        }

        // MinimumMultiwayCut
        "MinimumMultiwayCut" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create MinimumMultiwayCut --graph 0-1,1-2,2-3 --terminals 0,2 [--edge-weights 1,1,1]"
                )
            })?;
            let terminals = parse_terminals(args, graph.num_vertices())?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            (
                ser(MinimumMultiwayCut::new(graph, terminals, edge_weights))?,
                resolved_variant.clone(),
            )
        }

        // MinimumTardinessSequencing
        "MinimumTardinessSequencing" => {
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumTardinessSequencing requires --deadlines and --n\n\n\
                     Usage: pred create MinimumTardinessSequencing --n 5 --deadlines 5,5,5,3,3 [--precedence-pairs \"0>3,1>3,1>4,2>4\"]"
                )
            })?;
            let num_tasks = args.n.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumTardinessSequencing requires --n (number of tasks)\n\n\
                     Usage: pred create MinimumTardinessSequencing --n 5 --deadlines 5,5,5,3,3"
                )
            })?;
            let deadlines: Vec<usize> = util::parse_comma_list(deadlines_str)?;
            let precedences = parse_precedence_pairs(args.precedence_pairs.as_deref())?;
            anyhow::ensure!(
                deadlines.len() == num_tasks,
                "deadlines length ({}) must equal num_tasks ({})",
                deadlines.len(),
                num_tasks
            );
            validate_precedence_pairs(&precedences, num_tasks)?;
            (
                ser(MinimumTardinessSequencing::new(
                    num_tasks,
                    deadlines,
                    precedences,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SchedulingWithIndividualDeadlines
        "SchedulingWithIndividualDeadlines" => {
            let usage = "Usage: pred create SchedulingWithIndividualDeadlines --n 7 --deadlines 2,1,2,2,3,3,2 [--num-processors 3 | --m 3] [--precedence-pairs \"0>3,1>3,1>4,2>4,2>5\"]";
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --deadlines, --n, and a processor count (--num-processors or --m)\n\n{usage}"
                )
            })?;
            let num_tasks = args.n.ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --n (number of tasks)\n\n{usage}"
                )
            })?;
            let num_processors = resolve_processor_count_flags(
                "SchedulingWithIndividualDeadlines",
                usage,
                args.num_processors,
                args.m,
            )?
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --num-processors or --m\n\n{usage}"
                )
            })?;
            let deadlines: Vec<usize> = util::parse_comma_list(deadlines_str)?;
            let precedences: Vec<(usize, usize)> = match args.precedence_pairs.as_deref() {
                Some(s) if !s.is_empty() => s
                    .split(',')
                    .map(|pair| {
                        let parts: Vec<&str> = pair.trim().split('>').collect();
                        anyhow::ensure!(
                            parts.len() == 2,
                            "Invalid precedence format '{}', expected 'u>v'",
                            pair.trim()
                        );
                        Ok((
                            parts[0].trim().parse::<usize>()?,
                            parts[1].trim().parse::<usize>()?,
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?,
                _ => vec![],
            };
            anyhow::ensure!(
                deadlines.len() == num_tasks,
                "deadlines length ({}) must equal num_tasks ({})",
                deadlines.len(),
                num_tasks
            );
            for &(pred, succ) in &precedences {
                anyhow::ensure!(
                    pred < num_tasks && succ < num_tasks,
                    "precedence index out of range: ({}, {}) but num_tasks = {}",
                    pred,
                    succ,
                    num_tasks
                );
            }
            (
                ser(SchedulingWithIndividualDeadlines::new(
                    num_tasks,
                    num_processors,
                    deadlines,
                    precedences,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SequencingToMinimizeWeightedCompletionTime
        "SequencingToMinimizeWeightedCompletionTime" => {
            let lengths_str = args.lengths.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedCompletionTime requires --lengths and --weights\n\n\
                     Usage: pred create SequencingToMinimizeWeightedCompletionTime --lengths 2,1,3,1,2 --weights 3,5,1,4,2 [--precedence-pairs \"0>2,1>4\"]"
                )
            })?;
            let weights_str = args.weights.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedCompletionTime requires --weights\n\n\
                     Usage: pred create SequencingToMinimizeWeightedCompletionTime --lengths 2,1,3,1,2 --weights 3,5,1,4,2"
                )
            })?;
            let lengths: Vec<u64> = util::parse_comma_list(lengths_str)?;
            let weights: Vec<u64> = util::parse_comma_list(weights_str)?;
            anyhow::ensure!(
                lengths.len() == weights.len(),
                "lengths length ({}) must equal weights length ({})",
                lengths.len(),
                weights.len()
            );
            anyhow::ensure!(
                lengths.iter().all(|&length| length > 0),
                "task lengths must be positive"
            );
            let num_tasks = lengths.len();
            let precedences: Vec<(usize, usize)> = match args.precedence_pairs.as_deref() {
                Some(s) if !s.is_empty() => s
                    .split(',')
                    .map(|pair| {
                        let parts: Vec<&str> = pair.trim().split('>').collect();
                        anyhow::ensure!(
                            parts.len() == 2,
                            "Invalid precedence format '{}', expected 'u>v'",
                            pair.trim()
                        );
                        Ok((
                            parts[0].trim().parse::<usize>()?,
                            parts[1].trim().parse::<usize>()?,
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?,
                _ => vec![],
            };
            for &(pred, succ) in &precedences {
                anyhow::ensure!(
                    pred < num_tasks && succ < num_tasks,
                    "precedence index out of range: ({}, {}) but num_tasks = {}",
                    pred,
                    succ,
                    num_tasks
                );
            }
            (
                ser(SequencingToMinimizeWeightedCompletionTime::new(
                    lengths,
                    weights,
                    precedences,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SequencingToMinimizeWeightedTardiness
        "SequencingToMinimizeWeightedTardiness" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --sizes, --weights, --deadlines, and --bound\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --sizes 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let weights_str = args.weights.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --weights (comma-separated tardiness weights)\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --sizes 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --deadlines (comma-separated job deadlines)\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --sizes 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --bound\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --sizes 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            anyhow::ensure!(bound >= 0, "--bound must be non-negative");

            let lengths: Vec<u64> = util::parse_comma_list(sizes_str)?;
            let weights: Vec<u64> = util::parse_comma_list(weights_str)?;
            let deadlines: Vec<u64> = util::parse_comma_list(deadlines_str)?;

            anyhow::ensure!(
                lengths.len() == weights.len(),
                "sizes length ({}) must equal weights length ({})",
                lengths.len(),
                weights.len()
            );
            anyhow::ensure!(
                lengths.len() == deadlines.len(),
                "sizes length ({}) must equal deadlines length ({})",
                lengths.len(),
                deadlines.len()
            );

            (
                ser(SequencingToMinimizeWeightedTardiness::new(
                    lengths,
                    weights,
                    deadlines,
                    bound as u64,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SequencingToMinimizeMaximumCumulativeCost
        "SequencingToMinimizeMaximumCumulativeCost" => {
            let costs_str = args.costs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeMaximumCumulativeCost requires --costs\n\n\
                     Usage: pred create SequencingToMinimizeMaximumCumulativeCost --costs 2,-1,3,-2,1,-3 --precedence-pairs \"0>2,1>2,1>3,2>4,3>5,4>5\""
                )
            })?;
            let costs: Vec<i64> = util::parse_comma_list(costs_str)?;
            let precedences = parse_precedence_pairs(args.precedence_pairs.as_deref())?;
            validate_precedence_pairs(&precedences, costs.len())?;
            (
                ser(SequencingToMinimizeMaximumCumulativeCost::new(
                    costs,
                    precedences,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SequencingWithinIntervals
        "SequencingWithinIntervals" => {
            let usage = "Usage: pred create SequencingWithinIntervals --release-times 0,0,5 --deadlines 11,11,6 --lengths 3,1,1";
            let rt_str = args.release_times.as_deref().ok_or_else(|| {
                anyhow::anyhow!("SequencingWithinIntervals requires --release-times\n\n{usage}")
            })?;
            let dl_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!("SequencingWithinIntervals requires --deadlines\n\n{usage}")
            })?;
            let len_str = args.lengths.as_deref().ok_or_else(|| {
                anyhow::anyhow!("SequencingWithinIntervals requires --lengths\n\n{usage}")
            })?;
            let release_times: Vec<u64> = util::parse_comma_list(rt_str)?;
            let deadlines: Vec<u64> = util::parse_comma_list(dl_str)?;
            let lengths: Vec<u64> = util::parse_comma_list(len_str)?;
            validate_sequencing_within_intervals_inputs(
                &release_times,
                &deadlines,
                &lengths,
                usage,
            )?;
            (
                ser(SequencingWithinIntervals::new(
                    release_times,
                    deadlines,
                    lengths,
                ))?,
                resolved_variant.clone(),
            )
        }

        // OptimalLinearArrangement — graph only (optimization)
        "OptimalLinearArrangement" => {
            let usage = "Usage: pred create OptimalLinearArrangement --graph 0-1,1-2,2-3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            (
                ser(OptimalLinearArrangement::new(graph))?,
                resolved_variant.clone(),
            )
        }

        // RootedTreeArrangement — graph + bound
        "RootedTreeArrangement" => {
            let usage =
                "Usage: pred create RootedTreeArrangement --graph 0-1,0-2,1-2,2-3,3-4 --bound 7";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let bound_raw = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "RootedTreeArrangement requires --bound (upper bound K on total tree stretch)\n\n{usage}"
                )
            })?;
            let bound = parse_nonnegative_usize_bound(bound_raw, "RootedTreeArrangement", usage)?;
            (
                ser(RootedTreeArrangement::new(graph, bound))?,
                resolved_variant.clone(),
            )
        }

        // FlowShopScheduling
        "FlowShopScheduling" => {
            let task_str = args.task_lengths.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "FlowShopScheduling requires --task-lengths and --deadline\n\n\
                     Usage: pred create FlowShopScheduling --task-lengths \"3,4,2;2,3,5;4,1,3\" --deadline 25 --num-processors 3"
                )
            })?;
            let deadline = args.deadline.ok_or_else(|| {
                anyhow::anyhow!(
                    "FlowShopScheduling requires --deadline\n\n\
                     Usage: pred create FlowShopScheduling --task-lengths \"3,4,2;2,3,5;4,1,3\" --deadline 25 --num-processors 3"
                )
            })?;
            let task_lengths: Vec<Vec<u64>> = task_str
                .split(';')
                .map(|row| util::parse_comma_list(row.trim()))
                .collect::<Result<Vec<_>>>()?;
            let num_processors = resolve_processor_count_flags(
                "FlowShopScheduling",
                "Usage: pred create FlowShopScheduling --task-lengths \"3,4,2;2,3,5;4,1,3\" --deadline 25 --num-processors 3",
                args.num_processors,
                args.m,
            )?
            .or_else(|| task_lengths.first().map(Vec::len))
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Cannot infer num_processors from empty task list; use --num-processors"
                )
            })?;
            for (j, row) in task_lengths.iter().enumerate() {
                if row.len() != num_processors {
                    bail!(
                        "task_lengths row {} has {} entries, expected {} (num_processors)",
                        j,
                        row.len(),
                        num_processors
                    );
                }
            }
            (
                ser(FlowShopScheduling::new(
                    num_processors,
                    task_lengths,
                    deadline,
                ))?,
                resolved_variant.clone(),
            )
        }

        // JobShopScheduling
        "JobShopScheduling" => {
            let usage = "Usage: pred create JobShopScheduling --job-tasks \"0:3,1:4;1:2,0:3,1:2;0:4,1:3\" --num-processors 2";
            let job_tasks = args.job_tasks.as_deref().ok_or_else(|| {
                anyhow::anyhow!("JobShopScheduling requires --job-tasks\n\n{usage}")
            })?;
            let jobs = parse_job_shop_jobs(job_tasks)?;
            let inferred_processors = jobs
                .iter()
                .flat_map(|job| job.iter().map(|(processor, _)| *processor))
                .max()
                .map(|processor| processor + 1);
            let num_processors = resolve_processor_count_flags(
                "JobShopScheduling",
                usage,
                args.num_processors,
                args.m,
            )?
            .or(inferred_processors)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Cannot infer num_processors from empty job list; use --num-processors"
                )
            })?;
            anyhow::ensure!(
                num_processors > 0,
                "JobShopScheduling requires --num-processors > 0\n\n{usage}"
            );
            for (job_index, job) in jobs.iter().enumerate() {
                for (task_index, &(processor, _)) in job.iter().enumerate() {
                    anyhow::ensure!(
                        processor < num_processors,
                        "job {job_index} task {task_index} uses processor {processor}, but num_processors = {num_processors}"
                    );
                }
                for (task_index, pair) in job.windows(2).enumerate() {
                    anyhow::ensure!(
                        pair[0].0 != pair[1].0,
                        "job {job_index} tasks {task_index} and {} must use different processors\n\n{usage}",
                        task_index + 1
                    );
                }
            }
            (
                ser(JobShopScheduling::new(num_processors, jobs))?,
                resolved_variant.clone(),
            )
        }

        // StaffScheduling
        "StaffScheduling" => {
            let usage = "Usage: pred create StaffScheduling --schedules \"1,1,1,1,1,0,0;0,1,1,1,1,1,0;0,0,1,1,1,1,1;1,0,0,1,1,1,1;1,1,0,0,1,1,1\" --requirements 2,2,2,3,3,2,1 --num-workers 4 --k 5";
            let schedules = parse_schedules(args, usage)?;
            let requirements = parse_requirements(args, usage)?;
            let num_workers = args.num_workers.ok_or_else(|| {
                anyhow::anyhow!("StaffScheduling requires --num-workers\n\n{usage}")
            })?;
            let shifts_per_schedule = args
                .k
                .ok_or_else(|| anyhow::anyhow!("StaffScheduling requires --k\n\n{usage}"))?;
            validate_staff_scheduling_args(
                &schedules,
                &requirements,
                shifts_per_schedule,
                num_workers,
                usage,
            )?;

            (
                ser(problemreductions::models::misc::StaffScheduling::new(
                    shifts_per_schedule,
                    schedules,
                    requirements,
                    num_workers,
                ))?,
                resolved_variant.clone(),
            )
        }

        // TimetableDesign
        "TimetableDesign" => {
            let usage = "Usage: pred create TimetableDesign --num-periods 3 --num-craftsmen 5 --num-tasks 5 --craftsman-avail \"1,1,1;1,1,0;0,1,1;1,0,1;1,1,1\" --task-avail \"1,1,0;0,1,1;1,0,1;1,1,1;1,1,1\" --requirements \"1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0\"";
            let num_periods = args.num_periods.ok_or_else(|| {
                anyhow::anyhow!("TimetableDesign requires --num-periods\n\n{usage}")
            })?;
            let num_craftsmen = args.num_craftsmen.ok_or_else(|| {
                anyhow::anyhow!("TimetableDesign requires --num-craftsmen\n\n{usage}")
            })?;
            let num_tasks = args.num_tasks.ok_or_else(|| {
                anyhow::anyhow!("TimetableDesign requires --num-tasks\n\n{usage}")
            })?;
            let craftsman_avail =
                parse_named_bool_rows(args.craftsman_avail.as_deref(), "--craftsman-avail", usage)?;
            let task_avail =
                parse_named_bool_rows(args.task_avail.as_deref(), "--task-avail", usage)?;
            let requirements = parse_timetable_requirements(args.requirements.as_deref(), usage)?;
            validate_timetable_design_args(
                num_periods,
                num_craftsmen,
                num_tasks,
                &craftsman_avail,
                &task_avail,
                &requirements,
                usage,
            )?;

            (
                ser(TimetableDesign::new(
                    num_periods,
                    num_craftsmen,
                    num_tasks,
                    craftsman_avail,
                    task_avail,
                    requirements,
                ))?,
                resolved_variant.clone(),
            )
        }

        // DirectedTwoCommodityIntegralFlow
        "DirectedTwoCommodityIntegralFlow" => {
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "DirectedTwoCommodityIntegralFlow requires --arcs\n\n\
                     Usage: pred create DirectedTwoCommodityIntegralFlow \
                     --arcs \"0>2,0>3,1>2,1>3,2>4,2>5,3>4,3>5\" \
                     --capacities 1,1,1,1,1,1,1,1 \
                     --source-1 0 --sink-1 4 --source-2 1 --sink-2 5 \
                     --requirement-1 1 --requirement-2 1"
                )
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let capacities: Vec<u64> = if let Some(ref s) = args.capacities {
                util::parse_comma_list(s)?
            } else {
                vec![1; num_arcs]
            };
            anyhow::ensure!(
                capacities.len() == num_arcs,
                "capacities length ({}) must match number of arcs ({num_arcs})",
                capacities.len()
            );
            let n = graph.num_vertices();
            let source_1 = args.source_1.ok_or_else(|| {
                anyhow::anyhow!("DirectedTwoCommodityIntegralFlow requires --source-1")
            })?;
            let sink_1 = args.sink_1.ok_or_else(|| {
                anyhow::anyhow!("DirectedTwoCommodityIntegralFlow requires --sink-1")
            })?;
            let source_2 = args.source_2.ok_or_else(|| {
                anyhow::anyhow!("DirectedTwoCommodityIntegralFlow requires --source-2")
            })?;
            let sink_2 = args.sink_2.ok_or_else(|| {
                anyhow::anyhow!("DirectedTwoCommodityIntegralFlow requires --sink-2")
            })?;
            for (name, idx) in [
                ("source_1", source_1),
                ("sink_1", sink_1),
                ("source_2", source_2),
                ("sink_2", sink_2),
            ] {
                anyhow::ensure!(idx < n, "{name} ({idx}) >= num_vertices ({n})");
            }
            let requirement_1 = args.requirement_1.unwrap_or(1);
            let requirement_2 = args.requirement_2.unwrap_or(1);
            (
                ser(DirectedTwoCommodityIntegralFlow::new(
                    graph,
                    capacities,
                    source_1,
                    sink_1,
                    source_2,
                    sink_2,
                    requirement_1,
                    requirement_2,
                ))?,
                resolved_variant.clone(),
            )
        }

        // IntegralFlowHomologousArcs
        "IntegralFlowHomologousArcs" => {
            let usage = "Usage: pred create IntegralFlowHomologousArcs --arcs \"0>1,0>2,1>3,2>3,1>4,2>4,3>5,4>5\" --capacities 1,1,1,1,1,1,1,1 --source 0 --sink 5 --requirement 2 --homologous-pairs \"2=5;4=3\"";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --arcs\n\n{usage}")
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities: Vec<u64> = if let Some(ref s) = args.capacities {
                s.split(',')
                    .map(|token| {
                        let trimmed = token.trim();
                        trimmed
                            .parse::<u64>()
                            .with_context(|| format!("Invalid capacity `{trimmed}`\n\n{usage}"))
                    })
                    .collect::<Result<Vec<_>>>()?
            } else {
                vec![1; num_arcs]
            };
            anyhow::ensure!(
                capacities.len() == num_arcs,
                "Expected {} capacities but got {}\n\n{}",
                num_arcs,
                capacities.len(),
                usage
            );
            for (arc_index, &capacity) in capacities.iter().enumerate() {
                let fits = usize::try_from(capacity)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .is_some();
                anyhow::ensure!(
                    fits,
                    "capacity {} at arc index {} is too large for this platform\n\n{}",
                    capacity,
                    arc_index,
                    usage
                );
            }
            let num_vertices = graph.num_vertices();
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --sink\n\n{usage}")
            })?;
            let requirement = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowHomologousArcs requires --requirement\n\n{usage}")
            })?;
            validate_vertex_index("source", source, num_vertices, usage)?;
            validate_vertex_index("sink", sink, num_vertices, usage)?;
            let homologous_pairs =
                parse_homologous_pairs(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            for &(a, b) in &homologous_pairs {
                anyhow::ensure!(
                    a < num_arcs && b < num_arcs,
                    "homologous pair ({}, {}) references arc >= num_arcs ({})\n\n{}",
                    a,
                    b,
                    num_arcs,
                    usage
                );
            }
            (
                ser(IntegralFlowHomologousArcs::new(
                    graph,
                    capacities,
                    source,
                    sink,
                    requirement,
                    homologous_pairs,
                ))?,
                resolved_variant.clone(),
            )
        }

        // PathConstrainedNetworkFlow
        "PathConstrainedNetworkFlow" => {
            let usage = "Usage: pred create PathConstrainedNetworkFlow --arcs \"0>1,0>2,1>3,1>4,2>4,3>5,4>5,4>6,5>7,6>7\" --capacities 2,1,1,1,1,1,1,1,2,1 --source 0 --sink 7 --paths \"0,2,5,8;0,3,6,8;0,3,7,9;1,4,6,8;1,4,7,9\" --requirement 3";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --arcs\n\n{usage}")
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities: Vec<u64> = if let Some(ref s) = args.capacities {
                util::parse_comma_list(s)?
            } else {
                vec![1; num_arcs]
            };
            anyhow::ensure!(
                capacities.len() == num_arcs,
                "capacities length ({}) must match number of arcs ({num_arcs})",
                capacities.len()
            );
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --sink\n\n{usage}")
            })?;
            let requirement = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --requirement\n\n{usage}")
            })?;
            let paths = parse_prescribed_paths(args, num_arcs, usage)?;
            (
                ser(PathConstrainedNetworkFlow::new(
                    graph,
                    capacities,
                    source,
                    sink,
                    paths,
                    requirement,
                ))?,
                resolved_variant.clone(),
            )
        }

        // MinimumFeedbackArcSet
        "MinimumFeedbackArcSet" => {
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumFeedbackArcSet requires --arcs\n\n\
                     Usage: pred create FAS --arcs \"0>1,1>2,2>0\" [--weights 1,1,1] [--num-vertices N]"
                )
            })?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let weights = parse_arc_weights(args, num_arcs)?;
            (
                ser(MinimumFeedbackArcSet::new(graph, weights))?,
                resolved_variant.clone(),
            )
        }

        // AcyclicPartition
        "AcyclicPartition" => {
            let usage = "Usage: pred create AcyclicPartition/i32 --arcs \"0>1,0>2,1>3,1>4,2>4,2>5,3>5,4>5\" --weights 2,3,2,1,3,1 --arc-costs 1,1,1,1,1,1,1,1 --weight-bound 5 --cost-bound 5";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("AcyclicPartition requires --arcs\n\n{usage}"))?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let vertex_weights = parse_vertex_weights(args, graph.num_vertices())?;
            let arc_costs = parse_arc_costs(args, num_arcs)?;
            let weight_bound = args.weight_bound.ok_or_else(|| {
                anyhow::anyhow!("AcyclicPartition requires --weight-bound\n\n{usage}")
            })?;
            let cost_bound = args.cost_bound.ok_or_else(|| {
                anyhow::anyhow!("AcyclicPartition requires --cost-bound\n\n{usage}")
            })?;
            if vertex_weights.iter().any(|&weight| weight <= 0) {
                bail!("AcyclicPartition --weights must be positive (Z+)");
            }
            if arc_costs.iter().any(|&cost| cost <= 0) {
                bail!("AcyclicPartition --arc-costs must be positive (Z+)");
            }
            if weight_bound <= 0 {
                bail!("AcyclicPartition --weight-bound must be positive (Z+)");
            }
            if cost_bound <= 0 {
                bail!("AcyclicPartition --cost-bound must be positive (Z+)");
            }
            (
                ser(AcyclicPartition::new(
                    graph,
                    vertex_weights,
                    arc_costs,
                    weight_bound,
                    cost_bound,
                ))?,
                resolved_variant.clone(),
            )
        }

        // MinMaxMulticenter (vertex p-center)
        "MinMaxMulticenter" => {
            let usage = "Usage: pred create MinMaxMulticenter --graph 0-1,1-2,2-3 [--weights 1,1,1,1] [--edge-weights 1,1,1] --k 2";
            let (graph, n) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let vertex_weights = parse_vertex_weights(args, n)?;
            let edge_lengths = parse_edge_weights(args, graph.num_edges())?;
            let k = args.k.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinMaxMulticenter requires --k (number of centers)\n\n\
                     Usage: pred create MinMaxMulticenter --graph 0-1,1-2,2-3 --k 2"
                )
            })?;
            if vertex_weights.iter().any(|&weight| weight < 0) {
                bail!("MinMaxMulticenter --weights must be non-negative");
            }
            if edge_lengths.iter().any(|&length| length < 0) {
                bail!("MinMaxMulticenter --edge-weights must be non-negative");
            }
            (
                ser(MinMaxMulticenter::new(
                    graph,
                    vertex_weights,
                    edge_lengths,
                    k,
                ))?,
                resolved_variant.clone(),
            )
        }

        // StrongConnectivityAugmentation
        "StrongConnectivityAugmentation" => {
            let usage = "Usage: pred create StrongConnectivityAugmentation --arcs \"0>1,1>2\" --candidate-arcs \"2>0:1\" --bound 1 [--num-vertices N]";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "StrongConnectivityAugmentation requires --arcs\n\n\
                     {usage}"
                )
            })?;
            let (graph, _) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let candidate_arcs = parse_candidate_arcs(args, graph.num_vertices())?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "StrongConnectivityAugmentation requires --bound\n\n\
                     {usage}"
                )
            })? as i32;
            (
                ser(
                    StrongConnectivityAugmentation::try_new(graph, candidate_arcs, bound)
                        .map_err(|e| anyhow::anyhow!(e))?,
                )?,
                resolved_variant.clone(),
            )
        }

        // MinimumDummyActivitiesPert
        "MinimumDummyActivitiesPert" => {
            let usage = "Usage: pred create MinimumDummyActivitiesPert --arcs \"0>2,0>3,1>3,1>4,2>5\" [--num-vertices N]";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumDummyActivitiesPert requires --arcs\n\n\
                     {usage}"
                )
            })?;
            let (graph, _) = parse_directed_graph(arcs_str, args.num_vertices)?;
            (
                ser(MinimumDummyActivitiesPert::try_new(graph).map_err(|e| anyhow::anyhow!(e))?)?,
                resolved_variant.clone(),
            )
        }

        // MixedChinesePostman
        "MixedChinesePostman" => {
            let usage = "Usage: pred create MixedChinesePostman --graph 0-2,1-3,0-4,4-2 --arcs \"0>1,1>2,2>3,3>0\" --edge-weights 2,3,1,2 --arc-costs 2,3,1,4 [--num-vertices N]";
            let graph = parse_mixed_graph(args, usage)?;
            let arc_costs = parse_arc_costs(args, graph.num_arcs())?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            if arc_costs.iter().any(|&cost| cost < 0) {
                bail!("MixedChinesePostman --arc-costs must be non-negative\n\n{usage}");
            }
            if edge_weights.iter().any(|&weight| weight < 0) {
                bail!("MixedChinesePostman --edge-weights must be non-negative\n\n{usage}");
            }
            if resolved_variant.get("weight").map(|w| w.as_str()) == Some("One")
                && (arc_costs.iter().any(|&cost| cost != 1)
                    || edge_weights.iter().any(|&weight| weight != 1))
            {
                bail!(
                    "Non-unit lengths are not supported for MixedChinesePostman/One.\n\n\
                     Use the weighted variant instead:\n  pred create MixedChinesePostman/i32 --graph ... --arcs ... --edge-weights ... --arc-costs ..."
                );
            }
            (
                ser(MixedChinesePostman::new(graph, arc_costs, edge_weights))?,
                resolved_variant.clone(),
            )
        }

        // MinimumSumMulticenter (p-median)
        "MinimumSumMulticenter" => {
            let (graph, n) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create MinimumSumMulticenter --graph 0-1,1-2,2-3 [--weights 1,1,1,1] [--edge-weights 1,1,1] --k 2"
                )
            })?;
            let vertex_weights = parse_vertex_weights(args, n)?;
            let edge_lengths = parse_edge_weights(args, graph.num_edges())?;
            let k = args.k.ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumSumMulticenter requires --k (number of centers)\n\n\
                     Usage: pred create MinimumSumMulticenter --graph 0-1,1-2,2-3 --k 2"
                )
            })?;
            (
                ser(MinimumSumMulticenter::new(
                    graph,
                    vertex_weights,
                    edge_lengths,
                    k,
                ))?,
                resolved_variant.clone(),
            )
        }

        // SubgraphIsomorphism
        "SubgraphIsomorphism" => {
            let (host_graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create SubgraphIsomorphism --graph 0-1,1-2,2-0 --pattern 0-1"
                )
            })?;
            let pattern_str = args.pattern.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SubgraphIsomorphism requires --pattern (pattern graph edges)\n\n\
                     Usage: pred create SubgraphIsomorphism --graph 0-1,1-2,2-0 --pattern 0-1"
                )
            })?;
            let pattern_edges: Vec<(usize, usize)> = pattern_str
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
                            "Invalid edge '{}': self-loops are not allowed in simple graphs",
                            pair.trim()
                        );
                    }
                    Ok((u, v))
                })
                .collect::<Result<Vec<_>>>()?;
            let pattern_nv = pattern_edges
                .iter()
                .flat_map(|(u, v)| [*u, *v])
                .max()
                .map(|m| m + 1)
                .unwrap_or(0);
            let pattern_graph = SimpleGraph::new(pattern_nv, pattern_edges);
            (
                ser(SubgraphIsomorphism::new(host_graph, pattern_graph))?,
                resolved_variant.clone(),
            )
        }

        // PartitionIntoTriangles
        "PartitionIntoTriangles" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create PartitionIntoTriangles --graph 0-1,1-2,0-2"
                )
            })?;
            anyhow::ensure!(
                graph.num_vertices() % 3 == 0,
                "PartitionIntoTriangles requires vertex count divisible by 3, got {}",
                graph.num_vertices()
            );
            (
                ser(PartitionIntoTriangles::new(graph))?,
                resolved_variant.clone(),
            )
        }

        // ShortestCommonSupersequence
        "ShortestCommonSupersequence" => {
            let usage = "Usage: pred create SCS --strings \"0,1,2;1,2,0\"";
            let strings_str = args.strings.as_deref().ok_or_else(|| {
                anyhow::anyhow!("ShortestCommonSupersequence requires --strings\n\n{usage}")
            })?;
            let strings: Vec<Vec<usize>> = strings_str
                .split(';')
                .map(|s| {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        return Ok(Vec::new());
                    }
                    trimmed
                        .split(',')
                        .map(|v| {
                            v.trim()
                                .parse::<usize>()
                                .map_err(|e| anyhow::anyhow!("Invalid alphabet index: {}", e))
                        })
                        .collect::<Result<Vec<_>>>()
                })
                .collect::<Result<Vec<_>>>()?;
            let inferred = strings
                .iter()
                .flat_map(|s| s.iter())
                .copied()
                .max()
                .map(|m| m + 1)
                .unwrap_or(0);
            let alphabet_size = args.alphabet_size.unwrap_or(inferred);
            if alphabet_size < inferred {
                anyhow::bail!(
                    "--alphabet-size {} is smaller than the largest symbol + 1 ({}) in the strings",
                    alphabet_size,
                    inferred
                );
            }
            (
                ser(ShortestCommonSupersequence::new(alphabet_size, strings))?,
                resolved_variant.clone(),
            )
        }

        // MinimumFeedbackVertexSet
        "MinimumFeedbackVertexSet" => {
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumFeedbackVertexSet requires --arcs\n\n\
                     Usage: pred create FVS --arcs \"0>1,1>2,2>0\" [--weights 1,1,1] [--num-vertices N]"
                )
            })?;
            let (graph, _) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let num_v = graph.num_vertices();
            let weights = parse_vertex_weights(args, num_v)?;
            (
                ser(MinimumFeedbackVertexSet::new(graph, weights))?,
                resolved_variant.clone(),
            )
        }

        "ConjunctiveQueryFoldability" => {
            bail!(
                "ConjunctiveQueryFoldability has complex nested input.\n\n\
                 Use: pred create --example ConjunctiveQueryFoldability\n\
                 Or provide a JSON file directly."
            )
        }

        // PartitionIntoPathsOfLength2
        "PartitionIntoPathsOfLength2" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create PartitionIntoPathsOfLength2 --graph 0-1,1-2,3-4,4-5"
                )
            })?;
            if graph.num_vertices() % 3 != 0 {
                bail!(
                    "PartitionIntoPathsOfLength2 requires vertex count divisible by 3, got {}",
                    graph.num_vertices()
                );
            }
            (
                ser(problemreductions::models::graph::PartitionIntoPathsOfLength2::new(graph))?,
                resolved_variant.clone(),
            )
        }

        // ConjunctiveBooleanQuery
        "ConjunctiveBooleanQuery" => {
            let usage = "Usage: pred create CBQ --domain-size 6 --relations \"2:0,3|1,3;3:0,1,5|1,2,5\" --conjuncts-spec \"0:v0,c3;0:v1,c3;1:v0,v1,c5\"";
            let domain_size = args.domain_size.ok_or_else(|| {
                anyhow::anyhow!("ConjunctiveBooleanQuery requires --domain-size\n\n{usage}")
            })?;
            let relations_str = args.relations.as_deref().ok_or_else(|| {
                anyhow::anyhow!("ConjunctiveBooleanQuery requires --relations\n\n{usage}")
            })?;
            let conjuncts_str = args.conjuncts_spec.as_deref().ok_or_else(|| {
                anyhow::anyhow!("ConjunctiveBooleanQuery requires --conjuncts-spec\n\n{usage}")
            })?;
            // Parse relations: "arity:t1,t2|t3,t4;arity:t5,t6,t7|t8,t9,t10"
            // An empty tuple list (e.g., "2:") produces an empty relation.
            let relations: Vec<CbqRelation> = relations_str
                .split(';')
                .map(|rel_str| {
                    let rel_str = rel_str.trim();
                    let (arity_str, tuples_str) = rel_str.split_once(':').ok_or_else(|| {
                        anyhow::anyhow!(
                            "Invalid relation format: expected 'arity:tuples', got '{rel_str}'"
                        )
                    })?;
                    let arity: usize = arity_str
                        .trim()
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Invalid arity '{arity_str}': {e}"))?;
                    let tuples: Vec<Vec<usize>> = if tuples_str.trim().is_empty() {
                        Vec::new()
                    } else {
                        tuples_str
                            .split('|')
                            .filter(|t| !t.trim().is_empty())
                            .map(|t| {
                                let tuple: Vec<usize> = t
                                    .trim()
                                    .split(',')
                                    .map(|v| {
                                        v.trim().parse::<usize>().map_err(|e| {
                                            anyhow::anyhow!("Invalid tuple value: {e}")
                                        })
                                    })
                                    .collect::<Result<Vec<_>>>()?;
                                if tuple.len() != arity {
                                    bail!(
                                        "Relation tuple has {} entries, expected arity {arity}",
                                        tuple.len()
                                    );
                                }
                                for &val in &tuple {
                                    if val >= domain_size {
                                        bail!("Tuple value {val} >= domain-size {domain_size}");
                                    }
                                }
                                Ok(tuple)
                            })
                            .collect::<Result<Vec<_>>>()?
                    };
                    Ok(CbqRelation { arity, tuples })
                })
                .collect::<Result<Vec<_>>>()?;
            // Parse conjuncts: "rel_idx:arg1,arg2;rel_idx:arg1,arg2,arg3"
            let mut num_vars_inferred: usize = 0;
            let conjuncts: Vec<(usize, Vec<QueryArg>)> = conjuncts_str
                .split(';')
                .map(|conj_str| {
                    let conj_str = conj_str.trim();
                    let (idx_str, args_str) = conj_str.split_once(':').ok_or_else(|| {
                        anyhow::anyhow!(
                            "Invalid conjunct format: expected 'rel_idx:args', got '{conj_str}'"
                        )
                    })?;
                    let rel_idx: usize = idx_str.trim().parse().map_err(|e| {
                        anyhow::anyhow!("Invalid relation index '{idx_str}': {e}")
                    })?;
                    if rel_idx >= relations.len() {
                        bail!(
                            "Conjunct references relation {rel_idx}, but only {} relations exist",
                            relations.len()
                        );
                    }
                    let query_args: Vec<QueryArg> = args_str
                        .split(',')
                        .map(|a| {
                            let a = a.trim();
                            if let Some(rest) = a.strip_prefix('v') {
                                let v: usize = rest.parse().map_err(|e| {
                                    anyhow::anyhow!("Invalid variable index '{rest}': {e}")
                                })?;
                                if v + 1 > num_vars_inferred {
                                    num_vars_inferred = v + 1;
                                }
                                Ok(QueryArg::Variable(v))
                            } else if let Some(rest) = a.strip_prefix('c') {
                                let c: usize = rest.parse().map_err(|e| {
                                    anyhow::anyhow!("Invalid constant value '{rest}': {e}")
                                })?;
                                if c >= domain_size {
                                    bail!(
                                        "Constant {c} >= domain-size {domain_size}"
                                    );
                                }
                                Ok(QueryArg::Constant(c))
                            } else {
                                Err(anyhow::anyhow!(
                                    "Invalid query arg '{a}': expected vN (variable) or cN (constant)"
                                ))
                            }
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let expected_arity = relations[rel_idx].arity;
                    if query_args.len() != expected_arity {
                        bail!(
                            "Conjunct has {} args, but relation {rel_idx} has arity {expected_arity}",
                            query_args.len()
                        );
                    }
                    Ok((rel_idx, query_args))
                })
                .collect::<Result<Vec<_>>>()?;
            (
                ser(ConjunctiveBooleanQuery::new(
                    domain_size,
                    relations,
                    num_vars_inferred,
                    conjuncts,
                ))?,
                resolved_variant.clone(),
            )
        }

        // PartiallyOrderedKnapsack
        "PartiallyOrderedKnapsack" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "PartiallyOrderedKnapsack requires --sizes, --values, and --capacity (--precedences is optional)\n\n\
                     Usage: pred create PartiallyOrderedKnapsack --sizes 2,3,4,1,2,3 --values 3,2,5,4,3,8 --precedences \"0>2,0>3,1>4,3>5,4>5\" --capacity 11"
                )
            })?;
            let values_str = args.values.as_deref().ok_or_else(|| {
                anyhow::anyhow!("PartiallyOrderedKnapsack requires --values (e.g., 3,2,5,4,3,8)")
            })?;
            let cap_str = args.capacity.as_deref().ok_or_else(|| {
                anyhow::anyhow!("PartiallyOrderedKnapsack requires --capacity (e.g., 11)")
            })?;
            let weights: Vec<i64> = util::parse_comma_list(sizes_str)?;
            let values: Vec<i64> = util::parse_comma_list(values_str)?;
            let capacity: i64 = cap_str.parse()?;
            let precedences = match args.precedences.as_deref() {
                Some(s) if !s.trim().is_empty() => s
                    .split(',')
                    .map(|pair| {
                        let parts: Vec<&str> = pair.trim().split('>').collect();
                        anyhow::ensure!(
                            parts.len() == 2,
                            "Invalid precedence format '{}', expected 'a>b'",
                            pair.trim()
                        );
                        Ok((
                            parts[0].trim().parse::<usize>()?,
                            parts[1].trim().parse::<usize>()?,
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?,
                _ => vec![],
            };
            (
                ser(PartiallyOrderedKnapsack::new(
                    weights,
                    values,
                    precedences,
                    capacity,
                ))?,
                resolved_variant.clone(),
            )
        }

        // PrimeAttributeName
        "PrimeAttributeName" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "PrimeAttributeName requires --universe, --deps, and --query\n\n\
                     Usage: pred create PrimeAttributeName --universe 6 --deps \"0,1>2,3,4,5;2,3>0,1,4,5\" --query 3"
                )
            })?;
            let deps_str = args.deps.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "PrimeAttributeName requires --deps\n\n\
                     Usage: pred create PrimeAttributeName --universe 6 --deps \"0,1>2,3,4,5;2,3>0,1,4,5\" --query 3"
                )
            })?;
            let query = args.query.ok_or_else(|| {
                anyhow::anyhow!(
                    "PrimeAttributeName requires --query\n\n\
                     Usage: pred create PrimeAttributeName --universe 6 --deps \"0,1>2,3,4,5;2,3>0,1,4,5\" --query 3"
                )
            })?;
            let dependencies = parse_deps(deps_str)?;
            for (i, (lhs, rhs)) in dependencies.iter().enumerate() {
                for &attr in lhs.iter().chain(rhs.iter()) {
                    if attr >= universe {
                        bail!(
                            "Dependency {} references attribute {} outside universe of size {}",
                            i,
                            attr,
                            universe
                        );
                    }
                }
            }
            if query >= universe {
                bail!(
                    "Query attribute {} is outside universe of size {}",
                    query,
                    universe
                );
            }
            (
                ser(PrimeAttributeName::new(universe, dependencies, query))?,
                resolved_variant.clone(),
            )
        }

        // SequencingWithReleaseTimesAndDeadlines
        "SequencingWithReleaseTimesAndDeadlines" => {
            let lengths_str = args.lengths.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingWithReleaseTimesAndDeadlines requires --lengths, --release-times, and --deadlines\n\n\
                     Usage: pred create SequencingWithReleaseTimesAndDeadlines --lengths 3,2,4 --release-times 0,1,5 --deadlines 5,6,10"
                )
            })?;
            let release_str = args.release_times.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingWithReleaseTimesAndDeadlines requires --release-times\n\n\
                     Usage: pred create SequencingWithReleaseTimesAndDeadlines --lengths 3,2,4 --release-times 0,1,5 --deadlines 5,6,10"
                )
            })?;
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingWithReleaseTimesAndDeadlines requires --deadlines\n\n\
                     Usage: pred create SequencingWithReleaseTimesAndDeadlines --lengths 3,2,4 --release-times 0,1,5 --deadlines 5,6,10"
                )
            })?;
            let lengths: Vec<u64> = util::parse_comma_list(lengths_str)?;
            let release_times: Vec<u64> = util::parse_comma_list(release_str)?;
            let deadlines: Vec<u64> = util::parse_comma_list(deadlines_str)?;
            if lengths.len() != release_times.len() || lengths.len() != deadlines.len() {
                bail!(
                    "All three lists must have the same length: lengths={}, release_times={}, deadlines={}",
                    lengths.len(),
                    release_times.len(),
                    deadlines.len()
                );
            }
            (
                ser(SequencingWithReleaseTimesAndDeadlines::new(
                    lengths,
                    release_times,
                    deadlines,
                ))?,
                resolved_variant.clone(),
            )
        }

        // StringToStringCorrection
        "StringToStringCorrection" => {
            let usage = "Usage: pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2";
            let source_str = args.source_string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("StringToStringCorrection requires --source-string\n\n{usage}")
            })?;
            let target_str = args.target_string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("StringToStringCorrection requires --target-string\n\n{usage}")
            })?;
            let bound = parse_nonnegative_usize_bound(
                args.bound.ok_or_else(|| {
                    anyhow::anyhow!("StringToStringCorrection requires --bound\n\n{usage}")
                })?,
                "StringToStringCorrection",
                usage,
            )?;
            let parse_symbols = |s: &str| -> Result<Vec<usize>> {
                if s.trim().is_empty() {
                    return Ok(Vec::new());
                }
                s.split(',')
                    .map(|v| v.trim().parse::<usize>().context("invalid symbol index"))
                    .collect()
            };
            let source = parse_symbols(source_str)?;
            let target = parse_symbols(target_str)?;
            let inferred = source
                .iter()
                .chain(target.iter())
                .copied()
                .max()
                .map_or(0, |m| m + 1);
            let alphabet_size = args.alphabet_size.unwrap_or(inferred);
            if alphabet_size < inferred {
                anyhow::bail!(
                    "--alphabet-size {} is smaller than max symbol + 1 ({}) in the strings",
                    alphabet_size,
                    inferred
                );
            }
            (
                ser(StringToStringCorrection::new(
                    alphabet_size,
                    source,
                    target,
                    bound,
                ))?,
                resolved_variant.clone(),
            )
        }

        _ => bail!("{}", crate::problem_name::unknown_problem_error(canonical)),
    };

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
fn create_vertex_weight_problem(
    args: &CreateArgs,
    canonical: &str,
    graph_type: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> Result<(serde_json::Value, BTreeMap<String, String>)> {
    match graph_type {
        "KingsSubgraph" => {
            let positions = parse_int_positions(args)?;
            let n = positions.len();
            let graph = KingsSubgraph::new(positions);
            let weights = parse_vertex_weights(args, n)?;
            reject_nonunit_weights_for_one_variant(
                canonical,
                graph_type,
                resolved_variant,
                &weights,
            )?;
            Ok((
                ser_vertex_weight_problem_with(canonical, graph, weights)?,
                resolved_variant.clone(),
            ))
        }
        "TriangularSubgraph" => {
            let positions = parse_int_positions(args)?;
            let n = positions.len();
            let graph = TriangularSubgraph::new(positions);
            let weights = parse_vertex_weights(args, n)?;
            reject_nonunit_weights_for_one_variant(
                canonical,
                graph_type,
                resolved_variant,
                &weights,
            )?;
            Ok((
                ser_vertex_weight_problem_with(canonical, graph, weights)?,
                resolved_variant.clone(),
            ))
        }
        "UnitDiskGraph" => {
            let positions = parse_float_positions(args)?;
            let n = positions.len();
            let radius = args.radius.unwrap_or(1.0);
            let graph = UnitDiskGraph::new(positions, radius);
            let weights = parse_vertex_weights(args, n)?;
            reject_nonunit_weights_for_one_variant(
                canonical,
                graph_type,
                resolved_variant,
                &weights,
            )?;
            Ok((
                ser_vertex_weight_problem_with(canonical, graph, weights)?,
                resolved_variant.clone(),
            ))
        }
        _ => {
            // SimpleGraph path (existing)
            let (graph, n) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create {} --graph 0-1,1-2,2-3 [--weights 1,1,1,1]",
                    canonical
                )
            })?;
            let weights = parse_vertex_weights(args, n)?;
            reject_nonunit_weights_for_one_variant(
                canonical,
                graph_type,
                resolved_variant,
                &weights,
            )?;
            let data = ser_vertex_weight_problem_with(canonical, graph, weights)?;
            Ok((data, resolved_variant.clone()))
        }
    }
}

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
fn parse_couplings(args: &CreateArgs, num_edges: usize) -> Result<Vec<i32>> {
    match &args.couplings {
        Some(w) => {
            let vals: Vec<i32> = w
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if vals.len() != num_edges {
                bail!("Expected {} couplings but got {}", num_edges, vals.len());
            }
            Ok(vals)
        }
        None => Ok(vec![1i32; num_edges]),
    }
}

/// Parse `--fields` as SpinGlass on-site fields (i32), defaulting to all 0s.
fn parse_fields(args: &CreateArgs, num_vertices: usize) -> Result<Vec<i32>> {
    match &args.fields {
        Some(w) => {
            let vals: Vec<i32> = w
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if vals.len() != num_vertices {
                bail!("Expected {} fields but got {}", num_vertices, vals.len());
            }
            Ok(vals)
        }
        None => Ok(vec![0i32; num_vertices]),
    }
}

/// Check if a CLI string value contains float syntax (a decimal point).
fn has_float_syntax(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|s| s.contains('.'))
}

/// Parse `--couplings` as SpinGlass pairwise couplings (f64), defaulting to all 1.0.
fn parse_couplings_f64(args: &CreateArgs, num_edges: usize) -> Result<Vec<f64>> {
    match &args.couplings {
        Some(w) => {
            let vals: Vec<f64> = w
                .split(',')
                .map(|s| s.trim().parse::<f64>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if vals.len() != num_edges {
                bail!("Expected {} couplings but got {}", num_edges, vals.len());
            }
            Ok(vals)
        }
        None => Ok(vec![1.0f64; num_edges]),
    }
}

/// Parse `--fields` as SpinGlass on-site fields (f64), defaulting to all 0.0.
fn parse_fields_f64(args: &CreateArgs, num_vertices: usize) -> Result<Vec<f64>> {
    match &args.fields {
        Some(w) => {
            let vals: Vec<f64> = w
                .split(',')
                .map(|s| s.trim().parse::<f64>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if vals.len() != num_vertices {
                bail!("Expected {} fields but got {}", num_vertices, vals.len());
            }
            Ok(vals)
        }
        None => Ok(vec![0.0f64; num_vertices]),
    }
}

/// Parse `--clauses` as semicolon-separated clauses of comma-separated literals.
/// E.g., "1,2;-1,3;2,-3"
fn parse_clauses(args: &CreateArgs) -> Result<Vec<CNFClause>> {
    let clauses_str = args
        .clauses
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("SAT problems require --clauses (e.g., \"1,2;-1,3\")"))?;

    clauses_str
        .split(';')
        .map(|clause| {
            let literals: Vec<i32> = clause
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(CNFClause::new(literals))
        })
        .collect()
}

/// Parse `--sets` as semicolon-separated sets of comma-separated usize.
/// E.g., "0,1;1,2;0,2"
fn parse_sets(args: &CreateArgs) -> Result<Vec<Vec<usize>>> {
    parse_named_sets(args.sets.as_deref(), "--sets")
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
fn parse_deps(s: &str) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
    s.split(';')
        .map(|dep| {
            let parts: Vec<&str> = dep.split('>').collect();
            if parts.len() != 2 {
                bail!("Invalid dependency format '{}': expected 'lhs>rhs'", dep);
            }
            let lhs = parse_index_list(parts[0])?;
            let rhs = parse_index_list(parts[1])?;
            Ok((lhs, rhs))
        })
        .collect()
}

/// Parse a comma-separated list of usize indices.
fn parse_index_list(s: &str) -> Result<Vec<usize>> {
    s.split(',')
        .map(|x| {
            x.trim()
                .parse::<usize>()
                .map_err(|e| anyhow::anyhow!("Invalid index '{}': {}", x.trim(), e))
        })
        .collect()
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
    let raw_bound = args
        .bound
        .ok_or_else(|| anyhow::anyhow!("MultipleChoiceBranching requires --bound\n\n{usage}"))?;
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
fn parse_set_weights(args: &CreateArgs, num_sets: usize) -> Result<Vec<i32>> {
    parse_named_set_weights(args.weights.as_deref(), num_sets, "--weights")
}

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

fn parse_schedules(args: &CreateArgs, usage: &str) -> Result<Vec<Vec<bool>>> {
    let schedules_str = args
        .schedules
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("StaffScheduling requires --schedules\n\n{usage}"))?;
    parse_bool_rows(schedules_str)
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

fn parse_requirements(args: &CreateArgs, usage: &str) -> Result<Vec<u64>> {
    let requirements_str = args
        .requirements
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("StaffScheduling requires --requirements\n\n{usage}"))?;
    util::parse_comma_list(requirements_str)
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

fn validate_staff_scheduling_args(
    schedules: &[Vec<bool>],
    requirements: &[u64],
    shifts_per_schedule: usize,
    num_workers: u64,
    usage: &str,
) -> Result<()> {
    if num_workers >= usize::MAX as u64 {
        bail!(
            "StaffScheduling requires --num-workers to fit in usize for brute-force enumeration\n\n{usage}"
        );
    }

    let num_periods = requirements.len();
    for (index, schedule) in schedules.iter().enumerate() {
        if schedule.len() != num_periods {
            bail!(
                "schedule {} has {} periods, expected {}\n\n{}",
                index,
                schedule.len(),
                num_periods,
                usage
            );
        }
        let ones = schedule.iter().filter(|&&active| active).count();
        if ones != shifts_per_schedule {
            bail!(
                "schedule {} has {} active periods, expected {}\n\n{}",
                index,
                ones,
                shifts_per_schedule,
                usage
            );
        }
    }

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
fn parse_quantifiers(args: &CreateArgs, num_vars: usize) -> Result<Vec<Quantifier>> {
    let q_str = args
        .quantifiers
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("QBF requires --quantifiers (e.g., \"E,A,E\")"))?;

    let quantifiers: Vec<Quantifier> = q_str
        .split(',')
        .map(|s| match s.trim().to_lowercase().as_str() {
            "e" | "exists" => Ok(Quantifier::Exists),
            "a" | "forall" => Ok(Quantifier::ForAll),
            other => Err(anyhow::anyhow!(
                "Invalid quantifier '{}': expected E/Exists or A/ForAll",
                other
            )),
        })
        .collect::<Result<Vec<_>>>()?;

    if quantifiers.len() != num_vars {
        bail!(
            "Expected {} quantifiers but got {}",
            num_vars,
            quantifiers.len()
        );
    }
    Ok(quantifiers)
}

/// Parse a semicolon-separated matrix of i64 values.
/// E.g., "0,5;5,0"
fn parse_i64_matrix(s: &str) -> Result<Vec<Vec<i64>>> {
    let matrix: Vec<Vec<i64>> = s
        .split(';')
        .enumerate()
        .map(|(row_idx, row)| {
            row.trim()
                .split(',')
                .enumerate()
                .map(|(col_idx, v)| {
                    v.trim().parse::<i64>().map_err(|e| {
                        anyhow::anyhow!("Invalid value at row {row_idx}, col {col_idx}: {e}")
                    })
                })
                .collect()
        })
        .collect::<Result<_>>()?;
    if let Some(first_len) = matrix.first().map(|r| r.len()) {
        for (i, row) in matrix.iter().enumerate() {
            if row.len() != first_len {
                bail!(
                    "Ragged matrix: row {i} has {} columns, expected {first_len}",
                    row.len()
                );
            }
        }
    }
    Ok(matrix)
}

fn parse_potential_edges(args: &CreateArgs) -> Result<Vec<(usize, usize, i32)>> {
    let edges_str = args.potential_edges.as_deref().ok_or_else(|| {
        anyhow::anyhow!("BiconnectivityAugmentation requires --potential-edges (e.g., 0-2:3,1-3:5)")
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

/// Parse `--arc-costs` as per-arc costs (i32), defaulting to all 1s.
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
fn parse_candidate_arcs(
    args: &CreateArgs,
    num_vertices: usize,
) -> Result<Vec<(usize, usize, i32)>> {
    let arcs_str = args.candidate_arcs.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "StrongConnectivityAugmentation requires --candidate-arcs (e.g., \"2>0:1,2>1:3\")"
        )
    })?;

    arcs_str
        .split(',')
        .map(|entry| {
            let entry = entry.trim();
            let (arc_part, weight_part) = entry.split_once(':').ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid candidate arc '{}': expected format u>v:w (e.g., 2>0:1)",
                    entry
                )
            })?;
            let parts: Vec<&str> = arc_part.split('>').collect();
            if parts.len() != 2 {
                bail!(
                    "Invalid candidate arc '{}': expected format u>v:w (e.g., 2>0:1)",
                    entry
                );
            }

            let u: usize = parts[0].parse()?;
            let v: usize = parts[1].parse()?;
            anyhow::ensure!(
                u < num_vertices && v < num_vertices,
                "candidate arc ({}, {}) references vertex >= num_vertices ({})",
                u,
                v,
                num_vertices
            );

            let w: i32 = weight_part.parse()?;
            Ok((u, v, w))
        })
        .collect()
}

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
             MaximumMatching, MinimumDominatingSet, SpinGlass, KColoring, KClique, TravelingSalesman, \
             BottleneckTravelingSalesman, SteinerTreeInGraphs, HamiltonianCircuit, SteinerTree, \
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use clap::Parser;

    use super::help_flag_hint;
    use super::help_flag_name;
    use super::parse_bool_rows;
    use super::*;
    use super::{ensure_attribute_indices_in_range, problem_help_flag_name};
    use crate::cli::{Cli, Commands};
    use crate::output::OutputConfig;

    fn temp_output_path(name: &str) -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{}_{}.json", name, suffix))
    }

    #[test]
    fn test_problem_help_uses_bound_for_length_bounded_disjoint_paths() {
        assert_eq!(
            problem_help_flag_name("LengthBoundedDisjointPaths", "max_length", "usize", false),
            "bound"
        );
    }

    #[test]
    fn test_problem_help_preserves_generic_field_kebab_case() {
        assert_eq!(
            problem_help_flag_name("LengthBoundedDisjointPaths", "max_paths", "usize", false,),
            "max-paths"
        );
    }

    #[test]
    fn test_help_flag_name_mentions_m_alias_for_scheduling_processors() {
        assert_eq!(
            help_flag_name("SchedulingWithIndividualDeadlines", "num_processors"),
            "num-processors/--m"
        );
        assert_eq!(
            help_flag_name("FlowShopScheduling", "num_processors"),
            "num-processors/--m"
        );
    }

    #[test]
    fn test_ensure_attribute_indices_in_range_rejects_out_of_range_index() {
        let err = ensure_attribute_indices_in_range(&[0, 4], 3, "Functional dependency '0:4' rhs")
            .unwrap_err();
        assert!(
            err.to_string().contains("out of range"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_create_scheduling_with_individual_deadlines_accepts_m_alias() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "SchedulingWithIndividualDeadlines",
            "--n",
            "3",
            "--deadlines",
            "1,1,2",
            "--m",
            "2",
        ])
        .expect("parse create command");

        let Commands::Create(args) = cli.command else {
            panic!("expected create subcommand");
        };

        let out = OutputConfig {
            output: Some(
                std::env::temp_dir()
                    .join("pred_test_create_scheduling_with_individual_deadlines_m_alias.json"),
            ),
            quiet: true,
            json: false,
            auto_json: false,
        };
        create(&args, &out).expect("`--m` should satisfy --num-processors alias");

        let created: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(out.output.as_ref().unwrap()).unwrap())
                .unwrap();
        std::fs::remove_file(out.output.as_ref().unwrap()).ok();

        assert_eq!(created["type"], "SchedulingWithIndividualDeadlines");
        assert_eq!(created["data"]["num_processors"], 2);
    }

    #[test]
    fn test_problem_help_uses_prime_attribute_name_cli_overrides() {
        assert_eq!(
            problem_help_flag_name("PrimeAttributeName", "num_attributes", "usize", false),
            "universe"
        );
        assert_eq!(
            problem_help_flag_name(
                "PrimeAttributeName",
                "dependencies",
                "Vec<(Vec<usize>, Vec<usize>)>",
                false,
            ),
            "deps"
        );
        assert_eq!(
            problem_help_flag_name("PrimeAttributeName", "query_attribute", "usize", false),
            "query"
        );
    }

    #[test]
    fn test_problem_help_uses_problem_specific_lcs_strings_hint() {
        assert_eq!(
            help_flag_hint(
                "LongestCommonSubsequence",
                "strings",
                "Vec<Vec<usize>>",
                None,
            ),
            "raw strings: \"ABAC;BACA\" or symbol lists: \"0,1,0;1,0,1\""
        );
    }

    #[test]
    fn test_problem_help_uses_string_to_string_correction_cli_flags() {
        assert_eq!(
            problem_help_flag_name("StringToStringCorrection", "source", "Vec<usize>", false),
            "source-string"
        );
        assert_eq!(
            problem_help_flag_name("StringToStringCorrection", "target", "Vec<usize>", false),
            "target-string"
        );
        assert_eq!(
            problem_help_flag_name("StringToStringCorrection", "bound", "usize", false),
            "bound"
        );
    }

    #[test]
    fn test_problem_help_keeps_generic_vec_vec_usize_hint_for_other_models() {
        assert_eq!(
            help_flag_hint("SetBasis", "sets", "Vec<Vec<usize>>", None),
            "semicolon-separated sets: \"0,1;1,2;0,2\""
        );
    }

    #[test]
    fn test_problem_help_uses_k_for_staff_scheduling() {
        assert_eq!(
            help_flag_name("StaffScheduling", "shifts_per_schedule"),
            "k"
        );
        assert_eq!(
            problem_help_flag_name("StaffScheduling", "shifts_per_schedule", "usize", false),
            "k"
        );
    }

    #[test]
    fn test_parse_bool_rows_reports_generic_invalid_boolean_entry() {
        let err = parse_bool_rows("1,maybe").unwrap_err().to_string();
        assert_eq!(
            err,
            "Invalid boolean entry 'maybe': expected 0/1 or true/false"
        );
    }

    #[test]
    fn test_create_staff_scheduling_outputs_problem_json() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "StaffScheduling",
            "--schedules",
            "1,1,1,1,1,0,0;0,1,1,1,1,1,0;0,0,1,1,1,1,1;1,0,0,1,1,1,1;1,1,0,0,1,1,1",
            "--requirements",
            "2,2,2,3,3,2,1",
            "--num-workers",
            "4",
            "--k",
            "5",
        ])
        .unwrap();

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output_path =
            std::env::temp_dir().join(format!("staff-scheduling-create-{suffix}.json"));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&output_path).unwrap()).unwrap();
        assert_eq!(json["type"], "StaffScheduling");
        assert_eq!(json["data"]["num_workers"], 4);
        assert_eq!(
            json["data"]["requirements"],
            serde_json::json!([2, 2, 2, 3, 3, 2, 1])
        );
        std::fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_create_path_constrained_network_flow_outputs_problem_json() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "PathConstrainedNetworkFlow",
            "--arcs",
            "0>1,0>2,1>3,1>4,2>4,3>5,4>5,4>6,5>7,6>7",
            "--capacities",
            "2,1,1,1,1,1,1,1,2,1",
            "--source",
            "0",
            "--sink",
            "7",
            "--paths",
            "0,2,5,8;0,3,6,8;0,3,7,9;1,4,6,8;1,4,7,9",
            "--requirement",
            "3",
        ])
        .expect("parse create command");

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let output_path = temp_output_path("path_constrained_network_flow");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).expect("create PathConstrainedNetworkFlow JSON");

        let created: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output_path).unwrap()).unwrap();
        fs::remove_file(output_path).ok();

        assert_eq!(created["type"], "PathConstrainedNetworkFlow");
        assert_eq!(created["data"]["source"], 0);
        assert_eq!(created["data"]["sink"], 7);
        assert_eq!(created["data"]["requirement"], 3);
        assert_eq!(created["data"]["paths"][0], serde_json::json!([0, 2, 5, 8]));
    }

    #[test]
    fn test_create_path_constrained_network_flow_rejects_invalid_paths() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "PathConstrainedNetworkFlow",
            "--arcs",
            "0>1,1>2,2>3",
            "--capacities",
            "1,1,1",
            "--source",
            "0",
            "--sink",
            "3",
            "--paths",
            "0,3",
            "--requirement",
            "1",
        ])
        .expect("parse create command");

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("out of bounds") || err.contains("not contiguous"));
    }

    #[test]
    fn test_create_staff_scheduling_reports_invalid_schedule_without_panic() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "StaffScheduling",
            "--schedules",
            "1,1,1,1,1,0,0;0,1,1,1,1,1",
            "--requirements",
            "2,2,2,3,3,2,1",
            "--num-workers",
            "4",
            "--k",
            "5",
        ])
        .unwrap();

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let result = std::panic::catch_unwind(|| create(&args, &out));
        assert!(result.is_ok(), "create should return an error, not panic");
        let err = result.unwrap().unwrap_err().to_string();
        // parse_bool_rows catches ragged rows before validate_staff_scheduling_args
        assert!(
            err.contains("All rows") || err.contains("schedule 1 has 6 periods, expected 7"),
            "expected row-length validation error, got: {err}"
        );
    }

    #[test]
    fn test_problem_help_uses_num_tasks_for_timetable_design() {
        assert_eq!(
            problem_help_flag_name("TimetableDesign", "num_tasks", "usize", false),
            "num-tasks"
        );
        assert_eq!(
            help_flag_hint("TimetableDesign", "craftsman_avail", "Vec<Vec<bool>>", None),
            "semicolon-separated 0/1 rows: \"1,1,0;0,1,1\""
        );
    }

    #[test]
    fn test_example_for_path_constrained_network_flow_mentions_paths_flag() {
        let example = example_for("PathConstrainedNetworkFlow", None);
        assert!(example.contains("--paths"));
        assert!(example.contains("--requirement"));
    }

    #[test]
    fn test_example_for_three_partition_mentions_sizes_and_bound() {
        let example = example_for("ThreePartition", None);
        assert!(example.contains("--sizes"));
        assert!(example.contains("--bound"));
    }

    #[test]
    fn test_create_three_partition_outputs_problem_json() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "ThreePartition",
            "--sizes",
            "4,5,6,4,6,5",
            "--bound",
            "15",
        ])
        .expect("parse create command");

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let output_path = temp_output_path("three_partition_create");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).expect("create ThreePartition JSON");

        let created: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output_path).unwrap()).unwrap();
        fs::remove_file(output_path).ok();

        assert_eq!(created["type"], "ThreePartition");
        assert_eq!(
            created["data"]["sizes"],
            serde_json::json!([4, 5, 6, 4, 6, 5])
        );
        assert_eq!(created["data"]["bound"], 15);
    }

    #[test]
    fn test_create_three_partition_requires_bound() {
        let cli =
            Cli::try_parse_from(["pred", "create", "ThreePartition", "--sizes", "4,5,6,4,6,5"])
                .expect("parse create command");

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("ThreePartition requires --bound"));
    }

    #[test]
    fn test_create_three_partition_rejects_invalid_instance() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "ThreePartition",
            "--sizes",
            "4,5,6,4,6,5",
            "--bound",
            "14",
        ])
        .expect("parse create command");

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("must equal m * bound"));
    }

    #[test]
    fn test_create_timetable_design_outputs_problem_json() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "TimetableDesign",
            "--num-periods",
            "3",
            "--num-craftsmen",
            "5",
            "--num-tasks",
            "5",
            "--craftsman-avail",
            "1,1,1;1,1,0;0,1,1;1,0,1;1,1,1",
            "--task-avail",
            "1,1,0;0,1,1;1,0,1;1,1,1;1,1,1",
            "--requirements",
            "1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0",
        ])
        .unwrap();

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let output_path =
            std::env::temp_dir().join(format!("timetable-design-create-{suffix}.json"));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&output_path).unwrap()).unwrap();
        assert_eq!(json["type"], "TimetableDesign");
        assert_eq!(json["data"]["num_periods"], 3);
        assert_eq!(json["data"]["num_craftsmen"], 5);
        assert_eq!(json["data"]["num_tasks"], 5);
        assert_eq!(
            json["data"]["craftsman_avail"],
            serde_json::json!([
                [true, true, true],
                [true, true, false],
                [false, true, true],
                [true, false, true],
                [true, true, true]
            ])
        );
        assert_eq!(
            json["data"]["task_avail"],
            serde_json::json!([
                [true, true, false],
                [false, true, true],
                [true, false, true],
                [true, true, true],
                [true, true, true]
            ])
        );
        assert_eq!(
            json["data"]["requirements"],
            serde_json::json!([
                [1, 0, 1, 0, 0],
                [0, 1, 0, 0, 1],
                [0, 0, 0, 1, 0],
                [0, 0, 0, 0, 1],
                [0, 1, 0, 0, 0]
            ])
        );
        std::fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_create_timetable_design_reports_invalid_matrix_without_panic() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "TimetableDesign",
            "--num-periods",
            "3",
            "--num-craftsmen",
            "5",
            "--num-tasks",
            "5",
            "--craftsman-avail",
            "1,1,1;1,1",
            "--task-avail",
            "1,1,0;0,1,1;1,0,1;1,1,1;1,1,1",
            "--requirements",
            "1,0,1,0,0;0,1,0,0,1;0,0,0,1,0;0,0,0,0,1;0,1,0,0,0",
        ])
        .unwrap();

        let args = match cli.command {
            Commands::Create(args) => args,
            _ => panic!("expected create command"),
        };

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let result = std::panic::catch_unwind(|| create(&args, &out));
        assert!(result.is_ok(), "create should return an error, not panic");
        let err = result.unwrap().unwrap_err().to_string();
        assert!(
            err.contains("--craftsman-avail"),
            "expected timetable matrix validation error, got: {err}"
        );
        assert!(err.contains("Usage: pred create TimetableDesign"));
    }

    #[test]
    fn test_create_generalized_hex_serializes_problem_json() {
        let output = temp_output_path("generalized_hex_create");
        let cli = Cli::try_parse_from([
            "pred",
            "-o",
            output.to_str().unwrap(),
            "create",
            "GeneralizedHex",
            "--graph",
            "0-1,0-2,0-3,1-4,2-4,3-4,4-5",
            "--source",
            "0",
            "--sink",
            "5",
        ])
        .unwrap();
        let out = OutputConfig {
            output: cli.output.clone(),
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        fs::remove_file(&output).unwrap();
        assert_eq!(json["type"], "GeneralizedHex");
        assert_eq!(json["variant"]["graph"], "SimpleGraph");
        assert_eq!(json["data"]["source"], 0);
        assert_eq!(json["data"]["target"], 5);
    }

    #[test]
    fn test_create_generalized_hex_requires_sink() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "GeneralizedHex",
            "--graph",
            "0-1,1-2,2-3",
            "--source",
            "0",
        ])
        .unwrap();
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err();
        assert!(err.to_string().contains("GeneralizedHex requires --sink"));
    }

    #[test]
    fn test_create_capacity_assignment_serializes_problem_json() {
        let output = temp_output_path("capacity_assignment_create");
        let cli = Cli::try_parse_from([
            "pred",
            "-o",
            output.to_str().unwrap(),
            "create",
            "CapacityAssignment",
            "--capacities",
            "1,2,3",
            "--cost-matrix",
            "1,3,6;2,4,7;1,2,5",
            "--delay-matrix",
            "8,4,1;7,3,1;6,3,1",
            "--delay-budget",
            "12",
        ])
        .expect("parse create command");
        let out = OutputConfig {
            output: cli.output.clone(),
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        fs::remove_file(&output).unwrap();
        assert_eq!(json["type"], "CapacityAssignment");
        assert_eq!(json["data"]["capacities"], serde_json::json!([1, 2, 3]));
        assert_eq!(json["data"]["delay_budget"], 12);
    }

    #[test]
    fn test_create_production_planning_serializes_problem_json() {
        let output = temp_output_path("production_planning_create");
        let cli = Cli::try_parse_from([
            "pred",
            "-o",
            output.to_str().unwrap(),
            "create",
            "ProductionPlanning",
            "--num-periods",
            "6",
            "--demands",
            "5,3,7,2,8,5",
            "--capacities",
            "12,12,12,12,12,12",
            "--setup-costs",
            "10,10,10,10,10,10",
            "--production-costs",
            "1,1,1,1,1,1",
            "--inventory-costs",
            "1,1,1,1,1,1",
            "--cost-bound",
            "80",
        ])
        .expect("parse create command");
        let out = OutputConfig {
            output: cli.output.clone(),
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        fs::remove_file(&output).unwrap();
        assert_eq!(json["type"], "ProductionPlanning");
        assert_eq!(json["data"]["num_periods"], 6);
        assert_eq!(
            json["data"]["demands"],
            serde_json::json!([5, 3, 7, 2, 8, 5])
        );
        assert_eq!(
            json["data"]["capacities"],
            serde_json::json!([12, 12, 12, 12, 12, 12])
        );
        assert_eq!(
            json["data"]["setup_costs"],
            serde_json::json!([10, 10, 10, 10, 10, 10])
        );
        assert_eq!(
            json["data"]["production_costs"],
            serde_json::json!([1, 1, 1, 1, 1, 1])
        );
        assert_eq!(
            json["data"]["inventory_costs"],
            serde_json::json!([1, 1, 1, 1, 1, 1])
        );
        assert_eq!(json["data"]["cost_bound"], 80);
    }

    #[test]
    fn test_create_production_planning_requires_all_period_vectors() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "ProductionPlanning",
            "--num-periods",
            "6",
            "--demands",
            "5,3,7,2,8,5",
            "--capacities",
            "12,12,12,12,12,12",
            "--setup-costs",
            "10,10,10,10,10,10",
            "--inventory-costs",
            "1,1,1,1,1,1",
            "--cost-bound",
            "80",
        ])
        .expect("parse create command");
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err();
        assert!(err
            .to_string()
            .contains("ProductionPlanning requires --production-costs"));
    }

    #[test]
    fn test_create_production_planning_rejects_mismatched_period_lengths() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "ProductionPlanning",
            "--num-periods",
            "6",
            "--demands",
            "5,3,7,2,8",
            "--capacities",
            "12,12,12,12,12,12",
            "--setup-costs",
            "10,10,10,10,10,10",
            "--production-costs",
            "1,1,1,1,1,1",
            "--inventory-costs",
            "1,1,1,1,1,1",
            "--cost-bound",
            "80",
        ])
        .expect("parse create command");
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err();
        assert!(err
            .to_string()
            .contains("--demands must contain exactly 6 entries"));
    }

    #[test]
    fn test_create_example_production_planning_uses_canonical_example() {
        let output = temp_output_path("production_planning_example_create");
        let cli = Cli::try_parse_from([
            "pred",
            "-o",
            output.to_str().unwrap(),
            "create",
            "--example",
            "ProductionPlanning",
        ])
        .expect("parse create command");
        let out = OutputConfig {
            output: cli.output.clone(),
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        fs::remove_file(&output).unwrap();
        assert_eq!(json["type"], "ProductionPlanning");
        assert_eq!(json["data"]["num_periods"], 4);
        assert_eq!(json["data"]["demands"], serde_json::json!([2, 1, 3, 2]));
        assert_eq!(json["data"]["cost_bound"], 16);
    }

    #[test]
    fn test_create_longest_path_serializes_problem_json() {
        let output = temp_output_path("longest_path_create");
        let cli = Cli::try_parse_from([
            "pred",
            "-o",
            output.to_str().unwrap(),
            "create",
            "LongestPath",
            "--graph",
            "0-1,0-2,1-3,2-3,2-4,3-5,4-5,4-6,5-6,1-6",
            "--edge-lengths",
            "3,2,4,1,5,2,3,2,4,1",
            "--source-vertex",
            "0",
            "--target-vertex",
            "6",
        ])
        .unwrap();
        let out = OutputConfig {
            output: cli.output.clone(),
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        fs::remove_file(&output).unwrap();
        assert_eq!(json["type"], "LongestPath");
        assert_eq!(json["variant"]["graph"], "SimpleGraph");
        assert_eq!(json["variant"]["weight"], "i32");
        assert_eq!(json["data"]["source_vertex"], 0);
        assert_eq!(json["data"]["target_vertex"], 6);
        assert_eq!(
            json["data"]["edge_lengths"],
            serde_json::json!([3, 2, 4, 1, 5, 2, 3, 2, 4, 1])
        );
    }

    #[test]
    fn test_create_undirected_flow_lower_bounds_serializes_problem_json() {
        let output = temp_output_path("undirected_flow_lower_bounds_create");
        let cli = Cli::try_parse_from([
            "pred",
            "-o",
            output.to_str().unwrap(),
            "create",
            "UndirectedFlowLowerBounds",
            "--graph",
            "0-1,0-2,1-3,2-3,1-4,3-5,4-5",
            "--capacities",
            "2,2,2,2,1,3,2",
            "--lower-bounds",
            "1,1,0,0,1,0,1",
            "--source",
            "0",
            "--sink",
            "5",
            "--requirement",
            "3",
        ])
        .unwrap();
        let out = OutputConfig {
            output: cli.output.clone(),
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        create(&args, &out).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        fs::remove_file(&output).unwrap();
        assert_eq!(json["type"], "UndirectedFlowLowerBounds");
        assert_eq!(json["data"]["source"], 0);
        assert_eq!(json["data"]["sink"], 5);
        assert_eq!(json["data"]["requirement"], 3);
        assert_eq!(
            json["data"]["lower_bounds"],
            serde_json::json!([1, 1, 0, 0, 1, 0, 1])
        );
    }

    #[test]
    fn test_create_capacity_assignment_rejects_non_monotone_cost_row() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "CapacityAssignment",
            "--capacities",
            "1,2,3",
            "--cost-matrix",
            "1,3,2;2,4,7;1,2,5",
            "--delay-matrix",
            "8,4,1;7,3,1;6,3,1",
            "--delay-budget",
            "12",
        ])
        .expect("parse create command");
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("cost row 0"));
        assert!(err.contains("non-decreasing"));
    }

    #[test]
    fn test_create_capacity_assignment_rejects_matrix_width_mismatch() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "CapacityAssignment",
            "--capacities",
            "1,2,3",
            "--cost-matrix",
            "1,3;2,4,7;1,2,5",
            "--delay-matrix",
            "8,4,1;7,3,1;6,3,1",
            "--delay-budget",
            "12",
        ])
        .expect("parse create command");
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("cost row 0"));
        assert!(err.contains("capacities length"));
    }

    #[test]
    fn test_create_longest_path_requires_edge_lengths() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "LongestPath",
            "--graph",
            "0-1,1-2",
            "--source-vertex",
            "0",
            "--target-vertex",
            "2",
        ])
        .unwrap();
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err();
        assert!(err
            .to_string()
            .contains("LongestPath requires --edge-lengths"));
    }

    #[test]
    fn test_create_longest_path_rejects_weights_flag() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "LongestPath",
            "--graph",
            "0-1,1-2",
            "--weights",
            "1,1,1",
            "--source-vertex",
            "0",
            "--target-vertex",
            "2",
            "--edge-lengths",
            "5,7",
        ])
        .unwrap();
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err();
        assert!(err
            .to_string()
            .contains("LongestPath uses --edge-lengths, not --weights"));
    }

    #[test]
    fn test_create_undirected_flow_lower_bounds_requires_lower_bounds() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "UndirectedFlowLowerBounds",
            "--graph",
            "0-1,0-2,1-3,2-3,1-4,3-5,4-5",
            "--capacities",
            "2,2,2,2,1,3,2",
            "--source",
            "0",
            "--sink",
            "5",
            "--requirement",
            "3",
        ])
        .unwrap();
        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let err = create(&args, &out).unwrap_err();
        assert!(err
            .to_string()
            .contains("UndirectedFlowLowerBounds requires --lower-bounds"));
    }

    fn empty_args() -> CreateArgs {
        CreateArgs {
            problem: Some("BiconnectivityAugmentation".to_string()),
            example: None,
            example_target: None,
            example_side: crate::cli::ExampleSide::Source,
            graph: None,
            weights: None,
            edge_weights: None,
            edge_lengths: None,
            capacities: None,
            demands: None,
            setup_costs: None,
            production_costs: None,
            inventory_costs: None,
            bundle_capacities: None,
            cost_matrix: None,
            delay_matrix: None,
            lower_bounds: None,
            multipliers: None,
            source: None,
            sink: None,
            requirement: None,
            num_paths_required: None,
            paths: None,
            couplings: None,
            fields: None,
            clauses: None,
            num_vars: None,
            matrix: None,
            k: None,
            random: false,
            source_vertex: None,
            target_vertex: None,
            num_vertices: None,
            edge_prob: None,
            seed: None,
            target: None,
            m: None,
            n: None,
            positions: None,
            radius: None,
            source_1: None,
            sink_1: None,
            source_2: None,
            sink_2: None,
            requirement_1: None,
            requirement_2: None,
            sizes: None,
            probabilities: None,
            capacity: None,
            sequence: None,
            sets: None,
            r_sets: None,
            s_sets: None,
            r_weights: None,
            s_weights: None,
            partition: None,
            bundles: None,
            universe: None,
            biedges: None,
            left: None,
            right: None,
            rank: None,
            basis: None,
            target_vec: None,
            bounds: None,
            release_times: None,
            lengths: None,
            terminals: None,
            terminal_pairs: None,
            tree: None,
            required_edges: None,
            bound: None,
            latency_bound: None,
            length_bound: None,
            weight_bound: None,
            cost_bound: None,
            delay_budget: None,
            pattern: None,
            strings: None,
            string: None,
            arc_costs: None,
            arcs: None,
            values: None,
            precedences: None,
            distance_matrix: None,
            potential_edges: None,
            budget: None,
            max_cycle_length: None,
            candidate_arcs: None,
            deadlines: None,
            precedence_pairs: None,
            task_lengths: None,
            job_tasks: None,
            resource_bounds: None,
            resource_requirements: None,
            deadline: None,
            num_processors: None,
            alphabet_size: None,
            deps: None,
            query: None,
            dependencies: None,
            num_attributes: None,
            source_string: None,
            target_string: None,
            schedules: None,
            requirements: None,
            num_workers: None,
            num_periods: None,
            num_craftsmen: None,
            num_tasks: None,
            craftsman_avail: None,
            task_avail: None,
            num_groups: None,
            num_sectors: None,
            domain_size: None,
            relations: None,
            conjuncts_spec: None,
            relation_attrs: None,
            known_keys: None,
            num_objects: None,
            attribute_domains: None,
            frequency_tables: None,
            known_values: None,
            costs: None,
            cut_bound: None,
            size_bound: None,
            usage: None,
            storage: None,
            quantifiers: None,
            homologous_pairs: None,
            coeff_a: None,
            coeff_b: None,
            rhs: None,
            qde_rhs: None,
            required_columns: None,
        }
    }

    #[test]
    fn test_all_data_flags_empty_treats_potential_edges_as_input() {
        let mut args = empty_args();
        args.potential_edges = Some("0-2:3,1-3:5".to_string());
        assert!(!all_data_flags_empty(&args));
    }

    #[test]
    fn test_all_data_flags_empty_treats_budget_as_input() {
        let mut args = empty_args();
        args.budget = Some("7".to_string());
        assert!(!all_data_flags_empty(&args));
    }

    #[test]
    fn test_all_data_flags_empty_treats_max_cycle_length_as_input() {
        let mut args = empty_args();
        args.max_cycle_length = Some(4);
        assert!(!all_data_flags_empty(&args));
    }

    #[test]
    fn test_all_data_flags_empty_treats_homologous_pairs_as_input() {
        let mut args = empty_args();
        args.homologous_pairs = Some("2=5;4=3".to_string());
        assert!(!all_data_flags_empty(&args));
    }

    #[test]
    fn test_all_data_flags_empty_treats_job_tasks_as_input() {
        let mut args = empty_args();
        args.job_tasks = Some("0:1,1:1;1:1,0:1".to_string());
        assert!(!all_data_flags_empty(&args));
    }

    #[test]
    fn test_parse_potential_edges() {
        let mut args = empty_args();
        args.potential_edges = Some("0-2:3,1-3:5".to_string());

        let potential_edges = parse_potential_edges(&args).unwrap();

        assert_eq!(potential_edges, vec![(0, 2, 3), (1, 3, 5)]);
    }

    #[test]
    fn test_parse_potential_edges_rejects_missing_weight() {
        let mut args = empty_args();
        args.potential_edges = Some("0-2,1-3:5".to_string());

        let err = parse_potential_edges(&args).unwrap_err().to_string();

        assert!(err.contains("u-v:w"));
    }

    #[test]
    fn test_parse_budget() {
        let mut args = empty_args();
        args.budget = Some("7".to_string());

        assert_eq!(parse_budget(&args).unwrap(), 7);
    }

    #[test]
    fn test_create_disjoint_connecting_paths_json() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::graph::DisjointConnectingPaths;

        let mut args = empty_args();
        args.problem = Some("DisjointConnectingPaths".to_string());
        args.graph = Some("0-1,1-3,0-2,1-4,2-4,3-5,4-5".to_string());
        args.terminal_pairs = Some("0-3,2-5".to_string());

        let output_path =
            std::env::temp_dir().join(format!("dcp-create-{}.json", std::process::id()));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = std::fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "DisjointConnectingPaths");
        assert_eq!(
            created.variant,
            BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())])
        );

        let problem: DisjointConnectingPaths<SimpleGraph> =
            serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.num_vertices(), 6);
        assert_eq!(problem.num_edges(), 7);
        assert_eq!(problem.terminal_pairs(), &[(0, 3), (2, 5)]);

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_create_disjoint_connecting_paths_rejects_overlapping_terminal_pairs() {
        let mut args = empty_args();
        args.problem = Some("DisjointConnectingPaths".to_string());
        args.graph = Some("0-1,1-2,2-3,3-4".to_string());
        args.terminal_pairs = Some("0-2,2-4".to_string());

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("pairwise disjoint"));
    }

    #[test]
    fn test_parse_homologous_pairs() {
        let mut args = empty_args();
        args.homologous_pairs = Some("2=5;4=3".to_string());

        assert_eq!(parse_homologous_pairs(&args).unwrap(), vec![(2, 5), (4, 3)]);
    }

    #[test]
    fn test_parse_homologous_pairs_rejects_invalid_token() {
        let mut args = empty_args();
        args.homologous_pairs = Some("2-5".to_string());

        let err = parse_homologous_pairs(&args).unwrap_err().to_string();

        assert!(err.contains("u=v"));
    }

    #[test]
    fn test_parse_graph_respects_explicit_num_vertices() {
        let mut args = empty_args();
        args.graph = Some("0-1".to_string());
        args.num_vertices = Some(3);

        let (graph, num_vertices) = parse_graph(&args).unwrap();

        assert_eq!(num_vertices, 3);
        assert_eq!(graph.num_vertices(), 3);
        assert_eq!(graph.edges(), vec![(0, 1)]);
    }

    #[test]
    fn test_validate_potential_edges_rejects_existing_graph_edge() {
        let err = validate_potential_edges(&SimpleGraph::path(3), &[(0, 1, 5)])
            .unwrap_err()
            .to_string();

        assert!(err.contains("already exists in the graph"));
    }

    #[test]
    fn test_validate_potential_edges_rejects_duplicate_edges() {
        let err = validate_potential_edges(&SimpleGraph::path(4), &[(0, 3, 1), (3, 0, 2)])
            .unwrap_err()
            .to_string();

        assert!(err.contains("Duplicate potential edge"));
    }

    #[test]
    fn test_create_biconnectivity_augmentation_json() {
        let mut args = empty_args();
        args.graph = Some("0-1,1-2,2-3".to_string());
        args.potential_edges = Some("0-2:3,0-3:4,1-3:2".to_string());
        args.budget = Some("5".to_string());

        let output_path = std::env::temp_dir().join("pred_test_create_biconnectivity.json");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["type"], "BiconnectivityAugmentation");
        assert_eq!(json["data"]["budget"], 5);
        assert_eq!(
            json["data"]["potential_weights"][0],
            serde_json::json!([0, 2, 3])
        );

        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_create_biconnectivity_augmentation_json_with_isolated_vertices() {
        let mut args = empty_args();
        args.graph = Some("0-1".to_string());
        args.num_vertices = Some(3);
        args.potential_edges = Some("1-2:1".to_string());
        args.budget = Some("1".to_string());

        let output_path =
            std::env::temp_dir().join("pred_test_create_biconnectivity_isolated.json");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        let problem: BiconnectivityAugmentation<SimpleGraph, i32> =
            serde_json::from_value(json["data"].clone()).unwrap();

        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.potential_weights(), &[(1, 2, 1)]);
        assert_eq!(problem.budget(), &1);

        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_create_partial_feedback_edge_set_json() {
        use problemreductions::models::graph::PartialFeedbackEdgeSet;

        let mut args = empty_args();
        args.problem = Some("PartialFeedbackEdgeSet".to_string());
        args.graph = Some("0-1,1-2,2-0".to_string());
        args.budget = Some("1".to_string());
        args.max_cycle_length = Some(3);

        let output_path =
            std::env::temp_dir().join("pred_test_create_partial_feedback_edge_set.json");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["type"], "PartialFeedbackEdgeSet");
        assert_eq!(json["data"]["budget"], 1);
        assert_eq!(json["data"]["max_cycle_length"], 3);

        let problem: PartialFeedbackEdgeSet<SimpleGraph> =
            serde_json::from_value(json["data"].clone()).unwrap();
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.num_edges(), 3);
        assert_eq!(problem.budget(), 1);
        assert_eq!(problem.max_cycle_length(), 3);

        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_create_partial_feedback_edge_set_requires_max_cycle_length() {
        let mut args = empty_args();
        args.problem = Some("PartialFeedbackEdgeSet".to_string());
        args.graph = Some("0-1,1-2,2-0".to_string());
        args.budget = Some("1".to_string());

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("PartialFeedbackEdgeSet requires --max-cycle-length"));
    }

    #[test]
    fn test_create_ensemble_computation_json() {
        let mut args = empty_args();
        args.problem = Some("EnsembleComputation".to_string());
        args.universe = Some(4);
        args.sets = Some("0,1,2;0,1,3".to_string());
        args.budget = Some("4".to_string());

        let output_path = std::env::temp_dir().join("pred_test_create_ensemble_computation.json");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["type"], "EnsembleComputation");
        assert_eq!(json["data"]["universe_size"], 4);
        assert_eq!(
            json["data"]["subsets"],
            serde_json::json!([[0, 1, 2], [0, 1, 3]])
        );
        assert_eq!(json["data"]["budget"], 4);

        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_create_expected_retrieval_cost_json() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::misc::ExpectedRetrievalCost;

        let mut args = empty_args();
        args.problem = Some("ExpectedRetrievalCost".to_string());
        args.probabilities = Some("0.2,0.15,0.15,0.2,0.1,0.2".to_string());
        args.num_sectors = Some(3);

        let output_path = std::env::temp_dir().join(format!(
            "expected-retrieval-cost-{}.json",
            std::process::id()
        ));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = std::fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "ExpectedRetrievalCost");

        let problem: ExpectedRetrievalCost = serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.num_records(), 6);
        assert_eq!(problem.num_sectors(), 3);
        use problemreductions::types::Min;
        assert!(matches!(
            problem.evaluate(&[0, 1, 2, 1, 0, 2]),
            Min(Some(_))
        ));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_create_job_shop_scheduling_json() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::misc::JobShopScheduling;
        use problemreductions::traits::Problem;
        use problemreductions::types::Min;

        let mut args = empty_args();
        args.problem = Some("JobShopScheduling".to_string());
        args.job_tasks = Some("0:3,1:4;1:2,0:3,1:2;0:4,1:3;1:5,0:2;0:2,1:3,0:1".to_string());

        let output_path =
            std::env::temp_dir().join(format!("job-shop-scheduling-{}.json", std::process::id()));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = std::fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "JobShopScheduling");
        assert!(created.variant.is_empty());

        let problem: JobShopScheduling = serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.num_processors(), 2);
        assert_eq!(problem.num_jobs(), 5);
        assert_eq!(
            problem.evaluate(&[0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0]),
            Min(Some(19))
        );

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_create_job_shop_scheduling_requires_job_tasks() {
        let mut args = empty_args();
        args.problem = Some("JobShopScheduling".to_string());
        args.num_processors = Some(2);

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("JobShopScheduling requires --job-tasks"));
    }

    #[test]
    fn test_create_job_shop_scheduling_rejects_malformed_operation() {
        let mut args = empty_args();
        args.problem = Some("JobShopScheduling".to_string());
        args.job_tasks = Some("0-3,1:4".to_string());

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("expected 'processor:length'"));
    }

    #[test]
    fn test_create_job_shop_scheduling_rejects_consecutive_same_processor() {
        let mut args = empty_args();
        args.problem = Some("JobShopScheduling".to_string());
        args.job_tasks = Some("0:1,0:1".to_string());

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("must use different processors"));
    }

    #[test]
    fn test_create_rooted_tree_storage_assignment_json() {
        let mut args = empty_args();
        args.problem = Some("RootedTreeStorageAssignment".to_string());
        args.universe = Some(5);
        args.sets = Some("0,2;1,3;0,4;2,4".to_string());
        args.bound = Some(1);

        let output_path =
            std::env::temp_dir().join("pred_test_create_rooted_tree_storage_assignment.json");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["type"], "RootedTreeStorageAssignment");
        assert_eq!(json["data"]["universe_size"], 5);
        assert_eq!(
            json["data"]["subsets"],
            serde_json::json!([[0, 2], [1, 3], [0, 4], [2, 4]])
        );
        assert_eq!(json["data"]["bound"], 1);

        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_create_stacker_crane_json() {
        let mut args = empty_args();
        args.problem = Some("StackerCrane".to_string());
        args.num_vertices = Some(6);
        args.arcs = Some("0>4,2>5,5>1,3>0,4>3".to_string());
        args.graph = Some("0-1,1-2,2-3,3-5,4-5,0-3,1-5".to_string());
        args.arc_costs = Some("3,4,2,5,3".to_string());
        args.edge_lengths = Some("2,1,3,2,1,4,3".to_string());

        let output_path = std::env::temp_dir().join("pred_test_create_stacker_crane.json");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let content = std::fs::read_to_string(&output_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["type"], "StackerCrane");
        assert_eq!(json["data"]["num_vertices"], 6);
        assert_eq!(json["data"]["arcs"][0], serde_json::json!([0, 4]));
        assert_eq!(json["data"]["edge_lengths"][6], 3);

        std::fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_create_stacker_crane_rejects_mismatched_arc_lengths() {
        let mut args = empty_args();
        args.problem = Some("StackerCrane".to_string());
        args.num_vertices = Some(6);
        args.arcs = Some("0>4,2>5,5>1,3>0,4>3".to_string());
        args.graph = Some("0-1,1-2,2-3,3-5,4-5,0-3,1-5".to_string());
        args.arc_costs = Some("3,4,2,5".to_string());
        args.edge_lengths = Some("2,1,3,2,1,4,3".to_string());
        args.bound = Some(20);

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("Expected 5 arc costs but got 4"));
    }

    #[test]
    fn test_create_stacker_crane_rejects_out_of_range_vertices() {
        let mut args = empty_args();
        args.problem = Some("StackerCrane".to_string());
        args.num_vertices = Some(5);
        args.arcs = Some("0>4,2>5,5>1,3>0,4>3".to_string());
        args.graph = Some("0-1,1-2,2-3,3-5,4-5,0-3,1-5".to_string());
        args.arc_costs = Some("3,4,2,5,3".to_string());
        args.edge_lengths = Some("2,1,3,2,1,4,3".to_string());
        args.bound = Some(20);

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("--num-vertices (5) is too small for the arcs"));
    }

    #[test]
    fn test_create_minimum_dummy_activities_pert_json() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::graph::MinimumDummyActivitiesPert;

        let mut args = empty_args();
        args.problem = Some("MinimumDummyActivitiesPert".to_string());
        args.num_vertices = Some(6);
        args.arcs = Some("0>2,0>3,1>3,1>4,2>5".to_string());

        let output_path = temp_output_path("minimum_dummy_activities_pert");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "MinimumDummyActivitiesPert");
        assert!(created.variant.is_empty());

        let problem: MinimumDummyActivitiesPert = serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.num_vertices(), 6);
        assert_eq!(problem.num_arcs(), 5);

        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_create_minimum_dummy_activities_pert_rejects_cycles() {
        let mut args = empty_args();
        args.problem = Some("MinimumDummyActivitiesPert".to_string());
        args.num_vertices = Some(3);
        args.arcs = Some("0>1,1>2,2>0".to_string());

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("requires the input graph to be a DAG"));
    }

    #[test]
    fn test_create_balanced_complete_bipartite_subgraph() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::graph::BalancedCompleteBipartiteSubgraph;

        let mut args = empty_args();
        args.problem = Some("BalancedCompleteBipartiteSubgraph".to_string());
        args.biedges = Some("0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3".to_string());
        args.left = Some(4);
        args.right = Some(4);
        args.k = Some(3);
        args.graph = None;

        let output_path =
            std::env::temp_dir().join(format!("bcbs-create-{}.json", std::process::id()));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = std::fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "BalancedCompleteBipartiteSubgraph");
        assert!(created.variant.is_empty());

        let problem: BalancedCompleteBipartiteSubgraph =
            serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.left_size(), 4);
        assert_eq!(problem.right_size(), 4);
        assert_eq!(problem.num_edges(), 12);
        assert_eq!(problem.k(), 3);

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_create_balanced_complete_bipartite_subgraph_rejects_out_of_range_biedges() {
        let mut args = empty_args();
        args.problem = Some("BalancedCompleteBipartiteSubgraph".to_string());
        args.biedges = Some("4-0".to_string());
        args.left = Some(4);
        args.right = Some(4);
        args.k = Some(3);
        args.graph = None;

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("out of bounds for left partition size 4"));
    }

    #[test]
    fn test_create_kclique() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::graph::KClique;

        let mut args = empty_args();
        args.problem = Some("KClique".to_string());
        args.graph = Some("0-1,0-2,1-3,2-3,2-4,3-4".to_string());
        args.k = Some(3);

        let output_path =
            std::env::temp_dir().join(format!("kclique-create-{}.json", std::process::id()));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = std::fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "KClique");
        assert_eq!(
            created.variant.get("graph").map(String::as_str),
            Some("SimpleGraph")
        );

        let problem: KClique<SimpleGraph> = serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.k(), 3);
        assert_eq!(problem.num_vertices(), 5);
        assert!(problem.evaluate(&[0, 0, 1, 1, 1]));

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_create_kclique_requires_valid_k() {
        let mut args = empty_args();
        args.problem = Some("KClique".to_string());
        args.graph = Some("0-1,0-2,1-3,2-3,2-4,3-4".to_string());
        args.k = None;

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err();
        assert!(
            err.to_string().contains("KClique requires --k"),
            "unexpected error: {err}"
        );

        args.k = Some(6);
        let err = create(&args, &out).unwrap_err();
        assert!(
            err.to_string().contains("k must be <= graph num_vertices"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_create_sparse_matrix_compression_json() {
        use crate::dispatch::ProblemJsonOutput;

        let mut args = empty_args();
        args.problem = Some("SparseMatrixCompression".to_string());
        args.matrix = Some("1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0".to_string());
        args.bound = Some(2);

        let output_path =
            std::env::temp_dir().join(format!("smc-create-{}.json", std::process::id()));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = std::fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "SparseMatrixCompression");
        assert!(created.variant.is_empty());
        assert_eq!(
            created.data,
            serde_json::json!({
                "matrix": [
                    [true, false, false, true],
                    [false, true, false, false],
                    [false, false, true, false],
                    [true, false, false, false],
                ],
                "bound_k": 2,
            })
        );

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_create_sparse_matrix_compression_requires_bound() {
        let mut args = empty_args();
        args.problem = Some("SparseMatrixCompression".to_string());
        args.matrix = Some("1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0".to_string());

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("SparseMatrixCompression requires --matrix and --bound"));
        assert!(err.contains("Usage: pred create SparseMatrixCompression"));
    }

    #[test]
    fn test_create_sparse_matrix_compression_rejects_zero_bound() {
        let mut args = empty_args();
        args.problem = Some("SparseMatrixCompression".to_string());
        args.matrix = Some("1,0;0,1".to_string());
        args.bound = Some(0);

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("bound >= 1"));
    }

    #[test]
    fn test_create_consecutive_ones_matrix_augmentation_json() {
        use crate::dispatch::ProblemJsonOutput;

        let mut args = empty_args();
        args.problem = Some("ConsecutiveOnesMatrixAugmentation".to_string());
        args.matrix = Some("1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0".to_string());
        args.bound = Some(2);

        let output_path =
            std::env::temp_dir().join(format!("coma-create-{}.json", std::process::id()));
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = std::fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "ConsecutiveOnesMatrixAugmentation");
        assert!(created.variant.is_empty());
        assert_eq!(
            created.data,
            serde_json::json!({
                "matrix": [
                    [true, false, false, true, true],
                    [true, true, false, false, false],
                    [false, true, true, false, true],
                    [false, false, true, true, false],
                ],
                "bound": 2,
            })
        );

        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn test_create_consecutive_ones_matrix_augmentation_requires_bound() {
        let mut args = empty_args();
        args.problem = Some("ConsecutiveOnesMatrixAugmentation".to_string());
        args.matrix = Some("1,0;0,1".to_string());

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("ConsecutiveOnesMatrixAugmentation requires --matrix and --bound"));
        assert!(err.contains("Usage: pred create ConsecutiveOnesMatrixAugmentation"));
    }

    #[test]
    fn test_create_consecutive_ones_matrix_augmentation_negative_bound() {
        let mut args = empty_args();
        args.problem = Some("ConsecutiveOnesMatrixAugmentation".to_string());
        args.matrix = Some("1,0;0,1".to_string());
        args.bound = Some(-1);

        let out = OutputConfig {
            output: None,
            quiet: true,
            json: false,
            auto_json: false,
        };

        let err = create(&args, &out).unwrap_err().to_string();
        assert!(err.contains("nonnegative"));
    }
}
