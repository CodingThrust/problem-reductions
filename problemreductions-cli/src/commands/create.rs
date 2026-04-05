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
    AlgebraicEquationsOverGF2, ClosestVectorProblem, ConsecutiveBlockMinimization,
    ConsecutiveOnesMatrixAugmentation, ConsecutiveOnesSubmatrix, FeasibleBasisExtension,
    MinimumMatrixCover, MinimumMatrixDomination, MinimumWeightDecoding,
    MinimumWeightSolutionToLinearEquations, QuadraticCongruences, QuadraticDiophantineEquations,
    SimultaneousIncongruences, SparseMatrixCompression, BMF,
};
use problemreductions::models::formula::Quantifier;
use problemreductions::models::graph::{
    DirectedHamiltonianPath, DisjointConnectingPaths, GeneralizedHex, GraphPartitioning,
    HamiltonianCircuit, HamiltonianPath, HamiltonianPathBetweenTwoVertices, IntegralFlowBundles,
    Kernel, LengthBoundedDisjointPaths, LongestCircuit, LongestPath, MinimumCutIntoBoundedSets,
    MinimumDummyActivitiesPert, MinimumGeometricConnectedDominatingSet, MinimumMaximalMatching,
    MinimumMultiwayCut, MixedChinesePostman, MultipleChoiceBranching, PathConstrainedNetworkFlow,
    RootedTreeArrangement, SteinerTree, SteinerTreeInGraphs, StrongConnectivityAugmentation,
    VertexCover,
};
use problemreductions::models::misc::{
    AdditionalKey, Betweenness, BinPacking, BoyceCoddNormalFormViolation, CapacityAssignment,
    CbqRelation, Clustering, ConjunctiveBooleanQuery, ConsistencyOfDatabaseFrequencyTables,
    CyclicOrdering, DynamicStorageAllocation, EnsembleComputation, ExpectedRetrievalCost,
    FeasibleRegisterAssignment, FlowShopScheduling, FrequencyTable, GroupingBySwapping, IntExpr,
    IntegerExpressionMembership, JobShopScheduling, KnownValue, KthLargestMTuple,
    LongestCommonSubsequence, MaximumLikelihoodRanking, MinimumAxiomSet,
    MinimumCodeGenerationOneRegister, MinimumCodeGenerationParallelAssignments,
    MinimumCodeGenerationUnlimitedRegisters, MinimumDecisionTree, MinimumDisjunctiveNormalForm,
    MinimumExternalMacroDataCompression, MinimumFaultDetectionTestSet,
    MinimumInternalMacroDataCompression, MinimumRegisterSufficiencyForLoops,
    MinimumTardinessSequencing, MinimumWeightAndOrGraph, MultiprocessorScheduling,
    NonLivenessFreePetriNet, Numerical3DimensionalMatching, OpenShopScheduling, PaintShop,
    PartiallyOrderedKnapsack, PreemptiveScheduling, ProductionPlanning, QueryArg,
    RectilinearPictureCompression, RegisterSufficiency, ResourceConstrainedScheduling,
    SchedulingToMinimizeWeightedCompletionTime, SchedulingWithIndividualDeadlines,
    SequencingToMinimizeMaximumCumulativeCost, SequencingToMinimizeTardyTaskWeight,
    SequencingToMinimizeWeightedCompletionTime, SequencingToMinimizeWeightedTardiness,
    SequencingWithDeadlinesAndSetUpTimes, SequencingWithReleaseTimesAndDeadlines,
    SequencingWithinIntervals, ShortestCommonSupersequence, SquareTiling, StringToStringCorrection,
    SubsetProduct, SubsetSum, SumOfSquaresPartition, ThreePartition, TimetableDesign,
};
use problemreductions::models::BiconnectivityAugmentation;
use problemreductions::prelude::*;
use problemreductions::registry::collect_schemas;
use problemreductions::topology::{
    BipartiteGraph, DirectedGraph, Graph, KingsSubgraph, MixedGraph, SimpleGraph,
    TriangularSubgraph, UnitDiskGraph,
};
use problemreductions::types::One;
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

#[derive(Debug, Clone, Default)]
struct CreateContext {
    num_vertices: Option<usize>,
    num_edges: Option<usize>,
    num_arcs: Option<usize>,
    parsed_fields: BTreeMap<String, serde_json::Value>,
}

impl CreateContext {
    #[cfg(test)]
    fn with_field(mut self, name: &str, value: serde_json::Value) -> Self {
        self.parsed_fields.insert(name.to_string(), value);
        self
    }

    fn seed_field<T: Serialize>(&mut self, name: &str, value: T) -> Result<()> {
        let value = serde_json::to_value(value)?;
        if name == "num_vertices" {
            self.num_vertices = value.as_u64().and_then(|raw| usize::try_from(raw).ok());
        }
        self.parsed_fields.insert(name.to_string(), value);
        Ok(())
    }

