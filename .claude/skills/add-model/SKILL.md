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
| 9 | **Best known exact algorithm** | Complexity with variable definitions | "O(1.1996^n) by Xiao & Nagamochi (2017), where n = \|V\|" |
| 10 | **Solving strategy** | How it can be solved | "BruteForce works; ILP reduction available" |
| 11 | **Category** | Which sub-module under `src/models/` | `graph`, `formula`, `set`, `algebraic`, `misc` |
| 12 | **Expected outcome from the issue** | Concrete outcome for the issue's example instance | Optimization: one optimal solution + optimal value. Satisfaction: one valid/satisfying solution + why it is valid |

If any item is missing, ask the user to provide it. Do NOT proceed until the checklist is complete.

The issue's **Expected Outcome** section is the source of truth for the implementation-facing example.
- For optimization problems, use the issue's optimal solution and optimal objective value.
- For satisfaction problems, use the issue's valid / satisfying solution and its justification.
- Do not invent or replace the expected outcome during implementation unless the issue is corrected first.

### Associated Rule Check

Before implementation, verify that at least one reduction rule exists or is planned for this problem — otherwise it will be an orphan node in the reduction graph.

**Check both directions:**

1. **Outbound (this issue → rule issues):** Look for rule issue numbers in the model issue's "Reduction Rule Crossref" section.
2. **Inbound (rule issues → this problem):** Search open rule issues that reference this problem as source or target:
   ```bash
   gh issue list --label rule --state open --limit 500 --json number,title | \
     jq '[.[] | select(.title | test("<ProblemName>"; "i"))]'
   ```

**If no associated rules are found:**
- Warn the user: "This model has no associated rule issues. It will be an orphan node in the reduction graph and will be flagged during review."
- Ask whether to proceed anyway or file a companion rule issue first (via `/propose rule`).
- If proceeding, add a visible `<!-- WARNING: orphan model — no associated rule issue -->` comment in the PR description.

**If associated rules are found:** List them and continue.

## Reference Implementations

Read these first to understand the patterns:
- **Optimization problem:** `src/models/graph/maximum_independent_set.rs`
- **Satisfaction problem:** `src/models/formula/sat.rs`
- **Model tests:** `src/unit_tests/models/graph/maximum_independent_set.rs`
- **Trait definitions:** `src/traits.rs` (`Problem`, `OptimizationProblem`, `SatisfactionProblem`)
- **Registry dispatch boundary:** `src/registry/mod.rs`, `src/registry/variant.rs`
- **CLI aliases:** `problemreductions-cli/src/problem_name.rs`
- **CLI creation:** `problemreductions-cli/src/commands/create.rs`
- **Canonical model examples:** `src/example_db/model_builders.rs`

## Pre-review Checklist

Before implementing, make sure the plan explicitly covers these items that structural review checks later:
- `ProblemSchemaEntry` metadata is complete for the current schema shape (`display_name`, `aliases`, `dimensions`, and constructor-facing `fields`)
- `declare_variants!` is present with the correct `opt`/`sat` marker and exactly one `default` variant when multiple concrete variants exist
- CLI discovery and `pred create <ProblemName>` support are included where applicable
- A canonical model example is registered for example-db / `pred create --example`
- `docs/paper/reductions.typ` adds both the display-name dictionary entry and the `problem-def(...)`

## Step 1: Determine the category

Choose the appropriate sub-module under `src/models/`:
- `graph/` -- problems defined on graphs (vertex/edge selection, SpinGlass, etc.)
- `formula/` -- logical formulas and circuits (SAT, k-SAT, CircuitSAT)
- `set/` -- set-based problems (set packing, set cover)
- `algebraic/` -- matrices, linear systems, lattices (QUBO, ILP, CVP, BMF)
- `misc/` -- unique input structures that don't fit other categories (BinPacking, PaintShop, Factoring)

## Step 1.5: Infer problem size getters

From the **best known exact algorithm** complexity (item 9), infer what problem size getter methods the struct should expose. The variables used in the complexity expression define the natural size metrics.

**How to infer:**
- Parse the complexity expression for variable names (e.g., `O(1.1996^n)` where `n = |V|` → `num_vertices`)
- Each variable that measures a distinct dimension of the input becomes a getter method
- Common mappings:
  - `n = |V|` → `num_vertices()`
  - `m = |E|` → `num_edges()`
  - `n` (number of variables) → `num_vars()`
  - `m` (number of clauses) → `num_clauses()`
  - `k` (number of sets) → `num_sets()`

These getters are used by the overhead system for reduction overhead expressions. Implement them as inherent methods on the struct.

## Step 2: Implement the model

Create `src/models/<category>/<name>.rs`:

