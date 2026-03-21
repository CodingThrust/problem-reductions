# IntegralFlowWithMultipliers Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `IntegralFlowWithMultipliers` graph model, including registry metadata, CLI creation/inspection support, canonical example coverage, and a paper entry, for issue `#290`.

**Architecture:** Implement this as a fixed-variant satisfaction model over `DirectedGraph` with one integer variable per arc. Feasibility checks enforce capacity bounds, multiplier-scaled conservation at non-terminal vertices, and sink net inflow `>= requirement`; metadata and examples follow the issue/comment decisions (`u64` data, no misleading generalized-flow alias, concrete complexity `(max_capacity + 1)^num_arcs`). Execute the paper work in a separate batch after the Rust/example-db work is complete so the paper can load the final canonical example data.

**Tech Stack:** Rust workspace, registry inventory metadata, Clap CLI, example-db, Typst paper, `make` verification targets.

---

## Issue Context

- Issue: `#290` `[Model] IntegralFlowWithMultipliers`
- Pipeline state: Ready -> claimed to In progress by `run-pipeline`
- `issue-context` result: `Good` label present, action = `create-pr`, no PR to resume
- Companion rule issue exists: `#363` `[Rule] PARTITION to INTEGRAL FLOW WITH MULTIPLIERS`
- Use the repaired issue body/comment decisions as source of truth:
  - store `multipliers`, `capacities`, `requirement` as `u64`
  - `multipliers.len() == num_vertices`, with source/sink entries ignored
  - no `Generalized Flow` alias
  - use complexity string `"(max_capacity + 1)^num_arcs"`
  - use the cleaned YES instance as the canonical worked example
  - keep the repaired diamond-graph NO instance in tests only

## Batch Layout

- **Batch 1:** add-model Steps 1-5.5
  - Rust model, registry wiring, CLI creation/help, canonical example, non-paper tests
- **Batch 2:** add-model Step 6
  - `docs/paper/reductions.typ` entry + paper/example verification

## Reference Files

- Model pattern: `src/models/graph/directed_two_commodity_integral_flow.rs`
- Metadata/size-field pattern: `src/models/graph/undirected_two_commodity_integral_flow.rs`
- Trait smoke coverage: `src/unit_tests/trait_consistency.rs`
- CLI creation pattern: `problemreductions-cli/src/commands/create.rs`
- Paper pattern: `docs/paper/reductions.typ` entries for `DirectedTwoCommodityIntegralFlow` and `MaximumIndependentSet`

## Batch 1

### Task 1: Add the failing model/unit tests first

**Files:**
- Create: `src/unit_tests/models/graph/integral_flow_with_multipliers.rs`

**Step 1: Write the failing tests**

Add tests that cover:
- creation/accessors/dims/size getters
- a satisfying assignment for the repaired YES instance
- an unsatisfying assignment for the repaired NO instance
- multiplier-scaled conservation failure
- sink net-inflow check uses `>= requirement`
- wrong config length returns `false`
- serde round-trip
- brute-force solver finds a satisfying config for the YES instance and none for the NO instance

Also add a paper-example-oriented test scaffold that can be finalized once the canonical example is wired.

**Step 2: Run the focused tests and confirm they fail**

Run:

```bash
cargo test integral_flow_with_multipliers --lib
```

Expected: compilation/test failures because the model type and module do not exist yet.

**Step 3: Commit the red state only if it is helpful**

Optional; skip if the branch policy prefers keeping the red state local.

### Task 2: Implement the Rust model and registry wiring