    fn usize_field(&self, name: &str) -> Option<usize> {
        self.parsed_fields
            .get(name)
            .and_then(serde_json::Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
    }

    fn f64_field(&self, name: &str) -> Option<f64> {
        self.parsed_fields
            .get(name)
            .and_then(serde_json::Value::as_f64)
    }

    fn remember(&mut self, name: &str, concrete_type: &str, value: &serde_json::Value) {
        self.parsed_fields.insert(name.to_string(), value.clone());

        match normalize_type_name(concrete_type).as_str() {
            "SimpleGraph" => {
                self.num_vertices = value
                    .get("num_vertices")
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|raw| usize::try_from(raw).ok());
                self.num_edges = value
                    .get("edges")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            "DirectedGraph" => {
                self.num_vertices = value
                    .get("num_vertices")
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|raw| usize::try_from(raw).ok());
                self.num_arcs = value
                    .get("arcs")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            "KingsSubgraph" | "TriangularSubgraph" => {
                self.num_vertices = value
                    .get("positions")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            "UnitDiskGraph" => {
                self.num_vertices = value
                    .get("positions")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
                self.num_edges = value
                    .get("edges")
                    .and_then(serde_json::Value::as_array)
                    .map(Vec::len);
            }
            _ => {}
        }
    }
}

fn create_schema_driven(
    args: &CreateArgs,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> Result<Option<(serde_json::Value, BTreeMap<String, String>)>> {
    if !schema_driven_supported_problem(canonical) {
        return Ok(None);
    }

    let Some(schema) = collect_schemas()
        .into_iter()
        .find(|schema| schema.name == canonical)
    else {
        return Ok(None);
    };
    let Some(variant_entry) =
        problemreductions::registry::find_variant_entry(canonical, resolved_variant)
    else {
        return Ok(None);
    };

    let graph_type = resolved_graph_type(resolved_variant);
    let is_geometry = matches!(
        graph_type,
        "KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph"
    );
    let flag_map = args.flag_map();
    let mut context = CreateContext::default();
    seed_schema_context_from_cli(args, graph_type, &mut context)?;
    validate_schema_driven_semantics(args, canonical, resolved_variant, &serde_json::Value::Null)
        .map_err(|error| with_schema_usage(error, canonical, resolved_variant))?;
    let mut json_map = serde_json::Map::new();

    for field in &schema.fields {
        let concrete_type = resolve_schema_field_type(&field.type_name, resolved_variant);
        let flag_keys =
            schema_field_flag_keys(canonical, &field.name, &field.type_name, is_geometry);
        let raw_value = get_schema_flag_value(&flag_map, &flag_keys);
        let value = if !schema_field_requires_derived_input(&field.name, &concrete_type) {
            if let Some(raw_value) = raw_value.clone() {
                match parse_schema_field_value(
                    args,
                    canonical,
                    &concrete_type,
                    &field.name,
                    &raw_value,
                    &context,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(with_schema_usage(error, canonical, resolved_variant))
                    }
                }
            } else if let Some(derived) =
                derive_schema_field_value(args, canonical, &field.name, &concrete_type, &context)?
            {
                derived
            } else {
                return Err(with_schema_usage(
                    missing_schema_field_error(
                        canonical,
                        &field.name,
                        &field.type_name,
                        is_geometry,
                    ),
                    canonical,
                    resolved_variant,
                ));
            }
        } else if let Some(derived) =
            derive_schema_field_value(args, canonical, &field.name, &concrete_type, &context)?
        {
            derived
        } else if let Some(raw_value) = raw_value {
            match parse_schema_field_value(
                args,
                canonical,
                &concrete_type,
                &field.name,
                &raw_value,
                &context,
            ) {
                Ok(value) => value,
                Err(error) => return Err(with_schema_usage(error, canonical, resolved_variant)),
            }
        } else {
            return Err(with_schema_usage(
                missing_schema_field_error(canonical, &field.name, &field.type_name, is_geometry),
                canonical,
                resolved_variant,
            ));
        };

        context.remember(&field.name, &concrete_type, &value);
        json_map.insert(field.name.clone(), value);
    }

    let data = serde_json::Value::Object(json_map);
    validate_schema_driven_semantics(args, canonical, resolved_variant, &data)
        .map_err(|error| with_schema_usage(error, canonical, resolved_variant))?;
    (variant_entry.factory)(data.clone()).map_err(|error| {
        with_schema_usage(
            anyhow::anyhow!(
                "Schema-driven factory rejected generated data for {canonical}: {error}"
            ),
            canonical,
            resolved_variant,
        )
    })?;

    Ok(Some((data, resolved_variant.clone())))
}

fn missing_schema_field_error(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> anyhow::Error {
    let display = problem_help_flag_name(canonical, field_name, field_type, is_geometry);
    let flags: Vec<String> = display
        .split('/')
        .filter_map(|part| {
            let trimmed = part.trim().trim_start_matches("--");
            (!trimmed.is_empty()).then(|| format!("--{trimmed}"))
        })
        .collect();
    let requirement = match flags.as_slice() {
        [] => format!("--{}", field_name.replace('_', "-")),
        [flag] => flag.clone(),
        [first, second] => format!("{first} or {second}"),
        _ => {
            let last = flags.last().cloned().unwrap_or_default();
            format!("{}, or {}", flags[..flags.len() - 1].join(", "), last)
        }
    };
    anyhow::anyhow!("{canonical} requires {requirement}")
}

fn parse_schema_field_value(
    args: &CreateArgs,
    canonical: &str,
    concrete_type: &str,
    field_name: &str,
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    match (canonical, field_name) {
        ("BoyceCoddNormalFormViolation", "functional_deps") => {
            let num_attributes = args.n.ok_or_else(|| {
                anyhow::anyhow!("BoyceCoddNormalFormViolation requires --n, --sets, and --target")
            })?;
            Ok(serde_json::to_value(parse_bcnf_functional_deps(
                raw,
                num_attributes,
            )?)?)
        }
        ("BoundedComponentSpanningForest", "max_weight") => {
            let usage = "Usage: pred create BoundedComponentSpanningForest --graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --max-weight 6";
            let bound_raw = args.bound.ok_or_else(|| {
                anyhow::anyhow!("BoundedComponentSpanningForest requires --max-weight\n\n{usage}")
            })?;
            let max_weight = i32::try_from(bound_raw).map_err(|_| {
                anyhow::anyhow!(
                    "BoundedComponentSpanningForest requires --max-weight within i32 range\n\n{usage}"
                )
            })?;
            Ok(serde_json::json!(max_weight))
        }
        ("ConsecutiveBlockMinimization", "matrix") => {
            let usage = "Usage: pred create ConsecutiveBlockMinimization --matrix '[[true,false,true],[false,true,true]]' --bound-k 2";
            let matrix: Vec<Vec<bool>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array (e.g., '[[true,false,true],[false,true,true]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("FeasibleBasisExtension", "matrix") => {
            let usage = "Usage: pred create FeasibleBasisExtension --matrix '[[1,0,1],[0,1,0]]' --rhs '7,5' --required-columns '0'";
            let matrix: Vec<Vec<i64>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "FeasibleBasisExtension requires --matrix as a JSON 2D integer array (e.g., '[[1,0,1],[0,1,0]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("IntegralFlowBundles", "bundle_capacities") => {
            let usage = "Usage: pred create IntegralFlowBundles --arcs \"0>1,0>2,1>3,2>3,1>2,2>1\" --bundles \"0,1;2,5;3,4\" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --arcs\n\n{usage}"))?;
            let (_, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let bundles = parse_bundles(args, num_arcs, usage)?;
            Ok(serde_json::to_value(parse_bundle_capacities(
                args,
                bundles.len(),
                usage,
            )?)?)
        }
        ("IntegralFlowHomologousArcs", "homologous_pairs") => {
            Ok(serde_json::to_value(parse_homologous_pairs(args)?)?)
        }
        ("LengthBoundedDisjointPaths", "max_length") => {
            let usage = "Usage: pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --max-length 3";
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --max-length\n\n{usage}")
            })?;
            let max_length = usize::try_from(bound).map_err(|_| {
                anyhow::anyhow!(
                    "--max-length must be a nonnegative integer for LengthBoundedDisjointPaths\n\n{usage}"
                )
            })?;
            Ok(serde_json::json!(max_length))
        }
        ("LongestCommonSubsequence", "strings") => {
            let (strings, _) = parse_lcs_strings(raw)?;
            Ok(serde_json::to_value(strings)?)
        }
        ("MinimumDecisionTree", "test_matrix") => {
            let usage = "Usage: pred create MinimumDecisionTree --test-matrix '[[true,true,false,false],[true,false,false,false],[false,true,false,true]]' --num-objects 4 --num-tests 3";
            let matrix: Vec<Vec<bool>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "MinimumDecisionTree requires --test-matrix as a JSON 2D bool array\n\n{usage}\n\nFailed to parse --test-matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("MinimumWeightDecoding", "matrix") => {
            let usage = "Usage: pred create MinimumWeightDecoding --matrix '[[true,false,true],[false,true,true]]' --rhs 'true,true'";
            let matrix: Vec<Vec<bool>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "MinimumWeightDecoding requires --matrix as a JSON 2D bool array (e.g., '[[true,false],[false,true]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("MinimumWeightSolutionToLinearEquations", "matrix") => {
            let usage = "Usage: pred create MinimumWeightSolutionToLinearEquations --matrix '[[1,2,3,1],[2,1,1,3]]' --rhs '5,4'";
            let matrix: Vec<Vec<i64>> = serde_json::from_str(raw).map_err(|err| {
                anyhow::anyhow!(
                    "MinimumWeightSolutionToLinearEquations requires --matrix as a JSON 2D integer array (e.g., '[[1,2,3],[4,5,6]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            Ok(serde_json::to_value(matrix)?)
        }
        ("GroupingBySwapping", "string")
        | ("StringToStringCorrection", "source")
        | ("StringToStringCorrection", "target") => {
            Ok(serde_json::to_value(parse_symbol_list_allow_empty(raw)?)?)
        }
        ("MultipleCopyFileAllocation", "usage") => {
            let (_, num_vertices) = parse_graph(args)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{MULTIPLE_COPY_FILE_ALLOCATION_USAGE}"))?;
            Ok(serde_json::to_value(parse_vertex_i64_values(
                args.usage.as_deref(),
                "usage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?)?)
        }
        ("MultipleCopyFileAllocation", "storage") => {
            let (_, num_vertices) = parse_graph(args)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{MULTIPLE_COPY_FILE_ALLOCATION_USAGE}"))?;
            Ok(serde_json::to_value(parse_vertex_i64_values(
                args.storage.as_deref(),
                "storage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?)?)
        }
        ("SequencingToMinimizeMaximumCumulativeCost", "precedences") => {
            Ok(serde_json::to_value(parse_precedence_pairs(
                args.precedences
                    .as_deref()
                    .or(args.precedence_pairs.as_deref()),
            )?)?)
        }
        ("UndirectedTwoCommodityIntegralFlow", "capacities") => {
            let usage = "Usage: pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            Ok(serde_json::to_value(parse_capacities(
                args,
                graph.num_edges(),
                usage,
            )?)?)
        }
        _ => parse_field_value(concrete_type, field_name, raw, context),
    }
}

fn schema_driven_supported_problem(canonical: &str) -> bool {
    canonical != "ILP" && canonical != "CircuitSAT"
}

fn schema_field_flag_keys(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> Vec<String> {
    let mut keys = vec![field_name.replace('_', "-")];
    for display_key in problem_help_flag_name(canonical, field_name, field_type, is_geometry)
        .split('/')
        .map(|key| key.trim().trim_start_matches("--").to_string())
        .filter(|key| !key.is_empty())
    {
        if !keys.contains(&display_key) {
            keys.push(display_key);
        }
    }
    keys
}

fn get_schema_flag_value(
    flag_map: &std::collections::HashMap<&'static str, Option<String>>,
    keys: &[String],
) -> Option<String> {
    keys.iter()
        .find_map(|key| flag_map.get(key.as_str()).cloned().flatten())
}

fn resolve_schema_field_type(
    type_name: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> String {
    let normalized = normalize_type_name(type_name);
    let graph_type = resolved_variant
        .get("graph")
        .map(String::as_str)
        .unwrap_or("SimpleGraph");
    let weight_type = resolved_variant
        .get("weight")
        .map(String::as_str)
        .unwrap_or("One");

    match normalized.as_str() {
        "G" => graph_type.to_string(),
        "W" => weight_type.to_string(),
        "W::Sum" => weight_sum_type(weight_type).to_string(),
        "Vec<W>" => format!("Vec<{weight_type}>"),
        "Vec<Vec<W>>" => format!("Vec<Vec<{weight_type}>>"),
        "Vec<(usize,usize,W)>" => format!("Vec<(usize,usize,{weight_type})>"),
        "Vec<Vec<T>>" => format!("Vec<Vec<{weight_type}>>"),
        other => other.to_string(),
    }
}

fn weight_sum_type(weight_type: &str) -> &'static str {
    match weight_type {
        "One" | "i32" => "i32",
        "f64" => "f64",
        _ => "i32",
    }
}

fn seed_schema_context_from_cli(
    args: &CreateArgs,
    graph_type: &str,
    context: &mut CreateContext,
) -> Result<()> {
    if let Some(num_vertices) = args.num_vertices {
        context.seed_field("num_vertices", num_vertices)?;
    }
    if graph_type == "UnitDiskGraph" {
        context.seed_field("radius", args.radius.unwrap_or(1.0))?;
    }
    Ok(())
}

fn derive_schema_field_value(
    args: &CreateArgs,
    canonical: &str,
    field_name: &str,
    concrete_type: &str,
    context: &CreateContext,
) -> Result<Option<serde_json::Value>> {
    if let Some(defaulted) =
        derive_schema_default_value(canonical, field_name, concrete_type, context)?
    {
        return Ok(Some(defaulted));
    }

    if field_name == "graph" && concrete_type == "MixedGraph" {
        let usage = format!(
            "Usage: pred create {canonical} {}",
            example_for(canonical, None)
        );
        return Ok(Some(serde_json::to_value(parse_mixed_graph(
            args, &usage,
        )?)?));
    }

    if field_name == "graph" && concrete_type == "BipartiteGraph" {
        let left = args
            .left
            .ok_or_else(|| anyhow::anyhow!("{canonical} requires --left"))?;
        let right = args
            .right
            .ok_or_else(|| anyhow::anyhow!("{canonical} requires --right"))?;
        let edges_raw = args
            .biedges
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("{canonical} requires --biedges"))?;
        let edges = util::parse_edge_pairs(edges_raw)?;
        validate_bipartite_edges(canonical, left, right, &edges)?;
        return Ok(Some(serde_json::to_value(BipartiteGraph::new(
            left, right, edges,
        ))?));
    }

    if canonical == "ClosestVectorProblem"
        && field_name == "bounds"
        && normalize_type_name(concrete_type) == "Vec<VarBounds>"
    {
        return Ok(Some(parse_cvp_bounds_value(
            args.bounds.as_deref(),
            context,
        )?));
    }

    if canonical == "ConjunctiveBooleanQuery"
        && field_name == "num_variables"
        && normalize_type_name(concrete_type) == "usize"
    {
        let raw = args
            .conjuncts_spec
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("ConjunctiveBooleanQuery requires --conjuncts-spec"))?;
        return Ok(Some(serde_json::json!(infer_cbq_num_variables(raw)?)));
    }

    if canonical == "GroupingBySwapping"
        && field_name == "alphabet_size"
        && normalize_type_name(concrete_type) == "usize"
    {
        let raw = args
            .string
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("GroupingBySwapping requires --string"))?;
        let string = parse_symbol_list_allow_empty(raw)?;
        let inferred = string.iter().copied().max().map_or(0, |value| value + 1);
        return Ok(Some(serde_json::json!(args
            .alphabet_size
            .unwrap_or(inferred))));
    }

    if canonical == "JobShopScheduling"
        && field_name == "num_processors"
        && normalize_type_name(concrete_type) == "usize"
    {
        let usage = "Usage: pred create JobShopScheduling --jobs \"0:3,1:4;1:2,0:3,1:2;0:4,1:3\" --num-processors 2";
        let inferred_processors = match args.job_tasks.as_deref() {
            Some(job_tasks) => {
                let jobs = parse_job_shop_jobs(job_tasks)?;
                jobs.iter()
                    .flat_map(|job| job.iter().map(|(processor, _)| *processor))
                    .max()
                    .map(|processor| processor + 1)
            }
            None => None,
        };
        let num_processors =
            resolve_processor_count_flags("JobShopScheduling", usage, args.num_processors, args.m)?
                .or(inferred_processors)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Cannot infer num_processors from empty job list; use --num-processors"
                    )
                })?;
        return Ok(Some(serde_json::json!(num_processors)));
    }

    if canonical == "LongestCommonSubsequence"
        && field_name == "alphabet_size"
        && normalize_type_name(concrete_type) == "usize"
    {
        let raw = args
            .strings
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("LongestCommonSubsequence requires --strings"))?;
        let (_, inferred_alphabet_size) = parse_lcs_strings(raw)?;
        return Ok(Some(serde_json::json!(args
            .alphabet_size
            .unwrap_or(inferred_alphabet_size))));
    }

    if canonical == "LongestCommonSubsequence"
        && field_name == "max_length"
        && normalize_type_name(concrete_type) == "usize"
    {
        let strings: Vec<Vec<usize>> =
            serde_json::from_value(context.parsed_fields.get("strings").cloned().ok_or_else(
                || anyhow::anyhow!("LCS max_length derivation requires parsed strings"),
            )?)?;
        let max_length = strings.iter().map(Vec::len).min().unwrap_or(0);
        return Ok(Some(serde_json::json!(max_length)));
    }

    if canonical == "QUBO"
        && field_name == "num_vars"
        && normalize_type_name(concrete_type) == "usize"
    {
        let matrix = parse_matrix(args)?;
        return Ok(Some(serde_json::json!(matrix.len())));
    }

    if canonical == "StringToStringCorrection"
        && field_name == "alphabet_size"
        && normalize_type_name(concrete_type) == "usize"
    {
        let source = parse_symbol_list_allow_empty(args.source_string.as_deref().unwrap_or(""))?;
        let target = parse_symbol_list_allow_empty(args.target_string.as_deref().unwrap_or(""))?;
        let inferred = source
            .iter()
            .chain(target.iter())
            .copied()
            .max()
            .map_or(0, |value| value + 1);
        return Ok(Some(serde_json::json!(args
            .alphabet_size
            .unwrap_or(inferred))));
    }

    if field_name == "precedences"
        && normalize_type_name(concrete_type) == "Vec<(usize,usize)>"
        && args.precedences.is_none()
        && args.precedence_pairs.is_none()
    {
        return Ok(Some(serde_json::json!([])));
    }

    if canonical == "ComparativeContainment"
        && matches!(field_name, "r_weights" | "s_weights")
        && matches!(
            normalize_type_name(concrete_type).as_str(),
            "Vec<One>" | "Vec<i32>" | "Vec<f64>"
        )
    {
        let sets_len = context
            .parsed_fields
            .get(match field_name {
                "r_weights" => "r_sets",
                _ => "s_sets",
            })
            .and_then(serde_json::Value::as_array)
            .map(Vec::len);
        if let Some(len) = sets_len {
            let value = match normalize_type_name(concrete_type).as_str() {
                "Vec<One>" | "Vec<i32>" => serde_json::json!(vec![1_i32; len]),
                "Vec<f64>" => serde_json::json!(vec![1.0_f64; len]),
                _ => unreachable!(),
            };
            return Ok(Some(value));
        }
    }

    if canonical == "ConsistencyOfDatabaseFrequencyTables"
        && field_name == "known_values"
        && normalize_type_name(concrete_type) == "Vec<KnownValue>"
        && args.known_values.is_none()
    {
        return Ok(Some(serde_json::json!([])));
    }

    if canonical == "LengthBoundedDisjointPaths"
        && field_name == "max_paths"
        && normalize_type_name(concrete_type) == "usize"
    {
        let graph_value = context.parsed_fields.get("graph").cloned();
        let source = context.usize_field("source");
        let sink = context.usize_field("sink");
        if let (Some(graph_value), Some(source), Some(sink)) = (graph_value, source, sink) {
            let graph: SimpleGraph =
                serde_json::from_value(graph_value).context("Failed to deserialize graph")?;
            let max_paths = graph
                .neighbors(source)
                .len()
                .min(graph.neighbors(sink).len());
            return Ok(Some(serde_json::json!(max_paths)));
        }
    }

    Ok(None)
}

fn derive_schema_default_value(
    canonical: &str,
    field_name: &str,
    concrete_type: &str,
    context: &CreateContext,
) -> Result<Option<serde_json::Value>> {
    let normalized = normalize_type_name(concrete_type);

    let one_list = |len: usize| match normalized.as_str() {
        "Vec<One>" | "Vec<i32>" => Some(serde_json::json!(vec![1_i32; len])),
        "Vec<u64>" => Some(serde_json::json!(vec![1_u64; len])),
        "Vec<i64>" => Some(serde_json::json!(vec![1_i64; len])),
        "Vec<usize>" => Some(serde_json::json!(vec![1_usize; len])),
        "Vec<f64>" => Some(serde_json::json!(vec![1.0_f64; len])),
        _ => None,
    };

    let derived = match field_name {
        "weights" | "vertex_weights" => context.num_vertices.and_then(one_list),
        "edge_weights" | "edge_lengths" => context.num_edges.and_then(one_list),
        "arc_weights" | "arc_lengths" if context.num_arcs.is_some() => {
            context.num_arcs.and_then(one_list)
        }
        "capacities" if canonical == "PathConstrainedNetworkFlow" => {
            context.num_arcs.and_then(one_list)
        }
        "couplings" if canonical == "SpinGlass" => context.num_edges.and_then(one_list),
        "fields" if canonical == "SpinGlass" => match normalized.as_str() {
            "Vec<i32>" => context
                .num_vertices
                .map(|len| serde_json::json!(vec![0_i32; len])),
            "Vec<f64>" => context
                .num_vertices
                .map(|len| serde_json::json!(vec![0.0_f64; len])),
            _ => None,
        },
        _ => None,
    };

    Ok(derived)
}

fn schema_field_requires_derived_input(field_name: &str, concrete_type: &str) -> bool {
    field_name == "graph" && matches!(concrete_type, "MixedGraph" | "BipartiteGraph")
}

fn is_unsupported_schema_parser(error: &anyhow::Error) -> bool {
    error.to_string().contains("Unsupported schema parser")
}

fn with_schema_usage(
    error: anyhow::Error,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> anyhow::Error {
    let message = error.to_string();
    if message.contains("Usage: pred create") {
        return error;
    }
    let graph_type = resolved_variant.get("graph").map(String::as_str);
    anyhow::anyhow!(
        "{message}\n\nUsage: pred create {canonical} {}",
        example_for(canonical, graph_type)
    )
}

fn parse_field_value(
    concrete_type: &str,
    field_name: &str,
    raw: &str,
    context: &CreateContext,
) -> Result<serde_json::Value> {
    let normalized_type = normalize_type_name(concrete_type);
    let value = match normalized_type.as_str() {
        "SimpleGraph" => parse_simple_graph_value(raw, context)?,
        "DirectedGraph" => parse_directed_graph_value(raw, context)?,
        "KingsSubgraph" => parse_grid_subgraph_value(raw, true)?,
        "TriangularSubgraph" => parse_grid_subgraph_value(raw, false)?,
        "UnitDiskGraph" => parse_unit_disk_graph_value(raw, context)?,
        "Vec<i32>" => parse_numeric_list_value::<i32>(raw)?,
        "Vec<f64>" => parse_numeric_list_value::<f64>(raw)?,
        "Vec<u64>" => parse_numeric_list_value::<u64>(raw)?,
        "Vec<i64>" => parse_numeric_list_value::<i64>(raw)?,
        "Vec<usize>" => parse_numeric_list_value::<usize>(raw)?,
        "Vec<One>" => parse_numeric_list_value::<i32>(raw)?,
        "Vec<bool>" => parse_bool_list_value(raw)?,
        "Vec<Vec<usize>>" => parse_nested_numeric_list_value::<usize>(raw)?,
        "Vec<Vec<u64>>" => parse_nested_numeric_list_value::<u64>(raw)?,
        "Vec<Vec<i32>>" => parse_nested_numeric_list_value::<i32>(raw)?,
        "Vec<Vec<i64>>" => parse_nested_numeric_list_value::<i64>(raw)?,
        "Vec<Vec<f64>>" => parse_nested_numeric_list_value::<f64>(raw)?,
        "Vec<Vec<One>>" => parse_nested_numeric_list_value::<i32>(raw)?,
        "Vec<Vec<bool>>" => parse_bool_rows_value(raw, field_name)?,
        "Vec<Vec<Vec<usize>>>" => parse_3d_numeric_list_value::<usize>(raw)?,
        "Vec<Vec<Vec<i64>>>" => parse_3d_numeric_list_value::<i64>(raw)?,
        "Vec<[usize;3]>" => parse_triple_array_list_value(raw)?,
        "Vec<CNFClause>" => serde_json::to_value(parse_clauses_raw(raw)?)?,
        "Vec<(usize,usize)>" => parse_pair_list_value(raw)?,
        "Vec<(u64,u64)>" => parse_semicolon_tuple_list_value::<u64, 2>(raw)?,
        "Vec<(usize,f64)>" => parse_indexed_numeric_pairs_value::<f64>(raw)?,
        "Vec<(usize,usize,usize)>" => parse_semicolon_tuple_list_value::<usize, 3>(raw)?,
        "Vec<(usize,usize,usize,usize)>" => parse_semicolon_tuple_list_value::<usize, 4>(raw)?,
        "Vec<(usize,usize,One)>" => parse_weighted_edge_list_value::<i32>(raw)?,
        "Vec<(usize,usize,i32)>" => parse_weighted_edge_list_value::<i32>(raw)?,
        "Vec<(usize,usize,i64)>" => parse_weighted_edge_list_value::<i64>(raw)?,
        "Vec<(usize,usize,u64)>" => parse_weighted_edge_list_value::<u64>(raw)?,
        "Vec<(usize,usize,f64)>" => parse_weighted_edge_list_value::<f64>(raw)?,
        "Vec<(Vec<usize>,Vec<usize>)>" => serde_json::to_value(parse_dependencies(raw)?)?,
        "Vec<(Vec<usize>,usize)>" => serde_json::to_value(parse_implications(raw)?)?,
        "Vec<(usize,Vec<QueryArg>)>" => serde_json::to_value(parse_cbq_conjuncts(raw, context)?)?,
        "Vec<(usize,Vec<usize>)>" => parse_indexed_usize_lists_value(raw)?,
        "Vec<Vec<(usize,u64)>>" => serde_json::to_value(parse_job_shop_jobs(raw)?)?,
        "Vec<(f64,f64)>" => serde_json::to_value(util::parse_positions::<f64>(raw, "0.0,0.0")?)?,
        "Vec<FrequencyTable>" => {
            serde_json::to_value(parse_cdft_frequency_tables_value(raw, context)?)?
        }
        "Vec<KnownValue>" => serde_json::to_value(parse_cdft_known_values_value(raw, context)?)?,
        "Vec<Relation>" => serde_json::to_value(parse_cbq_relations(raw, context)?)?,
        "Vec<String>" => parse_string_list_value(raw)?,
        "Vec<VarBounds>" => parse_cvp_bounds_value(Some(raw), context)?,
        "Vec<BigUint>" => parse_biguint_list_value(raw)?,
        "BigUint" => parse_biguint_value(raw)?,
        "Vec<Option<bool>>" => parse_optional_bool_list_value(raw)?,
        "Vec<Quantifier>" => serde_json::to_value(parse_quantifiers_raw(raw, context)?)?,
        "IntExpr" => parse_json_passthrough_value(raw)?,
        "bool" => serde_json::to_value(parse_bool_token(raw.trim())?)?,
        "One" => serde_json::json!(1),
        "usize" => parse_scalar_value::<usize>(raw)?,
        "u64" => parse_scalar_value::<u64>(raw)?,
        "i32" => parse_scalar_value::<i32>(raw)?,
        "i64" => parse_scalar_value::<i64>(raw)?,
        "f64" => parse_scalar_value::<f64>(raw)?,
        other => bail!("Unsupported schema parser for field '{field_name}' with type '{other}'"),
    };

    Ok(value)
}

fn normalize_type_name(type_name: &str) -> String {
    type_name.chars().filter(|ch| !ch.is_whitespace()).collect()
}

fn parse_scalar_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    Ok(serde_json::to_value(raw.trim().parse::<T>().map_err(
        |err| anyhow::anyhow!("Invalid value '{}': {err}", raw.trim()),
    )?)?)
}

fn parse_numeric_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    Ok(serde_json::to_value(util::parse_comma_list::<T>(raw)?)?)
}

