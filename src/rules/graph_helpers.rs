//! Shared helpers for graph-based reductions.

use crate::topology::{Graph, SimpleGraph};

/// Order the vertices of a selected Hamiltonian cycle.
/// Target feasibility and the reduction's premises establish a single cycle.
pub(crate) fn edges_to_cycle_order<G: Graph>(graph: &G, target_solution: &[bool]) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut adjacency = vec![Vec::new(); n];
    for ((u, v), &selected) in graph.edges().into_iter().zip(target_solution) {
        if selected {
            adjacency[u].push(v);
            adjacency[v].push(u);
        }
    }
    let mut order = Vec::with_capacity(n);
    let mut previous = n;
    let mut current = 0;
    for _ in 0..n {
        order.push(current);
        let neighbors = &adjacency[current];
        let next = neighbors[usize::from(neighbors[0] == previous)];
        previous = current;
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
