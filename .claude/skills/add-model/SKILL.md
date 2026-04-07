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
| 3 | **Problem type** | Objective (`Max`/`Min`), witness (`bool`), or aggregate-only (`Sum`/`And`/custom `Aggregate`) | Objective (Maximize) |
| 4 | **Type parameters** | Graph type `G`, weight type `W`, or other | `G: Graph`, `W: WeightElement` |
| 5 | **Struct fields** | What the struct holds | `graph: G`, `weights: Vec<W>` |
| 6 | **Configuration space** | What `dims()` returns | `vec![2; num_vertices]` for binary vertex selection |
| 7 | **Feasibility check** | How to validate a configuration | "All selected vertices must be pairwise adjacent" |
| 8 | **Per-configuration value** | How `evaluate()` computes the aggregate contribution | "Return `Max(Some(total_weight))` for feasible configs" |
| 9 | **Best known exact algorithm** | Complexity with variable definitions | "O(1.1996^n) by Xiao & Nagamochi (2017), where n = \|V\|" |
| 10 | **Solving strategy** | How it can be solved | "BruteForce works; ILP reduction available" |
| 11 | **Category** | Which sub-module under `src/models/` | `graph`, `formula`, `set`, `algebraic`, `misc` |
| 12 | **Expected outcome from the issue** | Concrete outcome for the issue's example instance | Objective: one optimal solution + optimal value. Witness: one valid/satisfying solution + why it is valid. Aggregate-only: the final aggregate value and how it is derived |

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

**If the issue explicitly claims ILP solvability in "How to solve":**
- One associated rule MUST be a direct `[Rule] <ProblemName> to ILP`
- Treat that direct ILP rule as part of the same implementation scope
- Do NOT split the model and its direct ILP rule into separate PRs

## Reference Implementations

Read these first to understand the patterns:
- **Optimization problem:** `src/models/graph/maximum_independent_set.rs`
- **Satisfaction problem:** `src/models/formula/sat.rs`
- **Model tests:** `src/unit_tests/models/graph/maximum_independent_set.rs`
- **Trait definitions / aggregate types:** `src/traits.rs` (`Problem`), `src/types.rs` (`Aggregate`, `Max`, `Min`, `Sum`, `Or`, `And`, `Extremum`)
- **Registry dispatch boundary:** `src/registry/mod.rs`, `src/registry/variant.rs`
- **CLI aliases:** `problemreductions-cli/src/problem_name.rs`
- **CLI creation:** `problemreductions-cli/src/commands/create.rs`
- **Canonical model examples:** `src/example_db/model_builders.rs`

## Pre-review Checklist

