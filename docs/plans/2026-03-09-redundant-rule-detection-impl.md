# Redundant Rule Detection — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Detect primitive reduction rules dominated by composite paths, using polynomial normalization and monomial-dominance comparison.

**Architecture:** Add `analysis.rs` module in `src/rules/` that normalizes overhead expressions into sum-of-monomials form, compares asymptotic growth via monomial dominance, validates path trustworthiness (excluding ILP→QUBO), and iterates all graph edges to find dominated rules. Integration test with feature-gated allow-list prevents regressions.

**Tech Stack:** Rust, existing `Expr`/`ReductionOverhead`/`ReductionGraph` infrastructure.

---

### Task 1: Create `analysis.rs` with polynomial normalization and comparison

**Files:**
- Create: `src/rules/analysis.rs`
- Modify: `src/rules/mod.rs:3` (add `pub mod analysis;`)
- Create: `src/unit_tests/rules/analysis.rs`

**Step 1: Write failing tests in `src/unit_tests/rules/analysis.rs`**

Create `src/unit_tests/rules/analysis.rs`:

```rust
use crate::expr::Expr;
use crate::rules::analysis::{compare_overhead, ComparisonStatus};
use crate::rules::registry::ReductionOverhead;

// --- Polynomial normalization + comparison tests ---

#[test]
fn test_compare_overhead_equal() {
    let a = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let b = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&a, &b), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_composite_smaller_degree() {
    // primitive: num_vars = n^2, composite: num_vars = n → dominated
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    )]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_composite_worse() {
    // primitive: num_vars = n, composite: num_vars = n^2 → not dominated
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_multi_field_mixed() {
    // One field better, one worse → not dominated
    let prim = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("n")),
        (
            "num_constraints",
            Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        ),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_vars", Expr::pow(Expr::Var("n"), Expr::Const(2.0))),
        ("num_constraints", Expr::Var("n")),
    ]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_no_common_fields() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![("num_spins", Expr::Var("n"))]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_unknown_exp() {
    // exp(n) can't be normalized → Unknown
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::Exp(Box::new(Expr::Var("n"))),
    )]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Unknown);
}

#[test]
fn test_compare_overhead_unknown_log() {
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::Log(Box::new(Expr::Var("n"))),
    )]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Unknown);
}

#[test]
fn test_compare_overhead_multivariate_product_vs_sum() {
    // n * m (degree 2) vs n + m (degree 1):
    // monomial n*m has exponents {n:1, m:1}
    // monomials n, m each have exponent 1 in one variable
    // n*m is NOT dominated by either n or m → composite is worse
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::add(Expr::Var("n"), Expr::Var("m")),
    )]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::mul(Expr::Var("n"), Expr::Var("m")),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_multivariate_product_vs_square() {
    // n * m (has m) vs n^2 (no m): incomparable
    // n*m monomial {n:1, m:1} — dominated by n^2 {n:2}?
    // exponent_n: 1 <= 2 ✓, exponent_m: 1 <= 0 ✗ → not dominated
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    )]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::mul(Expr::Var("n"), Expr::Var("m")),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_sum_vs_single_var() {
    // composite: n, primitive: n + m → composite ≤ primitive (n dominated by n)
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::add(Expr::Var("n"), Expr::Var("m")),
    )]);
    let comp = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_constant_factor() {
    // 3*n vs n → same asymptotic class → dominated (equal)
    let prim = ReductionOverhead::new(vec![("num_vars", Expr::Var("n"))]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::mul(Expr::Const(3.0), Expr::Var("n")),
    )]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}

#[test]
fn test_compare_overhead_polynomial_expansion() {
    // (n + m)^2 = n^2 + 2nm + m^2 (degree 2) vs n^3 (degree 3)
    // Each monomial of composite has total degree ≤ 2, primitive has degree 3
    // n^2 dominated by n^3? exponent_n: 2 ≤ 3 ✓ → yes
    // 2*n*m dominated by n^3? exponent_n: 1 ≤ 3 ✓, exponent_m: 1 ≤ 0 ✗ → no!
    // So composite is NOT dominated — (n+m)^2 can exceed n^3 when m is large
    let prim = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(Expr::Var("n"), Expr::Const(3.0)),
    )]);
    let comp = ReductionOverhead::new(vec![(
        "num_vars",
        Expr::pow(
            Expr::add(Expr::Var("n"), Expr::Var("m")),
            Expr::Const(2.0),
        ),
    )]);
    assert_eq!(
        compare_overhead(&prim, &comp),
        ComparisonStatus::NotDominated
    );
}

#[test]
fn test_compare_overhead_multi_field_all_smaller() {
    // Both fields: composite has smaller degree → dominated
    let prim = ReductionOverhead::new(vec![
        (
            "num_vars",
            Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        ),
        (
            "num_constraints",
            Expr::pow(Expr::Var("n"), Expr::Const(3.0)),
        ),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("n")),
        ("num_constraints", Expr::Var("n")),
    ]);
    assert_eq!(compare_overhead(&prim, &comp), ComparisonStatus::Dominated);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib test_compare_overhead -- --no-run 2>&1 | head -5`