fn parse_bool_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<bool> = raw
        .split(',')
        .map(|entry| parse_bool_token(entry.trim()))
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(values)?)
}

fn parse_bool_rows_value(raw: &str, field_name: &str) -> Result<serde_json::Value> {
    let flag = format!("--{}", field_name.replace('_', "-"));
    let rows = parse_bool_rows(raw)
        .map_err(|err| anyhow::anyhow!("{}", err.to_string().replace("--matrix", &flag)))?;
    Ok(serde_json::to_value(rows)?)
}

fn parse_nested_numeric_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let rows: Vec<Vec<T>> = raw
        .split(';')
        .map(|row| util::parse_comma_list::<T>(row.trim()))
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(rows)?)
}

fn parse_3d_numeric_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let matrices: Vec<Vec<Vec<T>>> = raw
        .split('|')
        .map(|matrix| {
            matrix
                .split(';')
                .map(|row| util::parse_comma_list::<T>(row.trim()))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(matrices)?)
}

fn parse_triple_array_list_value(raw: &str) -> Result<serde_json::Value> {
    let triples: Vec<[usize; 3]> = raw
        .split(';')
        .map(|entry| {
            let values: Vec<usize> = util::parse_comma_list(entry.trim())?;
            anyhow::ensure!(
                values.len() == 3,
                "Expected triple with exactly 3 entries, got {}",
                values.len()
            );
            Ok([values[0], values[1], values[2]])
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(triples)?)
}

fn parse_clauses_raw(raw: &str) -> Result<Vec<CNFClause>> {
    raw.split(';')
        .map(|clause| {
            let literals: Vec<i32> = clause
                .trim()
                .split(',')
                .map(|value| value.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            Ok(CNFClause::new(literals))
        })
        .collect()
}

fn parse_pair_list_value(raw: &str) -> Result<serde_json::Value> {
    let pairs: Vec<(usize, usize)> = raw
        .split(',')
        .map(|entry| {
            let entry = entry.trim();
            let parts: Vec<&str> = if entry.contains('>') {
                entry.split('>').collect()
            } else {
                entry.split('-').collect()
            };
            anyhow::ensure!(
                parts.len() == 2,
                "Invalid pair '{entry}': expected u-v or u>v"
            );
            Ok((
                parts[0].trim().parse::<usize>()?,
                parts[1].trim().parse::<usize>()?,
            ))
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(pairs)?)
}

fn infer_cbq_num_variables(raw: &str) -> Result<usize> {
    let mut num_vars = 0usize;
    for conjunct in raw.split(';').filter(|entry| !entry.trim().is_empty()) {
        let (_, args_str) = conjunct.trim().split_once(':').ok_or_else(|| {
            anyhow::anyhow!(
                "Invalid conjunct format: expected 'rel_idx:args', got '{}'",
                conjunct.trim()
            )
        })?;
        for arg in args_str
            .split(',')
            .map(str::trim)
            .filter(|arg| !arg.is_empty())
        {
            if let Some(rest) = arg.strip_prefix('v') {
                let index: usize = rest
                    .parse()
                    .map_err(|err| anyhow::anyhow!("Invalid variable index '{rest}': {err}"))?;
                num_vars = num_vars.max(index + 1);
            }
        }
    }
    Ok(num_vars)
}

fn parse_cbq_relations(raw: &str, context: &CreateContext) -> Result<Vec<CbqRelation>> {
    let domain_size = context.usize_field("domain_size").ok_or_else(|| {
        anyhow::anyhow!("CBQ relation parsing requires a prior domain_size field")
    })?;

    raw.split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|rel_str| {
            let rel_str = rel_str.trim();
            let (arity_str, tuples_str) = rel_str.split_once(':').ok_or_else(|| {
                anyhow::anyhow!("Invalid relation format: expected 'arity:tuples', got '{rel_str}'")
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
                    .filter(|tuple| !tuple.trim().is_empty())
                    .map(|tuple| {
                        let tuple: Vec<usize> = util::parse_comma_list(tuple.trim())?;
                        anyhow::ensure!(
                            tuple.len() == arity,
                            "Relation tuple has {} entries, expected arity {arity}",
                            tuple.len()
                        );
                        for &value in &tuple {
                            anyhow::ensure!(
                                value < domain_size,
                                "Tuple value {value} >= domain-size {domain_size}"
                            );
                        }
                        Ok(tuple)
                    })
                    .collect::<Result<_>>()?
            };
            Ok(CbqRelation { arity, tuples })
        })
        .collect()
}

fn parse_cbq_conjuncts(raw: &str, context: &CreateContext) -> Result<Vec<(usize, Vec<QueryArg>)>> {
    let relations: Vec<CbqRelation> =
        serde_json::from_value(context.parsed_fields.get("relations").cloned().ok_or_else(
            || anyhow::anyhow!("CBQ conjunct parsing requires prior relations field"),
        )?)
        .context("Failed to deserialize parsed CBQ relations")?;
    let domain_size = context
        .usize_field("domain_size")
        .ok_or_else(|| anyhow::anyhow!("CBQ conjunct parsing requires prior domain_size field"))?;
    let num_variables = context.usize_field("num_variables").unwrap_or(0);

    raw.split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|conj_str| {
            let conj_str = conj_str.trim();
            let (idx_str, args_str) = conj_str.split_once(':').ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid conjunct format: expected 'rel_idx:args', got '{conj_str}'"
                )
            })?;
            let rel_idx: usize = idx_str
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid relation index '{idx_str}': {e}"))?;
            anyhow::ensure!(
                rel_idx < relations.len(),
                "Conjunct references relation {rel_idx}, but only {} relations exist",
                relations.len()
            );

            let query_args: Vec<QueryArg> = args_str
                .split(',')
                .map(|arg| {
                    let arg = arg.trim();
                    if let Some(rest) = arg.strip_prefix('v') {
                        let variable: usize = rest
                            .parse()
                            .map_err(|e| anyhow::anyhow!("Invalid variable index '{rest}': {e}"))?;
                        anyhow::ensure!(
                            variable < num_variables,
                            "Variable({variable}) >= num_variables ({num_variables})"
                        );
                        Ok(QueryArg::Variable(variable))
                    } else if let Some(rest) = arg.strip_prefix('c') {
                        let constant: usize = rest
                            .parse()
                            .map_err(|e| anyhow::anyhow!("Invalid constant value '{rest}': {e}"))?;
                        anyhow::ensure!(
                            constant < domain_size,
                            "Constant {constant} >= domain-size {domain_size}"
                        );
                        Ok(QueryArg::Constant(constant))
                    } else {
                        Err(anyhow::anyhow!(
                            "Invalid query arg '{arg}': expected vN (variable) or cN (constant)"
                        ))
                    }
                })
                .collect::<Result<_>>()?;
            anyhow::ensure!(
                query_args.len() == relations[rel_idx].arity,
                "Conjunct has {} args, but relation {rel_idx} has arity {}",
                query_args.len(),
                relations[rel_idx].arity
            );
            Ok((rel_idx, query_args))
        })
        .collect()
}

fn parse_semicolon_tuple_list_value<T, const N: usize>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let tuples: Vec<Vec<T>> = raw
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let values: Vec<T> = util::parse_comma_list(entry.trim())?;
            anyhow::ensure!(
                values.len() == N,
                "Expected tuple with {N} entries, got {}",
                values.len()
            );
            Ok(values)
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(tuples)?)
}

fn parse_weighted_edge_list_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let edges: Vec<(usize, usize, T)> = raw
        .split(',')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let entry = entry.trim();
            let (edge_part, weight_part) = entry.split_once(':').ok_or_else(|| {
                anyhow::anyhow!("Invalid weighted edge '{entry}': expected u-v:w")
            })?;
            let (u_str, v_str) = if let Some((u, v)) = edge_part.split_once('-') {
                (u, v)
            } else if let Some((u, v)) = edge_part.split_once('>') {
                (u, v)
            } else {
                bail!("Invalid weighted edge '{entry}': expected u-v:w or u>v:w");
            };
            Ok((
                u_str.trim().parse::<usize>()?,
                v_str.trim().parse::<usize>()?,
                weight_part.trim().parse::<T>().map_err(|err| {
                    anyhow::anyhow!("Invalid edge weight '{}': {err}", weight_part.trim())
                })?,
            ))
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(edges)?)
}

fn parse_indexed_numeric_pairs_value<T>(raw: &str) -> Result<serde_json::Value>
where
    T: std::str::FromStr + Serialize,
    T::Err: std::fmt::Display,
{
    let pairs: Vec<(usize, T)> =
        raw.split(',')
            .filter(|entry| !entry.trim().is_empty())
            .map(|entry| {
                let entry = entry.trim();
                let (index, value) = entry.split_once(':').ok_or_else(|| {
                    anyhow::anyhow!("Invalid pair '{entry}': expected index:value")
                })?;
                Ok((
                    index.trim().parse::<usize>()?,
                    value.trim().parse::<T>().map_err(|err| {
                        anyhow::anyhow!("Invalid value '{}': {err}", value.trim())
                    })?,
                ))
            })
            .collect::<Result<_>>()?;
    Ok(serde_json::to_value(pairs)?)
}

fn parse_indexed_usize_lists_value(raw: &str) -> Result<serde_json::Value> {
    let entries: Vec<(usize, Vec<usize>)> = raw
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| {
            let entry = entry.trim();
            let (index, values) = entry
                .split_once(':')
                .ok_or_else(|| anyhow::anyhow!("Invalid entry '{entry}': expected index:values"))?;
            Ok((
                index.trim().parse::<usize>()?,
                if values.trim().is_empty() {
                    Vec::new()
                } else {
                    util::parse_comma_list(values.trim())?
                },
            ))
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(entries)?)
}

fn parse_string_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<String> = raw
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| entry.trim().to_string())
        .collect();
    Ok(serde_json::to_value(values)?)
}

fn parse_symbol_list_allow_empty(raw: &str) -> Result<Vec<usize>> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    raw.split(',')
        .map(|value| {
            value
                .trim()
                .parse::<usize>()
                .context("invalid symbol index")
        })
        .collect()
}

fn parse_lcs_strings(raw: &str) -> Result<(Vec<Vec<usize>>, usize)> {
    let segments: Vec<&str> = raw.split(';').map(str::trim).collect();
    let comma_mode = segments.iter().any(|segment| segment.contains(','));

    if comma_mode {
        let strings = segments
            .iter()
            .map(|segment| parse_symbol_list_allow_empty(segment))
            .collect::<Result<Vec<_>>>()?;
        let inferred_alphabet_size = strings
            .iter()
            .flat_map(|string| string.iter())
            .copied()
            .max()
            .map(|value| value + 1)
            .unwrap_or(0);
        return Ok((strings, inferred_alphabet_size));
    }

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
    Ok((strings, next_symbol))
}

fn parse_bcnf_functional_deps(
    raw: &str,
    num_attributes: usize,
) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
    raw.split(';')
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
                num_attributes,
                &format!("Functional dependency '{fd_str}' lhs"),
            )?;
            ensure_attribute_indices_in_range(
                &rhs,
                num_attributes,
                &format!("Functional dependency '{fd_str}' rhs"),
            )?;
            Ok((lhs, rhs))
        })
        .collect()
}

fn parse_cdft_frequency_tables_value(
    raw: &str,
    context: &CreateContext,
) -> Result<Vec<FrequencyTable>> {
    let attribute_domains: Vec<usize> = serde_json::from_value(
        context
            .parsed_fields
            .get("attribute_domains")
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "CDFT frequency table parsing requires prior attribute_domains field"
                )
            })?,
    )
    .context("Failed to deserialize parsed CDFT attribute domains")?;
    let num_objects = context.usize_field("num_objects").ok_or_else(|| {
        anyhow::anyhow!("CDFT frequency table parsing requires prior num_objects field")
    })?;
    parse_cdft_frequency_tables(raw, &attribute_domains, num_objects)
}

