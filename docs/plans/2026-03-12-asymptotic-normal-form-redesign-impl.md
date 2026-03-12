# Asymptotic Normal Form Redesign — Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Redesign expression normalization into two explicit phases: exact `canonical_form()` and asymptotic `big_o_normal_form()`, plus a `pred-sym` CLI tool for debugging.

**Architecture:** The existing monolithic `asymptotic_normal_form()` is split into two layers. `canonical_form()` does exact symbolic simplification (signed coefficients, term merging, deterministic ordering). `big_o_normal_form()` projects the canonical result into an asymptotic growth class by dropping dominated terms and constant factors. A `pred-sym` binary exposes the symbolic engine directly.

**Tech Stack:** Rust, `problemreductions` crate (`src/expr.rs`), `problemreductions-cli` crate, `clap` CLI framework.

---

## File Structure

### New files
- `src/canonical.rs` — `CanonicalTerm`, `CanonicalSum`, `canonical_form()`, internal algebra model
- `src/big_o.rs` — `big_o_normal_form()`, additive dominance, multiplicative projection
- `src/unit_tests/canonical.rs` — Tests for exact canonicalization
- `src/unit_tests/big_o.rs` — Tests for Big-O projection
- `problemreductions-cli/src/bin/pred_sym.rs` — `pred-sym` binary

### Modified files
- `src/expr.rs` — Add `CanonicalizationError`, keep `asymptotic_normal_form()` as wrapper
- `src/lib.rs` — Re-export new public functions
- `problemreductions-cli/Cargo.toml` — Add `pred-sym` binary
- `problemreductions-cli/src/commands/graph.rs` — Switch to `big_o_normal_form()`

### Reference files (read, don't modify unless specified)
- `src/rules/analysis.rs:96-263` — Existing `Monomial` / `NormalizedPoly` / `normalize_polynomial()` for dominance comparison
- `src/unit_tests/expr.rs` — Existing tests for `asymptotic_normal_form()` (some expectations change)
- `docs/plans/2026-03-11-asymptotic-normal-form-redesign-design.md` — Design document

---

## Chunk 1: Exact Canonicalization Engine

### Task 1: Create canonical.rs with internal algebra model

**Files:**
- Create: `src/canonical.rs`
- Create: `src/unit_tests/canonical.rs`
- Modify: `src/expr.rs` (add `CanonicalizationError` enum)
- Modify: `src/lib.rs` (add module declarations and re-exports)

The internal model is a **canonical sum of terms**, where each term is a signed coefficient times a canonical multiset of factors. For polynomial pieces, factors collapse into monomials. For non-polynomial pieces (exp, log, fractional-power), the canonicalized subexpression is an opaque factor atom.

#### Step 1.1: Define the error type

- [ ] **Add `CanonicalizationError` to `src/expr.rs`**

Add the error enum right after the existing `AsymptoticAnalysisError` definition (around line 286):

```rust
/// Error returned when exact canonicalization fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanonicalizationError {
    /// Expression cannot be canonicalized (e.g., variable in both base and exponent).
    Unsupported(String),
}

impl fmt::Display for CanonicalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported(expr) => write!(f, "unsupported expression for canonicalization: {expr}"),
        }
    }
}

impl std::error::Error for CanonicalizationError {}
```

- [ ] **Verify it compiles**

Run: `cargo check`

- [ ] **Commit**

```
feat: add CanonicalizationError type
```

#### Step 1.2: Create canonical.rs with CanonicalFactor and CanonicalTerm

- [ ] **Write the core data structures in `src/canonical.rs`**

```rust
//! Exact symbolic canonicalization for `Expr`.
//!
//! Normalizes expressions into a canonical sum-of-terms form with signed
//! coefficients and deterministic ordering, without losing algebraic precision.

use std::collections::BTreeMap;
use std::fmt;

use crate::expr::{CanonicalizationError, Expr};

/// An opaque non-polynomial factor (exp, log, fractional-power base).
///
/// Stored by its canonical string representation for deterministic ordering.
#[derive(Clone, Debug, PartialEq)]
struct OpaqueFactor {
    /// The canonical string form (used for equality and ordering).
    key: String,
    /// The original `Expr` for reconstruction.
    expr: Expr,
}

impl Eq for OpaqueFactor {}

impl PartialOrd for OpaqueFactor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpaqueFactor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

/// A single additive term: coefficient × product of canonical factors.
#[derive(Clone, Debug)]
struct CanonicalTerm {
    /// Signed numeric coefficient.
    coeff: f64,
    /// Polynomial variable exponents (variable_name → exponent).
    vars: BTreeMap<&'static str, f64>,
    /// Non-polynomial opaque factors, sorted by key.
    opaque: Vec<OpaqueFactor>,
}

/// Try to merge a new opaque factor into an existing list using transcendental identities.
/// Returns `Some(updated_list)` if a merge happened, `None` if no identity applies.
fn try_merge_opaque(existing: &[OpaqueFactor], new: &OpaqueFactor) -> Option<Vec<OpaqueFactor>> {
    for (i, existing_factor) in existing.iter().enumerate() {
        // exp(a) * exp(b) -> exp(a + b)
        if let (Expr::Exp(a), Expr::Exp(b)) = (&existing_factor.expr, &new.expr) {
            let merged_arg = (**a).clone() + (**b).clone();
            let merged_expr = Expr::Exp(Box::new(
                canonical_form(&merged_arg).unwrap_or(merged_arg),
            ));
            let mut result = existing.to_vec();
            result[i] = OpaqueFactor {
                key: merged_expr.to_string(),
                expr: merged_expr,
            };
            return Some(result);
        }

        // c^a * c^b -> c^(a+b) for matching constant base c
        if let (
            Expr::Pow(base1, exp1),
            Expr::Pow(base2, exp2),
        ) = (&existing_factor.expr, &new.expr)
        {
            if let (Some(c1), Some(c2)) = (base1.constant_value(), base2.constant_value()) {
                if (c1 - c2).abs() < 1e-15 {
                    let merged_exp = (**exp1).clone() + (**exp2).clone();
                    let canon_exp = canonical_form(&merged_exp).unwrap_or(merged_exp);
                    let merged_expr = Expr::Pow(base1.clone(), Box::new(canon_exp));
                    let mut result = existing.to_vec();
                    result[i] = OpaqueFactor {
                        key: merged_expr.to_string(),
                        expr: merged_expr,
                    };
                    return Some(result);
                }
            }
        }
    }
    None
}

/// A canonical sum of terms: the exact normal form of an expression.
#[derive(Clone, Debug)]
pub(crate) struct CanonicalSum {
    terms: Vec<CanonicalTerm>,
}
```

