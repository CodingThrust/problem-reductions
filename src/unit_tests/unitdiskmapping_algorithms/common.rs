//! Common test utilities for mapping tests.

use crate::models::optimization::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::MaximumIndependentSet;
use crate::rules::unitdiskmapping::MappingResult;
use crate::rules::{ReduceTo, ReductionResult};
use crate::solvers::ILPSolver;
use crate::topology::SimpleGraph;

/// Check if a configuration is a valid independent set.
pub fn is_independent_set(edges: &[(usize, usize)], config: &[usize]) -> bool {
    for &(u, v) in edges {
        if config.get(u).copied().unwrap_or(0) > 0 && config.get(v).copied().unwrap_or(0) > 0 {
            return false;
        }
    }
    true
}

/// Solve maximum independent set using ILP.
/// Returns the size of the MIS.
pub fn solve_mis(num_vertices: usize, edges: &[(usize, usize)]) -> usize {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(num_vertices, edges.to_vec());
    let reduction = <MaximumIndependentSet<SimpleGraph, i32> as ReduceTo<ILP>>::reduce_to(&problem);
    let solver = ILPSolver::new();
    if let Some(solution) = solver.solve(reduction.target_problem()) {
        solution.iter().filter(|&&x| x > 0).count()
    } else {
        0
    }
}

/// Solve MIS and return the binary configuration.
pub fn solve_mis_config(num_vertices: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(num_vertices, edges.to_vec());
    let reduction = <MaximumIndependentSet<SimpleGraph, i32> as ReduceTo<ILP>>::reduce_to(&problem);
    let solver = ILPSolver::new();
    if let Some(solution) = solver.solve(reduction.target_problem()) {
        solution
            .iter()
            .map(|&x| if x > 0 { 1 } else { 0 })
            .collect()
    } else {
        vec![0; num_vertices]
    }
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

    let weights: Vec<i32> = (0..num_vertices)
        .map(|i| result.node_weights.get(i).copied().unwrap_or(1))
        .collect();

    solve_weighted_mis(num_vertices, &edges, &weights) as usize
}

/// Solve weighted MIS on a graph using ILP.
/// Returns the maximum weighted independent set value.
pub fn solve_weighted_mis(num_vertices: usize, edges: &[(usize, usize)], weights: &[i32]) -> i32 {
    let constraints: Vec<LinearConstraint> = edges
        .iter()
        .map(|&(i, j)| LinearConstraint::le(vec![(i, 1.0), (j, 1.0)], 1.0))
        .collect();

    let objective: Vec<(usize, f64)> = weights
        .iter()
        .enumerate()
        .map(|(i, &w)| (i, w as f64))
        .collect();

    let ilp = ILP::binary(
        num_vertices,
        constraints,
        objective,
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    if let Some(solution) = solver.solve(&ilp) {
        solution
            .iter()
            .zip(weights.iter())
            .map(|(&x, &w)| if x > 0 { w } else { 0 })
            .sum()
    } else {
        0
    }
}

/// Solve weighted MIS and return the binary configuration.
#[allow(dead_code)]
pub fn solve_weighted_mis_config(
    num_vertices: usize,
    edges: &[(usize, usize)],
    weights: &[i32],
) -> Vec<usize> {
    let constraints: Vec<LinearConstraint> = edges
        .iter()
        .map(|&(i, j)| LinearConstraint::le(vec![(i, 1.0), (j, 1.0)], 1.0))
        .collect();

    let objective: Vec<(usize, f64)> = weights
        .iter()
        .enumerate()
        .map(|(i, &w)| (i, w as f64))
        .collect();

    let ilp = ILP::binary(
        num_vertices,
        constraints,
        objective,
        ObjectiveSense::Maximize,
    );

    let solver = ILPSolver::new();
    if let Some(solution) = solver.solve(&ilp) {
        solution
            .iter()
            .map(|&x| if x > 0 { 1 } else { 0 })
            .collect()
    } else {
        vec![0; num_vertices]
    }
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
