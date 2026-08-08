//! Tests for the multi-label elementary-path search (`src/rules/pareto.rs`) and its two label
//! domains. Covers:
//! - The measured concrete-instance search's known-answer and budget semantics.
//! - The generic kernel's correctness on a hand-built diamond (negative control): a
//!   scalar-cost path selection commits to the wrong prefix, while the Pareto search
//!   returns the path with the strictly-better final measured size.

use super::*;
use crate::expr::Expr;
use crate::growth::Growth;
use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::graph::HamiltonianCircuit;
use crate::rules::pareto::{GrowthLabel, PathLabel, ReductionEdge};
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::DynReductionResult;
use crate::rules::{ReductionAutoCast, ReductionGraph, ReductionMode, SizeBudget};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Or, ProblemSize};
use std::any::Any;
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Clone)]
struct MeasuredSource;

#[derive(Clone)]
struct MeasuredBranchA;

#[derive(Clone)]
struct MeasuredBranchB;

macro_rules! impl_measured_test_problem {
    ($ty:ty, $name:literal) => {
        impl Problem for $ty {
            const NAME: &'static str = $name;
            type Value = Or;

            fn dims(&self) -> Vec<usize> {
                vec![]
            }

            fn evaluate(&self, _config: &[usize]) -> Or {
                Or(true)
            }

            fn variant() -> Vec<(&'static str, &'static str)> {
                vec![]
            }
        }
    };
}

impl_measured_test_problem!(MeasuredSource, "MeasuredSource");
impl_measured_test_problem!(MeasuredBranchA, "MeasuredBranchA");
impl_measured_test_problem!(MeasuredBranchB, "MeasuredBranchB");

fn measured_source_to_a(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredSource>()
        .expect("expected MeasuredSource");
    Box::new(ReductionAutoCast::<MeasuredSource, MeasuredBranchA>::new(
        MeasuredBranchA,
    ))
}

fn measured_source_to_b(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredSource>()
        .expect("expected MeasuredSource");
    Box::new(ReductionAutoCast::<MeasuredSource, MeasuredBranchB>::new(
        MeasuredBranchB,
    ))
}

fn measured_a_to_sat(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredBranchA>()
        .expect("expected MeasuredBranchA");
    Box::new(ReductionAutoCast::<MeasuredBranchA, Satisfiability>::new(
        Satisfiability::new(1, vec![CNFClause::new(vec![1])]),
    ))
}

fn measured_b_to_sat(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredBranchB>()
        .expect("expected MeasuredBranchB");
    Box::new(ReductionAutoCast::<MeasuredBranchB, Satisfiability>::new(
        Satisfiability::new(1, vec![CNFClause::new(vec![-1])]),
    ))
}

fn measured_sat_to_structure_dependent_ilp(any: &dyn Any) -> Box<dyn DynReductionResult> {
    let sat = any
        .downcast_ref::<Satisfiability>()
        .expect("expected Satisfiability");
    let first_literal = sat.clauses()[0].literals[0];
    let num_vars = if first_literal > 0 { 100 } else { 1 };
    let target = ILP::<bool>::new(num_vars, vec![], vec![], ObjectiveSense::Minimize);
    Box::new(ReductionAutoCast::<Satisfiability, ILP<bool>>::new(target))
}

fn measured_source_to_small_ilp(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredSource>()
        .expect("expected MeasuredSource");
    let target = ILP::<bool>::new(1, vec![], vec![], ObjectiveSense::Minimize);
    Box::new(ReductionAutoCast::<MeasuredSource, ILP<bool>>::new(target))
}

fn measured_a_to_incomparable_ilp(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredBranchA>()
        .expect("expected branch A");
    let constraints = (0..10).map(|_| LinearConstraint::eq(vec![], 0.0)).collect();
    Box::new(ReductionAutoCast::<MeasuredBranchA, ILP<bool>>::new(
        ILP::new(1, constraints, vec![], ObjectiveSense::Minimize),
    ))
}

fn measured_b_to_incomparable_ilp(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredBranchB>()
        .expect("expected branch B");
    Box::new(ReductionAutoCast::<MeasuredBranchB, ILP<bool>>::new(
        ILP::new(
            10,
            vec![LinearConstraint::eq(vec![], 0.0)],
            vec![],
            ObjectiveSense::Minimize,
        ),
    ))
}

fn measured_a_to_equal_ilp(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredBranchA>()
        .expect("expected branch A");
    Box::new(ReductionAutoCast::<MeasuredBranchA, ILP<bool>>::new(
        ILP::new(2, vec![], vec![], ObjectiveSense::Minimize),
    ))
}

fn measured_b_to_equal_ilp(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredBranchB>()
        .expect("expected branch B");
    Box::new(ReductionAutoCast::<MeasuredBranchB, ILP<bool>>::new(
        ILP::new(2, vec![], vec![], ObjectiveSense::Minimize),
    ))
}

thread_local! {
    static CONSTRUCTIONS: Cell<usize> = const { Cell::new(0) };
}

fn counted_large_ilp(any: &dyn Any) -> Box<dyn DynReductionResult> {
    any.downcast_ref::<MeasuredSource>()
        .expect("expected source");
    CONSTRUCTIONS.with(|count| count.set(count.get() + 1));
    Box::new(ReductionAutoCast::<MeasuredSource, ILP<bool>>::new(
        ILP::new(2, vec![], vec![], ObjectiveSense::Minimize),
    ))
}

fn measured_edge(
    reduce_fn: fn(&dyn Any) -> Box<dyn DynReductionResult>,
    asymptotic_prediction: f64,
) -> ReductionEdgeData {
    ReductionEdgeData {
        overhead: ReductionOverhead::new(vec![(
            "predicted_total",
            Expr::Const(asymptotic_prediction),
        )]),
        reduce_fn: Some(reduce_fn),
        reduce_aggregate_fn: None,
        turing: false,
    }
}

// ---------------------------------------------------------------------------
// Verification 1: measured known-answer check.
// ---------------------------------------------------------------------------

/// A triangular-prism graph with 6 vertices and 9 edges.
fn prism_hamiltonian_circuit() -> HamiltonianCircuit<SimpleGraph> {
    let prism = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (3, 4),
            (4, 5),
            (5, 3),
            (0, 3),
            (1, 4),
            (2, 5),
        ],
    );
    HamiltonianCircuit::new(prism)
}

/// The measured Pareto search includes the route's concrete final ILP vector.
///
/// A previously documented chain through HamiltonianPath and
/// ConsecutiveOnesSubmatrix no longer exists on the current reduction graph.
/// This test pins the LongestCircuit route's component values without collapsing them
/// into a scalar.
#[test]
fn test_hamiltoniancircuit_to_ilp_measured_vector() {
    let hc = prism_hamiltonian_circuit();
    let graph = ReductionGraph::new();
    let variant = ReductionGraph::variant_to_map(&[("graph", "SimpleGraph")]);

    let measured = graph
        .measured_front_to_name(
            "HamiltonianCircuit",
            &variant,
            "ILP",
            ReductionMode::Witness,
            &hc as &dyn Any,
            SizeBudget::new(BTreeMap::from([
                ("num_vars".to_string(), 1_000),
                ("num_constraints".to_string(), 1_000),
            ])),
            crate::rules::SearchMode::Exact,
        )
        .expect("valid budget")
        .value
        .into_iter()
        .find(|path| path.path.type_names() == ["HamiltonianCircuit", "LongestCircuit", "ILP"])
        .expect("measured front contains LongestCircuit route");

    assert_eq!(measured.size.get("num_vars"), Some(105));
    assert_eq!(measured.size.get("num_constraints"), Some(127));
    // Via LongestCircuit, to the bool ILP variant.
    assert_eq!(
        measured.path.type_names(),
        vec!["HamiltonianCircuit", "LongestCircuit", "ILP"],
    );

    // The constructed chain is reusable: the final target is a genuine ILP<bool>.
    use crate::models::algebraic::ILP;
    let ilp = measured
        .target_problem_any()
        .downcast_ref::<ILP<bool>>()
        .expect("final target is ILP<bool>");
    assert_eq!(ilp.num_vars, 105);
}

#[test]
fn test_measured_any_target_uses_one_request_limit_tracker() {
    let hc = prism_hamiltonian_circuit();
    let graph = ReductionGraph::new();
    let variant = ReductionGraph::variant_to_map(&[("graph", "SimpleGraph")]);
    let outcome = graph
        .measured_front_to_name(
            "HamiltonianCircuit",
            &variant,
            "ILP",
            ReductionMode::Witness,
            &hc as &dyn Any,
            SizeBudget::new(BTreeMap::from([
                ("num_vars".to_string(), 1_000),
                ("num_constraints".to_string(), 1_000),
            ])),
            crate::rules::SearchMode::Approximate(crate::rules::ApproximationPolicy::Bounded(
                crate::rules::SearchLimits {
                    max_expanded_states: Some(1),
                    ..Default::default()
                },
            )),
        )
        .expect("valid budget");

    assert_eq!(outcome.stats.expanded_states, 1);
    assert!(outcome
        .completeness
        .reasons()
        .contains(&crate::rules::LimitReached::ExpandedStatesLimit));
}

