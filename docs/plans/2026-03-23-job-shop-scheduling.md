# JobShopScheduling Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `JobShopScheduling` satisfaction model, CLI creation support, canonical example data, tests, and paper documentation for issue `#510`.

**Architecture:** Represent a witness as one permutation per machine, encoded with concatenated Lehmer-code segments. `evaluate()` will decode those machine orders, orient the disjunctive graph, reject cyclic orientations, and compute earliest start times by longest-path propagation; the schedule is feasible iff every task completes by the global deadline. This intentionally supersedes the issue body’s unverified “start-time variable” sketch because the issue comments and example statistics (`6! * 6! = 518400` task-orderings) clearly assume machine-order enumeration.

**Tech Stack:** Rust core model registry, serde/inventory metadata, `problemreductions-cli` create command, example-db exports, Typst paper docs.

---

## Batch 1: add-model Steps 1-5.5

### Task 1: Write the model tests first

**Files:**
- Create: `src/unit_tests/models/misc/job_shop_scheduling.rs`
- Reference: `src/unit_tests/models/misc/flow_shop_scheduling.rs`

**Step 1: Write the failing tests**

Add targeted tests that define the intended semantics before any production code exists:
- `test_job_shop_scheduling_creation_and_dims`
  - Construct `JobShopScheduling::new(2, vec![vec![(0, 3), (1, 4)], vec![(1, 2), (0, 3), (1, 2)]], 20)`
  - Assert `num_processors() == 2`, `num_jobs() == 2`, `num_tasks() == 5`
  - Assert `dims() == vec![2, 1, 3, 2, 1]` for machine-0 tasks `[j0.t0, j1.t1]` and machine-1 tasks `[j0.t1, j1.t0, j1.t2]`
- `test_job_shop_scheduling_evaluate_issue_example`
  - Use the corrected issue instance with 5 jobs / 12 tasks / deadline `20`
  - Assert the machine-order config `[0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0]` evaluates to `true`
- `test_job_shop_scheduling_rejects_machine_overlap_or_cycle`
  - Use a small 2-machine instance whose chosen machine orders force a precedence cycle, and assert `evaluate()` returns `false`
- `test_job_shop_scheduling_invalid_config_and_serialization`
  - Reject wrong-length or out-of-range Lehmer digits
  - Round-trip through `serde_json`
- `test_job_shop_scheduling_solver_small_instance`
  - Use a tiny 2-job / 2-machine instance where brute force can find a satisfying witness

**Step 2: Run the tests to verify they fail**

Run: `cargo test job_shop_scheduling --lib`

Expected: FAIL because `JobShopScheduling` and its test linkage do not exist yet.

**Step 3: Commit the red test file once it exists and fails cleanly**

Run:
```bash
git add src/unit_tests/models/misc/job_shop_scheduling.rs
git commit -m "test: add red tests for JobShopScheduling"
```

### Task 2: Implement the core model and schedule evaluator

**Files:**
- Create: `src/models/misc/job_shop_scheduling.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Add the model scaffold**

Implement `JobShopScheduling` with:
- `num_processors: usize`
- `jobs: Vec<Vec<(usize, u64)>>`
- `deadline: u64`

Register `ProblemSchemaEntry` with constructor-facing fields:
- `num_processors: usize`
- `jobs: Vec<Vec<(usize, u64)>>`
- `deadline: u64`

Constructor invariants:
- every processor index is `< num_processors`
- consecutive tasks within a job use different processors (Garey-Johnson formulation)
- `num_processors > 0` when the instance contains tasks

Add getters:
- `num_processors()`
- `jobs()`
- `deadline()`
- `num_jobs()`
- `num_tasks()`

**Step 2: Implement the permutation-based witness encoding**

Add helpers that:
- flatten tasks into stable task ids in `(job_index, task_index)` order
- group task ids by machine in ascending task-id order
- decode one Lehmer-code segment per machine into an ordered list of task ids
- concatenate segment dimensions in `dims()` as `[k, k-1, ..., 1]` for each machine with `k` assigned tasks

Use `Problem::Metric = bool`, `variant() = crate::variant_params![]`, and `impl SatisfactionProblem for JobShopScheduling {}`.

**Step 3: Implement `evaluate()` by disjunctive-graph orientation**

`evaluate(config)` should:
- reject invalid config length or out-of-range Lehmer digits
- decode per-machine task orders
- build directed edges:
  - job-precedence edge `u -> v` with weight `len(u)`
  - machine-order edge `u -> v` with weight `len(u)`
- run topological sort on the oriented graph; if cyclic, return `false`
- compute earliest start times by longest-path DP over the DAG
- return `true` iff every task finishes by `deadline`

Expose a small helper such as `schedule_from_config(&self, config) -> Option<Vec<u64>>` if it keeps the paper/example tests readable.

**Step 4: Register the model and complexity metadata**

Update exports in:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`

Add:
```rust
crate::declare_variants! {
    default sat JobShopScheduling => "factorial(num_tasks)",
}
```

Use `factorial(num_tasks)` rather than the issue body’s `factorial(num_jobs)` because the chosen witness representation enumerates machine task orders, and `factorial(num_jobs)` undercounts jobs with more than one operation on the same machine.

