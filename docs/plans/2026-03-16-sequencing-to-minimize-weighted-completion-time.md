# SequencingToMinimizeWeightedCompletionTime Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `SequencingToMinimizeWeightedCompletionTime` model, an exact `ILP<i32>` reduction, and the required CLI/example-db/paper wiring for issue `#497`.

**Architecture:** Model the problem as a direct minimization problem over precedence-respecting schedules, not as the raw decision form with an explicit bound `K`. Reuse the Lehmer-code schedule encoding already used by `MinimumTardinessSequencing` and `FlowShopScheduling`, then add an `ILP<i32>` reduction that uses integer completion-time variables plus 0/1 pair-order variables (represented in the integer domain with `<= 1` constraints) so the repo gains an exact ILP path without introducing a separate mixed-domain solver.

**Tech Stack:** Rust workspace, inventory-based problem registry, `pred create` CLI, Typst paper, `BruteForce`, `ILPSolver`, feature flags `ilp-highs` and `example-db`.

---

## Design Decisions

- Implement the optimization form directly: the model should store `lengths`, `weights`, and `precedences`, with objective `sum_t w(t) * C(t)`. Do not keep `bound_k` on the struct.
- Use Lehmer-code schedule encoding (`dims() = [n, n-1, ..., 1]`) so brute force naturally enumerates permutations while precedence feasibility remains model-local.
- Add the ILP reduction in the same PR. The issue explicitly marks ILP as a supported solving strategy, and the `issue-to-pr` workflow requires model + ILP together when integer programming is part of the solver story.
- Start from the issue's 5-task example, but do not assume the listed schedule is optimal. Compute the true optimum with `BruteForce` before freezing the canonical example, paper example, and expected objective assertions.

## Batch Structure

- **Batch 1:** add-model Steps 1-5.5 plus the `SequencingToMinimizeWeightedCompletionTime -> ILP` rule
- **Batch 2:** paper/docs/example finalization after Batch 1 has stabilized exports and the verified optimal example

### Task 1: Write the Red Tests for the New Model

**Files:**
- Create: `src/unit_tests/models/misc/sequencing_to_minimize_weighted_completion_time.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing model tests**

Cover these behaviors before adding production code:

- constructor + getters: `num_tasks`, `lengths`, `weights`, `precedences`, `num_precedences`
- `dims()` should be `[n, n-1, ..., 1]`
- `direction()` should be `Direction::Minimize`
- `evaluate()` on a valid Lehmer-coded schedule should return the expected weighted completion time
- `evaluate()` should reject wrong-length configs, out-of-range Lehmer digits, and precedence violations
- brute force should recover the best objective on a tiny acyclic instance
- cyclic precedences should yield no feasible brute-force solution
- serde round-trip

Use `MinimumTardinessSequencing` as the primary reference and keep at least one test around the exact 5-task issue instance.

**Step 2: Run the new tests and verify they fail for the right reason**

Run:

```bash
cargo test sequencing_to_minimize_weighted_completion_time --lib --features "ilp-highs example-db"
```

Expected: compile/test failure because the new model type does not exist yet.

### Task 2: Implement and Register the Model

**Files:**
- Create: `src/models/misc/sequencing_to_minimize_weighted_completion_time.rs`
- Modify: `src/models/misc/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Implement the model**

Follow the `add-model` pattern from `MinimumTardinessSequencing`:

- add `ProblemSchemaEntry`
- add the struct with fields:
  - `lengths: Vec<u64>`
  - `weights: Vec<u64>`
  - `precedences: Vec<(usize, usize)>`
- validate equal lengths/weights length and in-range precedence endpoints in `new()`
- add inherent getters:
  - `num_tasks()`
  - `lengths()`
  - `weights()`
  - `precedences()`
  - `num_precedences()`
  - `total_processing_time()` (needed for the ILP formulation)
- decode Lehmer code into a schedule inside `evaluate()`
- compute completion times cumulatively along the schedule
- reject precedence violations
- return `SolutionSize::Valid(total_weighted_completion_time)` with `OptimizationProblem::direction() == Minimize`