// ---------------------------------------------------------------------------
// Verification 2: measured search does not discard equal-size concrete states.
// ---------------------------------------------------------------------------

#[test]
fn test_measured_search_keeps_equal_size_structure_dependent_instances() {
    let ilp_variant = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let graph = ReductionGraph::from_test_variant_edges(
        &[
            ("MeasuredSource", BTreeMap::new()),
            ("MeasuredBranchA", BTreeMap::new()),
            ("MeasuredBranchB", BTreeMap::new()),
            ("Satisfiability", BTreeMap::new()),
            ("ILP", ilp_variant.clone()),
        ],
        &[
            (
                "MeasuredSource",
                "MeasuredBranchA",
                measured_edge(measured_source_to_a, 0.0),
            ),
            (
                "MeasuredSource",
                "MeasuredBranchB",
                measured_edge(measured_source_to_b, 0.0),
            ),
            (
                "MeasuredBranchA",
                "Satisfiability",
                measured_edge(measured_a_to_sat, 0.0),
            ),
            (
                "MeasuredBranchB",
                "Satisfiability",
                measured_edge(measured_b_to_sat, 0.0),
            ),
            (
                "Satisfiability",
                "ILP",
                measured_edge(measured_sat_to_structure_dependent_ilp, 0.0),
            ),
        ],
    );
    let empty = BTreeMap::new();
    let source = MeasuredSource;

    let ilp = ILP::<bool>::new(1, vec![], vec![], ObjectiveSense::Minimize);
    let ilp_size = ReductionGraph::compute_source_size("ILP", &ilp_variant, &ilp);
    assert_eq!(ilp_size.get("num_vars"), Some(1));

    let bad_sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let good_sat = Satisfiability::new(1, vec![CNFClause::new(vec![-1])]);
    assert_eq!(
        ReductionGraph::compute_source_size("Satisfiability", &empty, &bad_sat),
        ReductionGraph::compute_source_size("Satisfiability", &empty, &good_sat),
        "the two structurally different hub instances must have identical measured sizes",
    );

    let measured = graph
        .measured_front(
            "MeasuredSource",
            &empty,
            "ILP",
            &ilp_variant,
            ReductionMode::Witness,
            &source,
            SizeBudget::new(BTreeMap::from([
                ("num_vars".to_string(), 1_000),
                ("num_constraints".to_string(), 1_000),
            ])),
            crate::rules::SearchMode::Exact,
        )
        .expect("valid budget")
        .value
        .into_iter()
        .find(|path| {
            path.path.type_names() == ["MeasuredSource", "MeasuredBranchB", "Satisfiability", "ILP"]
        })
        .expect("the structure-dependent small continuation must survive");

    assert_eq!(
        measured.path.type_names(),
        ["MeasuredSource", "MeasuredBranchB", "Satisfiability", "ILP",],
    );
    assert_eq!(measured.size.get("num_vars"), Some(1));
}

#[test]
fn test_asymptotic_overhead_is_not_a_concrete_budget_guard() {
    let ilp_variant = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let graph = ReductionGraph::from_test_variant_edges(
        &[
            ("MeasuredSource", BTreeMap::new()),
            ("ILP", ilp_variant.clone()),
        ],
        &[(
            "MeasuredSource",
            "ILP",
            measured_edge(measured_source_to_small_ilp, 1_000_000.0),
        )],
    );
    let empty = BTreeMap::new();
    let source = MeasuredSource;

    let measured = graph
        .measured_front(
            "MeasuredSource",
            &empty,
            "ILP",
            &ilp_variant,
            ReductionMode::Witness,
            &source,
            SizeBudget::new(BTreeMap::from([
                ("num_vars".to_string(), 1),
                ("num_constraints".to_string(), 1),
            ])),
            crate::rules::SearchMode::Exact,
        )
        .expect("valid budget")
        .value
        .into_iter()
        .next()
        .expect("the only explicit route is in budget");

    assert_eq!(measured.size.get("num_vars"), Some(1));
}

fn measured_two_route_graph(
    a_to_ilp: fn(&dyn Any) -> Box<dyn DynReductionResult>,
    b_to_ilp: fn(&dyn Any) -> Box<dyn DynReductionResult>,
) -> (ReductionGraph, BTreeMap<String, String>) {
    let ilp_variant = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    (
        ReductionGraph::from_test_variant_edges(
            &[
                ("MeasuredSource", BTreeMap::new()),
                ("MeasuredBranchA", BTreeMap::new()),
                ("MeasuredBranchB", BTreeMap::new()),
                ("ILP", ilp_variant.clone()),
            ],
            &[
                (
                    "MeasuredSource",
                    "MeasuredBranchA",
                    measured_edge(measured_source_to_a, 0.0),
                ),
                (
                    "MeasuredSource",
                    "MeasuredBranchB",
                    measured_edge(measured_source_to_b, 0.0),
                ),
                ("MeasuredBranchA", "ILP", measured_edge(a_to_ilp, 0.0)),
                ("MeasuredBranchB", "ILP", measured_edge(b_to_ilp, 0.0)),
            ],
        ),
        ilp_variant,
    )
}

fn unlimited_ilp_budget() -> SizeBudget {
    SizeBudget::new(BTreeMap::from([
        ("num_vars".to_string(), usize::MAX),
        ("num_constraints".to_string(), usize::MAX),
    ]))
}

#[test]
fn test_measured_front_keeps_incomparable_vectors() {
    let (graph, target) = measured_two_route_graph(
        measured_a_to_incomparable_ilp,
        measured_b_to_incomparable_ilp,
    );
    let outcome = graph
        .measured_front(
            "MeasuredSource",
            &BTreeMap::new(),
            "ILP",
            &target,
            ReductionMode::Witness,
            &MeasuredSource,
            unlimited_ilp_budget(),
            crate::rules::SearchMode::Exact,
        )
        .expect("valid fields");
    let sizes: Vec<_> = outcome
        .value
        .iter()
        .map(|path| (path.size.get("num_vars"), path.size.get("num_constraints")))
        .collect();
    assert_eq!(sizes, [(Some(1), Some(10)), (Some(10), Some(1))]);
}

#[test]
fn test_measured_front_removes_dominated_and_deduplicates_equal_vectors() {
    let (graph, target) =
        measured_two_route_graph(measured_a_to_equal_ilp, measured_b_to_incomparable_ilp);
    let dominated = graph
        .measured_front(
            "MeasuredSource",
            &BTreeMap::new(),
            "ILP",
            &target,
            ReductionMode::Witness,
            &MeasuredSource,
            unlimited_ilp_budget(),
            crate::rules::SearchMode::Exact,
        )
        .expect("valid fields")
        .value;
    assert_eq!(dominated.len(), 1);
    assert_eq!(dominated[0].size.get("num_vars"), Some(2));

    let (graph, target) =
        measured_two_route_graph(measured_a_to_equal_ilp, measured_b_to_equal_ilp);
    let equal = graph
        .measured_front(
            "MeasuredSource",
            &BTreeMap::new(),
            "ILP",
            &target,
            ReductionMode::Witness,
            &MeasuredSource,
            unlimited_ilp_budget(),
            crate::rules::SearchMode::Exact,
        )
        .expect("valid fields")
        .value;
    assert_eq!(equal.len(), 1);
    assert_eq!(
        equal[0].path.type_names(),
        ["MeasuredSource", "MeasuredBranchA", "ILP"]
    );
}

