---
name: add-model
description: Use when adding a new problem model to the codebase, either from an issue or interactively
---

# Add Model

Step-by-step guide for adding a new problem model to the codebase.

## Step 0: Gather Required Information

Before any implementation, collect all required information. If called from `issue-to-pr`, the issue should already provide these. If used standalone, brainstorm with the user to fill in every item below.

### Required Information Checklist

| # | Item | Description | Example |
|---|------|-------------|---------|
| 1 | **Problem name** | Struct name with optimization prefix | `MaximumClique`, `MinimumDominatingSet` |
| 2 | **Mathematical definition** | Formal definition with objective/constraints | "Given graph G=(V,E), find max-weight subset S where all pairs in S are adjacent" |
| 3 | **Problem type** | Optimization (maximize/minimize) or satisfaction | Optimization (Maximize) |
| 4 | **Type parameters** | Graph type `G`, weight type `W`, or other | `G: Graph`, `W: WeightElement` |
| 5 | **Struct fields** | What the struct holds | `graph: G`, `weights: Vec<W>` |
| 6 | **Configuration space** | What `dims()` returns | `vec![2; num_vertices]` for binary vertex selection |
| 7 | **Feasibility check** | How to validate a configuration | "All selected vertices must be pairwise adjacent" |
| 8 | **Objective function** | How to compute the metric | "Sum of weights of selected vertices" |
| 9 | **Complexity class** | NP-hard, NP-complete, etc. | NP-hard |
| 10 | **Solving strategy** | How it can be solved | "BruteForce works; ILP reduction available" |
| 11 | **Category** | Which sub-module under `src/models/` | `graph`, `optimization`, `satisfiability`, `set`, `specialized` |

If any item is missing, ask the user to provide it. Do NOT proceed until the checklist is complete.

## Reference Implementations

Read these first to understand the patterns:
- **Optimization problem:** `src/models/graph/maximum_independent_set.rs`
- **Satisfaction problem:** `src/models/satisfiability/sat.rs`
- **Model tests:** `src/unit_tests/models/graph/maximum_independent_set.rs`
- **Trait definitions:** `src/traits.rs` (`Problem`, `OptimizationProblem`, `SatisfactionProblem`)
- **CLI dispatch:** `problemreductions-cli/src/dispatch.rs`
- **CLI aliases:** `problemreductions-cli/src/problem_name.rs`

## Step 1: Determine the category

Choose the appropriate sub-module under `src/models/`:
- `graph/` -- problems defined on graphs (vertex/edge selection)
- `optimization/` -- generic optimization formulations (QUBO, ILP, SpinGlass)
- `satisfiability/` -- boolean satisfaction problems (SAT, k-SAT)
- `set/` -- set-based problems (set packing, set cover)
- `specialized/` -- problems that don't fit other categories (factoring, circuit, paintshop)

## Step 2: Implement the model

Create `src/models/<category>/<name>.rs`:

```rust
// Required structure:
// 1. inventory::submit! for ProblemSchemaEntry
// 2. Struct definition with #[derive(Debug, Clone, Serialize, Deserialize)]
// 3. Constructor (new) + accessor methods
// 4. Problem trait impl (NAME, Metric, dims, evaluate, variant, problem_size_names, problem_size_values)
// 5. OptimizationProblem or SatisfactionProblem impl
// 6. #[cfg(test)] #[path = "..."] mod tests;
```

Key decisions:
- **Optimization problems:** `type Metric = SolutionSize<W::Sum>`, implement `OptimizationProblem` with `direction()`
- **Satisfaction problems:** `type Metric = bool`, implement `SatisfactionProblem` (marker trait)
- **Weight management:** use inherent methods (`weights()`, `set_weights()`, `is_weighted()`), NOT traits
- **`dims()`:** returns the configuration space dimensions (e.g., `vec![2; n]` for binary variables)
- **`evaluate()`:** must check feasibility first, then compute objective

## Step 3: Register the model

Update these files to register the new problem type:

1. `src/models/<category>/mod.rs` -- add `pub(crate) mod <name>;` and `pub use <name>::<ProblemType>;`
2. `src/models/mod.rs` -- add to the appropriate re-export line
3. `src/lib.rs` or `prelude` -- if the type should be in `prelude::*`, add it there

## Step 4: Register in CLI

Update the CLI dispatch table so `pred` can load, solve, and serialize the new problem:

1. **`problemreductions-cli/src/dispatch.rs`:**
   - Add a match arm in `load_problem()` -- use `deser_opt::<T>` for optimization or `deser_sat::<T>` for satisfaction
   - Add a match arm in `serialize_any_problem()` -- use `try_ser::<T>`

2. **`problemreductions-cli/src/problem_name.rs`:**
   - Add a lowercase alias mapping in `resolve_alias()` (e.g., `"newproblem" => "NewProblem".to_string()`)
   - Optionally add short aliases to `ALIASES` array (e.g., `("NP", "NewProblem")`)

## Step 5: Write unit tests

Create `src/unit_tests/models/<category>/<name>.rs`:

Required tests:
- `test_<name>_creation` -- construct an instance, verify dimensions
- `test_<name>_evaluation` -- verify `evaluate()` on valid and invalid configs
- `test_<name>_direction` -- verify optimization direction (if optimization problem)
- `test_<name>_serialization` -- round-trip serde test (optional but recommended)
- `test_<name>_solver` -- verify brute-force solver finds correct solutions

Link the test file via `#[cfg(test)] #[path = "..."] mod tests;` at the bottom of the model file.

## Step 6: Document in paper

Update `docs/paper/reductions.typ`:
- Add to the `display-name` dictionary: `"ProblemName": [Display Name],`
- Add a `#problem-def("ProblemName")[...]` block with the mathematical definition

## Step 7: Verify

```bash
make test clippy  # Must pass
```

Then run the [review-implementation](../review-implementation/SKILL.md) skill to verify all structural and semantic checks pass.

## Naming Conventions

- Struct names use explicit optimization prefixes: `MaximumX`, `MinimumX`
- No prefix for problems without clear min/max direction: `QUBO`, `Satisfiability`, `KColoring`
- File names use snake_case: `maximum_independent_set.rs`
- See CLAUDE.md "Problem Names" section for the full list

## Common Mistakes

| Mistake | Fix |
|---------|-----|
| Implementing weight management as a trait | Use inherent methods: `weights()`, `set_weights()`, `is_weighted()` |
| Forgetting `inventory::submit!` | Every problem needs a `ProblemSchemaEntry` registration |
| Missing `#[path]` test link | Add `#[cfg(test)] #[path = "..."] mod tests;` at file bottom |
| Wrong `dims()` | Must match the actual configuration space (e.g., `vec![2; n]` for binary) |
| Not registering in `mod.rs` | Must update both `<category>/mod.rs` and `models/mod.rs` |
| Forgetting CLI dispatch | Must add match arms in `dispatch.rs` (`load_problem` + `serialize_any_problem`) |
| Forgetting CLI alias | Must add lowercase entry in `problem_name.rs` `resolve_alias()` |
