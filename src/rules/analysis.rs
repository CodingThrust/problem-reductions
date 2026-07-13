//! Analysis utilities for the reduction graph.
//!
//! Detects primitive reduction rules that are dominated by composite paths,
//! comparing overhead expressions through the shared symbolic growth domain
//! ([`crate::growth::Growth`]).
//!
//! This analysis is **sound but incomplete**: it reports `Dominated` only when
//! the growth comparison is trustworthy, and `Unknown` when a field's growth is
//! [`Growth::Unknown`] (nonlinear exponent, factorial, …).

use crate::expr::Expr;
use crate::growth::Growth;
use crate::rules::graph::{ReductionGraph, ReductionPath};
use crate::rules::registry::ReductionOverhead;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Result of comparing one primitive rule against one composite path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonStatus {
    /// Composite is equal or better on all common fields.
    Dominated,
    /// Composite is worse on at least one common field.
    NotDominated,
    /// Cannot decide: expression not normalizable or path not trustworthy.
    Unknown,
}

/// A primitive reduction rule proven dominated by a composite path.
#[derive(Debug, Clone)]
pub struct DominatedRule {
    pub source_name: &'static str,
    pub source_variant: BTreeMap<String, String>,
    pub target_name: &'static str,
    pub target_variant: BTreeMap<String, String>,
    pub primitive_overhead: ReductionOverhead,
    pub dominating_path: ReductionPath,
    pub composed_overhead: ReductionOverhead,
    pub comparable_fields: Vec<String>,
}

impl DominatedRule {
    pub fn source_display(&self) -> String {
        format_problem_variant(self.source_name, &self.source_variant)
    }

    pub fn target_display(&self) -> String {
        format_problem_variant(self.target_name, &self.target_variant)
    }
}

impl fmt::Display for DominatedRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source_display(), self.target_display())
    }
}

/// A candidate comparison that could not be decided soundly.
#[derive(Debug, Clone)]
pub struct UnknownComparison {
    pub source_name: &'static str,
    pub source_variant: BTreeMap<String, String>,
    pub target_name: &'static str,
    pub target_variant: BTreeMap<String, String>,
    pub candidate_path: ReductionPath,
    pub reason: String,
}

impl UnknownComparison {
    pub fn source_display(&self) -> String {
        format_problem_variant(self.source_name, &self.source_variant)
    }

    pub fn target_display(&self) -> String {
        format_problem_variant(self.target_name, &self.target_variant)
    }
}

impl fmt::Display for UnknownComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.source_display(), self.target_display())
    }
}

