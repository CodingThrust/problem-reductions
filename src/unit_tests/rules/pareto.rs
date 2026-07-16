//! Tests for the Pareto label-setting search (`src/rules/pareto.rs`) and its two label
//! domains. Covers:
//! - The measured concrete-instance search (issue #788 known-answer and budget semantics).
//! - The generic kernel's correctness on a hand-built diamond (negative control): a
//!   scalar-cost path selection commits to the wrong prefix, while the Pareto search
//!   returns the path with the strictly-better final measured size.

use super::*;
use crate::expr::Expr;
use crate::growth::Growth;
use crate::models::algebraic::{ObjectiveSense, ILP};
use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::graph::HamiltonianCircuit;
use crate::rules::cost::CustomCost;
use crate::rules::pareto::{GrowthLabel, PathLabel, ReductionEdge};
use crate::rules::registry::{EdgeCapabilities, ReductionOverhead};
use crate::rules::traits::DynReductionResult;
use crate::rules::{ReductionAutoCast, ReductionGraph, ReductionMode};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{Or, ProblemSize};
use std::any::Any;
use std::cell::Cell;
use std::collections::BTreeMap;
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
        capabilities: EdgeCapabilities::witness_only(),
    }
}

// ---------------------------------------------------------------------------
// Verification 1: issue #788 known-answer check.
// ---------------------------------------------------------------------------

/// The prism (triangular-prism) graph from issue #788: 6 vertices, 9 edges.
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

