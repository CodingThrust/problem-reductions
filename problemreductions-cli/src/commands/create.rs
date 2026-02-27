use crate::cli::CreateArgs;
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::{parse_problem_spec, resolve_variant};
use anyhow::{bail, Context, Result};
use problemreductions::prelude::*;
use problemreductions::registry::collect_schemas;
use problemreductions::topology::{
    Graph, KingsSubgraph, SimpleGraph, TriangularSubgraph, UnitDiskGraph,
};
use problemreductions::variant::{K2, K3, KN};
use serde::Serialize;
use std::collections::BTreeMap;

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
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            "--graph 0-1,1-2,2-3 --edge-weights 1,1,1"
        }
        "Satisfiability" => "--num-vars 3 --clauses \"1,2;-1,3\"",
        "KSatisfiability" => "--num-vars 3 --clauses \"1,2,3;-1,2,-3\" --k 3",
        "QUBO" => "--matrix \"1,0.5;0.5,2\"",
        "SpinGlass" => "--graph 0-1,1-2 --couplings 1,1",
        "KColoring" => "--graph 0-1,1-2,2-0 --k 3",
        "Factoring" => "--target 15 --m 4 --n 4",
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
                eprintln!(
                    "  --{:<16} {} ({})",
                    field.name.replace('_', "-"),
                    field.description,
                    hint
                );
            }
        }
    } else {
        eprintln!("{canonical}\n");
        eprintln!("No schema information available.");
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

    // Show schema-driven help when no data flags are provided
    if all_data_flags_empty(args) {
        let gt = if graph_type != "SimpleGraph" {
            Some(graph_type)
        } else {
            None
        };
        return print_problem_help(canonical, gt);
    }

    let (data, variant) = match canonical.as_str() {
        // Graph problems with vertex weights
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet" => {
            create_vertex_weight_problem(args, canonical, graph_type, &resolved_variant)?
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
            let data = match args.k {
                Some(2) => ser(KColoring::<K2, SimpleGraph>::new(graph))?,
                Some(3) => ser(KColoring::<K3, SimpleGraph>::new(graph))?,
                Some(k) => ser(KColoring::<KN, SimpleGraph>::with_k(graph, k))?,
                None => bail!(
                    "KColoring requires --k <num_colors>\n\n\
                     Usage: pred create KColoring --graph 0-1,1-2,2-0 --k 3"
                ),
            };
            (data, resolved_variant.clone())
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
            let data = match args.k {
                Some(2) => ser(KSatisfiability::<K2>::new(num_vars, clauses))?,
                Some(3) => ser(KSatisfiability::<K3>::new(num_vars, clauses))?,
                _ => ser(KSatisfiability::<KN>::new(num_vars, clauses))?,
            };
            (data, resolved_variant.clone())
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
            let couplings = parse_couplings(args, graph.num_edges())?;
            let fields = parse_fields(args, n)?;
            (
                ser(SpinGlass::from_graph(graph, couplings, fields))?,
                resolved_variant.clone(),
            )
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
            let data = match canonical {
                "MaximumIndependentSet" => ser(MaximumIndependentSet::new(graph, weights))?,
                "MinimumVertexCover" => ser(MinimumVertexCover::new(graph, weights))?,
                "MaximumClique" => ser(MaximumClique::new(graph, weights))?,
                "MinimumDominatingSet" => ser(MinimumDominatingSet::new(graph, weights))?,
                _ => unreachable!(),
            };
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
        _ => unreachable!(),
    }
}

fn ser<T: Serialize>(problem: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(problem)?)
}

fn variant_map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Parse `--graph` into a SimpleGraph, inferring num_vertices from max index.
fn parse_graph(args: &CreateArgs) -> Result<(SimpleGraph, usize)> {
    let edges_str = args
        .graph
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --graph (e.g., 0-1,1-2,2-3)"))?;

    let edges: Vec<(usize, usize)> = edges_str
        .split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split('-').collect();
            if parts.len() != 2 {
                bail!("Invalid edge '{}': expected format u-v", pair.trim());
            }
            let u: usize = parts[0].parse()?;
            let v: usize = parts[1].parse()?;
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

/// Parse semicolon-separated x,y pairs from a string.
fn parse_positions<T: std::str::FromStr>(pos_str: &str, example: &str) -> Result<Vec<(T, T)>>
where
    T::Err: std::fmt::Display,
{
    pos_str
        .split(';')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split(',').collect();
            if parts.len() != 2 {
                bail!(
                    "Invalid position '{}': expected format x,y (e.g., {example})",
                    pair.trim()
                );
            }
            let x: T = parts[0]
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid x in '{}': {e}", pair.trim()))?;
            let y: T = parts[1]
                .trim()
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid y in '{}': {e}", pair.trim()))?;
            Ok((x, y))
        })
        .collect()
}

/// Parse `--positions` as integer grid positions.
fn parse_int_positions(args: &CreateArgs) -> Result<Vec<(i32, i32)>> {
    let pos_str = args.positions.as_deref().ok_or_else(|| {
        anyhow::anyhow!("This variant requires --positions (e.g., \"0,0;1,0;1,1\")")
    })?;
    parse_positions(pos_str, "0,0")
}

