//! Tests for the Pareto label-setting search (`src/rules/pareto.rs`) and its two label
//! domains. Covers:
//! - The measured concrete-instance label (issue #788 known-answer, OOM pre-flight guard).
//! - The generic kernel's correctness on a hand-built diamond (negative control): a
//!   scalar-cost path selection commits to the wrong prefix, while the Pareto search
//!   returns the path with the strictly-better final measured size.

use super::*;
use crate::expr::Expr;
use crate::growth::Growth;
use crate::models::graph::{HamiltonianCircuit, HighlyConnectedDeletion};
use crate::rules::cost::CustomCost;
use crate::rules::pareto::{GrowthLabel, PathLabel, ReductionEdge};
use crate::rules::registry::{EdgeCapabilities, ReductionOverhead};
use crate::rules::{ReductionGraph, ReductionMode, DEFAULT_SIZE_BUDGET};
use crate::topology::SimpleGraph;
use crate::types::ProblemSize;
use std::any::Any;
use std::collections::BTreeMap;
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
// magnitude 4). Scalar branch-and-bound would let the cheaper path A complete first
// and then prune B (cost 4 ≥ 3), silently dropping a Pareto-optimal path. This is
// the case the equal-magnitude negative control above does NOT catch; it passes only
// because `GrowthLabel` opts out of branch-and-bound (`BRANCH_AND_BOUND = false`) and
// relies on exact dominance pruning.
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
