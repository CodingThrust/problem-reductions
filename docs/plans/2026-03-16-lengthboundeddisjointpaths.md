# LengthBoundedDisjointPaths Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `LengthBoundedDisjointPaths` graph satisfaction model, CLI creation support, canonical example, tests, and paper entry for issue `#298`.

**Architecture:** Implement the model as a generic graph satisfaction problem with `J * |V|` binary variables, where each path slot selects a vertex subset that must induce a simple `s-t` path of length at most `K`. Enforce internal-vertex disjointness across path slots inside `evaluate()`, declare `SimpleGraph` as the default registered variant, and keep the canonical public problem name as `LengthBoundedDisjointPaths`.

**Tech Stack:** Rust, serde, inventory registry, problem registry macros, CLI create command, example-db, Typst paper.

---

## Batch 1: Model, tests, registry, CLI, examples

### Task 1: Add failing model tests for the chosen encoding

**Files:**
- Create: `src/unit_tests/models/graph/length_bounded_disjoint_paths.rs`
- Reference: `src/unit_tests/models/graph/hamiltonian_path.rs`
- Reference: `src/unit_tests/models/graph/maximum_independent_set.rs`

**Step 1: Write the failing tests**

Add tests that cover:
- construction and size getters (`num_vertices`, `num_edges`, `num_paths_required`, `max_length`)
- a valid YES instance using the 7-vertex issue example with `s = 0`, `t = 6`, `J = 2`, `K = 3`
- invalid configs for missing source/sink, disconnected selected vertices, overlong paths, and shared internal vertices across slots
- brute-force solver behavior on a small YES instance and a small NO instance
- serde round-trip
- the paper/example instance count of satisfying solutions

Use the binary slot encoding directly in the test configs: the first `|V|` bits are slot 0, the next `|V|` bits are slot 1, etc.

**Step 2: Run the new test target and verify RED**

Run:
```bash
cargo test length_bounded_disjoint_paths --lib
```

Expected: compile or linkage failure because the model does not exist yet.

**Step 3: Commit after green later**

```bash
git add src/unit_tests/models/graph/length_bounded_disjoint_paths.rs
git commit -m "test: add LengthBoundedDisjointPaths model coverage"
```

### Task 2: Implement the model and make the tests pass

**Files:**
- Create: `src/models/graph/length_bounded_disjoint_paths.rs`
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Write the minimal model implementation**

Implement:
- `ProblemSchemaEntry` with canonical name `LengthBoundedDisjointPaths`
- `LengthBoundedDisjointPaths<G>` with fields `graph`, `source`, `sink`, `num_paths_required`, `max_length`
- inherent getters and `is_valid_solution()`
- `Problem<Metric = bool>` + `SatisfactionProblem`
- `dims() = vec![2; num_paths_required * num_vertices]`
- `evaluate()` that:
  - partitions the config into `J` path slots
  - validates each slot induces a connected simple `s-t` path
  - rejects paths longer than `K`
  - rejects reuse of any internal vertex across different slots
- `declare_variants!` with `default sat LengthBoundedDisjointPaths<SimpleGraph> => "2^(num_paths_required * num_vertices)"`
- canonical example-db spec for the same small YES instance used in tests

Keep helper functions private unless tests need them.

**Step 2: Register the model exports**

Wire the new file into:
- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs` prelude / re-exports

Also extend `canonical_model_example_specs()` in `src/models/graph/mod.rs`.

**Step 3: Run the targeted tests and verify GREEN**

Run:
```bash
cargo test length_bounded_disjoint_paths --lib
```

Expected: the new unit tests pass.

**Step 4: Commit**

```bash
git add src/models/graph/length_bounded_disjoint_paths.rs src/models/graph/mod.rs src/models/mod.rs src/lib.rs
git commit -m "feat: add LengthBoundedDisjointPaths model"
```

### Task 3: Add CLI creation support and trait consistency coverage

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `src/unit_tests/trait_consistency.rs`

**Step 1: Write the failing CLI/trait tests if coverage exists nearby, otherwise extend existing assertions first**

Add the trait-consistency entry for a small instance immediately, then add CLI support for:
- direct creation from `--graph`, `--source`, `--sink`, `--num-paths-required`, `--bound`
- random creation with sane defaults (`source = 0`, `sink = n - 1`, `num_paths_required = 1`, `bound = n - 1` unless overridden)

Update help text and `all_data_flags_empty()` for the new flags.

**Step 2: Implement the CLI arm**

Follow the `HamiltonianPath` / `OptimalLinearArrangement` patterns:
- extend `CreateArgs`
- add example/help text for `LengthBoundedDisjointPaths`
- parse required integers and build the model

**Step 3: Run targeted verification**

Run:
```bash
cargo test trait_consistency --lib
cargo test create --package problemreductions-cli
```

Expected: the trait consistency test stays green and CLI tests/build still pass for the touched areas.

**Step 4: Commit**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs src/unit_tests/trait_consistency.rs
git commit -m "feat: wire LengthBoundedDisjointPaths into CLI"
```

### Task 4: Run focused example-db and fixture verification

**Files:**
- Reference: `src/example_db/model_builders.rs`
- Reference: `src/unit_tests/example_db.rs`

**Step 1: Verify the canonical example is exported correctly**

Run:
```bash
cargo test example_db --lib --features example-db
```

Expected: the new model example is discoverable and structurally valid.

**Step 2: If fixture/export regeneration is required, run the repo command that updates checked-in exports**

Run the smallest repo-supported command needed after the model lands, then stage the generated changes if they are expected.

**Step 3: Commit if generated exports changed**

```bash
git add docs/src/reductions/problem_schemas.json docs/src/reductions/reduction_graph.json
git commit -m "chore: refresh generated model exports"
```

## Batch 2: Paper entry

### Task 5: Add the paper definition and example after implementation is stable

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add the display name**

Insert:
- `\"LengthBoundedDisjointPaths\": [Length-Bounded Disjoint Paths],`

**Step 2: Add the `problem-def(...)` entry**

Document:
- formal decision definition with `G`, `s`, `t`, `J`, `K`
- brief background and the Itai-Perl-Shiloach complexity threshold (`K >= 5` NP-complete, `K <= 4` polynomial)
- a small example that matches the canonical model example and explains the chosen satisfying configuration

Use the same 7-vertex example as the model/example-db tests so the paper, tests, and exports stay aligned.

**Step 3: Run paper and model verification**

Run:
```bash
make paper
cargo test length_bounded_disjoint_paths --lib --features example-db
```

Expected: paper compiles and the paper/example-alignment test remains green.

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: document LengthBoundedDisjointPaths"
```

## Final verification

After both batches are done, run:

```bash
make fmt
make check
```

If export files changed as part of verification, stage them before the final push.

## Notes / constraints

- Keep the public canonical problem name `LengthBoundedDisjointPaths`; do not reintroduce a `Maximum` prefix for the decision problem.
- The open inbound rule issue `#371` still uses stale wording (`MAXIMUM LENGTH-BOUNDED DISJOINT PATHS`). Treat that as a follow-up naming/tooling concern, not as justification to change the canonical model name in this PR.
- Keep the example small enough that brute force remains practical in unit tests.
