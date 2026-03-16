//! Shared utilities for CLI and MCP: parsing helpers and random generation.

use anyhow::{bail, Result};
use num_bigint::BigUint;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;
use problemreductions::variant::{K2, K3, KN};
use serde::Serialize;
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// K-parameter validation
// ---------------------------------------------------------------------------

/// Derive the k variant string from a numeric k value.
fn k_variant_str(k: usize) -> &'static str {
    match k {
        1 => "K1",
        2 => "K2",
        3 => "K3",
        4 => "K4",
        5 => "K5",
        _ => "KN",
    }
}

/// Validate that `--k` (or `params.k`) is consistent with a variant suffix
/// (e.g., `/K2`). Returns the effective k value and variant map.
///
/// Rules:
/// - If the resolved variant has a specific k (e.g., K2), `k_flag` must
///   either be `None` or match. A mismatch is an error.
/// - If the resolved variant has k=KN (or no k), any `k_flag` is accepted.
/// - If `k_flag` is `None`, k is inferred from the variant (K2→2, K3→3, etc.),
///   or defaults to `default_k`.
pub fn validate_k_param(
    resolved_variant: &BTreeMap<String, String>,
    k_flag: Option<usize>,
    default_k: Option<usize>,
    problem_name: &str,
) -> Result<(usize, BTreeMap<String, String>)> {
    let variant_k_str = resolved_variant.get("k").map(|s| s.as_str());
    let variant_k_num: Option<usize> = match variant_k_str {
        Some("K1") => Some(1),
        Some("K2") => Some(2),
        Some("K3") => Some(3),
        Some("K4") => Some(4),
        Some("K5") => Some(5),
        _ => None, // KN or absent
    };

    let effective_k = match (k_flag, variant_k_num) {
        (Some(flag), Some(from_variant)) if flag != from_variant => {
            bail!(
                "{problem_name}: --k {flag} conflicts with variant /{} (k={from_variant}). \
                 Either omit the suffix or match the --k value.",
                variant_k_str.unwrap()
            );
        }
        (Some(flag), _) => flag,
        (None, Some(from_variant)) => from_variant,
        (None, None) => match default_k {
            Some(d) => d,
            None => bail!("{problem_name} requires --k <value>"),
        },
    };

    // Build the variant map with the effective k
    let mut variant = resolved_variant.clone();
    variant.insert("k".to_string(), k_variant_str(effective_k).to_string());

    Ok((effective_k, variant))
}

// ---------------------------------------------------------------------------
// K-problem serialization
// ---------------------------------------------------------------------------

/// Serialize a KColoring instance given a graph and validated k.
pub fn ser_kcoloring(
    graph: SimpleGraph,
    k: usize,
) -> Result<(serde_json::Value, BTreeMap<String, String>)> {
    match k {
        2 => Ok((
            ser(KColoring::<K2, SimpleGraph>::new(graph))?,
            variant_map(&[("k", "K2"), ("graph", "SimpleGraph")]),
        )),
        3 => Ok((
            ser(KColoring::<K3, SimpleGraph>::new(graph))?,
            variant_map(&[("k", "K3"), ("graph", "SimpleGraph")]),
        )),
        _ => Ok((
            ser(KColoring::<KN, SimpleGraph>::with_k(graph, k))?,
            variant_map(&[("k", "KN"), ("graph", "SimpleGraph")]),
        )),
    }
}

/// Serialize a KSatisfiability instance given clauses and validated k.
pub fn ser_ksat(
    num_vars: usize,
    clauses: Vec<CNFClause>,
    k: usize,
) -> Result<(serde_json::Value, BTreeMap<String, String>)> {
    match k {
        2 => Ok((
            ser(KSatisfiability::<K2>::new(num_vars, clauses))?,
            variant_map(&[("k", "K2")]),
        )),
        3 => Ok((
            ser(KSatisfiability::<K3>::new(num_vars, clauses))?,
            variant_map(&[("k", "K3")]),
        )),
        _ => Ok((
            ser(KSatisfiability::<KN>::new(num_vars, clauses))?,
            variant_map(&[("k", "KN")]),
        )),
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

/// Parse semicolon-separated x,y pairs from a string.
pub fn parse_positions<T: std::str::FromStr>(pos_str: &str, example: &str) -> Result<Vec<(T, T)>>
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

// ---------------------------------------------------------------------------
// Random generation (LCG-based)
// ---------------------------------------------------------------------------

/// LCG PRNG step — returns next state and a uniform f64 in [0, 1).
pub fn lcg_step(state: &mut u64) -> f64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*state >> 33) as f64 / (1u64 << 31) as f64
}

/// Initialize LCG state from seed or system time.
pub fn lcg_init(seed: Option<u64>) -> u64 {
    seed.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    })
}

/// Generate a random Erdos-Renyi graph using a simple LCG PRNG.
pub fn create_random_graph(num_vertices: usize, edge_prob: f64, seed: Option<u64>) -> SimpleGraph {
    let mut state = lcg_init(seed);
    let mut edges = Vec::new();
    for i in 0..num_vertices {
        for j in (i + 1)..num_vertices {
            let rand_val = lcg_step(&mut state);
            if rand_val < edge_prob {
                edges.push((i, j));
            }
        }
    }
    SimpleGraph::new(num_vertices, edges)
}

/// Generate random unique integer positions on a grid for KingsSubgraph/TriangularSubgraph.
pub fn create_random_int_positions(num_vertices: usize, seed: Option<u64>) -> Vec<(i32, i32)> {
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
pub fn create_random_float_positions(num_vertices: usize, seed: Option<u64>) -> Vec<(f64, f64)> {
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

// ---------------------------------------------------------------------------
// Small shared helpers
// ---------------------------------------------------------------------------

pub fn ser<T: Serialize>(problem: T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(problem)?)
}

pub fn variant_map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Parse a comma-separated list of values.
pub fn parse_comma_list<T: std::str::FromStr>(s: &str) -> Result<Vec<T>>
where
    T::Err: std::fmt::Display,
{
    s.split(',')
        .map(|v| {
            v.trim()
                .parse::<T>()
                .map_err(|e| anyhow::anyhow!("Invalid value '{}': {e}", v.trim()))
        })
        .collect()
}

pub fn parse_decimal_biguint(s: &str) -> Result<BigUint> {
    BigUint::parse_bytes(s.trim().as_bytes(), 10)
        .ok_or_else(|| anyhow::anyhow!("Invalid decimal integer '{}'", s.trim()))
}

pub fn parse_biguint_list(s: &str) -> Result<Vec<BigUint>> {
    s.split(',')
        .map(|value| parse_decimal_biguint(value.trim()))
        .collect()
}

/// Parse edge pairs like "0-1,1-2,2-3" into Vec<(usize, usize)>.
pub fn parse_edge_pairs(s: &str) -> Result<Vec<(usize, usize)>> {
    s.split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split('-').collect();
            if parts.len() != 2 {
                bail!("Invalid edge '{}': expected format u-v", pair.trim());
            }
            let u: usize = parts[0].trim().parse()?;
            let v: usize = parts[1].trim().parse()?;
            Ok((u, v))
        })
        .collect()
}
