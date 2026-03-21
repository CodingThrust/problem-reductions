# ExpectedRetrievalCost Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `ExpectedRetrievalCost` satisfaction model, register it across the library/CLI/example-db, and document the canonical worked example in the paper.

**Architecture:** Implement a new `misc` model whose configuration assigns each record to one of `m` sectors (`vec![m; n]`). Evaluation aggregates probability mass per sector, computes the circular latency objective from the issue, and returns `true` exactly when the expected retrieval cost is at most the configured bound.

**Tech Stack:** Rust workspace (`problemreductions`, `problemreductions-cli`), serde/inventory registry, canonical example-db, Typst paper.

---

**Issue:** #408 - [Model] ExpectedRetrievalCost  
**Skill:** add-model  
**Associated rule:** #423 - [Rule] Partition / 3-Partition to Expected Retrieval Cost

## Information Checklist

| # | Item | Value |
|---|------|-------|
| 1 | Problem name | `ExpectedRetrievalCost` |
| 2 | Mathematical definition | Given record probabilities summing to 1, a number of sectors `m`, and a bound `K`, decide whether the records can be partitioned into `m` sectors so the expected rotational latency is at most `K` |
| 3 | Problem type | Satisfaction (`Metric = bool`) |
| 4 | Type parameters | None |
| 5 | Struct fields | `probabilities: Vec<f64>`, `num_sectors: usize`, `bound: f64` |
| 6 | Configuration space | `vec![num_sectors; num_records]`; each config entry is a 0-based sector assignment for one record |
| 7 | Feasibility check | Config length must equal `num_records`, every assignment must be `< num_sectors`, probabilities must already be valid at construction, and the computed expected cost must be `<= bound + EPSILON` |
| 8 | Objective function | Aggregate sector masses `p(R_i)` and compute `sum_(i,j) p(R_i) * p(R_j) * d(i,j)` using the circular latency from the issue |
| 9 | Best known exact algorithm | Brute-force over all `m^n` assignments; complexity string `"num_sectors ^ num_records"` |
| 10 | Solving strategy | Existing `BruteForce` solver is sufficient |
| 11 | Category | `misc` |
| 12 | Expected outcome | Canonical YES example from the issue uses probabilities `[0.2, 0.15, 0.15, 0.2, 0.1, 0.2]`, `m = 3`, `K = 1.01`, and satisfying config `[0, 1, 2, 1, 0, 2]` (0-based form of the issue's sector allocation). Brute-force confirms 54 satisfying assignments and minimum cost `1.0025`. |

## Design Notes

- Keep configs repo-standard and 0-based even though the issue writes sectors as `1..m`; paper text can explain the human-readable 1-based sectors while code/tests use `0..m-1`.
- Constructor validation should enforce: non-empty probabilities, `num_sectors >= 2`, every probability finite and in `[0, 1]`, and total probability within a small tolerance of `1.0`.
- Use helper methods so tests can assert intermediate behavior directly:
  - `num_records()`
  - `num_sectors()`
  - `probabilities()`
  - `bound()`
  - `sector_masses(config) -> Option<Vec<f64>>`
  - `expected_cost(config) -> Option<f64>`
  - `is_valid_solution(config) -> bool`
- Register `ProblemSizeFieldEntry` for `num_records` and `num_sectors`; this keeps future reduction overhead metadata straightforward.
- CLI creation needs dedicated flags because existing `--bound` is integer-only. Add:
  - `--probabilities`
  - `--num-sectors`
  - `--latency-bound`
- `problem_name.rs` should not need changes because alias resolution already consults the catalog case-insensitively.

## Batch Structure

- **Batch 1:** add-model Steps 1-5 plus CLI/example-db wiring and verification
- **Batch 2:** add-model Step 6 (paper entry), then final verification

## Batch 1

### Task 1: Add the failing model tests first

**Files:**
- Create: `src/unit_tests/models/misc/expected_retrieval_cost.rs`
- Reference: `src/unit_tests/models/misc/partition.rs`
- Reference: `src/unit_tests/models/graph/multiple_copy_file_allocation.rs`

**Steps:**
1. Write tests that fail because `ExpectedRetrievalCost` does not exist yet:
   - constructor/accessor test
   - `dims()` / `num_variables()` test
   - `expected_cost()` for the issue's YES example (`1.0025`)
   - `evaluate()` / `is_valid_solution()` for YES and NO configs
   - wrong-length / out-of-range config tests returning `false` / `None`
   - brute-force solver test for the YES instance
   - serde round-trip test
   - paper-example test asserting the canonical config is satisfying and that brute-force finds 54 satisfying assignments
2. Run only the new test target and confirm it fails for the expected missing-type reasons.

### Task 2: Implement the model in `src/models/misc/expected_retrieval_cost.rs`

**Files:**
- Create: `src/models/misc/expected_retrieval_cost.rs`
- Reference: `src/models/misc/partition.rs`
- Reference: `src/models/graph/multiple_copy_file_allocation.rs`

**Steps:**
1. Add `ProblemSchemaEntry` with display name, description, and constructor-facing fields.
2. Add `ProblemSizeFieldEntry` with `num_records` and `num_sectors`.
3. Define the struct and constructor validation.
4. Implement helper methods listed in the design notes.
5. Implement `Problem` and `SatisfactionProblem`.
6. Add `declare_variants! { default sat ExpectedRetrievalCost => "num_sectors ^ num_records" }`.
7. Link the new test file with `#[cfg(test)]`.
8. Run the focused model test file again and make it pass before moving on.

### Task 3: Register the model in the library and canonical example-db

**Files:**
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/example_db/model_builders.rs` (only if needed by the existing chain)

**Steps:**
1. Export the module/type from `misc`, `models`, and `prelude`.
2. Add `canonical_model_example_specs()` in the model file using the issue's YES instance and 0-based satisfying config.
3. Register that example in the `misc` example-spec chain.
4. Run the focused tests again to confirm module wiring works.

### Task 4: Add CLI create support for manual instance construction

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`

**Steps:**
1. Add `CreateArgs` fields for `probabilities`, `num_sectors`, and `latency_bound`.
2. Include them in `all_data_flags_empty()`.
3. Add help-table and example text for `ExpectedRetrievalCost`.
4. Import the new model in `create.rs`.
5. Add an `example_for()` entry such as `--probabilities 0.2,0.15,0.15,0.2,0.1,0.2 --num-sectors 3 --latency-bound 1.01`.
6. Add a create match arm that parses the probabilities as `Vec<f64>`, validates `num_sectors`, parses `latency_bound` as `f64`, and serializes the constructed model.
7. Add/adjust create-command tests if existing coverage patterns touch the new flags.
8. Run the relevant CLI tests or focused `cargo test` targets and make them pass.

### Task 5: Run focused verification for Batch 1

**Steps:**
1. Run focused unit tests for the new model and any touched CLI tests.
2. Run `cargo fmt`.
3. Commit the implementation batch once the focused checks are green.

## Batch 2

### Task 6: Add the paper entry and keep the example aligned

**Files:**
- Modify: `docs/paper/reductions.typ`

**Steps:**
1. Add `"ExpectedRetrievalCost": [Expected Retrieval Cost]` to the display-name dictionary.
2. Add a `#problem-def("ExpectedRetrievalCost")[...]` entry in the same style as other `misc` models.
3. Cover:
   - formal definition from the issue
   - historical context and the Cody-Coffman / Garey-Johnson citations
   - note that the implementation uses floating-point probabilities/bounds for practicality
   - the canonical YES example with sector masses `(0.3, 0.35, 0.35)` and computed cost `1.0025`
   - `pred-commands()` based on `pred create --example ExpectedRetrievalCost`
4. Confirm the paper example matches the canonical example-db instance and the unit test.

### Task 7: Final verification

**Steps:**
1. Run `make test`.
2. Run `make clippy`.
3. Run `make paper`.
4. If any command regenerates ignored exports, verify only intended tracked files remain staged.
5. Commit the paper/docs batch.

## Execution Notes

- Follow TDD strictly: no production code before the new test exists and is observed failing.
- Do not add reduction code in this PR; this issue is model-only.
- Use the issue body plus fix-issue comments as the authoritative spec. If implementation reveals a contradiction, stop and record it in the PR summary rather than silently changing the model.
