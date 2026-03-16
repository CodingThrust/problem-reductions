# MultipleChoiceBranching Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `MultipleChoiceBranching` model as a directed-graph satisfaction problem, wire it into the registry/example-db/CLI, and document it in the paper using the issue's YES instance.

**Architecture:** Implement `MultipleChoiceBranching<W>` as a `DirectedGraph`-backed satisfaction problem with one binary variable per arc. The constructor owns the static invariants (weight count, partition indices in range, partition groups pairwise disjoint, and full coverage of all arcs); `evaluate()` then checks dynamic feasibility on a configuration: binary domain, one selected arc per partition group, in-degree at most one at each vertex, acyclicity of the selected subgraph, and total selected weight at least the threshold. Use the issue's 8-arc YES instance as the canonical model example so tests, CLI examples, and the paper stay aligned.

**Tech Stack:** Rust, serde, inventory schema registry, `DirectedGraph`, `BruteForce`, `pred create`, Typst paper/example fixtures.

---

## Constraints And Decisions

- Follow repo-local [`.claude/skills/add-model/SKILL.md`](/Users/jinguomini/rcode/problem-reductions/.worktrees/issue-253-multiple-choice-branching/.claude/skills/add-model/SKILL.md) Steps 1-7.
- Keep the paper work in a separate batch from the implementation work. Do not start Batch 2 until Batch 1 is green.
- Model shape:
  - Type: `MultipleChoiceBranching<W>`
  - Category: `src/models/graph/`
  - Problem kind: satisfaction (`Metric = bool`)
  - Variant dimensions: weight only, default `i32`
  - Complexity string: `"2^num_arcs"`
- Canonical example:
  - Use issue #253 YES instance with arc order `[(0,1), (0,2), (1,3), (2,3), (1,4), (3,5), (4,5), (2,4)]`
  - Use satisfying configuration `[1,0,1,0,0,1,0,1]`
  - Keep the threshold at `10`
- There is currently no open rule issue referencing `MultipleChoiceBranching`. Treat that as an orphan-model warning to surface in the PR summary/body, but do not block implementation.

## Batch 1: Add-Model Steps 1-5.5

### Task 1: Write The Failing Model Tests First

**Files:**
- Create: `src/unit_tests/models/graph/multiple_choice_branching.rs`
- Reference: `src/unit_tests/models/graph/hamiltonian_path.rs`
- Reference: `src/unit_tests/models/graph/minimum_feedback_arc_set.rs`
- Reference: `src/models/graph/rural_postman.rs`

**Step 1: Write failing tests for the model contract**

Cover these behaviors explicitly:
- `test_multiple_choice_branching_creation_and_accessors`
  - Construct the YES instance from issue #253.
  - Assert `num_vertices() == 6`, `num_arcs() == 8`, `num_partition_groups() == 4`.
  - Assert `dims() == vec![2; 8]`.
  - Assert `weights()`, `partition()`, `threshold()`, and `graph()` expose the expected data.
- `test_multiple_choice_branching_partition_validation`
  - Constructor should panic for out-of-range arc indices.
  - Constructor should panic for overlapping groups.
  - Constructor should panic if the partition omits an arc.
- `test_multiple_choice_branching_evaluate_yes_instance`
  - Use config `[1,0,1,0,0,1,0,1]`.
  - Assert `evaluate()` returns `true`.
- `test_multiple_choice_branching_rejects_constraint_violations`
  - One config that violates the partition constraint.
  - One config that violates in-degree-at-most-one.
  - One config that contains a directed cycle.
  - One config that stays feasible but misses the threshold.
- `test_multiple_choice_branching_solver_issue_examples`
  - Assert `BruteForce::find_satisfying()` finds a solution on the YES instance.
  - Assert `BruteForce::find_satisfying()` returns `None` on a small NO instance.
  - Assert every solution returned by `find_all_satisfying()` is accepted by `evaluate()`.
- `test_multiple_choice_branching_serialization`
  - Round-trip through serde JSON.

**Step 2: Run the focused test target and verify RED**

Run:

```bash
cargo test multiple_choice_branching --lib
```

Expected: compile or link failure because the new model/module does not exist yet.

**Step 3: Commit nothing yet**

Do not write production code until the failing tests are in place and the RED run has been observed.

