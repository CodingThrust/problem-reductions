# Plan: Add RootedTreeStorageAssignment Model

**Issue:** #409 — [Model] RootedTreeStorageAssignment
**Skill:** add-model
**Execution:** `issue-to-pr` Batch 1 covers add-model Steps 1-5 plus the canonical example wiring; Batch 2 covers add-model Step 6 (paper) after the model, CLI path, and tests are stable.

## Issue Packet Summary

- `Good` label is present and `issue-context` returned `action=create-pr`, so this run should create a new PR from branch `issue-409`.
- Use the maintainer fix comments on #409 as the implementation source of truth for the concrete encoding, cleaned-up examples, and complexity string.
- Associated open rule issue exists: #424 `[Rule] Rooted Tree Arrangement to Rooted Tree Storage Assignment`.

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `RootedTreeStorageAssignment` |
| 2 | Mathematical definition | Given a finite set `X`, a collection `C = {X_1, ..., X_n}` of subsets of `X`, and an integer `K`, decide whether there exists a directed rooted tree `T = (X, A)` and supersets `X_i' ⊇ X_i` such that every `X_i'` forms a directed path in `T` and `sum_i |X_i' - X_i| <= K` |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | None |
| 5 | Struct fields | `universe_size: usize`, `subsets: Vec<Vec<usize>>`, `bound: usize` |
| 6 | Configuration space | `vec![universe_size; universe_size]`, where `config[v]` is the parent of vertex `v` and the root satisfies `config[root] = root` |
| 7 | Feasibility check | The config must encode exactly one rooted tree on `0..universe_size`, and every subset must lie on a single ancestor-descendant chain so its minimal path extension is well-defined |
| 8 | Objective function | `bool` — true iff the total minimal extension cost across all subsets is at most `bound` |
| 9 | Best known exact algorithm | Brute-force over parent arrays; use complexity string `"universe_size^universe_size"` with getters `universe_size()` and `num_subsets()` |
| 10 | Solving strategy | Existing `BruteForce` works directly over the parent-array encoding; no ILP path is required in this issue |
| 11 | Category | `set` |
| 12 | Expected outcome | The canonical YES instance uses `X = {0,1,2,3,4}`, subsets `[{0,2},{1,3},{0,4},{2,4}]`, `K = 1`, and satisfying config `[0,0,0,1,2]` (tree edges `0→1`, `0→2`, `1→3`, `2→4`) |

## Design Decisions

### Category and API shape

- Implement this under `src/models/set/` because the input is a universe plus subset family, and the closest existing models are `ConsecutiveSets`, `TwoDimensionalConsecutiveSets`, and `SetBasis`.
- Follow the newer validated-constructor pattern used by `EnsembleComputation` and `TwoDimensionalConsecutiveSets`: expose `try_new(...) -> Result<Self, String>`, keep `new(...)` as the panicking convenience wrapper, and validate again on deserialize.

### Tree encoding and evaluation semantics

- Keep the parent-array encoding from the maintainer fix comment: `config[v] = parent(v)` with exactly one root encoded by `config[root] = root`.
- `evaluate()` should:
  1. Reject wrong-length configs and parent values outside `0..universe_size`.
  2. Validate that the config encodes exactly one rooted tree (one self-parent root, every other node reaches that root, no cycles).
  3. Precompute `depth[v]` and an `is_ancestor(u, v)` helper.
  4. For each subset, verify that all vertices are pairwise comparable by ancestry after sorting by depth.
  5. Compute the minimal path-extension cost as the number of vertices on the shallowest-to-deepest path that are not already in the subset.
  6. Return `false` immediately if any subset is infeasible or if the running total exceeds `bound`.

### CLI and registry notes

- Registry-backed discovery comes from `ProblemSchemaEntry` plus `declare_variants!`; do not add manual load/serialize dispatch code.
- `problem_name.rs` should not need changes unless a test proves otherwise, because `find_problem_type_by_alias()` already matches canonical names case-insensitively.
- CLI creation can reuse the existing `--universe`, `--sets`, and `--bound` flags, so only `create.rs` and the help table in `cli.rs` should need updates.

## Batch 1: Model, CLI, Example, Tests

### Step 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/set/rooted_tree_storage_assignment.rs`

Add tests that fail before the model exists:

1. `test_rooted_tree_storage_assignment_creation`
   - Construct the issue's YES instance.
   - Assert `universe_size() == 5`, `num_subsets() == 4`, `bound() == 1`, and `dims() == vec![5; 5]`.
2. `test_rooted_tree_storage_assignment_evaluate_yes_instance`
   - Assert `evaluate(&[0, 0, 0, 1, 2])` is `true`.
3. `test_rooted_tree_storage_assignment_rejects_invalid_tree_configs`
   - Cover wrong length, out-of-range parent, multiple roots, and a directed cycle.
4. `test_rooted_tree_storage_assignment_no_instance`
   - Use the same subsets with `bound = 0`, run `BruteForce::find_all_satisfying`, assert no solutions.