**Files:**
- Create: `src/models/graph/integral_flow_with_multipliers.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Implement the model with the smallest code that satisfies Task 1**

Add `IntegralFlowWithMultipliers` with:
- `ProblemSchemaEntry` metadata
- `ProblemSizeFieldEntry` declaring at least `num_vertices`, `num_arcs`, `max_capacity`, and `requirement`
- fields: `graph`, `source`, `sink`, `multipliers`, `capacities`, `requirement`
- constructor validation:
  - `capacities.len() == graph.num_arcs()`
  - `multipliers.len() == graph.num_vertices()`
  - `source`/`sink` in bounds and distinct
  - non-terminal multipliers are positive
  - each capacity fits into `usize` for `dims()`
- accessors/getters: `graph()`, `capacities()`, `multipliers()`, `source()`, `sink()`, `requirement()`, `num_vertices()`, `num_arcs()`, `max_capacity()`
- feasibility helper using `i128` balance accumulation per vertex
  - enforce `0 <= f(a) <= c(a)` implicitly from `dims()`/config decoding
  - for each non-terminal `v`, require `h(v) * inflow(v) == outflow(v)`
  - require sink net inflow `incoming - outgoing >= requirement`
- `Problem` impl:
  - `NAME = "IntegralFlowWithMultipliers"`
  - `Metric = bool`
  - `variant() = variant_params![]`
  - `dims() = capacities.iter().map(|c| c + 1)`
  - `evaluate()` delegates to feasibility
- `SatisfactionProblem` impl
- `declare_variants! { default sat IntegralFlowWithMultipliers => "(max_capacity + 1)^num_arcs", }`
- canonical example spec using the repaired YES instance from the issue
- test module link at the bottom

Wire the new model into:
- `src/models/graph/mod.rs` docs/mod exports/example-spec chain
- `src/models/mod.rs`
- `src/lib.rs` prelude exports
- `src/unit_tests/trait_consistency.rs`

**Step 2: Run the focused library tests and make them green**

Run:

```bash
cargo test integral_flow_with_multipliers --lib
cargo test trait_consistency --lib
```

Expected: the new model tests and trait smoke test pass.

**Step 3: Refactor only after green**

Keep helper methods local to the model file; do not generalize flow utilities prematurely.

### Task 3: Add CLI creation/help coverage and example-db wiring

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write/extend failing CLI tests first**

Add tests that cover:
- `pred create IntegralFlowWithMultipliers` with `--arcs`, `--capacities`, `--multipliers`, `--source`, `--sink`, `--requirement`
- `pred inspect`/`pred show` exposing the new size fields and schema fields
- error cases for missing `--multipliers` or wrong multiplier/capacity lengths
- `pred create --example IntegralFlowWithMultipliers` returning the canonical JSON shape

**Step 2: Run the focused CLI tests and confirm they fail**

Run:

```bash
cargo test -p problemreductions-cli integral_flow_with_multipliers
```

Expected: failures because the CLI does not yet know the problem/flags.

**Step 3: Implement the minimal CLI support**

Add:
- new `CreateArgs` field for `--multipliers`
- new `CreateArgs` field for singular `--requirement`
- `all_data_flags_empty()` coverage for both new fields
- after-help table/examples entry in `problemreductions-cli/src/cli.rs`
- `example_for()` entry in `problemreductions-cli/src/commands/create.rs`
- create-arm in `problemreductions-cli/src/commands/create.rs` using `parse_directed_graph(...)`
  - require `--arcs`
  - parse `--capacities` (default all ones if omitted only if that matches existing CLI norms; otherwise require explicitly)
  - require `--multipliers`, `--source`, `--sink`, `--requirement`
  - validate vector lengths and vertex bounds
  - construct `IntegralFlowWithMultipliers::new(...)`

If the registry alias machinery already handles the canonical name, do **not** add a made-up short alias.

**Step 4: Run the focused CLI tests and make them green**

Run:

```bash
cargo test -p problemreductions-cli integral_flow_with_multipliers
```

### Task 4: Run the Batch 1 verification set

**Files:**
- None beyond the files above

**Step 1: Run the verification commands**

Run:

```bash
cargo test integral_flow_with_multipliers
make fmt
make clippy
```

If `make clippy` is too broad while iterating, use targeted `cargo clippy --all-targets --all-features -- -D warnings` and finish with the repo target before closing Batch 1.

**Step 2: Commit the Batch 1 implementation**

Suggested message:

```bash
git add src/models/graph/integral_flow_with_multipliers.rs \
        src/models/graph/mod.rs src/models/mod.rs src/lib.rs \
        src/unit_tests/models/graph/integral_flow_with_multipliers.rs \
        src/unit_tests/trait_consistency.rs \
        problemreductions-cli/src/cli.rs \
        problemreductions-cli/src/commands/create.rs \
        problemreductions-cli/tests/cli_tests.rs
git commit -m "Add IntegralFlowWithMultipliers model"
```

## Batch 2

### Task 5: Add the paper entry and paper-example validation

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/graph/integral_flow_with_multipliers.rs`

**Step 1: Write the failing paper/example test first**

Complete the paper-example test in the model unit test file so it:
- builds the canonical YES instance
- evaluates the documented satisfying config
- confirms the brute-force solver finds at least one satisfying config

**Step 2: Run the focused test and confirm red if needed**

Run:

```bash
cargo test integral_flow_with_multipliers_paper_example --lib
```

Expected: failure until the final documented example/config is aligned.

**Step 3: Implement the paper entry**

In `docs/paper/reductions.typ`:
- add display name entry for `IntegralFlowWithMultipliers`
- add `problem-def("IntegralFlowWithMultipliers")`
- use the issue-approved formulation: directed graph, vertex multipliers on non-terminals, sink requirement `>= R`
- cite the Sahni 1974 and Jewell 1962 references
- explain the polynomial special case when all multipliers are 1 / continuous-flow relaxation
- use the canonical YES example from the issue and load it from example-db
- include a small directed-network figure and `pred-commands()` snippet derived from the canonical example data

**Step 4: Run the paper/example verification**

Run:

```bash
cargo test integral_flow_with_multipliers_paper_example --lib
make paper
```

**Step 5: Commit the paper batch**

Suggested message:

```bash
git add docs/paper/reductions.typ src/unit_tests/models/graph/integral_flow_with_multipliers.rs
git commit -m "Document IntegralFlowWithMultipliers"
```

## Final Verification

Run before cleanup/push:

```bash
make check
git status --short
```

Success criteria:
- new model is discoverable through the registry/CLI
- `pred create IntegralFlowWithMultipliers ...` works
- canonical example exists and is used by tests/paper
- the plan file can be deleted before the final push, per `issue-to-pr`
