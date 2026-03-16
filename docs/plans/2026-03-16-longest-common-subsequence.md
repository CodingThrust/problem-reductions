# LongestCommonSubsequence Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Align `LongestCommonSubsequence` with GitHub issue `#414` by refactoring the existing optimization-oriented implementation into the Garey-Johnson decision model with `alphabet_size`, `strings`, and `bound`, then carry that contract through the ILP solver path, CLI, examples, tests, and paper.

**Architecture:** `origin/main` already contains a mismatched `LongestCommonSubsequence` stack: the model, CLI arm, ILP reduction, tests, and paper entry all treat LCS as an optimization problem over raw byte strings. Reuse `ShortestCommonSupersequence` as the structural reference, migrate the existing LCS surfaces to the issue’s decision semantics, and keep the canonical problem name and `LCS` alias stable. Keep paper/example export work in a separate batch after the Rust-side contract is green.

**Tech Stack:** Rust workspace, registry-driven problem catalog, `pred` CLI, Typst paper, example-db fixtures, `cargo test`, `make regenerate-fixtures`, `make paper`, `make clippy`

---

## Batch 1: add-model Steps 1-5.5 plus the required ILP solver migration

### Task 1: Lock the issue contract down with failing tests

**Files:**
- Modify: `src/unit_tests/models/misc/longest_common_subsequence.rs`
- Modify: `src/unit_tests/rules/longestcommonsubsequence_ilp.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Rewrite the model tests around the issue #414 decision semantics**

Add red tests that expect:
- `LongestCommonSubsequence::new(alphabet_size, strings, bound)`
- `type Metric = bool`
- `dims() == vec![alphabet_size; bound]`
- `evaluate()` accepts a candidate witness string of exact length `bound`
- the verified YES witness from the issue evaluates to `true`
- a wrong-length config or out-of-range symbol evaluates to `false`

Use the issue’s verified binary YES instance as the canonical contract:

```rust
fn issue_yes_instance() -> LongestCommonSubsequence {
    LongestCommonSubsequence::new(
        2,
        vec![
            vec![0, 1, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0, 1],
            vec![0, 0, 1, 0, 1, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0],
        ],
        3,
    )
}

#[test]
fn test_lcs_issue_yes_instance() {
    let problem = issue_yes_instance();
    assert_eq!(problem.dims(), vec![2; 3]);
    assert!(problem.evaluate(&[0, 1, 0]));
    assert!(!problem.evaluate(&[1, 1, 1]));
}
```

**Step 2: Add solver-facing tests for the satisfaction model**

Add red tests for:
- `BruteForce::find_satisfying`
- `BruteForce::find_all_satisfying`
- a simple NO instance from the issue
- serde round-trip with `alphabet_size`, `strings`, and `bound`
- `test_lcs_paper_example` that uses the same canonical instance/example you plan to show in the paper

**Step 3: Add red ILP reduction tests for the new semantics**

Convert the rule tests to expect a satisfaction-preserving reduction. Keep the rule example small and two-string if needed, but the source model must be the new decision version:

```rust
#[test]
fn test_lcs_to_ilp_preserves_yes_instance() {
    let problem = LongestCommonSubsequence::new(3, vec![vec![0, 1, 2], vec![1, 0, 2]], 2);
    let reduction = problem.reduce_to();
    let ilp = reduction.target_problem();
    let solver = BruteForce::new();
    let target_solution = solver.find_satisfying(ilp).expect("ILP should be feasible");
    let source_solution = reduction.extract_solution(&target_solution);
    assert!(problem.evaluate(&source_solution));
}
```

**Step 4: Add the trait-consistency red test**

Replace the old optimization-style LCS entry with the new constructor shape:

```rust
check_problem_trait(
    &LongestCommonSubsequence::new(2, vec![vec![0, 1, 0], vec![1, 0, 1]], 2),
    "LongestCommonSubsequence",
);
```

**Step 5: Run the red phase and confirm the failures are the expected API/semantics mismatches**

Run:

```bash
cargo test --features "ilp-highs example-db" longest_common_subsequence -- --include-ignored
cargo test --features "ilp-highs example-db" longestcommonsubsequence_ilp -- --include-ignored
```

Expected: compile failures and/or assertion failures because the current model is still optimization-oriented and the current ILP reduction still targets the old binary-selection encoding.

### Task 2: Refactor the core model to the issue #414 decision schema

**Files:**
- Modify: `src/models/misc/longest_common_subsequence.rs`
- Modify: `src/models/misc/mod.rs`

**Step 1: Replace the old raw-byte optimization struct with the issue schema**

Refactor the model to:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongestCommonSubsequence {
    alphabet_size: usize,
    strings: Vec<Vec<usize>>,
    bound: usize,
}
```

