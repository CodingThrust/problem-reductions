# Asymptotic Normal Form Redesign

**Date:** 2026-03-11
**Approach:** Explicit exact-canonicalization layer plus Big-O projection

## Summary

Redesign expression normalization into two explicit phases:

1. `canonical_form(expr)` performs exact symbolic simplification with signed coefficients.
2. `big_o_normal_form(expr)` projects the canonical expression into an asymptotic growth class.
3. `pred-sym` exposes the symbolic engine directly from a separate test-oriented binary for debugging and regression reproduction.

The existing `asymptotic_normal_form(expr)` remains temporarily as a compatibility wrapper to `big_o_normal_form(expr)`.

This change fixes three current pain points:

- duplicate additive terms survive composition (`O(x + x)` instead of `O(x)`)
- exact formulas with negative coefficients fail normalization and fall back to raw display
- `Pow(_, 0.5)` renders ambiguously in user-facing output (`2^n^0.5`)

## Current Status (2026-03-12)

Design is complete, but the redesign itself is not implemented yet.

Observed branch status:

- `src/expr.rs` still exposes only the legacy `asymptotic_normal_form()` pipeline.
- `canonical_form()` and `big_o_normal_form()` do not exist yet.
- `problemreductions-cli/src/commands/graph.rs` still formats Big-O through `asymptotic_normal_form()`.
- `problemreductions-cli/Cargo.toml` still exposes only the `pred` binary; `pred-sym` has not been added.
- `src/unit_tests/expr.rs` still encodes legacy behavior such as rejecting negative forms.
- The working tree already contains local WIP in `src/expr.rs`, related tests, and CLI files. That WIP should be reviewed before broad refactors so we do not overwrite useful progress.

One prerequisite has already landed separately: the CLI fallback now always wraps non-normalizable expressions in `O(...)`. This plan assumes that fix stays as-is while the engine is redesigned underneath it.

## Problems With The Current Design

The current `asymptotic_normal_form()` in `src/expr.rs` tries to do two jobs at once:

- algebraic simplification
- asymptotic reduction

That conflation makes the function brittle. It rejects subtraction because negative coefficients are treated as unsupported, even when the overall expression is a valid exact size formula. It also deduplicates only partially because nested additive structure is flattened incompletely. As a result, the function is too strict for exact formulas and too weak as a canonicalizer.

The public API also hides the distinction between exact symbolic equivalence and Big-O equivalence. That makes callers and tests harder to reason about and pushes formatting work into ad hoc fallbacks.

## Goals

- Preserve exact symbolic structure before asymptotic dropping.
- Support signed polynomial-style expressions such as `n^3 - n^2 + 2*n + 4*n*m`.
- Keep current safe transcendental identities:
  - `sqrt(x)` and `x^(1/2)`
  - `exp(a) * exp(b)`
  - `c^a * c^b` for constant positive `c`
  - simple `log` reductions already supported today
- Produce deterministic canonical ASTs and deterministic display order.
- Make Big-O output conservative: only drop terms when dominance is provable.
- Provide a lightweight separate binary for symbolic normalization testing without going through the full graph explorer.

## Non-Goals

- Build a general computer algebra system.
- Prove arbitrary symbolic inequalities across `exp`, `log`, and nested non-polynomial forms.
- Change the `Expr` surface syntax or parser grammar.
- Remove `asymptotic_normal_form()` immediately.

## Public API

Add two new public functions in `src/expr.rs` and re-export them from `src/lib.rs`:

```rust
pub fn canonical_form(expr: &Expr) -> Result<Expr, CanonicalizationError>;
pub fn big_o_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError>;
```

`asymptotic_normal_form(expr)` remains public for compatibility but becomes a thin wrapper:

```rust
pub fn asymptotic_normal_form(expr: &Expr) -> Result<Expr, AsymptoticAnalysisError> {
    big_o_normal_form(expr)
}
```

This makes the contract explicit:

- `canonical_form()` means exact symbolic normalization
- `big_o_normal_form()` means asymptotic projection
- `asymptotic_normal_form()` means legacy alias

## Companion CLI Tool

Add a second binary to `problemreductions-cli`:

```toml
[[bin]]
name = "pred-sym"
path = "src/bin/pred-sym.rs"
```

`pred-sym` is a focused test/developer tool for the symbolic engine. It should remain a separate binary, share parsing and formatting helpers with `pred` where practical, and keep its command surface narrow.

Recommended commands:

- `pred-sym parse '<expr>'`
  - parse and echo the raw AST / formula
- `pred-sym canon '<expr>'`
  - run `canonical_form()`
- `pred-sym big-o '<expr>'`
  - run `big_o_normal_form()`
- `pred-sym compare '<expr-a>' '<expr-b>'`
  - compare exact canonical forms and, optionally, Big-O-normal forms
- `pred-sym eval '<expr>' --vars n=10,m=20`
  - evaluate expressions against a `ProblemSize`-style variable map

Output modes:

- human-readable text by default
- `--json` for ASTs, canonical forms, and normalization results

This tool serves three purposes:

- reproducible debugging for normalization bugs
- easier test-case authoring during engine development
- isolated testing of symbolic behavior independent of reduction metadata

Design choice: prefer a separate binary over a `pred sym ...` subcommand. `pred-sym` is mainly a testing harness for the symbolic engine, so keeping it outside the main `pred` command surface avoids mixing developer/test workflows into the user-facing reduction explorer.

## Canonicalization Model

`canonical_form()` should normalize through a small internal algebra layer rather than by rewriting the AST directly.

Recommended internal model:

- a canonical sum of terms
- each term contains:
  - one signed numeric coefficient
  - a canonical multiset of multiplicative factors

For polynomial pieces, factors collapse into monomials such as `n^3 * m`. For supported non-polynomial pieces, the canonicalized subexpression is treated as an opaque factor atom, for example:

- `exp(m + n)`
- `2^(m + n)`
- `log(n)`
- `n^0.5`

This is intentionally bounded. It provides exact cancellation and deduplication without requiring general symbolic theorem proving.

## Canonicalization Rules

### Addition

- Flatten nested `Add`
- Canonicalize children first
- Merge equal factor-multisets by summing coefficients
- Drop zero-coefficient terms
- Sort terms deterministically

Examples:

- `n + n` -> `2 * n`
- `n - n` -> `0`
- `n + n - m + 2 * m` -> `2 * n + m`

### Multiplication

- Flatten nested `Mul`
- Multiply numeric coefficients
- Merge repeated factors into powers
- Preserve current safe identities:
  - `exp(a) * exp(b)` -> `exp(a + b)`
  - `2^a * 2^b` -> `2^(a + b)`
  - `sqrt(x)` -> `x^0.5`
- Sort factors deterministically

Examples:

- `n * n^(1/2)` -> `n^1.5`
- `2^n * 2^m` -> `2^(m + n)`

### Powers And Logs

- Constant folding remains allowed where safe
- Negative exponents remain unsupported for Big-O purposes because they imply division
- Existing simple `log` identities stay:
  - `log(exp(x)) -> x`
  - `log(c^x) -> x` for constant positive `c`
  - `log(x^k) -> log(x)` for constant positive `k`

## Big-O Projection

`big_o_normal_form()` operates only on canonical output.

### Additive Dominance

For sums, keep only terms not dominated by another term.

Dominance rule:

- use the existing conservative monomial comparison model from `src/rules/analysis.rs`
- drop a term only when dominance is provable
- keep incomparable terms

Examples:

- `n^3 + n^2` -> `n^3`
- `n^2 + n * m` -> `n^2 + n * m`
- `n^3 - n^2 + 2 * n + 4 * n * m` -> `n^3 + n * m`

Negative lower-order terms disappear naturally because they do not contribute to the dominant positive growth class.

### Multiplicative Projection

- discard overall nonzero constant factors
- preserve canonical symbolic products otherwise
- do not perform extra “improvement” beyond canonicalization

Examples:

- `3 * n^2` -> `n^2`
- `-4 * n^2` should be rejected as an asymptotic growth form unless embedded in a larger expression with surviving positive dominant terms

### Fallback Policy

For supported but incomparable opaque terms, keep all survivors rather than erroring.

Examples:

- `exp(n) + n^10` stays as `exp(n) + n^10` if no safe dominance rule exists
- `2^sqrt(n) + n^3` stays as both terms unless the comparison logic is extended

Errors should be reserved for genuinely invalid asymptotic outputs:

- division / negative exponents
- expressions with no surviving nonzero growth term
- sign-only or zero-only results that cannot be represented meaningfully as Big-O

## Display Rules

User-facing formatting should avoid precedence ambiguity. The preferred output for `Pow(base, Const(0.5))` is `sqrt(base)` when the exponent is exactly `0.5`.

This can be implemented either:

- in `Display for Expr`, or
- in a Big-O-specific formatter used by the CLI

Recommendation: keep canonical ASTs numeric (`0.5`) and improve display formatting. That preserves algebraic uniformity while fixing the ambiguous `O(2^num_vertices^0.5)` output.

## Implementation Plan

### Phase 0: Reconcile Current WIP

**Status:** In progress

Before changing architecture, review the existing local modifications in:

- `src/expr.rs`
- `src/unit_tests/expr.rs`
- `src/unit_tests/rules/analysis.rs`
- `problemreductions-cli/src/commands/graph.rs`
- adjacent CLI/test files already modified in the working tree

Goal:

- identify which edits are unrelated and should be preserved as-is
- identify any edits that already move toward this redesign
- avoid rewriting user work accidentally

Exit criteria:

- current local changes categorized as either “keep and adapt” or “leave untouched”
- baseline compile/test command chosen before structural work begins

### Phase 1: Add Exact Canonicalization

**Status:** Not started

Implement `canonical_form()` and its internal term model in `src/expr.rs`.

Scope:

- exact signed coefficients
- additive flattening and deduplication
- multiplicative flattening and power merging
- deterministic rebuild to `Expr`
- bounded transcendental identities already described above

Files:

- `src/expr.rs`
- `src/lib.rs`
- `src/unit_tests/expr.rs`