fn parse_cdft_known_values_value(raw: &str, context: &CreateContext) -> Result<Vec<KnownValue>> {
    let attribute_domains: Vec<usize> = serde_json::from_value(
        context
            .parsed_fields
            .get("attribute_domains")
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!("CDFT known-value parsing requires prior attribute_domains field")
            })?,
    )
    .context("Failed to deserialize parsed CDFT attribute domains")?;
    let num_objects = context.usize_field("num_objects").ok_or_else(|| {
        anyhow::anyhow!("CDFT known-value parsing requires prior num_objects field")
    })?;
    parse_cdft_known_values(Some(raw), num_objects, &attribute_domains)
}

fn parse_cvp_bounds_value(raw: Option<&str>, context: &CreateContext) -> Result<serde_json::Value> {
    let basis_len = context
        .parsed_fields
        .get("basis")
        .and_then(serde_json::Value::as_array)
        .map(Vec::len)
        .ok_or_else(|| anyhow::anyhow!("CVP bounds parsing requires a prior basis field"))?;

    let (lower, upper) = match raw {
        Some(raw) => {
            let parts: Vec<i64> = util::parse_comma_list(raw)?;
            anyhow::ensure!(
                parts.len() == 2,
                "--bounds expects \"lower,upper\" (e.g., \"-10,10\")"
            );
            (parts[0], parts[1])
        }
        None => (-10, 10),
    };
    let bounds =
        vec![problemreductions::models::algebraic::VarBounds::bounded(lower, upper); basis_len];
    Ok(serde_json::to_value(bounds)?)
}

fn parse_biguint_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<String> = util::parse_biguint_list(raw)?
        .into_iter()
        .map(|value| value.to_string())
        .collect();
    Ok(serde_json::to_value(values)?)
}

fn parse_biguint_value(raw: &str) -> Result<serde_json::Value> {
    let value: BigUint = util::parse_decimal_biguint(raw)?;
    Ok(serde_json::Value::String(value.to_string()))
}

fn parse_optional_bool_list_value(raw: &str) -> Result<serde_json::Value> {
    let values: Vec<Option<bool>> = raw
        .split(',')
        .map(|entry| {
            let entry = entry.trim();
            match entry {
                "?" => Ok(None),
                _ => Ok(Some(parse_bool_token(entry)?)),
            }
        })
        .collect::<Result<_>>()?;
    Ok(serde_json::to_value(values)?)
}

fn parse_quantifiers_raw(raw: &str, context: &CreateContext) -> Result<Vec<Quantifier>> {
    let quantifiers: Vec<Quantifier> = raw
        .split(',')
        .map(|entry| match entry.trim().to_lowercase().as_str() {
            "e" | "exists" => Ok(Quantifier::Exists),
            "a" | "forall" => Ok(Quantifier::ForAll),
            other => Err(anyhow::anyhow!(
                "Invalid quantifier '{}': expected E/Exists or A/ForAll",
                other
            )),
        })
        .collect::<Result<_>>()?;

    if let Some(num_vars) = context.usize_field("num_vars") {
        anyhow::ensure!(
            quantifiers.len() == num_vars,
            "Expected {num_vars} quantifiers but got {}",
            quantifiers.len()
        );
    }

    Ok(quantifiers)
}

fn parse_json_passthrough_value(raw: &str) -> Result<serde_json::Value> {
    serde_json::from_str(raw).context("Invalid JSON input")
}

fn parse_bool_token(raw: &str) -> Result<bool> {
    match raw.trim() {
        "1" | "true" | "TRUE" | "True" => Ok(true),
        "0" | "false" | "FALSE" | "False" => Ok(false),
        other => bail!("Invalid boolean entry '{other}': expected 0/1 or true/false"),
    }
}

fn parse_simple_graph_value(raw: &str, context: &CreateContext) -> Result<serde_json::Value> {
    let raw = raw.trim();
    let num_vertices = context.usize_field("num_vertices").or(context.num_vertices);
    let graph = if raw.is_empty() {
        let num_vertices = num_vertices.ok_or_else(|| {
            anyhow::anyhow!(
                "Empty graph string. To create a graph with isolated vertices, provide num_vertices first."
            )
        })?;
        SimpleGraph::empty(num_vertices)
    } else {
        let edges = util::parse_edge_pairs(raw)?;
        let inferred_num_vertices = edges
            .iter()
            .flat_map(|&(u, v)| [u, v])
            .max()
            .map(|max_vertex| max_vertex + 1)
            .unwrap_or(0);
        let num_vertices = match num_vertices {
            Some(explicit) => {
                anyhow::ensure!(
                    explicit >= inferred_num_vertices,
                    "num_vertices ({explicit}) is too small for the graph: need at least {inferred_num_vertices}"
                );
                explicit
            }
            None => inferred_num_vertices,
        };
        SimpleGraph::new(num_vertices, edges)
    };
    Ok(serde_json::to_value(graph)?)
}

fn parse_directed_graph_value(raw: &str, context: &CreateContext) -> Result<serde_json::Value> {
    let (graph, _) = parse_directed_graph(
        raw,
        context.usize_field("num_vertices").or(context.num_vertices),
    )?;
    Ok(serde_json::to_value(graph)?)
}

fn parse_grid_subgraph_value(raw: &str, kings: bool) -> Result<serde_json::Value> {
    let positions = util::parse_positions::<i32>(raw, "0,0")?;
    if kings {
        Ok(serde_json::to_value(KingsSubgraph::new(positions))?)
    } else {
        Ok(serde_json::to_value(TriangularSubgraph::new(positions))?)
    }
}

fn parse_unit_disk_graph_value(raw: &str, context: &CreateContext) -> Result<serde_json::Value> {
    let positions = util::parse_positions::<f64>(raw, "0.0,0.0")?;
    let radius = context
        .f64_field("radius")
        .ok_or_else(|| anyhow::anyhow!("UnitDiskGraph parsing requires a prior radius field"))?;
    Ok(serde_json::to_value(UnitDiskGraph::new(positions, radius))?)
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
        "VertexCover" => "--graph 0-1,1-2,0-2,2-3 --k 2",
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
            "--graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --max-weight 6"
        }
        "HamiltonianPath" => "--graph 0-1,1-2,2-3",
        "HamiltonianPathBetweenTwoVertices" => {
            "--graph 0-1,0-3,1-2,1-4,2-5,3-4,4-5,2-3 --source-vertex 0 --target-vertex 5"
        }
        "GraphPartitioning" => "--graph 0-1,1-2,2-3,3-0 --num-partitions 2",
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
            "--graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --max-length 4"
        }
        "PathConstrainedNetworkFlow" => {
            "--arcs \"0>1,0>2,1>3,1>4,2>4,3>5,4>5,4>6,5>7,6>7\" --capacities 2,1,1,1,1,1,1,1,2,1 --source 0 --sink 7 --paths \"0,2,5,8;0,3,6,8;0,3,7,9;1,4,6,8;1,4,7,9\" --requirement 3"
        }
        "IsomorphicSpanningTree" => "--graph 0-1,1-2,0-2 --tree 0-1,1-2",
        "BoundedDiameterSpanningTree" => {
            "--graph 0-1,0-2,0-3,1-2,1-4,2-3,3-4 --edge-weights 1,2,1,1,2,1,1 --weight-bound 5 --diameter-bound 3"
        }
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
            "--graph 0-1,1-2,2-3 --potential-weights 0-2:3,0-3:4,1-3:2 --budget 5"
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
        "Maximum2Satisfiability" => "--num-vars 4 --clauses \"1,2;1,-2;-1,3;-1,-3;2,4;-3,-4;3,4\"",
        "NonTautology" => {
            "--num-vars 3 --disjuncts \"1,2,3;-1,-2,-3\""
        }
        "OneInThreeSatisfiability" => {
            "--num-vars 4 --clauses \"1,2,3;-1,3,4;2,-3,-4\""
        }
        "Planar3Satisfiability" => {
            "--num-vars 4 --clauses \"1,2,3;-1,2,4;1,-3,4;-2,3,-4\""
        }
        "QUBO" => "--matrix \"1,0.5;0.5,2\"",
        "QuadraticAssignment" => "--matrix \"0,5;5,0\" --distance-matrix \"0,1;1,0\"",
        "SpinGlass" => "--graph 0-1,1-2 --couplings 1,1",
        "KColoring" => "--graph 0-1,1-2,2-0 --k 3",
        "HamiltonianCircuit" => "--graph 0-1,1-2,2-3,3-0",
        "MaximumLeafSpanningTree" => "--graph 0-1,0-2,0-3,1-4,2-4,2-5,3-5,4-5,1-3",
        "EnsembleComputation" => "--universe-size 4 --subsets \"0,1,2;0,1,3\"",
        "RootedTreeStorageAssignment" => {
            "--universe-size 5 --subsets \"0,2;1,3;0,4;2,4\" --bound 1"
        }
        "MinMaxMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "MinimumSumMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "BalancedCompleteBipartiteSubgraph" => {
            "--left 4 --right 4 --biedges 0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3 --k 3"
        }
        "MaximumAchromaticNumber" => "--graph 0-1,1-2,2-3,3-4,4-5,5-0",
        "MaximumDomaticNumber" => "--graph 0-1,1-2,0-2",
        "MinimumCoveringByCliques" => "--graph 0-1,1-2,0-2,2-3",
        "MinimumIntersectionGraphBasis" => "--graph 0-1,1-2",
        "MinimumMaximalMatching" => "--graph 0-1,1-2,2-3,3-4,4-5",
        "DegreeConstrainedSpanningTree" => "--graph 0-1,0-2,0-3,1-2,1-4,2-3,3-4 --k 2",
        "MonochromaticTriangle" => "--graph 0-1,0-2,0-3,1-2,1-3,2-3",
        "PartitionIntoTriangles" => "--graph 0-1,1-2,0-2",
        "PartitionIntoCliques" => "--graph 0-1,0-2,1-2,3-4,3-5,4-5 --k 3",
        "PartitionIntoForests" => "--graph 0-1,1-2,2-0,3-4,4-5,5-3 --k 2",
        "PartitionIntoPerfectMatchings" => "--graph 0-1,2-3,0-2,1-3 --k 2",
        "Factoring" => "--target 15 --m 4 --n 4",
        "CapacityAssignment" => {
            "--capacities 1,2,3 --cost-matrix \"1,3,6;2,4,7;1,2,5\" --delay-matrix \"8,4,1;7,3,1;6,3,1\" --delay-budget 12"
        }
        "ProductionPlanning" => {
            "--num-periods 6 --demands 5,3,7,2,8,5 --capacities 12,12,12,12,12,12 --setup-costs 10,10,10,10,10,10 --production-costs 1,1,1,1,1,1 --inventory-costs 1,1,1,1,1,1 --cost-bound 80"
        }
        "MultiprocessorScheduling" => "--lengths 4,5,3,2,6 --num-processors 2 --deadline 10",
        "PreemptiveScheduling" => {
            "--lengths 2,1,3,2,1 --num-processors 2 --precedences \"0>2,1>3\""
        }
        "SchedulingToMinimizeWeightedCompletionTime" => {
            "--lengths 1,2,3,4,5 --weights 6,4,3,2,1 --num-processors 2"
        }
        "JobShopScheduling" => {
            "--jobs \"0:3,1:4;1:2,0:3,1:2;0:4,1:3;1:5,0:2;0:2,1:3,0:1\" --num-processors 2"
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
            "--arcs \"0>1,0>2,1>3,1>4,2>4,2>5,3>5,4>5\" --weights 2,3,2,1,3,1 --arc-weights 1,1,1,1,1,1,1,1 --weight-bound 5 --cost-bound 5"
        }
        "OptimalLinearArrangement" => "--graph 0-1,1-2,2-3",
        "RootedTreeArrangement" => "--graph 0-1,0-2,1-2,2-3,3-4 --bound 7",
        "DirectedTwoCommodityIntegralFlow" => {
            "--arcs \"0>2,0>3,1>2,1>3,2>4,2>5,3>4,3>5\" --capacities 1,1,1,1,1,1,1,1 --source-1 0 --sink-1 4 --source-2 1 --sink-2 5 --requirement-1 1 --requirement-2 1"
        }
        "MinimumEdgeCostFlow" => {
            "--arcs \"0>1,0>2,0>3,1>4,2>4,3>4\" --edge-weights 3,1,2,0,0,0 --capacities 2,2,2,2,2,2 --source 0 --sink 4 --requirement 3"
        }
        "MinimumFeedbackArcSet" => "--arcs \"0>1,1>2,2>0\"",
        "DirectedHamiltonianPath" => {
            "--arcs \"0>1,0>3,1>3,1>4,2>0,2>4,3>2,3>5,4>5,5>1\" --num-vertices 6"
        }
        "Kernel" => "--arcs \"0>1,0>2,1>3,2>3,3>4,4>0,4>1\"",
        "MinimumGeometricConnectedDominatingSet" => {
            "--positions \"0,0;3,0;6,0;9,0;0,3;3,3;6,3;9,3\" --radius 3.5"
        }
        "MinimumDummyActivitiesPert" => "--arcs \"0>2,0>3,1>3,1>4,2>5\" --num-vertices 6",
        "FeasibleRegisterAssignment" => {
            "--arcs \"0>1,0>2,1>3\" --assignment 0,1,0,0 --k 2 --num-vertices 4"
        }
        "MinimumFaultDetectionTestSet" => {
            "--arcs \"0>2,0>3,1>3,1>4,2>5,3>5,3>6,4>6\" --inputs 0,1 --outputs 5,6 --num-vertices 7"
        }
        "MinimumWeightAndOrGraph" => {
            "--arcs \"0>1,0>2,1>3,1>4,2>5,2>6\" --source 0 --gate-types \"AND,OR,OR,L,L,L,L\" --weights 1,2,3,1,4,2 --num-vertices 7"
        }
        "MinimumRegisterSufficiencyForLoops" => {
            "--loop-length 6 --loop-variables \"0,3;2,3;4,3\""
        }
        "RegisterSufficiency" => {
            "--arcs \"2>0,2>1,3>1,4>2,4>3,5>0,6>4,6>5\" --bound 3 --num-vertices 7"
        }
        "StrongConnectivityAugmentation" => {
            "--arcs \"0>1,1>2\" --candidate-arcs \"2>0:1\" --bound 1"
        }
        "MixedChinesePostman" => {
            "--graph 0-2,1-3,0-4,4-2 --arcs \"0>1,1>2,2>3,3>0\" --edge-weights 2,3,1,2 --arc-weights 2,3,1,4"
        }
        "RuralPostman" => {
            "--graph 0-1,1-2,2-3,3-0 --edge-weights 1,1,1,1 --required-edges 0,2"
        }
        "StackerCrane" => {
            "--arcs \"0>4,2>5,5>1,3>0,4>3\" --graph \"0-1,1-2,2-3,3-5,4-5,0-3,1-5\" --arc-lengths 3,4,2,5,3 --edge-lengths 2,1,3,2,1,4,3 --num-vertices 6"
        }
        "MultipleChoiceBranching" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --threshold 10"
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
            "--lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
        }
        "IntegerKnapsack" => "--sizes 3,4,5,2,7 --values 4,5,7,3,9 --capacity 15",
        "SubsetProduct" => "--sizes 2,3,5,7,6,10 --target 210",
        "SubsetSum" => "--sizes 3,7,1,8,2,4 --target 11",
        "MinimumAxiomSet" => {
            "--n 8 --true-sentences 0,1,2,3,4,5,6,7 --implications \"0>2;0>3;1>4;1>5;2,4>6;3,5>7;6,7>0;6,7>1\""
        }
        "IntegerExpressionMembership" => {
            "--expression '{\"Sum\":[{\"Sum\":[{\"Union\":[{\"Atom\":1},{\"Atom\":4}]},{\"Union\":[{\"Atom\":3},{\"Atom\":6}]}]},{\"Union\":[{\"Atom\":2},{\"Atom\":5}]}]}' --target 12"
        }
        "NonLivenessFreePetriNet" => {
            "--n 4 --m 3 --arcs \"0>0,1>1,2>2\" --output-arcs \"0>1,1>2,2>3\" --initial-marking 1,0,0,0"
        }
        "Betweenness" => "--n 5 --sets \"0,1,2;2,3,4;0,2,4;1,3,4\"",
        "CyclicOrdering" => "--n 5 --sets \"0,1,2;2,3,0;1,3,4\"",
        "Numerical3DimensionalMatching" => "--w-sizes 4,5 --x-sizes 4,5 --y-sizes 5,7 --bound 15",
        "ThreePartition" => "--sizes 4,5,6,4,6,5 --bound 15",
        "DynamicStorageAllocation" => "--release-times 0,0,1,2,3 --deadlines 3,2,4,5,5 --sizes 2,3,1,3,2 --capacity 6",
        "KthLargestMTuple" => "--sets \"2,5,8;3,6;1,4,7\" --k 14 --bound 12",
        "AlgebraicEquationsOverGF2" => "--num-vars 3 --equations \"0,1:2;1,2:0:;0:1:2:\"",
        "QuadraticCongruences" => "--coeff-a 4 --coeff-b 15 --coeff-c 10",
        "QuadraticDiophantineEquations" => "--coeff-a 3 --coeff-b 5 --coeff-c 53",
        "SimultaneousIncongruences" => "--pairs \"2,2;1,3;2,5;3,7\"",
        "BoyceCoddNormalFormViolation" => {
            "--n 6 --sets \"0,1:2;2:3;3,4:5\" --target 0,1,2,3,4,5"
        }
        "Clustering" => {
            "--distance-matrix \"0,1,1,3;1,0,1,3;1,1,0,3;3,3,3,0\" --k 2 --diameter-bound 1"
        }
        "SumOfSquaresPartition" => "--sizes 5,3,8,2,7,1 --num-groups 3",
        "ComparativeContainment" => {
            "--universe-size 4 --r-sets \"0,1,2,3;0,1\" --s-sets \"0,1,2,3;2,3\" --r-weights 2,5 --s-weights 3,6"
        }
        "SetBasis" => "--universe-size 4 --subsets \"0,1;1,2;0,2;0,1,2\" --k 3",
        "SetSplitting" => "--universe-size 6 --subsets \"0,1,2;2,3,4;0,4,5;1,3,5\"",
        "LongestCommonSubsequence" => {
            "--strings \"010110;100101;001011\" --alphabet-size 2"
        }
        "GroupingBySwapping" => "--string \"0,1,2,0,1,2\" --bound 5",
        "MinimumExternalMacroDataCompression" | "MinimumInternalMacroDataCompression" => {
            "--string \"0,1,0,1\" --pointer-cost 2 --alphabet-size 2"
        }
        "MinimumCardinalityKey" => {
            "--num-attributes 6 --dependencies \"0,1>2;0,2>3;1,3>4;2,4>5\""
        }
        "PrimeAttributeName" => {
            "--universe 6 --dependencies \"0,1>2,3,4,5;2,3>0,1,4,5\" --query-attribute 3"
        }
        "TwoDimensionalConsecutiveSets" => {
            "--alphabet-size 6 --subsets \"0,1,2;3,4,5;1,3;2,4;0,5\""
        }
        "ShortestCommonSupersequence" => "--strings \"0,1,2;1,2,0\"",
        "ConsecutiveBlockMinimization" => "--matrix '[[true,false,true],[false,true,true]]' --bound-k 2",
        "ConsecutiveOnesMatrixAugmentation" => {
            "--matrix \"1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0\" --bound 2"
        }
        "SparseMatrixCompression" => "--matrix \"1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0\" --bound-k 2",
        "MaximumLikelihoodRanking" => "--matrix \"0,4,3,5;1,0,4,3;2,1,0,4;0,2,1,0\"",
        "MinimumMatrixCover" => "--matrix \"0,3,1,0;3,0,0,2;1,0,0,4;0,2,4,0\"",
        "MinimumMatrixDomination" => "--matrix \"0,1,0;1,0,1;0,1,0\"",
        "MinimumWeightDecoding" => {
            "--matrix '[[true,false,true,true],[false,true,true,false],[true,true,false,true]]' --rhs 'true,true,false'"
        }
        "MinimumWeightSolutionToLinearEquations" => {
            "--matrix '[[1,2,3,1],[2,1,1,3]]' --rhs '5,4'"
        }
        "ConjunctiveBooleanQuery" => {
            "--domain-size 6 --relations \"2:0,3|1,3|2,4;3:0,1,5|1,2,5\" --conjuncts-spec \"0:v0,c3;0:v1,c3;1:v0,v1,c5\""
        }
        "ConjunctiveQueryFoldability" => "(use --example ConjunctiveQueryFoldability)",
        "EquilibriumPoint" => "(use --example EquilibriumPoint)",
        "SequencingToMinimizeMaximumCumulativeCost" => {
            "--costs 2,-1,3,-2,1,-3 --precedence-pairs \"0>2,1>2,1>3,2>4,3>5,4>5\""
        }
        "StringToStringCorrection" => {
            "--source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2"
        }
        "FeasibleBasisExtension" => {
            "--matrix '[[1,0,1,2,-1,0],[0,1,0,1,1,2],[0,0,1,1,0,1]]' --rhs '7,5,3' --required-columns '0,1'"
        }
        "MinimumCodeGenerationParallelAssignments" => {
            "--num-variables 4 --assignments \"0:1,2;1:0;2:3;3:1,2\""
        }
        "MinimumDecisionTree" => {
            "--test-matrix '[[true,true,false,false],[true,false,false,false],[false,true,false,true]]' --num-objects 4 --num-tests 3"
        }
        "MinimumDisjunctiveNormalForm" => {
            "--num-vars 3 --truth-table 0,1,1,1,1,1,1,0"
        }
        "SquareTiling" => {
            "--num-colors 3 --tiles \"0,1,2,0;0,0,2,1;2,1,0,0;2,0,0,1\" --grid-size 2"
        }
        _ => "",
    }
}