#[test]
fn test_measured_budget_is_per_field_and_post_construction() {
    CONSTRUCTIONS.with(|count| count.set(0));
    let target = ReductionGraph::variant_to_map(&ILP::<bool>::variant());
    let graph = ReductionGraph::from_test_variant_edges(
        &[("MeasuredSource", BTreeMap::new()), ("ILP", target.clone())],
        &[(
            "MeasuredSource",
            "ILP",
            measured_edge(counted_large_ilp, 0.0),
        )],
    );
    let outcome = graph
        .measured_front(
            "MeasuredSource",
            &BTreeMap::new(),
            "ILP",
            &target,
            ReductionMode::Witness,
            &MeasuredSource,
            SizeBudget::new(BTreeMap::from([("num_vars".to_string(), 1)])),
            crate::rules::SearchMode::Exact,
        )
        .expect("known field");
    assert!(outcome.value.is_empty());
    assert_eq!(
        CONSTRUCTIONS.with(Cell::get),
        1,
        "budget is checked after construction"
    );

    let error = graph
        .measured_front(
            "MeasuredSource",
            &BTreeMap::new(),
            "ILP",
            &target,
            ReductionMode::Witness,
            &MeasuredSource,
            SizeBudget::new(BTreeMap::from([("not_a_size_field".to_string(), 1)])),
            crate::rules::SearchMode::Exact,
        )
        .err()
        .expect("unknown field must fail");
    assert_eq!(error.0, "not_a_size_field");

    let allowed = graph
        .measured_front(
            "MeasuredSource",
            &BTreeMap::new(),
            "ILP",
            &target,
            ReductionMode::Witness,
            &MeasuredSource,
            SizeBudget::new(BTreeMap::from([("num_constraints".to_string(), 0)])),
            crate::rules::SearchMode::Exact,
        )
        .expect("known field");
    assert_eq!(
        allowed.value.len(),
        1,
        "missing intermediate fields are not fabricated"
    );
}

// ---------------------------------------------------------------------------
// Verification 4: negative control on a hand-built diamond.
// ---------------------------------------------------------------------------

/// A test label whose objective is the *final* measured size `s`, while carrying a
/// separate accumulated step cost `c`. All intermediate labels survive; componentwise
/// Pareto order over `(c, s)` is applied only to completed paths.
#[derive(Clone)]
struct DiamondLabel {
    /// Accumulated step cost.
    c: f64,
    /// Current (path-dependent) measured size.
    s: f64,
}

impl DiamondLabel {
    fn ctx(&self) -> ProblemSize {
        ProblemSize::new(vec![("s", self.s.round().max(0.0) as usize)])
    }
}

impl PathLabel for DiamondLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        let ctx = self.ctx();
        let add_c = edge.overhead.get("c").map(|e| e.eval(&ctx)).unwrap_or(0.0);
        let new_s = edge
            .overhead
            .get("s")
            .map(|e| e.eval(&ctx))
            .unwrap_or(self.s);
        Some(DiamondLabel {
            c: self.c + add_c,
            s: new_s,
        })
    }

    fn final_dominates(&self, other: &Self) -> bool {
        self.c <= other.c && self.s <= other.s
    }
}

fn diamond_edge(c: f64, s: Expr) -> ReductionEdgeData {
    ReductionEdgeData {
        overhead: ReductionOverhead::new(vec![("c", Expr::Const(c)), ("s", s)]),
        reduce_fn: Some(measured_source_to_a),
        reduce_aggregate_fn: None,
        turing: false,
    }
}

/// Negative control: the two terminal vectors are incomparable, so both survive.
#[test]
fn test_negative_control_diamond_keeps_componentwise_front() {
    let empty = std::collections::BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "M", "P", "T"],
        &[
            // S -> M: cheap first edge (c=1), large intermediate size (s=100).
            ("S", "M", diamond_edge(1.0, Expr::Const(100.0))),
            // S -> P: pricier first edge (c=2), small size (s=5).
            ("S", "P", diamond_edge(2.0, Expr::Const(5.0))),
            // P -> M: small size (s=6).
            ("P", "M", diamond_edge(1.0, Expr::Const(6.0))),
            // M -> T: identity on size (final size = size at M).
            ("M", "T", diamond_edge(1.0, Expr::Var("s"))),
        ],
    );

    let initial = DiamondLabel { c: 0.0, s: 0.0 };
    let front = graph
        .pareto_search_by_name(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            initial,
            crate::rules::SearchMode::Exact,
        )
        .value;
    assert!(!front.is_empty(), "front should reach T");
    assert_eq!(front.len(), 2);
    assert!(front
        .iter()
        .any(|(path, label)| path.type_names() == ["S", "M", "T"] && label.s == 100.0));
    assert!(front
        .iter()
        .any(|(path, label)| path.type_names() == ["S", "P", "M", "T"] && label.s == 6.0));
}

/// Exact multi-label search retains both incomparable routes into M.
#[test]
fn test_diamond_exact_multi_label_keeps_incomparable_routes() {
    let empty = std::collections::BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "M", "P", "T"],
        &[
            ("S", "M", diamond_edge(1.0, Expr::Const(100.0))),
            ("S", "P", diamond_edge(2.0, Expr::Const(5.0))),
            ("P", "M", diamond_edge(1.0, Expr::Const(6.0))),
            ("M", "T", diamond_edge(1.0, Expr::Var("s"))),
        ],
    );
    let front = graph
        .pareto_search_by_name(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            DiamondLabel { c: 0.0, s: 0.0 },
            crate::rules::SearchMode::Exact,
        )
        .value;
    assert_eq!(front.len(), 2);
    assert!(front
        .iter()
        .any(|(path, label)| path.type_names() == ["S", "M", "T"] && label.c == 2.0));
    assert!(front
        .iter()
        .any(|(path, label)| path.type_names() == ["S", "P", "M", "T"] && label.c == 4.0));
}

// ---------------------------------------------------------------------------
// GrowthLabel (asymptotic, instance-free) domain — design M3/F3a.
// ---------------------------------------------------------------------------

/// A power `Var(v)^k`.
fn powk(v: &'static str, k: f64) -> Expr {
    Expr::pow(Expr::Var(v), Expr::Const(k))
}

/// A test edge carrying only a symbolic overhead (target field → Expr over the
/// current node's fields), no executable reduction.
fn growth_edge(fields: Vec<(&'static str, Expr)>) -> ReductionEdgeData {
    ReductionEdgeData {
        overhead: ReductionOverhead::new(fields),
        reduce_fn: Some(measured_source_to_a),
        reduce_aggregate_fn: None,
        turing: false,
    }
}

/// The rendered Big-O string for one field of a growth label (or `"?"` for
/// `Unknown`), for compact assertions.
fn field_big_o(label: &GrowthLabel, field: &str) -> String {
    match label.fields().get(field) {
        Some(g) => match g.to_expr() {
            Some(e) => e.to_string(),
            None => "?".to_string(),
        },
        None => "<absent>".to_string(),
    }
}

/// `extend` substitutes the current label's growth into an edge's overhead and
/// reduces in the growth domain, yielding the target field's growth in source vars.
#[test]
fn test_growth_label_extend_composes_overhead() {
    // Source S has fields n, m; edge maps a = n^2, b = m (in the source's variables).
    let edge_data = growth_edge(vec![("a", powk("n", 2.0)), ("b", Expr::Var("m"))]);
    let target_variant = BTreeMap::new();
    let redge = ReductionEdge {
        overhead: &edge_data.overhead,
        reduce_fn: None,
        target_name: "Target",
        target_variant: &target_variant,
    };

    let initial = GrowthLabel::source(&["n", "m"]);
    let next = initial
        .extend(&redge)
        .expect("asymptotic extend never prunes");
    assert_eq!(field_big_o(&next, "a"), "n^2");
    assert_eq!(field_big_o(&next, "b"), "m");

    // A second hop composes: c = a * b substitutes a→n^2, b→m ⇒ n^2 * m.
    let edge2 = growth_edge(vec![("c", Expr::Var("a") * Expr::Var("b"))]);
    let redge2 = ReductionEdge {
        overhead: &edge2.overhead,
        reduce_fn: None,
        target_name: "Target2",
        target_variant: &target_variant,
    };
    let composed = next.extend(&redge2).expect("extend");
    assert_eq!(field_big_o(&composed, "c"), "m * n^2");
}

/// An overhead field that depends on an `Unknown`-growth current field stays
/// `Unknown` — the bound is never fabricated.
#[test]
fn test_growth_label_propagates_unknown() {
    // Build a label whose field `x` is Unknown (factorial growth).
    let mut fields = BTreeMap::new();
    fields.insert(
        "x",
        Growth::from_expr(&Expr::Factorial(Box::new(Expr::Var("n")))),
    );
    fields.insert("y", Growth::from_expr(&Expr::Var("n")));
    let label = GrowthLabel::from_fields(fields);
    assert!(matches!(label.fields().get("x"), Some(Growth::Unknown)));

    // out1 uses x (Unknown) → Unknown; out2 uses only y → bounded.
    let edge = growth_edge(vec![
        ("out1", Expr::Var("x") * Expr::Var("y")),
        ("out2", powk("y", 2.0)),
    ]);
    let tv = BTreeMap::new();
    let redge = ReductionEdge {
        overhead: &edge.overhead,
        reduce_fn: None,
        target_name: "T",
        target_variant: &tv,
    };
    let next = label.extend(&redge).expect("extend");
    assert_eq!(field_big_o(&next, "out1"), "?");
    assert_eq!(field_big_o(&next, "out2"), "n^2");
}

#[test]
fn test_symbolic_front_excludes_unknown_with_analysis_reason() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "Known", "Unknown", "T"],
        &[
            ("S", "Known", growth_edge(vec![("x", Expr::Const(1.0))])),
            ("Known", "T", growth_edge(vec![("out", Expr::Var("x"))])),
            (
                "S",
                "Unknown",
                growth_edge(vec![("x", Expr::Var("missing"))]),
            ),
            ("Unknown", "T", growth_edge(vec![("out", Expr::Var("x"))])),
        ],
    );
    let outcome = graph.asymptotic_front(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        crate::rules::SearchMode::Exact,
    );
    assert!(outcome.completeness.is_exact());
    let result = outcome.value.expect("known route is analyzable");
    assert_eq!(result.front.len(), 1);
    assert_eq!(result.excluded.len(), 1);
    assert_eq!(result.coverage.analyzed_paths, 1);
    assert_eq!(result.coverage.excluded_paths, 1);
    assert_eq!(result.excluded[0].failure.fields, ["out"]);
    assert!(result.excluded[0].failure.reason.contains("Unknown"));
}