Use `u64` for lengths, weights, and the optimization value unless a concrete compiler/runtime constraint forces a different integer type.

**Step 2: Register the problem**

- export the module from `src/models/misc/mod.rs`
- re-export it from `src/models/mod.rs`
- add it to `src/lib.rs` / prelude if needed for the public API
- add `declare_variants!` with a factorial-sized brute-force complexity, most likely:

```rust
crate::declare_variants! {
    default opt SequencingToMinimizeWeightedCompletionTime => "factorial(num_tasks)",
}
```

**Step 3: Re-run the model tests**

Run:

```bash
cargo test sequencing_to_minimize_weighted_completion_time --lib --features "ilp-highs example-db"
```

Expected: the new model tests compile and pass.

### Task 3: Add CLI Creation Support and the Canonical Model Example

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `src/models/misc/sequencing_to_minimize_weighted_completion_time.rs`
- Modify: `src/models/misc/mod.rs`

**Step 1: Add the CLI flags and create-arm**

Add creation support for:

```bash
pred create SequencingToMinimizeWeightedCompletionTime \
  --lengths 2,1,3,1,2 \
  --weights 3,5,1,4,2 \
  --precedence-pairs "0>2,1>4"
```

Implementation notes:

- add a new `--lengths` flag in `problemreductions-cli/src/cli.rs`
- reuse `--weights` for task weights
- reuse `--precedence-pairs`
- document the new problem in the CLI help table
- add the create-arm in `problemreductions-cli/src/commands/create.rs`

**Step 2: Add the canonical model example**

Inside the new model file:

- add `canonical_model_example_specs()`
- keep the example small enough for `BruteForce`
- make sure the example instance matches the later paper example

Wire the example into `src/models/misc/mod.rs` so `pred create --example SequencingToMinimizeWeightedCompletionTime` works.

**Step 3: Verify the CLI path**

Run at least one focused command:

```bash
cargo run -p problemreductions-cli -- \
  create SequencingToMinimizeWeightedCompletionTime \
  --lengths 2,1,3 \
  --weights 3,5,1 \
  --precedence-pairs "0>2"
```

Expected: JSON for the new model is emitted successfully.

### Task 4: Write the Red Tests for the ILP Reduction

