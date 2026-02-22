use crate::cli::CreateArgs;
use crate::dispatch::ProblemJsonOutput;
use crate::output::OutputConfig;
use crate::problem_name::resolve_alias;
use anyhow::{bail, Context, Result};
use problemreductions::prelude::*;
use problemreductions::topology::{Graph, SimpleGraph};
use problemreductions::variant::{K2, K3, KN};
use serde::Serialize;
use std::collections::BTreeMap;

pub fn create(args: &CreateArgs, out: &OutputConfig) -> Result<()> {
    let canonical = resolve_alias(&args.problem);

    if args.random {
        return create_random(args, &canonical, out);
    }

    let (data, variant) = match canonical.as_str() {
        // Graph problems with vertex weights
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet" => {
            let (graph, n) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create {} --edges 0-1,1-2,2-3 [--weights 1,1,1,1] --json -o problem.json",
                    args.problem
                )
            })?;
            let weights = parse_vertex_weights(args, n)?;
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            let data = match canonical.as_str() {
                "MaximumIndependentSet" => ser(MaximumIndependentSet::new(graph, weights))?,
                "MinimumVertexCover" => ser(MinimumVertexCover::new(graph, weights))?,
                "MaximumClique" => ser(MaximumClique::new(graph, weights))?,
                "MinimumDominatingSet" => ser(MinimumDominatingSet::new(graph, weights))?,
                _ => unreachable!(),
            };
            (data, variant)
        }

        // Graph problems with edge weights
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create {} --edges 0-1,1-2,2-3 [--weights 1,1,1] --json -o problem.json",
                    args.problem
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            let data = match canonical.as_str() {
                "MaxCut" => ser(MaxCut::new(graph, edge_weights))?,
                "MaximumMatching" => ser(MaximumMatching::new(graph, edge_weights))?,
                "TravelingSalesman" => ser(TravelingSalesman::new(graph, edge_weights))?,
                _ => unreachable!(),
            };
            (data, variant)
        }

        // KColoring
        "KColoring" => {
            let (graph, _) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create KColoring --edges 0-1,1-2,2-0 --k 3 --json -o problem.json"
                )
            })?;
            let variant;
            let data;
            match args.k {
                Some(2) => {
                    variant = variant_map(&[("k", "K2"), ("graph", "SimpleGraph")]);
                    data = ser(KColoring::<K2, SimpleGraph>::new(graph))?;
                }
                Some(3) => {
                    variant = variant_map(&[("k", "K3"), ("graph", "SimpleGraph")]);
                    data = ser(KColoring::<K3, SimpleGraph>::new(graph))?;
                }
                Some(k) => {
                    variant = variant_map(&[("k", "KN"), ("graph", "SimpleGraph")]);
                    data = ser(KColoring::<KN, SimpleGraph>::with_k(graph, k))?;
                }
                None => bail!(
                    "KColoring requires --k <num_colors>\n\n\
                     Usage: pred create KColoring --edges 0-1,1-2,2-0 --k 3 --json -o problem.json"
                ),
            }
            (data, variant)
        }

        // SAT
        "Satisfiability" => {
            let num_vars = args.num_vars.ok_or_else(|| {
                anyhow::anyhow!(
                    "Satisfiability requires --num-vars\n\n\
                     Usage: pred create SAT --num-vars 3 --clauses \"1,2;-1,3\" --json -o problem.json"
                )
            })?;
            let clauses = parse_clauses(args)?;
            let variant = BTreeMap::new();
            (ser(Satisfiability::new(num_vars, clauses))?, variant)
        }
        "KSatisfiability" => {
            let num_vars = args.num_vars.ok_or_else(|| {
                anyhow::anyhow!(
                    "KSatisfiability requires --num-vars\n\n\
                     Usage: pred create 3SAT --num-vars 3 --clauses \"1,2,3;-1,2,-3\" --json -o problem.json"
                )
            })?;
            let clauses = parse_clauses(args)?;
            let variant;
            let data;
            match args.k {
                Some(2) => {
                    variant = variant_map(&[("k", "K2")]);
                    data = ser(KSatisfiability::<K2>::new(num_vars, clauses))?;
                }
                Some(3) => {
                    variant = variant_map(&[("k", "K3")]);
                    data = ser(KSatisfiability::<K3>::new(num_vars, clauses))?;
                }
                _ => {
                    variant = variant_map(&[("k", "KN")]);
                    data = ser(KSatisfiability::<KN>::new(num_vars, clauses))?;
                }
            }
            (data, variant)
        }

        // QUBO
        "QUBO" => {
            let matrix = parse_matrix(args)?;
            let variant = BTreeMap::new();
            (ser(QUBO::from_matrix(matrix))?, variant)
        }

        // SpinGlass
        "SpinGlass" => {
            let (graph, n) = parse_graph(args).map_err(|e| {
                anyhow::anyhow!(
                    "{e}\n\nUsage: pred create SpinGlass --edges 0-1,1-2 [--weights 1,1] --json -o problem.json"
                )
            })?;
            let edge_weights = parse_edge_weights(args, graph.num_edges())?;
            let fields = vec![0i32; n];
            let couplings = edge_weights;
            let variant = variant_map(&[("graph", "SimpleGraph"), ("weight", "i32")]);
            (
                ser(SpinGlass::from_graph(graph, couplings, fields))?,
                variant,
            )
        }

        // Factoring
        "Factoring" => {
            let usage = "Usage: pred create Factoring --target 15 --bits-m 4 --bits-n 4";
            let target = args
                .target
                .ok_or_else(|| anyhow::anyhow!("Factoring requires --target\n\n{usage}"))?;
            let m = args
                .bits_m
                .ok_or_else(|| anyhow::anyhow!("Factoring requires --bits-m\n\n{usage}"))?;
            let n = args
                .bits_n
                .ok_or_else(|| anyhow::anyhow!("Factoring requires --bits-n\n\n{usage}"))?;
            let variant = BTreeMap::new();
            (ser(Factoring::new(m, n, target))?, variant)
        }

        _ => bail!("{}", crate::problem_name::unknown_problem_error(&canonical)),
    };

    let output = ProblemJsonOutput {
        problem_type: canonical.clone(),
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

fn ser<T: Serialize>(problem: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(problem)?)
}

