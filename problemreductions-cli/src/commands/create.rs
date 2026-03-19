use crate::cli::{CreateArgs, ExampleSide};
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::{
    parse_problem_spec, resolve_catalog_problem_ref, resolve_problem_ref, unknown_problem_error,
};
use crate::util;
use anyhow::{bail, Context, Result};
use problemreductions::export::{ModelExample, ProblemRef, ProblemSide, RuleExample};
use problemreductions::models::algebraic::{ClosestVectorProblem, ConsecutiveOnesSubmatrix, BMF};
use problemreductions::models::graph::{
    GraphPartitioning, HamiltonianCircuit, HamiltonianPath, LengthBoundedDisjointPaths,
    MinimumMultiwayCut, MultipleChoiceBranching, SteinerTree, StrongConnectivityAugmentation,
};
use problemreductions::models::misc::{
    BinPacking, CbqRelation, ConjunctiveBooleanQuery, FlowShopScheduling, LongestCommonSubsequence,
    MinimumTardinessSequencing, MultiprocessorScheduling, PaintShop, QueryArg,
    RectilinearPictureCompression, SequencingWithReleaseTimesAndDeadlines,
    SequencingWithinIntervals, ShortestCommonSupersequence, StringToStringCorrection, SubsetSum,
    SumOfSquaresPartition,
};
use problemreductions::models::BiconnectivityAugmentation;
use problemreductions::prelude::*;
use problemreductions::registry::collect_schemas;
use problemreductions::topology::{
    BipartiteGraph, DirectedGraph, Graph, KingsSubgraph, SimpleGraph, TriangularSubgraph,
    UnitDiskGraph,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// Check if all data flags are None (no problem-specific input provided).
fn all_data_flags_empty(args: &CreateArgs) -> bool {
    args.graph.is_none()
        && args.weights.is_none()
        && args.edge_weights.is_none()
        && args.capacities.is_none()
        && args.source.is_none()
        && args.sink.is_none()
        && args.num_paths_required.is_none()
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
        && args.sizes.is_none()
        && args.capacity.is_none()
        && args.sequence.is_none()
        && args.sets.is_none()
        && args.r_sets.is_none()
        && args.s_sets.is_none()
        && args.r_weights.is_none()
        && args.s_weights.is_none()
        && args.partition.is_none()
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
        && args.tree.is_none()
        && args.required_edges.is_none()
        && args.bound.is_none()
        && args.pattern.is_none()
        && args.strings.is_none()
        && args.arcs.is_none()
        && args.distance_matrix.is_none()
        && args.candidate_arcs.is_none()
        && args.potential_edges.is_none()
        && args.budget.is_none()
        && args.precedence_pairs.is_none()
        && args.task_lengths.is_none()
        && args.deadline.is_none()
        && args.num_processors.is_none()
        && args.schedules.is_none()
        && args.requirements.is_none()
        && args.num_workers.is_none()
        && args.alphabet_size.is_none()
        && args.num_groups.is_none()
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
        && args.domain_size.is_none()
        && args.relations.is_none()
        && args.conjuncts_spec.is_none()
        && args.deps.is_none()
        && args.query.is_none()
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
        "Vec<usize>" => "comma-separated indices: 0,2,4",
        "Vec<(usize, usize, W)>" | "Vec<(usize,usize,W)>" => {
            "comma-separated weighted edges: 0-2:3,1-3:5"
        }
        "Vec<Vec<usize>>" => "semicolon-separated sets: \"0,1;1,2;0,2\"",
        "Vec<CNFClause>" => "semicolon-separated clauses: \"1,2;-1,3\"",
        "Vec<Vec<bool>>" => "semicolon-separated binary rows: \"1,1,0;0,1,1\"",
        "Vec<Vec<W>>" => "semicolon-separated rows: \"1,0.5;0.5,2\"",
        "usize" | "W::Sum" => "integer",
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
        "GraphPartitioning" => "--graph 0-1,1-2,2-3,0-2,1-3,0-3",
        "BoundedComponentSpanningForest" => {
            "--graph 0-1,1-2,2-3,3-4,4-5,5-6,6-7,0-7,1-5,2-6 --weights 2,3,1,2,3,1,2,1 --k 3 --bound 6"
        }
        "HamiltonianPath" => "--graph 0-1,1-2,2-3",
        "UndirectedTwoCommodityIntegralFlow" => {
            "--graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1"
        },
        "LengthBoundedDisjointPaths" => {
            "--graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --num-paths-required 2 --bound 3"
        }
        "IsomorphicSpanningTree" => "--graph 0-1,1-2,0-2 --tree 0-1,1-2",
        "KthBestSpanningTree" => "--graph 0-1,0-2,1-2 --edge-weights 2,3,1 --k 1 --bound 3",
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1"
        }
        "BiconnectivityAugmentation" => {
            "--graph 0-1,1-2,2-3 --potential-edges 0-2:3,0-3:4,1-3:2 --budget 5"
        }
        "Satisfiability" => "--num-vars 3 --clauses \"1,2;-1,3\"",
        "KSatisfiability" => "--num-vars 3 --clauses \"1,2,3;-1,2,-3\" --k 3",
        "QUBO" => "--matrix \"1,0.5;0.5,2\"",
        "QuadraticAssignment" => "--matrix \"0,5;5,0\" --distance-matrix \"0,1;1,0\"",
        "SpinGlass" => "--graph 0-1,1-2 --couplings 1,1",
        "KColoring" => "--graph 0-1,1-2,2-0 --k 3",
        "HamiltonianCircuit" => "--graph 0-1,1-2,2-3,3-0",
        "MinimumSumMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "BalancedCompleteBipartiteSubgraph" => {
            "--left 4 --right 4 --biedges 0-0,0-1,0-2,1-0,1-1,1-2,2-0,2-1,2-2,3-0,3-1,3-3 --k 3"
        }
        "PartitionIntoTriangles" => "--graph 0-1,1-2,0-2",
        "Factoring" => "--target 15 --m 4 --n 4",
        "MultiprocessorScheduling" => "--lengths 4,5,3,2,6 --num-processors 2 --deadline 10",
        "MinimumMultiwayCut" => "--graph 0-1,1-2,2-3 --terminals 0,2 --edge-weights 1,1,1",
        "SequencingWithinIntervals" => "--release-times 0,0,5 --deadlines 11,11,6 --lengths 3,1,1",
        "StaffScheduling" => {
            "--schedules \"1,1,1,1,1,0,0;0,1,1,1,1,1,0;0,0,1,1,1,1,1;1,0,0,1,1,1,1;1,1,0,0,1,1,1\" --requirements 2,2,2,3,3,2,1 --num-workers 4 --k 5"
        }
        "SteinerTree" => "--graph 0-1,1-2,1-3,3-4 --edge-weights 2,2,1,1 --terminals 0,2,4",
        "OptimalLinearArrangement" => "--graph 0-1,1-2,2-3 --bound 5",
        "DirectedTwoCommodityIntegralFlow" => {
            "--arcs \"0>2,0>3,1>2,1>3,2>4,2>5,3>4,3>5\" --capacities 1,1,1,1,1,1,1,1 --source-1 0 --sink-1 4 --source-2 1 --sink-2 5 --requirement-1 1 --requirement-2 1"
        }
        "MinimumFeedbackArcSet" => "--arcs \"0>1,1>2,2>0\"",
        "StrongConnectivityAugmentation" => {
            "--arcs \"0>1,1>2\" --candidate-arcs \"2>0:1\" --bound 1"
        }
        "RuralPostman" => {
            "--graph 0-1,1-2,2-3,3-0 --edge-weights 1,1,1,1 --required-edges 0,2 --bound 4"
        }
        "MultipleChoiceBranching" => {
            "--arcs \"0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4\" --weights 3,2,4,1,2,3,1,3 --partition \"0,1;2,3;4,7;5,6\" --bound 10"
        }
        "SubgraphIsomorphism" => "--graph 0-1,1-2,2-0 --pattern 0-1",
        "RectilinearPictureCompression" => {
            "--matrix \"1,1,0,0;1,1,0,0;0,0,1,1;0,0,1,1\" --k 2"
        }
        "SubsetSum" => "--sizes 3,7,1,8,2,4 --target 11",
        "SumOfSquaresPartition" => "--sizes 5,3,8,2,7,1 --num-groups 3 --bound 240",
        "ComparativeContainment" => {
            "--universe 4 --r-sets \"0,1,2,3;0,1\" --s-sets \"0,1,2,3;2,3\" --r-weights 2,5 --s-weights 3,6"
        }
        "SetBasis" => "--universe 4 --sets \"0,1;1,2;0,2;0,1,2\" --k 3",
        "PrimeAttributeName" => {
            "--universe 6 --deps \"0,1>2,3,4,5;2,3>0,1,4,5\" --query 3"
        }
        "LongestCommonSubsequence" => {
            "--strings \"010110;100101;001011\" --bound 3 --alphabet-size 2"
        }
        "MinimumCardinalityKey" => {
            "--num-attributes 6 --dependencies \"0,1>2;0,2>3;1,3>4;2,4>5\" --k 2"
        }
        "ShortestCommonSupersequence" => "--strings \"0,1,2;1,2,0\" --bound 4",
        "ConjunctiveBooleanQuery" => {
            "--domain-size 6 --relations \"2:0,3|1,3|2,4;3:0,1,5|1,2,5\" --conjuncts-spec \"0:v0,c3;0:v1,c3;1:v0,v1,c5\""
        }
        "StringToStringCorrection" => {
            "--source-string \"0,1,2,3,1,0\" --target-string \"0,1,3,2,1\" --bound 2"
        }
        _ => "",
    }
}