Exit criteria:

- `canonical_form()` exists and is exported
- exact-form tests pass
- legacy `asymptotic_normal_form()` behavior is unchanged for existing callers

### Phase 2: Add Big-O Projection

**Status:** Not started

Implement `big_o_normal_form()` on top of `canonical_form()`, then convert `asymptotic_normal_form()` into a compatibility wrapper.

Scope:

- dominant-term extraction for provable polynomial dominance
- conservative fallback for incomparable opaque terms
- rejection of invalid negative-power / division forms

Files:

- `src/expr.rs`
- `src/lib.rs`
- `src/unit_tests/expr.rs`

Exit criteria:

- `big_o_normal_form()` exists
- duplicate additive terms collapse correctly
- signed polynomial exact counts produce useful Big-O output
- `asymptotic_normal_form()` delegates to the new implementation

### Phase 3: Integrate Callers And Display

**Status:** Not started

Update current call sites to use the explicit APIs and fix ambiguous formatting of `^0.5`.

Scope:

- switch CLI formatting to `big_o_normal_form()`
- decide whether `sqrt(...)` rendering lives in `Display` or a Big-O formatter
- update rule-analysis preparation paths to use `canonical_form()` and/or `big_o_normal_form()` intentionally

Files:

- `problemreductions-cli/src/commands/graph.rs`
- `src/rules/analysis.rs`
- `src/expr.rs`
- affected tests

Exit criteria:

- CLI no longer depends on the legacy name internally
- no ambiguous `2^n^0.5`-style output remains
- analysis code uses the new API deliberately rather than via legacy fallback

### Phase 4: Add `pred-sym`

**Status:** Not started

Add the separate test-oriented binary for symbolic-engine inspection.

Scope:

- `parse`
- `canon`
- `big-o`
- `compare`
- `eval`
- optional `--json`

Files:

- `problemreductions-cli/Cargo.toml`
- `problemreductions-cli/src/bin/pred-sym.rs`
- shared helper modules if needed
- CLI tests

Exit criteria:

- `pred-sym` builds as a separate binary
- symbolic engine behavior can be reproduced without invoking `pred show`
- CLI output is stable enough for regression tests

### Phase 5: Expand And Rebaseline Tests

**Status:** Not started

Split tests by layer and replace obsolete expectations from the old engine.

Scope:

- exact canonicalization tests
- Big-O projection tests
- CLI regression tests for `pred`
- CLI tests for `pred-sym`

Exit criteria:

- negative-form tests reflect the new split API instead of the old monolithic rejection model
- ILP/TSP/composed-overhead regression cases are covered
- old expectations that rely on accidental formatting are removed

## Test Matrix

### Canonical Form

- flatten nested sums and products
- combine duplicate additive terms
- combine repeated multiplicative factors
- preserve signed exact formulas
- cancel zero terms
- preserve deterministic order
- preserve current transcendental identities

Key cases:

- `n + n` -> `2 * n`
- `n - n` -> `0`
- `n + n - m + 2 * m` -> `2 * n + m`
- `n * n^(1/2)` -> `n^1.5`
- `2^n * 2^m` -> `2^(m + n)`
- `sqrt(n * m)` canonicalizes equivalently to `(n * m)^(1/2)`
- `n^3 - n^2 + 2 * n + 4 * n * m` remains exact

### Big-O Normal Form

- duplicate composed terms collapse
- lower-order positive terms drop
- lower-order negative terms drop
- incomparable dominant terms remain
- invalid negative-power forms still error

Key cases:

- `n + n` -> `n`
- `n^3 - n^2 + 2 * n + 4 * n * m` -> `n^3 + n * m`
- `(n + m) + (m + n)` -> `m + n`
- `2^(sqrt(n))` displays clearly as `2^sqrt(n)` or `2^(sqrt(n))`

### CLI Regressions

- ILP complexity always renders as `O(...)`
- TSP `num_constraints` shows a simplified dominant Big-O expression
- composed reduction overheads do not show duplicate additive terms
- no ambiguous `^0.5` formatting in Big-O output

### `pred-sym` CLI

- `pred-sym canon 'n + n'` prints `2 * n`
- `pred-sym big-o 'n^3 - n^2 + 2*n + 4*n*m'` prints `n^3 + n * m`
- `pred-sym big-o '2^(n^0.5)'` renders using `sqrt(n)`
- `pred-sym eval 'n + m' --vars n=3,m=4` prints `7`
- `pred-sym --json ...` returns stable machine-readable output

## Risks And Tradeoffs

- `canonical_form()` adds internal complexity compared with the current tree-rewrite approach.
- Reusing the monomial-dominance logic from `src/rules/analysis.rs` reduces duplication conceptually, but care is needed to avoid circular abstractions or subtly different semantics.
- Keeping `asymptotic_normal_form()` as an alias avoids breakage, but it also extends the migration window. A later cleanup should deprecate it formally.

## Recommendation

Implement the redesign as an explicit two-phase pipeline:

1. exact symbolic canonicalization
2. conservative Big-O projection

This is the smallest design that fixes the known bugs while giving the expression system a clear contract for future work.