/// #788: the measured Pareto search selects the path whose *measured* final ILP size is
/// smallest.
///
/// The literal reduction chain quoted in issue #788 (HC → HP → ConsecutiveOnesSubmatrix →
/// ILP, total 60) no longer exists on the current reduction graph. The *current* measured
/// optimum is HC → LongestCircuit → ILP<bool> with a measured total of 232
/// (num_constraints=127, num_vars=105); the next candidates are RuralPostman → ILP<i32>
/// (366) and TravelingSalesman → ILP<bool> (768). This test pins the measured optimum so
/// the selector is proven to rank by *measured* final size, not by step count or formula.
#[test]
fn test_hamiltoniancircuit_to_ilp_measured_optimum_788() {
    let hc = prism_hamiltonian_circuit();
    let graph = ReductionGraph::new();
    let variant = ReductionGraph::variant_to_map(&[("graph", "SimpleGraph")]);

    let measured = graph
        .find_measured_best_path_to_name(
            "HamiltonianCircuit",
            &variant,
            "ILP",
            ReductionMode::Witness,
            &hc as &dyn Any,
            1_000,
        )
        .expect("a measured witness path from HamiltonianCircuit to ILP");

    // Measured final ILP size is the current-graph optimum.
    assert_eq!(
        measured.size.total(),
        232,
        "measured optimum should be 232, got {:?}",
        measured.size
    );
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

// ---------------------------------------------------------------------------
// Verification 2: measured search does not discard equal-size concrete states.
// ---------------------------------------------------------------------------

#[test]
fn test_measured_search_keeps_equal_size_structure_dependent_instances() {
    let graph = ReductionGraph::from_test_edges(
        &[
            "MeasuredSource",
            "MeasuredBranchA",
            "MeasuredBranchB",
            "Satisfiability",
            "ILP",
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

    let bad_sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let good_sat = Satisfiability::new(1, vec![CNFClause::new(vec![-1])]);
    assert_eq!(
        ReductionGraph::compute_source_size("Satisfiability", &bad_sat),
        ReductionGraph::compute_source_size("Satisfiability", &good_sat),
        "the two structurally different hub instances must have identical measured sizes",
    );

    let measured = graph
        .find_measured_best_path(
            "MeasuredSource",
            &empty,
            "ILP",
            &empty,
            ReductionMode::Witness,
            &source,
            1_000,
        )
        .expect("the structure-dependent small continuation must survive");

    assert_eq!(
        measured.path.type_names(),
        ["MeasuredSource", "MeasuredBranchB", "Satisfiability", "ILP",],
    );
    assert_eq!(measured.size.total(), 1);
}

#[test]
fn test_asymptotic_overhead_is_not_a_concrete_budget_guard() {
    let graph = ReductionGraph::from_test_edges(
        &["MeasuredSource", "ILP"],
        &[(
            "MeasuredSource",
            "ILP",
            measured_edge(measured_source_to_small_ilp, 1_000_000.0),
        )],
    );
    let empty = BTreeMap::new();
    let source = MeasuredSource;

    let measured = graph
        .find_measured_best_path(
            "MeasuredSource",
            &empty,
            "ILP",
            &empty,
            ReductionMode::Witness,
            &source,
            1,
        )
        .expect("a loose asymptotic expression must not prune an actually in-budget target");

    assert_eq!(measured.size.total(), 1);
}

// ---------------------------------------------------------------------------
// Verification 4: negative control on a hand-built diamond.
// ---------------------------------------------------------------------------

/// A test label whose objective is the *final* measured size `s`, while carrying a
/// separate accumulated step cost `c`. Dominance is componentwise Pareto over `(c, s)`,
/// so two labels that trade off `c` against `s` are incomparable and both survive — the
/// exact structure a scalar Dijkstra collapses (keeping only the min-`c` label, and thus
/// its `s`).
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

    fn dominates(&self, other: &Self) -> bool {
        self.c <= other.c && self.s <= other.s
    }

    fn cost(&self) -> f64 {
        self.s
    }
}

fn diamond_edge(c: f64, s: Expr) -> ReductionEdgeData {
    ReductionEdgeData {
        overhead: ReductionOverhead::new(vec![("c", Expr::Const(c)), ("s", s)]),
        reduce_fn: None,
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
    }
}

/// Negative control: P1 (S→M→T) has the lower first-edge cost but a larger measured
/// intermediate size at M; P2 (S→P→M→T) has a higher first-edge cost but a strictly
/// smaller final measured size. A scalar-cost path selection (`find_cheapest_path` over
/// the additive step cost) commits to P1's prefix at M and returns P1; the measured
/// Pareto search keeps both routes into M (they are incomparable) and returns P2.
#[test]
fn test_negative_control_diamond_pareto_beats_scalar() {
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

    // (a) Scalar-cost selection (minimize additive step cost `c`) commits to P1.
    let scalar = graph
        .find_cheapest_path(
            "S",
            &empty,
            "T",
            &empty,
            &ProblemSize::new(vec![]),
            &CustomCost(|oh: &ReductionOverhead, sz: &ProblemSize| {
                oh.get("c").map(|e| e.eval(sz)).unwrap_or(0.0)
            }),
        )
        .expect("scalar path S -> T");
    assert_eq!(
        scalar.type_names(),
        vec!["S", "M", "T"],
        "scalar cost selection should commit to the cheap-prefix P1"
    );

    // (b) The measured Pareto search returns P2 (strictly smaller final size).
    let initial = DiamondLabel { c: 0.0, s: 0.0 };
    let front = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial,
        false,
    );
    assert!(!front.is_empty(), "front should reach T");
    let (best_path, best_label) = &front[0];
    assert_eq!(
        best_path.type_names(),
        vec!["S", "P", "M", "T"],
        "Pareto search should return the better-final-size P2"
    );
    assert_eq!(best_label.cost(), 6.0, "P2's final measured size is 6");
}

/// The `exhaustive` flag disables only the heuristic componentwise-dominance guard; the
/// front still contains the true optimum. On the diamond, both routes into M survive
/// regardless, so the answer is unchanged.
#[test]
fn test_diamond_exhaustive_matches_pruned() {
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
    let front = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        DiamondLabel { c: 0.0, s: 0.0 },
        true,
    );
    assert_eq!(front[0].0.type_names(), vec!["S", "P", "M", "T"]);
    assert_eq!(front[0].1.cost(), 6.0);
}

// ---------------------------------------------------------------------------
// GrowthLabel (asymptotic, instance-free) domain — issue #1080 / design M3/F3a.
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
        reduce_fn: None,
        reduce_aggregate_fn: None,
        capabilities: EdgeCapabilities::witness_only(),
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
        capabilities: EdgeCapabilities::witness_only(),
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
        capabilities: EdgeCapabilities::witness_only(),
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
        capabilities: EdgeCapabilities::witness_only(),
        target_name: "T",
        target_variant: &tv,
    };
    let next = label.extend(&redge).expect("extend");
    assert_eq!(field_big_o(&next, "out1"), "?");
    assert_eq!(field_big_o(&next, "out2"), "n^2");
}

