# Redundant Rule Detection — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Detect primitive reduction rules dominated by composite paths, using asymptotic growth-rate classification.

**Architecture:** Add `Expr::growth_rate()` method for classifying expressions, then build `analysis.rs` module in `src/rules/` that iterates all graph edges, finds alternative paths, composes overheads, and compares growth rates. Integration test with allow-list prevents regressions.

**Tech Stack:** Rust, petgraph (already used), existing `Expr`/`ReductionOverhead`/`ReductionGraph` infrastructure.

---

### Task 1: Add `GrowthRate` enum and `Expr::growth_rate()` to `src/expr.rs`

**Files:**
- Modify: `src/expr.rs` (add `GrowthRate` enum + `growth_rate` method after line 133)
- Test: `src/unit_tests/expr.rs` (add growth_rate tests)

**Step 1: Write failing tests in `src/unit_tests/expr.rs`**

Append to end of `src/unit_tests/expr.rs`:

```rust
// --- GrowthRate classification tests ---

use super::GrowthRate;

#[test]
fn test_growth_rate_constant() {
    assert_eq!(Expr::Const(5.0).growth_rate("n"), Ok(GrowthRate::Constant));
}

#[test]
fn test_growth_rate_other_variable() {
    assert_eq!(Expr::Var("m").growth_rate("n"), Ok(GrowthRate::Constant));
}

#[test]
fn test_growth_rate_linear() {
    assert_eq!(
        Expr::Var("n").growth_rate("n"),
        Ok(GrowthRate::Polynomial(1.0))
    );
}

#[test]
fn test_growth_rate_quadratic() {
    let e = Expr::pow(Expr::Var("n"), Expr::Const(2.0));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(2.0)));
}

#[test]
fn test_growth_rate_add_takes_max() {
    // n^2 + n → Polynomial(2.0)
    let e = Expr::add(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
        Expr::Var("n"),
    );
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(2.0)));
}

#[test]
fn test_growth_rate_mul_adds_degrees() {
    // n * n → Polynomial(2.0)
    let e = Expr::mul(Expr::Var("n"), Expr::Var("n"));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(2.0)));
}

#[test]
fn test_growth_rate_mul_const_preserves() {
    // 3 * n → Polynomial(1.0)
    let e = Expr::mul(Expr::Const(3.0), Expr::Var("n"));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(1.0)));
}

#[test]
fn test_growth_rate_exponential() {
    // 2^n → Exponential(2.0)
    let e = Expr::pow(Expr::Const(2.0), Expr::Var("n"));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Exponential(2.0)));
}

#[test]
fn test_growth_rate_super_exponential() {
    // n^n → SuperExponential
    let e = Expr::pow(Expr::Var("n"), Expr::Var("n"));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::SuperExponential));
}

#[test]
fn test_growth_rate_exp_log_cancellation() {
    // exp(log(n^2)) → Polynomial(2.0)
    let e = Expr::Exp(Box::new(Expr::Log(Box::new(
        Expr::pow(Expr::Var("n"), Expr::Const(2.0)),
    ))));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(2.0)));
}

#[test]
fn test_growth_rate_exp_of_polynomial() {
    // exp(n) → Exponential(e)
    let e = Expr::Exp(Box::new(Expr::Var("n")));
    assert_eq!(
        e.growth_rate("n"),
        Ok(GrowthRate::Exponential(std::f64::consts::E))
    );
}

#[test]
fn test_growth_rate_exp_of_constant() {
    // exp(5) → Constant
    let e = Expr::Exp(Box::new(Expr::Const(5.0)));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Constant));
}

#[test]
fn test_growth_rate_log_of_polynomial() {
    // log(n) → Logarithmic
    let e = Expr::Log(Box::new(Expr::Var("n")));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Logarithmic));
}

#[test]
fn test_growth_rate_log_of_exponential() {
    // log(2^n) → Polynomial(1.0)
    let e = Expr::Log(Box::new(Expr::pow(Expr::Const(2.0), Expr::Var("n"))));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(1.0)));
}

#[test]
fn test_growth_rate_log_of_constant() {
    // log(5) → Constant
    let e = Expr::Log(Box::new(Expr::Const(5.0)));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Constant));
}

#[test]
fn test_growth_rate_sqrt() {
    // sqrt(n) → Polynomial(0.5)
    let e = Expr::Sqrt(Box::new(Expr::Var("n")));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(0.5)));
}

#[test]
fn test_growth_rate_sqrt_of_quadratic() {
    // sqrt(n^2) → Polynomial(1.0)
    let e = Expr::Sqrt(Box::new(Expr::pow(Expr::Var("n"), Expr::Const(2.0))));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Polynomial(1.0)));
}

#[test]
fn test_growth_rate_pow_const_log() {
    // 2^log(n) → Polynomial(ln(2))
    let e = Expr::pow(
        Expr::Const(2.0),
        Expr::Log(Box::new(Expr::Var("n"))),
    );
    assert_eq!(
        e.growth_rate("n"),
        Ok(GrowthRate::Polynomial(2.0_f64.ln()))
    );
}

#[test]
fn test_growth_rate_exp_of_exponential() {
    // exp(2^n) → SuperExponential
    let e = Expr::Exp(Box::new(Expr::pow(Expr::Const(2.0), Expr::Var("n"))));
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::SuperExponential));
}

#[test]
fn test_growth_rate_sqrt_of_exponential_errors() {
    // sqrt(2^n) → Error (not classifiable into our categories)
    let e = Expr::Sqrt(Box::new(Expr::pow(Expr::Const(2.0), Expr::Var("n"))));
    assert!(e.growth_rate("n").is_err());
}

#[test]
fn test_growth_rate_ordering() {
    assert!(GrowthRate::Constant < GrowthRate::Logarithmic);
    assert!(GrowthRate::Logarithmic < GrowthRate::Polynomial(1.0));
    assert!(GrowthRate::Polynomial(1.0) < GrowthRate::Polynomial(2.0));
    assert!(GrowthRate::Polynomial(2.0) < GrowthRate::Exponential(2.0));
    assert!(GrowthRate::Exponential(2.0) < GrowthRate::SuperExponential);
    // Different exponential bases
    assert!(GrowthRate::Exponential(1.5) < GrowthRate::Exponential(2.0));
}

#[test]
fn test_growth_rate_mul_exp_poly() {
    // n * 2^n → Exponential(2.0)  (poly * exp = exp)
    let e = Expr::mul(
        Expr::Var("n"),
        Expr::pow(Expr::Const(2.0), Expr::Var("n")),
    );
    assert_eq!(e.growth_rate("n"), Ok(GrowthRate::Exponential(2.0)));
}

#[test]
fn test_growth_rate_real_overhead_identity() {
    // num_vertices (w.r.t. num_vertices) → Polynomial(1.0)
    assert_eq!(
        Expr::Var("num_vertices").growth_rate("num_vertices"),
        Ok(GrowthRate::Polynomial(1.0))
    );
}

#[test]
fn test_growth_rate_real_overhead_quadratic() {
    // num_vertices^2 (w.r.t. num_vertices) → Polynomial(2.0)
    let e = Expr::pow(Expr::Var("num_vertices"), Expr::Const(2.0));
    assert_eq!(
        e.growth_rate("num_vertices"),
        Ok(GrowthRate::Polynomial(2.0))
    );
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib test_growth_rate -- --no-run 2>&1 | head -5`
Expected: compile error — `GrowthRate` not defined, `growth_rate` method not found.