#[test]
fn test_symbolic_coverage_counts_dominated_analyzable_paths() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["MaximumIndependentSet", "Small", "Large", "T"],
        &[
            (
                "MaximumIndependentSet",
                "Small",
                growth_edge(vec![("x", Expr::Const(1.0))]),
            ),
            ("Small", "T", growth_edge(vec![("out", Expr::Var("x"))])),
            (
                "MaximumIndependentSet",
                "Large",
                growth_edge(vec![("x", Expr::Var("num_vertices"))]),
            ),
            ("Large", "T", growth_edge(vec![("out", Expr::Var("x"))])),
        ],
    );
    let result = graph
        .asymptotic_front(
            "MaximumIndependentSet",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            crate::rules::SearchMode::Exact,
        )
        .value
        .expect("both routes are analyzable");
    assert_eq!(result.front.len(), 1);
    assert_eq!(result.coverage.analyzed_paths, 2);
    assert_eq!(result.coverage.excluded_paths, 0);
}

#[test]
fn test_symbolic_front_all_unknown_is_explicit_error() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "T"],
        &[("S", "T", growth_edge(vec![("out", Expr::Var("missing"))]))],
    );
    let error = graph
        .asymptotic_front(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            crate::rules::SearchMode::Exact,
        )
        .value
        .expect_err("all Unknown routes must not yield a front");
    assert_eq!(error.excluded.len(), 1);
    assert_eq!(error.coverage.analyzed_paths, 0);
    assert_eq!(error.coverage.excluded_paths, 1);
}

#[test]
fn test_symbolic_all_discovered_unknown_can_still_be_search_incomplete() {
    use crate::rules::{ApproximationPolicy, LimitReached, SearchLimits, SearchMode};

    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "B", "C", "T"],
        &[
            ("S", "A", growth_edge(vec![("x", Expr::Var("missing"))])),
            ("A", "T", growth_edge(vec![("out", Expr::Var("x"))])),
            ("S", "B", growth_edge(vec![("x", Expr::Const(1.0))])),
            ("B", "C", growth_edge(vec![("x", Expr::Var("x"))])),
            ("C", "T", growth_edge(vec![("out", Expr::Var("x"))])),
        ],
    );
    let outcome = graph.asymptotic_front(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        SearchMode::Approximate(ApproximationPolicy::Bounded(SearchLimits {
            max_hops: Some(2),
            ..Default::default()
        })),
    );
    assert!(outcome.value.is_err());
    assert!(outcome
        .completeness
        .reasons()
        .contains(&LimitReached::HopLimit));
}

/// Unknown is an analysis boundary and never participates in dominance.
#[test]
fn test_growth_label_unknown_is_incomparable() {
    let known = GrowthLabel::from_fields({
        let mut m = BTreeMap::new();
        m.insert("a", Growth::from_expr(&powk("n", 2.0)));
        m.insert("b", Growth::from_expr(&Expr::Var("m")));
        m
    });
    let with_unknown = GrowthLabel::from_fields({
        let mut m = BTreeMap::new();
        m.insert("a", Growth::from_expr(&powk("n", 2.0)));
        m.insert("b", Growth::Unknown);
        m
    });
    assert!(!known.final_dominates(&with_unknown));
    assert!(!with_unknown.final_dominates(&known));
}

/// Componentwise terminal dominance: `self` dominates `other` iff it grows no faster on
/// every field, including equality.
#[test]
fn test_growth_label_terminal_dominance_partial_order() {
    let a = GrowthLabel::from_fields({
        let mut m = BTreeMap::new();
        m.insert("v", Growth::from_expr(&Expr::Var("n"))); // n
        m.insert("e", Growth::from_expr(&Expr::Var("m"))); // m
        m
    });
    let b = GrowthLabel::from_fields({
        let mut m = BTreeMap::new();
        m.insert("v", Growth::from_expr(&powk("n", 2.0))); // n^2
        m.insert("e", Growth::from_expr(&Expr::Var("m"))); // m
        m
    });
    // a (n, m) grows slower in v, equal in e ⇒ a dominates b; b does not dominate a.
    assert!(a.final_dominates(&b));
    assert!(!b.final_dominates(&a));
    assert!(a.final_dominates(&a.clone()));

    // Incomparable pair: one better in v, the other better in e.
    let c = GrowthLabel::from_fields({
        let mut m = BTreeMap::new();
        m.insert("v", Growth::from_expr(&powk("n", 2.0))); // n^2
        m.insert("e", Growth::from_expr(&Expr::Var("m"))); // m
        m
    });
    let d = GrowthLabel::from_fields({
        let mut m = BTreeMap::new();
        m.insert("v", Growth::from_expr(&Expr::Var("n"))); // n
        m.insert("e", Growth::from_expr(&powk("m", 2.0))); // m^2
        m
    });
    assert!(!c.final_dominates(&d));
    assert!(!d.final_dominates(&c));
}

/// **Negative control:** two S→T paths whose composed growths are
/// incomparable — path A costs `O(n^2)` in `vertices` / `O(m)` in `edges`, path B
/// costs `O(n)` / `O(m^2)` — must *both* appear in the asymptotic Pareto front. An
/// implementation that scalarizes or keeps a single representative fails this.
#[test]
fn test_growth_negative_control_incomparable_front() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "B", "T"],
        &[
            // Both prefixes just carry the source fields n, m through unchanged.
            (
                "S",
                "A",
                growth_edge(vec![("n", Expr::Var("n")), ("m", Expr::Var("m"))]),
            ),
            (
                "S",
                "B",
                growth_edge(vec![("n", Expr::Var("n")), ("m", Expr::Var("m"))]),
            ),
            // Path A: vertices = n^2, edges = m.
            (
                "A",
                "T",
                growth_edge(vec![
                    ("vertices", powk("n", 2.0)),
                    ("edges", Expr::Var("m")),
                ]),
            ),
            // Path B: vertices = n, edges = m^2.
            (
                "B",
                "T",
                growth_edge(vec![
                    ("vertices", Expr::Var("n")),
                    ("edges", powk("m", 2.0)),
                ]),
            ),
        ],
    );

    let initial = GrowthLabel::source(&["n", "m"]);
    let front = graph
        .pareto_search_by_name(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            initial,
            crate::rules::SearchMode::Exact,
        )
        .value;

    // The front must contain BOTH incomparable paths — not one representative.
    assert_eq!(
        front.len(),
        2,
        "front should keep both incomparable paths, got {:?}",
        front
            .iter()
            .map(|(p, _)| p.type_names())
            .collect::<Vec<_>>()
    );
    let mut seen: Vec<(String, String)> = front
        .iter()
        .map(|(p, label)| {
            (
                p.type_names().join("→"),
                format!(
                    "v={} e={}",
                    field_big_o(label, "vertices"),
                    field_big_o(label, "edges")
                ),
            )
        })
        .collect();
    seen.sort();
    assert_eq!(
        seen,
        vec![
            ("S→A→T".to_string(), "v=n^2 e=m".to_string()),
            ("S→B→T".to_string(), "v=n e=m^2".to_string()),
        ],
    );
}

