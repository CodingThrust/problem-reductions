# Symbolic Expression System — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace `Polynomial`/`Monomial`/`poly!` with a general-purpose symbolic expression DSL supporting exponentials, logs, min/max, floor/ceil.

**Architecture:** New `src/expr.rs` module with AST, Pratt parser, evaluator, and Display. `ReductionOverhead` switches from `Polynomial` to `Expr` parsed from string literals. All 30 reduction files migrate from `poly!()` to string specs.

**Tech Stack:** Pure Rust, no new dependencies. Pratt parser hand-written.

**Design doc:** `docs/plans/2026-02-15-symbolic-expr-design.md`

---

### Task 1: Core AST and Evaluator

**Files:**
- Create: `src/expr.rs`
- Create: `src/unit_tests/expr.rs`

**Step 1: Write failing tests for AST construction and evaluation**

Create `src/unit_tests/expr.rs`:

```rust
use super::*;
use crate::types::ProblemSize;

#[test]
fn test_eval_num() {
    let expr = Expr::Num(42.0);
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 42.0);
}

#[test]
fn test_eval_var() {
    let expr = Expr::Var("n".into());
    let size = ProblemSize::new(vec![("n", 10)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 10.0);
}

#[test]
fn test_eval_unknown_var() {
    let expr = Expr::Var("missing".into());
    let size = ProblemSize::new(vec![]);
    assert!(matches!(expr.evaluate(&size), Err(EvalError::UnknownVar(_))));
}

#[test]
fn test_eval_add() {
    let expr = Expr::binop(BinOp::Add, Expr::Num(3.0), Expr::Num(4.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 7.0);
}

#[test]
fn test_eval_sub() {
    let expr = Expr::binop(BinOp::Sub, Expr::Num(10.0), Expr::Num(3.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 7.0);
}

#[test]
fn test_eval_mul() {
    let expr = Expr::binop(BinOp::Mul, Expr::Num(3.0), Expr::Var("n".into()));
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 15.0);
}

#[test]
fn test_eval_div() {
    let expr = Expr::binop(BinOp::Div, Expr::Num(10.0), Expr::Num(4.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 2.5);
}

#[test]
fn test_eval_div_by_zero() {
    let expr = Expr::binop(BinOp::Div, Expr::Num(1.0), Expr::Num(0.0));
    let size = ProblemSize::new(vec![]);
    assert!(matches!(expr.evaluate(&size), Err(EvalError::DivideByZero)));
}

#[test]
fn test_eval_pow() {
    let expr = Expr::binop(BinOp::Pow, Expr::Num(2.0), Expr::Num(10.0));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 1024.0);
}

#[test]
fn test_eval_pow_fractional_base_negative() {
    // negative base with non-integer exponent -> domain error
    let expr = Expr::binop(BinOp::Pow, Expr::Num(-2.0), Expr::Num(0.5));
    let size = ProblemSize::new(vec![]);
    assert!(matches!(expr.evaluate(&size), Err(EvalError::Domain { .. })));
}

#[test]
fn test_eval_neg() {
    let expr = Expr::Neg(Box::new(Expr::Num(5.0)));
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), -5.0);
}

#[test]
fn test_eval_log2() {
    let expr = Expr::Call { func: Func::Log2, args: vec![Expr::Num(8.0)] };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 3.0);
}

#[test]
fn test_eval_log2_negative() {
    let expr = Expr::Call { func: Func::Log2, args: vec![Expr::Num(-1.0)] };
    let size = ProblemSize::new(vec![]);
    assert!(matches!(expr.evaluate(&size), Err(EvalError::Domain { .. })));
}

#[test]
fn test_eval_sqrt() {
    let expr = Expr::Call { func: Func::Sqrt, args: vec![Expr::Num(25.0)] };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 5.0);
}

#[test]
fn test_eval_min() {
    let expr = Expr::Call { func: Func::Min, args: vec![Expr::Num(3.0), Expr::Num(7.0)] };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 3.0);
}

#[test]
fn test_eval_max() {
    let expr = Expr::Call { func: Func::Max, args: vec![Expr::Num(3.0), Expr::Num(7.0)] };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 7.0);
}

#[test]
fn test_eval_floor() {
    let expr = Expr::Call { func: Func::Floor, args: vec![Expr::Num(3.7)] };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 3.0);
}

#[test]
fn test_eval_ceil() {
    let expr = Expr::Call { func: Func::Ceil, args: vec![Expr::Num(3.2)] };
    let size = ProblemSize::new(vec![]);
    assert_eq!(expr.evaluate(&size).unwrap(), 4.0);
}

#[test]
fn test_eval_arity_error() {
    let expr = Expr::Call { func: Func::Log2, args: vec![Expr::Num(1.0), Expr::Num(2.0)] };
    let size = ProblemSize::new(vec![]);
    assert!(matches!(expr.evaluate(&size), Err(EvalError::Arity { .. })));
}

#[test]
fn test_eval_complex() {
    // 3 * n ^ 2 + 1.44 ^ m
    let expr = Expr::binop(
        BinOp::Add,
        Expr::binop(BinOp::Mul, Expr::Num(3.0), Expr::binop(BinOp::Pow, Expr::Var("n".into()), Expr::Num(2.0))),
        Expr::binop(BinOp::Pow, Expr::Num(1.44), Expr::Var("m".into())),
    );
    let size = ProblemSize::new(vec![("n", 4), ("m", 3)]);
    let result = expr.evaluate(&size).unwrap();
    let expected = 3.0 * 16.0 + 1.44_f64.powi(3);
    assert!((result - expected).abs() < 1e-10);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test expr --lib`
