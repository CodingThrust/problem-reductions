# MinimumDummyActivitiesPert Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `MinimumDummyActivitiesPert` model from issue #301, including registry/CLI/example-db support and a paper entry that matches the canonical issue example.

**Architecture:** Represent a candidate PERT encoding with one binary decision per precedence arc in the input DAG: either merge the predecessor's finish event with the successor's start event, or keep a dummy arc between them. `evaluate()` will quotient the selected endpoint merges, rebuild the induced event network, reject cyclic or reachability-changing encodings, and return the number of distinct dummy arcs that remain. This keeps `dims()` at `vec![2; num_arcs]`, matches the issue's worked example, and makes the 6-task witness brute-forceable in tests.

**Tech Stack:** Rust workspace, `DirectedGraph`, `inventory` problem registry, `BruteForce`, `problemreductions-cli`, Typst paper.

---

## Batch 1: Model, registrations, CLI, and tests

### Task 1: Add failing model tests for the issue example and constructor guards

**Files:**
- Create: `src/unit_tests/models/graph/minimum_dummy_activities_pert.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Write the failing tests**

Add tests covering:
- constructor/accessor basics on a small DAG
- constructor rejection of cyclic input graphs
- the issue's 6-task example, with the merge-selection config for `A->C`, `B->E`, `C->F` evaluating to `SolutionSize::Valid(2)`
- an invalid config that merges both incoming arcs to `D`, causing spurious reachability and therefore `SolutionSize::Invalid`
- brute-force optimality on the 6-task example (`find_best()` returns value `2`)
- serde round-trip

Use the precedence-arc order from `DirectedGraph::arcs()` and build the test helpers from the issue example directly.

**Step 2: Run the targeted test to verify RED**

Run: `cargo test minimum_dummy_activities_pert --lib`

Expected: FAIL because the model module does not exist yet.

**Step 3: Write minimal implementation**

Create `src/models/graph/minimum_dummy_activities_pert.rs` with:
- `ProblemSchemaEntry` for `MinimumDummyActivitiesPert`
- struct field `graph: DirectedGraph`
- `try_new(graph) -> Result<Self, String>` enforcing `graph.is_dag()`
- `new(graph)` panicking on invalid input
- getters `graph()`, `num_vertices()`, `num_arcs()`
- helpers:
  - ordered precedence arcs (`graph.arcs()`)
  - a tiny union-find over the `2 * num_vertices` task endpoints
  - event-network construction from a binary merge config
  - transitive-reachability checks on both the input DAG and the derived event DAG
- `Problem` impl with `dims() = vec![2; self.graph.num_arcs()]`
- `OptimizationProblem` impl with `Direction::Minimize`
- `declare_variants! { default opt MinimumDummyActivitiesPert => "2^num_arcs" }`
- canonical example spec using the issue's 6-task instance and optimal merge config
- test link at file bottom

Interpret config bit `1` as "merge this precedence arc" and bit `0` as "keep a dummy arc unless the quotient already identifies those endpoints". Count dummy arcs after quotienting by unique ordered event-class pairs.

**Step 4: Run the targeted tests to verify GREEN**

Run: `cargo test minimum_dummy_activities_pert --lib`

Expected: PASS for the new model tests.

**Step 5: Commit**

```bash
git add src/models/graph/minimum_dummy_activities_pert.rs src/unit_tests/models/graph/minimum_dummy_activities_pert.rs src/models/graph/mod.rs
git commit -m "Add MinimumDummyActivitiesPert model"
```

### Task 2: Register the model throughout the library and example database

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs` or prelude export surface if the graph models are re-exported there
- Modify: `src/unit_tests/example_db.rs`

**Step 1: Write the failing example-db / registry test**

Add a test in `src/unit_tests/example_db.rs` asserting that `find_model_example()` resolves `MinimumDummyActivitiesPert` with an empty variant map and exposes a non-empty optimal config.

**Step 2: Run the targeted test to verify RED**

Run: `cargo test test_find_model_example_minimum_dummy_activities_pert --lib`

Expected: FAIL because the model is not fully exported/registered yet.

**Step 3: Register the model and canonical example**

Update module exports so:
- `src/models/graph/mod.rs` declares and re-exports `minimum_dummy_activities_pert`
- `src/models/graph/mod.rs::canonical_model_example_specs()` extends with the new model's example specs
- `src/models/mod.rs` re-exports `MinimumDummyActivitiesPert`
- any public prelude/lib exports remain consistent with the other graph models

**Step 4: Run the targeted test to verify GREEN**

Run: `cargo test test_find_model_example_minimum_dummy_activities_pert --lib`

Expected: PASS.

**Step 5: Commit**

```bash
git add src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/example_db.rs
git commit -m "Register MinimumDummyActivitiesPert"
```

