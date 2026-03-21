# StackerCrane Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `[Model] StackerCrane` decision problem as a new `misc` model with canonical example-db support, `pred create` support for mixed-graph inputs, and a paper `problem-def` entry based on issue #245's corrected hourglass example.

**Architecture:** Implement `StackerCrane` as a no-variant satisfaction problem whose configuration is a permutation of the required directed arcs. `evaluate()` should reject non-permutations, then compute the closed-walk length by traversing the required arcs in the chosen order and inserting shortest-path connectors through a mixed graph formed from directed arcs plus bidirectional undirected edges. Keep the model self-contained in `src/models/misc/stacker_crane.rs`; do not introduce a new topology type for this issue.

**Tech Stack:** Rust workspace (`problemreductions`, `problemreductions-cli`), serde/inventory registry metadata, existing brute-force solver, Typst paper in `docs/paper/reductions.typ`.

---

## Constraints And Notes

- Treat the issue comments as the approved design basis. The latest `fix-issue` changelog supersedes the original vague encoding.
- This issue currently has no open companion rule issue mentioning `StackerCrane`. The implementation should proceed, but the PR body must carry an orphan-model warning.
- Keep `StackerCrane` as a **decision** problem with `Metric = bool`. Do not convert it to an optimization model in this PR.
- Use the issue's hourglass instance and satisfying permutation `[0, 2, 1, 4, 3]` as the canonical example and paper example.
- Do not add a manual alias in `problemreductions-cli/src/problem_name.rs`; alias resolution is registry-backed in this checkout. If an alias is added at all, it must come from `ProblemSchemaEntry.aliases`. Prefer no short alias here because `SCP` is ambiguous.
- Keep the paper batch separate from implementation so it runs after the example-db/model wiring is finished.

## Batch Layout

- **Batch 1:** Add-model Steps 1-5.5
  - Model implementation, registry/module wiring, canonical example, CLI creation, tests, verification.
- **Batch 2:** Add-model Step 6
  - Paper citations and `problem-def`, then `make paper`.

### Task 1: Write The Failing Model Tests First

**Files:**
- Create: `src/unit_tests/models/misc/stacker_crane.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Test: `src/unit_tests/models/misc/stacker_crane.rs`

**Step 1: Write failing tests for the intended public API**

Add a new test file with these initial tests:

- `test_stacker_crane_creation_and_metadata`
  - Construct the issue's hourglass instance.
  - Assert `num_vertices() == 6`, `num_arcs() == 5`, `num_edges() == 7`, `bound() == 20`.
  - Assert `dims() == vec![5; 5]`.
  - Assert `Problem::NAME == "StackerCrane"` and `Problem::variant().is_empty()`.
- `test_stacker_crane_rejects_non_permutations_and_wrong_lengths`
  - Reject duplicate arc indices, out-of-range indices, and wrong config length.
- `test_stacker_crane_issue_witness_and_tighter_bound`
  - Assert `[0, 2, 1, 4, 3]` evaluates to `true` for `B = 20`.
  - Assert the same instance with `B = 19` evaluates to `false`.
- `test_stacker_crane_small_solver_instance`
  - Use a tiny 2-arc instance with search space small enough for brute force and assert `BruteForce::find_satisfying()` returns a valid permutation.
- `test_stacker_crane_serialization_round_trip`
  - Round-trip serde JSON and re-check the witness.
- `test_stacker_crane_is_available_in_prelude`
  - Use `crate::prelude::StackerCrane` to ensure the export is actually wired.

In the same task, add the module/re-export placeholders in:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`

The test should compile far enough to fail because `StackerCrane` does not exist yet.

**Step 2: Run the targeted test and verify RED**

Run:

```bash
cargo test stacker_crane --lib
```

Expected:
- FAIL at compile time with unresolved `StackerCrane` symbols or missing module errors.

**Step 3: Commit the failing-test scaffold**

```bash
git add src/unit_tests/models/misc/stacker_crane.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs
git commit -m "test: add failing StackerCrane model tests"
```

### Task 2: Implement The Core `StackerCrane` Model

**Files:**
- Create: `src/models/misc/stacker_crane.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Test: `src/unit_tests/models/misc/stacker_crane.rs`

**Step 1: Implement the minimal model to satisfy Task 1**

Create `src/models/misc/stacker_crane.rs` with:

- `inventory::submit!` `ProblemSchemaEntry`:
  - `name = "StackerCrane"`
  - `display_name = "Stacker Crane"`
  - `aliases = &[]`
  - `dimensions = &[]`
  - constructor-facing fields:
    - `num_vertices: usize`
    - `arcs: Vec<(usize, usize)>`
    - `edges: Vec<(usize, usize)>`
    - `arc_lengths: Vec<i32>`
    - `edge_lengths: Vec<i32>`
    - `bound: i32`
- `#[derive(Debug, Clone, Serialize, Deserialize)] pub struct StackerCrane`
- `new(...)` plus `try_new(...) -> Result<Self, String>`:
  - lengths must match arc/edge counts
  - all endpoints must be `< num_vertices`
  - all lengths must be non-negative
  - `bound` must be non-negative