// Completeness under ASYMMETRIC magnitudes: the two incomparable paths have
// different scalar `cost` summaries (A: n^2 + m ⇒ magnitude 3; B: n + m^3 ⇒
// magnitude 4). A scalar branch-and-bound (were the kernel to use one) would let the
// cheaper path A complete first and then prune B (cost 4 ≥ 3), silently dropping a
// Pareto-optimal path. This is the case the equal-magnitude negative control above
// does NOT catch; it passes because the kernel never uses scalar `cost` to prune.
#[test]
fn test_growth_asymmetric_incomparable_front_complete() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "B", "T"],
        &[
            (
                "S",
                "A",
                growth_edge(vec![("n", Expr::Var("n")), ("m", Expr::Var("m"))]),
            ),
            (
                "S",
                "B",
                growth_edge(vec![("n", Expr::Var("n")), ("m", Expr::Var("m"))]),
            ),
            // Path A: vertices = n^2, edges = m   (magnitude 2 + 1 = 3).
            (
                "A",
                "T",
                growth_edge(vec![
                    ("vertices", powk("n", 2.0)),
                    ("edges", Expr::Var("m")),
                ]),
            ),
            // Path B: vertices = n, edges = m^3   (magnitude 1 + 3 = 4).
            (
                "B",
                "T",
                growth_edge(vec![
                    ("vertices", Expr::Var("n")),
                    ("edges", powk("m", 3.0)),
                ]),
            ),
        ],
    );

    let front = graph
        .pareto_search_by_name(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            GrowthLabel::source(&["n", "m"]),
            crate::rules::SearchMode::Exact,
        )
        .value;

    let mut seen: Vec<(String, String)> = front
        .iter()
        .map(|(p, label)| {
            (
                p.type_names().join("→"),
                format!(
                    "v={} e={}",
                    field_big_o(label, "vertices"),
                    field_big_o(label, "edges")
                ),
            )
        })
        .collect();
    seen.sort();
    assert_eq!(
        seen,
        vec![
            ("S→A→T".to_string(), "v=n^2 e=m".to_string()),
            ("S→B→T".to_string(), "v=n e=m^3".to_string()),
        ],
        "both incomparable paths must survive despite different scalar magnitudes",
    );
}

/// Positive monotone overheads preserve GrowthLabel's terminal order. This is useful in
/// the symbolic domain, but the kernel does not rely on it for intermediate pruning
/// because repository overheads are not restricted to this subset.
#[test]
fn test_growth_label_monotone_overhead_preserves_order() {
    // A = (n, m) dominates B = (n^2, m^2) componentwise.
    let a = GrowthLabel::source(&["n", "m"]);
    let b = GrowthLabel::from_fields({
        let mut mm = BTreeMap::new();
        mm.insert("n", Growth::from_expr(&powk("n", 2.0)));
        mm.insert("m", Growth::from_expr(&powk("m", 2.0)));
        mm
    });
    assert!(a.final_dominates(&b));

    let tv = BTreeMap::new();
    // A monotone overhead in both fields.
    for overhead in [
        growth_edge(vec![("x", Expr::Var("n") * Expr::Var("m"))]),
        growth_edge(vec![("x", powk("n", 3.0)), ("y", Expr::Var("m"))]),
    ] {
        let redge = ReductionEdge {
            overhead: &overhead.overhead,
            reduce_fn: None,
            target_name: "T",
            target_variant: &tv,
        };
        let ea = a.extend(&redge).unwrap();
        let eb = b.extend(&redge).unwrap();
        // A ⪰ B ⇒ extend(A) ⪰ extend(B) (dominates-or-equal). Equality is possible
        // when the overhead collapses the difference, so accept dominate-or-equal.
        assert!(
            ea.final_dominates(&eb) || ea == eb,
            "monotone overhead reversed growth order: {ea:?} vs {eb:?}"
        );
    }
}

/// `asymptotic_front` reports **one representative per distinct growth vector**, not
/// one per route. On the real graph, `MinimumVertexCover → ILP` has many syntactically
/// distinct chains that compose to the same Big-O profile; terminal equality filtering
/// must leave no duplicate growth vectors.
#[test]
fn test_asymptotic_front_dedups_by_growth_vector() {
    let graph = ReductionGraph::new();
    let src_v = graph
        .default_variant_for("MinimumVertexCover")
        .or_else(|| graph.variants_for("MinimumVertexCover").into_iter().next())
        .expect("MinimumVertexCover registered");
    let dst_v = graph
        .default_variant_for("ILP")
        .or_else(|| graph.variants_for("ILP").into_iter().next())
        .expect("ILP registered");

    let front = graph
        .asymptotic_front(
            "MinimumVertexCover",
            &src_v,
            "ILP",
            &dst_v,
            ReductionMode::Witness,
            crate::rules::SearchMode::Exact,
        )
        .value
        .expect("at least one analyzable path")
        .front;
    assert!(!front.is_empty(), "MVC -> ILP must have a path");

    // No two front entries share a growth vector (GrowthLabel PartialEq).
    for i in 0..front.len() {
        for j in (i + 1)..front.len() {
            assert!(
                front[i].1 != front[j].1,
                "duplicate growth vector in front:\n  {}\n  {}",
                front[i].0.type_names().join("→"),
                front[j].0.type_names().join("→"),
            );
        }
    }
    // The generic kernel itself performs terminal filtering, so the public wrapper does
    // not need a second deduplication pass.
    let src_fields = graph.size_field_names("MinimumVertexCover");
    let raw = graph
        .pareto_search_by_name(
            "MinimumVertexCover",
            &src_v,
            "ILP",
            &dst_v,
            ReductionMode::Witness,
            GrowthLabel::source(&src_fields),
            crate::rules::SearchMode::Exact,
        )
        .value;
    assert_eq!(raw.len(), front.len());
}

/// A composed front label must express every size field's growth purely in the
/// **source problem's** own size variables — never in a downstream getter alias or an
/// intermediate node's field name.
///
/// Regression for the `MinimumFeedbackVertexSet → ILP` bug: the `ILP<i32> → ILP<bool>`
/// binary-encoding cast declared its overhead as `num_vars = "31 * num_variables"`,
/// referencing the getter *alias* `num_variables()` instead of ILP's size-field *name*
/// `num_vars`. Instance mode and raw-overhead rendering both resolve the getter, so the
/// mistake was invisible there — but growth composition threads field *names*, so the
/// alias was unmapped and leaked through as `num_vars = O(num_variables)` instead of
/// the correct `O(num_vertices)`.
#[test]
fn test_asymptotic_front_uses_only_source_variables_mfvs_ilp() {
    let graph = ReductionGraph::new();
    let src_v = graph
        .default_variant_for("MinimumFeedbackVertexSet")
        .or_else(|| {
            graph
                .variants_for("MinimumFeedbackVertexSet")
                .into_iter()
                .next()
        })
        .expect("MinimumFeedbackVertexSet registered");
    let dst_v = graph
        .default_variant_for("ILP")
        .or_else(|| graph.variants_for("ILP").into_iter().next())
        .expect("ILP registered");

    let front = graph
        .asymptotic_front(
            "MinimumFeedbackVertexSet",
            &src_v,
            "ILP",
            &dst_v,
            ReductionMode::Witness,
            crate::rules::SearchMode::Exact,
        )
        .value
        .expect("at least one analyzable path")
        .front;

    // The direct route (MFVS → ILP/i32 → ILP/bool; the ILP variants collapse in the
    // deduplicated node-name view) is the one exercised by the fixed cast.
    let (_, label) = front
        .iter()
        .find(|(p, _)| p.type_names() == ["MinimumFeedbackVertexSet", "ILP"])
        .expect("direct MinimumFeedbackVertexSet -> ILP path");

    // The size fields of MinimumFeedbackVertexSet — the only variables any composed
    // growth is allowed to mention.
    let allowed = ["num_arcs", "num_vertices"];
    for (field, growth) in label.fields() {
        let expr = growth
            .to_expr()
            .unwrap_or_else(|| panic!("field {field} should have a bounded growth"));
        for var in expr.variables() {
            assert!(
                allowed.contains(&var),
                "field `{field}` growth O({expr}) references `{var}`, which is not a \
                 MinimumFeedbackVertexSet source variable {allowed:?}",
            );
        }
    }

    // The previously-buggy field, pinned to the correct source-variable Big-O.
    let num_vars = label
        .fields()
        .get("num_vars")
        .expect("ILP has a num_vars size field");
    assert_eq!(
        num_vars.to_expr().unwrap().to_string(),
        "num_vertices",
        "ILP num_vars must compose to O(num_vertices), not the getter alias num_variables"
    );
}

// ---------------------------------------------------------------------------
// Fix A: the kernel never applies intermediate pruning or branch-and-bound.
// ---------------------------------------------------------------------------

/// A test label whose `cost` is the label's current absolute value — a value a late edge
/// can *shrink* below an already-completed route's final value. It verifies that the
/// generic kernel does not silently add scalar branch-and-bound.
#[derive(Clone)]
struct ShrinkLabel {
    v: f64,
}

#[derive(Clone)]
struct FormulaSizeLabel(ProblemSize);

impl PathLabel for FormulaSizeLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        Some(Self(edge.overhead.evaluate_output_size(&self.0)))
    }

    fn final_dominates(&self, other: &Self) -> bool {
        self.0.components.len() == other.0.components.len()
            && self
                .0
                .components
                .iter()
                .all(|(field, value)| other.0.get(field).is_some_and(|other| *value <= other))
    }
}