Expected: compile error — `analysis` module not found.

**Step 3: Create `src/rules/analysis.rs`**

```rust
//! Analysis utilities for the reduction graph.
//!
//! Detects primitive reduction rules that are dominated by composite paths,
//! using polynomial normalization and monomial-dominance comparison.
//!
//! This analysis is **sound but incomplete**: it reports `Dominated` only when
//! the symbolic comparison is trustworthy, and `Unknown` when metadata is too
//! weak to compare safely.

use crate::expr::Expr;
use crate::rules::graph::{ReductionGraph, ReductionPath};
use crate::rules::registry::ReductionOverhead;
use std::collections::BTreeMap;

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

// ────────── Polynomial normalization ──────────

/// A monomial: coefficient × ∏(variable ^ exponent).
#[derive(Debug, Clone)]
struct Monomial {
    coeff: f64,
    /// Variable name → exponent. Only non-zero exponents stored.
    vars: BTreeMap<&'static str, f64>,
}

impl Monomial {
    fn constant(c: f64) -> Self {
        Self {
            coeff: c,
            vars: BTreeMap::new(),
        }
    }

    fn variable(name: &'static str) -> Self {
        let mut vars = BTreeMap::new();
        vars.insert(name, 1.0);
        Self { coeff: 1.0, vars }
    }

    /// Multiply two monomials.
    fn mul(&self, other: &Monomial) -> Monomial {
        let coeff = self.coeff * other.coeff;
        let mut vars = self.vars.clone();
        for (&v, &e) in &other.vars {
            *vars.entry(v).or_insert(0.0) += e;
        }
        Monomial { coeff, vars }
    }
}

/// A polynomial (sum of monomials) in normal form.
#[derive(Debug, Clone)]
struct NormalizedPoly {
    terms: Vec<Monomial>,
}

impl NormalizedPoly {
    fn add(mut self, other: NormalizedPoly) -> NormalizedPoly {
        self.terms.extend(other.terms);
        self
    }

    fn mul(&self, other: &NormalizedPoly) -> NormalizedPoly {
        let mut terms = Vec::new();
        for a in &self.terms {
            for b in &other.terms {
                terms.push(a.mul(b));
            }
        }
        NormalizedPoly { terms }
    }

    /// True if any monomial has a negative coefficient.
    fn has_negative_coefficients(&self) -> bool {
        self.terms.iter().any(|m| m.coeff < -1e-15)
    }
}

/// Normalize an expression into a sum of monomials.
///
/// Supports: constants, variables, addition, multiplication,
/// and powers with non-negative constant exponents.
/// Returns `Err` for exp, log, sqrt, division, and negative exponents.
fn normalize(expr: &Expr) -> Result<NormalizedPoly, String> {
    match expr {
        Expr::Const(c) => Ok(NormalizedPoly {
            terms: vec![Monomial::constant(*c)],
        }),
        Expr::Var(v) => Ok(NormalizedPoly {
            terms: vec![Monomial::variable(v)],
        }),
        Expr::Add(a, b) => {
            let pa = normalize(a)?;
            let pb = normalize(b)?;
            Ok(pa.add(pb))
        }
        Expr::Mul(a, b) => {
            let pa = normalize(a)?;
            let pb = normalize(b)?;
            Ok(pa.mul(&pb))
        }
        Expr::Pow(base, exp) => {
            if let Expr::Const(c) = exp.as_ref() {
                if *c < 0.0 {
                    return Err(format!("negative exponent: {c}"));
                }
                let pb = normalize(base)?;
                // Single monomial: multiply exponents
                if pb.terms.len() == 1 {
                    let m = &pb.terms[0];
                    let coeff = m.coeff.powf(*c);
                    let vars: BTreeMap<_, _> =
                        m.vars.iter().map(|(&v, &e)| (v, e * c)).collect();
                    return Ok(NormalizedPoly {
                        terms: vec![Monomial { coeff, vars }],
                    });
                }
                // Multi-term polynomial raised to non-negative integer power
                let n = *c as usize;
                if c.fract().abs() < 1e-10 {
                    if n == 0 {
                        return Ok(NormalizedPoly {
                            terms: vec![Monomial::constant(1.0)],
                        });
                    }
                    let mut result = pb.clone();
                    for _ in 1..n {
                        result = result.mul(&pb);
                    }
                    return Ok(result);
                }
                Err(format!(
                    "non-integer power of multi-term polynomial: ({base})^{c}"
                ))
            } else {
                Err(format!("variable exponent: ({base})^({exp})"))
            }
        }
        Expr::Exp(_) => Err("exp() not supported".into()),
        Expr::Log(_) => Err("log() not supported".into()),
        Expr::Sqrt(_) => Err("sqrt() not supported".into()),
    }
}

// ────────── Monomial-dominance comparison ──────────

/// Check if monomial `small` is asymptotically dominated by monomial `big`.
///
/// True iff for every variable in `small`, `big` has at least as large an exponent.
/// This means `small` grows no faster than `big` as all variables → ∞.
fn monomial_dominated_by(small: &Monomial, big: &Monomial) -> bool {
    for (&var, &exp_small) in &small.vars {
        let exp_big = big.vars.get(var).copied().unwrap_or(0.0);
        if exp_small > exp_big + 1e-10 {
            return false;
        }
    }
    true
}

/// Check if polynomial `a` is asymptotically ≤ polynomial `b`.
///
/// True iff every positive-coefficient monomial in `a` is dominated by
/// some positive-coefficient monomial in `b`.
fn poly_leq(a: &NormalizedPoly, b: &NormalizedPoly) -> bool {
    let b_positive: Vec<&Monomial> = b
        .terms
        .iter()
        .filter(|m| m.coeff > 1e-15)
        .collect();

    for a_term in &a.terms {
        if a_term.coeff <= 1e-15 {
            continue; // zero or negative — can only make `a` smaller
        }
        let dominated = b_positive
            .iter()
            .any(|b_term| monomial_dominated_by(a_term, b_term));
        if !dominated {
            return false;
        }
    }
    true
}

// ────────── Overhead comparison ──────────

/// Compare two overheads across all common fields.
///
/// Returns `Dominated` if composite ≤ primitive on all common fields.
/// Returns `NotDominated` if composite is worse on any common field.
/// Returns `Unknown` if any common field's expressions cannot be normalized
/// or contain negative coefficients.
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

        let pn = match normalize(prim_expr) {
            Ok(p) => p,
            Err(_) => return ComparisonStatus::Unknown,
        };
        let cn = match normalize(comp_expr) {
            Ok(p) => p,
            Err(_) => return ComparisonStatus::Unknown,
        };

        // Reject expressions with negative coefficients
        if pn.has_negative_coefficients() || cn.has_negative_coefficients() {
            return ComparisonStatus::Unknown;
        }

        // Check: composite ≤ primitive on this field
        if !poly_leq(&cn, &pn) {
            return ComparisonStatus::NotDominated;
        }
    }

    if any_common {
        ComparisonStatus::Dominated
    } else {
        ComparisonStatus::NotDominated
    }
}

// ────────── Trust model ──────────

/// Known reduction edges whose symbolic overhead is incomplete.
///
/// ILP → QUBO: the implementation adds slack bits per inequality constraint
/// whose count depends on coefficient magnitudes, but the symbolic overhead
/// only records `num_vars = num_vars`.
const UNTRUSTED_EDGES: &[(&str, &str)] = &[("ILP", "QUBO")];

/// Check whether every edge in a path has trustworthy symbolic overhead.
fn path_is_symbolically_trustworthy(path: &ReductionPath) -> Result<(), String> {
    for window in path.steps.windows(2) {
        let src = window[0].name.as_str();
        let dst = window[1].name.as_str();
        for &(u_src, u_dst) in UNTRUSTED_EDGES {
            if src == u_src && dst == u_dst {
                return Err(format!(
                    "path contains untrustworthy edge: {src} → {dst}"
                ));
            }
        }
    }
    Ok(())
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
pub fn find_dominated_rules(
    graph: &ReductionGraph,
) -> (Vec<DominatedRule>, Vec<UnknownComparison>) {
    let mut dominated = Vec::new();
    let mut unknown = Vec::new();

    for edge_info in all_edges(graph) {
        let paths = graph.find_all_paths(
            edge_info.source_name,
            &edge_info.source_variant,
            edge_info.target_name,
            &edge_info.target_variant,
        );

        let mut best_dominating: Option<(ReductionPath, ReductionOverhead, Vec<String>)> =
            None;

        for path in paths {
            if path.len() <= 1 {
                continue; // skip the direct edge itself
            }

            // Trust check
            if let Err(reason) = path_is_symbolically_trustworthy(&path) {
                unknown.push(UnknownComparison {
                    source_name: edge_info.source_name,
                    source_variant: edge_info.source_variant.clone(),
                    target_name: edge_info.target_name,
                    target_variant: edge_info.target_variant.clone(),
                    candidate_path: path,
                    reason,
                });
                continue;
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
        (a.source_name, a.target_name, a.dominating_path.len())
            .cmp(&(b.source_name, b.target_name, b.dominating_path.len()))
    });
    unknown.sort_by(|a, b| {
        (a.source_name, a.target_name).cmp(&(b.source_name, b.target_name))
    });

    (dominated, unknown)
}

/// Fields present in both overheads.
fn common_fields(a: &ReductionOverhead, b: &ReductionOverhead) -> Vec<String> {
    let b_fields: std::collections::HashSet<&str> =
        b.output_size.iter().map(|(n, _)| *n).collect();
    a.output_size
        .iter()
        .filter_map(|(f, _)| b_fields.contains(f).then(|| f.to_string()))
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

#[cfg(test)]
#[path = "../unit_tests/rules/analysis.rs"]
mod tests;
```