**Step 3: Implement `GrowthRate` and `Expr::growth_rate()` in `src/expr.rs`**

After the `is_polynomial` method (line 132), before the closing `}` of `impl Expr`, add:

```rust
    /// Classify the asymptotic growth rate of this expression w.r.t. a single variable.
    ///
    /// Returns `Err` if the expression cannot be classified into the standard hierarchy.
    pub fn growth_rate(&self, var: &str) -> Result<GrowthRate, String> {
        match self {
            Expr::Const(_) => Ok(GrowthRate::Constant),
            Expr::Var(name) => {
                if *name == var {
                    Ok(GrowthRate::Polynomial(1.0))
                } else {
                    Ok(GrowthRate::Constant)
                }
            }
            Expr::Add(a, b) => {
                let ga = a.growth_rate(var)?;
                let gb = b.growth_rate(var)?;
                Ok(ga.max(gb))
            }
            Expr::Mul(a, b) => {
                let ga = a.growth_rate(var)?;
                let gb = b.growth_rate(var)?;
                GrowthRate::mul(ga, gb)
            }
            Expr::Pow(base, exp) => {
                let gb = base.growth_rate(var)?;
                let ge = exp.growth_rate(var)?;
                GrowthRate::pow(gb, ge, base, exp)
            }
            Expr::Exp(inner) => {
                // Special case: exp(log(x)) cancels
                if let Expr::Log(x) = inner.as_ref() {
                    return x.growth_rate(var);
                }
                let gi = inner.growth_rate(var)?;
                match gi {
                    GrowthRate::Constant => Ok(GrowthRate::Constant),
                    GrowthRate::Logarithmic => Ok(GrowthRate::Polynomial(1.0)),
                    GrowthRate::Polynomial(_) => Ok(GrowthRate::Exponential(std::f64::consts::E)),
                    GrowthRate::Exponential(_) | GrowthRate::SuperExponential => {
                        Ok(GrowthRate::SuperExponential)
                    }
                }
            }
            Expr::Log(inner) => {
                let gi = inner.growth_rate(var)?;
                match gi {
                    GrowthRate::Constant => Ok(GrowthRate::Constant),
                    GrowthRate::Polynomial(_) => Ok(GrowthRate::Logarithmic),
                    GrowthRate::Exponential(_) => Ok(GrowthRate::Polynomial(1.0)),
                    _ => Err(format!("cannot classify log({self}) w.r.t. {var}")),
                }
            }
            Expr::Sqrt(inner) => {
                let gi = inner.growth_rate(var)?;
                match gi {
                    GrowthRate::Constant => Ok(GrowthRate::Constant),
                    GrowthRate::Polynomial(k) => Ok(GrowthRate::Polynomial(k / 2.0)),
                    _ => Err(format!("cannot classify sqrt({self}) w.r.t. {var}")),
                }
            }
        }
    }
```

