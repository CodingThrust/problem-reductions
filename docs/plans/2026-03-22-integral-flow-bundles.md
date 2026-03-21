# IntegralFlowBundles Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `IntegralFlowBundles` for issue `#293`, including the model, direct ILP solver reduction, CLI/example-db support, and paper coverage.

**Architecture:** Implement `IntegralFlowBundles` as a `SatisfactionProblem` under `src/models/misc/` with one integer decision variable per directed arc. Each arc domain is bounded by the minimum bundle capacity among bundles containing that arc; evaluation enforces bundle capacities, flow conservation at nonterminals, and required net inflow at the sink. Because the issue explicitly promises ILP solving, also add a direct `IntegralFlowBundles -> ILP<i32>` reduction with a canonical rule example and paper theorem in the same branch.

**Tech Stack:** Rust workspace, inventory registry, Clap CLI, example-db fixtures, Typst paper, optional `ilp-solver` feature.

---

## Batch 1: Implement the model, solver rule, tests, and CLI

### Task 1: Add failing model tests and the minimal model scaffold

**Files:**
- Create: `src/models/misc/integral_flow_bundles.rs`
- Create: `src/unit_tests/models/misc/integral_flow_bundles.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing test**

Add model tests that pin down the issue-backed example and the repo review checklist:
- `test_integral_flow_bundles_creation_and_getters`
- `test_integral_flow_bundles_dims_use_tight_arc_bounds`
- `test_integral_flow_bundles_evaluate_yes_and_no_examples`
- `test_integral_flow_bundles_rejects_bad_bundle_sum_or_conservation`
- `test_integral_flow_bundles_serialization`

The yes-instance should be the issue example:
- `num_vertices = 4`
- `arcs = [(0,1), (0,2), (1,3), (2,3), (1,2), (2,1)]`
- `bundles = [[0,1], [2,5], [3,4]]`
- `bundle_capacities = [1,1,1]`
- `source = 0`, `sink = 3`, `requirement = 1`
- satisfying config: `[1,0,1,0,0,0]`

The no-instance should reuse the same data with `requirement = 2`.

**Step 2: Run test to verify it fails**

Run:
```bash
cargo test test_integral_flow_bundles_creation_and_getters --lib
```

Expected: FAIL because `IntegralFlowBundles` does not exist yet.

**Step 3: Write minimal implementation**

Create `src/models/misc/integral_flow_bundles.rs` with:
- `ProblemSchemaEntry` and `ProblemSizeFieldEntry`
- `IntegralFlowBundles` struct with fields:
  - `num_vertices: usize`
  - `arcs: Vec<(usize, usize)>`
  - `source: usize`
  - `sink: usize`
  - `bundles: Vec<Vec<usize>>`
  - `bundle_capacities: Vec<u64>`
  - `requirement: u64`
- constructor validation:
  - bundle count matches capacity count
  - every arc index in every bundle is in bounds
  - every arc appears in at least one bundle
  - `source`/`sink` are valid and distinct
  - each per-arc upper bound fits `usize` for `dims()`
- getters:
  - `num_vertices()`
  - `num_arcs()`
  - `num_bundles()`
  - accessors for all stored fields
- `Problem` impl:
  - `Metric = bool`
  - `dims()` returns one domain size per arc: `min(capacity over containing bundles) + 1`
  - `evaluate()` checks config length, domain bounds, bundle sums, flow conservation, and net sink inflow
- `SatisfactionProblem` impl
- `declare_variants! { default sat IntegralFlowBundles => "2^num_arcs" }`
- `canonical_model_example_specs()` using the yes-instance above
- `#[cfg(test)]` link to the new unit test file

Register the type in:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`

**Step 4: Run test to verify it passes**

Run:
```bash
cargo test integral_flow_bundles --lib
```

Expected: PASS for the new model tests.

**Step 5: Commit**

```bash
git add src/models/misc/integral_flow_bundles.rs src/unit_tests/models/misc/integral_flow_bundles.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs
git commit -m "Add IntegralFlowBundles model"
```

### Task 2: Add the direct ILP reduction and rule tests

**Files:**
- Create: `src/rules/integralflowbundles_ilp.rs`
- Create: `src/unit_tests/rules/integralflowbundles_ilp.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write the failing test**

Add rule tests that cover both structure and closed-loop solving:
- `test_reduction_creates_valid_ilp`
- `test_integral_flow_bundles_to_ilp_closed_loop`
- `test_extract_solution_returns_arc_flows`
- `test_solve_reduced_integral_flow_bundles`

Target behavior:
- one ILP variable per arc, integer and lower-bounded by `0`
- one bundle-capacity constraint per bundle
- one conservation equality per nonterminal vertex
- one sink-inflow inequality enforcing `>= requirement`
- objective can be zero/minimize-zero because the source problem is a satisfaction problem

Use the canonical yes-instance from Task 1 as the closed-loop fixture.

**Step 2: Run test to verify it fails**

Run:
```bash
cargo test --features ilp-solver integralflowbundles_ilp
```

Expected: FAIL because the reduction does not exist yet.

**Step 3: Write minimal implementation**

