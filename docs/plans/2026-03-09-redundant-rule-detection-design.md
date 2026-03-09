# Redundant Rule Detection Utility

**Date:** 2026-03-09
**Issue:** #193

## Goal

Detect primitive reduction rules whose overhead is dominated (equal or worse) by a composite path through other rules. Prevents adding redundant rules and documents existing redundancies.

## Location

`src/rules/analysis.rs` — new module alongside `graph.rs`, `registry.rs`, `cost.rs`.

## Types

```rust
/// Asymptotic growth rate classification.
/// Ordered: Constant < Logarithmic < Polynomial(k) < Exponential(c) < SuperExponential.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum GrowthRate {
    Constant,
    Logarithmic,
    Polynomial(f64),   // degree k
    Exponential(f64),  // base c
    SuperExponential,
}

/// A primitive rule dominated by a composite path.
struct DominatedRule {
    source: String,
    target: String,
    primitive_overhead: ReductionOverhead,
    dominating_path: ReductionPath,
    composed_overhead: ReductionOverhead,
    comparison: Vec<(String, Ordering)>,  // per-field ordering
}
```

## Key Functions

### `Expr::growth_rate(var: &str) -> Result<GrowthRate, String>`

Recursively classify an expression's growth w.r.t. a single variable.

Rules (recursive — classify children first, then combine):

| Expression | Child classifications | Result |
|---|---|---|
| `Const(_)` | — | `Constant` |
| `Var(v)` where `v == var` | — | `Polynomial(1.0)` |
| `Var(v)` where `v != var` | — | `Constant` |
| `Add(a, b)` | any | `max(a, b)` |
| `Mul(a, b)` | both `Poly(k1, k2)` | `Polynomial(k1 + k2)` |
| `Mul(a, b)` | one `Exp`, one `Poly` | `Exponential(same base)` |
| `Mul(a, b)` | otherwise | `Error` |
| `Pow(base, exp)` | `Poly(k)`, `Const(c)` | `Polynomial(k * c)` |
| `Pow(base, exp)` | `Const(c)`, `Poly(k)` | `Exponential(c)` |
| `Pow(base, exp)` | `Const(c)`, `Logarithmic` | `Polynomial(c.ln())` |
| `Pow(base, exp)` | `Poly`, `Poly` | `SuperExponential` |
| `Pow(base, exp)` | otherwise | `Error` |
| `Exp(Log(inner))` | — | `growth_rate(inner)` (cancellation) |
| `Exp(inner)` | `Constant` | `Constant` |
| `Exp(inner)` | `Poly(k)`, k >= 1 | `Exponential(e)` |
| `Exp(inner)` | `Exponential` | `SuperExponential` |
| `Exp(inner)` | otherwise | `Error` |
| `Log(inner)` | `Constant` | `Constant` |
| `Log(inner)` | `Polynomial(_)` | `Logarithmic` |
| `Log(inner)` | `Exponential(c)` | `Polynomial(1.0)` |
| `Log(inner)` | otherwise | `Error` |
| `Sqrt(inner)` | `Polynomial(k)` | `Polynomial(k / 2.0)` |
| `Sqrt(inner)` | otherwise | `Error` |

Returns `Err` for unclassifiable expressions rather than guessing.

### `compare_overhead(primitive: &ReductionOverhead, composite: &ReductionOverhead) -> Option<Ordering>`

For each common field:
1. Collect all variables referenced by both expressions
2. For each variable, compute `growth_rate` of both expressions
3. Take the max growth rate across all variables for each expression
4. Compare: composite <= primitive on all fields → `Some(Equal | Less)`; otherwise `None`

### `find_dominated_rules(graph: &ReductionGraph) -> Vec<DominatedRule>`

1. Iterate all edges in the variant-level graph
2. For each edge (u, v), call `find_all_paths(u, v)`
3. Filter to composite paths (len > 1)
4. Compose overhead via `ReductionOverhead::compose`
5. Compare with `compare_overhead`
6. Collect dominated results

## Integration Test

In `src/unit_tests/rules/analysis.rs`:
- Call `find_dominated_rules` on the global `ReductionGraph`
- Maintain an allow-list of known dominated rules (currently 9: 6 genuine + 3 cast-composed)
- **Fail if a new dominated rule appears** that is not in the allow-list
- **Fail if an allow-listed rule is no longer dominated** (stale allow-list)

## Existing Infrastructure Used

- `ReductionGraph::find_all_paths()` — path enumeration
- `ReductionOverhead::compose()` — chains overheads via `Expr::substitute`
- `Expr::is_polynomial()` — partial classification (extended by `growth_rate`)
- `Expr::substitute()` — variable substitution for composition

## Known Dominated Rules (Allow-List)

| Primitive | Best Composite | Category |
|---|---|---|
| KColoring → QUBO | KColoring → ILP → QUBO | genuine |
| MIS → ILP | MIS → MVC → ILP | genuine |
| MIS → QUBO | MIS → MVC → QUBO | genuine |
| MaxSetPacking → ILP | SetPacking → MIS → ILP | genuine |
| MVC → ILP | MVC → MinSetCovering → ILP | genuine |
| MVC → QUBO | MVC → MIS → QUBO | genuine |
| KSAT/K2 → SAT | K2 → KN → SAT | cast-composed |
| KSAT/K3 → SAT | K3 → KN → SAT | cast-composed |
| MIS/One → MIS/Kings/i32 | One→i32 → Kings/i32 | cast-composed |