And above `impl Expr`, add the `GrowthRate` enum:

```rust
/// Asymptotic growth rate classification for expression comparison.
///
/// Ordered: Constant < Logarithmic < Polynomial(k) < Exponential(c) < SuperExponential.
#[derive(Debug, Clone, PartialEq)]
pub enum GrowthRate {
    Constant,
    Logarithmic,
    Polynomial(f64),
    Exponential(f64),
    SuperExponential,
}

impl GrowthRate {
    /// Combine growth rates for multiplication.
    fn mul(a: GrowthRate, b: GrowthRate) -> Result<GrowthRate, String> {
        use GrowthRate::*;
        match (&a, &b) {
            (Constant, other) | (other, Constant) => Ok(other.clone()),
            (Polynomial(k1), Polynomial(k2)) => Ok(Polynomial(k1 + k2)),
            (Exponential(c), Polynomial(_)) | (Polynomial(_), Exponential(c)) => {
                Ok(Exponential(*c))
            }
            (Logarithmic, Polynomial(k)) | (Polynomial(k), Logarithmic) => Ok(Polynomial(*k)),
            (Logarithmic, Logarithmic) => Ok(Logarithmic), // log * log < n^eps
            _ => Err(format!("cannot classify mul({a:?}, {b:?})")),
        }
    }

    /// Combine growth rates for exponentiation.
    fn pow(
        base_rate: GrowthRate,
        exp_rate: GrowthRate,
        base_expr: &Expr,
        exp_expr: &Expr,
    ) -> Result<GrowthRate, String> {
        use GrowthRate::*;
        match (&base_rate, &exp_rate) {
            // Anything ^ 0 (constant exponent of 0)
            (_, Constant) => {
                // base^const: if base is poly(k), result is poly(k*c)
                if let Constant = base_rate {
                    return Ok(Constant);
                }
                if let Polynomial(k) = base_rate {
                    // Extract the constant value from the exponent expression
                    if let Expr::Const(c) = exp_expr {
                        return Ok(Polynomial(k * c));
                    }
                }
                // const^const = const
                Ok(Constant)
            }
            // const^poly = exponential
            (Constant, Polynomial(_)) => {
                if let Expr::Const(c) = base_expr {
                    if *c > 0.0 && (*c - 1.0).abs() > 1e-10 {
                        return Ok(Exponential(*c));
                    }
                    // 1^n = constant
                    if (*c - 1.0).abs() < 1e-10 {
                        return Ok(Constant);
                    }
                }
                Ok(Exponential(std::f64::consts::E))
            }
            // const^log = polynomial
            (Constant, Logarithmic) => {
                if let Expr::Const(c) = base_expr {
                    if *c > 0.0 {
                        return Ok(Polynomial(c.ln()));
                    }
                }
                Ok(Polynomial(1.0))
            }
            // poly^poly = super-exponential (e.g., n^n)
            (Polynomial(_), Polynomial(_)) => Ok(SuperExponential),
            _ => Err(format!(
                "cannot classify pow({base_rate:?}, {exp_rate:?})"
            )),
        }
    }

    /// Return the maximum of two growth rates.
    fn max(self, other: GrowthRate) -> GrowthRate {
        if self >= other { self } else { other }
    }
}

impl PartialOrd for GrowthRate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        use GrowthRate::*;

        fn rank(g: &GrowthRate) -> u8 {
            match g {
                Constant => 0,
                Logarithmic => 1,
                Polynomial(_) => 2,
                Exponential(_) => 3,
                SuperExponential => 4,
            }
        }

        let ra = rank(self);
        let rb = rank(other);
        if ra != rb {
            return ra.partial_cmp(&rb);
        }
        // Same category: compare within
        match (self, other) {
            (Polynomial(a), Polynomial(b)) => a.partial_cmp(b),
            (Exponential(a), Exponential(b)) => a.partial_cmp(b),
            _ => Some(Equal), // Constant==Constant, Log==Log, Super==Super
        }
    }
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test --lib test_growth_rate`
Expected: all `test_growth_rate_*` tests PASS.

