# GroupingBySwapping Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `GroupingBySwapping` satisfaction model from issue #440, including registry/CLI/example-db/paper integration, with the issue's corrected `abcabc` example as the canonical fixture.

**Architecture:** Implement `GroupingBySwapping` as a `misc` satisfaction model with fields `alphabet_size`, `string`, and `budget`. A configuration is a length-`budget` swap program whose entries are adjacent swap positions `0..string_len-2` plus the no-op slot `string_len-1`; `evaluate()` applies the swaps and accepts exactly when every symbol appears in a single contiguous block. Batch 1 covers model code, registration, CLI, tests, and canonical example wiring. Batch 2 covers the paper entry after the implementation is complete and exports are available.

**Tech Stack:** Rust library crate, `inventory` schema registration, registry-backed CLI creation, serde, Typst paper, `cargo test`, `make paper`, `make test`, `make clippy`

---

## Batch 1: Model, Registry, CLI, Tests, Example DB

### Task 1: Write the failing model tests

**Files:**
- Create: `src/unit_tests/models/misc/grouping_by_swapping.rs`
- Modify: `src/models/misc/grouping_by_swapping.rs`

**Step 1: Write the failing test**

Add tests that describe the issue-backed behavior before implementing the model:
- `test_grouping_by_swapping_basic`
- `test_grouping_by_swapping_evaluate_issue_yes`
- `test_grouping_by_swapping_rejects_wrong_length_and_out_of_range_swaps`
- `test_grouping_by_swapping_bruteforce_yes_and_no`
- `test_grouping_by_swapping_paper_example`
- `test_grouping_by_swapping_serialization`

Use these concrete instances:
- YES instance: `alphabet_size = 3`, `string = [0,1,2,0,1,2]`, `budget = 5`, satisfying config `[2,1,3,5,5]`
- NO-short-budget instance: same string with `budget = 2`
- Minimum-3-swaps witness: same string with `budget = 3`, config `[2,1,3]`

**Step 2: Run test to verify it fails**

Run: `cargo test grouping_by_swapping --lib`
Expected: compile/test failure because `GroupingBySwapping` is not implemented or not registered yet.

**Step 3: Write minimal implementation**

Create `src/models/misc/grouping_by_swapping.rs` with:
- `ProblemSchemaEntry`
- `GroupingBySwapping` struct
- constructor + getters
- helper(s) to apply a swap program and test groupedness
- `Problem` + `SatisfactionProblem` impls
- `declare_variants! { default sat GroupingBySwapping => "string_len ^ budget", }`
- `#[cfg(test)]` link to the test file

Required issue-specific semantics:
- symbols are encoded as `0..alphabet_size-1`
- each config entry is either an adjacent swap position or the no-op slot `string_len-1`
- `evaluate()` returns `false` on wrong-length configs, out-of-range values, or invalid input symbols
- groupedness means no symbol reappears after its contiguous block ends

**Step 4: Run test to verify it passes**

Run: `cargo test grouping_by_swapping --lib`
Expected: the new model tests pass.

**Step 5: Commit**

Run:
```bash
git add src/models/misc/grouping_by_swapping.rs src/unit_tests/models/misc/grouping_by_swapping.rs
git commit -m "Add GroupingBySwapping model"
```

### Task 2: Register the model in the crate surface

**Files:**
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the failing test**

Extend the new unit test file with assertions that:
- `Problem::NAME == "GroupingBySwapping"`
- `Problem::variant() == vec![]`
- the type is reachable through `crate::models::misc`, `crate::models`, and `crate::prelude`

**Step 2: Run test to verify it fails**

Run: `cargo test grouping_by_swapping --lib`
Expected: import/re-export assertions fail or do not compile because the type is not fully re-exported yet.

**Step 3: Write minimal implementation**

Register the model in:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude exports

Also add the example-db spec hook in `src/models/misc/mod.rs` once Task 4 creates it.

**Step 4: Run test to verify it passes**

Run: `cargo test grouping_by_swapping --lib`
Expected: re-export/import assertions pass.

**Step 5: Commit**

Run:
```bash
git add src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/grouping_by_swapping.rs
git commit -m "Register GroupingBySwapping exports"
```

### Task 3: Add CLI discovery and creation support

