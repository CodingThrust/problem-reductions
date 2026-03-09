# Redundant Rule Detection Utility

**Date:** 2026-03-09
**Issue:** #193

## Goal

Detect primitive reduction rules whose symbolic overhead is asymptotically no better than a composite path through other rules.

This analysis must be **sound but incomplete**:
- Report `dominated` only when the symbolic comparison is trustworthy
- Report `unknown` when the metadata is too weak to compare safely
- Never claim domination from incomplete metadata

## Scope

- Analyze exact variant-level reductions, not just base problem names
- Use current symbolic `ReductionOverhead` metadata where it is trustworthy
- Exclude or mark `unknown` any path that relies on incomplete symbolic metadata

## Non-Goal

This utility does **not** try to prove dominance for every path in the graph.

In particular, `ILP -> QUBO` is a valid reduction but its symbolic overhead is currently incomplete: the implementation adds slack bits based on constraint coefficients and right-hand sides, while the exposed symbolic size fields only include `num_vars` and `num_constraints`. That edge must therefore be treated as `unknown` for shortcut detection.

## Location

Add `src/rules/analysis.rs` and export it from `src/rules/mod.rs`.

As with other rule modules, wire tests via:

```rust
#[cfg(test)]
#[path = "../unit_tests/rules/analysis.rs"]
mod tests;
```

## Core Types

```rust
/// Exact identity of a primitive reduction rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RuleKey {
    source_name: &'static str,
    source_variant: BTreeMap<String, String>,
    target_name: &'static str,
    target_variant: BTreeMap<String, String>,
    module_path: &'static str,
}

/// Result of comparing one primitive rule against one composite path.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ComparisonStatus {
    Dominated,
    NotDominated,
    Unknown,
}

/// A primitive rule proven dominated by a composite path.
#[derive(Debug, Clone)]
struct DominatedRule {
    primitive: RuleKey,
    primitive_overhead: ReductionOverhead,
    dominating_path: ReductionPath,
    composed_overhead: ReductionOverhead,
    comparable_fields: Vec<String>,
}

/// A candidate comparison that could not be decided soundly.
#[derive(Debug, Clone)]
struct UnknownComparison {
    primitive: RuleKey,
    candidate_path: ReductionPath,
    reason: String,
}
```

## Trust Model

### 1. Primitive candidates come from `ReductionEntry`, not graph edges

The utility should iterate `inventory::iter::<ReductionEntry>` so each primitive rule keeps its own identity (`module_path`, exact source variant, exact target variant).

This avoids conflating:
- `KSatisfiability<K2> -> Satisfiability`
- `KSatisfiability<K3> -> Satisfiability`

which share the same base names but are distinct variant-level rules.

### 2. The reduction graph is used only for variant-level path enumeration

For a primitive rule `(source_variant, target_variant)`, use:

```rust
graph.find_all_paths(source_name, &source_variant, target_name, &target_variant)
```

and discard direct paths (`len() == 1`).

### 3. Comparisons are allowed only on trustworthy symbolic paths

Add:

```rust
fn path_is_symbolically_trustworthy(path: &ReductionPath) -> Result<(), String>;
```

This validates that every edge on the path has symbolic overhead suitable for comparison.

Initial explicit exclusion:
- `ILP -> QUBO`

Reason:
- the implementation adds slack bits per inequality constraint
- slack growth depends on coefficients and rhs values
- current symbolic metadata does not encode enough information to reconstruct that growth

If a candidate path contains this edge, the comparison result is `Unknown`, not `Dominated` and not `NotDominated`.

## Expression Comparison

### Restricted objective

The detector only needs to compare **reduction overhead expressions**, not solver complexity expressions.

Current reduction overhead formulas are intended to be simple symbolic size maps, so the comparison logic should be intentionally narrower and more conservative than a full asymptotic algebra engine.

### Supported expression family

Implement comparison only for expressions that can be interpreted as sums of polynomial-like terms:
- constants
- variables
- addition
- multiplication
- powers with non-negative constant exponents
- parser-lowered negation (`-1 * expr`)
- constant factors multiplying a term