**Step 4: Register the module in `src/rules/mod.rs`**

After `pub mod cost;` (line 3), add:

```rust
pub mod analysis;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test --lib test_compare_overhead`
Expected: all `test_compare_overhead_*` tests PASS.

**Step 6: Commit**

```bash
git add src/rules/analysis.rs src/rules/mod.rs src/unit_tests/rules/analysis.rs
git commit -m "feat: add polynomial comparison for overhead analysis (#193)"
```

---

### Task 2: Add `find_dominated_rules` integration test

**Files:**
- Modify: `src/unit_tests/rules/analysis.rs` (append integration tests)

**Step 1: Append integration tests to `src/unit_tests/rules/analysis.rs`**

```rust
use crate::rules::analysis::{find_dominated_rules, ComparisonStatus, DominatedRule};
use crate::rules::graph::ReductionGraph;

#[test]
fn test_find_dominated_rules_returns_known_set() {
    let graph = ReductionGraph::new();
    let (dominated, unknown) = find_dominated_rules(&graph);

    // Print for debugging
    eprintln!("Dominated rules ({}):", dominated.len());
    for rule in &dominated {
        let path_str: String = rule
            .dominating_path
            .steps
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" → ");
        eprintln!(
            "  {} → {} dominated by [{}]",
            rule.source_name, rule.target_name, path_str,
        );
    }
    eprintln!("\nUnknown comparisons ({}):", unknown.len());
    for u in &unknown {
        eprintln!(
            "  {} → {}: {}",
            u.source_name, u.target_name, u.reason,
        );
    }

    // ── Allow-list of expected dominated rules ──
    // Keyed by (source_name, target_name).
    // This list must be updated when new reductions are added.
    let allowed: std::collections::HashSet<(&str, &str)> = [
        // Genuine redundancies (composite path is same or better overhead)
        ("MaximumIndependentSet", "QUBO"),
        ("MinimumVertexCover", "QUBO"),
        // Cast-composed (variant casts compose to equivalent overhead)
        ("KSatisfiability", "Satisfiability"),
        ("MaximumIndependentSet", "MaximumIndependentSet"),
    ]
    .into_iter()
    .collect();

    // Feature-gated: ILP reductions add more dominated rules
    #[cfg(feature = "ilp-solver")]
    let allowed = {
        let mut a = allowed;
        a.insert(("MaximumIndependentSet", "ILP"));
        a.insert(("MaximumSetPacking", "ILP"));
        a.insert(("MinimumVertexCover", "ILP"));
        a.insert(("MinimumVertexCover", "QUBO")); // may find better path via ILP
        a
    };

    // Check: no unexpected dominated rules
    for rule in &dominated {
        let key = (rule.source_name, rule.target_name);
        assert!(
            allowed.contains(&key),
            "Unexpected dominated rule: {} → {} (dominated by {})",
            rule.source_name,
            rule.target_name,
            rule.dominating_path
                .steps
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" → "),
        );
    }

    // Check: no stale entries in allow-list
    let found: std::collections::HashSet<(&str, &str)> = dominated
        .iter()
        .map(|r| (r.source_name, r.target_name))
        .collect();
    for &key in &allowed {
        assert!(
            found.contains(&key),
            "Allow-list entry {:?} → {:?} is stale (no longer dominated)",
            key.0,
            key.1,
        );
    }
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_ilp_qubo_paths_are_unknown() {
    let graph = ReductionGraph::new();
    let (_, unknown) = find_dominated_rules(&graph);

    // Any path through ILP → QUBO should be reported as Unknown
    let ilp_qubo_unknowns: Vec<_> = unknown
        .iter()
        .filter(|u| u.reason.contains("ILP") && u.reason.contains("QUBO"))
        .collect();

    assert!(
        !ilp_qubo_unknowns.is_empty(),
        "Expected at least one Unknown comparison involving ILP → QUBO"
    );

    // Specifically, KColoring → QUBO via ILP should be Unknown, not Dominated
    let (dominated, _) = find_dominated_rules(&graph);
    let kcoloring_qubo_dominated = dominated
        .iter()
        .any(|r| r.source_name == "KColoring" && r.target_name == "QUBO");
    assert!(
        !kcoloring_qubo_dominated,
        "KColoring → QUBO should NOT be flagged as Dominated (path goes through ILP → QUBO)"
    );
}

#[test]
fn test_no_duplicate_primitive_rules_per_variant_pair() {
    use crate::rules::registry::ReductionEntry;
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    for entry in inventory::iter::<ReductionEntry> {
        let src_variant: BTreeMap<String, String> = entry
            .source_variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let dst_variant: BTreeMap<String, String> = entry
            .target_variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let key = (
            entry.source_name,
            src_variant,
            entry.target_name,
            dst_variant,
        );
        assert!(
            seen.insert(key.clone()),
            "Duplicate primitive rule: {} {:?} → {} {:?}",
            key.0,
            key.1,
            key.2,
            key.3,
        );
    }
}
```