/// A label with an `Unknown` field is dominated by any fully-known label, and never
/// dominates one — undecidable paths rank last.
#[test]
fn test_growth_label_unknown_ranks_last() {
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
    // Known is strictly better on field b (n^0? no: bounded vs Unknown) ⇒ known dominates.
    assert!(known.dominates(&with_unknown));
    assert!(!with_unknown.dominates(&known));
}

/// Componentwise search-sense dominance: `self` dominates `other` iff it grows no
/// faster on every field and strictly slower on at least one.
#[test]
fn test_growth_label_dominance_partial_order() {
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
    assert!(a.dominates(&b));
    assert!(!b.dominates(&a));
    // Reflexivity is *not* strict dominance: equal labels do not dominate each other.
    assert!(!a.dominates(&a.clone()));

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
    assert!(!c.dominates(&d));
    assert!(!d.dominates(&c));
}

/// **Negative control (issue #1080):** two S→T paths whose composed growths are
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
    let front = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial,
        false,
    );

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
// does NOT catch; it passes because the kernel prunes by exact dominance only, never
// by the scalar `cost`.
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

    let front = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        GrowthLabel::source(&["n", "m"]),
        false,
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
            ("S→B→T".to_string(), "v=n e=m^3".to_string()),
        ],
        "both incomparable paths must survive despite different scalar magnitudes",
    );
}

/// Isotonicity of `extend` (design invariant): if `A` dominates `B`, then
/// `extend(A, e)` dominates `extend(B, e)` for the same edge — the correctness
/// condition for the kernel's dominance pruning.
#[test]
fn test_growth_label_extend_isotone() {
    // A = (n, m) dominates B = (n^2, m^2) componentwise.
    let a = GrowthLabel::source(&["n", "m"]);
    let b = GrowthLabel::from_fields({
        let mut mm = BTreeMap::new();
        mm.insert("n", Growth::from_expr(&powk("n", 2.0)));
        mm.insert("m", Growth::from_expr(&powk("m", 2.0)));
        mm
    });
    assert!(a.dominates(&b));

    let tv = BTreeMap::new();
    // A monotone overhead in both fields.
    for overhead in [
        growth_edge(vec![("x", Expr::Var("n") * Expr::Var("m"))]),
        growth_edge(vec![("x", powk("n", 3.0)), ("y", Expr::Var("m"))]),
    ] {
        let redge = ReductionEdge {
            overhead: &overhead.overhead,
            reduce_fn: None,
            capabilities: EdgeCapabilities::witness_only(),
            target_name: "T",
            target_variant: &tv,
        };
        let ea = a.extend(&redge).unwrap();
        let eb = b.extend(&redge).unwrap();
        // A ⪰ B ⇒ extend(A) ⪰ extend(B) (dominates-or-equal). Equality is possible
        // when the overhead collapses the difference, so accept dominate-or-equal.
        assert!(
            ea.dominates(&eb) || ea == eb,
            "isotonicity violated: {ea:?} vs {eb:?}"
        );
    }
}

