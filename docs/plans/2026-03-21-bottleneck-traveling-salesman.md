# Bottleneck Traveling Salesman Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.
>
> **Repo skill reference:** Follow [.claude/skills/add-model/SKILL.md](../../.claude/skills/add-model/SKILL.md) Steps 1-7. Batch 1 below covers add-model Steps 1-5.5. Batch 2 covers add-model Step 6.

**Goal:** Add the `BottleneckTravelingSalesman` graph model from issue #240 as a fixed `SimpleGraph + i32` optimization problem, with brute-force support, CLI creation, canonical example data, and paper documentation.

**Architecture:** Reuse the existing edge-subset representation and Hamiltonian-cycle validation used by `TravelingSalesman`, but change the objective aggregation from sum to max. Keep the new model non-generic as requested by the issue comments (`SimpleGraph` plus `i32` only), register it through the schema/variant inventory, and wire its canonical K5 example through the graph example-spec chain so both the CLI example flow and the paper can load the same source of truth.

**Tech Stack:** Rust workspace, inventory registry metadata, `pred` CLI, Typst paper, brute-force solver, GitHub issue #240 canonical example.

---

## Batch 1: Model, Registration, CLI, Tests

### Task 1: Write the model tests first

**Files:**
- Create: `src/unit_tests/models/graph/bottleneck_traveling_salesman.rs`
- Reference: `src/unit_tests/models/graph/traveling_salesman.rs`
- Reference: `src/models/graph/traveling_salesman.rs`

**Step 1: Write failing tests for the new model**

Create `src/unit_tests/models/graph/bottleneck_traveling_salesman.rs` with tests that assume a future `BottleneckTravelingSalesman` type exists and covers the exact issue semantics:

- `test_bottleneck_traveling_salesman_creation_and_size_getters`
  - Construct the issue K5 graph with 10 edges and the issue weights `[5, 4, 4, 5, 4, 1, 2, 1, 5, 4]`
  - Assert `graph().num_vertices() == 5`
  - Assert `num_vertices() == 5`
  - Assert `num_edges() == 10`
  - Assert `dims() == vec![2; 10]`
- `test_bottleneck_traveling_salesman_evaluate_valid_and_invalid_configs`
  - Valid config for the issue optimum: `[0, 1, 1, 0, 1, 0, 1, 0, 0, 1]` corresponding to edges `(0,2), (0,3), (1,2), (1,4), (3,4)`
  - Assert `evaluate(valid) == SolutionSize::Valid(4)`
  - Assert a degree-violating config and a disconnected-subtour config both return `SolutionSize::Invalid`
- `test_bottleneck_traveling_salesman_direction`
  - Assert `direction() == Direction::Minimize`
- `test_bottleneck_traveling_salesman_solver_unique_optimum`
  - Use `BruteForce::find_all_best`
  - Assert exactly one best config
  - Assert it matches the issue optimum config
  - Assert its objective is `SolutionSize::Valid(4)`
- `test_bottleneck_traveling_salesman_serialization`
  - Round-trip through `serde_json`
  - Assert graph size and weights are preserved
- `test_bottleneck_traveling_salesman_paper_example`
  - Use the same K5 instance and assert the worked-example config is valid and optimal
  - Assert every best solution found by brute force has bottleneck `4`

**Step 2: Run the new tests to verify RED**

Run:

```bash
cargo test test_bottleneck_traveling_salesman_creation_and_size_getters --lib
```

Expected: FAIL at compile time because `BottleneckTravelingSalesman` does not exist yet.

**Step 3: Do not implement here**

Stop after the failure. The implementation belongs in Task 2.

### Task 2: Implement the model and make the tests pass

**Files:**
- Create: `src/models/graph/bottleneck_traveling_salesman.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Implement the new model file**

Create `src/models/graph/bottleneck_traveling_salesman.rs` with:

- `inventory::submit!` schema entry:
  - `name: "BottleneckTravelingSalesman"`
  - `display_name: "Bottleneck Traveling Salesman"`
  - `aliases: &[]`
  - `dimensions: &[]`
  - `fields` for `graph: SimpleGraph` and `edge_weights: Vec<i32>`
- `#[derive(Debug, Clone, Serialize, Deserialize)]`
- struct:

```rust
pub struct BottleneckTravelingSalesman {
    graph: SimpleGraph,
    edge_weights: Vec<i32>,
}
```

- inherent methods:
  - `new(graph: SimpleGraph, edge_weights: Vec<i32>)`
  - `graph(&self) -> &SimpleGraph`
  - `weights(&self) -> Vec<i32>`
  - `set_weights(&mut self, weights: Vec<i32>)`
  - `edges(&self) -> Vec<(usize, usize, i32)>`
  - `num_vertices(&self) -> usize`
  - `num_edges(&self) -> usize`
  - `is_weighted(&self) -> bool` returning `true`
  - `is_valid_solution(&self, config: &[usize]) -> bool`
- `Problem` impl:
  - `const NAME = "BottleneckTravelingSalesman"`
  - `type Metric = SolutionSize<i32>`
  - `variant() -> crate::variant_params![]`
  - `dims() -> vec![2; self.graph.num_edges()]`
  - `evaluate()`:
    - reject non-Hamiltonian-cycle configs
    - reuse `super::traveling_salesman::is_hamiltonian_cycle(&self.graph, &selected)`
    - compute the maximum selected edge weight
    - return `SolutionSize::Valid(max_weight)` for feasible configs
- `OptimizationProblem` impl with `Direction::Minimize`
- canonical example spec under `#[cfg(feature = "example-db")]` using the issue K5 instance and optimum config `[0, 1, 1, 0, 1, 0, 1, 0, 0, 1]`
- `crate::declare_variants! { default opt BottleneckTravelingSalesman => "num_vertices^2 * 2^num_vertices", }`
- the unit-test link at the bottom