**Step 2: Run tests**

Run: `cargo test --lib --features ilp-solver analysis -- --nocapture 2>&1`
Expected: tests pass. Examine stderr output to verify the dominated set matches expectations.

**Step 3: Adjust allow-list if needed**

The allow-list above is tentative. If the test fails because:
- An expected rule is NOT found: remove it from the allow-list
- An unexpected rule IS found: investigate, then add to the allow-list

Common adjustments:
- Some MVC → QUBO domination may only appear with `ilp-solver` (extra paths through ILP)
- Cast-composed entries (KSAT K2/K3) may show as single entries or separate variant entries

**Step 4: Commit**

```bash
git add src/unit_tests/rules/analysis.rs
git commit -m "test: add integration test for dominated rule detection (#193)"
```

---

### Task 3: Run full test suite and verify no regressions

**Step 1: Run tests without ilp-solver**

Run: `cargo test --lib analysis`
Expected: all tests pass (ILP-gated tests skipped).

**Step 2: Run tests with ilp-solver**

Run: `cargo test --lib --features ilp-solver analysis -- --nocapture`
Expected: all tests pass, stderr shows expected dominated rules.

**Step 3: Run full test suite**

Run: `cargo test --features ilp-solver`
Expected: no regressions.

**Step 4: Commit any adjustments**

```bash
git add src/unit_tests/rules/analysis.rs
git commit -m "test: finalize dominated rule allow-list (#193)"
```

---

### Task 4: Run clippy and format check

**Step 1: Run fmt and clippy**

Run: `make fmt && make clippy`
Expected: no warnings or errors.

**Step 2: Fix any issues**

Apply fixes to `src/rules/analysis.rs` as needed.

**Step 3: Run full check**

Run: `make check`
Expected: all pass.

**Step 4: Commit any fixes**

```bash
git add -u
git commit -m "chore: clippy and format fixes (#193)"
```
