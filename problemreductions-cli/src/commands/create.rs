use crate::cli::CreateArgs;
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::{parse_problem_spec, resolve_variant};
use crate::util;
use anyhow::{bail, Context, Result};
use problemreductions::models::algebraic::{ClosestVectorProblem, BMF};
use problemreductions::models::graph::GraphPartitioning;
use problemreductions::models::misc::{BinPacking, LongestCommonSubsequence, PaintShop, SubsetSum};
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
        && args.strings.is_none()
        && args.arcs.is_none()
        && args.potential_edges.is_none()
        && args.budget.is_none()
}

fn type_format_hint(type_name: &str, graph_type: Option<&str>) -> &'static str {
    match type_name {
        "G" => match graph_type {
            Some("KingsSubgraph" | "TriangularSubgraph") => "integer positions: \"0,0;1,0;1,1\"",
            Some("UnitDiskGraph") => "float positions: \"0.0,0.0;1.0,0.0\"",
            _ => "edge list: 0-1,1-2,2-3",
        },
        "Vec<W>" => "comma-separated: 1,2,3",
        "Vec<(usize, usize, W)>" | "Vec<(usize,usize,W)>" => {
            "comma-separated weighted edges: 0-2:3,1-3:5"
        }
        "Vec<CNFClause>" => "semicolon-separated clauses: \"1,2;-1,3\"",
        "Vec<Vec<W>>" => "semicolon-separated rows: \"1,0.5;0.5,2\"",
        "usize" | "W::Sum" => "integer",
        "u64" => "integer",
        "i64" => "integer",
        "Vec<i64>" => "comma-separated integers: 3,7,1,8",
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
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1"
        }
        "BiconnectivityAugmentation" => {
            "--graph 0-1,1-2,2-3 --potential-edges 0-2:3,0-3:4,1-3:2 --budget 5"
        }
        "Satisfiability" => "--num-vars 3 --clauses \"1,2;-1,3\"",
        "KSatisfiability" => "--num-vars 3 --clauses \"1,2,3;-1,2,-3\" --k 3",
        "QUBO" => "--matrix \"1,0.5;0.5,2\"",
        "SpinGlass" => "--graph 0-1,1-2 --couplings 1,1",
        "KColoring" => "--graph 0-1,1-2,2-0 --k 3",
        "Factoring" => "--target 15 --m 4 --n 4",
        "SubsetSum" => "--sizes 3,7,1,8,2,4 --target 11",
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
            } else {
                let hint = type_format_hint(&field.type_name, graph_type);
                let flag_name = if field.name == "potential_weights" {
                    "potential-edges".to_string()
                } else {
                    field.name.replace('_', "-")
                };
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

/// Resolve the graph type from the variant map (e.g., "KingsSubgraph", "UnitDiskGraph", or "SimpleGraph").
fn resolved_graph_type(variant: &BTreeMap<String, String>) -> &str {
    variant
        .get("graph")
        .map(|s| s.as_str())
        .unwrap_or("SimpleGraph")
}

pub fn create(args: &CreateArgs, out: &OutputConfig) -> Result<()> {
    let spec = parse_problem_spec(&args.problem)?;
    let canonical = &spec.name;

    // Resolve variant early so random and help can use it
    let rgraph = problemreductions::rules::ReductionGraph::new();
    let known_variants = rgraph.variants_for(canonical);
    let resolved_variant = if known_variants.is_empty() {
        BTreeMap::new()
    } else {
        resolve_variant(&spec, &known_variants)?
    };
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

        // Graph problems with edge weights
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create {} --graph 0-1,1-2,2-3 [--edge-weights 1,1,1]",
                    args.problem
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
                     Usage: pred create 3SAT --num-vars 3 --clauses \"1,2,3;-1,2,-3\""
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
                .ok_or_else(|| anyhow::anyhow!("Factoring requires --target\n\n{usage}"))?;
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
            let target = args.target.ok_or_else(|| {
                anyhow::anyhow!(
                    "SubsetSum requires --target\n\n\
                     Usage: pred create SubsetSum --sizes 3,7,1,8,2,4 --target 11"
                )
            })?;
            let sizes: Vec<i64> = util::parse_comma_list(sizes_str)?;
            (
                ser(SubsetSum::new(sizes, target as i64))?,
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

        // MinimumFeedbackVertexSet
        "MinimumFeedbackVertexSet" => {
            let arcs_str = args.arcs.as_ref().ok_or_else(|| {
                anyhow::anyhow!(
                    "MinimumFeedbackVertexSet requires --arcs\n\n\
                     Usage: pred create FVS --arcs \"0>1,1>2,2>0\" [--weights 1,1,1] [--num-vertices N]"
                )
            })?;
            let arcs: Vec<(usize, usize)> = arcs_str
                .split(',')
                .map(|s| {
                    let parts: Vec<&str> = s.split('>').collect();
                    anyhow::ensure!(
                        parts.len() == 2,
                        "Invalid arc format '{}', expected 'u>v'",
                        s
                    );
                    Ok((
                        parts[0].trim().parse::<usize>()?,
                        parts[1].trim().parse::<usize>()?,
                    ))
                })
                .collect::<Result<Vec<_>>>()?;
            let inferred_num_v = arcs
                .iter()
                .flat_map(|&(u, v)| [u, v])
                .max()
                .map(|m| m + 1)
                .unwrap_or(0);
            let num_v = match args.num_vertices {
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
            let graph = DirectedGraph::new(num_v, arcs);
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

    let json = serde_json::to_value(&output)?;

    if let Some(ref path) = out.output {
        let content = serde_json::to_string_pretty(&json).context("Failed to serialize JSON")?;
        std::fs::write(path, &content)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        out.info(&format!("Wrote {}", path.display()));
    } else {
        // Print JSON to stdout so data is not lost (consistent with reduce)
        println!("{}", serde_json::to_string_pretty(&json)?);
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
                    args.problem
                )
            })?;
            let weights = parse_vertex_weights(args, n)?;
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
            bail!("Potential edge {}-{} already exists in the graph", edge.0, edge.1);
        }
        if !seen_potential_edges.insert(edge) {
            bail!("Duplicate potential edge {}-{} is not allowed", edge.0, edge.1);
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
            args.problem
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

        _ => bail!(
            "Random generation is not supported for {canonical}. \
             Supported: graph-based problems (MIS, MVC, MaxCut, MaxClique, \
             MaximumMatching, MinimumDominatingSet, SpinGlass, KColoring, TravelingSalesman)"
        ),
    };

    let output = ProblemJsonOutput {
        problem_type: canonical.to_string(),
        variant,
        data,
    };

    let json = serde_json::to_value(&output)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_args() -> CreateArgs {
        CreateArgs {
            problem: "BiconnectivityAugmentation".to_string(),
            graph: None,
            weights: None,
            edge_weights: None,
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
            sizes: None,
            capacity: None,
            sequence: None,
            sets: None,
            universe: None,
            biedges: None,
            left: None,
            right: None,
            rank: None,
            basis: None,
            target_vec: None,
            bounds: None,
            strings: None,
            arcs: None,
            potential_edges: None,
            budget: None,
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
}