**Step 2: Register the module locally**

Update `src/models/graph/mod.rs` to:

- add the module doc bullet for `BottleneckTravelingSalesman`
- add `pub(crate) mod bottleneck_traveling_salesman;`
- add `pub use bottleneck_traveling_salesman::BottleneckTravelingSalesman;`
- extend `canonical_model_example_specs()` with `bottleneck_traveling_salesman::canonical_model_example_specs()`

**Step 3: Run the targeted tests to verify GREEN**

Run:

```bash
cargo test bottleneck_traveling_salesman --lib
```

Expected: PASS for the new model tests.

**Step 4: Refactor only if needed**

Keep the model self-contained unless test failures prove a shared helper needs adjustment. Do not generalize `TravelingSalesman` or add type parameters.

### Task 3: Register the model across exports and catalog surfaces

**Files:**
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Add crate-level exports**

Update:

- `src/models/mod.rs` re-export list to include `BottleneckTravelingSalesman`
- `src/lib.rs` graph-model re-exports and `prelude` re-exports to include `BottleneckTravelingSalesman`

**Step 2: Run a focused compile check**

Run:

```bash
cargo test test_bottleneck_traveling_salesman_direction --lib
```

Expected: PASS and no unresolved-export errors.

### Task 4: Add CLI create support for the new model

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Extend the CLI problem groups**

Update `problemreductions-cli/src/commands/create.rs` in all existing edge-weight graph groupings so `BottleneckTravelingSalesman` is treated like `TravelingSalesman`:

- problem example args list near the top-level example matcher
- `all_data_flags_empty()` or equivalent edge-weight validation grouping if the file requires it
- explicit create handler match arm for graph + `--edge-weights`
- random instance generation arm

The concrete constructor in both explicit and random creation paths should be:

```rust
BottleneckTravelingSalesman::new(graph, edge_weights)
```

Import the new model type if the file’s `use problemreductions::models::graph::{...}` list needs it.

**Step 2: Update help text**

In `problemreductions-cli/src/cli.rs`, add `BottleneckTravelingSalesman` to the edge-weight graph help line, for example by expanding:

```text
MaxCut, MaxMatching, TSP, BottleneckTravelingSalesman  --graph, --edge-weights
```

Do not invent a short alias such as `BTSP`; the issue and registry metadata do not establish one.

**Step 3: Verify RED/GREEN with a CLI smoke test**

Run:

```bash
cargo run -p problemreductions-cli -- create BottleneckTravelingSalesman --graph 0-1,0-2,0-3,0-4,1-2,1-3,1-4,2-3,2-4,3-4 --edge-weights 5,4,4,5,4,1,2,1,5,4
```

Expected: command succeeds and prints serialized JSON for the new problem without unknown-problem or flag-validation errors.

### Task 5: Finish verification for Batch 1

**Files:**
- Modify only if previous tasks exposed missing imports or example wiring errors

**Step 1: Run the graph/example-db focused checks**

Run:

```bash
cargo test bottleneck_traveling_salesman --lib
cargo test example_db --lib
```

Expected: PASS. The example-db test proves the canonical example spec is wired into the graph chain correctly.

**Step 2: Inspect git diff for Batch 1 scope**

Run:

```bash
git diff -- src/models/graph src/models/mod.rs src/lib.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
```

Expected: only the new model, exports, CLI hooks, and related docs/comments.

## Batch 2: Paper and Reference Updates

### Task 6: Add the paper entry and bibliography support

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`

**Step 1: Add the missing bibliography entry if needed**

Check whether `Gilmore1964` already exists in `docs/paper/references.bib`. If not, add a BibTeX entry for:

- P. C. Gilmore and R. E. Gomory
- "Sequencing a one state-variable machine: a solvable case of the traveling salesman problem"
- Operations Research 12 (1964), 655-679

Reuse the existing `heldkarp1962` entry already present in the repo.

**Step 2: Add the display name**

Add:

```typst
"BottleneckTravelingSalesman": [Bottleneck Traveling Salesman],
```

to the `display-name` dictionary in `docs/paper/reductions.typ`.

**Step 3: Add the `problem-def` entry**

Place a new `problem-def("BottleneckTravelingSalesman")` entry near `TravelingSalesman`. The entry should:

- define the optimization version explicitly
- mention the decision version from Garey and Johnson as the threshold form it subsumes
- cite Held-Karp for the `O(n^2 2^n)` exact algorithm discussion
- cite Gilmore-Gomory only as the polynomial-time special-case remark from the G&J note
- load the canonical example from the example-db path rather than duplicating raw data
- use the issue’s K5 example and explain:
  - the unique optimum bottleneck is `4`
  - the optimal BTSP tour differs from the minimum-total-weight TSP tour

**Step 4: Build the paper**

Run:

```bash
make paper
```

Expected: PASS with no Typst errors or missing-citation failures.

## Final Verification

### Task 7: Run the repo-required verification before handoff

**Files:**
- No intentional edits in this task

**Step 1: Run the full verification commands**

Run:

```bash
make test
make clippy
```

Expected: PASS.

If `make paper` generated ignored exports under `docs/src/reductions/`, confirm they remain unstaged.

**Step 2: Manual CLI sanity check**

Run:

```bash
cargo run -p problemreductions-cli -- create --example BottleneckTravelingSalesman
```

Expected: the canonical example loads successfully through the example-db pathway.

**Step 3: Record implementation notes for the PR summary**

Capture:

- files added/modified
- whether any shared helper had to move beyond `traveling_salesman::is_hamiltonian_cycle`
- whether the paper entry deviated from the initial structure because of Typst/example-db constraints

These notes feed the implementation-summary PR comment required by `issue-to-pr`.