/// `asymptotic_front` reports **one representative per distinct growth vector**, not
/// one per route. On the real graph, `MinimumVertexCover → ILP` has dozens of
/// syntactically distinct reduction chains that compose to only a handful of Big-O
/// profiles; the front must (a) contain no two entries with identical growth vectors
/// and (b) collapse to that small handful — while the raw kernel front (same search,
/// no dedup) still holds the many redundant routes.
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

    let front = graph.asymptotic_front(
        "MinimumVertexCover",
        &src_v,
        "ILP",
        &dst_v,
        ReductionMode::Witness,
    );
    assert!(!front.is_empty(), "MVC -> ILP must have a path");

    // (a) No two front entries share a growth vector (GrowthLabel PartialEq).
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
    // (b) A proper Pareto front is a small handful, not the dozens of redundant routes.
    assert!(
        (1..=4).contains(&front.len()),
        "expected 1..=4 distinct growth vectors, got {}",
        front.len()
    );

    // The dedup genuinely collapsed routes: the raw kernel front (same search, no
    // dedup) is strictly larger and does contain repeated growth vectors.
    let src_fields = graph.size_field_names("MinimumVertexCover");
    let raw = graph.pareto_search_by_name(
        "MinimumVertexCover",
        &src_v,
        "ILP",
        &dst_v,
        ReductionMode::Witness,
        GrowthLabel::source(&src_fields),
        false,
    );
    assert!(
        raw.len() > front.len(),
        "dedup should collapse redundant routes: raw {} vs deduped {}",
        raw.len(),
        front.len()
    );
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

    let front = graph.asymptotic_front(
        "MinimumFeedbackVertexSet",
        &src_v,
        "ILP",
        &dst_v,
        ReductionMode::Witness,
    );

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
// Fix A: the kernel prunes by dominance only — never (unsound) branch-and-bound.
// ---------------------------------------------------------------------------

/// A test label whose `cost` is the label's current absolute value — a value a late edge
/// can *shrink* below an already-completed route's final value. It verifies that the
/// generic kernel does not silently add scalar branch-and-bound.
#[derive(Clone)]
struct ShrinkLabel {
    v: f64,
}

impl PathLabel for ShrinkLabel {
    fn extend(&self, edge: &ReductionEdge) -> Option<Self> {
        // The edge sets a new absolute value (`v`), which may be smaller than the current.
        let z = ProblemSize::new(vec![]);
        let v = edge.overhead.get("v").map(|e| e.eval(&z)).unwrap_or(self.v);
        Some(ShrinkLabel { v })
    }

    fn dominates(&self, other: &Self) -> bool {
        self.v <= other.v
    }

    fn cost(&self) -> f64 {
        self.v
    }
}

/// Kernel regression for Fix A: a route that *shrinks late* (its intermediate cost 100 is
/// higher than a rival route that completes early at 50, but a final edge drops it to 10)
/// must survive to the front. A kernel that applied branch-and-bound would prune the
/// intermediate node (100 ≥ best-so-far 50) and silently drop the true optimum. Because
/// the kernel prunes by dominance only, the shrink-late route reaches the front even under
/// `exhaustive = true` (which disables only the dominance guard).
#[test]
fn test_kernel_keeps_shrink_late_route_dominance_only() {
    let empty = std::collections::BTreeMap::new();
    let graph = ReductionGraph::from_test_edges(
        &["S", "A", "T"],
        &[
            // S -> T: completes early with final value 50.
            ("S", "T", growth_edge(vec![("v", Expr::Const(50.0))])),
            // S -> A: intermediate value 100 (would trip a B&B bound of 50).
            ("S", "A", growth_edge(vec![("v", Expr::Const(100.0))])),
            // A -> T: shrinks the value to 10 (globally best).
            ("A", "T", growth_edge(vec![("v", Expr::Const(10.0))])),
        ],
    );

    let front = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        ShrinkLabel { v: 0.0 },
        true,
    );

    // The shrink-late route S -> A -> T (final value 10) must be present in the front.
    let shrink_late = front
        .iter()
        .find(|(p, _)| p.type_names() == ["S", "A", "T"])
        .expect("shrink-late route S -> A -> T must survive without branch-and-bound");
    assert_eq!(
        shrink_late.1.cost(),
        10.0,
        "the shrink-late route finishes at the global optimum value 10"
    );
    // The kernel's best (lowest cost) front element is that shrink-late route.
    assert_eq!(front[0].0.type_names(), ["S", "A", "T"]);
    assert_eq!(front[0].1.cost(), 10.0);
}

// ---------------------------------------------------------------------------
// Fix B: CostLabel dominance is componentwise over (cost, size).
// ---------------------------------------------------------------------------

