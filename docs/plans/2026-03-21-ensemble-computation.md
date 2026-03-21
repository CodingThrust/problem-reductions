# EnsembleComputation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `EnsembleComputation` satisfaction model from issue #215, including registry/CLI/example-db integration, tests, and a paper entry.

**Architecture:** Implement `EnsembleComputation` as a new `misc` satisfaction problem with fields `universe_size`, `subsets`, and `budget`. Encode each union step by two operand indices over the fixed domain `0..(universe_size + budget)`, validate operand references against the prefix of previously computed sets, and treat the issue's `j <= J` semantics as "a satisfying prefix of at most `budget` operations exists" while still using a fixed-length config for brute force.

**Tech Stack:** Rust workspace (`problemreductions` + `problemreductions-cli`), serde, inventory registry, clap CLI, Typst paper, existing brute-force solver.

---

## Batch 1: Model, Tests, Registry, CLI, Example DB

### Task 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/misc/ensemble_computation.rs`
- Reference: `src/unit_tests/models/misc/multiprocessor_scheduling.rs`
- Reference: `src/unit_tests/models/misc/sequencing_within_intervals.rs`

**Step 1: Write the failing tests**

Add tests for:
- construction/getters/dims for `EnsembleComputation::new(4, vec![vec![0, 1, 2], vec![0, 1, 3]], 4)`
- satisfiable witness evaluation using a concrete full-length config such as `[0, 1, 4, 2, 4, 3, 0, 1]`
- invalid configs for future references, overlapping operands, out-of-range config length, and missing required subsets
- a small brute-force-solvable instance (for example `universe_size = 3`, `subsets = [[0, 1]]`, `budget = 1`)
- serde round-trip
- paper/canonical example validity without brute-force exhaustiveness if the search space is too large

**Step 2: Run the targeted test to verify it fails**

Run: `cargo test ensemble_computation --lib`

Expected: FAIL because `EnsembleComputation` does not exist yet.

**Step 3: Do not write production code until the failure is confirmed**

Use this failing run as the RED checkpoint for the model implementation.

### Task 2: Implement the model and register it in the library

**Files:**
- Create: `src/models/misc/ensemble_computation.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the minimal model implementation**

Implement:
- `ProblemSchemaEntry` with display name `Ensemble Computation`
- struct fields `universe_size: usize`, `subsets: Vec<Vec<usize>>`, `budget: usize`
- getters `universe_size()`, `num_subsets()`, `budget()`
- `Problem` with `Metric = bool`, `variant_params![]`, `dims() = vec![universe_size + budget; 2 * budget]`
- `evaluate()` that:
  - rejects wrong config length
  - decodes operand references as either singletons or previously computed `z_k`
  - rejects non-disjoint unions
  - tracks computed sets in sequence order
  - returns `true` once every required subset has appeared as some computed `z_i`
- `SatisfactionProblem`
- `declare_variants! { default sat EnsembleComputation => "(universe_size + budget)^(2 * budget)" }`
- `canonical_model_example_specs()` using the issue's satisfiable example instance

**Step 2: Register exports**

Wire the new model through:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` / `prelude`
- misc example-spec aggregation in `src/models/misc/mod.rs`

**Step 3: Run the targeted tests**

Run: `cargo test ensemble_computation --lib`

Expected: PASS for the new model test file.

**Step 4: Refactor only if needed**

Keep helpers local to the model file unless another existing model clearly needs reuse.

### Task 3: Add CLI creation support and example-path integration

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/problem_name.rs` only if a lowercase alias mapping is needed

**Step 1: Write or extend a failing CLI-focused test if there is an existing pattern**

If there is a nearby unit/integration test pattern for `pred create`, add a focused failing test for `EnsembleComputation`.
If there is no practical pattern already in the workspace, skip adding a new CLI test file and rely on `cargo test` plus manual `pred create` verification later.

**Step 2: Implement the CLI arm**

In `create.rs`, add a new `EnsembleComputation` arm that parses:
- `--universe` as `universe_size`
- `--sets` as required subsets
- `--budget` as the union-operation bound

Also update:
- `example_for(...)`
- any field-to-flag mapping needed so schema-driven help shows `--universe`, `--sets`, and `--budget`
- `cli.rs` "Flags by problem type" help table

Do not invent a short literature alias unless one is clearly standard.

**Step 3: Verify the CLI path**

Run:
- `cargo test -p problemreductions-cli create`
- `cargo run -p problemreductions-cli -- create EnsembleComputation --universe 4 --sets "0,1,2;0,1,3" --budget 4 --json`

Expected:
- tests pass
- the CLI emits a valid serialized `EnsembleComputation` JSON object

### Task 4: Run batch-1 verification before moving to paper work

**Files:**
- No new files

**Step 1: Run focused workspace verification**

Run:
- `cargo test ensemble_computation`
- `cargo test -p problemreductions-cli create`
- `cargo clippy --all-targets --all-features -- -D warnings`

Expected: all pass.

**Step 2: Commit batch-1 work**

Run:
- `git add src/models/misc/ensemble_computation.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/ensemble_computation.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs problemreductions-cli/src/problem_name.rs`
- `git commit -m "Add EnsembleComputation model"`

Only include `problem_name.rs` in the commit if it was actually changed.

## Batch 2: Paper Entry and Documentation-Specific Verification

### Task 5: Add the paper entry and any missing bibliography entry

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib` if the Järvisalo et al. 2012 citation is not already present

**Step 1: Write the paper example after implementation is stable**

Add:
- `display-name` entry for `EnsembleComputation`
- `problem-def("EnsembleComputation")` with a self-contained definition
- short background tying Garey & Johnson PO9 to monotone/ensemble circuit computation
- algorithm note using the brute-force search-space expression and, if cited, the SAT 2012 practical approach
- a worked example based on the issue instance, explicitly explaining the union sequence

Keep the paper text consistent with the implemented encoding:
- the mathematical problem remains "at most `J` operations"
- the code-level config uses `2 * budget` operand slots
- the example should explain how the witness sequence maps onto that encoding

**Step 2: Run paper verification**

Run: `make paper`

Expected: PASS with regenerated untracked docs artifacts only.

**Step 3: Re-run the paper/example test if needed**

Run: `cargo test ensemble_computation_paper_example --lib`

Expected: PASS.

### Task 6: Final verification and pipeline handoff

**Files:**
- No new files

**Step 1: Run the full verification required before claiming completion**

Run:
- `make test`
- `make clippy`
- `git status --short`

Expected:
- test and clippy succeed
- only intended tracked changes remain
- ignored generated doc exports may appear but are not staged

**Step 2: Commit the documentation batch**

Run:
- `git add docs/paper/reductions.typ docs/paper/references.bib`
- `git commit -m "Document EnsembleComputation"`

Only include `references.bib` if it changed.

**Step 3: Prepare PR summary inputs**

Collect:
- files changed
- any deviation from the issue (especially the fixed-length encoding choice for `j <= J`)
- verification commands actually run

This summary will be posted to the PR before the final push in the pipeline skill.
