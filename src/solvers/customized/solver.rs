//! CustomizedSolver: structure-exploiting exact witness solver.
//!
//! Uses direct downcast dispatch to call dedicated backends for
//! supported problem types, returning `None` for unsupported problems.

use super::fd_subset_search::{
    self, compute_closure, find_essential_attributes, find_essential_attributes_restricted,
    is_minimal_key, is_superkey, BranchDecision,
};
use crate::models::misc::{AdditionalKey, BoyceCoddNormalFormViolation};
use crate::models::set::{MinimumCardinalityKey, PrimeAttributeName};
use std::collections::HashSet;

/// A solver that uses problem-specific backends for exact witness recovery.
///
/// Unlike `BruteForce`, which enumerates all configurations, `CustomizedSolver`
/// exploits problem structure (functional-dependency closure, cycle hitting,
/// tree arrangement) to prune search and find witnesses more efficiently.
///
/// Returns `None` for unsupported problem types.
pub struct CustomizedSolver;

impl CustomizedSolver {
    /// Create a new `CustomizedSolver`.
    pub fn new() -> Self {
        Self
    }

    /// Check whether a type-erased problem is supported by the customized solver.
    pub fn supports_problem(any: &dyn std::any::Any) -> bool {
        any.is::<MinimumCardinalityKey>()
            || any.is::<AdditionalKey>()
            || any.is::<PrimeAttributeName>()
            || any.is::<BoyceCoddNormalFormViolation>()
    }

    /// Attempt to solve a type-erased problem using a dedicated backend.
    ///
    /// Returns `Some(config)` if a satisfying witness is found, `None` if
    /// the problem type is unsupported or no witness exists.
    pub fn solve_dyn(&self, any: &dyn std::any::Any) -> Option<Vec<usize>> {
        if let Some(p) = any.downcast_ref::<MinimumCardinalityKey>() {
            return solve_minimum_cardinality_key(p);
        }
        if let Some(p) = any.downcast_ref::<AdditionalKey>() {
            return solve_additional_key(p);
        }
        if let Some(p) = any.downcast_ref::<PrimeAttributeName>() {
            return solve_prime_attribute_name(p);
        }
        if let Some(p) = any.downcast_ref::<BoyceCoddNormalFormViolation>() {
            return solve_bcnf_violation(p);
        }
        None
    }
}

/// Solve MinimumCardinalityKey: find a minimal key with cardinality <= bound.
fn solve_minimum_cardinality_key(problem: &MinimumCardinalityKey) -> Option<Vec<usize>> {
    let n = problem.num_attributes();
    let deps = problem.dependencies().to_vec();
    let bound = problem.bound() as usize;

    let essential = find_essential_attributes(n, &deps);

    // If essential attributes alone exceed the bound, no solution
    if essential.len() > bound {
        return None;
    }

    // Build branch order: non-essential attributes
    let essential_set: HashSet<usize> = essential.iter().copied().collect();
    let branch_order: Vec<usize> = (0..n).filter(|i| !essential_set.contains(i)).collect();

    let result = fd_subset_search::search_fd_subset(
        n,
        &essential,
        &branch_order,
        |selected, _depth| {
            let count = selected.iter().filter(|&&v| v).count();
            if count > bound {
                return BranchDecision::Prune;
            }
            BranchDecision::Continue
        },
        |selected| {
            let count = selected.iter().filter(|&&v| v).count();
            count <= bound && is_minimal_key(selected, &deps)
        },
    );

    // Convert selected indices to config format (binary vector)
    result.map(|indices| {
        let mut config = vec![0; n];
        for i in indices {
            config[i] = 1;
        }
        config
    })
}