### Task 3: Add CLI create support and CLI-facing tests

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Write the failing CLI tests**

Add tests in `problemreductions-cli/src/commands/create.rs` covering:
- `all_data_flags_empty()` treats `--arcs` as input for this problem path
- `create()` serializes a `MinimumDummyActivitiesPert` JSON payload from `--arcs "0>2,0>3,1>3,1>4,2>5" --num-vertices 6`
- `create()` rejects cyclic input with the constructor error

**Step 2: Run the targeted tests to verify RED**

Run: `cargo test -p problemreductions-cli minimum_dummy_activities_pert`

Expected: FAIL because the create arm/help text do not exist yet.

**Step 3: Implement minimal CLI support**

Add:
- a `create()` match arm using `parse_directed_graph(args.arcs, args.num_vertices)` and `MinimumDummyActivitiesPert::try_new`
- a problem-specific usage string, example usage snippet, and `after_help` entry in `problemreductions-cli/src/cli.rs`
- an example command in the schema-driven help list near the other directed-graph problems

Use:
`pred create MinimumDummyActivitiesPert --arcs "0>2,0>3,1>3,1>4,2>5" --num-vertices 6`

**Step 4: Run the targeted tests to verify GREEN**

Run: `cargo test -p problemreductions-cli minimum_dummy_activities_pert`

Expected: PASS.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "Add CLI support for MinimumDummyActivitiesPert"
```

### Task 4: Add broad verification for Batch 1

**Files:**
- No new files

**Step 1: Run focused library and CLI tests**

Run:
- `cargo test minimum_dummy_activities_pert --lib`
- `cargo test -p problemreductions-cli minimum_dummy_activities_pert`

Expected: both PASS.

**Step 2: Run workspace checks likely to catch registration breakage**

Run:
- `make test`
- `make clippy`

Expected: PASS.

**Step 3: Commit if any cleanup/refactor was needed**

```bash
git add -A
git commit -m "Polish MinimumDummyActivitiesPert integration"
```

Only commit if Batch 1 verification required code changes.

## Batch 2: Paper entry and paper-example consistency

### Task 5: Add the paper entry and paper-example coverage

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/graph/minimum_dummy_activities_pert.rs`

**Step 1: Write the failing paper-example test**

Extend the model test file with `test_minimum_dummy_activities_pert_paper_example` that:
- builds the exact 6-task issue example
- evaluates the documented optimal merge config
- asserts `SolutionSize::Valid(2)`
- checks `BruteForce::find_best()` also returns value `2`

If this is already covered by Task 1's issue-example test, keep the dedicated paper-example test as a named wrapper around the same witness.

**Step 2: Run the targeted test to verify RED**

Run: `cargo test test_minimum_dummy_activities_pert_paper_example --lib`

Expected: FAIL until the named paper-example coverage exists.

**Step 3: Write the paper entry**

Update `docs/paper/reductions.typ`:
- add `"MinimumDummyActivitiesPert": [Minimum Dummy Activities in PERT Networks],` to `display-name`
- add `#problem-def("MinimumDummyActivitiesPert")[ ... ][ ... ]`
- derive the example from `#let x = load-model-example("MinimumDummyActivitiesPert")`
- explain the 6-task precedence DAG, the three selected merges, and the two remaining dummy arcs
- add a `pred-commands()` block using `problem-spec(x)`
- cite the Garey-Johnson / Krishnamoorthy-Deo background and note the brute-force complexity with a footnote if no sharper exact algorithm is being claimed

Keep the paper example aligned with the canonical example-db instance, using 1-indexed prose only where mathematically clearer.

**Step 4: Run the targeted test and paper build to verify GREEN**

Run:
- `cargo test test_minimum_dummy_activities_pert_paper_example --lib`
- `make paper`

Expected: PASS.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ src/unit_tests/models/graph/minimum_dummy_activities_pert.rs
git commit -m "Document MinimumDummyActivitiesPert in paper"
```

### Task 6: Final verification and pipeline handoff

**Files:**
- No new files

**Step 1: Run final verification**

Run:
- `make test`
- `make clippy`
- `make paper`
- `git status --short`

Expected:
- all commands PASS
- only intended tracked files are modified
- generated `docs/paper/data/examples.json` and other ignored exports remain unstaged

**Step 2: Prepare the issue-to-pr cleanup**

After implementation succeeds:
- keep the branch clean
- delete this plan file before the final push
- summarize the implementation in the PR comment, including the merge-vs-dummy encoding choice and any deviations (ideally none)

**Step 3: Final commit cleanup**

```bash
git rm docs/plans/2026-03-22-minimum-dummy-activities-pert.md
git commit -m "chore: remove plan file after implementation"
```
