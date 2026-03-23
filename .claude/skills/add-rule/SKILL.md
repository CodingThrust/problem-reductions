---
name: add-rule
description: Use when adding a new reduction rule to the codebase, either from an issue or interactively
---

# Add Rule

Step-by-step guide for adding a new reduction rule (A -> B) to the codebase.

## Step 0: Gather Required Information

Before any implementation, collect all required information. If called from `issue-to-pr`, the issue should already provide these. If used standalone, brainstorm with the user to fill in every item below.

### Required Information Checklist

| # | Item | Description | Example |
|---|------|-------------|---------|
| 1 | **Source problem** | The problem being reduced FROM (must already exist) | `MinimumVertexCover<SimpleGraph, i32>` |
| 2 | **Target problem** | The problem being reduced TO (must already exist) | `MaximumIndependentSet<SimpleGraph, i32>` |
| 3 | **Reduction algorithm** | How to transform source instance to target | "Copy graph and weights; IS on same graph as VC" |
| 4 | **Solution extraction** | How to map target solution back to source | "Complement: `1 - x` for each variable" |
| 5 | **Correctness argument** | Why the reduction preserves optimality | "S is independent set iff V\S is vertex cover" |
| 6 | **Size overhead** | How target size relates to source size | `num_vertices = "num_vertices", num_edges = "num_edges"` |
| 7 | **Concrete example** | A small worked-out instance (tutorial style, clear intuition) | "Triangle graph: VC={0,1} -> IS={2}" |
| 8 | **Solving strategy** | How to solve the target problem | "BruteForce, or existing ILP reduction" |
| 9 | **Reference** | Paper, textbook, or URL for the reduction | URL or citation |

If any item is missing, ask the user to provide it. Put a high standard on item 7 (concrete example): it must be in tutorial style with clear intuition and easy to understand. Do NOT proceed until the checklist is complete.

## Reference Implementations

Read these first to understand the patterns:
- **Reduction rule:** `src/rules/minimumvertexcover_maximumindependentset.rs`
- **Reduction tests:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
- **Paper entry:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet`
- **Traits:** `src/rules/traits.rs` (`ReduceTo<T>`, `ReduceToAggregate<T>`, `ReductionResult`, `AggregateReductionResult`)

## Step 1: Implement the reduction

Create `src/rules/<source>_<target>.rs` (all lowercase, no underscores between words within a problem name):

```rust
// Required structure:
// 1. ReductionResult struct (holds the target problem + mapping state)
// 2. ReductionResult trait impl (target_problem + extract_solution)
// 3. #[reduction(overhead = { ... })] on ReduceTo impl
// 4. ReduceTo trait impl (reduce_to method)
// 5. #[cfg(test)] #[path = "..."] mod tests;
```

Key elements:

**ReductionResult struct:**
```rust
#[derive(Debug, Clone)]
pub struct ReductionXToY {
    target: TargetType,
    // any additional mapping state needed for extract_solution
}
```

**ReductionResult trait impl:**
```rust
impl ReductionResult for ReductionXToY {
    type Source = SourceType;
    type Target = TargetType;
    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Map target solution back to source solution
    }
}
```

**ReduceTo with `#[reduction]` macro** (overhead is **required**):
```rust
#[reduction(overhead = {
    field_name = "source_field",
})]
impl ReduceTo<TargetType> for SourceType {
    type Result = ReductionXToY;
    fn reduce_to(&self) -> Self::Result { ... }
}
```

Each primitive reduction is determined by the exact source/target variant pair. Keep one primitive registration per endpoint pair and use only the `overhead` form of `#[reduction]`.

**Aggregate-only reductions:** when the rule preserves aggregate values but cannot recover a source witness from a target witness, implement `AggregateReductionResult` + `ReduceToAggregate<T>` instead of `ReductionResult` + `ReduceTo<T>`. Those edges are not auto-registered by `#[reduction]` yet; register them manually with `ReductionEntry { reduce_aggregate_fn: ..., capabilities: EdgeCapabilities::aggregate_only(), ... }`. See `src/unit_tests/rules/traits.rs` and `src/unit_tests/rules/graph.rs` for the reference pattern.

## Step 2: Register in mod.rs

Add to `src/rules/mod.rs`:
- `mod <source>_<target>;`
- If feature-gated (e.g., ILP): wrap with `#[cfg(feature = "ilp-solver")]`

## Step 3: Write unit tests

Create `src/unit_tests/rules/<source>_<target>.rs`:

**Required: closed-loop test** (`test_<source>_to_<target>_closed_loop`):
```rust
// 1. Create source problem instance
// 2. Reduce: let reduction = ReduceTo::<Target>::reduce_to(&source);
// 3. Solve target: solver.find_all_witnesses(reduction.target_problem())
// 4. Extract: reduction.extract_solution(&target_sol)
// 5. Verify: extracted solution is valid and optimal for source
```

Additional recommended tests:
- Verify target problem structure (correct size, edges, constraints)
- Edge cases (empty graph, single vertex, etc.)
- Weight preservation (if applicable)

For aggregate-only reductions, replace the closed-loop witness test with value-chain tests:
- Solve the target with `Solver::solve()`
- Map the aggregate value back with `extract_value()`
- If testing a path, use `ReductionGraph::reduce_aggregate_along_path(...)`

