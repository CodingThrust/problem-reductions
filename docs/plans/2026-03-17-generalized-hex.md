# GeneralizedHex Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `GeneralizedHex` graph model, expose it through the registry/CLI/example database, and document it in the paper with the issue's worked example.

**Architecture:** Model `GeneralizedHex` as a graph-based zero-variable `SatisfactionProblem`: the graph instance lives in the struct, and `evaluate([])` computes whether Player 1 has a forced win from the initial empty board. Use a memoized ternary game-state search over playable vertices (`unclaimed`, `blue`, `red`) with two fast terminal checks: an existing blue `s-t` path is an immediate win, and the absence of any `s-t` path in the graph induced by non-red vertices is an immediate loss.

**Tech Stack:** Rust, serde, inventory registry metadata, existing `SimpleGraph` topology, CLI `pred create`, Typst paper docs, cargo test/clippy.

---

## Batch 1: Model, Registration, CLI, Examples, Tests

### Task 1: Write the failing model tests first

**Files:**
- Create: `src/unit_tests/models/graph/generalized_hex.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Write the failing tests**

Add focused tests for the intended semantics:

```rust
#[test]
fn test_generalized_hex_creation_and_getters() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = GeneralizedHex::new(graph, 0, 3);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_playable_vertices(), 2);
    assert_eq!(problem.dims(), Vec::<usize>::new());
}

#[test]
fn test_generalized_hex_forced_win_on_issue_example() {
    let graph = SimpleGraph::new(
        8,
        vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 3), (2, 5), (3, 6), (4, 6), (5, 6), (6, 7)],
    );
    let problem = GeneralizedHex::new(graph, 0, 7);
    assert!(problem.evaluate(&[]));
}

#[test]
fn test_generalized_hex_detects_losing_position() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 3), (0, 2), (2, 3)]);
    let problem = GeneralizedHex::new(graph, 0, 3);
    assert!(!problem.evaluate(&[]));
}
```

Also include:
- a solver test asserting `BruteForce::new().find_satisfying(&problem) == Some(vec![])` for a winning instance
- a paper-example test placeholder matching the issue's worked example
- a validation test for invalid terminals (same endpoint or out of range)

**Step 2: Run the targeted test to verify RED**

Run:

```bash
cargo test generalized_hex --lib
```

Expected: compile failure because `GeneralizedHex` and its module/test hook do not exist yet.

**Step 3: Commit the test scaffold once it exists**

```bash
git add src/unit_tests/models/graph/generalized_hex.rs src/models/graph/mod.rs
git commit -m "test: add GeneralizedHex model coverage"
```

### Task 2: Implement the `GeneralizedHex` model

**Files:**
- Create: `src/models/graph/generalized_hex.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Implement the minimal model to satisfy the tests**

Build `GeneralizedHex<G>` with:
- schema metadata using the graph-variant pattern (`graph`, `source`, `target`)
- `new(graph, source, target)` with terminal validation
- getters `graph()`, `source()`, `target()`, `num_vertices()`, `num_edges()`, `num_playable_vertices()`
- `Problem` + `SatisfactionProblem` impls where `dims()` is empty and `evaluate([])` runs the forced-win search
- `declare_variants! { default sat GeneralizedHex<SimpleGraph> => "3^(num_vertices - 2)" }`

Core recursive helper shape:

```rust
fn first_player_wins(&self, state: &mut [ClaimState], memo: &mut HashMap<Vec<ClaimState>, bool>) -> bool {
    if self.has_blue_connection(state) {
        return true;
    }
    if !self.has_open_connection(state) {
        return false;
    }
    if let Some(&cached) = memo.get(state) {
        return cached;
    }
    let blue_turn = self.blue_count(state) == self.red_count(state);
    let answer = if blue_turn {
        self.unclaimed_vertices(state)
            .any(|idx| self.try_blue_move(idx, state, memo))
    } else {
        self.unclaimed_vertices(state)
            .all(|idx| self.try_red_move(idx, state, memo))
    };
    memo.insert(state.to_vec(), answer);
    answer
}
```

Represent only non-terminal vertices in the mutable state and map local indices back to graph vertices via a cached `playable_vertices: Vec<usize>`.

**Step 2: Run the targeted tests to verify GREEN**

Run:

```bash
cargo test generalized_hex --lib
```

Expected: the new model tests pass.

**Step 3: Refactor for clarity while staying green**

Refactor only after green:
- separate connectivity helpers for `blue` and `non-red`
- use a compact `ClaimState` enum or `u8`
- keep recursion deterministic and memoized

**Step 4: Commit**

```bash
git add src/models/graph/generalized_hex.rs src/models/graph/mod.rs
git commit -m "feat: implement GeneralizedHex model"
```