### Task 2: Implement The Model And Register It

**Files:**
- Create: `src/models/graph/multiple_choice_branching.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`

**Step 1: Implement the minimal model to satisfy Task 1**

Model requirements:
- Add `inventory::submit!` `ProblemSchemaEntry` with:
  - `name: "MultipleChoiceBranching"`
  - `display_name: "Multiple Choice Branching"`
  - one variant dimension for `weight` with default `i32`
  - fields `graph`, `weights`, `partition`, `threshold`
- Define:

```rust
pub struct MultipleChoiceBranching<W> {
    graph: DirectedGraph,
    weights: Vec<W>,
    partition: Vec<Vec<usize>>,
    threshold: W::Sum,
}
```

- Constructor:
  - assert `weights.len() == graph.num_arcs()`
  - assert every partition entry is in `0..graph.num_arcs()`
  - assert each arc index appears exactly once across all groups
- Accessors/getters:
  - `graph()`
  - `weights()`
  - `set_weights(...)`
  - `partition()`
  - `threshold()`
  - `num_vertices()`
  - `num_arcs()`
  - `num_partition_groups()`
  - `is_weighted()`
  - `is_valid_solution(config)`
- `Problem` impl:
  - `type Metric = bool`
  - `variant() = crate::variant_params![W]`
  - `dims() = vec![2; self.graph.num_arcs()]`
  - `evaluate()` returns `true` iff:
    - config length matches `num_arcs`
    - every entry is `0` or `1`
    - each partition group contains at most one selected arc
    - selected arcs give every vertex in-degree at most one
    - selected arcs form an acyclic subgraph via `DirectedGraph::is_acyclic_subgraph`
    - selected weight sum is `>= threshold`
- `impl SatisfactionProblem for MultipleChoiceBranching<W>`
- `declare_variants!`:

```rust
crate::declare_variants! {
    default sat MultipleChoiceBranching<i32> => "2^num_arcs",
}
```

- Add the test link at the bottom.

**Step 2: Register the new graph module**

Update:
- `src/models/graph/mod.rs`
- `src/models/mod.rs`

Required outcomes:
- export `multiple_choice_branching`
- re-export `MultipleChoiceBranching`
- extend `canonical_model_example_specs()` in the graph module once Task 3 adds the example spec

**Step 3: Run the focused test target and verify GREEN**

Run:

```bash
cargo test multiple_choice_branching --lib
```

Expected: the model tests compile and pass.

### Task 3: Add The Canonical Example And Trait Coverage

**Files:**
- Modify: `src/models/graph/multiple_choice_branching.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/unit_tests/trait_consistency.rs`
- Reference: `src/unit_tests/example_db.rs`

**Step 1: Add the canonical model example to the model file**

Inside `#[cfg(feature = "example-db")] canonical_model_example_specs()`:
- Build the YES instance from issue #253 exactly.
- Register one example id: `multiple_choice_branching_i32`.
- Use `crate::example_db::specs::satisfaction_example(problem, vec![vec![1,0,1,0,0,1,0,1]])`.

**Step 2: Wire the canonical example into the graph example aggregator**

Extend `src/models/graph/mod.rs` so `canonical_model_example_specs()` includes the new model's specs.

**Step 3: Add trait-consistency coverage**

Update `src/unit_tests/trait_consistency.rs`:
- Add a `check_problem_trait(...)` call with a small `MultipleChoiceBranching<i32>` instance.
- Do not add a `test_direction` entry because this is not an optimization problem.

**Step 4: Run the focused tests**

Run:

```bash
cargo test trait_consistency --lib
cargo test example_db --lib
```

Expected: both pass with the new model included.

### Task 4: Add CLI Creation Support With TDD

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`

**Step 1: Write failing CLI tests first**

Add tests covering:
- `test_create_multiple_choice_branching`
  - Example command:

```bash
pred create MultipleChoiceBranching/i32 \
  --arcs "0>1,0>2,1>3,2>3,1>4,3>5,4>5,2>4" \
  --weights 3,2,4,1,2,3,1,3 \
  --partition "0,1;2,3;4,7;5,6" \
  --bound 10
