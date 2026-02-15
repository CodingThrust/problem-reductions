# Design: Polish design.md

## Context

The current `docs/src/design.md` has empty sections (Overhead Evaluation, Reduction Execution), outdated content that doesn't reflect recent variant system changes, unclear section flow, and no unifying narrative for contributors.

## Audience

Library contributors who need to understand the internals to add new problems and reductions.

## Approach

**"Follow the Data"** — organize sections by the lifecycle of a reduction, from problem definition through graph construction to path resolution and execution. No overlap with `getting-started.md`.

## Proposed Structure

### 1. Module Overview (keep existing)
- Keep diagram + table
- Update opening line: "This guide covers the library internals for contributors."

### 2. Problem Model (renamed from "Models")
- Keep `Problem`, `OptimizationProblem`, `SatisfactionProblem` explanations
- Keep trait hierarchy diagram
- Minor tightening of examples

### 3. Variant System (expanded from "Problem variants")
- Keep concept intro, variant-hierarchy diagram, lattices diagram
- Add `VariantParam` trait definition (`CATEGORY`, `VALUE`, `PARENT_VALUE`)
- Add `impl_variant_param!` macro — 4 forms (root, with parent, KValue root, KValue with parent)
- Add `CastToParent` trait — runtime conversion for natural casts
- Keep `variant_params!` macro example

### 4. Reduction Rules (restructured)
- Keep `ReductionResult` struct + trait pattern
- Keep `ReduceTo<T>` impl with `#[reduction]` macro
- Add: what `#[reduction]` expands to (the `inventory::submit!(ReductionEntry { ... })` call)
- Add: `ReductionOverhead` declaration with `poly!` macro example

### 5. Reduction Graph (renamed from "Reduction")
- Construction: `ReductionGraph::new()` iterates inventory entries, builds `petgraph::DiGraph` + variant hierarchy with transitive closure
- Natural edges: auto-generated between same-name variant nodes via subtype check, identity overhead
- JSON export: `to_json()` produces `ReductionGraphJson`

### 6. Path Finding (keep and extend)
- Keep `resolve_path` algorithm steps and examples (MIS casting, KSat disambiguation)
- Keep `ResolvedPath` struct
- Add `find_cheapest_path` with Dijkstra + set-theoretic validation
- Add `PathCostFn` trait and built-in cost functions: `Minimize`, `MinimizeWeighted`, `MinimizeMax`, `MinimizeLexicographic`, `MinimizeSteps`, `CustomCost`

### 7. Overhead Evaluation (fill empty section)
- `ProblemSize`: named size components
- `Polynomial` / `Monomial`: overhead formula representation + `poly!` macro
- `ReductionOverhead::evaluate_output_size(input) -> ProblemSize`
- Composition: chain output of step N as input of step N+1
- Example: multi-step size propagation

### 8. Reduction Execution (fill empty section)
- `ResolvedPath` is a plan, not an executor
- Dispatch model: `Reduction` → `reduce_to()`, `NaturalCast` → `cast_to_parent()`
- Solution extraction: walk chain in reverse, `extract_solution()` at each Reduction step, natural casts preserve solution
- Design rationale: concrete types (no `dyn Problem`) for type safety

### 9. Solvers (expanded)
- `BruteForce`: enumerate all configs from `dims()`, `find_best`/`find_all_best`, `find_satisfying`/`find_all_satisfying`
- `ILPSolver`: feature-gated (`ilp`), HiGHS via `good_lp`, `solve_reduced()`
- Note: primarily for testing/verification

### 10. JSON Serialization (keep, minor polish)

### 11. Contributing (keep as-is)

## Removals
- The "Reduction" H2 header (line 132) — content redistributed into sections 5-8
- Duplicated `#[reduction]` example in "Reduction Graph" subsection

## Additions
- Sections 7 and 8 get real content
- Variant system gets `VariantParam`/`impl_variant_param!`/`CastToParent` machinery
- Path finding gets `find_cheapest_path` + `PathCostFn`
- Overhead gets `Polynomial`/`poly!` + size propagation