/// Fix B regression: an edge cost that DEPENDS on the carried size makes a cheaper-so-far
/// prefix with a *larger* intermediate size a trap — a scalar `cost <= other.cost`
/// dominance would evict the costlier-but-smaller prefix whose continuation is globally
/// cheapest. With componentwise `(cost, size)` dominance both prefixes survive at the hub
/// and `find_cheapest_path` returns the globally optimal route.
#[test]
fn test_cost_label_path_dependent_dominance() {
    let empty = std::collections::BTreeMap::new();
    // Edges carry `c` (base edge cost), `wf` (weight on the size-dependent term) and `w`
    // (the tracked size field). The cost function is `c + wf * current_w`, so the M -> T
    // edge's cost is exactly the size `w` accumulated at M.
    let graph = ReductionGraph::from_test_edges(
        &["S", "M", "P", "T"],
        &[
            // S -> M: cheap prefix (c = 1) but produces a LARGE intermediate size w = 100.
            (
                "S",
                "M",
                growth_edge(vec![
                    ("c", Expr::Const(1.0)),
                    ("wf", Expr::Const(0.0)),
                    ("w", Expr::Const(100.0)),
                ]),
            ),
            // S -> P: pricier prefix (c = 3) but a SMALL size w = 1.
            (
                "S",
                "P",
                growth_edge(vec![
                    ("c", Expr::Const(3.0)),
                    ("wf", Expr::Const(0.0)),
                    ("w", Expr::Const(1.0)),
                ]),
            ),
            // P -> M: cheap (c = 1), keeps the small size w = 1.
            (
                "P",
                "M",
                growth_edge(vec![
                    ("c", Expr::Const(1.0)),
                    ("wf", Expr::Const(0.0)),
                    ("w", Expr::Const(1.0)),
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

    // Cost function: c + wf * current_w. Depends on the carried size, so the two prefixes
    // into M are incomparable and must both be kept.
    let cost_fn = CustomCost(|oh: &ReductionOverhead, sz: &ProblemSize| {
        let c = oh.get("c").map(|e| e.eval(sz)).unwrap_or(0.0);
        let wf = oh.get("wf").map(|e| e.eval(sz)).unwrap_or(0.0);
        c + wf * sz.get("w").unwrap_or(0) as f64
    });

    let best = graph
        .find_cheapest_path(
            "S",
            &empty,
            "T",
            &empty,
            &ProblemSize::new(vec![("w", 0)]),
            &cost_fn,
        )
        .expect("cheapest path S -> T");

    // Globally cheapest: S -> P -> M -> T (total 3 + 1 + 1 = 5), NOT the cheap-prefix trap
    // S -> M -> T (total 1 + 100 = 101). A scalar-dominance CostLabel would evict the
    // small-w prefix at M and return the S -> M -> T trap.
    assert_eq!(
        best.type_names(),
        vec!["S", "P", "M", "T"],
        "componentwise (cost, size) dominance must keep the globally optimal small-w prefix"
    );
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
        capabilities: EdgeCapabilities::witness_only(),
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

/// A label carrying an `Rc<DropToken>` and a two-component `(c, s)` value. The engineered
/// `(c, s)` pairs are pairwise incomparable, so no label evicts another by dominance and
/// the per-node bag grows until the cap truncates it — exercising the truncation free path.
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

    fn dominates(&self, other: &Self) -> bool {
        self.c <= other.c && self.s <= other.s
    }

    fn cost(&self) -> f64 {
        self.c
    }
}

/// Fix D regression: drive the kernel on a graph that generates far more labels at one hub
/// than `BAG_CAP`, all incomparable so the bag truncates repeatedly. Because evicted /
/// truncated arena entries free their labels immediately, the *peak* number of live
/// `DropToken` instances stays well below the *total* ever created. If the arena pinned
/// evicted labels (the bug), peak would equal total.
#[test]
fn test_arena_frees_evicted_labels_bounds_live_memory() {
    TOK_LIVE.with(|c| c.set(0));
    TOK_PEAK.with(|c| c.set(0));
    TOK_CREATED.with(|c| c.set(0));

    // One hub M fed by N ≫ BAG_CAP parallel S -> M edges with pairwise-incomparable
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
    let front = graph.pareto_search_by_name(
        "S",
        &empty,
        "T",
        &empty,
        ReductionMode::Witness,
        initial,
        false,
    );
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
    // (peak is bounded by ~BAG_CAP per live node, created scales with N) so this is not
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
