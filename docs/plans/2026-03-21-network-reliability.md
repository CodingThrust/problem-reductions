# NetworkReliability Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add the `NetworkReliability` model, exact reliability helpers, CLI/example-db integration, tests, and paper documentation for issue #235.

**Architecture:** Implement `NetworkReliability` as a `SatisfactionProblem` whose binary configuration marks which graph edges survive. `evaluate()` should answer the per-configuration predicate "are all terminals connected in the surviving subgraph?", while separate model-local helpers perform the exact probability summation required for the real decision question `R(G, T, p) >= q`. Keep the model non-variant (`SimpleGraph`, `Vec<f64>`, `f64`) and wire it through the existing registry, CLI, example-db, and paper patterns used by other graph models with terminal sets.

**Tech Stack:** Rust workspace (`problemreductions`, `problemreductions-cli`), `inventory`/`declare_variants!`, `DimsIterator` for exact enumeration, Typst paper docs, cargo/make verification.

---

## Batch 1: Model + Integration (add-model Steps 1-5.5)

### Task 1: Lock down the model contract with failing tests

**Files:**
- Create: `src/unit_tests/models/graph/network_reliability.rs`
- Reference: `src/models/graph/steiner_tree_in_graphs.rs`
- Reference: `src/unit_tests/models/graph/steiner_tree_in_graphs.rs`

**Step 1: Write the failing tests**

Add focused tests that describe the intended behavior before any production code exists:

```rust
#[test]
fn test_network_reliability_creation_and_getters() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let problem = NetworkReliability::new(graph, vec![0, 3], vec![0.1, 0.2, 0.3], 0.8);
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 3);
    assert_eq!(problem.num_terminals(), 2);
    assert_eq!(problem.dims(), vec![2; 3]);
    assert_eq!(problem.threshold(), 0.8);
}

#[test]
fn test_network_reliability_evaluate_checks_terminal_connectivity() {
    let graph = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (0, 3)]);
    let problem = NetworkReliability::new(graph, vec![0, 2], vec![0.1; 4], 0.5);
    assert!(problem.evaluate(&[1, 1, 0, 0]));
    assert!(!problem.evaluate(&[1, 0, 0, 0]));
}

#[test]
fn test_network_reliability_exact_probability_matches_issue_example() {
    let problem = issue_235_example();
    let reliability = problem.reliability();
    assert!((reliability - 0.968425).abs() < 1e-9);
    assert!(problem.meets_threshold());
}
```

Also cover invalid constructor inputs:
- `failure_probs.len() != num_edges`
- duplicate or out-of-bounds terminals
- fewer than 2 terminals
- failure probability outside `[0.0, 1.0]`
- threshold outside `[0.0, 1.0]`

**Step 2: Run the test target and verify RED**

Run:

```bash
cargo test network_reliability --lib
```

Expected: compile failure because `NetworkReliability` and its test module do not exist yet.

**Step 3: Write the minimal implementation to satisfy the tests**

Create `src/models/graph/network_reliability.rs` with:
- `ProblemSchemaEntry` describing `graph`, `terminals`, `failure_probs`, and `threshold`
- `#[derive(Debug, Clone, Serialize, Deserialize)] pub struct NetworkReliability`
- constructor validation for terminal distinctness/bounds and probability/threshold ranges
- accessors: `graph()`, `terminals()`, `failure_probs()`, `threshold()`
- size getters: `num_vertices()`, `num_edges()`, `num_terminals()`
- helpers:
  - `is_valid_solution(&self, config: &[usize]) -> bool`
  - `configuration_probability(&self, config: &[usize]) -> f64`
  - `reliability(&self) -> f64`
  - `meets_threshold(&self) -> bool`
- internal connectivity check for terminals in the surviving-edge subgraph
- `Problem<Metric = bool>` + `SatisfactionProblem`
- `crate::declare_variants! { default sat NetworkReliability => "2^num_edges * num_vertices" }`
- `canonical_model_example_specs()` for the issue example, with a representative connected configuration for example-db even though the issue’s real expected outcome is the aggregate reliability
- `#[cfg(test)]` link to the new test file

Register the new model in:
- `src/models/graph/mod.rs`
- `src/models/mod.rs`
- `src/lib.rs`

**Step 4: Run the tests and verify GREEN**

Run:

```bash
cargo test network_reliability --lib
```

Expected: the new model/unit tests pass.

**Step 5: Commit the model slice**

```bash
git add src/models/graph/network_reliability.rs src/models/graph/mod.rs src/models/mod.rs src/lib.rs src/unit_tests/models/graph/network_reliability.rs
git commit -m "feat: add NetworkReliability model"
```

### Task 2: Add CLI creation and example-db plumbing

**Files:**
- Modify: `problemreductions-cli/src/cli.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/tests/cli_tests.rs`
- Modify: `src/models/graph/mod.rs`
- Reference: `problemreductions-cli/src/commands/create.rs`

**Step 1: Write the failing CLI tests**

Add CLI tests that exercise both direct creation and canonical example creation:

```rust
#[test]
fn test_create_network_reliability() {
    // pred create NetworkReliability --graph ... --terminals 0,5 --failure-probs 0.1,... --threshold 0.95
}

#[test]
fn test_create_example_network_reliability() {
    // pred create --example NetworkReliability
}
```

Assertions should verify:
- JSON `"type"` is `"NetworkReliability"`
- `failure_probs` round-trip as floats
- `threshold` round-trips as `0.95`
- `terminals` round-trip correctly

**Step 2: Run the CLI tests and verify RED**

Run:

```bash
cargo test -p problemreductions-cli network_reliability
```

Expected: failure because the CLI has no `NetworkReliability` arm or `--failure-probs` / `--threshold` support yet.

**Step 3: Implement the CLI path**

Update `problemreductions-cli/src/cli.rs`:
- add `NetworkReliability --graph, --terminals, --failure-probs, --threshold` to the `after_help` table
- add `failure_probs: Option<String>` and `threshold: Option<f64>` to `CreateArgs`
- include the new flags in `all_data_flags_empty()`

Update `problemreductions-cli/src/commands/create.rs`:
- add example text for `NetworkReliability`
- add a parser helper for `--failure-probs` as a comma-separated `Vec<f64>` matching `num_edges`
- add a `NetworkReliability` match arm that parses graph, terminals, failure probabilities, and threshold
- prefer `--threshold` over reusing integer `--bound`, since this problem’s threshold is rational and should remain a `f64`
- keep alias resolution unchanged unless tests prove the registry-driven lookup is insufficient

Keep the example-db registration consistent by ensuring the new `canonical_model_example_specs()` is included in the graph example chain.

**Step 4: Run the CLI tests and targeted example-db coverage**

Run:

```bash
cargo test -p problemreductions-cli network_reliability
cargo test example_db --lib
```

Expected: CLI creation and example-db validation both pass.

**Step 5: Commit the integration slice**

```bash
git add problemreductions-cli/src/cli.rs problemreductions-cli/src/commands/create.rs problemreductions-cli/tests/cli_tests.rs src/models/graph/mod.rs
git commit -m "feat: wire NetworkReliability through CLI"
```

## Batch 2: Paper + Final Verification (add-model Steps 6-7)

### Task 3: Document the model and lock the paper example

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `src/unit_tests/models/graph/network_reliability.rs`

**Step 1: Add the paper regression test first**

Extend `src/unit_tests/models/graph/network_reliability.rs` with a paper-example test that:
- constructs the issue #235 example instance
- checks the representative connected configuration used by example-db/paper
- checks `reliability()` is `0.968425` within tolerance
- checks `meets_threshold()` is `true`

**Step 2: Run the targeted test and verify RED if the paper/example wiring is still absent**

Run:

```bash
cargo test network_reliability_example --lib
```

Expected: fail until the final example and paper narrative are aligned.

**Step 3: Write the paper entry**

Update `docs/paper/reductions.typ`:
- add `"NetworkReliability": [Network Reliability]` to the display-name dictionary
- add a `#problem-def("NetworkReliability")[...][...]` block near the graph problems
- explain the model carefully:
  - the configuration space is surviving/failing edge patterns
  - `evaluate()` checks terminal connectivity for one pattern
  - `reliability()` performs the exact weighted sum over all patterns
  - the decision question compares that sum to `q`
- include the issue example with the exact value `0.968425` and a small graph figure
- cite the literature from the issue/comments (`@garey1979`, `@valiant1979`, `@ball1986`, and the Rosenthal reference already available in the bibliography if present)

**Step 4: Run the paper and model checks**

Run:

```bash
cargo test network_reliability --lib
make paper
```

Expected: the paper builds cleanly and the example tests stay green.

**Step 5: Commit the documentation slice**

```bash
git add docs/paper/reductions.typ src/unit_tests/models/graph/network_reliability.rs
git commit -m "docs: add NetworkReliability paper entry"
```

### Task 4: Full verification before handoff

**Files:**
- Modify only if verification exposes defects

**Step 1: Run formatting and regression checks**

Run:

```bash
cargo fmt --all
cargo test network_reliability --lib
cargo test example_db --lib
cargo test -p problemreductions-cli network_reliability
make paper
```

If any command fails, fix the issue and re-run the affected command before moving on.

**Step 2: Run a broader workspace confidence check**

Run:

```bash
make check
```

Expected: formatting, clippy, and workspace tests pass.

**Step 3: Prepare for issue-to-pr cleanup**

After implementation succeeds:
- ensure all code changes are committed
- leave `docs/plans/2026-03-21-network-reliability.md` in place for the initial plan PR commit only
- let the outer `issue-to-pr` workflow remove this file in its dedicated cleanup commit

**Step 4: Final implementation summary inputs**

Capture for the PR summary comment:
- model files added/changed
- CLI flags introduced (`--failure-probs`, `--threshold`)
- exact example result (`0.968425 > 0.95`)
- explicit design deviation: the standard solver still finds connected configurations, while exact reliability comparison lives in model-local helpers