```rust
// Required structure:
// 1. inventory::submit! for ProblemSchemaEntry
// 2. Struct definition with #[derive(Debug, Clone, Serialize, Deserialize)]
// 3. Constructor (new) + accessor methods
// 4. Problem trait impl (NAME, Metric, dims, evaluate, variant)
// 5. OptimizationProblem or SatisfactionProblem impl
// 6. #[cfg(test)] #[path = "..."] mod tests;
```

Key decisions:
- **Schema metadata:** `ProblemSchemaEntry` must reflect the current registry schema shape, including `display_name`, `aliases`, `dimensions`, and constructor-facing `fields`
- **Optimization problems:** `type Metric = SolutionSize<W::Sum>`, implement `OptimizationProblem` with `direction()`
- **Satisfaction problems:** `type Metric = bool`, implement `SatisfactionProblem` (marker trait)
- **Weight management:** use inherent methods (`weights()`, `set_weights()`, `is_weighted()`), NOT traits
- **`dims()`:** returns the configuration space dimensions (e.g., `vec![2; n]` for binary variables)
- **`evaluate()`:** must check feasibility first, then compute objective
- **`variant()`:** use the `variant_params!` macro — e.g., `crate::variant_params![G, W]` for `Problem<G, W>`, or `crate::variant_params![]` for problems with no type parameters. Each type parameter must implement `VariantParam` (already done for standard types like `SimpleGraph`, `i32`, `One`). See `src/variant.rs`.

## Step 2.5: Register variant complexity

Add `declare_variants!` at the bottom of the model file (after the trait impls, before the test link). Each line declares a concrete type instantiation with its best-known worst-case complexity:

```rust
crate::declare_variants! {
    opt ProblemName<SimpleGraph, i32> => "1.1996^num_vertices",
    default opt ProblemName<SimpleGraph, One> => "1.1996^num_vertices",
}
```

- Each entry must include an explicit solver kind:
  - `opt` for optimization problems (`BruteForce::find_best`)
  - `sat` for satisfaction problems (`BruteForce::find_satisfying`)
- Mark exactly one concrete variant `default` when the problem has multiple registered variants
- The complexity string references the getter method names from Step 1.5 (e.g., `num_vertices`) — variable names are validated at compile time against actual getters, so typos cause compile errors
- One entry per supported `(graph, weight)` combination
- The string is parsed as an `Expr` AST — supports `+`, `-`, `*`, `/`, `^`, `exp()`, `log()`, `sqrt()`
- Use only concrete numeric values (e.g., `"1.1996^num_vertices"`, not `"(2-epsilon)^num_vertices"`)
- A compiled `complexity_eval_fn` plus registry-backed load/serialize/solve dispatch metadata are auto-generated alongside the symbolic expression
- See `src/models/graph/maximum_independent_set.rs` for the reference pattern

## Step 3: Register the model

Update these files to register the new problem type:

1. `src/models/<category>/mod.rs` -- add `pub(crate) mod <name>;` and `pub use <name>::<ProblemType>;`
2. `src/models/mod.rs` -- add to the appropriate re-export line
3. `src/lib.rs` or `prelude` -- if the type should be in `prelude::*`, add it there

## Step 4: Register for CLI discovery

The CLI now loads, serializes, and brute-force solves problems through the core registry. Do **not** add manual match arms in `problemreductions-cli/src/dispatch.rs`.

1. **Registry-backed dispatch comes from `declare_variants!`:**
   - Make sure every concrete variant you want the CLI to load is listed in `declare_variants!`
   - Use the correct `opt`/`sat` marker per entry
   - Mark the intended default variant with `default` when applicable

2. **`problemreductions-cli/src/problem_name.rs`:**
   - Add a lowercase alias mapping in `resolve_alias()` (e.g., `"newproblem" => "NewProblem".to_string()`)
   - Only add short aliases to the `ALIASES` array if the abbreviation is **well-established in the literature** (e.g., MIS, MVC, SAT, TSP, CVP are standard; "KS" for Knapsack or "BP" for BinPacking are NOT — do not invent new abbreviations)

## Step 4.5: Add CLI creation support

Update `problemreductions-cli/src/commands/create.rs` so `pred create <ProblemName>` works:

1. **Add a match arm** in the `create()` function's main `match canonical.as_str()` block. Parse CLI flags and construct the problem:
   - Graph-based problems with vertex weights: add to the `"MaximumIndependentSet" | ... | "MaximalIS"` arm
   - Problems with unique fields: add a new arm that parses the required flags and calls the constructor
   - See existing arms for patterns (e.g., `"BinPacking"` for simple fields, `"MaximumSetPacking"` for set-based)

2. **Add CLI flags** in `problemreductions-cli/src/cli.rs` (`CreateArgs` struct) if the problem needs flags not already present. Update `all_data_flags_empty()` accordingly.

3. **Update help text** in `CreateArgs`'s `after_help` — add the new problem to the "Flags by problem type" table in `problemreductions-cli/src/cli.rs` (search for `Flags by problem type`).

