//! Tests for the Pareto label-setting search (`src/rules/pareto.rs`) and its two label
//! domains. Covers:
//! - The measured concrete-instance label (issue #788 known-answer, OOM pre-flight guard).
//! - The generic kernel's correctness on a hand-built diamond (negative control): a
//!   scalar-cost path selection commits to the wrong prefix, while the Pareto search
//!   returns the path with the strictly-better final measured size.

use super::*;
use crate::expr::Expr;
use crate::models::graph::{HamiltonianCircuit, HighlyConnectedDeletion};
use crate::rules::cost::CustomCost;
use crate::rules::pareto::{PathLabel, ReductionEdge};
use crate::rules::registry::{EdgeCapabilities, ReductionOverhead};
use crate::rules::{ReductionGraph, ReductionMode, DEFAULT_SIZE_BUDGET};
use crate::topology::SimpleGraph;
use crate::types::ProblemSize;
use std::any::Any;
use std::time::Instant;

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
            DEFAULT_SIZE_BUDGET,
            false,
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
// Verification 2: OOM pre-flight guard is real.
// ---------------------------------------------------------------------------

/// Routing a 64-vertex instance through the `2^num_vertices` overhead edge
/// (`highlyconnecteddeletion_ilp`) must be refused by the symbolic pre-flight guard
/// *before* the exponential construction is ever started: the search completes near
/// instantly and returns no in-budget path (the sole HCD → ILP edge is pruned).
///
/// The instance is a dense 64-vertex graph on purpose — if the guard were removed, the
/// reduction would enumerate ~2^64 feasible clusters and exhaust memory. Because guard 1
/// evaluates the formula (`2^64 ≫ budget`) and skips without executing, the test is safe.
#[test]
fn test_oom_preflight_guard_highlyconnecteddeletion() {
    // Dense 64-vertex graph (complete graph K_64): cheap to build, catastrophic to reduce.
    let n = 64;
    let mut edges = Vec::new();
    for u in 0..n {
        for v in (u + 1)..n {
            edges.push((u, v));
        }
    }
    let hcd = HighlyConnectedDeletion::new(SimpleGraph::new(n, edges));
    let graph = ReductionGraph::new();
    let variant = ReductionGraph::variant_to_map(&[("graph", "SimpleGraph")]);

    let start = Instant::now();
    let result = graph.find_measured_best_path_to_name(
        "HighlyConnectedDeletion",
        &variant,
        "ILP",
        ReductionMode::Witness,
        &hcd as &dyn Any,
        DEFAULT_SIZE_BUDGET,
        false,
    );
    let elapsed = start.elapsed();

    // The only HCD -> ILP path is the 2^num_vertices edge; it is pre-flight-pruned.
    assert!(
        result.is_none(),
        "the 2^num_vertices construction must be refused, not selected"
    );
    // Structural proof the exponential enumeration was never started: it finishes fast.
    assert!(
        elapsed.as_secs_f64() < 1.0,
        "search must complete in < 1s (never executes the exponential edge); took {:?}",
        elapsed
    );
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