fn uses_edge_weights_flag(canonical: &str) -> bool {
    matches!(
        canonical,
        "BottleneckTravelingSalesman"
            | "BoundedDiameterSpanningTree"
            | "KthBestSpanningTree"
            | "LongestCircuit"
            | "MaxCut"
            | "MaximumMatching"
            | "MixedChinesePostman"
            | "RuralPostman"
            | "TravelingSalesman"
    )
}

fn uses_edge_weights_flag_for_edge_lengths(canonical: &str) -> bool {
    matches!(
        canonical,
        "LongestCircuit" | "MinMaxMulticenter" | "MinimumSumMulticenter"
    )
}

fn help_flag_name(canonical: &str, field_name: &str) -> String {
    // Problem-specific overrides first
    match (canonical, field_name) {
        ("BoundedComponentSpanningForest", "max_components") => return "k".to_string(),
        ("BoundedComponentSpanningForest", "max_weight") => return "max-weight".to_string(),
        ("BoyceCoddNormalFormViolation", "num_attributes") => return "n".to_string(),
        ("BoyceCoddNormalFormViolation", "functional_deps") => return "sets".to_string(),
        ("BoyceCoddNormalFormViolation", "target_subset") => return "target".to_string(),
        ("CapacityAssignment", "cost") => return "cost-matrix".to_string(),
        ("CapacityAssignment", "delay") => return "delay-matrix".to_string(),
        ("FlowShopScheduling", "num_processors")
        | ("JobShopScheduling", "num_processors")
        | ("OpenShopScheduling", "num_machines")
        | ("SchedulingWithIndividualDeadlines", "num_processors") => {
            return "num-processors/--m".to_string();
        }
        ("JobShopScheduling", "jobs") => return "jobs".to_string(),
        ("LengthBoundedDisjointPaths", "max_length") => return "max-length".to_string(),
        ("ConsecutiveBlockMinimization", "bound") => return "bound-k".to_string(),
        ("GroupingBySwapping", "budget") => return "bound".to_string(),
        ("RectilinearPictureCompression", "bound") => return "bound".to_string(),
        ("PrimeAttributeName", "num_attributes") => return "universe".to_string(),
        ("PrimeAttributeName", "dependencies") => return "dependencies".to_string(),
        ("PrimeAttributeName", "query_attribute") => return "query-attribute".to_string(),
        ("ClosestVectorProblem", "target") => return "target-vec".to_string(),
        ("ConjunctiveBooleanQuery", "conjuncts") => return "conjuncts-spec".to_string(),
        ("MixedChinesePostman", "arc_weights") => return "arc-weights".to_string(),
        ("ConsecutiveOnesMatrixAugmentation", "bound") => return "bound".to_string(),
        ("ConsecutiveOnesSubmatrix", "bound") => return "bound".to_string(),
        ("SparseMatrixCompression", "bound_k") => return "bound-k".to_string(),
        ("MinimumCodeGenerationParallelAssignments", "num_variables") => {
            return "num-variables".to_string();
        }
        ("MinimumCodeGenerationParallelAssignments", "assignments") => {
            return "assignments".to_string();
        }
        ("StackerCrane", "edges") => return "graph".to_string(),
        ("StackerCrane", "arc_lengths") => return "arc-lengths".to_string(),
        ("StackerCrane", "edge_lengths") => return "edge-lengths".to_string(),
        ("StaffScheduling", "shifts_per_schedule") => return "k".to_string(),
        ("TimetableDesign", "num_tasks") => return "num-tasks".to_string(),
        _ => {}
    }
    // Edge-weight problems use --edge-weights instead of --weights
    if field_name == "weights" && uses_edge_weights_flag(canonical) {
        return "edge-weights".to_string();
    }
    if field_name == "edge_lengths" && uses_edge_weights_flag_for_edge_lengths(canonical) {
        return "edge-weights".to_string();
    }
    // General field-name overrides (previously in cli_flag_name)
    match field_name {
        "universe_size" => "universe-size".to_string(),
        "collection" | "subsets" => "subsets".to_string(),
        "left_size" => "left".to_string(),
        "right_size" => "right".to_string(),
        "edges" => "biedges".to_string(),
        "vertex_weights" => "weights".to_string(),
        "potential_weights" => "potential-weights".to_string(),
        "num_tasks" => "num-tasks".to_string(),
        "precedences" => "precedences".to_string(),
        "threshold" => "threshold".to_string(),
        "lengths" => "lengths".to_string(),
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
        ("DynamicStorageAllocation", "release_times") => "comma-separated arrival times: 0,0,1,2,3",
        ("DynamicStorageAllocation", "deadlines") => "comma-separated departure times: 3,2,4,5,5",
        ("DynamicStorageAllocation", "sizes") => "comma-separated item sizes: 2,3,1,3,2",
        ("DynamicStorageAllocation", "capacity") => "memory size D: 6",
        ("DisjointConnectingPaths", "terminal_pairs") => "comma-separated pairs: 0-3,2-5",
        ("PrimeAttributeName", "dependencies") => {
            "semicolon-separated dependencies: \"0,1>2,3;2,3>0,1\""
        }
        ("LongestCommonSubsequence", "strings") => {
            "raw strings: \"ABAC;BACA\" or symbol lists: \"0,1,0;1,0,1\""
        }
        ("GroupingBySwapping", "string") => "symbol list: \"0,1,2,0,1,2\"",
        ("MinimumExternalMacroDataCompression", "string")
        | ("MinimumInternalMacroDataCompression", "string") => "symbol list: \"0,1,0,1\"",
        ("MinimumExternalMacroDataCompression", "pointer_cost")
        | ("MinimumInternalMacroDataCompression", "pointer_cost") => "positive integer: 2",
        ("MinimumAxiomSet", "num_sentences") => "total number of sentences: 8",
        ("MinimumAxiomSet", "true_sentences") => "comma-separated indices: 0,1,2,3,4,5,6,7",
        ("MinimumAxiomSet", "implications") => "semicolon-separated rules: \"0>2;0>3;1>4;2,4>6\"",
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
        ("ConsecutiveBlockMinimization", "matrix") => {
            "JSON 2D bool array: '[[true,false,true],[false,true,true]]'"
        }
        ("ConsecutiveOnesMatrixAugmentation", "matrix") => {
            "semicolon-separated 0/1 rows: \"1,0;0,1\""
        }
        ("ConsecutiveOnesSubmatrix", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("SparseMatrixCompression", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("MaximumLikelihoodRanking", "matrix") => {
            "semicolon-separated i32 rows: \"0,4,3,5;1,0,4,3;2,1,0,4;0,2,1,0\""
        }
        ("MinimumMatrixCover", "matrix") => "semicolon-separated i64 rows: \"0,3,1;3,0,2;1,2,0\"",
        ("MinimumMatrixDomination", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        ("MinimumWeightDecoding", "matrix") => "JSON 2D bool array: '[[true,false],[false,true]]'",
        ("MinimumWeightDecoding", "target") => "comma-separated booleans: \"true,true,false\"",
        ("MinimumWeightSolutionToLinearEquations", "matrix") => {
            "JSON 2D integer array: '[[1,2,3],[4,5,6]]'"
        }
        ("MinimumWeightSolutionToLinearEquations", "rhs") => "comma-separated integers: \"5,4\"",
        ("FeasibleBasisExtension", "matrix") => "JSON 2D integer array: '[[1,0,1],[0,1,0]]'",
        ("FeasibleBasisExtension", "rhs") => "comma-separated integers: \"7,5,3\"",
        ("FeasibleBasisExtension", "required_columns") => "comma-separated column indices: \"0,1\"",
        ("MinimumCodeGenerationParallelAssignments", "assignments") => {
            "semicolon-separated target:reads entries: \"0:1,2;1:0;2:3;3:1,2\""
        }
        ("NonTautology", "disjuncts") => "semicolon-separated disjuncts: \"1,2,3;-1,-2,-3\"",
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

fn validate_prescribed_paths_against_graph(
    graph: &DirectedGraph,
    paths: &[Vec<usize>],
    source: usize,
    sink: usize,
    usage: &str,
) -> Result<()> {
    let arcs = graph.arcs();
    for path in paths {
        anyhow::ensure!(
            !path.is_empty(),
            "PathConstrainedNetworkFlow paths must be non-empty\n\n{usage}"
        );
        let mut visited_vertices = BTreeSet::from([source]);
        let mut current = source;
        for &arc_index in path {
            let &(tail, head) = arcs.get(arc_index).ok_or_else(|| {
                anyhow::anyhow!(
                    "Path arc index {arc_index} out of bounds for {} arcs\n\n{usage}",
                    arcs.len()
                )
            })?;
            anyhow::ensure!(
                tail == current,
                "prescribed path is not contiguous: expected arc leaving vertex {current}, got {tail}->{head}\n\n{usage}"
            );
            anyhow::ensure!(
                visited_vertices.insert(head),
                "prescribed path repeats vertex {head}, so it is not a simple path\n\n{usage}"
            );
            current = head;
        }
        anyhow::ensure!(
            current == sink,
            "prescribed path must end at sink {sink}, ended at {current}\n\n{usage}"
        );
    }
    Ok(())
}

fn validate_schema_driven_semantics(
    args: &CreateArgs,
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
    _data: &serde_json::Value,
) -> Result<()> {
    match canonical {
        "BalancedCompleteBipartiteSubgraph" => {
            let usage = "pred create BalancedCompleteBipartiteSubgraph --left 4 --right 4 --biedges 0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3 --k 3";
            let _ = parse_bipartite_problem_input(
                args,
                "BalancedCompleteBipartiteSubgraph",
                "balanced biclique size",
                usage,
            )?;
        }
        "BiconnectivityAugmentation" => {
            let usage = "Usage: pred create BiconnectivityAugmentation --graph 0-1,1-2,2-3 --potential-weights 0-2:3,0-3:4,1-3:2 --budget 5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let potential_edges = parse_potential_edges(args)?;
            validate_potential_edges(&graph, &potential_edges)?;
            let _ = parse_budget(args)?;
        }
        "BoundedComponentSpanningForest" => {
            let usage = "Usage: pred create BoundedComponentSpanningForest --graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --max-weight 6";
            let (_, n) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
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
                anyhow::anyhow!("BoundedComponentSpanningForest requires --max-weight\n\n{usage}")
            })?;
            if bound_raw <= 0 {
                bail!("BoundedComponentSpanningForest requires positive --max-weight\n\n{usage}");
            }
            let _ = i32::try_from(bound_raw).map_err(|_| {
                anyhow::anyhow!(
                    "BoundedComponentSpanningForest requires --max-weight within i32 range\n\n{usage}"
                )
            })?;
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
            let _ = args.delay_budget.ok_or_else(|| {
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
        }
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
            let _ = parse_bcnf_functional_deps(sets_str, n)?;
            let target: Vec<usize> = util::parse_comma_list(target_str)?;
            ensure_attribute_indices_in_range(&target, n, "Target subset")?;
        }
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
            let basis: Vec<Vec<f64>> = basis_str
                .split(';')
                .map(|row| util::parse_comma_list(row.trim()))
                .collect::<Result<Vec<_>>>()?;
            let target: Vec<f64> = util::parse_comma_list(target_str)?;
            let n = basis.len();
            let bounds = serde_json::from_value(parse_cvp_bounds_value(
                args.bounds.as_deref(),
                &CreateContext {
                    num_vertices: None,
                    num_edges: None,
                    num_arcs: None,
                    parsed_fields: BTreeMap::from([(
                        "basis".to_string(),
                        serde_json::json!(vec![serde_json::json!([0]); n]),
                    )]),
                },
            )?)?;
            let _ = ClosestVectorProblem::new(basis, target, bounds);
        }
        "ConsecutiveOnesMatrixAugmentation" => {
            let matrix = parse_bool_matrix(args)?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveOnesMatrixAugmentation requires --matrix and --bound\n\n\
                     Usage: pred create ConsecutiveOnesMatrixAugmentation --matrix \"1,0,0,1,1;1,1,0,0,0;0,1,1,0,1;0,0,1,1,0\" --bound 2"
                )
            })?;
            ConsecutiveOnesMatrixAugmentation::try_new(matrix, bound)
                .map_err(anyhow::Error::msg)?;
        }
        "ConsecutiveBlockMinimization" => {
            let usage = "Usage: pred create ConsecutiveBlockMinimization --matrix '[[true,false,true],[false,true,true]]' --bound-k 2";
            let matrix_str = args.matrix.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array and --bound-k\n\n{usage}"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("ConsecutiveBlockMinimization requires --bound-k\n\n{usage}")
            })?;
            let matrix: Vec<Vec<bool>> = serde_json::from_str(matrix_str).map_err(|err| {
                anyhow::anyhow!(
                    "ConsecutiveBlockMinimization requires --matrix as a JSON 2D bool array (e.g., '[[true,false,true],[false,true,true]]')\n\n{usage}\n\nFailed to parse --matrix: {err}"
                )
            })?;
            ConsecutiveBlockMinimization::try_new(matrix, bound)
                .map_err(|err| anyhow::anyhow!("{err}\n\n{usage}"))?;
        }
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
            match resolved_variant.get("weight").map(|value| value.as_str()) {
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
                    anyhow::ensure!(
                        r_weights.iter().all(|&w| w == 1) && s_weights.iter().all(|&w| w == 1),
                        "Non-unit weights are not supported for ComparativeContainment/One.\n\n\
                         Use `pred create ComparativeContainment/i32 ... --r-weights ... --s-weights ...` for weighted instances."
                    );
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
                }
                Some(other) => bail!(
                    "Unsupported ComparativeContainment weight variant: {}",
                    other
                ),
            }
        }
        "DisjointConnectingPaths" => {
            let usage =
                "Usage: pred create DisjointConnectingPaths --graph 0-1,1-3,0-2,1-4,2-4,3-5,4-5 --terminal-pairs 0-3,2-5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_terminal_pairs(args, graph.num_vertices())
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
        }
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
        }
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
            anyhow::ensure!(
                source != sink,
                "GeneralizedHex requires distinct --source and --sink\n\n{usage}"
            );
        }
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
            let string = parse_symbol_list_allow_empty(string_str)?;
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
        }
        "IntegralFlowBundles" => {
            let usage = "Usage: pred create IntegralFlowBundles --arcs \"0>1,0>2,1>3,2>3,1>2,2>1\" --bundles \"0,1;2,5;3,4\" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4";
            let arcs_str = args
                .arcs
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --arcs\n\n{usage}"))?;
            let (graph, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let bundles = parse_bundles(args, num_arcs, usage)?;
            let _ = parse_bundle_capacities(args, bundles.len(), usage)?;
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowBundles requires --source\n\n{usage}")
            })?;
            let sink = args
                .sink
                .ok_or_else(|| anyhow::anyhow!("IntegralFlowBundles requires --sink\n\n{usage}"))?;
            let _ = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowBundles requires --requirement\n\n{usage}")
            })?;
            validate_vertex_index("source", source, graph.num_vertices(), usage)?;
            validate_vertex_index("sink", sink, graph.num_vertices(), usage)?;
            anyhow::ensure!(
                source != sink,
                "IntegralFlowBundles requires distinct --source and --sink\n\n{usage}"
            );
        }
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
            let _ = args.requirement.ok_or_else(|| {
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
        }
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
            let _ = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("IntegralFlowWithMultipliers requires --requirement\n\n{usage}")
            })?;
        }
        "JobShopScheduling" => {
            let usage = "Usage: pred create JobShopScheduling --jobs \"0:3,1:4;1:2,0:3,1:2;0:4,1:3\" --num-processors 2";
            let job_tasks = args
                .job_tasks
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("JobShopScheduling requires --jobs\n\n{usage}"))?;
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
        }
        "KClique" => {
            let usage = "Usage: pred create KClique --graph 0-1,0-2,1-3,2-3,2-4,3-4 --k 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_kclique_threshold(args.k, graph.num_vertices(), usage)?;
        }
        "KColoring" => {
            let usage = "Usage: pred create KColoring --graph 0-1,1-2,2-0 --k 3";
            let _ = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = util::validate_k_param(&resolved_variant, args.k, None, "KColoring")
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
        }
        "KthBestSpanningTree" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let usage =
                "Usage: pred create KthBestSpanningTree --graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_edge_weights(args, graph.num_edges())?;
            let _ = util::validate_k_param(&resolved_variant, args.k, None, "KthBestSpanningTree")
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = args
                .bound
                .ok_or_else(|| anyhow::anyhow!("KthBestSpanningTree requires --bound\n\n{usage}"))?
                as i32;
        }
        "LengthBoundedDisjointPaths" => {
            let usage = "Usage: pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --max-length 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --sink\n\n{usage}")
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --max-length\n\n{usage}")
            })?;
            let _ = validate_length_bounded_disjoint_paths_args(
                graph.num_vertices(),
                source,
                sink,
                bound,
                Some(usage),
            )?;
        }
        "LongestCommonSubsequence" => {
            let usage =
                "Usage: pred create LCS --strings \"010110;100101;001011\" [--alphabet-size 2]";
            let strings_str = args.strings.as_deref().ok_or_else(|| {
                anyhow::anyhow!("LongestCommonSubsequence requires --strings\n\n{usage}")
            })?;
            let (strings, inferred_alphabet_size) = parse_lcs_strings(strings_str)?;
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
        }
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
        }
        "MixedChinesePostman" => {
            let usage = "Usage: pred create MixedChinesePostman --graph 0-2,1-3,0-4,4-2 --arcs \"0>1,1>2,2>3,3>0\" --edge-weights 2,3,1,2 --arc-weights 2,3,1,4 [--num-vertices N]";
            let graph = parse_mixed_graph(args, usage)?;
            let arc_costs = parse_arc_costs(args, graph.num_arcs())?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            if arc_costs.iter().any(|&cost| cost < 0) {
                bail!("MixedChinesePostman --arc-weights must be non-negative\n\n{usage}");
            }
            if edge_weights.iter().any(|&weight| weight < 0) {
                bail!("MixedChinesePostman --edge-weights must be non-negative\n\n{usage}");
            }
            if resolved_variant.get("weight").map(String::as_str) == Some("One")
                && (arc_costs.iter().any(|&cost| cost != 1)
                    || edge_weights.iter().any(|&weight| weight != 1))
            {
                bail!(
                    "Non-unit lengths are not supported for MixedChinesePostman/One.\n\n\
                     Use the weighted variant instead:\n  pred create MixedChinesePostman/i32 --graph ... --arcs ... --edge-weights ... --arc-weights ..."
                );
            }
        }
        "MinMaxMulticenter" => {
            let usage = "Usage: pred create MinMaxMulticenter --graph 0-1,1-2,2-3 [--weights 1,1,1,1] [--edge-weights 1,1,1] --k 2";
            let (graph, n) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let vertex_weights = parse_vertex_weights(args, n)?;
            let edge_lengths = parse_edge_weights(args, graph.num_edges())?;
            let _ = args.k.ok_or_else(|| {
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
        }
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet"
        | "MaximalIS" => {
            let graph_type = resolved_graph_type(resolved_variant);
            let num_vertices = match graph_type {
                "KingsSubgraph" | "TriangularSubgraph" => parse_int_positions(args)?.len(),
                "UnitDiskGraph" => parse_float_positions(args)?.len(),
                _ => {
                    parse_graph(args)
                        .map_err(|e| {
                            anyhow::anyhow!(
                            "{e}\n\nUsage: pred create {} --graph 0-1,1-2,2-3 [--weights 1,1,1,1]",
                            canonical
                        )
                        })?
                        .1
                }
            };
            let weights = parse_vertex_weights(args, num_vertices)?;
            reject_nonunit_weights_for_one_variant(
                canonical,
                graph_type,
                resolved_variant,
                &weights,
            )?;
        }
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
        }
        "MinimumDummyActivitiesPert" => {
            let usage = "Usage: pred create MinimumDummyActivitiesPert --arcs \"0>2,0>3,1>3,1>4,2>5\" [--num-vertices N]";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("MinimumDummyActivitiesPert requires --arcs\n\n{usage}")
            })?;
            let (graph, _) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let _ = MinimumDummyActivitiesPert::try_new(graph).map_err(anyhow::Error::msg)?;
        }
        "MinimumMultiwayCut" => {
            let usage =
                "Usage: pred create MinimumMultiwayCut --graph 0-1,1-2,2-3 --terminals 0,2 [--edge-weights 1,1,1]";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_terminals(args, graph.num_vertices())?;
            let _ = parse_edge_weights(args, graph.num_edges())?;
        }
        "MultipleChoiceBranching" => {
            let usage = "Usage: pred create MultipleChoiceBranching/i32 --arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --threshold 10";
            let arcs_str = args.arcs.as_deref().ok_or_else(|| {
                anyhow::anyhow!("MultipleChoiceBranching requires --arcs\n\n{usage}")
            })?;
            let (_, num_arcs) = parse_directed_graph(arcs_str, args.num_vertices)?;
            let _ = parse_arc_weights(args, num_arcs)?;
            let _ = parse_partition_groups(args, num_arcs)?;
            let _ = parse_multiple_choice_branching_threshold(args, usage)?;
        }
        "MultipleCopyFileAllocation" => {
            let (_, num_vertices) = parse_graph(args)
                .map_err(|e| anyhow::anyhow!("{e}\n\n{MULTIPLE_COPY_FILE_ALLOCATION_USAGE}"))?;
            let _ = parse_vertex_i64_values(
                args.usage.as_deref(),
                "usage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?;
            let _ = parse_vertex_i64_values(
                args.storage.as_deref(),
                "storage",
                num_vertices,
                "MultipleCopyFileAllocation",
                MULTIPLE_COPY_FILE_ALLOCATION_USAGE,
            )?;
        }
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
            anyhow::ensure!(
                num_processors > 0,
                "MultiprocessorScheduling requires --num-processors > 0\n\n{usage}"
            );
            let _ = args.deadline.ok_or_else(|| {
                anyhow::anyhow!("MultiprocessorScheduling requires --deadline\n\n{usage}")
            })?;
            let _: Vec<u64> = util::parse_comma_list(lengths_str)?;
        }
        "PartialFeedbackEdgeSet" => {
            let usage = "Usage: pred create PartialFeedbackEdgeSet --graph 0-1,1-2,2-0,2-3,3-4,4-2,3-5,5-4,0-3 --budget 3 --max-cycle-length 4";
            let _ = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = args
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
            let _ = args.max_cycle_length.ok_or_else(|| {
                anyhow::anyhow!("PartialFeedbackEdgeSet requires --max-cycle-length\n\n{usage}")
            })?;
        }
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
            let _ = args.requirement.ok_or_else(|| {
                anyhow::anyhow!("PathConstrainedNetworkFlow requires --requirement\n\n{usage}")
            })?;
            let paths = parse_prescribed_paths(args, num_arcs, usage)?;
            validate_prescribed_paths_against_graph(&graph, &paths, source, sink, usage)?;
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
            let _ = args.cost_bound.ok_or_else(|| {
                anyhow::anyhow!("ProductionPlanning requires --cost-bound\n\n{usage}")
            })?;

            for (flag, len) in [
                ("--demands", demands.len()),
                ("--capacities", capacities.len()),
                ("--setup-costs", setup_costs.len()),
                ("--production-costs", production_costs.len()),
                ("--inventory-costs", inventory_costs.len()),
            ] {
                ensure_named_len(len, num_periods, flag, usage)?;
            }
        }
        "SchedulingWithIndividualDeadlines" => {
            let usage = "Usage: pred create SchedulingWithIndividualDeadlines --num-tasks 7 --deadlines 2,1,2,2,3,3,2 [--num-processors 3 | --m 3] [--precedences \"0>3,1>3,1>4,2>4,2>5\"]";
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --deadlines, --num-tasks, and a processor count (--num-processors or --m)\n\n{usage}"
                )
            })?;
            let num_tasks = args.num_tasks.or(args.n).ok_or_else(|| {
                anyhow::anyhow!(
                    "SchedulingWithIndividualDeadlines requires --num-tasks (number of tasks)\n\n{usage}"
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
            let precedences = parse_precedence_pairs(
                args.precedences
                    .as_deref()
                    .or(args.precedence_pairs.as_deref()),
            )?;
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
            let _ = SchedulingWithIndividualDeadlines::new(
                num_tasks,
                num_processors,
                deadlines,
                precedences,
            );
        }
        "StringToStringCorrection" => {
            let usage = "Usage: pred create StringToStringCorrection --source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2";
            let source_str = args.source_string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("StringToStringCorrection requires --source-string\n\n{usage}")
            })?;
            let target_str = args.target_string.as_deref().ok_or_else(|| {
                anyhow::anyhow!("StringToStringCorrection requires --target-string\n\n{usage}")
            })?;
            let _ = parse_nonnegative_usize_bound(
                args.bound.ok_or_else(|| {
                    anyhow::anyhow!("StringToStringCorrection requires --bound\n\n{usage}")
                })?,
                "StringToStringCorrection",
                usage,
            )?;
            let source = parse_symbol_list_allow_empty(source_str)?;
            let target = parse_symbol_list_allow_empty(target_str)?;
            let inferred = source
                .iter()
                .chain(target.iter())
                .copied()
                .max()
                .map_or(0, |m| m + 1);
            let alphabet_size = args.alphabet_size.unwrap_or(inferred);
            anyhow::ensure!(
                alphabet_size >= inferred,
                "--alphabet-size {} is smaller than max symbol + 1 ({}) in the strings",
                alphabet_size,
                inferred
            );
        }
        "SparseMatrixCompression" => {
            let matrix = parse_bool_matrix(args)?;
            let usage = "Usage: pred create SparseMatrixCompression --matrix \"1,0,0,1;0,1,0,0;0,0,1,0;1,0,0,0\" --bound-k 2";
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "SparseMatrixCompression requires --matrix and --bound-k\n\n{usage}"
                )
            })?;
            let bound = parse_nonnegative_usize_bound(bound, "SparseMatrixCompression", usage)?;
            if bound == 0 {
                anyhow::bail!("SparseMatrixCompression requires bound >= 1\n\n{usage}");
            }
            let _ = SparseMatrixCompression::new(matrix, bound);
        }
        "StackerCrane" => {
            let usage = "Usage: pred create StackerCrane --arcs \"0>4,2>5,5>1,3>0,4>3\" --graph \"0-1,1-2,2-3,3-5,4-5,0-3,1-5\" --arc-lengths 3,4,2,5,3 --edge-lengths 2,1,3,2,1,4,3 --num-vertices 6";
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
            let _ = problemreductions::models::misc::StackerCrane::try_new(
                num_vertices,
                arcs_graph.arcs(),
                edges_graph.edges(),
                arc_lengths,
                edge_lengths,
            )
            .map_err(|e| anyhow::anyhow!(e))?;
        }
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
            let _ = ThreePartition::try_new(sizes, bound).map_err(anyhow::Error::msg)?;
        }
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
            let _ = UndirectedFlowLowerBounds::new(
                graph,
                capacities,
                lower_bounds,
                source,
                sink,
                requirement,
            );
        }
        "SequencingToMinimizeMaximumCumulativeCost" => {
            let costs_str = args.costs.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeMaximumCumulativeCost requires --costs\n\n\
                     Usage: pred create SequencingToMinimizeMaximumCumulativeCost --costs 2,-1,3,-2,1,-3 --precedences \"0>2,1>2,1>3,2>4,3>5,4>5\""
                )
            })?;
            let costs: Vec<i64> = util::parse_comma_list(costs_str)?;
            let precedences = parse_precedence_pairs(
                args.precedences
                    .as_deref()
                    .or(args.precedence_pairs.as_deref()),
            )?;
            validate_precedence_pairs(&precedences, costs.len())?;
        }
        "SequencingToMinimizeWeightedTardiness" => {
            let lengths_str = args.lengths.as_deref().or(args.sizes.as_deref()).ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --lengths, --weights, --deadlines, and --bound\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let weights_str = args.weights.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --weights (comma-separated tardiness weights)\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let deadlines_str = args.deadlines.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --deadlines (comma-separated job deadlines)\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "SequencingToMinimizeWeightedTardiness requires --bound\n\n\
                     Usage: pred create SequencingToMinimizeWeightedTardiness --lengths 3,4,2,5,3 --weights 2,3,1,4,2 --deadlines 5,8,4,15,10 --bound 13"
                )
            })?;
            anyhow::ensure!(bound >= 0, "--bound must be non-negative");
            let lengths: Vec<u64> = util::parse_comma_list(lengths_str)?;
            let weights: Vec<u64> = util::parse_comma_list(weights_str)?;
            let deadlines: Vec<u64> = util::parse_comma_list(deadlines_str)?;
            anyhow::ensure!(
                lengths.len() == weights.len(),
                "lengths length ({}) must equal weights length ({})",
                lengths.len(),
                weights.len()
            );
            anyhow::ensure!(
                lengths.len() == deadlines.len(),
                "lengths length ({}) must equal deadlines length ({})",
                lengths.len(),
                deadlines.len()
            );
        }
        "SequencingWithinIntervals" => {
            let usage =
                "Usage: pred create SequencingWithinIntervals --release-times 0,0,5 --deadlines 11,11,6 --lengths 3,1,1";
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
        }
        "SetBasis" => {
            let universe = args.universe.ok_or_else(|| {
                anyhow::anyhow!(
                    "SetBasis requires --universe, --sets, and --k\n\n\
                     Usage: pred create SetBasis --universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3"
                )
            })?;
            let _ = args.k.ok_or_else(|| {
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
        }
        "ShortestWeightConstrainedPath" => {
            let usage = "Usage: pred create ShortestWeightConstrainedPath --graph 0-1,0-2,1-3,2-3,2-4,3-5,4-5,1-4 --edge-lengths 2,4,3,1,5,4,2,6 --edge-weights 5,1,2,3,2,3,1,1 --source-vertex 0 --target-vertex 5 --weight-bound 8";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
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
        }
        "SteinerTree" => {
            let usage = "Usage: pred create SteinerTree --graph 0-1,1-2,1-3,3-4 --edge-weights 2,2,1,1 --terminals 0,2,4";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let _ = parse_edge_weights(args, graph.num_edges())?;
            let _ = parse_terminals(args, graph.num_vertices())
                .map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
        }
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
        }
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
            let _ = args.requirement_1.ok_or_else(|| {
                anyhow::anyhow!(
                    "UndirectedTwoCommodityIntegralFlow requires --requirement-1\n\n{usage}"
                )
            })?;
            let _ = args.requirement_2.ok_or_else(|| {
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
        }
        _ => {}
    }

    Ok(())
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

fn print_problem_help(canonical: &str, resolved_variant: &BTreeMap<String, String>) -> Result<()> {
    let graph_type = resolved_variant
        .get("graph")
        .map(String::as_str)
        .filter(|graph_type| *graph_type != "SimpleGraph");
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
        if canonical == "GraphPartitioning" {
            eprintln!(
                "  --{:<16} Number of partitions in the balanced partitioning model (must be 2) (integer)",
                "num-partitions"
            );
        }
    } else {
        bail!("{}", crate::problem_name::unknown_problem_error(canonical));
    }

    let example = schema_help_example_for(canonical, resolved_variant).or_else(|| {
        let fallback = example_for(canonical, graph_type);
        (!fallback.is_empty()).then(|| fallback.to_string())
    });
    if let Some(example) = example {
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

fn schema_help_example_for(
    canonical: &str,
    resolved_variant: &BTreeMap<String, String>,
) -> Option<String> {
    let schema = collect_schemas()
        .into_iter()
        .find(|schema| schema.name == canonical)?;
    let example = problemreductions::example_db::find_model_example(&ProblemRef {
        name: canonical.to_string(),
        variant: resolved_variant.clone(),
    })
    .ok()?;
    let instance = example.instance.as_object()?;
    let graph_type = resolved_variant
        .get("graph")
        .map(String::as_str)
        .filter(|graph_type| *graph_type != "SimpleGraph");
    let is_geometry = matches!(
        graph_type,
        Some("KingsSubgraph" | "TriangularSubgraph" | "UnitDiskGraph")
    );

    let mut args = Vec::new();
    for field in &schema.fields {
        let value = instance.get(&field.name)?;
        let concrete_type = resolve_schema_field_type(&field.type_name, resolved_variant);
        let flag_name =
            schema_example_flag_name(canonical, &field.name, &field.type_name, is_geometry);
        let rendered =
            format_schema_help_example_value(canonical, &field.name, &concrete_type, value)?;
        args.push(format!("--{flag_name} {}", quote_cli_arg(&rendered)));
    }
    Some(args.join(" "))
}

fn schema_example_flag_name(
    canonical: &str,
    field_name: &str,
    field_type: &str,
    is_geometry: bool,
) -> String {
    problem_help_flag_name(canonical, field_name, field_type, is_geometry)
        .split('/')
        .next()
        .unwrap_or(field_name)
        .trim_start_matches("--")
        .to_string()
}

fn quote_cli_arg(raw: &str) -> String {
    if raw.is_empty()
        || raw.chars().any(|ch| {
            ch.is_whitespace()
                || matches!(
                    ch,
                    ';' | '>' | '|' | '[' | ']' | '{' | '}' | '(' | ')' | '"' | '\''
                )
        })
    {
        format!("\"{}\"", raw.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        raw.to_string()
    }
}

fn format_schema_help_example_value(
    canonical: &str,
    field_name: &str,
    concrete_type: &str,
    value: &serde_json::Value,
) -> Option<String> {
    match (canonical, field_name) {
        ("ConsecutiveBlockMinimization", "matrix")
        | ("FeasibleBasisExtension", "matrix")
        | ("MinimumWeightDecoding", "matrix")
        | ("MinimumWeightSolutionToLinearEquations", "matrix") => {
            return serde_json::to_string(value).ok();
        }
        _ => {}
    }
    match normalize_type_name(concrete_type).as_str() {
        "SimpleGraph" => format_simple_graph_example(value),
        "DirectedGraph" => format_directed_graph_example(value),
        "Vec<CNFClause>" => format_cnf_clause_list_example(value),
        "Vec<Quantifier>" => format_quantifier_list_example(value),
        "Vec<Vec<(usize,u64)>>" => format_job_shop_example(value),
        "Vec<(Vec<usize>,Vec<usize>)>" => format_dependency_example(value),
        "Vec<usize>" | "Vec<u64>" | "Vec<i32>" | "Vec<i64>" | "Vec<f64>" | "Vec<BigUint>" => {
            format_scalar_array_example(value)
        }
        "Vec<bool>" => format_bool_array_example(value),
        "Vec<Vec<usize>>" | "Vec<Vec<u64>>" | "Vec<Vec<i32>>" | "Vec<Vec<i64>>"
        | "Vec<Vec<f64>>" => format_nested_numeric_rows(value),
        "Vec<Vec<bool>>" => format_bool_matrix_example(value),
        "Vec<String>" => Some(
            value
                .as_array()?
                .iter()
                .map(|entry| entry.as_str().map(str::to_string))
                .collect::<Option<Vec<_>>>()?
                .join(";"),
        ),
        "usize" | "u64" | "i32" | "i64" | "f64" | "BigUint" => format_scalar_example(value),
        _ => None,
    }
}

fn format_scalar_example(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Number(number) => Some(number.to_string()),
        serde_json::Value::String(string) => Some(string.clone()),
        serde_json::Value::Bool(boolean) => Some(boolean.to_string()),
        _ => None,
    }
}

fn format_scalar_array_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(format_scalar_example)
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

fn format_bool_array_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|entry| {
                entry
                    .as_bool()
                    .map(|boolean| if boolean { "1" } else { "0" }.to_string())
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

fn format_nested_numeric_rows(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|row| format_scalar_array_example(row))
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

fn format_cnf_clause_list_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|clause| format_scalar_array_example(clause.get("literals")?))
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

fn format_bool_matrix_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(format_bool_array_example)
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

fn format_simple_graph_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .get("edges")?
            .as_array()?
            .iter()
            .map(|edge| {
                let pair = edge.as_array()?;
                Some(format!(
                    "{}-{}",
                    pair.first()?.as_u64()?,
                    pair.get(1)?.as_u64()?
                ))
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

fn format_directed_graph_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .get("arcs")?
            .as_array()?
            .iter()
            .map(|arc| {
                let pair = arc.as_array()?;
                Some(format!(
                    "{}>{}",
                    pair.first()?.as_u64()?,
                    pair.get(1)?.as_u64()?
                ))
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

fn format_quantifier_list_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|entry| match entry.as_str()? {
                "Exists" => Some("E".to_string()),
                "ForAll" => Some("A".to_string()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?
            .join(","),
    )
}

fn format_job_shop_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|job| {
                Some(
                    job.as_array()?
                        .iter()
                        .map(|task| {
                            let task = task.as_array()?;
                            Some(format!(
                                "{}:{}",
                                task.first()?.as_u64()?,
                                task.get(1)?.as_u64()?
                            ))
                        })
                        .collect::<Option<Vec<_>>>()?
                        .join(","),
                )
            })
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
}

fn format_dependency_example(value: &serde_json::Value) -> Option<String> {
    Some(
        value
            .as_array()?
            .iter()
            .map(|dependency| {
                let dependency = dependency.as_array()?;
                let lhs = format_scalar_array_example(dependency.first()?)?;
                let rhs = format_scalar_array_example(dependency.get(1)?)?;
                Some(format!("{lhs}>{rhs}"))
            })
            .collect::<Option<Vec<_>>>()?
            .join(";"),
    )
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
        return "max-length".to_string();
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
            "--max-length must be a nonnegative integer for LengthBoundedDisjointPaths",
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
        return Err(lbdp_validation_error(
            "--max-length must be positive",
            usage,
        ));
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

fn parse_disjuncts(args: &CreateArgs) -> Result<Vec<Vec<i32>>> {
    let disjuncts_str = args
        .disjuncts
        .as_deref()
        .or(args.clauses.as_deref())
        .ok_or_else(|| {
            anyhow::anyhow!("NonTautology requires --disjuncts (e.g., \"1,2,3;-1,-2,-3\")")
        })?;

    disjuncts_str
        .split(';')
        .map(|disjunct| {
            disjunct
                .trim()
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(anyhow::Error::from)
        })
        .collect()
}

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
            "max-length"
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
    fn test_parse_field_value_parses_simple_graph_to_json() {
        let value = parse_field_value("SimpleGraph", "graph", "0-1,1-2", &CreateContext::default())
            .expect("parse graph");

        assert_eq!(
            value,
            serde_json::json!({
                "num_vertices": 3,
                "edges": [[0, 1], [1, 2]],
            })
        );
    }

    #[test]
    fn test_parse_field_value_parses_dependency_pairs() {
        let value = parse_field_value(
            "Vec<(Vec<usize>, Vec<usize>)>",
            "dependencies",
            "0,1>2,3;2>4",
            &CreateContext::default(),
        )
        .expect("parse dependencies");

        assert_eq!(value, serde_json::json!([[[0, 1], [2, 3]], [[2], [4]],]));
    }

    #[test]
    fn test_parse_field_value_parses_job_shop_jobs() {
        let value = parse_field_value(
            "Vec<Vec<(usize, u64)>>",
            "jobs",
            "0:3,1:4;1:2,0:3,1:2",
            &CreateContext::default(),
        )
        .expect("parse jobs");

        assert_eq!(
            value,
            serde_json::json!([[[0, 3], [1, 4]], [[1, 2], [0, 3], [1, 2]],])
        );
    }

    #[test]
    fn test_parse_field_value_parses_quantifiers_using_context_num_vars() {
        let context = CreateContext::default().with_field("num_vars", serde_json::json!(3));
        let value = parse_field_value("Vec<Quantifier>", "quantifiers", "E,A,E", &context)
            .expect("parse quantifiers");

        assert_eq!(value, serde_json::json!(["Exists", "ForAll", "Exists"]));
    }

    #[test]
    fn test_schema_driven_supported_problem_includes_cli_creatable_problem() {
        assert!(
            schema_driven_supported_problem("ConjunctiveBooleanQuery"),
            "all CLI-creatable problems should opt into schema-driven create unless explicitly excluded"
        );
        assert!(!schema_driven_supported_problem("ILP"));
        assert!(!schema_driven_supported_problem("CircuitSAT"));
    }

    #[test]
    fn test_create_schema_driven_builds_job_shop_scheduling() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "JobShopScheduling",
            "--jobs",
            "0:3,1:4;1:2,0:3,1:2",
            "--num-processors",
            "2",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let (data, variant) = create_schema_driven(&args, "JobShopScheduling", &BTreeMap::new())
            .expect("schema-driven create should parse")
            .expect("schema-driven path should support JobShopScheduling");

        let entry = problemreductions::registry::find_variant_entry("JobShopScheduling", &variant)
            .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["num_processors"], 2);
        assert_eq!(data["jobs"][0], serde_json::json!([[0, 3], [1, 4]]));
    }

    #[test]
    fn test_create_schema_driven_builds_quantified_boolean_formulas() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "QuantifiedBooleanFormulas",
            "--num-vars",
            "3",
            "--quantifiers",
            "E,A,E",
            "--clauses",
            "1,2;-1,3",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let (data, variant) =
            create_schema_driven(&args, "QuantifiedBooleanFormulas", &BTreeMap::new())
                .expect("schema-driven create should parse")
                .expect("schema-driven path should support QBF");

        let entry =
            problemreductions::registry::find_variant_entry("QuantifiedBooleanFormulas", &variant)
                .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(
            data["quantifiers"],
            serde_json::json!(["Exists", "ForAll", "Exists"])
        );
    }

    #[test]
    fn test_create_schema_driven_builds_undirected_flow_lower_bounds() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "UndirectedFlowLowerBounds",
            "--graph",
            "0-1,0-2,1-3,2-3",
            "--capacities",
            "2,2,2,2",
            "--lower-bounds",
            "1,0,0,1",
            "--source",
            "0",
            "--sink",
            "3",
            "--requirement",
            "2",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let (data, variant) =
            create_schema_driven(&args, "UndirectedFlowLowerBounds", &BTreeMap::new())
                .expect("schema-driven create should parse")
                .expect("schema-driven path should support UndirectedFlowLowerBounds");

        let entry =
            problemreductions::registry::find_variant_entry("UndirectedFlowLowerBounds", &variant)
                .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["graph"]["num_vertices"], 4);
        assert_eq!(data["capacities"], serde_json::json!([2, 2, 2, 2]));
        assert_eq!(data["lower_bounds"], serde_json::json!([1, 0, 0, 1]));
    }

    #[test]
    fn test_create_schema_driven_builds_conjunctive_boolean_query() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "ConjunctiveBooleanQuery",
            "--domain-size",
            "6",
            "--relations",
            "2:0,3|1,3;3:0,1,5|1,2,5",
            "--conjuncts-spec",
            "0:v0,c3;0:v1,c3;1:v0,v1,c5",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let (data, variant) =
            create_schema_driven(&args, "ConjunctiveBooleanQuery", &BTreeMap::new())
                .expect("schema-driven create should parse")
                .expect("schema-driven path should support CBQ");

        let entry =
            problemreductions::registry::find_variant_entry("ConjunctiveBooleanQuery", &variant)
                .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["num_variables"], 2);
        assert_eq!(data["relations"][0]["arity"], 2);
        assert_eq!(
            data["conjuncts"][1],
            serde_json::json!([0, [{"Variable": 1}, {"Constant": 3}]])
        );
    }

    #[test]
    fn test_create_schema_driven_builds_closest_vector_problem_with_default_bounds() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "CVP",
            "--basis",
            "1,0;0,1",
            "--target-vec",
            "0.5,0.5",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let resolved_variant = variant_map(&[("weight", "i32")]);
        let (data, variant) =
            create_schema_driven(&args, "ClosestVectorProblem", &resolved_variant)
                .expect("schema-driven create should parse")
                .expect("schema-driven path should support CVP");

        let entry =
            problemreductions::registry::find_variant_entry("ClosestVectorProblem", &variant)
                .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["basis"], serde_json::json!([[1, 0], [0, 1]]));
        assert_eq!(
            data["bounds"],
            serde_json::json!([
                {"lower": -10, "upper": 10},
                {"lower": -10, "upper": 10},
            ])
        );
    }

    #[test]
    fn test_create_schema_driven_builds_cdft() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "ConsistencyOfDatabaseFrequencyTables",
            "--num-objects",
            "6",
            "--attribute-domains",
            "2,3,2",
            "--frequency-tables",
            "0,1:1,1,1|1,1,1;1,2:1,1|0,2|1,1",
            "--known-values",
            "0,0,0;3,0,1;1,2,1",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let (data, variant) = create_schema_driven(
            &args,
            "ConsistencyOfDatabaseFrequencyTables",
            &BTreeMap::new(),
        )
        .expect("schema-driven create should parse")
        .expect("schema-driven path should support CDFT");

        let entry = problemreductions::registry::find_variant_entry(
            "ConsistencyOfDatabaseFrequencyTables",
            &variant,
        )
        .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["num_objects"], 6);
        assert_eq!(data["frequency_tables"][0]["attribute_a"], 0);
        assert_eq!(data["known_values"][2]["attribute"], 2);
    }

    #[test]
    fn test_create_schema_driven_builds_balanced_complete_bipartite_subgraph() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "BalancedCompleteBipartiteSubgraph",
            "--left",
            "4",
            "--right",
            "4",
            "--biedges",
            "0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3",
            "--k",
            "3",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let (data, variant) =
            create_schema_driven(&args, "BalancedCompleteBipartiteSubgraph", &BTreeMap::new())
                .expect("schema-driven create should parse")
                .expect("schema-driven path should support balanced biclique");

        let entry = problemreductions::registry::find_variant_entry(
            "BalancedCompleteBipartiteSubgraph",
            &variant,
        )
        .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["graph"]["left_size"], 4);
        assert_eq!(data["graph"]["right_size"], 4);
        assert_eq!(data["k"], 3);
    }

    #[test]
    fn test_create_schema_driven_builds_mixed_chinese_postman() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "MixedChinesePostman/i32",
            "--graph",
            "0-2,1-3,0-4,4-2",
            "--arcs",
            "0>1,1>2,2>3,3>0",
            "--edge-weights",
            "2,3,1,2",
            "--arc-weights",
            "2,3,1,4",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let resolved_variant = variant_map(&[("weight", "i32")]);
        let (data, variant) = create_schema_driven(&args, "MixedChinesePostman", &resolved_variant)
            .expect("schema-driven create should parse")
            .expect("schema-driven path should support mixed chinese postman");

        let entry =
            problemreductions::registry::find_variant_entry("MixedChinesePostman", &variant)
                .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["graph"]["num_vertices"], 5);
        assert_eq!(data["arc_weights"], serde_json::json!([2, 3, 1, 4]));
        assert_eq!(data["edge_weights"], serde_json::json!([2, 3, 1, 2]));
    }

    #[test]
    fn test_create_schema_driven_builds_unit_disk_graph_problem_with_default_radius() {
        let cli = Cli::parse_from([
            "pred",
            "create",
            "MIS/UnitDiskGraph",
            "--positions",
            "0,0;1,0;0.5,0.8",
        ]);

        let Commands::Create(args) = cli.command else {
            panic!("expected create command");
        };

        let resolved_variant = variant_map(&[("graph", "UnitDiskGraph"), ("weight", "One")]);
        let (data, variant) =
            create_schema_driven(&args, "MaximumIndependentSet", &resolved_variant)
                .expect("schema-driven create should parse")
                .expect("schema-driven path should support UnitDiskGraph variants");

        let entry =
            problemreductions::registry::find_variant_entry("MaximumIndependentSet", &variant)
                .expect("variant entry");
        (entry.factory)(data.clone()).expect("factory should deserialize generated JSON");
        assert_eq!(data["graph"]["positions"].as_array().unwrap().len(), 3);
        assert_eq!(
            data["graph"]["edges"],
            serde_json::json!([[0, 1], [0, 2], [1, 2]])
        );
    }

    #[test]
    fn test_schema_help_example_for_qbf_uses_example_db() {
        let example =
            schema_help_example_for("QuantifiedBooleanFormulas", &BTreeMap::new()).unwrap();
        assert_eq!(
            example,
            "--num-vars 2 --quantifiers E,A --clauses \"1,2;1,-2\""
        );
    }

    #[test]
    fn test_schema_help_example_for_cbm_uses_json_matrix_syntax() {
        let example =
            schema_help_example_for("ConsecutiveBlockMinimization", &BTreeMap::new()).unwrap();
        assert!(example.contains("--matrix \"[[false,true,false,false,false,false],[true,false,true,false,false,false],[false,true,false,true,false,false],[false,false,true,false,true,false],[false,false,false,true,false,true],[false,false,false,false,true,false]]\""));
        assert!(example.contains("--bound-k 6"));
    }

    #[test]
    fn test_problem_help_flag_name_uses_bound_for_grouping_by_swapping_budget() {
        assert_eq!(
            problem_help_flag_name("GroupingBySwapping", "budget", "usize", false),
            "bound"
        );
    }

    #[test]
    fn test_problem_help_flag_name_preserves_edge_lengths_for_shortest_weight_constrained_path() {
        assert_eq!(
            problem_help_flag_name(
                "ShortestWeightConstrainedPath",
                "edge_lengths",
                "Vec<W>",
                false
            ),
            "edge-lengths"
        );
    }

    #[test]
    fn test_problem_help_flag_name_uses_edge_weights_for_longest_circuit_edge_lengths() {
        assert_eq!(
            problem_help_flag_name("LongestCircuit", "edge_lengths", "Vec<W>", false),
            "edge-weights"
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
            "--num-tasks",
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
    fn test_create_prime_attribute_name_accepts_canonical_flags() {
        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "PrimeAttributeName",
            "--universe",
            "6",
            "--dependencies",
            "0,1>2,3,4,5;2,3>0,1,4,5",
            "--query-attribute",
            "3",
        ])
        .expect("parse create command");

        let Commands::Create(args) = cli.command else {
            panic!("expected create subcommand");
        };

        let output_path = temp_output_path("prime_attribute_name");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).expect("create PrimeAttributeName JSON");

        let created: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output_path).unwrap()).unwrap();
        fs::remove_file(output_path).ok();

        assert_eq!(created["type"], "PrimeAttributeName");
        assert_eq!(created["data"]["query_attribute"], 3);
        assert_eq!(
            created["data"]["dependencies"][0],
            serde_json::json!([[0, 1], [2, 3, 4, 5]])
        );
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
            "dependencies"
        );
        assert_eq!(
            problem_help_flag_name("PrimeAttributeName", "query_attribute", "usize", false),
            "query-attribute"
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
            disjuncts: None,
            num_vars: None,
            matrix: None,
            k: None,
            num_partitions: None,
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
            partitions: None,
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
            diameter_bound: None,
            cost_bound: None,
            delay_budget: None,
            pattern: None,
            strings: None,
            string: None,
            arc_costs: None,
            arcs: None,
            left_arcs: None,
            right_arcs: None,
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
            pointer_cost: None,
            expression: None,
            coeff_a: None,
            coeff_b: None,
            rhs: None,
            coeff_c: None,
            pairs: None,
            required_columns: None,
            compilers: None,
            setup_times: None,
            w_sizes: None,
            x_sizes: None,
            y_sizes: None,
            equations: None,
            assignment: None,
            initial_marking: None,
            output_arcs: None,
            gate_types: None,
            true_sentences: None,
            implications: None,
            loop_length: None,
            loop_variables: None,
            inputs: None,
            outputs: None,
            assignments: None,
            num_variables: None,
            truth_table: None,
            test_matrix: None,
            num_tests: None,
            tiles: None,
            grid_size: None,
            num_colors: None,
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
        assert!(err.contains("JobShopScheduling requires --jobs"));
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
    fn test_create_graph_partitioning_with_num_partitions() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::graph::GraphPartitioning;
        use problemreductions::topology::SimpleGraph;

        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "GraphPartitioning",
            "--graph",
            "0-1,1-2,2-3,3-0",
            "--num-partitions",
            "2",
        ])
        .unwrap();
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let output_path = temp_output_path("graph-partitioning-create");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "GraphPartitioning");
        let problem: GraphPartitioning<SimpleGraph> = serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.num_vertices(), 4);

        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_create_nontautology_with_disjuncts_flag() {
        use crate::dispatch::ProblemJsonOutput;
        use problemreductions::models::formula::NonTautology;

        let cli = Cli::try_parse_from([
            "pred",
            "create",
            "NonTautology",
            "--num-vars",
            "3",
            "--disjuncts",
            "1,2,3;-1,-2,-3",
        ])
        .unwrap();
        let args = match cli.command {
            Commands::Create(args) => args,
            _ => unreachable!(),
        };

        let output_path = temp_output_path("non-tautology-create");
        let out = OutputConfig {
            output: Some(output_path.clone()),
            quiet: true,
            json: false,
            auto_json: false,
        };

        create(&args, &out).unwrap();

        let json = fs::read_to_string(&output_path).unwrap();
        let created: ProblemJsonOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(created.problem_type, "NonTautology");
        let problem: NonTautology = serde_json::from_value(created.data).unwrap();
        assert_eq!(problem.disjuncts(), &[vec![1, 2, 3], vec![-1, -2, -3]]);

        let _ = fs::remove_file(output_path);
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