fn uses_edge_weights_flag(canonical: &str) -> bool {
    matches!(
        canonical,
        "KthBestSpanningTree" | "MaxCut" | "MaximumMatching" | "TravelingSalesman" | "RuralPostman"
    )
}

fn help_flag_name(canonical: &str, field_name: &str) -> String {
    // Problem-specific overrides first
    match (canonical, field_name) {
        ("BoundedComponentSpanningForest", "max_components") => return "k".to_string(),
        ("BoundedComponentSpanningForest", "max_weight") => return "bound".to_string(),
        ("LengthBoundedDisjointPaths", "max_length") => return "bound".to_string(),
        ("RectilinearPictureCompression", "bound_k") => return "k".to_string(),
        ("PrimeAttributeName", "num_attributes") => return "universe".to_string(),
        ("PrimeAttributeName", "dependencies") => return "deps".to_string(),
        ("PrimeAttributeName", "query_attribute") => return "query".to_string(),
        ("MinimumCardinalityKey", "bound_k") => return "k".to_string(),
        ("ConsecutiveOnesSubmatrix", "bound_k") => return "k".to_string(),
        ("StaffScheduling", "shifts_per_schedule") => return "k".to_string(),
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
        ("PrimeAttributeName", "dependencies") => {
            "semicolon-separated dependencies: \"0,1>2,3;2,3>0,1\""
        }
        ("LongestCommonSubsequence", "strings") => {
            "raw strings: \"ABAC;BACA\" or symbol lists: \"0,1,0;1,0,1\""
        }
        ("ShortestCommonSupersequence", "strings") => "symbol lists: \"0,1,2;1,2,0\"",
        ("MultipleChoiceBranching", "partition") => "semicolon-separated groups: \"0,1;2,3\"",
        ("ConsecutiveOnesSubmatrix", "matrix") => "semicolon-separated 0/1 rows: \"1,0;0,1\"",
        _ => type_format_hint(type_name, graph_type),
    }
}