#[test]
fn test_pareto_search_matches_independent_small_graph_oracle() {
    const NAMES: [&str; 7] = ["N0", "N1", "N2", "N3", "N4", "N5", "N6"];

    fn enumerate(
        node: usize,
        target: usize,
        adjacency: &[Vec<(usize, usize, usize)>],
        path: &mut Vec<usize>,
        terminal: &mut Vec<(Vec<usize>, (usize, usize))>,
    ) {
        if node == target {
            let edge = adjacency[path[path.len() - 2]]
                .iter()
                .find(|(next, _, _)| *next == target)
                .expect("terminal edge");
            terminal.push((path.clone(), (edge.1, edge.2)));
            return;
        }
        for &(next, _, _) in &adjacency[node] {
            if path.contains(&next) {
                continue;
            }
            path.push(next);
            enumerate(next, target, adjacency, path, terminal);
            path.pop();
        }
    }

    for nodes in 2..=7 {
        let mut state = 0x5eed_u64 + nodes as u64;
        let mut adjacency = vec![Vec::new(); nodes];
        let mut edges = Vec::new();
        for source in 0..nodes - 1 {
            for (target, target_name) in NAMES.iter().enumerate().take(nodes).skip(source + 1) {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                if target == source + 1 || state.is_multiple_of(3) {
                    let a = ((state >> 8) % 9 + 1) as usize;
                    let b = ((state >> 16) % 9 + 1) as usize;
                    adjacency[source].push((target, a, b));
                    edges.push((
                        NAMES[source],
                        *target_name,
                        growth_edge(vec![
                            ("a", Expr::Const(a as f64)),
                            ("b", Expr::Const(b as f64)),
                        ]),
                    ));
                }
            }
        }
        let graph = ReductionGraph::from_test_edges(&NAMES[..nodes], &edges);
        let production = graph
            .pareto_search_by_name(
                NAMES[0],
                &BTreeMap::new(),
                NAMES[nodes - 1],
                &BTreeMap::new(),
                ReductionMode::Witness,
                FormulaSizeLabel(ProblemSize::new(vec![])),
                crate::rules::SearchMode::Exact,
            )
            .value;

        let mut terminal = Vec::new();
        enumerate(0, nodes - 1, &adjacency, &mut vec![0], &mut terminal);
        terminal.sort_by(|a, b| a.0.len().cmp(&b.0.len()).then_with(|| a.0.cmp(&b.0)));
        let mut oracle: Vec<(Vec<usize>, (usize, usize))> = Vec::new();
        for candidate in terminal {
            let dominates = |a: &(Vec<usize>, (usize, usize)), b: &(Vec<usize>, (usize, usize))| {
                a.1 .0 <= b.1 .0 && a.1 .1 <= b.1 .1
            };
            if oracle
                .iter()
                .any(|existing| dominates(existing, &candidate))
            {
                continue;
            }
            oracle.retain(|existing| !dominates(&candidate, existing));
            oracle.push(candidate);
        }
        let production_paths: Vec<Vec<&str>> = production
            .iter()
            .map(|(path, _)| path.type_names())
            .collect();
        let oracle_paths: Vec<Vec<&str>> = oracle
            .iter()
            .map(|(path, _)| path.iter().map(|node| NAMES[*node]).collect())
            .collect();
        assert_eq!(production_paths, oracle_paths, "node count {nodes}");
    }
}

#[derive(Clone)]
struct ContractLabel {
    agenda_cost: f64,
    downstream_cost: f64,
}

impl PathLabel for ContractLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        let empty = ProblemSize::new(vec![]);
        let downstream_cost = edge
            .overhead
            .get("downstream")
            .map(|expr| expr.eval(&empty))
            .unwrap_or(self.downstream_cost);
        let agenda_cost = edge
            .overhead
            .get("agenda")
            .map(|expr| expr.eval(&empty))
            .unwrap_or(self.agenda_cost);
        Some(Self {
            agenda_cost,
            downstream_cost,
        })
    }

    fn final_dominates(&self, other: &Self) -> bool {
        self.agenda_cost <= other.agenda_cost && self.downstream_cost <= other.downstream_cost
    }
}

/// Contract regression for explicit completeness. Exact crosses both former hidden
/// limits. Bounded approximate search reports the precise limit that removes a route,
/// and generous limits upgrade to an exact outcome.
#[test]
fn test_search_mode_exact_and_approximate_contract() {
    use crate::rules::{
        ApproximationPolicy, LimitReached, SearchCompleteness, SearchLimits, SearchMode,
    };

    let empty = BTreeMap::new();
    let node_names = [
        "N00", "N01", "N02", "N03", "N04", "N05", "N06", "N07", "N08", "N09", "N10", "N11", "N12",
        "N13", "N14", "N15", "N16", "N17",
    ];
    let long_edges: Vec<_> = node_names
        .windows(2)
        .map(|pair| (pair[0], pair[1], growth_edge(vec![])))
        .collect();
    let long_graph = ReductionGraph::from_test_edges(&node_names, &long_edges);
    let initial = ContractLabel {
        agenda_cost: 0.0,
        downstream_cost: 0.0,
    };

    let exact_long = long_graph.pareto_search_by_name(
        "N00",
        &empty,
        "N17",
        &empty,
        ReductionMode::Witness,
        initial.clone(),
        SearchMode::Exact,
    );
    assert_eq!(exact_long.completeness, SearchCompleteness::Exact);
    assert_eq!(exact_long.value[0].0.len(), 17);

    let capped_long = long_graph.pareto_search_by_name(
        "N00",
        &empty,
        "N17",
        &empty,
        ReductionMode::Witness,
        initial.clone(),
        SearchMode::Approximate(ApproximationPolicy::Bounded(SearchLimits {
            max_hops: Some(16),
            ..Default::default()
        })),
    );
    assert!(capped_long.value.is_empty());
    assert!(capped_long
        .completeness
        .reasons()
        .contains(&LimitReached::HopLimit));

    let generous_long = long_graph.pareto_search_by_name(
        "N00",
        &empty,
        "N17",
        &empty,
        ReductionMode::Witness,
        initial.clone(),
        SearchMode::Approximate(ApproximationPolicy::Bounded(SearchLimits {
            max_hops: Some(17),
            max_labels_per_node: Some(34),
            max_expanded_states: Some(100),
            timeout: None,
        })),
    );
    assert_eq!(generous_long.completeness, SearchCompleteness::Exact);
    assert_eq!(generous_long.value[0].0.len(), 17);

    let make_bag_graph = |reverse: bool| {
        let mut edges = (0..33)
            .map(|i| {
                (
                    "S",
                    "M",
                    growth_edge(vec![
                        ("agenda", Expr::Const((i + 1) as f64)),
                        ("downstream", Expr::Const((33 - i) as f64)),
                    ]),
                )
            })
            .collect::<Vec<_>>();
        if reverse {
            edges.reverse();
        }
        edges.push(("M", "T", growth_edge(vec![])));
        ReductionGraph::from_test_edges(&["S", "M", "T"], &edges)
    };

    let exact_bag = make_bag_graph(false).pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial.clone(),
        SearchMode::Exact,
    );
    assert_eq!(exact_bag.completeness, SearchCompleteness::Exact);
    let exact_labels: BTreeSet<_> = exact_bag
        .value
        .iter()
        .map(|(_, label)| (label.agenda_cost as usize, label.downstream_cost as usize))
        .collect();
    assert_eq!(exact_labels.len(), 33);

    let capped_bag = make_bag_graph(false).pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial.clone(),
        SearchMode::Approximate(ApproximationPolicy::Bounded(SearchLimits {
            max_labels_per_node: Some(32),
            ..Default::default()
        })),
    );
    let capped_labels: BTreeSet<_> = capped_bag
        .value
        .iter()
        .map(|(_, label)| (label.agenda_cost as usize, label.downstream_cost as usize))
        .collect();
    assert_eq!(capped_labels.len(), 32);
    assert_eq!(exact_labels.difference(&capped_labels).count(), 1);
    assert!(capped_bag
        .completeness
        .reasons()
        .contains(&LimitReached::LabelsPerNodeLimit));

    let reversed = make_bag_graph(true).pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial,
        SearchMode::Exact,
    );
    assert_eq!(reversed.completeness, SearchCompleteness::Exact);
    let reversed_labels: BTreeSet<_> = reversed
        .value
        .iter()
        .map(|(_, label)| (label.agenda_cost as usize, label.downstream_cost as usize))
        .collect();
    assert_eq!(reversed_labels, exact_labels);
}