**Files:**
- Modify: `problemreductions-cli/src/problem_name.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write the failing test**

Add CLI integration coverage for:
- `pred create GroupingBySwapping --string "0,1,2,0,1,2" --bound 5`
- optional `--alphabet-size 3`
- `pred create --example GroupingBySwapping`
- helpful usage text when `--string` or `--bound` is missing

Prefer the same assertions used by the `LongestCommonSubsequence` and `StringToStringCorrection` tests:
- JSON `type == "GroupingBySwapping"`
- `data.alphabet_size == 3`
- `data.string == [0,1,2,0,1,2]`
- `data.budget == 5`

**Step 2: Run test to verify it fails**

Run: `cargo test -p problemreductions-cli grouping_by_swapping`
Expected: CLI tests fail because the alias/help/create path does not exist yet.

**Step 3: Write minimal implementation**

Add the canonical name wiring:
- `resolve_alias()` support for `"groupingbyswapping"`

Add a dedicated CLI flag:
- `--string` for a comma-separated symbol list

Update:
- `all_data_flags_empty()`
- `CreateArgs` docs / "Flags by problem type" help
- problem-specific help examples in `create.rs`
- `create()` match arm that parses `--string`, infers `alphabet_size` when omitted, validates `--alphabet-size >= max(symbol)+1`, and constructs `GroupingBySwapping`

**Step 4: Run test to verify it passes**

Run: `cargo test -p problemreductions-cli grouping_by_swapping`
Expected: the new CLI tests pass.

**Step 5: Commit**

Run:
```bash
git add problemreductions-cli/src/problem_name.rs problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/tests/cli_tests.rs
git commit -m "Add GroupingBySwapping CLI support"
```

### Task 4: Add the canonical example-db fixture and paper-backed test data

**Files:**
- Modify: `src/models/misc/grouping_by_swapping.rs`
- Modify: `src/models/misc/mod.rs`

**Step 1: Write the failing test**

Extend `src/unit_tests/models/misc/grouping_by_swapping.rs` to require:
- `canonical_model_example_specs()` exports the issue's `abcabc`, `K=5`, config `[2,1,3,5,5]`
- the paper/example test verifies the exact issue witness is satisfying
- `BruteForce` confirms the same string is unsatisfiable for `budget = 2`

**Step 2: Run test to verify it fails**

Run: `cargo test grouping_by_swapping --features example-db --lib`
Expected: failure because the canonical example spec is missing or incomplete.

**Step 3: Write minimal implementation**

Add `canonical_model_example_specs()` to the model file and register it from `src/models/misc/mod.rs`.

Use the issue-corrected example exactly:
- instance: `GroupingBySwapping::new(3, vec![0,1,2,0,1,2], 5)`
- optimal/satisfying config: `vec![2,1,3,5,5]`
- optimal value: `true`

**Step 4: Run test to verify it passes**

Run: `cargo test grouping_by_swapping --features example-db --lib`
Expected: example-db-aware tests pass.

**Step 5: Commit**

Run:
```bash
git add src/models/misc/grouping_by_swapping.rs src/models/misc/mod.rs src/unit_tests/models/misc/grouping_by_swapping.rs
git commit -m "Add GroupingBySwapping canonical example"
```

### Task 5: Batch 1 verification

**Files:**
- Modify: none

**Step 1: Run focused verification**

Run:
```bash
cargo test grouping_by_swapping --workspace
cargo test -p problemreductions-cli grouping_by_swapping
```

Expected: all focused library and CLI tests for `GroupingBySwapping` pass.

**Step 2: Run broader verification**

Run:
```bash
make test
make clippy
```

Expected: repository tests and clippy pass without introducing regressions.

**Step 3: Commit**

If verification required follow-up fixes, commit them coherently before moving to Batch 2.

## Batch 2: Paper Entry

### Task 6: Add the Typst paper entry after exports are available

**Files:**
- Modify: `docs/paper/reductions.typ`
- Test: `src/unit_tests/models/misc/grouping_by_swapping.rs`

**Step 1: Write the failing test**

Finalize `test_grouping_by_swapping_paper_example` so it matches the paper/example-db instance exactly:
- construct the canonical `abcabc`, `K=5` instance
- assert `[2,1,3,5,5]` is satisfying
- assert `budget = 2` is unsatisfiable via `BruteForce`

**Step 2: Run test to verify it fails if needed**

Run: `cargo test grouping_by_swapping_paper_example --lib`
Expected: if the example/test drifted during Batch 1, fix the test before touching the paper.

**Step 3: Write minimal implementation**

In `docs/paper/reductions.typ`:
- add `"GroupingBySwapping": [Grouping by Swapping]` to `display-name`
- add `#problem-def("GroupingBySwapping")[...]`
- use the canonical example data from the checked-in fixture flow
- describe the `abcabc -> aabbcc` witness with the issue's 3 effective swaps and 2 trailing no-ops
- include `pred-commands()` using the canonical example spec
- cite Garey & Johnson SR21 and note the brute-force bound used by the model metadata

**Step 4: Run paper verification**

Run: `make paper`
Expected: the Typst paper builds cleanly and the new entry renders without missing references.

**Step 5: Commit**

Run:
```bash
git add docs/paper/reductions.typ src/unit_tests/models/misc/grouping_by_swapping.rs
git commit -m "Document GroupingBySwapping in paper"
```

## Final Verification

### Task 7: Full verification before handoff

**Files:**
- Modify: none

**Step 1: Run the final required checks**

Run:
```bash
make test
make clippy
make paper
git status --short
```

Expected:
- tests pass
- clippy passes
- paper builds
- the tree is clean except for intentionally ignored/generated files

**Step 2: Prepare the implementation summary**

Summarize:
- model file and helper logic
- registration/export/CLI/example-db integration
- paper entry
- any deviations from this plan