Unsupported forms become `Unknown`:
- `exp`
- `log`
- `sqrt`
- division / negative exponents
- any expression that cannot be normalized into the supported family

This matters because the parser lowers:
- subtraction to `a + (-1) * b`
- division to `a * b^-1`

So the comparison code must explicitly treat constant multipliers and sign changes as harmless constant factors, while still rejecting true rational or transcendental growth.

### Comparison rule

For each output field present in both overheads:
1. Normalize each expression into a supported multivariate polynomial-like form
2. If either expression cannot be normalized, return `Unknown`
3. Compare asymptotic growth on that field using the normalized multivariate form
4. Require `composite <= primitive` on every comparable field
5. Require at least one field to be strictly smaller, or all equal

If any required field is `Unknown`, the whole primitive-vs-path comparison is `Unknown`.

Do **not** reduce a multivariate expression to the maximum single-variable growth rate. That loses information for terms such as:
- `num_vertices * num_edges`
- `num_vertices + num_edges`

which are not asymptotically equivalent.

## Key Functions

```rust
fn rule_key(entry: &ReductionEntry) -> RuleKey;

fn compare_overhead(
    primitive: &ReductionOverhead,
    composite: &ReductionOverhead,
) -> ComparisonStatus;

fn find_dominated_rules(
    graph: &ReductionGraph,
) -> (Vec<DominatedRule>, Vec<UnknownComparison>);
```

### `find_dominated_rules`

Algorithm:
1. Iterate primitive `ReductionEntry`s from inventory
2. Build the primitive `RuleKey`
3. Enumerate all composite variant-level paths with the same exact source and target variant
4. Skip direct paths
5. For each composite path:
   - validate `path_is_symbolically_trustworthy`
   - compose symbolic overhead via `graph.compose_path_overhead(path)`
   - compare with `compare_overhead`
6. Record:
   - the best proven dominating path in `Vec<DominatedRule>`
   - undecidable candidates in `Vec<UnknownComparison>`

Path ranking:
- prefer fewer edges
- then prefer lexicographically smaller `ReductionPath::steps`

This keeps results deterministic across runs.

## Practical Invariant

`ReductionGraph` currently coalesces exact variant pairs into a single graph edge. The design therefore assumes:

- there is at most one registered primitive reduction per exact `(source variant, target variant)` pair

Add a dedicated test for that invariant. If the invariant stops holding, shortcut analysis must move from graph-level path enumeration to a registry-level multigraph.

## Test Plan

Add `src/unit_tests/rules/analysis.rs`.

Tests:

1. `test_redundant_rule_detection_is_sound`
- Run `find_dominated_rules(&ReductionGraph::new())`
- Compare the dominated set against an allow-list keyed by `RuleKey`
- The allow-list must be exact-variant and feature-aware

2. `test_redundant_rule_detection_reports_unknown_for_ilp_qubo_paths`
- Build at least one candidate path that includes `ILP -> QUBO`
- Assert the comparison is reported as `Unknown`
- Assert it is not promoted to `Dominated`

3. `test_no_duplicate_primitive_rules_per_exact_variant_pair`
- Iterate `ReductionEntry` inventory
- Assert uniqueness of `(source_name, source_variant, target_name, target_variant)`

4. `test_allow_list_is_feature_aware`
- The expected dominated set must depend on whether `ilp-solver` is enabled
- Do not hard-code a single unconditional count in the design

## Initial Allow-List Policy

The design should not hard-code a flat statement like "currently 9 dominated rules".

Instead:
- store exact variant-level keys
- gate ILP-dependent expectations behind `#[cfg(feature = "ilp-solver")]`
- exclude any candidate whose best proof path passes through `ILP -> QUBO`

That means the previous `KColoring -> QUBO via KColoring -> ILP -> QUBO` shortcut is **not** part of the trusted allow-list.

## Summary

The redundant-rule detector should be:
- variant-aware
- inventory-driven
- sound-but-incomplete
- explicit about `Unknown`

Most importantly, `ILP -> QUBO` remains a valid reduction but is **not** eligible for symbolic shortcut detection until ILP exposes richer size metadata for slack growth.
