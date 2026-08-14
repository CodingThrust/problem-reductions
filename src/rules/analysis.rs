//! Topology analysis utilities for the reduction graph.

use crate::rules::graph::ReductionGraph;
use std::collections::{BTreeMap, BTreeSet};

// ────────── Topology checks ──────────

/// Result of checking graph connectivity at the problem-type level.
#[derive(Debug, Clone)]
pub struct ConnectivityReport {
    /// Total number of problem types in the graph.
    pub total_types: usize,
    /// Total number of registered reductions.
    pub total_reductions: usize,
    /// Problem types with no reductions in or out.
    pub isolated: Vec<IsolatedProblem>,
    /// Connected components (sorted largest first). Each component is a sorted
    /// list of problem type names.
    pub components: Vec<Vec<&'static str>>,
}

/// An isolated problem type with its variant count.
#[derive(Debug, Clone)]
pub struct IsolatedProblem {
    pub name: &'static str,
    pub num_variants: usize,
    /// Per-variant complexity strings (if available).
    pub variant_complexities: Vec<(BTreeMap<String, String>, Option<String>)>,
}

/// Check reduction graph connectivity: find isolated problems and connected components.
pub fn check_connectivity(graph: &ReductionGraph) -> ConnectivityReport {
    let mut types = graph.problem_types();
    types.sort();

    // Build undirected adjacency at the problem-type level
    let mut adj: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for &name in &types {
        adj.entry(name).or_default();
        for edge in graph.outgoing_reductions(name) {
            adj.entry(name).or_default().insert(edge.target_name);
            adj.entry(edge.target_name).or_default().insert(name);
        }
    }

    // Find connected components via BFS
    let mut visited: BTreeSet<&str> = BTreeSet::new();
    let mut components: Vec<Vec<&str>> = Vec::new();

    for &name in &types {
        if visited.contains(name) {
            continue;
        }
        let mut component = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(name);
        visited.insert(name);

        while let Some(current) = queue.pop_front() {
            component.push(current);
            if let Some(neighbors) = adj.get(current) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        component.sort();
        components.push(component);
    }

    components.sort_by_key(|c| std::cmp::Reverse(c.len()));

    let isolated: Vec<IsolatedProblem> = types
        .iter()
        .copied()
        .filter(|name| adj.get(name).is_some_and(|n| n.is_empty()))
        .map(|name| {
            let variants = graph.variants_for(name);
            let variant_complexities = variants
                .iter()
                .map(|v| {
                    let c = graph.variant_complexity(name, v).map(|e| e.to_string());
                    (v.clone(), c)
                })
                .collect();
            IsolatedProblem {
                name,
                num_variants: variants.len(),
                variant_complexities,
            }
        })
        .collect();

    ConnectivityReport {
        total_types: types.len(),
        total_reductions: graph.num_reductions(),
        isolated,
        components,
    }
}

/// Classification of a problem type that is unreachable from 3-SAT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnreachableReason {
    /// Known to be solvable in polynomial time.
    InP,
    /// Intermediate complexity (e.g., Factoring — believed neither in P nor NP-complete).
    Intermediate,
    /// No reductions at all (orphan).
    Orphan,
    /// NP-hard but missing a proof chain from 3-SAT.
    MissingProofChain,
}

/// A problem type not reachable from 3-SAT via directed reduction paths.
#[derive(Debug, Clone)]
pub struct UnreachableProblem {
    pub name: &'static str,
    pub reason: UnreachableReason,
    pub outgoing_count: usize,
    pub incoming_count: usize,
}

/// Result of checking NP-hardness proof chains from 3-SAT.
#[derive(Debug, Clone)]
pub struct ReachabilityReport {
    /// Total number of problem types.
    pub total_types: usize,
    /// Problem types reachable from 3-SAT, with minimum hop distance.
    pub reachable: BTreeMap<&'static str, usize>,
    /// Problem types not reachable, classified by reason.
    pub unreachable: Vec<UnreachableProblem>,
}

impl ReachabilityReport {
    /// Returns only the problems that are NP-hard but missing a proof chain.
    pub fn missing_proof_chains(&self) -> Vec<&UnreachableProblem> {
        self.unreachable
            .iter()
            .filter(|p| p.reason == UnreachableReason::MissingProofChain)
            .collect()
    }
}

/// Check which problems are reachable from 3-SAT (KSatisfiability) via directed
/// reduction paths. Problems without such a path are classified as P-time,
/// intermediate, orphan, or missing a proof chain.
pub fn check_reachability_from_3sat(graph: &ReductionGraph) -> ReachabilityReport {
    const SOURCE: &str = "KSatisfiability";

    let mut types = graph.problem_types();
    types.sort();

    // Build directed adjacency at the type level
    let mut adj: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for &name in &types {
        adj.entry(name).or_default();
        for edge in graph.outgoing_reductions(name) {
            adj.entry(name).or_default().insert(edge.target_name);
        }
    }

    // BFS from 3-SAT following directed edges
    let mut reachable: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut queue: std::collections::VecDeque<(&str, usize)> = std::collections::VecDeque::new();
    reachable.insert(SOURCE, 0);
    queue.push_back((SOURCE, 0));

    while let Some((current, hops)) = queue.pop_front() {
        if let Some(neighbors) = adj.get(current) {
            for &neighbor in neighbors {
                if !reachable.contains_key(neighbor) {
                    reachable.insert(neighbor, hops + 1);
                    queue.push_back((neighbor, hops + 1));
                }
            }
        }
    }

    // Known P-time problems and variants
    let p_time_checks: &[(&str, Option<(&str, &str)>)] = &[
        ("MaximumMatching", None),
        ("KSatisfiability", Some(("k", "K2"))),
        ("KColoring", Some(("graph", "SimpleGraph"))),
    ];

    let intermediate_names: &[&str] = &["Factoring"];

    let mut unreachable_problems: Vec<UnreachableProblem> = Vec::new();

    for &name in &types {
        if reachable.contains_key(name) {
            continue;
        }

        let out_count = graph.outgoing_reductions(name).len();
        let in_count = graph.incoming_reductions(name).len();

        // Orphan?
        if out_count == 0 && in_count == 0 {
            unreachable_problems.push(UnreachableProblem {
                name,
                reason: UnreachableReason::Orphan,
                outgoing_count: 0,
                incoming_count: 0,
            });
            continue;
        }

        // Known P-time?
        let is_p = p_time_checks.iter().any(|(pname, variant_check)| {
            if *pname != name {
                return false;
            }
            match variant_check {
                None => true,
                Some((key, val)) => {
                    let variants = graph.variants_for(name);
                    variants.len() == 1 && variants[0].get(*key).map(|s| s.as_str()) == Some(*val)
                }
            }
        });
        if is_p {
            unreachable_problems.push(UnreachableProblem {
                name,
                reason: UnreachableReason::InP,
                outgoing_count: out_count,
                incoming_count: in_count,
            });
            continue;
        }

        // Known intermediate?
        if intermediate_names.contains(&name) {
            unreachable_problems.push(UnreachableProblem {
                name,
                reason: UnreachableReason::Intermediate,
                outgoing_count: out_count,
                incoming_count: in_count,
            });
            continue;
        }

        // NP-hard but missing proof chain
        unreachable_problems.push(UnreachableProblem {
            name,
            reason: UnreachableReason::MissingProofChain,
            outgoing_count: out_count,
            incoming_count: in_count,
        });
    }

    ReachabilityReport {
        total_types: types.len(),
        reachable,
        unreachable: unreachable_problems,
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/analysis.rs"]
mod tests;