5. `test_rooted_tree_storage_assignment_solver_finds_known_solution`
   - Run brute force on the YES instance and assert `[0, 0, 0, 1, 2]` is among the satisfying configs.
6. `test_rooted_tree_storage_assignment_serialization`
   - Round-trip serde and confirm normalized subsets/bound survive.

Run the targeted test and confirm it fails because the model is not registered yet.

### Step 2: Add the model implementation

**Files:**
- Create: `src/models/set/rooted_tree_storage_assignment.rs`

Implement add-model Steps 1, 1.5, and 2 here:

1. Register `ProblemSchemaEntry` with:
   - `name = "RootedTreeStorageAssignment"`
   - `display_name = "Rooted Tree Storage Assignment"`
   - `aliases = &[]`
   - `fields = universe_size / subsets / bound`
2. Define the struct plus validated deserialize helper.
3. Add `try_new`, `new`, and getters:
   - `universe_size()`
   - `num_subsets()`
   - `bound()`
   - `subsets()`
4. Normalize subsets into sorted unique vectors and reject out-of-range elements.
5. Implement private helpers for:
   - tree validation / root detection
   - depth computation
   - ancestor checks
   - per-subset extension-cost computation
6. Implement `Problem`:
   - `NAME = "RootedTreeStorageAssignment"`
   - `Metric = bool`
   - `dims() = vec![self.universe_size; self.universe_size]`
   - `evaluate()` per the design above
   - `variant() = crate::variant_params![]`
7. Implement `SatisfactionProblem`.
8. Add `declare_variants! { default sat RootedTreeStorageAssignment => "universe_size^universe_size" }`.
9. Add `canonical_model_example_specs()` using the issue's YES instance and config `[0, 0, 0, 1, 2]`.
10. Link the test file with `#[cfg(test)] #[path = "../../unit_tests/models/set/rooted_tree_storage_assignment.rs"]`.

After each chunk, rerun the focused unit test until it turns green.

### Step 3: Register the model and example chain

**Files:**
- Modify: `src/models/set/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

1. Add the new module and `pub use` in `src/models/set/mod.rs`.
2. Extend the set-category `canonical_model_example_specs()` chain with `rooted_tree_storage_assignment::canonical_model_example_specs()`.
3. Re-export `RootedTreeStorageAssignment` from `src/models/mod.rs`.
4. Add it to the crate prelude in `src/lib.rs`.

Run a focused compile/test command after registration so missing exports show up early.

### Step 4: Add CLI creation support and CLI regression coverage

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

1. In `problemreductions-cli/src/commands/create.rs`:
   - Import `RootedTreeStorageAssignment`.
   - Add an `example_for()` entry like:
     - `"RootedTreeStorageAssignment" => "--universe 5 --sets \"0,2;1,3;0,4;2,4\" --bound 1"`
   - Add the `match` arm that parses `--universe`, `--sets`, and `--bound`, converts `--bound` to `usize`, and constructs the validated model.
2. Add a CLI regression test near the existing JSON creation tests:
   - `test_create_rooted_tree_storage_assignment_json`
   - Assert the JSON `type`, `universe_size`, `subsets`, and `bound`.
3. In `problemreductions-cli/src/cli.rs`, add `RootedTreeStorageAssignment --universe, --sets, --bound` to the help table.
4. Do not touch `problem_name.rs` unless a failing test proves the canonical-name lookup is insufficient.

Run the new CLI test first in RED/GREEN style, then rerun the model tests.

### Step 5: Batch-1 verification

Run enough fresh verification to justify the implementation commit before moving to paper work:

```bash
cargo test rooted_tree_storage_assignment
cargo test -p problemreductions-cli test_create_rooted_tree_storage_assignment_json
make test
make clippy
```

If `make test` or `make clippy` surfaces unrelated failures, stop and record them in the PR summary instead of papering over them.

## Batch 2: Paper Entry and Paper-Example Alignment

### Step 6: Document the model in the Typst paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/set/rooted_tree_storage_assignment.rs`

1. Add the display-name dictionary entry for `RootedTreeStorageAssignment`.
2. Add `#problem-def("RootedTreeStorageAssignment")[...][...]` with:
   - the corrected formal definition from the issue packet
   - short background plus the Garey & Johnson / Gavril citations already accepted in the issue review
   - an example based on the canonical YES instance
   - a `pred-commands()` block derived from the canonical example data
3. Add or refresh `test_rooted_tree_storage_assignment_paper_example` so the unit test matches the paper's exact instance and satisfying config.
4. Re-run `make paper` and fix any schema/example-export mismatches it reports.

## Final Verification and Handoff

Before the implementation summary comment and push, re-run the full verification set:

```bash
make test
make clippy
make paper
```

Expected implementation commits:

1. `Add plan for #409: [Model] RootedTreeStorageAssignment`
2. `Implement #409: [Model] RootedTreeStorageAssignment`
3. `chore: remove plan file after implementation`

When posting the PR summary, explicitly call out:

- the parent-array encoding choice
- the fact that CLI reused existing `--universe/--sets/--bound` flags
- any deviations from this plan
