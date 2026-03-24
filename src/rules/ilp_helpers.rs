//! Shared ILP linearization helpers for Tier 3 reductions.
//!
//! These functions generate `LinearConstraint` sets for common ILP patterns:
//! McCormick products, MTZ orderings, flow conservation, big-M activation,
//! absolute-value differentials, minimax bounds, and one-hot decoding.

#![allow(dead_code)]
use crate::models::algebraic::LinearConstraint;

/// McCormick linearization: `y = x_a * x_b` (both binary).
///
/// Returns 3 constraints: `y ≤ x_a`, `y ≤ x_b`, `y ≥ x_a + x_b - 1`.
pub fn mccormick_product(y_idx: usize, x_a: usize, x_b: usize) -> Vec<LinearConstraint> {
    vec![
        // y <= x_a
        LinearConstraint::le(vec![(y_idx, 1.0), (x_a, -1.0)], 0.0),
        // y <= x_b
        LinearConstraint::le(vec![(y_idx, 1.0), (x_b, -1.0)], 0.0),
        // y >= x_a + x_b - 1  =>  x_a + x_b - y <= 1
        LinearConstraint::le(vec![(x_a, 1.0), (x_b, 1.0), (y_idx, -1.0)], 1.0),
    ]
}

/// MTZ topological ordering for directed arcs.
///
/// For each arc `(u → v)`: `o_v - o_u ≥ 1 - M*(1 - x_u) - M*(1 - x_v)`
/// when both endpoints are kept (x=0 means kept, x=1 means removed).
/// Also emits bound constraints: `0 ≤ o_i ≤ n-1`.
///
/// `x_offset`: start index for removal indicator variables.
/// `o_offset`: start index for ordering variables.
pub fn mtz_ordering(
    arcs: &[(usize, usize)],
    n: usize,
    x_offset: usize,
    o_offset: usize,
) -> Vec<LinearConstraint> {
    let big_m = n as f64;
    let mut constraints = Vec::new();

    for &(u, v) in arcs {
        // o_v - o_u + M*x_u + M*x_v >= 1
        constraints.push(LinearConstraint::ge(
            vec![
                (o_offset + v, 1.0),
                (o_offset + u, -1.0),
                (x_offset + u, big_m),
                (x_offset + v, big_m),
            ],
            1.0,
        ));
    }

    // Bound constraints: 0 <= o_i <= n-1
    for i in 0..n {
        constraints.push(LinearConstraint::le(
            vec![(o_offset + i, 1.0)],
            (n - 1) as f64,
        ));
        constraints.push(LinearConstraint::ge(vec![(o_offset + i, 1.0)], 0.0));
    }

    constraints
}

/// Flow conservation at each node.
///
/// For each node `u`: `Σ_{(u,v)} f_{uv} - Σ_{(v,u)} f_{vu} = demand[u]`.
///
/// `flow_idx` maps an arc index to the ILP variable index for that arc's flow.
pub fn flow_conservation(
    arcs: &[(usize, usize)],
    num_nodes: usize,
    flow_idx: &dyn Fn(usize) -> usize,
    demand: &[f64],
) -> Vec<LinearConstraint> {
    let mut constraints = Vec::with_capacity(num_nodes);
    for (node, &rhs) in demand.iter().enumerate().take(num_nodes) {
        let mut terms = Vec::new();
        for (arc_idx, &(u, v)) in arcs.iter().enumerate() {
            if u == node {
                terms.push((flow_idx(arc_idx), 1.0)); // outgoing
            }
            if v == node {
                terms.push((flow_idx(arc_idx), -1.0)); // incoming
            }
        }
        constraints.push(LinearConstraint::eq(terms, rhs));
    }
    constraints
}

/// Big-M activation: `f ≤ M * y`. Single constraint.
pub fn big_m_activation(f_idx: usize, y_idx: usize, big_m: f64) -> LinearConstraint {
    // f - M*y <= 0
    LinearConstraint::le(vec![(f_idx, 1.0), (y_idx, -big_m)], 0.0)
}

/// Absolute value linearization: `|a - b| ≤ z`.
///
/// Returns 2 constraints: `a - b ≤ z`, `b - a ≤ z`.
pub fn abs_diff_le(a_idx: usize, b_idx: usize, z_idx: usize) -> Vec<LinearConstraint> {
    vec![
        // a - b - z <= 0
        LinearConstraint::le(vec![(a_idx, 1.0), (b_idx, -1.0), (z_idx, -1.0)], 0.0),
        // b - a - z <= 0
        LinearConstraint::le(vec![(b_idx, 1.0), (a_idx, -1.0), (z_idx, -1.0)], 0.0),
    ]
}

/// Minimax: `z ≥ expr_i` for each expression.
///
/// Each `expr` is a list of `(var_idx, coeff)` terms representing a linear expression.
pub fn minimax_constraints(
    z_idx: usize,
    expr_terms: &[Vec<(usize, f64)>],
) -> Vec<LinearConstraint> {
    expr_terms
        .iter()
        .map(|terms| {
            // z >= Σ coeff_j * x_j  =>  z - Σ coeff_j * x_j >= 0
            let mut constraint_terms = vec![(z_idx, 1.0)];
            for &(var, coeff) in terms {
                constraint_terms.push((var, -coeff));
            }
            LinearConstraint::ge(constraint_terms, 0.0)
        })
        .collect()
}

/// One-hot to index extraction.
///
/// Given `num_items * num_slots` binary assignment variables starting at `var_offset`,
/// decode each slot `p` → value `v` where `x_{v*num_slots + p} = 1`.
///
/// Layout: variable at index `var_offset + v * num_slots + p` represents
/// "item v is assigned to slot p".
pub fn one_hot_decode(
    solution: &[usize],
    num_items: usize,
    num_slots: usize,
    var_offset: usize,
) -> Vec<usize> {
    (0..num_slots)
        .map(|p| {
            (0..num_items)
                .find(|&v| solution[var_offset + v * num_slots + p] == 1)
                .unwrap_or(0)
        })
        .collect()
}

/// Convert a permutation to Lehmer code.
///
/// Given a permutation of `[0..n)`, returns the Lehmer code representation
/// where each element counts the number of smaller elements to its right.
pub fn permutation_to_lehmer(perm: &[usize]) -> Vec<usize> {
    let n = perm.len();
    let mut lehmer = Vec::with_capacity(n);
    for i in 0..n {
        let count = (i + 1..n).filter(|&j| perm[j] < perm[i]).count();
        lehmer.push(count);
    }
    lehmer
}

/// One-hot assignment constraints: each item assigned to exactly one slot,
/// each slot assigned at most one item.
///
/// Returns constraints for a `num_items × num_slots` assignment matrix
/// starting at `var_offset`.
pub fn one_hot_assignment_constraints(
    num_items: usize,
    num_slots: usize,
    var_offset: usize,
) -> Vec<LinearConstraint> {
    let mut constraints = Vec::new();

    // Each item assigned to exactly one slot
    for v in 0..num_items {
        let terms: Vec<(usize, f64)> = (0..num_slots)
            .map(|p| (var_offset + v * num_slots + p, 1.0))
            .collect();
        constraints.push(LinearConstraint::eq(terms, 1.0));
    }

    // Each slot assigned at most one item
    for p in 0..num_slots {
        let terms: Vec<(usize, f64)> = (0..num_items)
            .map(|v| (var_offset + v * num_slots + p, 1.0))
            .collect();
        constraints.push(LinearConstraint::le(terms, 1.0));
    }

    constraints
}

#[cfg(test)]
#[path = "../unit_tests/rules/ilp_helpers.rs"]
mod tests;