pub fn format_problem_variant(name: &str, variant: &BTreeMap<String, String>) -> String {
    if variant.is_empty() {
        return name.to_string();
    }

    let vars = variant
        .iter()
        .map(|(k, v)| format!("{k}: {v:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{name} {{{vars}}}")
}

// ────────── Overhead comparison ──────────

/// Compare two overheads across all common fields, using the shared symbolic
/// growth domain ([`Growth`]) as the single dominance order.
///
/// Fields present in only one overhead are skipped (common-field semantics).
/// For each common field with primitive growth `pg` and composite growth `cg`:
/// - if either is [`Growth::Unknown`] the whole comparison is `Unknown`;
/// - otherwise the field is fine iff the composite is dominated-or-equal by the
///   primitive (`pg` grows ≥ `cg`, i.e. `pg.dominates(&cg)` — reflexive, so an
///   equal field counts as fine);
/// - otherwise (composite strictly worse, or the two growths incomparable) the
///   comparison is `NotDominated`.
///
/// Returns `Dominated` when every common field is fine and at least one common
/// field exists; `NotDominated` when there is no common field.
pub fn compare_overhead(
    primitive: &ReductionOverhead,
    composite: &ReductionOverhead,
) -> ComparisonStatus {
    let comp_map: std::collections::HashMap<&str, &Expr> = composite
        .output_size
        .iter()
        .map(|(name, expr)| (*name, expr))
        .collect();

    let mut any_common = false;

    for (field, prim_expr) in &primitive.output_size {
        let Some(comp_expr) = comp_map.get(field) else {
            continue;
        };
        any_common = true;

        let pg = Growth::from_expr(prim_expr);
        let cg = Growth::from_expr(comp_expr);

        // A field whose growth we cannot bound symbolically makes the whole
        // comparison undecidable.
        if matches!(pg, Growth::Unknown) || matches!(cg, Growth::Unknown) {
            return ComparisonStatus::Unknown;
        }

        // `pg.dominates(&cg)` means the primitive grows at least as fast as the
        // composite on this field (composite ≤ primitive). `dominates` is
        // reflexive, so asymptotically-equal fields pass here. Anything else —
        // composite strictly worse, or the two growths incomparable — fails.
        if !pg.dominates(&cg) {
            return ComparisonStatus::NotDominated;
        }
    }

    if any_common {
        ComparisonStatus::Dominated
    } else {
        ComparisonStatus::NotDominated
    }
}

// ────────── Main analysis ──────────

/// Find all primitive reduction rules dominated by composite paths.
///
/// Returns a tuple of:
/// - `Vec<DominatedRule>`: rules proven dominated by a composite path
/// - `Vec<UnknownComparison>`: candidates that could not be decided
///
/// For each primitive rule (direct edge), enumerates all alternative paths,
/// validates trustworthiness, composes overheads, and compares.
/// Keeps only the best (shortest) dominating path per primitive rule.
///
/// Note: iterates the graph's coalesced edges rather than raw `inventory` entries.
/// This is sound because `test_no_duplicate_primitive_rules_per_variant_pair` guards
/// the invariant that at most one registration exists per (source_variant, target_variant) pair.
pub fn find_dominated_rules(
    graph: &ReductionGraph,
) -> (Vec<DominatedRule>, Vec<UnknownComparison>) {
    const MAX_PATHS_PER_EDGE: usize = 1024;
    const MAX_INTERMEDIATE_NODES: usize = 6;

    let mut dominated = Vec::new();
    let mut unknown = Vec::new();

    for edge_info in all_edges(graph) {
        let paths = graph.find_paths_up_to_mode_bounded(
            edge_info.source_name,
            &edge_info.source_variant,
            edge_info.target_name,
            &edge_info.target_variant,
            crate::rules::graph::ReductionMode::Witness,
            MAX_PATHS_PER_EDGE,
            Some(MAX_INTERMEDIATE_NODES),
        );

        let mut best_dominating: Option<(ReductionPath, ReductionOverhead, Vec<String>)> = None;

        for path in paths {
            if path.len() <= 1 {
                continue; // skip the direct edge itself
            }

            let composed = graph.compose_path_overhead(&path);

            match compare_overhead(&edge_info.overhead, &composed) {
                ComparisonStatus::Dominated => {
                    let comparable_fields = common_fields(&edge_info.overhead, &composed);
                    let is_better = match &best_dominating {
                        None => true,
                        Some((best_path, _, _)) => path.len() < best_path.len(),
                    };
                    if is_better {
                        best_dominating = Some((path, composed, comparable_fields));
                    }
                }
                ComparisonStatus::Unknown => {
                    unknown.push(UnknownComparison {
                        source_name: edge_info.source_name,
                        source_variant: edge_info.source_variant.clone(),
                        target_name: edge_info.target_name,
                        target_variant: edge_info.target_variant.clone(),
                        candidate_path: path,
                        reason: "expression comparison returned Unknown".into(),
                    });
                }
                ComparisonStatus::NotDominated => {}
            }
        }

        if let Some((path, composed, fields)) = best_dominating {
            dominated.push(DominatedRule {
                source_name: edge_info.source_name,
                source_variant: edge_info.source_variant.clone(),
                target_name: edge_info.target_name,
                target_variant: edge_info.target_variant.clone(),
                primitive_overhead: edge_info.overhead.clone(),
                dominating_path: path,
                composed_overhead: composed,
                comparable_fields: fields,
            });
        }
    }

    // Deterministic output
    dominated.sort_by(|a, b| {
        (
            format_problem_variant(a.source_name, &a.source_variant),
            format_problem_variant(a.target_name, &a.target_variant),
            a.dominating_path.len(),
        )
            .cmp(&(
                format_problem_variant(b.source_name, &b.source_variant),
                format_problem_variant(b.target_name, &b.target_variant),
                b.dominating_path.len(),
            ))
    });
    unknown.sort_by(|a, b| {
        (
            format_problem_variant(a.source_name, &a.source_variant),
            format_problem_variant(a.target_name, &a.target_variant),
        )
            .cmp(&(
                format_problem_variant(b.source_name, &b.source_variant),
                format_problem_variant(b.target_name, &b.target_variant),
            ))
    });

    (dominated, unknown)
}

/// Fields present in both overheads.
fn common_fields(a: &ReductionOverhead, b: &ReductionOverhead) -> Vec<String> {
    let b_fields: std::collections::HashSet<&str> = b.output_size.iter().map(|(n, _)| *n).collect();
    a.output_size
        .iter()
        .filter(|&(f, _)| b_fields.contains(f))
        .map(|(f, _)| f.to_string())
        .collect()
}

/// Collect all edges from the reduction graph.
fn all_edges(graph: &ReductionGraph) -> Vec<crate::rules::graph::ReductionEdgeInfo> {
    let mut edges = Vec::new();
    for name in graph.problem_types() {
        edges.extend(graph.outgoing_reductions(name));
    }
    edges
}

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