**Step 5: Commit**

```bash
git add src/expr.rs src/unit_tests/expr.rs
git commit -m "feat: add GrowthRate classification to Expr (#193)"
```

---

### Task 2: Add `analysis.rs` module with `compare_overhead` and `find_dominated_rules`

**Files:**
- Create: `src/rules/analysis.rs`
- Modify: `src/rules/mod.rs` (add `pub mod analysis;`)
- Create: `src/unit_tests/rules/analysis.rs`

**Step 1: Write failing test in `src/unit_tests/rules/analysis.rs`**

```rust
use crate::rules::analysis::{compare_overhead, find_dominated_rules, DominatedRule};
use crate::rules::graph::ReductionGraph;
use crate::rules::registry::ReductionOverhead;
use crate::expr::Expr;
use std::cmp::Ordering;

#[test]
fn test_compare_overhead_equal() {
    let a = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("num_vertices")),
    ]);
    let b = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("num_vertices")),
    ]);
    assert_eq!(compare_overhead(&a, &b), Some(Ordering::Equal));
}

#[test]
fn test_compare_overhead_composite_smaller() {
    // primitive: num_vars = num_vertices^2
    // composite: num_vars = num_vertices
    let prim = ReductionOverhead::new(vec![
        ("num_vars", Expr::pow(Expr::Var("num_vertices"), Expr::Const(2.0))),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("num_vertices")),
    ]);
    assert_eq!(compare_overhead(&prim, &comp), Some(Ordering::Less));
}

#[test]
fn test_compare_overhead_composite_worse() {
    // primitive: num_vars = num_vertices
    // composite: num_vars = num_vertices^2
    let prim = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("num_vertices")),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_vars", Expr::pow(Expr::Var("num_vertices"), Expr::Const(2.0))),
    ]);
    assert_eq!(compare_overhead(&prim, &comp), None);
}

#[test]
fn test_compare_overhead_multi_field_mixed() {
    // One field better, one worse → None (no dominance)
    let prim = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("n")),
        ("num_constraints", Expr::pow(Expr::Var("n"), Expr::Const(2.0))),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_vars", Expr::pow(Expr::Var("n"), Expr::Const(2.0))),
        ("num_constraints", Expr::Var("n")),
    ]);
    assert_eq!(compare_overhead(&prim, &comp), None);
}

#[test]
fn test_compare_overhead_no_common_fields() {
    let prim = ReductionOverhead::new(vec![
        ("num_vars", Expr::Var("n")),
    ]);
    let comp = ReductionOverhead::new(vec![
        ("num_spins", Expr::Var("n")),
    ]);
    assert_eq!(compare_overhead(&prim, &comp), None);
}

#[cfg(feature = "ilp-solver")]
#[test]
fn test_find_dominated_rules_returns_known_set() {
    let graph = ReductionGraph::new();
    let dominated = find_dominated_rules(&graph);

    // Deduplicate by (source, target) — keep only the shortest dominating path
    let mut seen = std::collections::HashMap::new();
    for rule in &dominated {
        let key = (rule.source.clone(), rule.target.clone());
        let entry = seen.entry(key).or_insert(rule);
        if rule.dominating_path.len() < entry.dominating_path.len() {
            *entry = rule;
        }
    }

    // Print for debugging
    for rule in seen.values() {
        eprintln!(
            "  {} → {} dominated by {} ({}→{})",
            rule.source,
            rule.target,
            rule.dominating_path
                .steps
                .iter()
                .map(|s| s.name.to_string())
                .collect::<Vec<_>>()
                .join(" → "),
            rule.dominating_path.source().unwrap_or("?"),
            rule.dominating_path.target().unwrap_or("?"),
        );
    }

    // Known allow-list of dominated rules (source_name, target_name)
    let allowed: std::collections::HashSet<(&str, &str)> = [
        ("KColoring", "QUBO"),
        ("MaximumIndependentSet", "ILP"),
        ("MaximumIndependentSet", "QUBO"),
        ("MaximumSetPacking", "ILP"),
        ("MinimumVertexCover", "ILP"),
        ("MinimumVertexCover", "QUBO"),
        ("KSatisfiability", "Satisfiability"),
        ("MaximumIndependentSet", "MaximumIndependentSet"),
    ]
    .into_iter()
    .collect();

    // Check: no unexpected dominated rules
    for rule in seen.values() {
        let key = (rule.source.as_str(), rule.target.as_str());
        assert!(
            allowed.contains(&key),
            "Unexpected dominated rule: {} → {} (dominated by {})",
            rule.source,
            rule.target,
            rule.dominating_path
                .steps
                .iter()
                .map(|s| s.name.to_string())
                .collect::<Vec<_>>()
                .join(" → "),
        );
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib --features ilp-solver test_compare_overhead -- --no-run 2>&1 | head -5`
Expected: compile error — `analysis` module not found.