/// Equal coarse labels with different paths must both survive. The route through Y is the
/// only one that can still visit X after M and reach final size zero.
#[test]
fn test_equal_labels_keep_incomparable_continuation_state() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "X", "Y", "M", "T"],
        &[
            ("S", "X", diamond_edge(0.0, Expr::Const(1.0))),
            ("X", "M", diamond_edge(0.0, Expr::Var("s"))),
            ("S", "Y", diamond_edge(0.0, Expr::Const(1.0))),
            ("Y", "M", diamond_edge(0.0, Expr::Var("s"))),
            ("M", "X", diamond_edge(0.0, Expr::Const(0.0))),
            ("X", "T", diamond_edge(0.0, Expr::Var("s"))),
        ],
    );

    let outcome = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        DiamondLabel { c: 0.0, s: 0.0 },
        crate::rules::SearchMode::Exact,
    );
    assert_eq!(outcome.value[0].1.s, 0.0);
    assert_eq!(
        outcome.value[0].0.type_names(),
        vec!["S", "Y", "M", "X", "T"]
    );
}

#[test]
fn test_equal_intermediate_labels_are_not_coalesced() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "M", "X", "T"],
        &[
            ("S", "M", diamond_edge(0.0, Expr::Const(1.0))),
            ("S", "X", diamond_edge(0.0, Expr::Const(1.0))),
            ("X", "M", diamond_edge(0.0, Expr::Var("s"))),
            ("M", "T", diamond_edge(0.0, Expr::Var("s"))),
        ],
    );

    let outcome = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        DiamondLabel { c: 0.0, s: 0.0 },
        crate::rules::SearchMode::Exact,
    );
    assert_eq!(outcome.stats.generated_states, 6);
    assert_eq!(outcome.stats.dominated_states, 1);
    assert_eq!(outcome.value[0].0.type_names(), vec!["S", "M", "T"]);
}

#[test]
fn test_state_and_timeout_limits_are_reported_before_expansion() {
    use crate::rules::{ApproximationPolicy, LimitReached, SearchLimits, SearchMode};
    use std::time::Duration;

    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(&["S", "T"], &[("S", "T", growth_edge(vec![]))]);
    let initial = ContractLabel {
        agenda_cost: 0.0,
        downstream_cost: 0.0,
    };

    let state_limited = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial.clone(),
        SearchMode::Approximate(ApproximationPolicy::Bounded(SearchLimits {
            max_expanded_states: Some(0),
            ..Default::default()
        })),
    );
    assert_eq!(state_limited.stats.expanded_states, 0);
    assert!(state_limited
        .completeness
        .reasons()
        .contains(&LimitReached::ExpandedStatesLimit));

    let timed_out = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial,
        SearchMode::Approximate(ApproximationPolicy::Bounded(SearchLimits {
            timeout: Some(Duration::ZERO),
            ..Default::default()
        })),
    );
    assert_eq!(timed_out.stats.expanded_states, 0);
    assert!(timed_out
        .completeness
        .reasons()
        .contains(&LimitReached::Timeout));
}

impl PathLabel for ShrinkLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        // The edge sets a new absolute value (`v`), which may be smaller than the current.
        let z = ProblemSize::new(vec![]);
        let v = edge.overhead.get("v").map(|e| e.eval(&z)).unwrap_or(self.v);
        Some(ShrinkLabel { v })
    }

    fn final_dominates(&self, other: &Self) -> bool {
        self.v <= other.v
    }
}

/// Kernel regression: a route that *shrinks late* (its intermediate value 100 is
/// higher than a rival route that completes early at 50, but a final edge drops it to 10)
/// must survive to the front. A kernel that applied branch-and-bound would prune the
/// intermediate node based on 50 and silently drop the non-dominated terminal vector. Because
/// the kernel retains every intermediate label, the shrink-late route reaches the front.
#[test]
fn test_kernel_keeps_shrink_late_route_without_intermediate_pruning() {
    let empty = std::collections::BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "T"],
        &[
            // S -> T: completes early with final value 50.
            ("S", "T", growth_edge(vec![("v", Expr::Const(50.0))])),
            // S -> A: intermediate value 100 (would trip a B&B bound of 50).
            ("S", "A", growth_edge(vec![("v", Expr::Const(100.0))])),
            // A -> T: shrinks the value to 10.
            ("A", "T", growth_edge(vec![("v", Expr::Const(10.0))])),
        ],
    );

    let front = graph
        .pareto_search_by_name(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            ShrinkLabel { v: 0.0 },
            crate::rules::SearchMode::Exact,
        )
        .value;

    // The shrink-late route S -> A -> T (final value 10) must be present in the front.
    let shrink_late = front
        .iter()
        .find(|(p, _)| p.type_names() == ["S", "A", "T"])
        .expect("shrink-late route S -> A -> T must survive without branch-and-bound");
    assert_eq!(
        shrink_late.1.v, 10.0,
        "the shrink-late route finishes at value 10"
    );
    assert_eq!(front.len(), 1, "the dominated terminal vector is removed");
}

// ---------------------------------------------------------------------------
// Formula-vector labels retain every intermediate route.
// ---------------------------------------------------------------------------

/// Formula vectors retain incomparable routes without scalar selection.
#[test]
fn test_formula_vector_keeps_incomparable_routes() {
    let empty = std::collections::BTreeMap::new();
    // Edges carry `c`, `wf`, and tracked size field `w`; the terminal vector remains
    // componentwise and is never collapsed into one scalar.
    let graph = ReductionGraph::from_test_edges(
        &["S", "M", "P", "T"],
        &[
            // S -> M: cheap prefix (c = 1) but expands the source size from 10 to 100.
            (
                "S",
                "M",
                growth_edge(vec![
                    ("c", Expr::Const(1.0)),
                    ("wf", Expr::Const(0.0)),
                    ("w", Expr::Const(10.0) * Expr::Var("w")),
                ]),
            ),
            // S -> P: pricier prefix (c = 3) but shrinks the source size from 10 to 1.
            (
                "S",
                "P",
                growth_edge(vec![
                    ("c", Expr::Const(3.0)),
                    ("wf", Expr::Const(0.0)),
                    ("w", Expr::Var("w") / Expr::Const(10.0)),
                ]),
            ),
            // P -> M: cheap (c = 1), keeps the small size w = 1.
            (
                "P",
                "M",
                growth_edge(vec![
                    ("c", Expr::Const(1.0)),
                    ("wf", Expr::Const(0.0)),
                    ("w", Expr::Var("w")),
                ]),
            ),
            // M -> T: cost = current w (wf = 1, c = 0); identity on size.
            (
                "M",
                "T",
                growth_edge(vec![
                    ("c", Expr::Const(0.0)),
                    ("wf", Expr::Const(1.0)),
                    ("w", Expr::Var("w")),
                ]),
            ),
        ],
    );

    let front = graph
        .pareto_search_by_name(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            FormulaSizeLabel(ProblemSize::new(vec![("w", 10)])),
            crate::rules::SearchMode::Exact,
        )
        .value;

    // Intermediate pruning could evict the small-w prefix at M and lose its terminal
    // vector, so the route must remain present.
    assert!(
        front
            .iter()
            .any(|(path, _)| path.type_names() == ["S", "P", "M", "T"]),
        "componentwise search must keep the small-w route"
    );
}

/// A legitimate reduction overhead may reverse componentwise size order. The smaller,
/// prefix at M must not discard the larger prefix, because complementing the edge count
/// reverses their terminal component order.
#[test]
fn test_formula_vector_nonmonotone_overhead_does_not_prune() {
    let empty = BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "B", "M", "T"],
        &[
            (
                "S",
                "A",
                growth_edge(vec![
                    ("n", Expr::Var("n")),
                    ("m", Expr::Var("m") - Expr::Const(3.0)),
                    ("edge_cost", Expr::Const(0.0)),
                ]),
            ),
            (
                "A",
                "M",
                growth_edge(vec![
                    ("n", Expr::Var("n")),
                    ("m", Expr::Var("m")),
                    ("edge_cost", Expr::Const(0.0)),
                ]),
            ),
            (
                "S",
                "B",
                growth_edge(vec![
                    ("n", Expr::Var("n")),
                    ("m", Expr::Var("m") + Expr::Const(3.0)),
                    ("edge_cost", Expr::Const(1.0)),
                ]),
            ),
            (
                "B",
                "M",
                growth_edge(vec![
                    ("n", Expr::Var("n")),
                    ("m", Expr::Var("m")),
                    ("edge_cost", Expr::Const(0.0)),
                ]),
            ),
            (
                "M",
                "T",
                growth_edge(vec![
                    (
                        "m",
                        Expr::Var("n") * (Expr::Var("n") - Expr::Const(1.0)) / Expr::Const(2.0)
                            - Expr::Var("m"),
                    ),
                    ("terminal", Expr::Const(1.0)),
                ]),
            ),
        ],
    );
    let front = graph
        .pareto_search_by_name(
            "S",
            &empty,
            "T",
            &empty,
            ReductionMode::Witness,
            FormulaSizeLabel(ProblemSize::new(vec![("n", 10), ("m", 5)])),
            crate::rules::SearchMode::Exact,
        )
        .value;

    assert!(front
        .iter()
        .any(|(path, _)| path.type_names() == ["S", "B", "M", "T"]));
}