Before implementing, make sure the plan explicitly covers these items that structural review checks later:
- `ProblemSchemaEntry` metadata is complete for the current schema shape (`display_name`, `aliases`, `dimensions`, and constructor-facing `fields`)
- `Problem::Value` uses the correct aggregate wrapper and witness support is intentional
- `declare_variants!` is present with exactly one `default` variant when multiple concrete variants exist
- CLI discovery and `pred create <ProblemName>` support are included where applicable
- A canonical model example is registered for example-db / `pred create --example`
- If the issue explicitly claims direct ILP solving, the plan also includes the direct `<Problem> -> ILP` rule with exact overhead metadata, feature-gated registration, strong regression tests, and ILP-enabled verification
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
// 4. Problem trait impl (NAME, Value, dims, evaluate, variant)
// 5. #[cfg(test)] #[path = "..."] mod tests;
```

Key decisions:
- **Schema metadata:** `ProblemSchemaEntry` must reflect the current registry schema shape, including `display_name`, `aliases`, `dimensions`, and constructor-facing `fields`
- **Objective problems:** use `type Value = Max<_>`, `Min<_>`, or `Extremum<_>` when the model should expose optimization-style witness helpers
- **Witness problems:** use `type Value = Or` for existential feasibility problems
- **Aggregate-only problems:** use a value-only aggregate such as `Sum<_>`, `And`, or a custom `Aggregate` when witnesses are not meaningful
- **Weight management:** use inherent methods (`weights()`, `set_weights()`, `is_weighted()`), NOT traits
- **`dims()`:** returns the configuration space dimensions (e.g., `vec![2; n]` for binary variables)
- **`evaluate()`:** must return the per-configuration aggregate value. For models with invalid configs, check feasibility first and return the appropriate invalid/false contribution
- **`variant()`:** use the `variant_params!` macro — e.g., `crate::variant_params![G, W]` for `Problem<G, W>`, or `crate::variant_params![]` for problems with no type parameters. Each type parameter must implement `VariantParam` (already done for standard types like `SimpleGraph`, `i32`, `One`). See `src/variant.rs`.
- **Solve surface:** `Solver::solve()` always computes the aggregate value. `pred solve problem.json` prints a `Solution` only when a witness exists; `pred solve bundle.json` and `--solver ilp` remain witness-only workflows

## Step 2.5: Register variant complexity

Add `declare_variants!` at the bottom of the model file (after the trait impls, before the test link). Each line declares a concrete type instantiation with its best-known worst-case complexity:

```rust
crate::declare_variants! {
    ProblemName<SimpleGraph, i32> => "1.1996^num_vertices",
    default ProblemName<SimpleGraph, One> => "1.1996^num_vertices",
}
```

- Mark exactly one concrete variant `default` when the problem has multiple registered variants
- The complexity string references the getter method names from Step 1.5 (e.g., `num_vertices`) — variable names are validated at compile time against actual getters, so typos cause compile errors
- One entry per supported `(graph, weight)` combination
- The string is parsed as an `Expr` AST — supports `+`, `-`, `*`, `/`, `^`, `exp()`, `log()`, `sqrt()`
- Use only concrete numeric values (e.g., `"1.1996^num_vertices"`, not `"(2-epsilon)^num_vertices"`)
- A compiled `complexity_eval_fn` plus registry-backed load/serialize/solve dispatch metadata are auto-generated alongside the symbolic expression
- See `src/models/graph/maximum_independent_set.rs` for the reference pattern

`declare_variants!` now handles objective, witness-capable, and aggregate-only models uniformly. Use manual `VariantEntry` wiring only for unusual dynamic-registration work, not for ordinary models.

## Step 3: Register the model

Update these files to register the new problem type:

1. `src/models/<category>/mod.rs` -- add `pub(crate) mod <name>;` and `pub use <name>::<ProblemType>;`
2. `src/models/mod.rs` -- add to the appropriate re-export line
3. `src/lib.rs` or `prelude` -- if the type should be in `prelude::*`, add it there

## Step 4: Register for CLI discovery

The CLI now loads, serializes, and brute-force solves problems through the core registry. Do **not** add manual match arms in `problemreductions-cli/src/dispatch.rs`.

1. **Registry-backed dispatch comes from `declare_variants!`:**
   - Make sure every concrete variant you want the CLI to load is listed in `declare_variants!`
   - Mark the intended default variant with `default` when applicable

2. **`problemreductions-cli/src/problem_name.rs`:**
   - Add a lowercase alias mapping in `resolve_alias()` (e.g., `"newproblem" => "NewProblem".to_string()`)
   - Only add short aliases to the `ALIASES` array if the abbreviation is **well-established in the literature** (e.g., MIS, MVC, SAT, TSP, CVP are standard; "KS" for Knapsack or "BP" for BinPacking are NOT — do not invent new abbreviations)

## Step 4.5: Add CLI creation support

CLI creation is **schema-driven** — `pred create <ProblemName>` automatically maps `ProblemSchemaEntry` fields to CLI flags via `snake_case → kebab-case` convention. No match arm in `create.rs` is needed.

1. **Ensure CLI flags exist** in `problemreductions-cli/src/cli.rs` (`CreateArgs` struct) for each field in your `ProblemSchemaEntry`. The flag name must match the field name via `snake_case → kebab-case` (e.g., field `edge_weights` → flag `--edge-weights`). If a flag already exists with the right name, you're done.

2. **Add new CLI flags** only if the problem needs flags not already present. Add them to `CreateArgs` and update `all_data_flags_empty()` accordingly. Also add entries to the `flag_map()` method on `CreateArgs`.

3. **Add type parser support** if the field uses a type not yet handled by `parse_field_value()` in `create.rs`. Check the existing type dispatch table — most standard types (`Vec<i32>`, `Vec<usize>`, `Vec<(usize, usize)>`, graph types, etc.) are already covered. Only add a new parser for genuinely new types.

4. **Schema alignment**: The `ProblemSchemaEntry` fields should list **constructor parameters** (what the user provides), not internal derived fields. For example, if `m` and `n` are derived from a matrix, only list `matrix` and `k` in the schema. Field names must match the struct field names exactly (used for JSON serialization and CLI flag mapping).

## Step 4.6: Add canonical model example to example_db

Add a builder function in `src/example_db/model_builders.rs` that constructs a small, canonical instance for this model. Register it in `build_model_examples()`.

Also add `canonical_model_example_specs()` **in the model file itself** (gated by `#[cfg(feature = "example-db")]`), and register it in the category `mod.rs` example chain (e.g., `specs.extend(<module>::canonical_model_example_specs());`). See any existing model in `src/models/graph/` for the pattern.

This example is now the canonical source for:
- `pred create --example <PROBLEM_SPEC>`
- paper/example exports via `load-model-example()` in `reductions.typ`
- example-db invariants tested in `src/unit_tests/example_db.rs`

## Step 4.7: Implement Direct ILP Rule When Claimed

If the issue explicitly says the model is solvable by reducing **directly** to ILP, implement `src/rules/<problem>_ilp.rs` in the **same PR** as the model. This is the one exception to the normal "one item per PR" policy: the direct `<Problem> -> ILP` rule is part of the model feature, not optional follow-up work.