/// Solve AdditionalKey: find a candidate key not in the known set.
fn solve_additional_key(problem: &AdditionalKey) -> Option<Vec<usize>> {
    let n_attrs = problem.num_attributes();
    let deps = problem.dependencies().to_vec();
    let relation_attrs = problem.relation_attrs();
    let known_keys = problem.known_keys();

    let essential = find_essential_attributes_restricted(n_attrs, &deps, relation_attrs);

    // Build branch order over relation_attrs indices (excluding essential)
    let essential_set: HashSet<usize> = essential.iter().copied().collect();
    let branch_indices: Vec<usize> = relation_attrs
        .iter()
        .copied()
        .filter(|a| !essential_set.contains(a))
        .collect();

    // We search over a boolean vector of size n_attrs
    let result = fd_subset_search::search_fd_subset(
        n_attrs,
        &essential,
        &branch_indices,
        |_selected, _depth| BranchDecision::Continue,
        |selected| {
            // Check that selected forms a superkey over relation_attrs
            let closure = compute_closure(selected, &deps);
            if !relation_attrs.iter().all(|&a| closure[a]) {
                return false;
            }
            // Check minimality: removing any selected relation_attr breaks coverage
            let selected_ra: Vec<usize> = relation_attrs
                .iter()
                .copied()
                .filter(|&a| selected[a])
                .collect();
            if selected_ra.is_empty() {
                return false;
            }
            for &a in &selected_ra {
                let mut reduced = selected.to_vec();
                reduced[a] = false;
                let reduced_closure = compute_closure(&reduced, &deps);
                if relation_attrs.iter().all(|&ra| reduced_closure[ra]) {
                    return false; // Not minimal
                }
            }
            // Check it's not in known_keys
            let mut sorted_selected: Vec<usize> = selected_ra;
            sorted_selected.sort_unstable();
            !known_keys.contains(&sorted_selected)
        },
    );

    // Convert to config format (binary vector over relation_attrs positions)
    result.map(|indices| {
        let index_set: HashSet<usize> = indices.into_iter().collect();
        relation_attrs
            .iter()
            .enumerate()
            .map(|(_, &attr)| if index_set.contains(&attr) { 1 } else { 0 })
            .collect()
    })
}

/// Solve PrimeAttributeName: find a candidate key containing the query attribute.
fn solve_prime_attribute_name(problem: &PrimeAttributeName) -> Option<Vec<usize>> {
    let n = problem.num_attributes();
    let deps = problem.dependencies().to_vec();
    let query = problem.query_attribute();

    let essential = find_essential_attributes(n, &deps);

    // Query attribute must be forcibly included
    let mut forced: Vec<usize> = essential.clone();
    if !forced.contains(&query) {
        forced.push(query);
    }
    forced.sort_unstable();
    forced.dedup();

    let forced_set: HashSet<usize> = forced.iter().copied().collect();
    let branch_order: Vec<usize> = (0..n).filter(|i| !forced_set.contains(i)).collect();

    let result = fd_subset_search::search_fd_subset(
        n,
        &forced,
        &branch_order,
        |selected, _depth| {
            // Early superkey check: if already a superkey, try to check completeness
            if is_superkey(selected, &deps) {
                // Even if already superkey, we want to continue to minimality check
                return BranchDecision::Continue;
            }
            BranchDecision::Continue
        },
        |selected| {
            selected[query] && is_minimal_key(selected, &deps)
        },
    );

    result.map(|indices| {
        let mut config = vec![0; n];
        for i in indices {
            config[i] = 1;
        }
        config
    })
}

/// Solve BoyceCoddNormalFormViolation: find a subset X of target_subset such that
/// the closure of X contains some but not all of target_subset \ X.
fn solve_bcnf_violation(problem: &BoyceCoddNormalFormViolation) -> Option<Vec<usize>> {
    let n_attrs = problem.num_attributes();
    let deps = problem.functional_deps().to_vec();
    let target = problem.target_subset();

    // Branch over target_subset attributes
    let branch_order: Vec<usize> = target.to_vec();

    let result = fd_subset_search::search_fd_subset(
        n_attrs,
        &[],
        &branch_order,
        |_selected, _depth| BranchDecision::Continue,
        |selected| {
            let x: HashSet<usize> = target
                .iter()
                .copied()
                .filter(|&a| selected[a])
                .collect();
            let closure = compute_closure(selected, &deps);
            // Check: ∃ y, z ∈ target \ X s.t. y ∈ closure ∧ z ∉ closure
            let mut has_in_closure = false;
            let mut has_not_in_closure = false;
            for &a in target {
                if !x.contains(&a) {
                    if closure[a] {
                        has_in_closure = true;
                    } else {
                        has_not_in_closure = true;
                    }
                }
            }
            has_in_closure && has_not_in_closure
        },
    );

    // Convert: binary vector over target_subset positions
    result.map(|indices| {
        let index_set: HashSet<usize> = indices.into_iter().collect();
        target
            .iter()
            .map(|&attr| if index_set.contains(&attr) { 1 } else { 0 })
            .collect()
    })
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/solver.rs"]
mod tests;
