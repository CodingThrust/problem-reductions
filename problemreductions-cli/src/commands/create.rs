use crate::cli::{CreateArgs, ExampleSide};
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::{resolve_problem_ref, unknown_problem_error};
use crate::util;
use anyhow::{bail, Context, Result};
use problemreductions::export::{ModelExample, ProblemRef, ProblemSide, RuleExample};
use problemreductions::models::algebraic::{ClosestVectorProblem, BMF};
use problemreductions::models::graph::{GraphPartitioning, HamiltonianPath};
use problemreductions::models::misc::{
    BinPacking, FlowShopScheduling, LongestCommonSubsequence, MinimumTardinessSequencing,
    PaintShop, ShortestCommonSupersequence, SubsetSum,
};
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
        && args.sizes.is_none()
        && args.capacity.is_none()
        && args.sequence.is_none()
        && args.sets.is_none()
        && args.universe.is_none()
        && args.biedges.is_none()
        && args.left.is_none()
        && args.right.is_none()
        && args.rank.is_none()
        && args.basis.is_none()
        && args.target_vec.is_none()
        && args.bounds.is_none()
        && args.terminals.is_none()
        && args.tree.is_none()
        && args.required_edges.is_none()
        && args.bound.is_none()
        && args.pattern.is_none()
        && args.strings.is_none()
        && args.arcs.is_none()
        && args.deadlines.is_none()
        && args.precedence_pairs.is_none()
        && args.task_lengths.is_none()
        && args.deadline.is_none()
        && args.num_processors.is_none()
        && args.alphabet_size.is_none()
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
        "G" => match graph_type {
            Some("KingsSubgraph" | "TriangularSubgraph") => "integer positions: \"0,0;1,0;1,1\"",
            Some("UnitDiskGraph") => "float positions: \"0.0,0.0;1.0,0.0\"",
            _ => "edge list: 0-1,1-2,2-3",
        },
        "Vec<W>" => "comma-separated: 1,2,3",
        "Vec<CNFClause>" => "semicolon-separated clauses: \"1,2;-1,3\"",
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
        "GraphPartitioning" => "--graph 0-1,1-2,2-3,0-2,1-3,0-3",
        "HamiltonianPath" => "--graph 0-1,1-2,2-3",
        "IsomorphicSpanningTree" => "--graph 0-1,1-2,0-2 --tree 0-1,1-2",
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1"
        }
        "Satisfiability" => "--num-vars 3 --clauses \"1,2;-1,3\"",
        "KSatisfiability" => "--num-vars 3 --clauses \"1,2,3;-1,2,-3\" --k 3",
        "QUBO" => "--matrix \"1,0.5;0.5,2\"",
        "SpinGlass" => "--graph 0-1,1-2 --couplings 1,1",
        "KColoring" => "--graph 0-1,1-2,2-0 --k 3",
        "MinimumSumMulticenter" => {
            "--graph 0-1,1-2,2-3 --weights 1,1,1,1 --edge-weights 1,1,1 --k 2"
        }
        "PartitionIntoTriangles" => "--graph 0-1,1-2,0-2",
        "Factoring" => "--target 15 --m 4 --n 4",
        "SteinerTree" => "--graph 0-1,1-2,1-3,3-4 --edge-weights 2,2,1,1 --terminals 0,2,4",
        "OptimalLinearArrangement" => "--graph 0-1,1-2,2-3 --bound 5",
        "MinimumFeedbackArcSet" => "--arcs \"0>1,1>2,2>0\"",
        "RuralPostman" => {
            "--graph 0-1,1-2,2-3,3-0 --edge-weights 1,1,1,1 --required-edges 0,2 --bound 4"
        }
        "SubgraphIsomorphism" => "--graph 0-1,1-2,2-0 --pattern 0-1",
        "SubsetSum" => "--sizes 3,7,1,8,2,4 --target 11",
        "ShortestCommonSupersequence" => "--strings \"0,1,2;1,2,0\" --bound 4",
        _ => "",
    }
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
            // For geometry variants, show --positions instead of --graph
            if field.type_name == "G" && is_geometry {
                let hint = type_format_hint(&field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({hint})", "positions", field.description);
                if graph_type == Some("UnitDiskGraph") {
                    eprintln!("  --{:<16} Distance threshold [default: 1.0]", "radius");
                }
            } else if field.type_name == "DirectedGraph" {
                // DirectedGraph fields use --arcs, not --graph
                let hint = type_format_hint(&field.type_name, graph_type);
                eprintln!("  --{:<16} {} ({})", "arcs", field.description, hint);
            } else {
                let hint = type_format_hint(&field.type_name, graph_type);
                eprintln!(
                    "  --{:<16} {} ({})",
                    field.name.replace('_', "-"),
                    field.description,
                    hint
                );
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
    let resolved = resolve_problem_ref(problem, &rgraph)?;
    let canonical = &resolved.name;
    let resolved_variant = resolved.variant;
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

    let (data, variant) = match canonical.as_str() {
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

        // Hamiltonian path (graph only, no weights)
        "HamiltonianPath" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!("{e}\n\nUsage: pred create HamiltonianPath --graph 0-1,1-2,2-3")
            })?;
            (ser(HamiltonianPath::new(graph))?, resolved_variant.clone())
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

        // Graph problems with edge weights
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create {} --graph 0-1,1-2,2-3 [--edge-weights 1,1,1]",
                    problem
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let data = match canonical.as_str() {
                "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                _ => unreachable!(),
            };
            (data, resolved_variant.clone())
        }

        // RuralPostman
        "RuralPostman" => {
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

        // BicliqueCover
        "BicliqueCover" => {
            let left = args.left.ok_or_else(|| {
                anyhow::anyhow!(
                    "BicliqueCover requires --left, --right, --biedges, and --k\n\n\
                     Usage: pred create BicliqueCover --left 2 --right 2 --biedges 0-0,0-1,1-1 --k 2"
                )
            })?;
            let right = args.right.ok_or_else(|| {
                anyhow::anyhow!("BicliqueCover requires --right (right partition size)")
            })?;
            let k = args.k.ok_or_else(|| {
                anyhow::anyhow!("BicliqueCover requires --k (number of bicliques)")
            })?;
            let edges_str = args.biedges.as_deref().ok_or_else(|| {
                anyhow::anyhow!("BicliqueCover requires --biedges (e.g., 0-0,0-1,1-1)")
            })?;
            let edges = util::parse_edge_pairs(edges_str)?;
            let graph = BipartiteGraph::new(left, right, edges);
            (ser(BicliqueCover::new(graph, k))?, resolved_variant.clone())
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

        // LongestCommonSubsequence
        "LongestCommonSubsequence" => {
            let strings_str = args.strings.as_deref().ok_or_else(|| {
                anyhow::anyhow!(
                    "LCS requires --strings\n\n\
                     Usage: pred create LCS --strings \"ABAC;BACA\""
                )
            })?;
            let strings: Vec<Vec<u8>> = strings_str
                .split(';')
                .map(|s| s.trim().as_bytes().to_vec())
                .collect();
            (
                ser(LongestCommonSubsequence::new(strings))?,
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

        // OptimalLinearArrangement — graph + bound
        "OptimalLinearArrangement" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create OptimalLinearArrangement --graph 0-1,1-2,2-3 --bound 5"
                )
            })?;
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!(
                    "OptimalLinearArrangement requires --bound (upper bound K on total edge length)\n\n\
                     Usage: pred create OptimalLinearArrangement --graph 0-1,1-2,2-3 --bound 5"
                )
            })? as usize;
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
            let bound = args.bound.ok_or_else(|| {
                anyhow::anyhow!("ShortestCommonSupersequence requires --bound\n\n{usage}")
            })? as usize;
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

