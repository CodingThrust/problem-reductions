//! Bounded short-cycle hitting backend for PartialFeedbackEdgeSet.
//!
//! Enumerates simple cycles of length <= L, then does branch-and-bound
//! over edge removal decisions to hit all such cycles within budget K.

use crate::models::graph::PartialFeedbackEdgeSet;
use crate::topology::{Graph, SimpleGraph};

/// Find a witness (binary edge-removal vector) for PartialFeedbackEdgeSet,
/// or None if no solution exists within budget.
pub(crate) fn find_witness(problem: &PartialFeedbackEdgeSet<SimpleGraph>) -> Option<Vec<usize>> {
    let graph = problem.graph();
    let n = graph.num_vertices();
    let edges = graph.edges();
    let m = edges.len();
    let budget = problem.budget();
    let max_cycle_len = problem.max_cycle_length();

    if max_cycle_len < 3 || n < 3 {
        // No cycles possible
        return Some(vec![0; m]);
    }

    // Build adjacency list with edge indices
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (idx, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, idx));
        adj[v].push((u, idx));
    }

    // Enumerate all simple cycles of length <= max_cycle_len.
    // Each cycle is stored as a set of edge indices.
    let cycles = enumerate_short_cycles(n, &adj, max_cycle_len);

    if cycles.is_empty() {
        return Some(vec![0; m]);
    }

    // Branch-and-bound: find a set of at most `budget` edges that hits all cycles.
    let mut removed = vec![false; m];
    let mut best = None;

    hitting_set_search(&cycles, budget, m, &mut removed, 0, 0, &mut best);

    best.map(|rem| rem.iter().map(|&v| if v { 1 } else { 0 }).collect())
}

/// Enumerate all simple cycles of length <= max_len.
/// Returns cycles as sets of edge indices.
fn enumerate_short_cycles(
    n: usize,
    adj: &[Vec<(usize, usize)>],
    max_len: usize,
) -> Vec<Vec<usize>> {
    let mut cycles = Vec::new();
    let mut visited = vec![false; n];
    let mut path_edges = Vec::new();

    for start in 0..n {
        visited[start] = true;
        for &(neighbor, edge_idx) in &adj[start] {
            if neighbor <= start {
                continue;
            }
            visited[neighbor] = true;
            path_edges.push(edge_idx);
            cycle_dfs(
                adj,
                start,
                neighbor,
                1,
                max_len,
                &mut visited,
                &mut path_edges,
                &mut cycles,
            );
            path_edges.pop();
            visited[neighbor] = false;
        }
        visited[start] = false;
    }

    // Deduplicate cycles (same edge set can be found from different starting vertices)
    cycles.sort();
    cycles.dedup();
    cycles
}

fn cycle_dfs(
    adj: &[Vec<(usize, usize)>],
    start: usize,
    current: usize,
    path_length: usize,
    max_len: usize,
    visited: &mut [bool],
    path_edges: &mut Vec<usize>,
    cycles: &mut Vec<Vec<usize>>,
) {
    for &(neighbor, edge_idx) in &adj[current] {
        if neighbor == start {
            let cycle_length = path_length + 1;
            if cycle_length >= 3 && cycle_length <= max_len {
                let mut cycle_edges = path_edges.clone();
                cycle_edges.push(edge_idx);
                cycle_edges.sort_unstable();
                cycles.push(cycle_edges);
            }
            continue;
        }

        if visited[neighbor] || neighbor <= start || path_length + 1 >= max_len {
            continue;
        }

        visited[neighbor] = true;
        path_edges.push(edge_idx);
        cycle_dfs(
            adj,
            start,
            neighbor,
            path_length + 1,
            max_len,
            visited,
            path_edges,
            cycles,
        );
        path_edges.pop();
        visited[neighbor] = false;
    }
}

/// Branch-and-bound hitting set search.
///
/// Finds a set of at most `budget` edges (from `m` total) that hits all cycles.
fn hitting_set_search(
    cycles: &[Vec<usize>],
    budget: usize,
    _m: usize,
    removed: &mut Vec<bool>,
    removed_count: usize,
    start_cycle: usize,
    best: &mut Option<Vec<bool>>,
) {
    if best.is_some() {
        return;
    }

    // Find the first uncovered cycle
    let uncovered = cycles[start_cycle..]
        .iter()
        .enumerate()
        .find(|(_, cycle)| !cycle.iter().any(|&e| removed[e]));

    let uncovered = match uncovered {
        Some((offset, _)) => start_cycle + offset,
        None => {
            // All cycles are covered
            *best = Some(removed.clone());
            return;
        }
    };

    if removed_count >= budget {
        return; // Can't remove more edges
    }

    // Branch on each edge in the uncovered cycle
    let cycle = cycles[uncovered].clone();
    for &edge in &cycle {
        if removed[edge] {
            continue;
        }
        removed[edge] = true;
        hitting_set_search(cycles, budget, _m, removed, removed_count + 1, uncovered + 1, best);
        removed[edge] = false;
        if best.is_some() {
            return;
        }
    }
}