- accessors:
  - `num_vertices()`, `num_arcs()`, `num_edges()`
  - `arcs()`, `edges()`, `arc_lengths()`, `edge_lengths()`, `bound()`
- helper(s):
  - permutation validation for configs
  - shortest-path routine on the mixed graph with non-negative integer weights
  - optional `closed_walk_length(config) -> Option<i32>` helper for test readability
- `Problem` impl:
  - `type Metric = bool`
  - `variant() -> crate::variant_params![]`
  - `dims() -> vec![num_arcs; num_arcs]`
  - `evaluate()` returns `false` on invalid config or unreachable connector, otherwise checks total length `<= bound`
- `SatisfactionProblem` impl
- `declare_variants! { default sat StackerCrane => "num_vertices * 2^num_arcs", }`
- `canonical_model_example_specs()` behind `#[cfg(feature = "example-db")]` using the issue's hourglass instance and optimal config `[0, 2, 1, 4, 3]`
- test link:

```rust
#[cfg(test)]
#[path = "../../unit_tests/models/misc/stacker_crane.rs"]
mod tests;
```

Wire the new module through:
- `src/models/misc/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`

**Step 2: Run the targeted model tests and verify GREEN**

Run:

```bash
cargo test stacker_crane --lib
```

Expected:
- PASS for the new `StackerCrane` tests.

**Step 3: Refactor only if needed**

Allowed cleanups:
- Extract adjacency construction or shortest-path helpers inside `stacker_crane.rs`
- Normalize constructor validation messages

Do not add new behavior beyond the tests.

**Step 4: Commit the model implementation**

```bash
git add src/models/misc/stacker_crane.rs src/models/misc/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/misc/stacker_crane.rs
git commit -m "feat: add the StackerCrane model"
```

### Task 3: Verify Example-DB Integration For The Canonical Instance

**Files:**
- Modify: `src/models/misc/stacker_crane.rs`
- Modify: `src/models/misc/mod.rs`
- Test: `src/unit_tests/example_db.rs`

**Step 1: Add a failing example-db assertion if coverage is missing**

Add or extend tests so the new example is exercised via the shared example DB. Prefer one targeted assertion in `src/unit_tests/example_db.rs`:

- `test_find_model_example_stacker_crane`
  - Look up `ProblemRef { name: "StackerCrane", variant: BTreeMap::new() }`
  - Assert the example exists and stores the expected optimal config `[0, 2, 1, 4, 3]`

If the generic example-db self-consistency tests already cover everything after registration, keep the new test minimal and focused on discoverability.

**Step 2: Run the targeted example-db test and verify RED/GREEN**

Run first after adding the test:

```bash
cargo test test_find_model_example_stacker_crane --features example-db
```

Expected:
- FAIL until the example registration is correct.

After fixing any missing registration, rerun the same command and expect PASS.

Then run the generic example-db consistency checks:

```bash
cargo test model_specs_are_self_consistent --features example-db
```

Expected:
- PASS, including the new `StackerCrane` spec.

**Step 3: Commit the example-db wiring**

```bash
git add src/models/misc/stacker_crane.rs src/models/misc/mod.rs src/unit_tests/example_db.rs
git commit -m "test: register StackerCrane canonical example"
```