/// Parse `--graph` into a SimpleGraph, inferring num_vertices from max index.
fn parse_graph(args: &CreateArgs) -> Result<(SimpleGraph, usize)> {
    let edges_str = args
        .graph
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --graph (e.g., 0-1,1-2,2-3)"))?;

    if edges_str.trim().is_empty() {
        bail!(
            "Empty graph string. To create a graph with isolated vertices, use:\n  \
             pred create <PROBLEM> --random --num-vertices N --edge-prob 0.0"
        );
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

    let num_vertices = edges
        .iter()
        .flat_map(|(u, v)| [*u, *v])
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);

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
        .ok_or_else(|| anyhow::anyhow!("SteinerTree requires --terminals (e.g., \"0,2,4\")"))?;
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
    let sets_str = args
        .sets
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --sets (e.g., \"0,1;1,2;0,2\")"))?;
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

/// Parse `--weights` for set-based problems (i32), defaulting to all 1s.
fn parse_set_weights(args: &CreateArgs, num_sets: usize) -> Result<Vec<i32>> {
    match &args.weights {
        Some(w) => {
            let weights: Vec<i32> = util::parse_comma_list(w)?;
            if weights.len() != num_sets {
                bail!("Expected {} weights but got {}", num_sets, weights.len());
            }
            Ok(weights)
        }
        None => Ok(vec![1i32; num_sets]),
    }
}

/// Parse `--matrix` as semicolon-separated rows of comma-separated bool values (0/1).
/// E.g., "1,0;0,1;1,1"
fn parse_bool_matrix(args: &CreateArgs) -> Result<Vec<Vec<bool>>> {
    let matrix_str = args
        .matrix
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --matrix (e.g., \"1,0;0,1;1,1\")"))?;
    matrix_str
        .split(';')
        .map(|row| {
            row.trim()
                .split(',')
                .map(|s| match s.trim() {
                    "1" | "true" => Ok(true),
                    "0" | "false" => Ok(false),
                    other => Err(anyhow::anyhow!(
                        "Invalid boolean value '{}': expected 0/1 or true/false",
                        other
                    )),
                })
                .collect()
        })
        .collect()
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
            let graph = util::create_random_graph(num_vertices, edge_prob, args.seed);
            let edge_weights = vec![1i32; graph.num_edges()];
            let num_terminals = std::cmp::max(2, num_vertices * 2 / 5);
            let mut indices: Vec<usize> = (0..num_vertices).collect();
            // Deterministic shuffle using seed (Fisher-Yates with simple LCG)
            let mut rng_state: u64 = args.seed.unwrap_or(42) as u64;
            for i in (1..indices.len()).rev() {
                rng_state = rng_state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let j = (rng_state >> 33) as usize % (i + 1);
                indices.swap(i, j);
            }
            let mut terminals: Vec<usize> = indices[..num_terminals].to_vec();
            terminals.sort_unstable();
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
            let bound = args
                .bound
                .map(|b| b as usize)
                .unwrap_or((n.saturating_sub(1)) * graph.num_edges());
            let variant = variant_map(&[("graph", "SimpleGraph")]);
            (ser(OptimalLinearArrangement::new(graph, bound))?, variant)
        }

        _ => bail!(
            "Random generation is not supported for {canonical}. \
             Supported: graph-based problems (MIS, MVC, MaxCut, MaxClique, \
             MaximumMatching, MinimumDominatingSet, SpinGlass, KColoring, TravelingSalesman, \
             SteinerTree, OptimalLinearArrangement, HamiltonianPath)"
        ),
    };

    let output = ProblemJsonOutput {
        problem_type: canonical.to_string(),
        variant,
        data,
    };

    emit_problem_output(&output, out)
}