fn parse_nonnegative_usize_bound(bound: i64, problem_name: &str, usage: &str) -> Result<usize> {
    usize::try_from(bound)
        .map_err(|_| anyhow::anyhow!("{problem_name} requires nonnegative --bound\n\n{usage}"))
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
            } else if field.type_name == "BipartiteGraph" {
                eprintln!(
                    "  --{:<16} {} ({})",
                    "left", "Vertices in the left partition", "integer"
                );
                eprintln!(
                    "  --{:<16} {} ({})",
                    "right", "Vertices in the right partition", "integer"
                );
                eprintln!(
                    "  --{:<16} {} ({})",
                    "biedges", "Bipartite edges as left-right pairs", "edge list: 0-0,0-1,1-2"
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
    if canonical == "LengthBoundedDisjointPaths" && field_name == "max_length" {
        return "bound".to_string();
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
    num_paths_required: usize,
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
    if num_paths_required == 0 {
        return Err(lbdp_validation_error(
            "--num-paths-required must be positive",
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

        // Graph partitioning (graph only, no weights)
        "GraphPartitioning" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create GraphPartitioning --graph 0-1,1-2,2-3,0-2,1-3,0-3"
                )
            })?;
            (
                ser(GraphPartitioning::new(graph))?,
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

        // UndirectedTwoCommodityIntegralFlow (graph + capacities + terminals + requirements)
        "UndirectedTwoCommodityIntegralFlow" => {
            let usage = "Usage: pred create UndirectedTwoCommodityIntegralFlow --graph 0-2,1-2,2-3 --capacities 1,1,2 --source-1 0 --sink-1 3 --source-2 1 --sink-2 3 --requirement-1 1 --requirement-2 1";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let capacities = parse_capacities(args, graph.num_edges(), usage)?;
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

        // LengthBoundedDisjointPaths (graph + source + sink + path count + bound)
        "LengthBoundedDisjointPaths" => {
            let usage = "Usage: pred create LengthBoundedDisjointPaths --graph 0-1,1-6,0-2,2-3,3-6,0-4,4-5,5-6 --source 0 --sink 6 --num-paths-required 2 --bound 3";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let source = args.source.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --source\n\n{usage}")
            })?;
            let sink = args.sink.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --sink\n\n{usage}")
            })?;
            let num_paths_required = args.num_paths_required.ok_or_else(|| {
                anyhow::anyhow!(
                    "LengthBoundedDisjointPaths requires --num-paths-required\n\n{usage}"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("LengthBoundedDisjointPaths requires --bound\n\n{usage}")
            })?;
            let max_length = validate_length_bounded_disjoint_paths_args(
                graph.num_vertices(),
                source,
                sink,
                num_paths_required,
                bound,
                Some(usage),
            )?;

            (
                ser(LengthBoundedDisjointPaths::new(
                    graph,
                    source,
                    sink,
                    num_paths_required,
                    max_length,
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
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create {} --graph 0-1,1-2,2-3 [--edge-weights 1,1,1]",
                    problem
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let data = match canonical {
                "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                _ => unreachable!(),
            };
            (data, resolved_variant.clone())
        }

        // RuralPostman
        "RuralPostman" => {
            reject_vertex_weights_for_edge_weight_problem(args, canonical, None)?;
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create RuralPostman --graph 0-1,1-2,2-3 --edge-weights 1,1,1 --required-edges 0,2 --bound 6"
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let required_edges_str = args.required_edges.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "RuralPostman requires --required-edges\n\n\
                     Usage: pred create RuralPostman --graph 0-1,1-2,2-3 --edge-weights 1,1,1 --required-edges 0,2 --bound 6"
                )
            })?;
            let required_edges: Vec<usize> = util::parse_comma_list(required_edges_str)?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "RuralPostman requires --bound\n\n\
                     Usage: pred create RuralPostman --graph 0-1,1-2,2-3 --edge-weights 1,1,1 --required-edges 0,2 --bound 6"
                )
            })? as i32;
            (
                ser(RuralPostman::new(
                    graph,
                    edge_weights,
                    required_edges,
                    bound,
                ))?,
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

        // SumOfSquaresPartition
        "SumOfSquaresPartition" => {
            let sizes_str = args.sizes.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "SumOfSquaresPartition requires --sizes, --num-groups, and --bound\n\n\
                     Usage: pred create SumOfSquaresPartition --sizes 5,3,8,2,7,1 --num-groups 3 --bound 240"
                )
            })?;
            let num_groups = args.num_groups.ok_or_else(|| {
                anyhow::anyhow!(
                    "SumOfSquaresPartition requires --num-groups\n\n\
                     Usage: pred create SumOfSquaresPartition --sizes 5,3,8,2,7,1 --num-groups 3 --bound 240"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "SumOfSquaresPartition requires --bound\n\n\
                     Usage: pred create SumOfSquaresPartition --sizes 5,3,8,2,7,1 --num-groups 3 --bound 240"
                )
            })?;
            let sizes: Vec<i64> = util::parse_comma_list(sizes_str)?;
            (
                ser(SumOfSquaresPartition::try_new(sizes, num_groups, bound)
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
                    "MinimumCardinalityKey requires --num-attributes, --dependencies, and --k\n\n\
                     Usage: pred create MinimumCardinalityKey --num-attributes 6 --dependencies \"0,1>2;0,2>3;1,3>4;2,4>5\" --k 2"
                )
            })?;
            let k = args.k.ok_or_else(|| {
                anyhow::anyhow!("MinimumCardinalityKey requires --k (bound on key cardinality)")
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
                    k,
                ))?,
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

        // RectilinearPictureCompression
        "RectilinearPictureCompression" => {
            let matrix = parse_bool_matrix(args)?;
            let k = args.k.ok_or_else(|| {
                anyhow::anyhow!(
                    "RectilinearPictureCompression requires --matrix and --k\n\n\
                     Usage: pred create RectilinearPictureCompression --matrix \"1,1,0,0;1,1,0,0;0,0,1,1;0,0,1,1\" --k 2"
                )
            })?;
            (
                ser(RectilinearPictureCompression::new(matrix, k))?,
                resolved_variant.clone(),
            )
        }

        // ConsecutiveOnesSubmatrix
        "ConsecutiveOnesSubmatrix" => {
            let matrix = parse_bool_matrix(args)?;
            let k = args.k.ok_or_else(|| {
                anyhow::anyhow!(
                    "ConsecutiveOnesSubmatrix requires --matrix and --k\n\n\
                     Usage: pred create ConsecutiveOnesSubmatrix --matrix \"1,1,0,1;1,0,1,1;0,1,1,0\" --k 3"
                )
            })?;
            (
                ser(ConsecutiveOnesSubmatrix::new(matrix, k))?,
                resolved_variant.clone(),
            )
        }

        // LongestCommonSubsequence
        "LongestCommonSubsequence" => {
            let usage =
                "Usage: pred create LCS --strings \"010110;100101;001011\" --bound 3 [--alphabet-size 2]";
            let strings_str = args.strings.as_deref().ok_or_else(|| {
                anyhow::anyhow!("LongestCommonSubsequence requires --strings\n\n{usage}")
            })?;
            let bound_i64 = args.bound.ok_or_else(|| {
                anyhow::anyhow!("LongestCommonSubsequence requires --bound\n\n{usage}")
            })?;
            anyhow::ensure!(
                bound_i64 >= 0,
                "LongestCommonSubsequence requires a nonnegative --bound, got {}",
                bound_i64
            );
            let bound = bound_i64 as usize;

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
                alphabet_size > 0 || (bound == 0 && strings.iter().all(|string| string.is_empty())),
                "LongestCommonSubsequence requires a positive alphabet. Provide --alphabet-size when all strings are empty and --bound > 0.\n\n{usage}"
            );
            (
                ser(LongestCommonSubsequence::new(alphabet_size, strings, bound))?,
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
                ser(MinimumTardinessSequencing::new(
                    num_tasks,
                    deadlines,
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
            (
                ser(SequencingWithinIntervals::new(
                    release_times,
                    deadlines,
                    lengths,
                ))?,
                resolved_variant.clone(),
            )
        }

        // OptimalLinearArrangement — graph + bound
        "OptimalLinearArrangement" => {
            let usage = "Usage: pred create OptimalLinearArrangement --graph 0-1,1-2,2-3 --bound 5";
            let (graph, _) = parse_graph(args).map_err(|e| anyhow::anyhow!("{e}\n\n{usage}"))?;
            let bound_raw = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "OptimalLinearArrangement requires --bound (upper bound K on total edge length)\n\n{usage}"
                )
            })?;
            let bound =
                parse_nonnegative_usize_bound(bound_raw, "OptimalLinearArrangement", usage)?;
            (
                ser(OptimalLinearArrangement::new(graph, bound))?,
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
            let num_processors = if let Some(np) = args.num_processors {
                np
            } else if let Some(m) = args.m {
                m
            } else if let Some(first) = task_lengths.first() {
                first.len()
            } else {
                bail!("Cannot infer num_processors from empty task list; use --num-processors");
            };
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
            let usage = "Usage: pred create SCS --strings \"0,1,2;1,2,0\" --bound 4";
            let strings_str = args.strings.as_deref().ok_or_else(|| {
                anyhow::anyhow!("ShortestCommonSupersequence requires --strings\n\n{usage}")
            })?;
            let bound_raw = args.bound.ok_or_else(|| {
                anyhow::anyhow!("ShortestCommonSupersequence requires --bound\n\n{usage}")
            })?;
            let bound =
                parse_nonnegative_usize_bound(bound_raw, "ShortestCommonSupersequence", usage)?;
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
                ser(ShortestCommonSupersequence::new(
                    alphabet_size,
                    strings,
                    bound,
                ))?,
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

/// Parse `--edge-weights` as edge weights (i32), defaulting to all 1s.
fn parse_edge_weights(args: &CreateArgs, num_edges: usize) -> Result<Vec<i32>> {
    match &args.edge_weights {
        Some(w) => {
            let weights: Vec<i32> = w
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<std::result::Result<Vec<_>, _>>()?;
            if weights.len() != num_edges {
                bail!(
                    "Expected {} edge weights but got {}",
                    num_edges,
                    weights.len()
                );
            }
            Ok(weights)
        }
        None => Ok(vec![1i32; num_edges]),
    }
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
    let capacities = args.capacities.as_deref().ok_or_else(|| {
        anyhow::anyhow!("UndirectedTwoCommodityIntegralFlow requires --capacities\n\n{usage}")
    })?;
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

        // GraphPartitioning (graph only, no weights; requires even vertex count)
        "GraphPartitioning" => {
            let num_vertices = if num_vertices % 2 != 0 {
                eprintln!(
                    "Warning: GraphPartitioning requires even vertex count; rounding {} up to {}",
                    num_vertices,
                    num_vertices + 1
                );
                num_vertices + 1
            } else {
                num_vertices
            };
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(GraphPartitioning::new(graph))?, variant)
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
            let num_paths_required = args.num_paths_required.unwrap_or(1);
            let bound = args.bound.unwrap_or((num_vertices - 1) as i64);
            let max_length = validate_length_bounded_disjoint_paths_args(
                num_vertices,
                source,
                sink,
                num_paths_required,
                bound,
                None,
            )?;
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (
                ser(LengthBoundedDisjointPaths::new(
                    graph,
                    source,
                    sink,
                    num_paths_required,
                    max_length,
                ))?,
                variant,
            )
        }

        // Graph problems with edge weights
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let num_edges = graph.num_edges();
            let edge_weights = vec![1i32; num_edges];
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            let data = match canonical {
                "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                _ => unreachable!(),
            };
            (data, variant)
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

        // OptimalLinearArrangement — graph + bound
        "OptimalLinearArrangement" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            // Default bound: (n-1) * num_edges ensures satisfiability (max edge stretch is n-1)
            let n = graph.num_vertices();
            let usage = "Usage: pred create OptimalLinearArrangement --random --num-vertices 5 [--edge-prob 0.5] [--seed 42] [--bound 10]";
            let bound = args
                .bound
                .map(|b| parse_nonnegative_usize_bound(b, "OptimalLinearArrangement", usage))
                .transpose()?
                .unwrap_or((n.saturating_sub(1)) * graph.num_edges());
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(OptimalLinearArrangement::new(graph, bound))?, variant)
        }

        _ => bail!(
            "Random generation is not supported for {canonical}. \
             Supported: graph-based problems (MIS, MVC, MaxCut, MaxClique, \
             MaximumMatching, MinimumDominatingSet, SpinGlass, KColoring, TravelingSalesman, \
             HamiltonianCircuit, SteinerTree, OptimalLinearArrangement, HamiltonianPath)"
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
    use super::create;
    use super::help_flag_hint;
    use super::help_flag_name;
    use super::parse_bool_rows;
    use super::problem_help_flag_name;
    use super::*;
    use crate::cli::{Cli, Commands};
    use clap::Parser;
    use std::time::{SystemTime, UNIX_EPOCH};

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
            problem_help_flag_name(
                "LengthBoundedDisjointPaths",
                "num_paths_required",
                "usize",
                false,
            ),
            "num-paths-required"
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

    fn empty_args() -> CreateArgs {
        CreateArgs {
            problem: Some("BiconnectivityAugmentation".to_string()),
            example: None,
            example_target: None,
            example_side: crate::cli::ExampleSide::Source,
            graph: None,
            weights: None,
            edge_weights: None,
            capacities: None,
            source: None,
            sink: None,
            num_paths_required: None,
            couplings: None,
            fields: None,
            clauses: None,
            num_vars: None,
            matrix: None,
            k: None,
            random: false,
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
            capacity: None,
            sequence: None,
            sets: None,
            r_sets: None,
            s_sets: None,
            r_weights: None,
            s_weights: None,
            partition: None,
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
            tree: None,
            required_edges: None,
            bound: None,
            pattern: None,
            strings: None,
            arcs: None,
            distance_matrix: None,
            potential_edges: None,
            budget: None,
            candidate_arcs: None,
            deadlines: None,
            precedence_pairs: None,
            task_lengths: None,
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
            num_groups: None,
            domain_size: None,
            relations: None,
            conjuncts_spec: None,
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
}