Constructor invariants:
- reject negative-style CLI casts before they reach the constructor
- require `alphabet_size > 0` when `bound > 0`
- require every symbol in every input string to be `< alphabet_size`
- allow `bound == 0` and/or empty strings

**Step 2: Implement the decision `Problem` contract**

Mirror `ShortestCommonSupersequence`, but dualized:
- `type Metric = bool`
- `impl SatisfactionProblem for LongestCommonSubsequence {}`
- `dims() -> vec![alphabet_size; bound]`
- `evaluate(config)` returns `true` exactly when `config` is a common subsequence of every input string

Use a helper like:

```rust
fn is_subsequence(candidate: &[usize], target: &[usize]) -> bool
```

**Step 3: Add the getters required by registry metadata and rule overhead**

Implement at least:
- `alphabet_size()`
- `strings()`
- `bound()`
- `num_strings()`
- `total_length()`
- `sum_squared_lengths()`

These getters are needed for:
- `ProblemSchemaEntry` documentation
- `declare_variants!`
- exact `#[reduction(overhead = ...)]` formulas in the ILP rule

**Step 4: Update schema metadata and variant complexity**

Set:

```rust
ProblemSchemaEntry {
    name: "LongestCommonSubsequence",
    display_name: "Longest Common Subsequence",
    aliases: &["LCS"],
    fields: &[
        FieldInfo { name: "alphabet_size", type_name: "usize", ... },
        FieldInfo { name: "strings", type_name: "Vec<Vec<usize>>", ... },
        FieldInfo { name: "bound", type_name: "usize", ... },
    ],
}

crate::declare_variants! {
    default sat LongestCommonSubsequence => "alphabet_size ^ bound",
}
```

**Step 5: Add the canonical model example hook**

Add `canonical_model_example_specs()` in the model file and register it from `src/models/misc/mod.rs`. Use the issue’s verified binary YES instance and witness `[0, 1, 0]`.

**Step 6: Run the targeted tests again**

Run:

```bash
cargo test --features "ilp-highs example-db" longest_common_subsequence -- --include-ignored
```

Expected: the model tests should turn green before any CLI or rule work begins.

### Task 3: Update the CLI and help surface to the new model contract

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Replace the `LongestCommonSubsequence` create arm**

Make `pred create LCS` require `--strings` and `--bound`, and accept optional `--alphabet-size` exactly like the SCS arm. Do not use `as usize` on the signed `bound`; reject negative values explicitly.

Recommended shape:
- if a segment contains commas, parse it as `Vec<usize>` like SCS
- otherwise, preserve current raw-string ergonomics by mapping each byte/character to an alphabet index and inferring `alphabet_size`
- serialize the model using the explicit numeric schema either way

**Step 2: Update problem-specific examples and type hints**

Update:
- `example_for("LongestCommonSubsequence", ...)`
- `type_format_hint()` if needed for `Vec<Vec<usize>>`

Use a concrete example like:

```text
--strings "0,1,0,1,1,0;1,0,0,1,0,1" --bound 3 --alphabet-size 2
```

**Step 3: Refresh the CLI help text**

Update the `CreateArgs` `after_help` table and the flag descriptions so LCS no longer claims it only needs `--strings`.

**Step 4: Run the CLI-focused tests/build checks**

Run:

```bash
cargo test --features "ilp-highs example-db" -p problemreductions-cli create -- --include-ignored
cargo test --features "ilp-highs example-db" longest_common_subsequence -- --include-ignored
```

Expected: the CLI crate still builds, and `pred create LCS ...` emits the new JSON shape.

### Task 4: Migrate the ILP reduction to the decision formulation

**Files:**
- Modify: `src/rules/longestcommonsubsequence_ilp.rs`
- Modify: `src/unit_tests/rules/longestcommonsubsequence_ilp.rs`

**Step 1: Replace the old optimization reduction with a feasibility reduction**

Use a bounded-witness ILP:
- `x_{p,a}` chooses symbol `a` for witness position `p`
- `y_{r,p,j}` says witness position `p` is matched to position `j` in input string `r`

