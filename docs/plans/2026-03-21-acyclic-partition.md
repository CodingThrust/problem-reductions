# AcyclicPartition Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `AcyclicPartition` model for issue `#226`, including registry/CLI/example-db integration, tests, and a paper entry.

**Architecture:** `AcyclicPartition<W>` will reuse the existing `DirectedGraph` topology type and store per-vertex weights, per-arc costs, a per-part weight bound, and a global inter-partition cost bound. `evaluate()` will treat each config entry as a partition label, validate per-part totals, accumulate cut cost across arcs whose endpoints land in different parts, construct the quotient digraph on used labels, and accept iff that quotient graph is acyclic and the total cut cost is within bound.

**Tech Stack:** Rust, serde, inventory, `DirectedGraph`, brute-force solver, Typst paper, CLI integration tests.

---

## Skill Mapping

- Primary implementation skill: `add-model`
- `add-model` Step 2: Task 2
- `add-model` Step 3: Task 3
- `add-model` Steps 4-4.6: Tasks 3-4
- `add-model` Step 5: Tasks 1-2
- `add-model` Step 6: Task 6
- `add-model` Step 7: Tasks 5 and 7

## Issue Notes

- Issue: `#226` `[Model] AcyclicPartition`
- Repo pipeline preflight: `Good` label present, no existing PR, action=`create-pr`
- Associated rule issue exists: `#247` `[Rule] 3-SATISFIABILITY to ACYCLIC PARTITION`
- The issue comments claiming `DirectedGraph` is missing are outdated; the repo already provides `src/topology/directed_graph.rs`
- Scope rule: model-only PR. Do **not** add the reduction rule in this branch.

## Batch Structure

- **Batch 1:** Implement model, register it, add CLI/example support, add tests, verify build/tests
- **Batch 2:** Add the paper entry after the model/example data exists, then rerun verification

### Task 1: Add failing model tests for the issue example

**Files:**
- Create: `src/unit_tests/models/graph/acyclic_partition.rs`
- Modify: `src/models/graph/acyclic_partition.rs`

**Step 1: Write the failing test file**

Create tests that encode the issue’s 6-vertex example and cover:
- constructor/accessors/dims (`dims() == vec![6; 6]`)
- valid YES config `[0, 1, 0, 2, 2, 2]`
- invalid config for `K = 4`
- invalid config that creates a quotient-cycle
- brute-force solver count for the 4 canonical satisfying configs
- serde round-trip

**Step 2: Run the focused test target to verify RED**

Run: `cargo test acyclic_partition --lib`
Expected: FAIL because `AcyclicPartition` does not exist yet.

**Step 3: Add the minimal model skeleton needed to compile**

Create `src/models/graph/acyclic_partition.rs` with:
- `ProblemSchemaEntry`
- struct fields
- constructor/accessors/size getters
- `Problem` + `SatisfactionProblem` impls
- `declare_variants!`
- `#[cfg(test)]` test link

Use `todo!()` only where necessary to keep the first green step small.

**Step 4: Re-run the focused tests**

Run: `cargo test acyclic_partition --lib`
Expected: FAIL for the intended behavior assertions, not missing symbols.

**Step 5: Commit**

```bash
git add src/models/graph/acyclic_partition.rs src/unit_tests/models/graph/acyclic_partition.rs
git commit -m "test: add AcyclicPartition model coverage"
```

### Task 2: Implement the model behavior until the tests pass

**Files:**
- Modify: `src/models/graph/acyclic_partition.rs`

**Step 1: Implement constructor validation**

Enforce:
- `vertex_weights.len() == graph.num_vertices()`
- `arc_costs.len() == graph.num_arcs()`

Expose:
- `graph()`
- `vertex_weights()`
- `arc_costs()`
- `weight_bound()`
- `cost_bound()`
- `set_vertex_weights()` / `set_arc_costs()`
- `is_weighted()`
- `num_vertices()` / `num_arcs()`

**Step 2: Implement `dims()` and label-range validation**

Use `vec![self.graph.num_vertices(); self.graph.num_vertices()]`.
Reject configs whose length is wrong or whose labels are outside `0..num_vertices`.

**Step 3: Implement `evaluate()` / `is_valid_solution()`**

Implement the exact feasibility checks in this order:
1. Config length and label range
2. Per-part accumulated vertex weight `<= weight_bound`
3. Inter-partition arc cost `<= cost_bound`
4. Quotient digraph acyclicity

Quotient-graph construction requirements:
- Ignore unused labels
- Compress used labels to dense `0..q-1`
- Add one quotient arc per distinct cross-part arc direction
- A self-loop must be impossible because intra-part arcs are ignored

**Step 4: Add helper(s) only if they simplify the logic**

Acceptable helpers:
- `used_partition_labels(config) -> Vec<usize>`
- `quotient_graph(config) -> DirectedGraph`
- `inter_partition_cost(config) -> W::Sum`

Do not introduce extra abstractions unless the tests force them.

**Step 5: Run the focused model tests to verify GREEN**

Run: `cargo test acyclic_partition --lib`
Expected: PASS