**Step 3: Create `src/rules/analysis.rs` with implementation**

```rust
//! Analysis utilities for the reduction graph.
//!
//! Detects primitive reduction rules that are dominated by composite paths,
//! using asymptotic growth rate classification of overhead expressions.

use crate::expr::{Expr, GrowthRate};
use crate::rules::graph::{ReductionGraph, ReductionPath};
use crate::rules::registry::ReductionOverhead;
use petgraph::visit::EdgeRef;
use std::cmp::Ordering;
use std::collections::HashSet;

/// A primitive reduction rule dominated by a composite path.
#[derive(Debug)]
pub struct DominatedRule {
    /// Source problem (name + variant label).
    pub source: String,
    /// Target problem (name + variant label).
    pub target: String,
    /// The direct edge's overhead.
    pub primitive_overhead: ReductionOverhead,
    /// The composite path that dominates the primitive rule.
    pub dominating_path: ReductionPath,
    /// The composed overhead of the dominating path.
    pub composed_overhead: ReductionOverhead,
    /// Per-field comparison result (field_name, ordering).
    pub comparison: Vec<(String, Ordering)>,
}

/// Compare two overhead expressions for a single field.
///
/// Returns the ordering of `composite` relative to `primitive`:
/// - `Some(Less)` if composite is strictly smaller
/// - `Some(Equal)` if they have the same growth rate
/// - `Some(Greater)` if composite is strictly larger
/// - `None` if comparison fails (unclassifiable expressions)
fn compare_field(primitive: &Expr, composite: &Expr) -> Option<Ordering> {
    // Collect all variables from both expressions
    let mut all_vars: Vec<&str> = primitive
        .variables()
        .union(&composite.variables())
        .copied()
        .collect();
    all_vars.sort();

    if all_vars.is_empty() {
        // Both are constants — compare numerically
        let pv = primitive.eval(&Default::default());
        let cv = composite.eval(&Default::default());
        return pv.partial_cmp(&cv).map(|o| o.reverse());
        // reverse because smaller composite is "Less" (better)
    }

    // For each variable, classify both expressions' growth rates
    let mut max_prim = GrowthRate::Constant;
    let mut max_comp = GrowthRate::Constant;

    for var in &all_vars {
        let gp = primitive.growth_rate(var).ok()?;
        let gc = composite.growth_rate(var).ok()?;
        max_prim = max_prim.max(gp);
        max_comp = max_comp.max(gc);
    }

    max_comp.partial_cmp(&max_prim)
}

/// Compare two overheads across all common fields.
///
/// Returns `Some(Equal)` if composite matches primitive on all fields.
/// Returns `Some(Less)` if composite is strictly smaller on at least one field
/// and no worse on all others.
/// Returns `None` if composite is worse on any field, or no common fields exist.
pub fn compare_overhead(
    primitive: &ReductionOverhead,
    composite: &ReductionOverhead,
) -> Option<Ordering> {
    let comp_map: std::collections::HashMap<&str, &Expr> = composite
        .output_size
        .iter()
        .map(|(name, expr)| (*name, expr))
        .collect();

    let mut any_common = false;
    let mut all_leq = true;
    let mut any_strictly_less = false;

    for (field, prim_expr) in &primitive.output_size {
        let Some(comp_expr) = comp_map.get(field) else {
            continue;
        };
        any_common = true;

        match compare_field(prim_expr, comp_expr) {
            Some(Ordering::Less) => {
                any_strictly_less = true;
            }
            Some(Ordering::Equal) => {}
            Some(Ordering::Greater) | None => {
                all_leq = false;
            }
        }
    }

    if !any_common || !all_leq {
        return None;
    }

    if any_strictly_less {
        Some(Ordering::Less)
    } else {
        Some(Ordering::Equal)
    }
}

/// Find all primitive reduction rules dominated by composite paths.
///
/// A primitive rule (direct edge) A→B is "dominated" if there exists a
/// composite path A→...→B (len > 1) whose composed overhead is asymptotically
/// equal to or smaller than the direct overhead, on all common fields.
pub fn find_dominated_rules(graph: &ReductionGraph) -> Vec<DominatedRule> {
    let mut results = Vec::new();

    // Iterate all edges in the graph
    for edge_info in all_edges(graph) {
        let paths = graph.find_all_paths(
            &edge_info.source_name,
            &edge_info.source_variant,
            &edge_info.target_name,
            &edge_info.target_variant,
        );

        // Filter to composite paths (more than 1 edge = more than 2 steps)
        for path in paths {
            if path.len() <= 1 {
                continue; // skip the direct edge itself
            }

            let composed = graph.compose_path_overhead(&path);
            let cmp = compare_overhead(&edge_info.overhead, &composed);

            if matches!(cmp, Some(Ordering::Equal) | Some(Ordering::Less)) {
                // Build per-field comparison details
                let comp_map: std::collections::HashMap<&str, &Expr> = composed
                    .output_size
                    .iter()
                    .map(|(n, e)| (*n, e))
                    .collect();
                let comparison: Vec<(String, Ordering)> = edge_info
                    .overhead
                    .output_size
                    .iter()
                    .filter_map(|(field, prim_expr)| {
                        let comp_expr = comp_map.get(field)?;
                        let ord = compare_field(prim_expr, comp_expr)?;
                        Some((field.to_string(), ord))
                    })
                    .collect();

                results.push(DominatedRule {
                    source: format_variant(
                        &edge_info.source_name,
                        &edge_info.source_variant,
                    ),
                    target: format_variant(
                        &edge_info.target_name,
                        &edge_info.target_variant,
                    ),
                    primitive_overhead: edge_info.overhead.clone(),
                    dominating_path: path,
                    composed_overhead: composed,
                    comparison,
                });
            }
        }
    }

    results
}

/// Format a problem variant as "Name {key=val, ...}" for display.
fn format_variant(
    name: &str,
    variant: &std::collections::BTreeMap<String, String>,
) -> String {
    if variant.is_empty() {
        name.to_string()
    } else {
        let pairs: Vec<String> = variant
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        format!("{name} {{{}}}", pairs.join(", "))
    }
}

/// Collect all edges from the reduction graph as ReductionEdgeInfo structs.
fn all_edges(
    graph: &ReductionGraph,
) -> Vec<crate::rules::graph::ReductionEdgeInfo> {
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

Add after `pub mod cost;` (line 3):

```rust
pub mod analysis;
```

**Step 5: Run tests to verify they pass**

Run: `cargo test --lib --features ilp-solver analysis`
Expected: all `test_compare_overhead_*` and `test_find_dominated_rules_*` tests PASS.

**Step 6: Commit**

```bash
git add src/rules/analysis.rs src/rules/mod.rs src/unit_tests/rules/analysis.rs
git commit -m "feat: add dominated rule detection utility (#193)"
```

---

### Task 3: Verify against known results and adjust allow-list

**Files:**
- Modify: `src/unit_tests/rules/analysis.rs` (adjust allow-list if needed)

**Step 1: Run the full test and examine output**

Run: `cargo test --lib --features ilp-solver test_find_dominated_rules -- --nocapture 2>&1`
Expected: test passes, stderr shows the known dominated rules matching our earlier analysis.

**Step 2: If any unexpected rules appear, investigate and update allow-list**

The allow-list in the test should match the 9 cases we identified. Adjust the set if the exact variant-level matches differ (e.g., the KSAT K2/K3→SAT might show as separate entries).

**Step 3: Run full test suite**

Run: `cargo test --features ilp-solver`
Expected: all tests pass, no regressions.

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

Apply fixes to `src/expr.rs` and `src/rules/analysis.rs` as needed.

**Step 3: Run full check**

Run: `make check`
Expected: all pass.

**Step 4: Commit any fixes**

```bash
git add -u
git commit -m "chore: clippy and format fixes (#193)"
```