Completeness bar:
- Feature-gate the rule under `ilp-solver` and register it normally
- Add exact overhead expressions and any required size-field getters; metadata must match the constructed ILP exactly
- Add strong tests in `src/unit_tests/rules/<problem>_ilp.rs`: structure/metadata, closed-loop semantics vs the source problem or brute force, extraction, `solve_reduced()` or ILP path coverage when appropriate, and weighted/infeasible/pathological regressions whenever the model semantics admit them
- Update CLI/example-db/paper paths so the claimed ILP solver route is actually usable and documented
- Verify with ILP-enabled workspace commands, not just non-ILP unit tests

A direct ILP rule shipped with a model issue must match the completeness bar of a standalone production ILP reduction. Do not add a stub just to satisfy the issue text.

## Step 5: Write unit tests

Create `src/unit_tests/models/<category>/<name>.rs`:

Every model needs **at least 3 test functions** (the structural reviewer enforces this). Choose from the coverage areas below — pick whichever are relevant to the model:

- **Creation/basic** — exercise constructor inputs, key accessors, `dims()` / `num_variables()`.
- **Evaluation** — valid and invalid configs so the feasibility boundary or aggregate contribution is explicit.
- **Direction / sense** — verify runtime optimization sense only for models that use `Extremum<_>`.
- **Solver** — brute-force `solve()` returns the correct aggregate value; if witnesses are supported, verify `find_witness()` / `find_all_witnesses()` as well.
- **Serialization** — round-trip serde (when the model is used in CLI/example-db flows).
- **Paper example** — verify the worked example from the paper entry (see below).

If Step 4.7 applies, also add a dedicated ILP rule test file under `src/unit_tests/rules/<problem>_ilp.rs`. Use strong direct-to-ILP reductions in the repo as the reference bar: the tests should validate the actual formulation semantics, not just that an ILP file exists.

When you add `test_<name>_paper_example`, it should:
1. Construct the same instance shown in the paper's example figure
2. Evaluate the solution from the issue's **Expected Outcome** section as shown in the paper and assert it is valid (and optimal for optimization problems)
3. Use `BruteForce` to confirm the claimed optimum/satisfying solution count when the instance is small enough for unit tests

This test is usually written **after** Step 6 (paper entry), once the example instance and expected outcome are finalized. If writing tests before the paper, use the issue's Example Instance + Expected Outcome as the source of truth and come back to verify consistency.

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

**Reproducibility:** The example section must include a `pred-commands()` call showing the create/solve/evaluate pipeline. The `pred create --example ...` spec must be derived from the loaded canonical example data via the helper pattern in `write-model-in-paper`; do not hand-write a bare alias and assume the default variant matches.

### 6d. Build and verify

```bash
make paper  # Must compile without errors
```

Checklist: display name registered, notation self-contained, background present, algorithms cited, example with diagram present, evaluation shown, paper compiles.

## Step 7: Verify

For ordinary model-only work:
```bash
make test clippy  # Must pass
```

If Step 4.7 applied, run ILP-enabled workspace verification instead:
```bash
cargo clippy --all-targets --features ilp-highs -- -D warnings
cargo test --features "ilp-highs example-db" --workspace --verbose
```

Structural and quality review is handled by the `review-pipeline` stage, not here. The run stage just needs to produce working code.

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
| Using the wrong aggregate wrapper | Objective models use `Max` / `Min` / `Extremum`, witness models use `bool`, aggregate-only models use a fold value like `Sum` / `And` |
| Not registering in `mod.rs` | Must update both `<category>/mod.rs` and `models/mod.rs` |
| Forgetting `declare_variants!` | Required for variant complexity metadata and registry-backed load/serialize/solve dispatch |
| Wrong aggregate wrapper | Use `Max` / `Min` / `Extremum` for objective problems, `Or` for existential witness problems, and `Sum` / `And` (or a custom aggregate) for value-only folds |
| Wrong `declare_variants!` syntax | Entries no longer use `opt` / `sat`; one entry per problem may be marked `default` |
| Forgetting CLI alias | Must add lowercase entry in `problem_name.rs` `resolve_alias()` |
| Adding a hand-written decision model | Use `Decision<P>` wrapper instead — see `decision_problem_meta!` + `register_decision_variant!` in `src/models/graph/minimum_vertex_cover.rs` for the pattern |
| Inventing short aliases | Only use well-established literature abbreviations (MIS, SAT, TSP); do NOT invent new ones |
| Forgetting CLI flags | Schema-driven create needs matching CLI flags in `CreateArgs` for each `ProblemSchemaEntry` field (snake_case → kebab-case). Also add to `flag_map()`. |
| Missing type parser | If the problem uses a new field type, add a handler in `parse_field_value()` in `create.rs` |
| Schema lists derived fields | Schema should list constructor params, not internal fields (e.g., `matrix, k` not `matrix, m, n, k`) |
| Missing canonical model example | Add a builder in `src/example_db/model_builders.rs` and keep it aligned with paper/example workflows |
| Paper example not tested | Must include `test_<name>_paper_example` that verifies the exact instance, solution, and solution count shown in the paper |
| Claiming direct ILP solving but leaving `<Problem> -> ILP` for later | If the issue promises a direct ILP path, implement that rule in the same PR with exact overhead metadata and production-level ILP tests |