### Task 4: Add `pred create` Support For Mixed-Graph Input

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`
- Test: `problemreductions-cli/src/commands/create.rs`
- Test: `problemreductions-cli/src/cli.rs`

**Step 1: Write failing CLI tests**

Add CLI tests in `problemreductions-cli/src/commands/create.rs`:

- `test_create_stacker_crane_json`
  - Use:

```text
pred create StackerCrane --arcs "0>4,2>5,5>1,3>0,4>3" --graph "0-1,1-2,2-3,3-5,4-5,0-3,1-5" --arc-costs 3,4,2,5,3 --edge-lengths 2,1,3,2,1,4,3 --bound 20 --num-vertices 6
```

  - Assert the serialized problem type is `StackerCrane` and key fields match the issue instance.
- `test_create_stacker_crane_rejects_mismatched_arc_lengths`
  - Expect an error when `--arc-costs` length does not match the arc count.
- `test_create_stacker_crane_rejects_out_of_range_vertices`
  - Expect an error when `--num-vertices` is smaller than the largest referenced endpoint.

Add a small help test in `problemreductions-cli/src/cli.rs` that checks the create help mentions:

- `StackerCrane --arcs, --graph, --arc-costs, --edge-lengths, --bound [--num-vertices]`

**Step 2: Run the targeted CLI tests and verify RED**

Run:

```bash
cargo test -p problemreductions-cli stacker_crane
```

Expected:
- FAIL because the create arm/help text do not exist yet.

**Step 3: Implement the CLI support**

Update `problemreductions-cli/src/commands/create.rs`:

- add a `StackerCrane` match arm in `create()`
- reuse existing parsers where possible:
  - `parse_directed_graph(args.arcs, args.num_vertices)`
  - `parse_graph(args)` for undirected edges
  - `parse_arc_costs(args, num_arcs)`
  - `parse_i32_edge_values(args.edge_lengths.as_ref(), num_edges, "edge length")` or a small wrapper
- parse `--bound` as a non-negative integer and convert to `i32` with a range check
- construct `StackerCrane::try_new(...)` and bubble validation errors instead of panicking

Update `problemreductions-cli/src/cli.rs`:
- add the StackerCrane line to the "Flags by problem type" help block
- add one example command to the create examples list if the file already keeps that section current

**Step 4: Run the targeted CLI tests and verify GREEN**

Run:

```bash
cargo test -p problemreductions-cli stacker_crane
```

Expected:
- PASS for the new create/help tests.

**Step 5: Commit the CLI support**

```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "feat: add pred create support for StackerCrane"
```

### Task 5: Batch-1 Verification Before Paper Work

**Files:**
- Modify: none unless verification exposes gaps

**Step 1: Run the batch-1 verification suite**

Run:

```bash
cargo test stacker_crane --lib
cargo test test_find_model_example_stacker_crane --features example-db
cargo test model_specs_are_self_consistent --features example-db
cargo test -p problemreductions-cli stacker_crane
```

Expected:
- All commands PASS.

**Step 2: Fix anything that fails, then rerun the same commands**

Do not move to the paper batch until these commands are green.

### Task 6: Add Citations And The `problem-def` Entry (Separate Batch)

**Files:**
- Modify: `docs/paper/references.bib`
- Modify: `docs/paper/reductions.typ`
- Test: `src/unit_tests/models/misc/stacker_crane.rs`

**Step 1: Add a failing paper-example assertion if missing**

If Task 1's `test_stacker_crane_issue_witness_and_tighter_bound` is not already acting as the canonical paper example check, extend `src/unit_tests/models/misc/stacker_crane.rs` with:

- `test_stacker_crane_paper_example`
  - Build the exact hourglass instance shown in the paper.
  - Assert `[0, 2, 1, 4, 3]` is satisfying.
  - For this small instance, use `BruteForce::find_all_satisfying()` and assert the known witness is among the satisfying configs.

Run:

```bash
cargo test test_stacker_crane_paper_example --lib
```

Expected:
- PASS before touching the paper text, so the paper is anchored to a verified example.

**Step 2: Update citations**

Add BibTeX entries to `docs/paper/references.bib` for:
- Frederickson, Hecht, Kim (1978), SIAM J. Comput. 7(2):178-193, DOI `10.1137/0207017`
- Frederickson and Guan (1993), *Nonpreemptive Ensemble Motion Planning on a Tree*, J. Algorithms 15(1):29-60

Keep the existing `frederickson1979` entry for the broader postman/routing context.

**Step 3: Add the display name and `problem-def` entry**

Update `docs/paper/reductions.typ`:

- add `"StackerCrane": [Stacker-Crane],` to the `display-name` dictionary
- add `#problem-def("StackerCrane")[ ... ][ ... ]` in the appropriate section near other routing/scheduling/misc problems

The body must include:
- background linking it to mixed-graph arc routing and the Hamiltonian Circuit reduction
- best-known exact algorithm prose consistent with the declared complexity (`O(|V| 2^{|A|})` style wording) and cited
- the hourglass example from the canonical example spec, not a separate invented instance
- a `pred-commands()` block using `pred create --example StackerCrane -o stacker-crane.json`
- a short verifier walkthrough explaining why `[0, 2, 1, 4, 3]` yields total length 20

If a full CeTZ mixed-graph figure is too time-consuming, prefer a compact text/table example over inventing a half-baked graphic, but keep the example concrete and reproducible.

**Step 4: Build the paper**

Run:

```bash
make paper
```

Expected:
- PASS. This auto-runs the graph/schema exports before Typst compilation.

**Step 5: Commit the paper batch**

```bash
git add docs/paper/references.bib docs/paper/reductions.typ src/unit_tests/models/misc/stacker_crane.rs
git commit -m "docs: add the StackerCrane paper entry"
```

### Task 7: Final Verification And PR-Ready Cleanup

**Files:**
- Modify: any file only if verification exposes a real defect

**Step 1: Run the full verification needed before completion claims**

Run:

```bash
make fmt
make check
cargo test --features example-db test_find_model_example_stacker_crane
cargo test --features example-db model_specs_are_self_consistent
cargo test -p problemreductions-cli stacker_crane
make paper
```

Expected:
- Every command exits 0.

**Step 2: Review against the issue and this plan**

Confirm all of the following before closing the task:
- `StackerCrane` exists in `src/models/misc/stacker_crane.rs`
- the hourglass example is canonical in example-db
- `pred create StackerCrane ...` works with mixed-graph input
- the paper has both display name and `problem-def`
- the implementation still uses the corrected decision encoding from the issue comments
- no new alias was invented

**Step 3: Final commit only if verification required follow-up fixes**

```bash
git add -A
git commit -m "fix: finish StackerCrane verification follow-ups"
```