```

  - Assert the emitted JSON has:
    - `problem_type == "MultipleChoiceBranching"`
    - variant `weight=i32`
    - the expected graph/weights/partition/threshold payload
- `test_create_multiple_choice_branching_from_example`
  - `pred create --example MultipleChoiceBranching/i32`
  - Assert it resolves to the canonical example.
- `test_create_multiple_choice_branching_round_trips_into_solve`
  - Pipe `pred create ...` into `pred solve - --solver brute-force`
  - Assert a satisfying solution is returned.

**Step 2: Run the CLI test target and verify RED**

Run:

```bash
cargo test -p problemreductions-cli test_create_multiple_choice_branching -- --nocapture
```

Expected: failure because the CLI does not know the new problem/flag yet.

**Step 3: Implement the CLI plumbing**

Update `problemreductions-cli/src/cli.rs`:
- Add `MultipleChoiceBranching` to the help tables and examples.
- Add a new flag:

```rust
#[arg(long)]
pub partition: Option<String>,
```

Update `all_data_flags_empty()` in `problemreductions-cli/src/commands/create.rs` to include `args.partition`.

Update `problemreductions-cli/src/commands/create.rs`:
- Add help/example text for `MultipleChoiceBranching`.
- Add a `create()` arm for `MultipleChoiceBranching`.
- Parse:
  - directed arcs from `--arcs`
  - arc weights from `--weights` using the existing integer-weight path
  - partition groups from `--partition` as semicolon-separated groups of comma-separated arc indices
  - threshold from `--bound` cast to `i32`
- Construct `MultipleChoiceBranching::new(...)`.
- Add a small `parse_partition_groups(...)` helper near the other parsing helpers.

**Step 4: Re-run the CLI target and verify GREEN**

Run:

```bash
cargo test -p problemreductions-cli test_create_multiple_choice_branching -- --nocapture
```

Expected: the new CLI tests pass.

### Task 5: Batch 1 Verification

**Files:**
- No new files

**Step 1: Run the implementation verification for Batch 1**

Run:

```bash
cargo test multiple_choice_branching --lib
cargo test trait_consistency --lib
cargo test example_db --lib
cargo test -p problemreductions-cli test_create_multiple_choice_branching -- --nocapture
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Expected: all pass before Batch 2 starts.

## Batch 2: Add-Model Step 6

### Task 6: Write The Paper Entry And Finalize The Paper Example Test

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/graph/multiple_choice_branching.rs`

**Step 1: Add the paper entry**

Update `docs/paper/reductions.typ`:
- Add `"MultipleChoiceBranching": [Multiple Choice Branching],` to `display-name`
- Add a `#problem-def("MultipleChoiceBranching")[...][...]` entry near the other graph problems
- Use the canonical YES instance from issue #253 for the example
- Show:
  - selected arcs
  - total weight `13`
  - one-per-group satisfaction
  - in-degree-at-most-one
  - acyclicity
- Cite the foundational context conservatively:
  - Garey & Johnson for NP-completeness/problem statement
  - Edmonds/Tarjan only for the special-case branching background
- Avoid making a stronger claim than the issue comments support about Suurballe; phrase it as "as referenced in Garey & Johnson" if retained at all

**Step 2: Finalize the `paper_example` unit test**

Add `test_multiple_choice_branching_paper_example` to `src/unit_tests/models/graph/multiple_choice_branching.rs`:
- Rebuild the exact paper example
- Assert `[1,0,1,0,0,1,0,1]` is satisfying
- Use `BruteForce::find_all_satisfying()` to compute the full satisfying set count
- Assert the paper prose matches that computed count exactly instead of hard-coding a guessed number

**Step 3: Verify the paper entry**

Run:

```bash
make paper
cargo test multiple_choice_branching --lib
```

Expected: the paper builds and the paper-example test passes.

### Task 7: Final Verification And Review Handoff

**Files:**
- No new files

**Step 1: Run the repo-level verification required before completion**

Run:

```bash
make test
make clippy
```

If runtime is acceptable, also run:

```bash
make paper
```

**Step 2: Run the implementation completeness review**

Run the repo-local review step after code is complete:
- `review-implementation` with auto-fixes if it finds structural gaps

**Step 3: Record PR summary notes**

The implementation summary comment should explicitly mention:
- new model file + tests
- new CLI `--partition` support
- canonical example/paper alignment
- orphan-model warning: no companion rule issue currently exists