Core constraints:
- exactly one symbol per witness position
- exactly one matched source position per `(r, p)`
- character consistency: `y_{r,p,j} <= x_{p, strings[r][j]}`
- strict left-to-right ordering across witness positions within each input string

The target should be an `ILP<bool>` with a zero objective; satisfiability, not optimization, is the contract.

**Step 2: Update solution extraction**

`extract_solution()` must reconstruct the witness string config from the selected `x_{p,a}` variables, returning a `Vec<usize>` of length `bound`.

**Step 3: Make the overhead formula match the new construction**

Add/adjust getters on the model so the rule can state exact overhead in terms of:
- `alphabet_size`
- `bound`
- `num_strings`
- `total_length`
- `sum_squared_lengths`

Do not leave stale names like `num_chars_first` / `num_chars_second`.

**Step 4: Re-run the red rule tests until they pass**

Run:

```bash
cargo test --features "ilp-highs example-db" longestcommonsubsequence_ilp -- --include-ignored
```

Expected: the source model, target ILP feasibility, and extracted witness all agree on the same YES/NO instances.

### Task 5: Finish registration and consistency work

**Files:**
- Modify: `src/unit_tests/trait_consistency.rs`
- Verify only if needed: `src/models/mod.rs`
- Verify only if needed: `src/lib.rs`

**Step 1: Keep the existing crate-level exports stable**

Because the canonical name is unchanged, only touch `src/models/mod.rs` or `src/lib.rs` if the refactor accidentally broke an import or prelude export. Do not make gratuitous churn here.

**Step 2: Run the targeted consistency checks**

Run:

```bash
cargo test --features "ilp-highs example-db" trait_consistency -- --include-ignored
cargo test --features "ilp-highs example-db" example_db -- --include-ignored
```

Expected: LCS participates in the standard trait checks and canonical example-db coverage.

## Batch 2: add-model Step 6 (paper and generated exports)

### Task 6: Rewrite the paper entry around the issue’s decision problem

**Files:**
- Modify: `docs/paper/reductions.typ`
- Regenerate: `src/example_db/fixtures/examples.json`
- Regenerate if changed: `docs/src/reductions/problem_schemas.json`
- Regenerate if changed: `docs/src/reductions/reduction_graph.json`

**Step 1: Replace the current optimization prose with the issue definition**

Model the new entry on `ShortestCommonSupersequence`:
- formal decision definition with `Sigma`, `Sigma^*`, `R`, and `K`
- explicit definition of subsequence
- complexity paragraph that distinguishes the `|R| = 2` polynomial case from the arbitrary-`|R|` NP-complete case
- brief note that this is the dual of SCS for two strings

**Step 2: Use the canonical issue example consistently**

Use the same example instance in:
- `canonical_model_example_specs()`
- `test_lcs_paper_example`
- the paper entry

For the paper body, reuse the verified binary YES instance from the issue and explain why witness `010` is a common subsequence.

**Step 3: Use a string-layout figure, not a graph diagram**

Follow the `ShortestCommonSupersequence` and `PaintShop` sequence-layout style:
- `stack` + `box`
- witness string on the top row
- one row per input string beneath it
- highlight matched positions consistently

**Step 4: Regenerate fixtures and rebuild the paper**

Run:

```bash
make regenerate-fixtures
make paper
```

Expected: the fixtures carry the new LCS schema/example, and the Typst paper compiles cleanly against the checked-in example database.

## Batch 3: add-model Step 7 plus issue-to-pr execution cleanup

### Task 7: Run full verification and prepare the branch for review

**Files:**
- Verify workspace-wide; no new source files expected here

**Step 1: Run the full verification set**

Run:

```bash
make test
make clippy
git status --short
```

Expected: tests pass with `--features "ilp-highs example-db"`, clippy is clean, and only the intentional source/generated files remain staged.

**Step 2: Run review-implementation and fix anything it flags**

Invoke the repo-local review workflow after the code is green. Pay special attention to:
- schema metadata completeness
- canonical example-db coverage
- trait consistency
- LCS paper/example/test consistency
- ILP overhead naming consistency

**Step 3: Capture deviations from the original plan before push**

If you intentionally preserve backward-compatible CLI parsing for raw character strings, or if the final ILP encoding differs from the initial variable naming here, record that in the PR implementation summary instead of hiding it.