- [ ] **Add the module declaration to `src/expr.rs` or `src/lib.rs`**

In `src/lib.rs`, add after the existing `pub(crate) mod expr;` line:

```rust
pub(crate) mod canonical;
```

- [ ] **Verify it compiles**

Run: `cargo check`

- [ ] **Commit**

```
feat: add canonical.rs with core data structures
```

#### Step 1.3: Implement CanonicalTerm operations

- [ ] **Write term operations in `src/canonical.rs`**

Add to the `CanonicalTerm` impl block:

```rust
impl CanonicalTerm {
    fn constant(c: f64) -> Self {
        Self {
            coeff: c,
            vars: BTreeMap::new(),
            opaque: Vec::new(),
        }
    }

    fn variable(name: &'static str) -> Self {
        let mut vars = BTreeMap::new();
        vars.insert(name, 1.0);
        Self {
            coeff: 1.0,
            vars,
            opaque: Vec::new(),
        }
    }

    fn opaque_factor(expr: Expr) -> Self {
        let key = expr.to_string();
        Self {
            coeff: 1.0,
            vars: BTreeMap::new(),
            opaque: vec![OpaqueFactor { key, expr }],
        }
    }

    /// Multiply two terms, applying transcendental identities:
    /// - `exp(a) * exp(b) -> exp(a + b)`
    /// - `c^a * c^b -> c^(a + b)` for matching constant base `c`
    fn mul(&self, other: &CanonicalTerm) -> CanonicalTerm {
        let coeff = self.coeff * other.coeff;
        let mut vars = self.vars.clone();
        for (&v, &e) in &other.vars {
            *vars.entry(v).or_insert(0.0) += e;
        }
        // Remove zero-exponent variables
        vars.retain(|_, e| e.abs() > 1e-15);

        // Merge opaque factors with transcendental identities
        let mut opaque = self.opaque.clone();
        for other_factor in &other.opaque {
            if let Some(merged) = try_merge_opaque(&opaque, other_factor) {
                opaque = merged;
            } else {
                opaque.push(other_factor.clone());
            }
        }
        opaque.sort();
        CanonicalTerm {
            coeff,
            vars,
            opaque,
        }
    }

    /// The "signature" of this term for merging: same variables with same exponents
    /// and same opaque factors, ignoring coefficient.
    fn signature(&self) -> (Vec<(&'static str, i64)>, Vec<&str>) {
        let vars: Vec<_> = self
            .vars
            .iter()
            .map(|(&k, &v)| (k, (v * 1000.0).round() as i64))
            .collect();
        let opaque: Vec<_> = self.opaque.iter().map(|o| o.key.as_str()).collect();
        (vars, opaque)
    }

    /// Deterministic sort key for ordering terms in a sum.
    fn sort_key(&self) -> (Vec<(&'static str, i64)>, Vec<String>) {
        let vars: Vec<_> = self
            .vars
            .iter()
            .map(|(&k, &v)| (k, (v * 1000.0).round() as i64))
            .collect();
        let opaque: Vec<_> = self.opaque.iter().map(|o| o.key.clone()).collect();
        (vars, opaque)
    }
}
```

- [ ] **Verify it compiles**

Run: `cargo check`

- [ ] **Commit**

```
feat: add CanonicalTerm operations
```

#### Step 1.4: Implement CanonicalSum operations

- [ ] **Write sum operations in `src/canonical.rs`**

```rust
impl CanonicalSum {
    fn zero() -> Self {
        Self { terms: vec![] }
    }

    fn from_term(term: CanonicalTerm) -> Self {
        Self {
            terms: vec![term],
        }
    }

    fn add(mut self, other: CanonicalSum) -> Self {
        self.terms.extend(other.terms);
        self
    }

    fn mul(&self, other: &CanonicalSum) -> CanonicalSum {
        let mut terms = Vec::new();
        for a in &self.terms {
            for b in &other.terms {
                terms.push(a.mul(b));
            }
        }
        CanonicalSum { terms }
    }

    fn scale(mut self, c: f64) -> Self {
        for term in &mut self.terms {
            term.coeff *= c;
        }
        self
    }

    /// Merge terms with the same signature and drop zero-coefficient terms.
    /// Sort the result deterministically.
    fn simplify(self) -> Self {
        use std::collections::BTreeMap as Map;

        // Signature → (representative term with coeff=0, accumulated coefficient)
        let mut groups: Map<(Vec<(&'static str, i64)>, Vec<String>), CanonicalTerm> = Map::new();

        for term in self.terms {
            let key = term.sort_key();
            groups
                .entry(key)
                .and_modify(|existing| existing.coeff += term.coeff)
                .or_insert(term);
        }

        let mut terms: Vec<_> = groups
            .into_values()
            .filter(|t| t.coeff.abs() > 1e-15)
            .collect();

        terms.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));

        CanonicalSum { terms }
    }
```

- [ ] **Verify it compiles**

Run: `cargo check`

- [ ] **Commit**

```
feat: add CanonicalSum operations with simplify
```

#### Step 1.5: Implement canonical_form() — the main entry point

- [ ] **Write the `canonical_form()` function and `to_expr()` reconstruction**

The conversion from `Expr` to `CanonicalSum`:

```rust
/// Normalize an expression into its exact canonical sum-of-terms form.
///
/// This performs exact symbolic simplification:
/// - Flattens nested Add/Mul
/// - Merges duplicate additive terms by summing coefficients
/// - Merges repeated multiplicative factors into powers
/// - Preserves signed coefficients (supports subtraction)
/// - Preserves transcendental identities: exp(a)*exp(b)=exp(a+b), etc.
/// - Produces deterministic ordering
///
/// Does NOT drop terms or constant factors — use `big_o_normal_form()` for that.
pub fn canonical_form(expr: &Expr) -> Result<Expr, CanonicalizationError> {
    let sum = expr_to_canonical(expr)?;
    let simplified = sum.simplify();
    Ok(canonical_sum_to_expr(&simplified))
}

fn expr_to_canonical(expr: &Expr) -> Result<CanonicalSum, CanonicalizationError> {
    match expr {
        Expr::Const(c) => Ok(CanonicalSum::from_term(CanonicalTerm::constant(*c))),
        Expr::Var(name) => Ok(CanonicalSum::from_term(CanonicalTerm::variable(name))),
        Expr::Add(a, b) => {
            let ca = expr_to_canonical(a)?;
            let cb = expr_to_canonical(b)?;
            Ok(ca.add(cb))
        }
        Expr::Mul(a, b) => {
            let ca = expr_to_canonical(a)?;
            let cb = expr_to_canonical(b)?;
            Ok(ca.mul(&cb))
        }
        Expr::Pow(base, exp) => canonicalize_pow(base, exp),
        Expr::Exp(arg) => {
            // Treat exp(canonicalized_arg) as an opaque factor
            let inner = canonical_form(arg)?;
            Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                Expr::Exp(Box::new(inner)),
            )))
        }
        Expr::Log(arg) => {
            let inner = canonical_form(arg)?;
            Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                Expr::Log(Box::new(inner)),
            )))
        }
        Expr::Sqrt(arg) => {
            // sqrt(x) = x^0.5 — canonicalize as power
            canonicalize_pow(arg, &Expr::Const(0.5))
        }
    }
}

fn canonicalize_pow(
    base: &Expr,
    exp: &Expr,
) -> Result<CanonicalSum, CanonicalizationError> {
    match (base, exp) {
        // Constant base, constant exp → numeric constant
        (_, _) if base.constant_value().is_some() && exp.constant_value().is_some() => {
            let b = base.constant_value().unwrap();
            let e = exp.constant_value().unwrap();
            Ok(CanonicalSum::from_term(CanonicalTerm::constant(b.powf(e))))
        }
        // Variable ^ constant integer exponent → polynomial
        (Expr::Var(name), _) if exp.constant_value().is_some() => {
            let e = exp.constant_value().unwrap();
            if e >= 0.0 && (e - e.round()).abs() < 1e-10 {
                let n = e.round() as usize;
                // Build x^n as repeated multiplication
                if n == 0 {
                    return Ok(CanonicalSum::from_term(CanonicalTerm::constant(1.0)));
                }
                let mut vars = BTreeMap::new();
                vars.insert(*name, e);
                Ok(CanonicalSum::from_term(CanonicalTerm {
                    coeff: 1.0,
                    vars,
                    opaque: Vec::new(),
                }))
            } else {
                // Fractional or negative exponent → opaque factor
                Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                    Expr::Pow(Box::new(base.clone()), Box::new(exp.clone())),
                )))
            }
        }
        // Polynomial base ^ constant integer exponent → expand
        (_, _) if exp.constant_value().is_some() => {
            let e = exp.constant_value().unwrap();
            if e >= 0.0 && (e - e.round()).abs() < 1e-10 {
                let n = e.round() as usize;
                let base_sum = expr_to_canonical(base)?;
                if n == 0 {
                    return Ok(CanonicalSum::from_term(CanonicalTerm::constant(1.0)));
                }
                let mut result = base_sum.clone();
                for _ in 1..n {
                    result = result.mul(&base_sum);
                }
                Ok(result)
            } else {
                // Fractional exponent with non-variable base → opaque
                let canon_base = canonical_form(base)?;
                Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                    Expr::Pow(Box::new(canon_base), Box::new(Expr::Const(e))),
                )))
            }
        }
        // Constant base ^ variable exponent → opaque (exponential growth)
        (_, _) if base.constant_value().is_some() => {
            let canon_exp = canonical_form(exp)?;
            Ok(CanonicalSum::from_term(CanonicalTerm::opaque_factor(
                Expr::Pow(Box::new(base.clone()), Box::new(canon_exp)),
            )))
        }
        // Variable base ^ variable exponent → unsupported
        _ => Err(CanonicalizationError::Unsupported(format!(
            "{}^{}",
            base, exp
        ))),
    }
}
```

The reconstruction from `CanonicalSum` back to `Expr`:

```rust
fn canonical_sum_to_expr(sum: &CanonicalSum) -> Expr {
    if sum.terms.is_empty() {
        return Expr::Const(0.0);
    }

    let term_exprs: Vec<Expr> = sum.terms.iter().map(canonical_term_to_expr).collect();

    let mut result = term_exprs[0].clone();
    for term in &term_exprs[1..] {
        result = result + term.clone();
    }
    result
}

fn canonical_term_to_expr(term: &CanonicalTerm) -> Expr {
    let mut factors: Vec<Expr> = Vec::new();

    // Add coefficient if not 1.0 (or -1.0, handled specially)
    let (coeff_factor, sign) = if term.coeff < 0.0 {
        (term.coeff.abs(), true)
    } else {
        (term.coeff, false)
    };

    let has_other_factors = !term.vars.is_empty() || !term.opaque.is_empty();

    if (coeff_factor - 1.0).abs() > 1e-15 || !has_other_factors {
        factors.push(Expr::Const(coeff_factor));
    }

    // Add variable powers
    for (&var, &exp) in &term.vars {
        if (exp - 1.0).abs() < 1e-15 {
            factors.push(Expr::Var(var));
        } else {
            factors.push(Expr::pow(Expr::Var(var), Expr::Const(exp)));
        }
    }

    // Add opaque factors
    for opaque in &term.opaque {
        factors.push(opaque.expr.clone());
    }

    let mut result = if factors.is_empty() {
        Expr::Const(1.0)
    } else {
        let mut r = factors[0].clone();
        for f in &factors[1..] {
            r = r * f.clone();
        }
        r
    };

    if sign {
        result = -result;
    }

    result
}
```

**Important:** The `constant_value()` method is currently a private helper on `Expr` in `src/expr.rs`. It needs to be made `pub(crate)` so `canonical.rs` can call it. Edit `src/expr.rs` line ~180:

```rust
    // Change: fn constant_value(&self) -> Option<f64> {
    // To:
    pub(crate) fn constant_value(&self) -> Option<f64> {
```

- [ ] **Verify it compiles**

Run: `cargo check`

- [ ] **Commit**

```
feat: implement canonical_form() with expr-to-canonical conversion
```

#### Step 1.6: Write tests for canonical_form()

- [ ] **Create `src/unit_tests/canonical.rs` and wire it up**

Add to `src/canonical.rs` at the bottom:

```rust
#[cfg(test)]
#[path = "unit_tests/canonical.rs"]
mod tests;
```

Write the test file:

```rust
use super::*;
use crate::expr::Expr;

#[test]
fn test_canonical_identity() {
    let e = Expr::Var("n");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "n");
}

#[test]
fn test_canonical_add_like_terms() {
    // n + n → 2 * n
    let e = Expr::Var("n") + Expr::Var("n");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "2 * n");
}

#[test]
fn test_canonical_subtract_to_zero() {
    // n - n → 0
    let e = Expr::Var("n") - Expr::Var("n");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "0");
}

#[test]
fn test_canonical_mixed_addition() {
    // n + n - m + 2*m → 2*n + m
    let e = Expr::Var("n") + Expr::Var("n") - Expr::Var("m") + Expr::Const(2.0) * Expr::Var("m");
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "m + 2 * n");
}

#[test]
fn test_canonical_power_merge() {
    // n * n^(1/2) → n^1.5
    let e = Expr::Var("n") * Expr::pow(Expr::Var("n"), Expr::Const(0.5));
    let c = canonical_form(&e).unwrap();
    // Note: n^0.5 becomes an opaque factor, so this merges to n * n^0.5 in product form
    // The exact representation depends on how opaque factors merge.
    // This test should be adjusted based on actual behavior.
    let size = crate::types::ProblemSize::new(vec![("n", 4)]);
    assert!((c.eval(&size) - 4.0_f64.powf(1.5)).abs() < 1e-10);
}

#[test]
fn test_canonical_exp_product_identity() {
    // exp(n) * exp(m) -> exp(n + m)  (transcendental identity)
    let e = Expr::Exp(Box::new(Expr::Var("n"))) * Expr::Exp(Box::new(Expr::Var("m")));
    let c = canonical_form(&e).unwrap();
    let s = c.to_string();
    // Should merge into a single exp() factor
    assert!(s.contains("exp"), "expected exp in result, got: {s}");
    // Verify numerical equivalence
    let size = crate::types::ProblemSize::new(vec![("n", 2), ("m", 3)]);
    assert!((c.eval(&size) - (2.0_f64.exp() * 3.0_f64.exp())).abs() < 1e-6);
}

#[test]
fn test_canonical_constant_base_exp_identity() {
    // 2^n * 2^m -> 2^(n + m)
    let e = Expr::pow(Expr::Const(2.0), Expr::Var("n"))
        * Expr::pow(Expr::Const(2.0), Expr::Var("m"));
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 3), ("m", 4)]);
    assert!((c.eval(&size) - 2.0_f64.powf(7.0)).abs() < 1e-6);
}

#[test]
fn test_canonical_polynomial_expansion() {
    // (n + m)^2 = n^2 + 2*n*m + m^2
    let e = Expr::pow(Expr::Var("n") + Expr::Var("m"), Expr::Const(2.0));
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 3), ("m", 4)]);
    assert_eq!(c.eval(&size), 49.0); // (3+4)^2 = 49
}

#[test]
fn test_canonical_signed_polynomial() {
    // n^3 - n^2 + 2*n + 4*n*m — should remain exact
    let e = Expr::pow(Expr::Var("n"), Expr::Const(3.0))
        - Expr::pow(Expr::Var("n"), Expr::Const(2.0))
        + Expr::Const(2.0) * Expr::Var("n")
        + Expr::Const(4.0) * Expr::Var("n") * Expr::Var("m");
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 3), ("m", 2)]);
    // 27 - 9 + 6 + 24 = 48
    assert_eq!(c.eval(&size), 48.0);
}

#[test]
fn test_canonical_division_becomes_negative_exponent() {
    // n / m should canonicalize; the division is represented as m^(-1)
    // which becomes an opaque factor (negative exponent)
    let e = Expr::Var("n") / Expr::Var("m");
    let c = canonical_form(&e).unwrap();
    let size = crate::types::ProblemSize::new(vec![("n", 6), ("m", 3)]);
    assert!((c.eval(&size) - 2.0).abs() < 1e-10);
}

#[test]
fn test_canonical_deterministic_order() {
    // m + n and n + m should produce the same canonical form
    let a = canonical_form(&(Expr::Var("m") + Expr::Var("n"))).unwrap();
    let b = canonical_form(&(Expr::Var("n") + Expr::Var("m"))).unwrap();
    assert_eq!(a.to_string(), b.to_string());
}

#[test]
fn test_canonical_constant_folding() {
    // 2 + 3 → 5
    let e = Expr::Const(2.0) + Expr::Const(3.0);
    let c = canonical_form(&e).unwrap();
    assert_eq!(c.to_string(), "5");
}

#[test]
fn test_canonical_sqrt_as_power() {
    // sqrt(n) should canonicalize the same as n^0.5
    let a = canonical_form(&Expr::Sqrt(Box::new(Expr::Var("n")))).unwrap();
    let b = canonical_form(&Expr::pow(Expr::Var("n"), Expr::Const(0.5))).unwrap();
    assert_eq!(a.to_string(), b.to_string());
}
```

- [ ] **Run tests to see which pass**

Run: `cargo test -p problemreductions canonical -- --nocapture`

- [ ] **Fix any test failures by adjusting expectations or the implementation**

The exact output format of `canonical_form()` depends on the reconstruction logic. Adjust test expectations to match actual output. The critical invariant is: `canonical_form(a).eval(vars) == a.eval(vars)` for all inputs.

- [ ] **Commit**

```
feat: add canonical_form tests
```

#### Step 1.7: Re-export canonical_form from lib.rs

- [ ] **Update `src/lib.rs`**

Change the re-export line to include the new items:

```rust
pub use expr::{asymptotic_normal_form, AsymptoticAnalysisError, CanonicalizationError, Expr};
pub use canonical::canonical_form;
```

But `canonical` is `pub(crate)`, so we need to re-export the function directly. Add to `src/lib.rs`:

```rust
pub use canonical::canonical_form;
```

And make `canonical_form` pub in `src/canonical.rs` (it should already be `pub`).

- [ ] **Run full test suite**

Run: `make check`

- [ ] **Commit**

```
feat: export canonical_form from crate root
```

---

## Chunk 2: Big-O Projection Layer

### Task 2: Implement big_o_normal_form()

**Files:**
- Create: `src/big_o.rs`
- Create: `src/unit_tests/big_o.rs`
- Modify: `src/lib.rs` (add module and re-export)
- Modify: `src/expr.rs` (convert `asymptotic_normal_form()` to wrapper)

The Big-O projection takes the output of `canonical_form()` and:
1. Drops constant terms
2. Drops constant multiplicative factors
3. Drops terms dominated by other terms (using monomial comparison)
4. Rejects invalid negative-only results

