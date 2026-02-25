# Overhead System Redesign

**Issue:** #61 — Introduce overhead system
**Date:** 2026-02-25
**Approach:** Macro-first dual emission

## Summary

Replace the current `Polynomial`-based overhead system with a general `Expr` AST, compile-time macro-parsed expression strings, and per-problem inherent getters. The proc macro emits both compiled Rust code (for evaluation + compiler validation) and symbolic `Expr` AST literals (for composition + export).

## Motivation

Three pain points with the current system:
1. **Ergonomics** — `problem_size_names()`/`problem_size_values()` parallel arrays are awkward; `poly!` macro is verbose
2. **Correctness** — variable name mismatches between overhead expressions and problem size fields are caught only at runtime
3. **Simplification** — `Polynomial` only supports sums of monomials; general math (exp, log) requires a new representation anyway

## Design

### 1. Expression AST (`Expr`)

Replaces `Polynomial` and `Monomial` with a general math expression tree.

```rust
// src/expr.rs (replaces src/polynomial.rs)

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Const(f64),
    Var(&'static str),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>),
    Log(Box<Expr>),
    Sqrt(Box<Expr>),
}
```

Key operations:
- `eval(&self, vars: &ProblemSize) -> f64`
- `substitute(&self, mapping: &HashMap<&str, &Expr>) -> Expr`
- `variables(&self) -> HashSet<&'static str>`
- `is_polynomial(&self) -> bool`
- `degree(&self) -> Option<u32>`
- `Display` for human-readable formulas
- `simplify(&self) -> Expr` — minimal constant folding

### 2. Problem Getters

Remove `problem_size_names()` and `problem_size_values()` from the `Problem` trait. Each problem type implements inherent getter methods instead.

```rust
// Before: trait methods returning parallel arrays
impl Problem for MaximumIndependentSet<SimpleGraph, i32> {
    fn problem_size_names() -> &'static [&'static str] { &["num_vertices", "num_edges"] }
    fn problem_size_values(&self) -> Vec<usize> {
        vec![self.graph().num_vertices(), self.graph().num_edges()]
    }
}

// After: inherent methods — natural, compiler-checked, IDE-friendly
impl<G: Graph, W: WeightElement> MaximumIndependentSet<G, W> {
    pub fn num_vertices(&self) -> usize { self.graph().num_vertices() }
    pub fn num_edges(&self) -> usize { self.graph().num_edges() }
}
```

### 3. Proc Macro — Dual Emission

The `#[reduction]` macro parses expression strings at compile time and emits two outputs.

User-facing syntax:
```rust
#[reduction(overhead = {
    num_vars = "num_vertices",
    num_constraints = "num_edges + num_vertices^2",
})]
impl ReduceTo<QUBO<f64>> for MaximumIndependentSet<SimpleGraph, i32> { ... }
```

Macro emits:
1. **Compiled evaluation function** — `src.num_vertices()`, `src.num_edges()` calls. Compiler catches missing getters.
2. **Symbolic Expr AST** — `Expr::Add(...)` construction for composition/export.

Expression grammar (Pratt parser, ~200 LOC in proc macro crate):
```
expr     = term (('+' | '-') term)*
term     = factor (('*' | '/') factor)*
factor   = base ('^' factor)?
base     = NUMBER | IDENT | func_call | '(' expr ')'
func_call = ('exp' | 'log' | 'sqrt') '(' expr ')'
```

### 4. Updated `ReductionOverhead` and `ReductionEntry`

```rust
pub struct ReductionOverhead {
    pub output_size: Vec<(&'static str, Expr)>,  // Expr replaces Polynomial
}

pub struct ReductionEntry {
    // ...existing fields...
    pub overhead_fn: fn() -> ReductionOverhead,       // symbolic (composition/export)
    pub overhead_eval_fn: fn(&dyn Any) -> ProblemSize, // compiled (evaluation)
    // REMOVED: source_size_names_fn, target_size_names_fn
}
```

`PathCostFn` uses the symbolic `ReductionOverhead` (via `Expr::eval`) since it operates on type-erased `ProblemSize` during graph traversal.

### 5. Export Pipeline

JSON format gains both structured AST and display string:
```json
{
  "overhead": [{
    "field": "num_vars",
    "expr": {"Pow": [{"Var": "num_vertices"}, {"Const": 2.0}]},
    "formula": "num_vertices^2"
  }]
}
```

The paper reads `formula` strings — no Typst code changes needed.

## Migration Strategy

| Phase | Description | Files | Risk |
|-------|-------------|-------|------|
| 1 | Add `Expr` type alongside `Polynomial` | 2-3 new | Low (additive) |
| 2 | Update proc macro with Pratt parser, support new syntax | 1 file | Medium |
| 3 | Add inherent getters to all problem types | ~15 model files | Low (additive) |
| 4 | Migrate all reductions to new syntax | ~20 rule files | Low (mechanical) |
| 5 | Remove deprecated APIs (`problem_size_*`, `Polynomial`, `poly!`) | ~10 files | Medium (breaking) |
| 6 | Update documentation and regenerate exports | 3-4 files | Low |

Phases 1-3 are purely additive. Phase 4 is bulk migration. Phase 5 is cleanup.
