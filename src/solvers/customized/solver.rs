//! Exact customized solvers and their exact-variant registrations.

use super::fd_subset_search::{
    self, compute_closure, find_essential_attributes, find_essential_attributes_restricted,
    is_minimal_key, is_superkey, BranchDecision,
};
use crate::models::graph::{
    MinimumCostCirculation, MinimumIntersectionGraphBasis, PartialFeedbackEdgeSet,
    RootedTreeArrangement,
};
use crate::models::misc::{
    AdditionalKey, BoyceCoddNormalFormViolation, EnsembleComputation, GroupingBySwapping,
    MinimumDecisionTree, ShortestCommonSuperstring, TimetableDesign,
};
use crate::models::set::{MinimumCardinalityKey, PrimeAttributeName};
use crate::solvers::registry::CustomizedSolverRegistration;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use std::collections::HashSet;

macro_rules! register_customized_solver {
    ($problem:ty, $implementation:literal, $solve:expr) => {
        inventory::submit! {
            CustomizedSolverRegistration {
                source_name: <$problem as Problem>::NAME,
                source_variant_fn: <$problem as Problem>::variant,
                implementation: $implementation,
                solve_fn: |any| {
                    let problem = any.downcast_ref::<$problem>().expect(
                        "customized solver registration received the wrong concrete type",
                    );
                    $solve(problem).map(|solution| {
                        solution.map(|solution: <$problem as Problem>::Solution| {
                            serde_json::to_value(solution)
                                .expect("customized solution serialization must succeed")
                        })
                    })
                },
            }
        }
    };
}

register_customized_solver!(
    MinimumCardinalityKey,
    "fd-minimum-cardinality-key",
    |problem| Ok(solve_minimum_cardinality_key(problem))
);
register_customized_solver!(AdditionalKey, "fd-additional-key", |problem| Ok(
    solve_additional_key(problem)
));
register_customized_solver!(PrimeAttributeName, "fd-prime-attribute-name", |problem| Ok(
    solve_prime_attribute_name(problem)
));
register_customized_solver!(
    BoyceCoddNormalFormViolation,
    "fd-bcnf-violation",
    |problem| Ok(solve_bcnf_violation(problem))
);
register_customized_solver!(
    PartialFeedbackEdgeSet<SimpleGraph>,
    "partial-feedback-edge-set",
    |problem| Ok(super::partial_feedback_edge_set::solve(problem))
);
register_customized_solver!(
    RootedTreeArrangement<SimpleGraph>,
    "rooted-tree-arrangement",
    |problem| Ok(super::rooted_tree_arrangement::solve(problem))
);
register_customized_solver!(GroupingBySwapping, "symbol-block-order", |problem| Ok(
    super::grouping_by_swapping::solve(problem)
));
register_customized_solver!(ShortestCommonSuperstring, "subset-dp", |problem| Ok(
    super::shortest_common_superstring::solve(problem)
));
register_customized_solver!(MinimumDecisionTree, "subset-dp", |problem| Ok(
    super::minimum_decision_tree::solve(problem)
));
register_customized_solver!(EnsembleComputation, "breadth-first-search", |problem| Ok(
    super::ensemble_computation::solve(problem)
));
register_customized_solver!(
    MinimumCostCirculation,
    "negative-cycle-canceling",
    |problem| Ok(super::minimum_cost_circulation::solve(problem))
);
register_customized_solver!(
    MinimumIntersectionGraphBasis<SimpleGraph>,
    "maximal-clique-edge-cover",
    |problem| Ok(super::minimum_intersection_graph_basis::solve(problem))
);
register_customized_solver!(
    TimetableDesign,
    "timetable-required-assignments",
    |problem| Ok(TimetableDesign::solve_via_required_assignments(problem))
);

register_customized_solver!(
    crate::models::algebraic::ClosestVectorProblem<i64>,
    "cvp-sphere-enumeration",
    |problem| super::closest_vector_problem::solve(problem).map(Some)
);
register_customized_solver!(
    crate::models::algebraic::ClosestVectorProblem<f64>,
    "cvp-sphere-enumeration",
    |problem| super::closest_vector_problem::solve(problem).map(Some)
);

/// Solve MinimumCardinalityKey: find a minimal key with smallest cardinality.
///
/// Uses iterative deepening by cardinality to guarantee the first solution
/// found has the minimum number of attributes.
pub(crate) fn solve_minimum_cardinality_key(problem: &MinimumCardinalityKey) -> Option<Vec<bool>> {
    let n = problem.num_attributes();
    let deps = problem.dependencies().to_vec();

    let essential = find_essential_attributes(n, &deps);
    let essential_count = essential.len();

    // Build branch order: non-essential attributes
    let essential_set: HashSet<usize> = essential.iter().copied().collect();
    let branch_order: Vec<usize> = (0..n).filter(|i| !essential_set.contains(i)).collect();

    // Iterative deepening: try smallest cardinality first
    for max_total in essential_count..=n {
        let result = fd_subset_search::search_fd_subset(
            n,
            &essential,
            &branch_order,
            |selected, _depth| {
                let count = selected.iter().filter(|&&v| v).count();
                if count > max_total {
                    BranchDecision::Prune
                } else {
                    BranchDecision::Continue
                }
            },
            |selected| {
                selected.iter().filter(|&&v| v).count() == max_total
                    && is_minimal_key(selected, &deps)
            },
        );

        if let Some(indices) = result {
            let mut config = vec![false; n];
            for i in indices {
                config[i] = true;
            }
            return Some(config);
        }
    }
    None
}

/// Solve AdditionalKey: find a candidate key not in the known set.
pub(crate) fn solve_additional_key(problem: &AdditionalKey) -> Option<Vec<bool>> {
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
            .map(|&attr| index_set.contains(&attr))
            .collect()
    })
}

/// Solve PrimeAttributeName: find a candidate key containing the query attribute.
pub(crate) fn solve_prime_attribute_name(problem: &PrimeAttributeName) -> Option<Vec<bool>> {
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
        |selected| selected[query] && is_minimal_key(selected, &deps),
    );

    result.map(|indices| {
        let mut config = vec![false; n];
        for i in indices {
            config[i] = true;
        }
        config
    })
}

/// Solve BoyceCoddNormalFormViolation: find a subset X of target_subset such that
/// the closure of X contains some but not all of target_subset \ X.
pub(crate) fn solve_bcnf_violation(problem: &BoyceCoddNormalFormViolation) -> Option<Vec<bool>> {
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
            let x: HashSet<usize> = target.iter().copied().filter(|&a| selected[a]).collect();
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
            .map(|&attr| index_set.contains(&attr))
            .collect()
    })
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/solver.rs"]
mod tests;