#### Step 2.1: Create big_o.rs with the projection function

- [ ] **Write `src/big_o.rs`**

```rust
//! Big-O asymptotic projection for canonical expressions.
//!
//! Takes the output of `canonical_form()` and projects it into an
//! asymptotic growth class by dropping dominated terms and constant factors.

use crate::canonical::canonical_form;
use crate::expr::{AsymptoticAnalysisError, CanonicalizationError, Expr};

/// Compute the Big-O normal form of an expression.
///
/// This is a two-phase pipeline:
/// 1. `canonical_form()` — exact symbolic simplification
/// 2. Asymptotic projection — drop dominated terms and constant factors
///
/// Returns an expression representing the asymptotic growth class.
pub fn big_o_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    let canonical = canonical_form(expr).map_err(|e| match e {
        CanonicalizationError::Unsupported(s) => AsymptoticAnalysisError::Unsupported(s),
    })?;

    project_big_o(&canonical)
}

/// Project a canonicalized expression into its Big-O growth class.
fn project_big_o(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    // Decompose into additive terms
    let mut terms = Vec::new();
    collect_additive_terms(expr, &mut terms);

    // Project each term: drop constant multiplicative factors
    let mut projected: Vec<Expr> = Vec::new();
    for term in &terms {
        if let Some(projected_term) = project_term(term) {
            projected.push(projected_term);
        }
        // Pure constants are dropped (asymptotically irrelevant)
    }

    // Remove dominated terms
    let survivors = remove_dominated_terms(projected);

    if survivors.is_empty() {
        // All terms were constants → O(1)
        return Ok(Expr::Const(1.0));
    }

    // Deduplicate
    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();
    for term in survivors {
        let key = term.to_string();
        if seen.insert(key) {
            deduped.push(term);
        }
    }

    // Rebuild sum
    let mut result = deduped[0].clone();
    for term in &deduped[1..] {
        result = result + term.clone();
    }

    Ok(result)
}

fn collect_additive_terms(expr: &Expr, out: &mut Vec<Expr>) {
    match expr {
        Expr::Add(a, b) => {
            collect_additive_terms(a, out);
            collect_additive_terms(b, out);
        }
        other => out.push(other.clone()),
    }
}

/// Project a single multiplicative term: strip constant factors.
/// Returns None if the term is a pure constant.
fn project_term(term: &Expr) -> Option<Expr> {
    if term.constant_value().is_some() {
        return None; // Pure constant → dropped
    }

    // Collect multiplicative factors
    let mut factors = Vec::new();
    collect_multiplicative_factors(term, &mut factors);

    // Remove constant factors, keep symbolic ones
    let symbolic: Vec<&Expr> = factors
        .iter()
        .filter(|f| f.constant_value().is_none())
        .collect();

    if symbolic.is_empty() {
        return None;
    }

    // Check for negative coefficient (Const(-1) * ...) — take absolute value
    let mut result = symbolic[0].clone();
    for f in &symbolic[1..] {
        result = result * (*f).clone();
    }
    Some(result)
}

fn collect_multiplicative_factors(expr: &Expr, out: &mut Vec<Expr>) {
    match expr {
        Expr::Mul(a, b) => {
            collect_multiplicative_factors(a, out);
            collect_multiplicative_factors(b, out);
        }
        other => out.push(other.clone()),
    }
}

/// Remove terms dominated by other terms using monomial comparison.
///
/// A term `t` is dominated if there exists another term `s` such that
/// `t` grows no faster than `s` asymptotically.
fn remove_dominated_terms(terms: Vec<Expr>) -> Vec<Expr> {
    if terms.len() <= 1 {
        return terms;
    }

    let mut survivors = Vec::new();
    for (i, term) in terms.iter().enumerate() {
        let is_dominated = terms.iter().enumerate().any(|(j, other)| {
            i != j && term_dominated_by(term, other)
        });
        if !is_dominated {
            survivors.push(term.clone());
        }
    }
    survivors
}

/// Check if `small` is asymptotically dominated by `big`.
///
/// Conservative: only returns true when dominance is provable
/// via monomial exponent comparison.
fn term_dominated_by(small: &Expr, big: &Expr) -> bool {
    // Extract monomial exponents for comparison
    let small_exps = extract_var_exponents(small);
    let big_exps = extract_var_exponents(big);

    // Both must be pure polynomial monomials for comparison
    let (Some(se), Some(be)) = (small_exps, big_exps) else {
        return false; // Can't compare non-polynomial terms
    };

    // small ≤ big if: for every variable in small, big has ≥ exponent
    // AND big has at least one strictly greater exponent or has a variable small doesn't
    let mut all_leq = true;
    let mut any_strictly_less = false;

    for (var, small_exp) in &se {
        let big_exp = be.get(var).copied().unwrap_or(0.0);
        if *small_exp > big_exp + 1e-15 {
            all_leq = false;
            break;
        }
        if *small_exp < big_exp - 1e-15 {
            any_strictly_less = true;
        }
    }

    // Also check variables in big not in small (those have implicit exponent 0 in small)
    if all_leq {
        for (var, big_exp) in &be {
            if !se.contains_key(var) && *big_exp > 1e-15 {
                any_strictly_less = true;
            }
        }
    }

    // Dominated if all exponents ≤ AND at least one is strictly less.
    // Equal terms are NOT dominated — they get deduped in a separate step.
    all_leq && any_strictly_less
}

/// Extract variable → exponent mapping from a monomial expression.
/// Returns None for non-polynomial terms (exp, log, etc.).
fn extract_var_exponents(expr: &Expr) -> Option<std::collections::BTreeMap<&'static str, f64>> {
    use std::collections::BTreeMap;
    let mut exps = BTreeMap::new();
    extract_var_exponents_inner(expr, &mut exps)?;
    Some(exps)
}

fn extract_var_exponents_inner(
    expr: &Expr,
    exps: &mut std::collections::BTreeMap<&'static str, f64>,
) -> Option<()> {
    match expr {
        Expr::Var(name) => {
            *exps.entry(name).or_insert(0.0) += 1.0;
            Some(())
        }
        Expr::Pow(base, exp) => {
            if let (Expr::Var(name), Some(e)) = (base.as_ref(), exp.constant_value()) {
                *exps.entry(name).or_insert(0.0) += e;
                Some(())
            } else {
                None // Non-simple power
            }
        }
        Expr::Mul(a, b) => {
            extract_var_exponents_inner(a, exps)?;
            extract_var_exponents_inner(b, exps)
        }
        Expr::Const(_) => Some(()), // Constants don't affect exponents
        _ => None, // exp, log, sqrt → not a polynomial monomial
    }
}
```