**Step 6: Refactor while staying green**

Keep the implementation straightforward; prefer one-pass accumulation over repeated scans.

**Step 7: Commit**

```bash
git add src/models/graph/acyclic_partition.rs src/unit_tests/models/graph/acyclic_partition.rs
git commit -m "feat: implement AcyclicPartition model"
```

### Task 3: Register the model in the crate and example-db

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/models/graph/acyclic_partition.rs`

**Step 1: Register the graph module export**

Add:
- `pub(crate) mod acyclic_partition;`
- `pub use acyclic_partition::AcyclicPartition;`
- graph-module docs bullet
- `canonical_model_example_specs()` chaining entry

**Step 2: Register crate-level re-exports**

Add `AcyclicPartition` to:
- `src/models/mod.rs`
- `src/lib.rs` prelude export list

**Step 3: Add the canonical model example in the model file**

Under `#[cfg(feature = "example-db")]`, add one `ModelExampleSpec` using the issue’s YES instance:
- 6 vertices
- arcs `(0,1),(0,2),(1,3),(1,4),(2,4),(2,5),(3,5),(4,5)`
- vertex weights `[2,3,2,1,3,1]`
- arc costs `[1; 8]`
- bounds `B=5`, `K=5`
- canonical config `[0,1,0,2,2,2]`
- optimal value `true`

**Step 4: Run a focused example-db-related test**

Run: `cargo test example_db --lib`
Expected: PASS for the touched example-db checks.

**Step 5: Commit**

```bash
git add src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/models/graph/acyclic_partition.rs
git commit -m "feat: register AcyclicPartition model"
```

### Task 4: Add CLI alias and `pred create` support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write failing CLI tests**

Add tests for:
- `pred create AcyclicPartition/i32 --arcs ... --vertex-weights ... --arc-weights ... --weight-bound 5 --cost-bound 5`
- `pred create --example AcyclicPartition/i32`
- `pred inspect AcyclicPartition/i32 --json` reporting `num_vertices` and `num_arcs`

**Step 2: Run the focused CLI tests to verify RED**

Run: `cargo test -p problemreductions-cli acyclic_partition`
Expected: FAIL because the alias/create branch does not exist yet.

**Step 3: Implement CLI support**

Update `problem_name.rs`:
- resolve lowercase `"acyclicpartition"` to `"AcyclicPartition"`

Update `create.rs`:
- import `AcyclicPartition`
- add help/example strings for `DirectedGraph`/weight-bound/cost-bound usage
- parse `--arcs`
- parse `--vertex-weights` against `graph.num_vertices()`
- parse `--arc-weights` against `graph.num_arcs()`
- require `--weight-bound`
- require `--cost-bound`
- construct `AcyclicPartition::new(...)`

Reuse existing directed-graph parsing helpers instead of adding new parsers.

**Step 4: Re-run the focused CLI tests to verify GREEN**

Run: `cargo test -p problemreductions-cli acyclic_partition`
Expected: PASS

**Step 5: Commit**

```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/tests/cli_tests.rs
git commit -m "feat: add CLI support for AcyclicPartition"
```

### Task 5: Batch-1 verification

**Files:**
- No code changes expected

**Step 1: Run targeted verification**

Run:
- `cargo test acyclic_partition --lib`
- `cargo test -p problemreductions-cli acyclic_partition`

Expected: PASS

**Step 2: Run broader verification**

Run:
- `make test`
- `make clippy`

Expected: PASS

**Step 3: Commit only if verification required a fix**

```bash
git add -A
git commit -m "test: fix AcyclicPartition verification issues"
```

### Task 6: Add the paper entry

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Write the failing paper-oriented test/check expectation**

Use the existing canonical example as the source of truth for the paper text:
- display name `Acyclic Partition`
- formal definition using `DirectedGraph`, vertex weights, arc costs, `B`, `K`
- background referencing Garey & Johnson ND15 and DAG partitioning applications
- worked example matching the canonical config `[0,1,0,2,2,2]`

**Step 2: Implement the paper entry**

Add:
- display-name dictionary entry
- `#problem-def("AcyclicPartition")[...]` block

Conventions to follow:
- load the model example with `load-model-example("AcyclicPartition")`
- explain the quotient graph explicitly
- include a directed figure with highlighted partition groups or highlighted quotient-order intuition
- keep the example text consistent with the unit tests and canonical example data

**Step 3: Run paper build to verify GREEN**

Run: `make paper`
Expected: PASS

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add AcyclicPartition paper entry"
```

### Task 7: Final verification and pipeline handoff

**Files:**
- No code changes expected

**Step 1: Run final repo checks**

Run:
- `make test`
- `make clippy`
- `make paper`

Expected: PASS

**Step 2: Inspect git status**

Run: `git status --short`
Expected: clean except for ignored/generated artifacts

**Step 3: Prepare implementation summary notes for the PR comment**

Capture:
- model file(s) added/modified
- CLI support added
- tests added
- paper entry added
- any deviations from the issue comments (notably that `DirectedGraph` already existed)
