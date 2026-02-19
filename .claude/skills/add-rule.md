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
| 6 | **Size overhead** | How target size relates to source size | `num_vertices: poly!(num_vertices), num_edges: poly!(num_edges)` |
| 7 | **Concrete example** | A small worked-out instance (tutorial style, clear intuition) | "Triangle graph: VC={0,1} -> IS={2}" |
| 8 | **Solving strategy** | How to solve the target problem | "BruteForce, or existing ILP reduction" |
| 9 | **Reference** | Paper, textbook, or URL for the reduction | URL or citation |

If any item is missing, ask the user to provide it. Put a high standard on item 7 (concrete example): it must be in tutorial style with clear intuition and easy to understand. Do NOT proceed until the checklist is complete.

## Reference Implementations

Read these first to understand the patterns:
- **Reduction rule:** `src/rules/minimumvertexcover_maximumindependentset.rs`
- **Reduction tests:** `src/unit_tests/rules/minimumvertexcover_maximumindependentset.rs`
- **Example program:** `examples/reduction_minimumvertexcover_to_maximumindependentset.rs`
- **Example registration:** `tests/suites/examples.rs`
- **Paper entry:** search `docs/paper/reductions.typ` for `MinimumVertexCover` `MaximumIndependentSet`
- **Traits:** `src/rules/traits.rs` (`ReduceTo<T>`, `ReductionResult`)

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

**ReduceTo with `#[reduction]` macro:**
```rust
#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("field_name", poly!(source_field)),
        ])
    }
)]
impl ReduceTo<TargetType> for SourceType {
    type Result = ReductionXToY;
    fn reduce_to(&self) -> Self::Result { ... }
}
```

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
// 3. Solve target: solver.find_all_best(reduction.target_problem())
// 4. Extract: reduction.extract_solution(&target_sol)
// 5. Verify: extracted solution is valid and optimal for source
```

Additional recommended tests:
- Verify target problem structure (correct size, edges, constraints)
- Edge cases (empty graph, single vertex, etc.)
- Weight preservation (if applicable)

Link via `#[cfg(test)] #[path = "..."] mod tests;` at the bottom of the rule file.

## Step 4: Write example program

Create `examples/reduction_<source>_to_<target>.rs`:

Required structure:
- `pub fn run()` -- main logic (required for test harness)
- `fn main() { run() }` -- entry point
- Use regular comments (`//`), not doc comments
- Create source instance, reduce, solve, extract, verify, export JSON

Register in `tests/suites/examples.rs`:
```rust
example_test!(reduction_<source>_to_<target>);
// ...
example_fn!(test_<source>_to_<target>, reduction_<source>_to_<target>);
```

## Step 5: Document in paper

Update `docs/paper/reductions.typ`:

```typst
#reduction-rule("Source", "Target",
  example: true,
  example-caption: [Caption text],
)[
  Reduction rule statement...
][
  Proof sketch...
]
```

Present the example in tutorial style with clear intuition. Reference the KColoring -> QUBO section for style guidance.

## Step 6: Regenerate graph and verify

```bash
cargo run --example export_graph  # Update reduction_graph.json
make test clippy                  # Must pass
```

## Solver Rules

- If the target problem already has a solver, use it directly.
- If the solving strategy requires ILP, implement the ILP reduction rule alongside (feature-gated under `ilp-solver`).
- If a custom solver is needed, implement in `src/solvers/` and document.

## CLI Impact

Adding a reduction rule does NOT require CLI changes -- the reduction graph is auto-generated from `#[reduction]` macros and the CLI discovers paths dynamically. However, both source and target models must already be registered in the CLI dispatch table (see `add-model` skill).

## File Naming

- Rule file: `src/rules/<sourcelower>_<targetlower>.rs` -- no underscores within a problem name
  - e.g., `maximumindependentset_qubo.rs`, `minimumvertexcover_maximumindependentset.rs`
- Example file: `examples/reduction_<source>_to_<target>.rs`
  - e.g., `reduction_minimumvertexcover_to_maximumindependentset.rs`
- Test file: `src/unit_tests/rules/<sourcelower>_<targetlower>.rs`

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Forgetting `#[reduction(...)]` macro | Required for compile-time registration in the reduction graph |
| Wrong overhead polynomial | Must accurately reflect the size relationship |
| Missing `extract_solution` mapping state | Store any index maps needed in the ReductionResult struct |
| Example missing `pub fn run()` | Required for the test harness (`include!` pattern) |
| Not registering example in `tests/suites/examples.rs` | Must add both `example_test!` and `example_fn!` |
| Not regenerating reduction graph | Run `cargo run --example export_graph` after adding a rule |
| Source/target model not in CLI dispatch | Both problems must be registered -- use `add-model` skill first |