- [ ] **Add module declaration to `src/lib.rs`**

```rust
pub(crate) mod big_o;
```

- [ ] **Verify it compiles**

Run: `cargo check`

- [ ] **Commit**

```
feat: implement big_o_normal_form() projection layer
```

#### Step 2.2: Write tests for big_o_normal_form()

- [ ] **Create `src/unit_tests/big_o.rs`**

Add to `src/big_o.rs`:

```rust
#[cfg(test)]
#[path = "unit_tests/big_o.rs"]
mod tests;
```

Write the tests:

```rust
use crate::big_o::big_o_normal_form;
use crate::expr::Expr;

#[test]
fn test_big_o_drops_constant_factors() {
    let e = Expr::parse("3 * n^2");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n^2");
}

#[test]
fn test_big_o_drops_additive_constants() {
    let e = Expr::parse("n + 1");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n");
}

#[test]
fn test_big_o_duplicate_terms_collapse() {
    // n + n → (canonical: 2*n) → big-o: n
    let e = Expr::parse("n + n");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n");
}

#[test]
fn test_big_o_lower_order_drops() {
    // n^3 + n^2 → n^3
    let e = Expr::parse("n^3 + n^2");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "n^3");
}

#[test]
fn test_big_o_signed_polynomial() {
    // n^3 - n^2 + 2*n + 4*n*m → n^3 + n*m
    let e = Expr::parse("n^3 - n^2 + 2 * n + 4 * n * m");
    let result = big_o_normal_form(&e).unwrap();
    // n^3 dominates n^2 and n; n*m is incomparable with n^3
    // Expected: n^3 + n*m (or m*n depending on ordering)
    let s = result.to_string();
    assert!(s.contains("n^3"), "missing n^3 term, got: {s}");
    assert!(s.contains("m") && s.contains("n"), "missing n*m term, got: {s}");
}

#[test]
fn test_big_o_commutative_sum() {
    let a = big_o_normal_form(&Expr::parse("n + m")).unwrap();
    let b = big_o_normal_form(&Expr::parse("m + n")).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_big_o_commutative_product() {
    let a = big_o_normal_form(&Expr::parse("n * m")).unwrap();
    let b = big_o_normal_form(&Expr::parse("m * n")).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_big_o_incomparable_terms_survive() {
    // n^2 + n*m — incomparable, both survive
    let e = Expr::parse("n^2 + n * m");
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("n"), "got: {s}");
    assert!(s.contains("m"), "got: {s}");
}

#[test]
fn test_big_o_composed_overhead_duplicate() {
    // (n + m) + (m + n) should reduce to m + n
    let e = Expr::parse("n + m + m + n");
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), big_o_normal_form(&Expr::parse("m + n")).unwrap().to_string());
}

#[test]
fn test_big_o_exp_with_polynomial() {
    // exp(n) + n^10 — incomparable, both survive
    let e = Expr::Exp(Box::new(Expr::Var("n"))) + Expr::pow(Expr::Var("n"), Expr::Const(10.0));
    let result = big_o_normal_form(&e).unwrap();
    let s = result.to_string();
    assert!(s.contains("exp"), "expected exp term to survive, got: {s}");
    assert!(s.contains("n"), "expected polynomial term to survive, got: {s}");
}

#[test]
fn test_big_o_pure_constant_returns_one() {
    let e = Expr::Const(42.0);
    let result = big_o_normal_form(&e).unwrap();
    assert_eq!(result.to_string(), "1");
}
```

- [ ] **Run tests**

Run: `cargo test -p problemreductions big_o -- --nocapture`

- [ ] **Fix failures by adjusting expectations or implementation**

- [ ] **Commit**

```
feat: add big_o_normal_form tests
```

#### Step 2.3: Wire asymptotic_normal_form as wrapper and re-export

- [ ] **Update `src/expr.rs`**

Replace the existing `asymptotic_normal_form()` function body (keep the function and its docs):

```rust
/// Return a normalized `Expr` representing the asymptotic behavior of `expr`.
///
/// This is now a compatibility wrapper for `big_o_normal_form()`.
pub fn asymptotic_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    crate::big_o::big_o_normal_form(expr)
}
```

**Important:** Keep the old implementation available temporarily. A safe approach:
1. Rename the old function to `asymptotic_normal_form_legacy()`
2. Make the new `asymptotic_normal_form()` call `big_o_normal_form()`
3. Run all existing tests — if any fail, investigate whether the new behavior is correct or needs adjustment
4. Once all tests pass, delete the legacy function and all its internal helpers

- [ ] **Update `src/lib.rs` re-exports**

```rust
pub use big_o::big_o_normal_form;
pub use canonical::canonical_form;
pub use expr::{asymptotic_normal_form, AsymptoticAnalysisError, CanonicalizationError, Expr};
```

- [ ] **Run full test suite**

Run: `make check`

Expect some existing `asymptotic_normal_form` tests to fail if behavior changed (e.g., `test_asymptotic_normal_form_rejects_negative_forms` — the new engine handles negatives). Adjust test expectations to match the redesign:

- `n - m` may now succeed (canonical form) but Big-O projection rejects if no positive dominant term
- Exponential identities should still work
- Duplicate term collapse should now work correctly

- [ ] **Commit**

```
feat: wire asymptotic_normal_form as wrapper for big_o_normal_form
```

#### Step 2.4: Clean up legacy code

- [ ] **Remove old normalization helpers from `src/expr.rs`**

Once all tests pass with the new pipeline, delete:
- `asymptotic_normal_form_legacy()` (if renamed)
- `normalize_pow()`
- `collect_sum_term()`
- `collect_product_factor()`
- `build_sum()`
- `build_product()`
- `build_pow()`
- `build_exp()`
- `build_exp_base()`
- `build_log()`
- `into_base_and_exponent()`
- `combine_add_chain()`
- `combine_mul_chain()`
- `format_float()`

Keep only: `asymptotic_normal_form()` (the wrapper), the `Expr` type, parsing, `Display`, operator impls, and `CanonicalizationError`.

- [ ] **Run full test suite**

Run: `make check`

- [ ] **Commit**