Create `src/rules/integralflowbundles_ilp.rs`:
- reduce `IntegralFlowBundles` to `ILP<i32>`
- use one integer ILP variable `f_a` per arc
- add bundle constraints `sum_{a in I_j} f_a <= c_j`
- add conservation equalities for each `v != source, sink`
- add a sink-requirement inequality using incoming minus outgoing sink flow
- return identity extraction (`target_solution.to_vec()`)
- register overhead with:
  - `num_vars = "num_arcs"`
  - `num_constraints = "num_bundles + num_vertices - 2 + 1"`
- add `canonical_rule_example_specs()` based on the issue example

Register the rule in `src/rules/mod.rs`.

**Step 4: Run test to verify it passes**

Run:
```bash
cargo test --features ilp-solver integralflowbundles_ilp
```

Expected: PASS for the new reduction tests.

**Step 5: Commit**

```bash
git add src/rules/integralflowbundles_ilp.rs src/unit_tests/rules/integralflowbundles_ilp.rs src/rules/mod.rs
git commit -m "Add IntegralFlowBundles to ILP reduction"
```

### Task 3: Add CLI creation support and example-db coverage

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write the failing test**

Add CLI tests for:
- `pred create IntegralFlowBundles --arcs "0>1,0>2,1>3,2>3,1>2,2>1" --bundles "0,1;2,5;3,4" --bundle-capacities 1,1,1 --source 0 --sink 3 --requirement 1 --num-vertices 4`
- missing `--bundles` or `--bundle-capacities`
- `pred create --example IntegralFlowBundles`

Also add or extend example-db assertions if needed so both:
- `find_model_example("IntegralFlowBundles")`
- `find_rule_example("IntegralFlowBundles" -> "ILP")`
remain covered by existing generic tests.

**Step 2: Run test to verify it fails**

Run:
```bash
cargo test -p problemreductions-cli integral_flow_bundles
```

Expected: FAIL because the CLI flags and create arm do not exist yet.

**Step 3: Write minimal implementation**

Update `problemreductions-cli/src/cli.rs`:
- add help-table entries for `IntegralFlowBundles`
- add `bundles: Option<String>` and `bundle_capacities: Option<String>` to `CreateArgs`
- treat both flags as data-bearing in `all_data_flags_empty()`

Update `problemreductions-cli/src/commands/create.rs`:
- add usage/example strings for `IntegralFlowBundles`
- parse `--arcs` with the existing directed-arc helper
- parse `--bundles` as semicolon-separated groups of comma-separated arc indices
- parse `--bundle-capacities` as comma-separated nonnegative integers
- add the `IntegralFlowBundles` create arm and serialize the new problem
- verify `pred create --example IntegralFlowBundles` works via the example-db registration from Tasks 1 and 2

Use the registry-backed alias flow; do not add manual alias tables unless a concrete failing test proves they are still required.

**Step 4: Run test to verify it passes**

Run:
```bash
cargo test -p problemreductions-cli integral_flow_bundles
cargo test example_db --features "example-db ilp-solver"
```

Expected: PASS.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/tests/cli_tests.rs src/models/misc/mod.rs src/rules/mod.rs
git commit -m "Wire IntegralFlowBundles through CLI and example db"
```

## Batch 2: Paper and final verification

### Task 4: Document the model and ILP reduction in the paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the failing test**

Add the paper content first, then use the paper build as the failing/passing gate:
- display name entry for `IntegralFlowBundles`
- `problem-def("IntegralFlowBundles")` with the formal bundled-flow definition
- example narrative tied to the canonical yes-instance and its satisfying config
- `pred-commands(...)` block using `pred create --example IntegralFlowBundles`
- `reduction-rule("IntegralFlowBundles", "ILP")` explaining the per-arc-variable ILP formulation

**Step 2: Run test to verify it fails or exposes missing pieces**

Run:
```bash
make paper
```

Expected: FAIL until the paper entry, citations, and example wiring are complete.

**Step 3: Write minimal implementation**

Add:
- `display-name["IntegralFlowBundles"]`
- model background with Garey-Johnson/Sahni citations
- the issue’s yes/no example, explicitly checking bundle sums and sink inflow
- a short ILP reduction theorem referencing the new solver rule

Mirror the style of:
- `UndirectedTwoCommodityIntegralFlow`
- `BinPacking -> ILP`

**Step 4: Run test to verify it passes**

Run:
```bash
make paper
```

Expected: PASS.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "Document IntegralFlowBundles and its ILP reduction"
```

### Task 5: Final repo verification before push

**Files:**
- Modify: none expected

**Step 1: Run the focused verification suite**

Run:
```bash
cargo test integral_flow_bundles --lib
cargo test --features ilp-solver integralflowbundles_ilp
cargo test -p problemreductions-cli integral_flow_bundles
cargo test example_db --features "example-db ilp-solver"
```

Expected: PASS.

**Step 2: Run the broader safety net**

Run:
```bash
make check
make paper
```

Expected: PASS.

**Step 3: Inspect the tree**

Run:
```bash
git status --short
```

Expected: only intended tracked changes; no leftover `docs/plans/*.md` after the later cleanup commit.

**Step 4: Commit any last adjustments**

```bash
git add -A
git commit -m "Polish IntegralFlowBundles implementation"
```