**Step 5: Run the focused tests**

Run: `cargo test job_shop_scheduling --lib`

Expected: PASS for the new model tests.

**Step 6: Commit the core model**

Run:
```bash
git add src/models/misc/job_shop_scheduling.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs
git commit -m "feat: add JobShopScheduling model"
```

### Task 3: Register example-db coverage and trait consistency

**Files:**
- Modify: `src/models/misc/mod.rs`
- Modify: `src/unit_tests/trait_consistency.rs`
- Modify: `src/example_db/model_builders.rs` (only if needed by the existing pattern)

**Step 1: Add the canonical example spec in the model file**

Inside `src/models/misc/job_shop_scheduling.rs`, add:
- `canonical_model_example_specs()`
- the corrected issue example with deadline `20`
- canonical satisfying config `[0, 0, 0, 0, 0, 0, 1, 3, 0, 1, 1, 0]`

Then wire the model into the `misc::canonical_model_example_specs()` chain.

**Step 2: Extend smoke coverage**

Add a `check_problem_trait(...)` entry for `JobShopScheduling` in `src/unit_tests/trait_consistency.rs`.

If example-db tests require any explicit expectations for the new example, add them in the existing example-db test module instead of inventing a new harness.

**Step 3: Run focused tests**

Run:
```bash
cargo test trait_consistency
cargo test example_db --features example-db
```

Expected: PASS, with the new model visible to registry/example-db consumers.

**Step 4: Commit the registration changes**

Run:
```bash
git add src/models/misc/mod.rs src/unit_tests/trait_consistency.rs src/example_db/model_builders.rs
git commit -m "test: register JobShopScheduling example coverage"
```

### Task 4: Add CLI discovery and `pred create` support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Step 1: Add the CLI input shape**

Add a new `CreateArgs` flag:
- `--job-tasks`

Format:
- semicolon-separated jobs
- comma-separated operations per job
- each operation encoded as `processor:length`
- example: `--job-tasks "0:3,1:4;1:2,0:3,1:2;0:4,1:3"`

Update:
- `all_data_flags_empty()`
- the “Flags by problem type” help table
- usage/help text strings mentioning `JobShopScheduling`

**Step 2: Add name resolution and constructor parsing**

In `problem_name.rs`, add the lowercase canonical mapping for `jobshopscheduling`.

In `create.rs`:
- add an example string for `JobShopScheduling`
- parse `--job-tasks`, `--deadline`, and optional `--num-processors`
- infer `num_processors` as `1 + max(processor index)` when the flag is omitted
- validate every parsed processor index against the resolved processor count
- serialize `JobShopScheduling::new(...)`

**Step 3: Add CLI tests first, then implementation wiring**

Use the existing `create.rs` unit-test section to add:
- one success case that round-trips the issue-style example
- one failure case for malformed `processor:length`
- one failure case for a missing `--job-tasks`

Run:
```bash
cargo test -p problemreductions-cli create::tests::job_shop
```

Expected: RED before the parser arm exists, then GREEN after the arm/help updates are added.

**Step 4: Commit the CLI support**

Run:
```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs
git commit -m "feat: add JobShopScheduling CLI support"
```

## Batch 2: add-model Step 6

### Task 5: Document the model in the paper and align the worked example

**Files:**
- Modify: `docs/paper/reductions.typ`
- Reference: `docs/paper/reductions.typ` `FlowShopScheduling` entry

**Step 1: Add the display name and `problem-def` entry**

Register:
- `"JobShopScheduling": [Job-Shop Scheduling]`

Then add a `#problem-def("JobShopScheduling")[...][...]` entry that:
- defines jobs as ordered task sequences with processor assignments and lengths
- explicitly calls out the Garey-Johnson “consecutive tasks use different processors” formulation
- explains the permutation-per-machine witness representation used in the implementation

**Step 2: Reuse the corrected canonical example**

In the paper body:
- load the example with `load-model-example("JobShopScheduling")`
- decode the machine-order config into earliest start times using the same reasoning as the Rust helper
- present the corrected 5-job / 2-machine / deadline-20 instance
- include a simple Gantt-style figure and a short explanation that the derived makespan is `19`

**Step 3: Add a paper-example test**

Back in `src/unit_tests/models/misc/job_shop_scheduling.rs`, add `test_job_shop_scheduling_paper_example` that:
- constructs the same canonical example
- evaluates the canonical config
- optionally checks the derived start-time vector or makespan `19`

**Step 4: Run verification**

Run:
```bash
cargo test job_shop_scheduling --lib
make paper
```

Expected: PASS, with the paper example and canonical example in sync.

**Step 5: Commit the paper/docs batch**

Run:
```bash
git add docs/paper/reductions.typ src/unit_tests/models/misc/job_shop_scheduling.rs
git commit -m "docs: add JobShopScheduling paper entry"
```

## Final Verification

After all tasks are green, run the full issue gate:

```bash
make test
make clippy
```

If the paper/example/export workflow updates tracked generated files that belong with the feature, stage them explicitly and keep ignored `docs/src/reductions/` outputs out of the commit.