fn variant_map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Parse `--edges` into a SimpleGraph, inferring num_vertices from max index.
fn parse_graph(args: &CreateArgs) -> Result<(SimpleGraph, usize)> {
    let edges_str = args
        .edges
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("This problem requires --edges (e.g., 0-1,1-2,2-3)"))?;

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

/// Parse `--weights` as edge weights (i32), defaulting to all 1s.
fn parse_edge_weights(args: &CreateArgs, num_edges: usize) -> Result<Vec<i32>> {
    match &args.weights {
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

/// Handle `pred create <PROBLEM> --random ...`
fn create_random(args: &CreateArgs, canonical: &str, out: &OutputConfig) -> Result<()> {
    let num_vertices = args.num_vertices.ok_or_else(|| {
        anyhow::anyhow!(
            "--random requires --num-vertices\n\n\
             Usage: pred create {} --random --num-vertices 10 [--edge-prob 0.3] [--seed 42]",
            args.problem
        )
    })?;
    let edge_prob = args.edge_prob.unwrap_or(0.5);
    if !(0.0..=1.0).contains(&edge_prob) {
        bail!("--edge-prob must be between 0.0 and 1.0");
    }

    let graph = create_random_graph(num_vertices, edge_prob, args.seed);
    let num_edges = graph.num_edges();

    let (data, variant) = match canonical {
        // Graph problems with vertex weights
        "MaximumIndependentSet"
        | "MinimumVertexCover"
        | "MaximumClique"
        | "MinimumDominatingSet" => {
            let weights = vec![1i32; num_vertices];
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

        // Graph problems with edge weights
        "MaxCut" | "MaximumMatching" | "TravelingSalesman" => {
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
