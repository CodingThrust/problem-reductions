//! Common test utilities for mapping tests.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::rules::unitdiskmapping::MappingResult;
use crate::solvers::ILPSolver;

fn build_mis_ilp(num_vertices: usize, edges: &[(usize, usize)], weights: &[i64]) -> ILP<bool> {
    let constraints: Vec<LinearConstraint> = edges
        .iter()
        .map(|&(i, j)| LinearConstraint::le(vec![(i, 1), (j, 1)], 1))
        .collect();

    let objective: Vec<(usize, i64)> = weights.iter().enumerate().map(|(i, &w)| (i, w)).collect();

    ILP::<bool>::new(
        num_vertices,
        constraints,
        objective,
        ObjectiveSense::Maximize,
    )
    .expect("MIS test ILP must be valid")
}

/// Check if a configuration is a valid independent set.
pub fn is_independent_set(edges: &[(usize, usize)], config: &[usize]) -> bool {
    for &(u, v) in edges {
        if config[u] > 0 && config[v] > 0 {
            return false;
        }
    }
    true
}

/// Solve maximum independent set using ILP.
/// Returns the size of the MIS.
pub fn solve_mis(num_vertices: usize, edges: &[(usize, usize)]) -> usize {
    let weights = vec![1; num_vertices];
    let ilp = build_mis_ilp(num_vertices, edges, &weights);
    let solver = ILPSolver::new();
    solver
        .solve(&ilp)
        .expect("test MIS solver must return a solution")
        .iter()
        .filter(|&&x| x > 0)
        .count()
}

/// Solve MIS and return the binary configuration.
pub fn solve_mis_config(num_vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let weights = vec![1; num_vertices];
    let ilp = build_mis_ilp(num_vertices, edges, &weights);
    let solver = ILPSolver::new();
    solver
        .solve(&ilp)
        .expect("test MIS solver must return a solution")
        .iter()
        .map(|&x| if x > 0 { 1 } else { 0 })
        .collect()
}

/// Solve MIS on a Grid using ILPSolver (unweighted).
#[allow(dead_code)]
pub fn solve_grid_mis(result: &MappingResult) -> usize {
    let edges = result.edges();
    let num_vertices = result.positions.len();
    solve_mis(num_vertices, &edges)
}

/// Solve weighted MIS on a Grid using ILPSolver.
#[allow(dead_code)]
pub fn solve_weighted_grid_mis(result: &MappingResult) -> usize {
    let edges = result.edges();
    let num_vertices = result.positions.len();

    assert_eq!(result.node_weights.len(), num_vertices);

    usize::try_from(solve_weighted_mis(
        num_vertices,
        &edges,
        &result.node_weights,
    ))
    .expect("test weighted MIS value must fit in usize")
}

/// Solve weighted MIS on a graph using ILP.
/// Returns the maximum weighted independent set value.
pub fn solve_weighted_mis(num_vertices: usize, edges: &[(usize, usize)], weights: &[i64]) -> i64 {
    let ilp = build_mis_ilp(num_vertices, edges, weights);
    let solver = ILPSolver::new();
    solver
        .solve(&ilp)
        .expect("test weighted MIS solver must return a solution")
        .iter()
        .zip(weights.iter())
        .map(|(&x, &w)| if x > 0 { w } else { 0 })
        .sum()
}

/// Solve weighted MIS and return the binary configuration.
#[allow(dead_code)]
pub fn solve_weighted_mis_config(
    num_vertices: usize,
    edges: &[(usize, usize)],
    weights: &[i64],
) -> Vec<usize> {
    let ilp = build_mis_ilp(num_vertices, edges, weights);

    let solver = ILPSolver::new();
    solver
        .solve(&ilp)
        .expect("test weighted MIS solver must return a solution")
        .iter()
        .map(|&x| if x > 0 { 1 } else { 0 })
        .collect()
}

/// Generate edges for triangular lattice using proper triangular coordinates.
/// Triangular coordinates: (row, col) maps to physical position:
/// - x = row + 0.5 if col is even, else row
/// - y = col * sqrt(3)/2
pub fn triangular_edges(locs: &[(usize, usize)], radius: f64) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for (i, &(r1, c1)) in locs.iter().enumerate() {
        for (j, &(r2, c2)) in locs.iter().enumerate() {
            if i < j {
                // Convert to physical triangular coordinates
                let x1 = r1 as f64 + if c1.is_multiple_of(2) { 0.5 } else { 0.0 };
                let y1 = c1 as f64 * (3.0_f64.sqrt() / 2.0);
                let x2 = r2 as f64 + if c2.is_multiple_of(2) { 0.5 } else { 0.0 };
                let y2 = c2 as f64 * (3.0_f64.sqrt() / 2.0);

                let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
                if dist <= radius {
                    edges.push((i, j));
                }
            }
        }
    }
    edges
}

/// Generate edges for the King's-subgraph topology.
pub fn ksg_edges(locations: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for (left, &(left_row, left_column)) in locations.iter().enumerate() {
        for (right, &(right_row, right_column)) in locations.iter().enumerate().skip(left + 1) {
            if left_row.abs_diff(right_row) <= 1 && left_column.abs_diff(right_column) <= 1 {
                edges.push((left, right));
            }
        }
    }
    edges
}