**Files:**
- Create: `src/unit_tests/rules/sequencingtominimizeweightedcompletiontime_ilp.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write failing rule tests**

Cover these behaviors before implementing the rule:

- ILP variable layout on a tiny instance
- objective sense should be `ObjectiveSense::Minimize`
- extraction should return a valid source configuration (Lehmer code) whose decoded schedule matches the ILP ordering
- closed-loop agreement between `BruteForce` on the source model and `ILPSolver` on the reduced ILP
- cyclic precedences should make the reduced ILP infeasible
- `solve_reduced` should recover a valid source solution

Use `LongestCommonSubsequence -> ILP`, `TravelingSalesman -> ILP`, and `Factoring -> ILP` as the main rule/test references.

**Step 2: Run the rule tests and verify they fail**

Run:

```bash
cargo test sequencingtominimizeweightedcompletiontime_ilp --features "ilp-highs example-db"
```

Expected: compile/test failure because the rule does not exist yet.

### Task 5: Implement the ILP Reduction and Register It

**Files:**
- Create: `src/rules/sequencingtominimizeweightedcompletiontime_ilp.rs`
- Modify: `src/rules/mod.rs`
- Modify: `src/models/misc/sequencing_to_minimize_weighted_completion_time.rs` (only if a small shared helper is needed for schedule encoding/decoding)

**Step 1: Implement the reduction**

Use `ILP<i32>` with this structure:

- completion-time variables `C_j` for each task
- pair-order variables `y_{ij}` for each unordered task pair `i < j`, constrained to `{0,1}` via `0 <= y_{ij} <= 1`
- lower bounds `C_j >= l_j`
- upper bounds `C_j <= sum_t l_t`
- precedence constraints `C_j - C_i >= l_j` for each required edge `i -> j`
- disjunctive non-overlap constraints for each unordered pair using a big-M value `M = total_processing_time`:
  - one constraint for `i` before `j`
  - one constraint for `j` before `i`
- objective `sum_j w_j * C_j`

**Step 2: Implement extraction**

- read the completion-time variables from the ILP solution
- sort tasks by completion time (stable tie-breaker by task index)
- convert the resulting schedule order back into Lehmer code
- return that source config from `extract_solution()`

**Step 3: Register the rule**

- add the feature-gated rule module to `src/rules/mod.rs`
- add `canonical_rule_example_specs()` using `direct_ilp_example::<_, i32, _>(...)`
- include the rule example in the `example-db` aggregation block in `src/rules/mod.rs`

**Step 4: Re-run the rule tests**

Run:

```bash
cargo test sequencingtominimizeweightedcompletiontime_ilp --features "ilp-highs example-db"
```

Expected: the new rule tests pass and the reduced ILP is solvable on the small acyclic examples.

### Task 6: Finalize the Canonical Example and Add the Paper-Example Tests

**Files:**
- Modify: `src/unit_tests/models/misc/sequencing_to_minimize_weighted_completion_time.rs`
- Modify: `src/unit_tests/rules/sequencingtominimizeweightedcompletiontime_ilp.rs`

**Step 1: Freeze the example after verifying the optimum**

- run `BruteForce::find_all_best()` on the candidate paper instance
- record the actual optimal weighted completion time and the number of optimal schedules
- update the model tests so `test_sequencing_to_minimize_weighted_completion_time_paper_example` checks the final paper/example-db instance and the exact optimum claim

**Step 2: Add a focused rule example assertion**

- use the same canonical instance for the rule-side closed-loop test when possible
- ensure `extract_solution()` yields a source metric equal to the brute-force optimum

### Task 7: Batch 2 Docs and Paper Work

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/src/reductions/problem_schemas.json`
- Modify: `docs/src/reductions/reduction_graph.json`

**Step 1: Add the model paper entry**

- register `"SequencingToMinimizeWeightedCompletionTime"` in the `display-name` dictionary
- add `problem-def("SequencingToMinimizeWeightedCompletionTime")`
- explain the problem as single-machine precedence-constrained scheduling minimizing `sum w_j C_j`
- cite the classical context from Lawler/Smith as appropriate
- use the verified canonical example and show the actual optimal schedule and objective

**Step 2: Add the ILP rule paper entry**

Because the ILP reduction is being implemented in the same PR, also add:

- `reduction-rule("SequencingToMinimizeWeightedCompletionTime", "ILP")`
- a short correctness sketch for the completion-time / pair-order formulation
- an example caption consistent with the exported rule example

**Step 3: Rebuild the paper**

Run:

```bash
make paper
```

Expected: paper compiles cleanly and any generated reduction JSON updates are limited to the expected schema/graph exports.

### Task 8: Verification, Review, and PR Cleanup

**Files:**
- Whole tree

**Step 1: Run final verification**

Run:

```bash
cargo test --features "ilp-highs example-db"
cargo clippy --all-targets --all-features -- -D warnings
```

If repo conventions require it and runtime permits, prefer the stronger wrapper:

```bash
make test clippy
```

**Step 2: Inspect the resulting tree**

- check `git status --short`
- stage only the expected source, test, paper, and generated export changes
- make sure `docs/plans/2026-03-16-sequencing-to-minimize-weighted-completion-time.md` is deleted before the final push

**Step 3: Run the implementation review**

- invoke the repo-local `review-implementation` skill
- fix any structural or quality findings before pushing

**Step 4: Commit sequence**

Use small commits while working, then follow the `issue-to-pr` flow:

```bash
git add -A
git commit -m "Implement #497: add SequencingToMinimizeWeightedCompletionTime"
git rm docs/plans/2026-03-16-sequencing-to-minimize-weighted-completion-time.md
git commit -m "chore: remove plan file after implementation"
```