Expected: compilation error (module doesn't exist)

**Step 3: Implement AST types and evaluator**

Create `src/expr.rs` with:

```rust
//! Symbolic expression system for reduction overhead.
//!
//! Provides a DSL for expressing how problem sizes transform during reductions.
//! Supports arithmetic, exponentiation, and built-in math functions.

use crate::types::ProblemSize;
use std::fmt;

/// A symbolic expression over named variables.
#[derive(Clone, Debug)]
pub enum Expr {
    /// Numeric literal.
    Num(f64),
    /// Named variable (e.g., `num_vertices`).
    Var(Box<str>),
    /// Binary operation.
    BinOp { op: BinOp, lhs: Box<Expr>, rhs: Box<Expr> },
    /// Unary negation.
    Neg(Box<Expr>),
    /// Built-in function call.
    Call { func: Func, args: Vec<Expr> },
}

/// Binary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp { Add, Sub, Mul, Div, Pow }

/// Built-in functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Func { Log2, Log10, Ln, Exp, Sqrt, Min, Max, Floor, Ceil, Abs }

/// Evaluation error.
#[derive(Debug)]
pub enum EvalError {
    UnknownVar(Box<str>),
    DivideByZero,
    Arity { func: Func, expected: usize, got: usize },
    Domain { func: Func, detail: Box<str> },
}

impl Expr {
    /// Convenience constructor for binary operations.
    pub fn binop(op: BinOp, lhs: Expr, rhs: Expr) -> Self {
        Expr::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }

    /// Evaluate the expression given variable bindings from `ProblemSize`.
    pub fn evaluate(&self, size: &ProblemSize) -> Result<f64, EvalError> {
        match self {
            Expr::Num(v) => Ok(*v),
            Expr::Var(name) => size
                .get(name)
                .map(|v| v as f64)
                .ok_or_else(|| EvalError::UnknownVar(name.clone())),
            Expr::Neg(inner) => Ok(-inner.evaluate(size)?),
            Expr::BinOp { op, lhs, rhs } => {
                let l = lhs.evaluate(size)?;
                let r = rhs.evaluate(size)?;
                match op {
                    BinOp::Add => Ok(l + r),
                    BinOp::Sub => Ok(l - r),
                    BinOp::Mul => Ok(l * r),
                    BinOp::Div => {
                        if r == 0.0 { return Err(EvalError::DivideByZero); }
                        Ok(l / r)
                    }
                    BinOp::Pow => {
                        if l < 0.0 && r.fract() != 0.0 {
                            return Err(EvalError::Domain {
                                func: Func::Sqrt, // closest built-in
                                detail: "negative base with non-integer exponent".into(),
                            });
                        }
                        let result = l.powf(r);
                        if result.is_nan() || result.is_infinite() {
                            return Err(EvalError::Domain {
                                func: Func::Exp,
                                detail: format!("{l} ^ {r} produced non-finite result").into(),
                            });
                        }
                        Ok(result)
                    }
                }
            }
            Expr::Call { func, args } => eval_func(*func, args, size),
        }
    }
}

fn eval_func(func: Func, args: &[Expr], size: &ProblemSize) -> Result<f64, EvalError> {
    // Check arity
    let (min_args, max_args) = match func {
        Func::Min | Func::Max => (2, 2),
        _ => (1, 1),
    };
    if args.len() < min_args || args.len() > max_args {
        return Err(EvalError::Arity { func, expected: min_args, got: args.len() });
    }

    let a = args[0].evaluate(size)?;
    match func {
        Func::Log2 => {
            if a <= 0.0 { return Err(EvalError::Domain { func, detail: "log2 of non-positive".into() }); }
            Ok(a.log2())
        }
        Func::Log10 => {
            if a <= 0.0 { return Err(EvalError::Domain { func, detail: "log10 of non-positive".into() }); }
            Ok(a.log10())
        }
        Func::Ln => {
            if a <= 0.0 { return Err(EvalError::Domain { func, detail: "ln of non-positive".into() }); }
            Ok(a.ln())
        }
        Func::Exp => {
            let result = a.exp();
            if result.is_infinite() {
                return Err(EvalError::Domain { func, detail: "exp overflow".into() });
            }
            Ok(result)
        }
        Func::Sqrt => {
            if a < 0.0 { return Err(EvalError::Domain { func, detail: "sqrt of negative".into() }); }
            Ok(a.sqrt())
        }
        Func::Abs => Ok(a.abs()),
        Func::Floor => Ok(a.floor()),
        Func::Ceil => Ok(a.ceil()),
        Func::Min => { let b = args[1].evaluate(size)?; Ok(a.min(b)) }
        Func::Max => { let b = args[1].evaluate(size)?; Ok(a.max(b)) }
    }
}
```

**Step 4: Wire up the module and test file**

In `src/expr.rs`, add at the bottom:
```rust
#[cfg(test)]
#[path = "unit_tests/expr.rs"]
mod tests;
```

In `src/lib.rs`, add: `pub mod expr;` (keep `polynomial` for now — we'll remove it later).

**Step 5: Run tests to verify they pass**

Run: `cargo test expr --lib`
Expected: all tests pass

**Step 6: Commit**

```bash
git add src/expr.rs src/unit_tests/expr.rs src/lib.rs
git commit -m "feat: add symbolic expression AST and evaluator"
```

---

### Task 2: Parser (Tokenizer + Pratt)

**Files:**
- Modify: `src/expr.rs`
- Modify: `src/unit_tests/expr.rs`

**Step 1: Write failing parser tests**

Add to `src/unit_tests/expr.rs`:

```rust
#[test]
fn test_parse_num() {
    let expr = Expr::parse("42").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 42.0);
}

#[test]
fn test_parse_float() {
    let expr = Expr::parse("1.44").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 1.44);
}

#[test]
fn test_parse_var() {
    let expr = Expr::parse("num_vertices").unwrap();
    let size = ProblemSize::new(vec![("num_vertices", 10)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 10.0);
}

#[test]
fn test_parse_add() {
    let expr = Expr::parse("3 + 4").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 7.0);
}

#[test]
fn test_parse_precedence_mul_add() {
    // 2 + 3 * 4 = 14 (not 20)
    let expr = Expr::parse("2 + 3 * 4").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 14.0);
}

#[test]
fn test_parse_precedence_pow() {
    // 2 ^ 3 ^ 2 = 2 ^ 9 = 512 (right-associative)
    let expr = Expr::parse("2 ^ 3 ^ 2").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 512.0);
}

#[test]
fn test_parse_unary_neg() {
    let expr = Expr::parse("-5").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), -5.0);
}

#[test]
fn test_parse_neg_pow() {
    // -2 ^ 2 = -(2^2) = -4
    let expr = Expr::parse("-2 ^ 2").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), -4.0);
}

#[test]
fn test_parse_parens() {
    let expr = Expr::parse("(2 + 3) * 4").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 20.0);
}

#[test]
fn test_parse_function_log2() {
    let expr = Expr::parse("log2(8)").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 3.0);
}

#[test]
fn test_parse_function_max() {
    let expr = Expr::parse("max(3, 7)").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 7.0);
}

#[test]
fn test_parse_function_case_insensitive() {
    let expr = Expr::parse("Log2(8)").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 3.0);
}

#[test]
fn test_parse_complex_expression() {
    // 3 * num_vertices ^ 2 + 1.44 ^ num_edges
    let expr = Expr::parse("3 * num_vertices ^ 2 + 1.44 ^ num_edges").unwrap();
    let size = ProblemSize::new(vec![("num_vertices", 4), ("num_edges", 3)]);
    let expected = 3.0 * 16.0 + 1.44_f64.powi(3);
    assert!((expr.evaluate(&size).unwrap() - expected).abs() < 1e-10);
}

#[test]
fn test_parse_nested_functions() {
    let expr = Expr::parse("floor(log2(16))").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 4.0);
}

#[test]
fn test_parse_unknown_function() {
    let result = Expr::parse("foo(3)");
    assert!(matches!(result, Err(ParseError::UnknownFunction { .. })));
}

#[test]
fn test_parse_unexpected_eof() {
    let result = Expr::parse("3 +");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty() {
    let result = Expr::parse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_leading_dot() {
    let expr = Expr::parse(".5").unwrap();
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 0.5);
}

#[test]
fn test_parse_subtraction() {
    let expr = Expr::parse("10 - 3 - 2").unwrap();
    // left-associative: (10 - 3) - 2 = 5
    assert_eq!(expr.evaluate(&ProblemSize::new(vec![])).unwrap(), 5.0);
}

#[test]
fn test_parse_division() {
    let expr = Expr::parse("num_vertices / 2").unwrap();
    let size = ProblemSize::new(vec![("num_vertices", 10)]);
    assert_eq!(expr.evaluate(&size).unwrap(), 5.0);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test expr --lib`
Expected: `Expr::parse` doesn't exist

**Step 3: Implement tokenizer and parser**

Add to `src/expr.rs`:

- `Span` struct
- `ParseError` enum
- `Token` enum (private)
- `Lexer` struct (private) — iterates chars, produces tokens with spans
- `Parser` struct (private) — Pratt parser consuming tokens
- `Expr::parse(input: &str) -> Result<Expr, ParseError>` public entry point

The Pratt parser uses binding powers:
- `+`, `-`: left bp = 1, right bp = 2
- `*`, `/`: left bp = 3, right bp = 4
- prefix `-`: right bp = 5
- `^`: left bp = 7, right bp = 6 (right-assoc: left > right)

Function names resolved via a match on the lowercased ident.

**Step 4: Run tests to verify they pass**

Run: `cargo test expr --lib`
Expected: all pass

**Step 5: Commit**

```bash
git add src/expr.rs src/unit_tests/expr.rs
git commit -m "feat: add expression parser with Pratt precedence climbing"
```

---

### Task 3: Display and Serde

**Files:**
- Modify: `src/expr.rs`
- Modify: `src/unit_tests/expr.rs`

**Step 1: Write failing tests for Display and round-tripping**

Add to `src/unit_tests/expr.rs`:

```rust
#[test]
fn test_display_num_integer() {
    let expr = Expr::Num(3.0);
    assert_eq!(expr.to_string(), "3");
}

#[test]
fn test_display_num_float() {
    let expr = Expr::Num(1.44);
    assert_eq!(expr.to_string(), "1.44");
}

#[test]
fn test_display_var() {
    let expr = Expr::Var("num_vertices".into());
    assert_eq!(expr.to_string(), "num_vertices");
}

#[test]
fn test_display_add() {
    let expr = Expr::parse("a + b").unwrap();
    assert_eq!(expr.to_string(), "a + b");
}

#[test]
fn test_display_precedence() {
    let expr = Expr::parse("a + b * c").unwrap();
    assert_eq!(expr.to_string(), "a + b * c");
}

#[test]
fn test_display_parens_needed() {
    let expr = Expr::parse("(a + b) * c").unwrap();
    assert_eq!(expr.to_string(), "(a + b) * c");
}

#[test]
fn test_display_pow() {
    let expr = Expr::parse("1.44 ^ n").unwrap();
    assert_eq!(expr.to_string(), "1.44 ^ n");
}

#[test]
fn test_display_neg() {
    let expr = Expr::parse("-x").unwrap();
    assert_eq!(expr.to_string(), "-x");
}

#[test]
fn test_display_neg_compound() {
    let expr = Expr::parse("-(a + b)").unwrap();
    assert_eq!(expr.to_string(), "-(a + b)");
}

#[test]
fn test_display_func() {
    let expr = Expr::parse("log2(n)").unwrap();
    assert_eq!(expr.to_string(), "log2(n)");
}

#[test]
fn test_display_func_two_args() {
    let expr = Expr::parse("max(a, b)").unwrap();
    assert_eq!(expr.to_string(), "max(a, b)");
}

#[test]
fn test_roundtrip_complex() {
    let cases = vec![
        "3 * n ^ 2 + 1.44 ^ m",
        "log2(n) * m",
        "max(n, m) + 1",
        "floor(n / 2)",
        "-(a + b) * c",
        "a ^ b ^ c",
        "a - b - c",
    ];
    let size = ProblemSize::new(vec![("n", 4), ("m", 3), ("a", 2), ("b", 3), ("c", 5)]);
    for case in cases {
        let expr1 = Expr::parse(case).unwrap();
        let displayed = expr1.to_string();
        let expr2 = Expr::parse(&displayed).unwrap();
        let v1 = expr1.evaluate(&size).unwrap();
        let v2 = expr2.evaluate(&size).unwrap();
        assert!((v1 - v2).abs() < 1e-10, "Round-trip failed for {case}: displayed as {displayed}");
    }
}

#[test]
fn test_serde_roundtrip() {
    let expr = Expr::parse("3 * n ^ 2 + 1").unwrap();
    let json = serde_json::to_string(&expr).unwrap();
    let back: Expr = serde_json::from_str(&json).unwrap();
    let size = ProblemSize::new(vec![("n", 5)]);
    assert_eq!(expr.evaluate(&size).unwrap(), back.evaluate(&size).unwrap());
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test expr --lib`
Expected: `Display` not implemented, serde not implemented

**Step 3: Implement Display**

Add `impl fmt::Display for Expr` to `src/expr.rs`:
- Use a helper that takes parent precedence context
- Parenthesize when child precedence < parent precedence
- For `^`, parenthesize LHS if same precedence (right-assoc)
- Integer floats display without decimal point
- Function names: lowercase canonical form

Also add `impl fmt::Display for EvalError` and `impl fmt::Display for ParseError`.

**Step 4: Implement Serde**

Add custom `Serialize`/`Deserialize` for `Expr`:
- `Serialize`: delegates to `Display` (serializes as string)
- `Deserialize`: calls `Expr::parse` (deserializes from string)

**Step 5: Run tests to verify they pass**

Run: `cargo test expr --lib`
Expected: all pass

**Step 6: Commit**

```bash
git add src/expr.rs src/unit_tests/expr.rs
git commit -m "feat: add Display and Serde for Expr (string round-trip)"
```

---

### Task 4: Migrate ReductionOverhead and Registry

**Files:**
- Modify: `src/rules/registry.rs`
- Modify: `src/unit_tests/rules/registry.rs`

**Step 1: Write updated tests**

Update `src/unit_tests/rules/registry.rs` — change from `poly!` to string specs:

```rust
use crate::rules::registry::ReductionOverhead;
use crate::types::ProblemSize;

#[test]
fn test_reduction_overhead_evaluate() {
    let overhead = ReductionOverhead::new(vec![("n", "3 * m"), ("m", "m ^ 2")]);
    let input = ProblemSize::new(vec![("m", 4)]);
    let output = overhead.evaluate_output_size(&input).unwrap();
    assert_eq!(output.get("n"), Some(12));
    assert_eq!(output.get("m"), Some(16));
}
```

Update the `ReductionEntry` test similarly — use `ReductionOverhead::new(vec![("n", "2 * n")])`.

**Step 2: Run tests to verify they fail**

Run: `cargo test registry --lib`
Expected: type mismatch (still expects `Polynomial`)

**Step 3: Update `src/rules/registry.rs`**

Replace:
```rust
use crate::polynomial::Polynomial;
```
with:
```rust
use crate::expr::Expr;
```

Change `ReductionOverhead`:
```rust
pub struct ReductionOverhead {
    pub output_size: Vec<(&'static str, Expr)>,
}

impl ReductionOverhead {
    pub fn new(specs: Vec<(&'static str, &'static str)>) -> Self {
        Self {
            output_size: specs
                .into_iter()
                .map(|(field, expr_str)| {
                    let expr = Expr::parse(expr_str).unwrap_or_else(|e| {
                        panic!("invalid overhead expression for '{field}': {e}")
                    });
                    (field, expr)
                })
                .collect(),
        }
    }

    pub fn evaluate_output_size(&self, input: &ProblemSize) -> Result<ProblemSize, crate::expr::EvalError> {
        let mut fields = Vec::new();
        for (name, expr) in &self.output_size {
            let val = expr.evaluate(input)?;
            let rounded = val.round();
            if !rounded.is_finite() || rounded < 0.0 || rounded > usize::MAX as f64 {
                return Err(crate::expr::EvalError::Domain {
                    func: crate::expr::Func::Floor,
                    detail: format!("overhead for '{name}' produced out-of-range value: {val}").into(),
                });
            }
            fields.push((*name, rounded as usize));
        }
        Ok(ProblemSize::new(fields))
    }
}
```

Keep `Default` impl producing empty `output_size`.

**Step 4: Run tests to verify they pass**

Run: `cargo test registry --lib`
Expected: pass

**Step 5: Commit**

```bash
git add src/rules/registry.rs src/unit_tests/rules/registry.rs
git commit -m "refactor: ReductionOverhead uses Expr parsed from strings"
```

---

### Task 5: Migrate Export System

**Files:**
- Modify: `src/export.rs`
- Modify: `src/unit_tests/export.rs`

**Step 1: Write updated tests**

Update `src/unit_tests/export.rs` — `overhead_to_json` now returns `Vec<OverheadEntry>` where each entry has `field` and `expression` (a string):

```rust
use crate::export::overhead_to_json;
use crate::rules::registry::ReductionOverhead;

#[test]
fn test_overhead_to_json_empty() {
    let overhead = ReductionOverhead::default();
    let entries = overhead_to_json(&overhead);
    assert!(entries.is_empty());
}

#[test]
fn test_overhead_to_json_single_field() {
    let overhead = ReductionOverhead::new(vec![("num_vertices", "n + m")]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].field, "num_vertices");
    assert_eq!(entries[0].expression, "n + m");
}

#[test]
fn test_overhead_to_json_multiple_fields() {
    let overhead = ReductionOverhead::new(vec![
        ("num_vertices", "n ^ 2"),
        ("num_edges", "1.44 ^ n"),
    ]);
    let entries = overhead_to_json(&overhead);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].expression, "n ^ 2");
    assert_eq!(entries[1].expression, "1.44 ^ n");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test export --lib`
Expected: `MonomialJson` still expected

**Step 3: Update `src/export.rs`**

Remove `MonomialJson`. Simplify `OverheadEntry`:
```rust
#[derive(Serialize, Clone, Debug)]
pub struct OverheadEntry {
    pub field: String,
    pub expression: String,
}
```

Simplify `overhead_to_json`:
```rust
pub fn overhead_to_json(overhead: &ReductionOverhead) -> Vec<OverheadEntry> {
    overhead
        .output_size
        .iter()
        .map(|(field, expr)| OverheadEntry {
            field: field.to_string(),
            expression: expr.to_string(),
        })
        .collect()
}
```

Also update `ReductionData.overhead` field type from `Vec<OverheadEntry>` — this should already work since `OverheadEntry` is still `Serialize`.

**Step 4: Run tests to verify they pass**

Run: `cargo test export --lib`
Expected: pass

**Step 5: Commit**

```bash
git add src/export.rs src/unit_tests/export.rs
git commit -m "refactor: simplify export overhead to expression strings"
```

---

### Task 6: Migrate Cost Functions

**Files:**
- Modify: `src/rules/cost.rs`
- Modify: `src/unit_tests/rules/cost.rs`

**Step 1: Update tests**

In `src/unit_tests/rules/cost.rs`, change the helper:
```rust
fn test_overhead() -> ReductionOverhead {
    ReductionOverhead::new(vec![
        ("n", "2 * n"),
        ("m", "m"),
    ])
}
```

Remove the `use crate::polynomial::Polynomial;` import.

Existing test assertions should still hold since the values are the same.

**Step 2: Run tests to verify they fail**

Run: `cargo test cost --lib`
Expected: fail on `Polynomial` import

**Step 3: Update `src/rules/cost.rs`**

`evaluate_output_size` now returns `Result`. For cost functions, we should unwrap since overhead expressions are known-good at this point. Change each call from:
```rust
overhead.evaluate_output_size(size).get(self.0).unwrap_or(0) as f64
```
to:
```rust
overhead.evaluate_output_size(size)
    .expect("overhead evaluation failed")
    .get(self.0).unwrap_or(0) as f64
```

Apply same pattern to `MinimizeWeighted`, `MinimizeMax`, `MinimizeLexicographic`.

**Step 4: Run tests to verify they pass**

Run: `cargo test cost --lib`
Expected: pass

**Step 5: Commit**

```bash
git add src/rules/cost.rs src/unit_tests/rules/cost.rs
git commit -m "refactor: migrate cost functions to Expr-based overhead"
```

---

### Task 7: Migrate Graph Module

**Files:**
- Modify: `src/rules/graph.rs`

**Step 1: Update evaluate_output_size call**

At line ~420, change:
```rust
let new_size = edge.overhead.evaluate_output_size(&current_size);
```
to:
```rust
let new_size = edge.overhead.evaluate_output_size(&current_size)
    .expect("overhead evaluation failed during path finding");
```

At line ~712, the `to_json` method uses `poly.to_string()` — since `Expr` also implements `Display`, this already works. Just verify the field name change: the `OverheadFieldJson.formula` field now comes from `Expr::to_string()` which should produce equivalent output.

**Step 2: Run: `cargo test graph --lib`**

Expected: pass (no behavioral change)

**Step 3: Commit**

```bash
git add src/rules/graph.rs
git commit -m "refactor: migrate graph module to Expr-based overhead"
```

---

### Task 8: Migrate All 30 Reduction Files (Simple Ones)

**Files:**
- Modify: 28 reduction files that use `poly!()` macro

These all follow the same mechanical transformation. The `poly!` expressions map to string literals:

| Old `poly!` syntax | New string literal |
|---|---|
| `poly!(num_vertices)` | `"num_vertices"` |
| `poly!(num_vertices ^ 2)` | `"num_vertices ^ 2"` |
| `poly!(3 * num_vars)` | `"3 * num_vars"` |
| `poly!(num_vertices * num_edges)` | `"num_vertices * num_edges"` |
| `poly!(3 * num_vars) + poly!(num_clauses)` | `"3 * num_vars + num_clauses"` |
| `poly!(num_clauses) + poly!(num_literals)` | `"num_clauses + num_literals"` |
| `poly!(num_clauses).scale(-5.0)` | `"-5 * num_clauses"` |
| `poly!(2 * num_vars) + poly!(5 * num_literals) + poly!(num_clauses).scale(-5.0) + poly!(3)` | `"2 * num_vars + 5 * num_literals - 5 * num_clauses + 3"` |

**Step 1: Migrate files alphabetically**

For each file, replace the `ReductionOverhead::new(vec![...])` body. Remove any `use crate::polynomial::Polynomial;` or `use crate::poly;` imports. The `#[reduction(overhead = { ... })]` wrapper stays the same.

Example — `src/rules/minimumvertexcover_maximumindependentset.rs`:

Old:
```rust
ReductionOverhead::new(vec![
    ("num_vertices", poly!(num_vertices)),
    ("num_edges", poly!(num_edges)),
])
```

New:
```rust
ReductionOverhead::new(vec![
    ("num_vertices", "num_vertices"),
    ("num_edges", "num_edges"),
])
```

**Step 2: Run `cargo test --lib` after each batch of ~5 files**

Expected: pass

**Step 3: Commit after all simple files**

```bash
git add src/rules/*.rs
git commit -m "refactor: migrate 28 reduction files from poly! to string expressions"
```

---

### Task 9: Migrate Complex Reduction Files

**Files:**
- Modify: `src/rules/travelingsalesman_ilp.rs`
- Modify: `src/rules/factoring_ilp.rs`

These two files construct `Polynomial`/`Monomial` structs directly.

**Step 1: Migrate `travelingsalesman_ilp.rs`**

Old (manual Monomial/Polynomial construction):
```rust
("num_vars", Polynomial::var_pow("num_vertices", 2) + Polynomial {
    terms: vec![Monomial {
        coefficient: 2.0,
        variables: vec![("num_vertices", 1), ("num_edges", 1)],
    }]
}),
("num_constraints", Polynomial::var_pow("num_vertices", 3) + Polynomial {
    terms: vec![
        Monomial { coefficient: -1.0, variables: vec![("num_vertices", 2)] },
        Monomial { coefficient: 2.0, variables: vec![("num_vertices", 1)] },
        Monomial { coefficient: 4.0, variables: vec![("num_vertices", 1), ("num_edges", 1)] },
    ]
}),
```

New:
```rust
("num_vars", "num_vertices ^ 2 + 2 * num_vertices * num_edges"),
("num_constraints", "num_vertices ^ 3 - num_vertices ^ 2 + 2 * num_vertices + 4 * num_vertices * num_edges"),
```

Remove `use crate::polynomial::{Monomial, Polynomial};`.

**Step 2: Migrate `factoring_ilp.rs`**

Old:
```rust
("num_vars", Polynomial { terms: vec![
    Monomial::var("num_bits_first").scale(2.0),
    Monomial::var("num_bits_second").scale(2.0),
    Monomial { coefficient: 1.0, variables: vec![("num_bits_first", 1), ("num_bits_second", 1)] },
] }),
("num_constraints", Polynomial { terms: vec![
    Monomial { coefficient: 3.0, variables: vec![("num_bits_first", 1), ("num_bits_second", 1)] },
    Monomial::var("num_bits_first"),
    Monomial::var("num_bits_second"),
    Monomial::constant(1.0),
] }),
```

New:
```rust
("num_vars", "2 * num_bits_first + 2 * num_bits_second + num_bits_first * num_bits_second"),
("num_constraints", "3 * num_bits_first * num_bits_second + num_bits_first + num_bits_second + 1"),
```

Remove `use crate::polynomial::{Monomial, Polynomial};`.

**Step 3: Run `cargo test --lib`**

Expected: pass

**Step 4: Commit**

```bash
git add src/rules/travelingsalesman_ilp.rs src/rules/factoring_ilp.rs
git commit -m "refactor: migrate complex reduction overheads to string expressions"
```

---

### Task 10: Delete Polynomial Module and poly! Macro

**Files:**
- Delete: `src/polynomial.rs`
- Delete: `src/unit_tests/polynomial.rs`
- Modify: `src/lib.rs` — remove `pub mod polynomial;`
- Modify: `src/rules/mod.rs` — remove any `poly!` re-export if present

**Step 1: Remove `pub mod polynomial;` from `src/lib.rs`**

**Step 2: Delete `src/polynomial.rs` and `src/unit_tests/polynomial.rs`**

**Step 3: Search for any remaining references**

Run: `grep -r "polynomial\|poly!" src/ --include="*.rs"`
Expected: no matches (only in test data or docs)

**Step 4: Run full test suite**

Run: `make test`
Expected: all pass

**Step 5: Commit**

```bash
git rm src/polynomial.rs src/unit_tests/polynomial.rs
git add src/lib.rs
git commit -m "refactor: remove Polynomial/Monomial/poly! (replaced by Expr)"
```

---

### Task 11: Update Examples

**Files:**
- Modify: all `examples/reduction_*.rs` files (~30 files)

These files call `overhead_to_json(&overhead)`. The function signature is unchanged, but the output format changed (from `MonomialJson` array to `expression` string). The `ReductionData` struct's `overhead` field type is `Vec<OverheadEntry>` which is still `Serialize`.

**Step 1: Verify examples compile**

Run: `cargo build --examples`
Expected: pass (no source changes needed — examples call `overhead_to_json` which still works)

If any example directly constructs `MonomialJson` or `Polynomial`, update it.

**Step 2: Regenerate example JSON outputs**

Run: `make examples`
Expected: JSON files regenerated with new `expression` string format

**Step 3: Verify tests**

Run: `make test`
Expected: all pass

**Step 4: Commit**

```bash
git add examples/ docs/paper/examples/
git commit -m "chore: regenerate example JSON with expression string format"
```

---

### Task 12: Run Full Verification

**Step 1: Format check**

Run: `make fmt-check`
Expected: pass

**Step 2: Clippy**

Run: `make clippy`
Expected: no warnings

**Step 3: Full test suite**

Run: `make test`
Expected: all pass

**Step 4: Build docs (includes reduction graph export)**

Run: `make doc`
Expected: builds successfully, reduction_graph.json regenerated with expression strings

**Step 5: Fix any issues found**

**Step 6: Final commit if needed**

```bash
git add -A
git commit -m "chore: fix formatting and clippy warnings"
```