// ---------------------------------------------------------------------------
// Fix C: GrowthLabel taints target fields referencing intermediate-only variables.
// ---------------------------------------------------------------------------

/// Fix C regression: an overhead output expression that references a variable ABSENT from
/// the current label (an intermediate-only field, e.g. `tseitin_*`, `num_encoding_bits`)
/// must taint its target field to `Growth::Unknown` — it must NOT pass through
/// `substitute` verbatim and surface as a fake source variable in the final bound.
#[test]
fn test_growth_label_taints_absent_variable() {
    // The label knows only the source field `n`.
    let label = GrowthLabel::source(&["n"]);
    // Edge output: `bounded` depends only on `n`; `leaky` references `tseitin`, which is
    // absent from the label (an intermediate-only construction variable).
    let edge = growth_edge(vec![
        ("bounded", Expr::Var("n")),
        ("leaky", Expr::Var("n") * Expr::Var("tseitin")),
    ]);
    let tv = BTreeMap::new();
    let redge = ReductionEdge {
        overhead: &edge.overhead,
        reduce_fn: None,
        target_name: "T",
        target_variant: &tv,
    };
    let next = label.extend(&redge).expect("extend");

    // Depends only on a mapped source variable ⇒ stays bounded.
    assert_eq!(field_big_o(&next, "bounded"), "n");
    // References an unmapped, intermediate-only variable ⇒ tainted to Unknown, never
    // leaked as `O(n * tseitin)`.
    assert!(
        matches!(next.fields().get("leaky"), Some(Growth::Unknown)),
        "a target field referencing an absent variable must become Unknown, got {:?}",
        next.fields().get("leaky")
    );
}

// ---------------------------------------------------------------------------
// Fix D: the arena frees evicted labels (bag cap bounds retained instance memory).
// ---------------------------------------------------------------------------

thread_local! {
    /// Live token instances on this thread.
    static TOK_LIVE: Cell<i64> = const { Cell::new(0) };
    /// Peak live token instances observed.
    static TOK_PEAK: Cell<i64> = const { Cell::new(0) };
    /// Total token instances ever created.
    static TOK_CREATED: Cell<i64> = const { Cell::new(0) };
}

/// A drop-tracking token. Each `new()` is a distinct live instance; `Drop` frees it. Held
/// behind `Rc` inside a label, so cloning a label shares the token. If the arena pinned
/// evicted labels, their tokens would stay live until the search ended, so `TOK_PEAK`
/// would reach `TOK_CREATED`.
struct DropToken;

impl DropToken {
    fn new() -> Self {
        let live = TOK_LIVE.with(|c| {
            let v = c.get() + 1;
            c.set(v);
            v
        });
        TOK_PEAK.with(|p| {
            if live > p.get() {
                p.set(live);
            }
        });
        TOK_CREATED.with(|c| c.set(c.get() + 1));
        DropToken
    }
}

impl Drop for DropToken {
    fn drop(&mut self) {
        TOK_LIVE.with(|c| c.set(c.get() - 1));
    }
}

/// A label carrying an `Rc<DropToken>` and a two-component `(c, s)` value. No
/// intermediate label is pruned, so an explicit approximate bag limit exercises the
/// truncation free path.
#[derive(Clone)]
struct TokenLabel {
    c: f64,
    s: f64,
    _tok: Rc<DropToken>,
}

impl PathLabel for TokenLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        let z = ProblemSize::new(vec![]);
        let c = edge.overhead.get("c").map(|e| e.eval(&z)).unwrap_or(self.c);
        let s = edge.overhead.get("s").map(|e| e.eval(&z)).unwrap_or(self.s);
        Some(TokenLabel {
            c,
            s,
            _tok: Rc::new(DropToken::new()),
        })
    }

    fn final_dominates(&self, other: &Self) -> bool {
        self.c <= other.c && self.s <= other.s
    }
}

/// Fix D regression: drive the kernel on a graph that generates far more labels at one hub
/// than an explicit bag limit, all incomparable so the bag truncates repeatedly. Because
/// truncated arena entries free their labels immediately, the *peak* number of live
/// `DropToken` instances stays well below the *total* ever created. If the arena pinned
/// evicted labels (the bug), peak would equal total.
#[test]
fn test_arena_frees_evicted_labels_bounds_live_memory() {
    TOK_LIVE.with(|c| c.set(0));
    TOK_PEAK.with(|c| c.set(0));
    TOK_CREATED.with(|c| c.set(0));

    // One hub M fed by N ≫ 32 parallel S -> M edges with pairwise-incomparable
    // (c = i+1, s = N-i) labels, then M -> T (identity). The M bag truncates repeatedly.
    let n: usize = 200;
    let mut edges: Vec<(&'static str, &'static str, ReductionEdgeData)> = Vec::new();
    // Leak small &'static str-free constants via Expr::Const (no string needed for values).
    for i in 0..n {
        edges.push((
            "S",
            "M",
            growth_edge(vec![
                ("c", Expr::Const((i + 1) as f64)),
                ("s", Expr::Const((n - i) as f64)),
            ]),
        ));
    }
    edges.push((
        "M",
        "T",
        growth_edge(vec![("c", Expr::Var("c")), ("s", Expr::Var("s"))]),
    ));
    let graph = ReductionGraph::from_test_edges(&["S", "M", "T"], &edges);

    let empty = std::collections::BTreeMap::new();
    let initial = TokenLabel {
        c: 0.0,
        s: 0.0,
        _tok: Rc::new(DropToken::new()),
    };
    let outcome = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial,
        crate::rules::SearchMode::Approximate(crate::rules::ApproximationPolicy::Bounded(
            crate::rules::SearchLimits {
                max_labels_per_node: Some(32),
                ..Default::default()
            },
        )),
    );
    assert!(outcome
        .completeness
        .reasons()
        .contains(&crate::rules::LimitReached::LabelsPerNodeLimit));
    let front = outcome.value;
    // Sanity: the search reached T.
    assert!(!front.is_empty(), "front should reach T");

    let created = TOK_CREATED.with(|c| c.get());
    let peak = TOK_PEAK.with(|c| c.get());
    // Many labels were created (≥ the N hub edges).
    assert!(
        created >= n as i64,
        "expected many token instances created, got {created}"
    );
    // Eviction frees labels: peak live is strictly below total created. With the bug
    // (arena pins evicted labels) peak would equal created; the margin here is large
    // (peak is bounded by ~32 per live node, created scales with N) so this is not
    // flaky.
    assert!(
        peak < created,
        "arena must free evicted labels: peak {peak} should be < created {created}"
    );

    // The retained tokens are bounded by the live bag entries, not by N. Concretely, far
    // fewer than the total are still live once the search completes.
    drop(front);
    let live_after = TOK_LIVE.with(|c| c.get());
    assert!(
        live_after < created,
        "retained tokens {live_after} must be bounded well below total {created}"
    );
}

#[test]
fn test_exact_dfs_releases_completed_prefixes() {
    TOK_LIVE.with(|c| c.set(0));
    TOK_PEAK.with(|c| c.set(0));
    TOK_CREATED.with(|c| c.set(0));

    let n = 200;
    let mut edges = Vec::new();
    for _ in 0..n {
        edges.push((
            "S",
            "M",
            growth_edge(vec![("c", Expr::Const(1.0)), ("s", Expr::Const(1.0))]),
        ));
    }
    edges.push((
        "M",
        "T",
        growth_edge(vec![("c", Expr::Var("c")), ("s", Expr::Var("s"))]),
    ));
    let graph = ReductionGraph::from_test_edges(&["S", "M", "T"], &edges);
    let empty = BTreeMap::new();
    let outcome = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        TokenLabel {
            c: 0.0,
            s: 0.0,
            _tok: Rc::new(DropToken::new()),
        },
        crate::rules::SearchMode::Exact,
    );

    assert_eq!(outcome.stats.generated_states, 1 + 2 * n);
    assert_eq!(outcome.stats.peak_labels_per_node, 1);
    assert_eq!(outcome.value.len(), 1);
    let created = TOK_CREATED.with(|c| c.get());
    let peak = TOK_PEAK.with(|c| c.get());
    assert!(
        peak * 10 < created,
        "exact DFS should release branch prefixes: peak {peak}, created {created}"
    );
}
