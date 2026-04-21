//! Shared helpers for graph-based reductions.

use crate::topology::{Graph, SimpleGraph};

/// Extract a Hamiltonian cycle vertex ordering from edge-selection configs on complete graphs.
///
/// Given a graph and a binary `target_solution` over its edges (1 = selected),
/// walks the selected edges to produce a vertex permutation representing the cycle.
/// Returns `vec![0; n]` if the selection does not form a valid Hamiltonian cycle.
pub(crate) fn edges_to_cycle_order<G: Graph>(graph: &G, target_solution: &[usize]) -> Vec<usize> {
    let n = graph.num_vertices();
    if n == 0 {
        return vec![];
    }

    let edges = graph.edges();
    if target_solution.len() != edges.len() {
        return vec![0; n];
    }

    let mut adjacency = vec![Vec::new(); n];
    let mut selected_count = 0usize;
    for (idx, &selected) in target_solution.iter().enumerate() {
        if selected != 1 {
            continue;
        }
        let (u, v) = edges[idx];
        adjacency[u].push(v);
        adjacency[v].push(u);
        selected_count += 1;
    }

    if selected_count != n || adjacency.iter().any(|neighbors| neighbors.len() != 2) {
        return vec![0; n];
    }

    let mut order = Vec::with_capacity(n);
    let mut prev = None;
    let mut current = 0usize;

    for _ in 0..n {
        order.push(current);
        let neighbors = &adjacency[current];
        let next = match prev {
            Some(previous) => {
                if neighbors[0] == previous {
                    neighbors[1]
                } else {
                    neighbors[0]
                }
            }
            None => neighbors[0],
        };
        prev = Some(current);
        current = next;
    }

    order
}

/// Build the complement graph edges: edges between all non-adjacent vertex pairs.
pub(crate) fn complement_edges(graph: &SimpleGraph) -> Vec<(usize, usize)> {
    let n = graph.num_vertices();
    let mut edges = Vec::new();
    for u in 0..n {
        for v in (u + 1)..n {
            if !graph.has_edge(u, v) {
                edges.push((u, v));
            }
        }
    }
    edges
}