```
refactor: remove legacy asymptotic normalization helpers
```

---

## Chunk 3: Display Improvements and Caller Integration

### Task 3: Fix sqrt display and update CLI callers

**Files:**
- Modify: `src/expr.rs` (Display impl for Pow with 0.5 exponent)
- Modify: `problemreductions-cli/src/commands/graph.rs` (switch to `big_o_normal_form`)
- Modify: `src/rules/analysis.rs` (use `canonical_form` where appropriate)
- Modify: relevant test files

#### Step 3.1: Improve Display for Pow(_, 0.5)

- [ ] **Update `Display for Expr` in `src/expr.rs`**

In the `Pow` arm of the `Display` impl, add a special case:

```rust
Expr::Pow(base, exp) => {
    // Special case: x^0.5 → sqrt(x)
    if let Expr::Const(e) = exp.as_ref() {
        if (*e - 0.5).abs() < 1e-15 {
            return write!(f, "sqrt({base})");
        }
    }
    // ... existing parenthesization logic ...
}
```

- [ ] **Add test**

In `src/unit_tests/expr.rs`:

```rust
#[test]
fn test_expr_display_pow_half_as_sqrt() {
    let e = Expr::pow(Expr::Var("n"), Expr::Const(0.5));
    assert_eq!(format!("{e}"), "sqrt(n)");
}

#[test]
fn test_expr_display_pow_half_complex_base() {
    let e = Expr::pow(Expr::Var("n") * Expr::Var("m"), Expr::Const(0.5));
    assert_eq!(format!("{e}"), "sqrt(n * m)");
}

#[test]
fn test_expr_display_pow_half_in_exponent() {
    // 2^(n^0.5) should display as 2^sqrt(n), NOT 2^n^0.5
    let e = Expr::pow(Expr::Const(2.0), Expr::pow(Expr::Var("n"), Expr::Const(0.5)));
    let s = format!("{e}");
    assert!(s.contains("sqrt"), "expected sqrt notation, got: {s}");
    assert!(!s.contains("0.5"), "should not contain raw 0.5, got: {s}");
}
```

- [ ] **Run tests**

Run: `make test`

- [ ] **Commit**

```
fix: display Pow(x, 0.5) as sqrt(x) to avoid ambiguous notation
```

#### Step 3.2: Switch CLI to use big_o_normal_form

- [ ] **Update `problemreductions-cli/src/commands/graph.rs`**

Change the import:

```rust
use problemreductions::{big_o_normal_form, Expr};
```

Update `big_o_of()`:

```rust
fn big_o_of(expr: &Expr) -> String {
    match big_o_normal_form(expr) {
        Ok(norm) => format!("O({})", norm),
        Err(_) => format!("O({})", expr),
    }
}
```

Remove the now-unused `asymptotic_normal_form` import.

- [ ] **Run CLI tests**

Run: `cargo test -p problemreductions-cli`

- [ ] **Commit**

```
refactor: switch CLI to big_o_normal_form
```

#### Step 3.3: Update analysis.rs to use canonical_form for comparisons

- [ ] **Read `src/rules/analysis.rs` compare_overhead function**

The existing `compare_overhead()` at line ~273 calls `asymptotic_normal_form()` then `normalize_polynomial()`. With the redesign, it should:
1. Call `canonical_form()` for exact normalization
2. Then use its own polynomial extraction for dominance comparison

Update the import and the function body:

```rust
use crate::canonical::canonical_form;
```

In `compare_overhead()`, replace `asymptotic_normal_form(prim_expr)` calls with `canonical_form(prim_expr)` (converting the error type). The polynomial extraction (`normalize_polynomial`) still operates on the canonical `Expr`, so it should work without changes.

- [ ] **Run analysis tests**

Run: `cargo test -p problemreductions analysis -- --nocapture`

- [ ] **Commit**

```
refactor: use canonical_form in overhead comparison
```

---

## Chunk 4: pred-sym Binary

### Task 4: Add pred-sym CLI tool

**Files:**
- Modify: `problemreductions-cli/Cargo.toml` (add binary)
- Create: `problemreductions-cli/src/bin/pred_sym.rs` (main binary)
- Add CLI integration tests

#### Step 4.1: Add binary to Cargo.toml

- [ ] **Update `problemreductions-cli/Cargo.toml`**

Add after the existing `[[bin]]` section:

```toml
[[bin]]
name = "pred-sym"
path = "src/bin/pred_sym.rs"
```

- [ ] **Create a minimal `src/bin/pred_sym.rs`**

```rust
use clap::{Parser, Subcommand};
use problemreductions::{Expr, canonical_form, big_o_normal_form};
use problemreductions::types::ProblemSize;

#[derive(Parser)]
#[command(name = "pred-sym", about = "Symbolic expression engine for problemreductions")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and echo an expression
    Parse {
        /// Expression string
        expr: String,
    },
    /// Compute exact canonical form
    Canon {
        /// Expression string
        expr: String,
    },
    /// Compute Big-O normal form
    BigO {
        /// Expression string
        #[arg(name = "expr")]
        expr: String,
    },
    /// Compare two expressions
    Compare {
        /// First expression
        a: String,
        /// Second expression
        b: String,
    },
    /// Evaluate an expression with variable bindings
    Eval {
        /// Expression string
        expr: String,
        /// Variable bindings (e.g., n=10,m=20)
        #[arg(long)]
        vars: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { expr } => {
            let parsed = Expr::parse(&expr);
            println!("{parsed}");
        }
        Commands::Canon { expr } => {
            let parsed = Expr::parse(&expr);
            match canonical_form(&parsed) {
                Ok(result) => println!("{result}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::BigO { expr } => {
            let parsed = Expr::parse(&expr);
            match big_o_normal_form(&parsed) {
                Ok(result) => println!("O({result})"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Compare { a, b } => {
            let expr_a = Expr::parse(&a);
            let expr_b = Expr::parse(&b);
            let canon_a = canonical_form(&expr_a);
            let canon_b = canonical_form(&expr_b);
            let big_o_a = big_o_normal_form(&expr_a);
            let big_o_b = big_o_normal_form(&expr_b);

            let exact_equal = match (&canon_a, &canon_b) {
                (Ok(a), Ok(b)) => Some(a == b),
                _ => None,
            };
            let big_o_equal = match (&big_o_a, &big_o_b) {
                (Ok(a), Ok(b)) => Some(a == b),
                _ => None,
            };

            println!("Expression A: {a}");
            println!("Expression B: {b}");
            if let (Ok(ca), Ok(cb)) = (&canon_a, &canon_b) {
                println!("Canonical A:  {ca}");
                println!("Canonical B:  {cb}");
                println!("Exact equal:  {}", ca == cb);
            }
            if let (Ok(ba), Ok(bb)) = (&big_o_a, &big_o_b) {
                println!("Big-O A:      O({ba})");
                println!("Big-O B:      O({bb})");
                println!("Big-O equal:  {}", ba == bb);
            }
        }
        Commands::Eval { expr, vars } => {
            let parsed = Expr::parse(&expr);
            let bindings: Vec<(&str, usize)> = vars
                .split(',')
                .filter_map(|pair| {
                    let mut parts = pair.splitn(2, '=');
                    let name = parts.next()?.trim();
                    let value: usize = parts.next()?.trim().parse().ok()?;
                    // Leak the name for &'static str compatibility
                    let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
                    Some((leaked, value))
                })
                .collect();
            let size = ProblemSize::new(bindings);
            let result = parsed.eval(&size);

            // Format as integer if it's a whole number
            if (result - result.round()).abs() < 1e-10 {
                println!("{}", result.round() as i64);
            } else {
                println!("{result}");
            }
        }
    }
}
```