4. **Schema alignment**: The `ProblemSchemaEntry` fields should list **constructor parameters** (what the user provides), not internal derived fields. For example, if `m` and `n` are derived from a matrix, only list `matrix` and `k` in the schema.

## Step 4.6: Add canonical model example to example_db

Add a builder function in `src/example_db/model_builders.rs` that constructs a small, canonical instance for this model. Register it in `build_model_examples()`.

This example is now the canonical source for:
- `pred create --example <PROBLEM_SPEC>`
- paper/example exports
- example-db invariants tested in `src/unit_tests/example_db.rs`

## Step 5: Write unit tests

Create `src/unit_tests/models/<category>/<name>.rs`:

Required tests:
- `test_<name>_creation` -- construct an instance, verify dimensions
- `test_<name>_evaluation` -- verify `evaluate()` on valid and invalid configs
- `test_<name>_direction` -- verify optimization direction (if optimization problem)
- `test_<name>_serialization` -- round-trip serde test (optional but recommended)
- `test_<name>_solver` -- verify brute-force solver finds correct solutions
- `test_<name>_paper_example` -- **use the same instance from the paper example** (Step 6), verify the issue's expected outcome is valid/optimal and the solution count matches

The `test_<name>_paper_example` test is critical for consistency between code and paper. It must:
1. Construct the exact same instance shown in the paper's example figure
2. Evaluate the solution from the issue's **Expected Outcome** section as shown in the paper and assert it is valid (and optimal for optimization problems)
3. Use `BruteForce` to find all optimal/satisfying solutions and assert the count matches the paper's claim

This test should be written **after** Step 6 (paper entry), once the example instance and expected outcome are finalized. If writing tests before the paper, use the issue's Example Instance + Expected Outcome as the source of truth and come back to verify consistency.

Link the test file via `#[cfg(test)] #[path = "..."] mod tests;` at the bottom of the model file.

## Step 6: Document in paper

Write a `problem-def` entry in `docs/paper/reductions.typ`. **Reference example:** search for `problem-def("MaximumIndependentSet")` to see the gold-standard entry — use it as a template.

### 6a. Register display name

Add to the `display-name` dictionary near the top of `reductions.typ`:
```typst
"ProblemName": [Display Name],
```

### 6b. Write formal definition (`def` parameter)

```typst
#problem-def("ProblemName")[
  Given [inputs with domains], find [solution] [maximizing/minimizing] [objective] such that [constraints].
][
```
Requirements: introduce all inputs first, state the objective, define all notation before use.

### 6c. Write body (background + example)

The body goes AFTER auto-generated sections (complexity table, reductions, schema). Four parts:

**Background (1-3 sentences):** Historical context, applications, structural properties.

**Best known algorithms:** Integrate naturally into prose with citations. Every complexity claim MUST have `@citation`. If best known is brute-force, add `#footnote[No algorithm improving on brute-force is known for ...]`.

**Example with visualization:** A concrete small instance with a CeTZ diagram. For graph problems, use `g-node()` and `g-edge()` helpers — see the MaximumIndependentSet entry. Highlight solution with `graph-colors.at(0)`.

**Evaluation:** Show the objective/verifier computed on the example solution (can be woven into example text).

### 6d. Build and verify

```bash
make paper  # Must compile without errors
```

Checklist: display name registered, notation self-contained, background present, algorithms cited, example with diagram present, evaluation shown, paper compiles.

## Step 7: Verify

```bash
make test clippy  # Must pass
```

If running standalone (not inside `make run-plan`), invoke [review-implementation](../review-implementation/SKILL.md) to verify all structural and semantic checks pass. When running inside a plan, the outer orchestrator handles the review.

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
| Forgetting `declare_variants!` | Required for variant complexity metadata and registry-backed load/serialize/solve dispatch |
| Wrong `declare_variants!` syntax | Every entry now needs `opt` or `sat`; one entry per problem may be marked `default` |
| Forgetting CLI alias | Must add lowercase entry in `problem_name.rs` `resolve_alias()` |
| Inventing short aliases | Only use well-established literature abbreviations (MIS, SAT, TSP); do NOT invent new ones |
| Forgetting CLI create | Must add creation handler in `commands/create.rs` and flags in `cli.rs` |
| Missing from CLI help table | Must add entry to "Flags by problem type" table in `cli.rs` `after_help` |
| Schema lists derived fields | Schema should list constructor params, not internal fields (e.g., `matrix, k` not `matrix, m, n, k`) |
| Missing canonical model example | Add a builder in `src/example_db/model_builders.rs` and keep it aligned with paper/example workflows |
| Paper example not tested | Must include `test_<name>_paper_example` that verifies the exact instance, solution, and solution count shown in the paper |