### Task 3: Register the model across the crate and example database

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`

**Step 1: Add the crate wiring**

Register the new module and exports:
- `pub(crate) mod generalized_hex;`
- `pub use generalized_hex::GeneralizedHex;`
- include `GeneralizedHex` in graph re-exports and `prelude`
- add `generalized_hex::canonical_model_example_specs()` to the graph example registry

**Step 2: Add a canonical example**

In `src/models/graph/generalized_hex.rs`, add:

```rust
#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<ModelExampleSpec> {
    vec![ModelExampleSpec {
        id: "generalized_hex_simplegraph",
        build: || {
            let problem = GeneralizedHex::new(/* issue example graph */, 0, 7);
            crate::example_db::specs::satisfaction_example(problem, vec![vec![]])
        },
    }]
}
```

The satisfying config is `vec![]`, because the problem has zero variables and the decision result is stored in `evaluate([])`.

**Step 3: Run a narrow registration check**

Run:

```bash
cargo test generalized_hex --lib
```

Expected: tests remain green and the model is discoverable through the catalog build.

**Step 4: Commit**

```bash
git add src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/models/graph/generalized_hex.rs
git commit -m "feat: register GeneralizedHex in model catalog"
```

### Task 4: Add CLI creation support

**Files:**
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/cli.rs`

**Step 1: Write/extend failing CLI-focused tests**

Add or extend command/create coverage for:
- `pred create GeneralizedHex --graph ... --source 0 --target 7`
- helpful error when `--source`/`--target` is missing
- example/help text mentions `GeneralizedHex`

If there is no focused CLI test file yet, add the narrowest viable regression in the existing create-command test module.

**Step 2: Run the targeted CLI test to verify RED**

Run:

```bash
cargo test create:: --package problemreductions-cli
```

Expected: failure because `GeneralizedHex` is not yet handled in CLI creation.

**Step 3: Implement the minimal CLI support**

Add:
- a `GeneralizedHex` import in `problemreductions-cli/src/commands/create.rs`
- a `match` arm alongside graph-only problems that parses `--graph`, validates `--source` and `--target`, and serializes `GeneralizedHex::new(graph, source, target)`
- help-table entries and examples in `problemreductions-cli/src/cli.rs`

Prefer existing `source`/`target` flags; do not add new CLI flags unless the current ones are insufficient.

**Step 4: Run the targeted CLI tests to verify GREEN**

Run:

```bash
cargo test create:: --package problemreductions-cli
```

Expected: the new CLI coverage passes.

**Step 5: Commit**

```bash
git add problemreductions-cli/src/commands/create.rs problemreductions-cli/src/cli.rs
git commit -m "feat: add CLI support for GeneralizedHex"
```

### Task 5: Finish model tests and repository verification for Batch 1

**Files:**
- Modify: `src/unit_tests/models/graph/generalized_hex.rs`
- Modify: `src/unit_tests/graph_models.rs` (only if this module list needs an entry)

**Step 1: Fill in the remaining model coverage**

Ensure the final test file covers:
- constructor/getters
- empty-dimension semantics
- winning and losing instances
- `BruteForce` solving from the empty config
- paper example
- serde round-trip if the model is exercised through CLI/example-db flows

**Step 2: Run the focused model suite**

Run:

```bash
cargo test generalized_hex
```

Expected: all `generalized_hex` tests pass.

**Step 3: Run broader checks for the touched areas**

Run:

```bash
cargo test --package problemreductions-cli create::
cargo test generalized_hex
```

Expected: both commands pass. If the branch has unrelated baseline failures, note them explicitly before moving to Batch 2.

**Step 4: Commit**

```bash
git add src/unit_tests/models/graph/generalized_hex.rs src/unit_tests/graph_models.rs
git commit -m "test: complete GeneralizedHex coverage"
```

## Batch 2: Paper Entry

### Task 6: Document `GeneralizedHex` in the Typst paper

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/graph/generalized_hex.rs`

**Step 1: Write the paper entry**

Add:
- a display-name dictionary entry for `GeneralizedHex`
- `#problem-def("GeneralizedHex")[...][...]`
- a short background referencing the Even-Tarjan PSPACE-completeness result and the Bruno-Weinberg edge-game contrast
- an example matching the issue's 8-vertex graph, with a graph diagram and prose explaining why claiming vertex `6` is the critical first move

Use the canonical example-db instance as the paper source of truth.

**Step 2: Add/finish the paper-example unit test**

Tie `test_generalized_hex_paper_example` to the exact issue example and assert:
- `evaluate(&[]) == true`
- `BruteForce::new().find_satisfying(&problem) == Some(vec![])`

**Step 3: Run the paper build**

Run:

```bash
make paper
```

Expected: the Typst paper compiles successfully.

**Step 4: Commit**

```bash
git add docs/paper/reductions.typ src/unit_tests/models/graph/generalized_hex.rs
git commit -m "docs: add GeneralizedHex paper entry"
```

## Final Verification

### Task 7: Full verification and cleanup

**Files:**
- Modify: any touched files as needed from review fixes

**Step 1: Run formatting and targeted verification**

Run:

```bash
cargo fmt --all
cargo test generalized_hex
cargo test --package problemreductions-cli create::
make paper
```

Expected: all commands pass.

**Step 2: Run broader repository checks if the branch is green**

Run:

```bash
make test
make clippy
```

Expected: pass. If unrelated baseline failures remain on `origin/main`, record them clearly in the PR summary instead of hiding them.

**Step 3: Prepare implementation summary for the PR**

Capture:
- files added/changed
- final design choice: zero-variable model with recursive forced-win evaluator
- any deviations from the issue text (for example, replacing the issue's assignment-based `dims()` with an empty config because the formal decision problem is evaluated directly)

**Step 4: Final commit**

```bash
git add -A
git commit -m "feat: complete GeneralizedHex implementation"
```