- [ ] **Verify it builds**

Run: `cargo build -p problemreductions-cli`

- [ ] **Commit**

```
feat: add pred-sym binary for symbolic engine inspection
```

#### Step 4.2: Add pred-sym integration tests

- [ ] **Add tests to an existing CLI test file or create a new one**

Add to `problemreductions-cli/tests/` (check the existing test structure first — the CLI uses `assert_cmd` or similar):

```rust
#[test]
fn test_pred_sym_parse() {
    let output = Command::cargo_bin("pred-sym")
        .unwrap()
        .args(["parse", "n + m"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "n + m");
}

#[test]
fn test_pred_sym_canon_merge_terms() {
    let output = Command::cargo_bin("pred-sym")
        .unwrap()
        .args(["canon", "n + n"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "2 * n");
}

#[test]
fn test_pred_sym_big_o() {
    let output = Command::cargo_bin("pred-sym")
        .unwrap()
        .args(["big-o", "3 * n^2 + n"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "O(n^2)");
}

#[test]
fn test_pred_sym_eval() {
    let output = Command::cargo_bin("pred-sym")
        .unwrap()
        .args(["eval", "n + m", "--vars", "n=3,m=4"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim(), "7");
}

#[test]
fn test_pred_sym_big_o_signed_polynomial() {
    let output = Command::cargo_bin("pred-sym")
        .unwrap()
        .args(["big-o", "n^3 - n^2 + 2*n + 4*n*m"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // n^3 dominates n^2 and n; n*m is incomparable
    assert!(stdout.contains("n^3"), "got: {}", stdout.trim());
}

#[test]
fn test_pred_sym_big_o_sqrt_display() {
    let output = Command::cargo_bin("pred-sym")
        .unwrap()
        .args(["big-o", "2^(n^(1/2))"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("sqrt"), "expected sqrt notation, got: {}", stdout.trim());
}

#[test]
fn test_pred_sym_compare() {
    let output = Command::cargo_bin("pred-sym")
        .unwrap()
        .args(["compare", "n + n", "2 * n"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("true"), "expected exact equality, got: {}", stdout.trim());
}

```

- [ ] **Run tests**

Run: `cargo test -p problemreductions-cli pred_sym`

- [ ] **Commit**

```
feat: add pred-sym integration tests
```

---

## Chunk 5: Test Rebaseline and Final Verification

### Task 5: Update existing tests and full verification

**Files:**
- Modify: `src/unit_tests/expr.rs` (update asymptotic_normal_form test expectations)
- Modify: `src/unit_tests/rules/analysis.rs` (adjust if comparison logic changed)

#### Step 5.1: Rebaseline asymptotic_normal_form tests

- [ ] **Run all tests and collect failures**

Run: `make test 2>&1 | grep "FAILED\|failures"`

- [ ] **For each failing test, decide:**

1. Is the old expectation still correct under the new design? → Keep
2. Does the new design produce a different but correct result? → Update expectation
3. Does the test encode behavior that the redesign explicitly changes? → Rewrite

Key expected changes:
- `test_asymptotic_normal_form_rejects_negative_forms`: `n - m` should now succeed through `canonical_form()` + `big_o_normal_form()` (both terms are O(n) and O(m), incomparable, both survive). Change the test from expecting `Err` to expecting `Ok` with both terms.
- Duplicate term tests should now pass where they previously produced `O(x + x)`.

- [ ] **Update each failing test**

- [ ] **Run full suite**

Run: `make check`

- [ ] **Commit**

```
test: rebaseline asymptotic_normal_form tests for two-phase pipeline
```

#### Step 5.2: Run coverage and verify >95%

- [ ] **Check coverage**

Run: `make coverage`

If new code in `canonical.rs` or `big_o.rs` has uncovered paths, add targeted tests.

- [ ] **Commit any coverage improvements**

```
test: improve coverage for canonical and big_o modules
```

#### Step 5.3: Final full check

- [ ] **Run the complete check suite**

Run: `make check`

All of: fmt, clippy, test must pass.

- [ ] **Run CLI demo**

Run: `make cli-demo`

Verify Big-O output looks correct in the CLI.

- [ ] **Test pred-sym manually**

```bash
cargo run -p problemreductions-cli --bin pred-sym -- canon 'n + n'
cargo run -p problemreductions-cli --bin pred-sym -- big-o 'n^3 - n^2 + 2*n + 4*n*m'
cargo run -p problemreductions-cli --bin pred-sym -- big-o '2^(n^0.5)'
cargo run -p problemreductions-cli --bin pred-sym -- eval 'n + m' --vars n=3,m=4
cargo run -p problemreductions-cli --bin pred-sym -- compare 'n + n' '2 * n'
```

Expected output:
```
2 * n
O(n^3 + m * n)     (or similar with n*m)
O(2^sqrt(n))
7
Exact equal: true, Big-O equal: true
```

- [ ] **Commit and push**

```
feat: complete asymptotic normal form redesign

Two-phase normalization pipeline:
- canonical_form(): exact symbolic simplification
- big_o_normal_form(): asymptotic projection
- pred-sym binary for symbolic engine inspection
- sqrt display for Pow(x, 0.5)
```