/// Parse `--positions` as float positions.
fn parse_float_positions(args: &CreateArgs) -> Result<Vec<(f64, f64)>> {
    let pos_str = args.positions.as_deref().ok_or_else(|| {
        anyhow::anyhow!("This variant requires --positions (e.g., \"0.0,0.0;1.0,0.0;0.5,0.87\")")
    })?;
    parse_positions(pos_str, "0.0,0.0")
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

/// Generate a random Erdos-Renyi graph using a simple LCG PRNG (no external dependency).
fn create_random_graph(num_vertices: usize, edge_prob: f64, seed: Option<u64>) -> SimpleGraph {
    let mut state: u64 = seed.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    });

    let mut edges = Vec::new();
    for i in 0..num_vertices {
        for j in (i + 1)..num_vertices {
            // LCG step
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let rand_val = (state >> 33) as f64 / (1u64 << 31) as f64;
            if rand_val < edge_prob {
                edges.push((i, j));
            }
        }
    }

    SimpleGraph::new(num_vertices, edges)
}

/// LCG PRNG step — returns next state and a uniform f64 in [0, 1).
fn lcg_step(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*state >> 33) as f64 / (1u64 << 31) as f64
}

/// Initialize LCG state from seed or system time.
fn lcg_init(seed: Option<u64>) -> u64 {
    seed.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    })
}

/// Generate random unique integer positions on a grid for KingsSubgraph/TriangularSubgraph.
fn create_random_int_positions(num_vertices: usize, seed: Option<u64>) -> Vec<(i32, i32)> {
    let mut state = lcg_init(seed);
    let grid_size = (num_vertices as f64).sqrt().ceil() as i32 + 1;
    let mut positions = std::collections::BTreeSet::new();
    while positions.len() < num_vertices {
        let x = (lcg_step(&mut state) * grid_size as f64) as i32;
        let y = (lcg_step(&mut state) * grid_size as f64) as i32;
        positions.insert((x, y));
    }
    positions.into_iter().collect()
}

/// Generate random float positions in [0, sqrt(N)] x [0, sqrt(N)] for UnitDiskGraph.
fn create_random_float_positions(num_vertices: usize, seed: Option<u64>) -> Vec<(f64, f64)> {
    let mut state = lcg_init(seed);
    let side = (num_vertices as f64).sqrt();
    (0..num_vertices)
        .map(|_| {
            let x = lcg_step(&mut state) * side;
            let y = lcg_step(&mut state) * side;
            (x, y)
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
            args.problem
        )
    })?;

    let graph_type = resolved_graph_type(resolved_variant);

    let (data, variant) = match canonical {
        // Graph problems with vertex weights
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet" => {
            let weights = vec![1i32; num_vertices];
            match graph_type {
                "KingsSubgraph" => {
                    let positions = create_random_int_positions(num_vertices, args.seed);
                    let graph = KingsSubgraph::new(positions);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                "TriangularSubgraph" => {
                    let positions = create_random_int_positions(num_vertices, args.seed);
                    let graph = TriangularSubgraph::new(positions);
                    (
                        ser_vertex_weight_problem_with(canonical, graph, weights)?,
                        resolved_variant.clone(),
                    )
                }
                "UnitDiskGraph" => {
                    let radius = args.radius.unwrap_or(1.0);
                    let positions = create_random_float_positions(num_vertices, args.seed);
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
                    let graph = create_random_graph(num_vertices, edge_prob, args.seed);
                    let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
                    let data = match canonical {
                        "MaximumIndependentSet" => ser(MaximumIndependentSet::new(graph, weights))?,
                        "MinimumVertexCover" => ser(MinimumVertexCover::new(graph, weights))?,
                        "MaximumClique" => ser(MaximumClique::new(graph, weights))?,
                        "MinimumDominatingSet" => ser(MinimumDominatingSet::new(graph, weights))?,
                        _ => unreachable!(),
                    };
                    (data, variant)
                }
            }
        }

        // Graph problems with edge weights
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            let edge_prob = args.edge_prob.unwrap_or(0.5);
            if !(0.0..=1.0).contains(&edge_prob) {
                bail!("--edge-prob must be between 0.0 and 1.0");
            }
            let graph = create_random_graph(num_vertices, edge_prob, args.seed);
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
            let graph = create_random_graph(num_vertices, edge_prob, args.seed);
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
            let graph = create_random_graph(num_vertices, edge_prob, args.seed);
            let k = args.k.unwrap_or(3);
            let variant;
            let data;
            match k {
                2 => {
                    variant = variant_map(&[("k", "K2"), ("graph", "SimpleGraph")]);
                    data = ser(KColoring::<K2, SimpleGraph>::new(graph))?;
                }
                3 => {
                    variant = variant_map(&[("k", "K3"), ("graph", "SimpleGraph")]);
                    data = ser(KColoring::<K3, SimpleGraph>::new(graph))?;
                }
                _ => {
                    variant = variant_map(&[("k", "KN"), ("graph", "SimpleGraph")]);
                    data = ser(KColoring::<KN, SimpleGraph>::with_k(graph, k))?;
                }
            }
            (data, variant)
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