Link via `#[cfg(test)] #[path = "..."] mod tests;` at the bottom of the rule file.

## Step 4: Add canonical example to example_db

Add a builder function in `src/example_db/rule_builders.rs` that constructs a small, canonical instance for this reduction. Follow the existing patterns in that file. Register the builder in `build_rule_examples()`.

## Step 5: Document in paper

Write a `reduction-rule` entry in `docs/paper/reductions.typ`. **Reference example:** search for `reduction-rule("KColoring", "QUBO"` to see the gold-standard entry — use it as a template. For a minimal example, see MinimumVertexCover -> MaximumIndependentSet.

### 5a. Write theorem body (rule statement)

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [Description ($n = ...$, $|E| = ...$)],
)[
  This $O(...)$ reduction @citation constructs [target structure] ... ($n k$ variables indexed by ...).
]
```

Three parts: complexity with citation, construction summary, overhead hint.

### 5b. Write proof body

Use these subsections with italic labels:

```typst
][
  _Construction._ [Full mathematical construction — enough detail to reimplement]

  _Correctness._ ($arrow.r.double$) If ... ($arrow.l.double$) If ...

  _Variable mapping._ [Only if non-trivial mapping]

  _Solution extraction._ [How to convert target solution back to source]
]
```

Must be self-contained (all notation defined) and reproducible.

### 5c. Write worked example (extra block)

Step-by-step walkthrough with concrete numbers from JSON data. Required steps:
1. Show source instance (dimensions, structure, graph visualization if applicable)
2. Walk through construction with intermediate values
3. Verify a concrete solution end-to-end
4. Witness semantics: state that the fixture stores one canonical witness; if multiplicity matters mathematically, explain it from the construction rather than from `solutions.len()`

Use `graph-colors`, `g-node()`, `g-edge()` for graph visualization — see reference examples.

**Reproducibility:** The `extra:` block must start with a `pred-commands()` call showing the create/reduce/solve/evaluate pipeline. The source-side `pred create --example ...` spec must be derived from the loaded canonical example data via the helper pattern in `write-rule-in-paper`; do not hand-write a bare alias and assume the default variant matches.

### 5d. Build and verify

```bash
make paper     # Must compile without errors
```

Checklist: notation self-contained, complexity cited, overhead consistent, example uses JSON data (not hardcoded), solution verified end-to-end, witness semantics respected, paper compiles.

## Step 6: Regenerate exports and verify

```bash
cargo run --example export_graph    # Generate reduction_graph.json for docs/paper builds
cargo run --example export_schemas  # Generate problem schemas for docs/paper builds
make regenerate-fixtures            # Regenerate example_db/fixtures/examples.json (slow, needs ILP)
make test clippy                    # Must pass
```

`make regenerate-fixtures` is required so the paper can load the new rule's example data from `src/example_db/fixtures/examples.json`. Without it, the `reduction-rule` entry in Step 5 will reference missing fixture data.

Structural and quality review is handled by the `review-pipeline` stage, not here. The run stage just needs to produce working code.

## Solver Rules

- If the target problem already has a solver, use it directly.
- If the solving strategy requires ILP, implement the ILP reduction rule alongside (feature-gated under `ilp-solver`).
- If a custom solver is needed, implement in `src/solvers/` and document.

## CLI Impact

Adding a witness-preserving reduction rule does NOT require CLI changes -- the reduction graph is auto-generated from `#[reduction]` macros and the CLI discovers paths dynamically. However, both source and target models must already be fully registered through their model files (`declare_variants!`), aliases as needed in `problem_name.rs`, and `pred create` support where applicable (see `add-model` skill).

Aggregate-only reductions currently have a narrower CLI surface:
- `pred solve <problem.json>` can still compute direct aggregate values for aggregate-only problems
- `pred reduce` and `pred solve bundle.json` remain witness-only workflows and reject aggregate-only paths
- Manual aggregate-edge registration affects runtime graph search and internal value extraction, but not bundle solving

## File Naming

- Rule file: `src/rules/<sourcelower>_<targetlower>.rs` -- no underscores within a problem name
  - e.g., `maximumindependentset_qubo.rs`, `minimumvertexcover_maximumindependentset.rs`
- Test file: `src/unit_tests/rules/<sourcelower>_<targetlower>.rs`
- Canonical example: builder function in `src/example_db/rule_builders.rs`

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Forgetting `#[reduction(...)]` macro | Required for compile-time registration in the reduction graph |
| Using `#[reduction]` for an aggregate-only rule | `#[reduction]` currently registers witness/config edges only; aggregate-only rules need manual `ReductionEntry` wiring with `reduce_aggregate_fn` |
| Wrong overhead expression | Must accurately reflect the size relationship |
| Adding extra reduction metadata or duplicate primitive endpoint registration | Keep one primitive registration per endpoint pair and use only the `overhead` form of `#[reduction]` |
| Missing `extract_solution` mapping state | Store any index maps needed in the ReductionResult struct |
| Not adding canonical example to `example_db` | Add builder in `src/example_db/rule_builders.rs` |
| Not regenerating reduction graph | Run `cargo run --example export_graph` after adding a rule |
| Source/target model not fully registered | Both problems must already have `declare_variants!`, aliases as needed, and CLI create support -- use `add-model` skill first |
