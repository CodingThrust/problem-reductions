# SteinerTree -> ILP Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement the direct `SteinerTree<SimpleGraph, i32> -> ILP<bool>` reduction from issue #123, add the canonical example/paper entry, and make the rule available through the reduction graph and `solve_reduced`.

**Architecture:** Add a new `src/rules/steinertree_ilp.rs` module that builds a rooted multi-commodity-flow ILP: one binary selector `y_e` per source edge plus one binary directed-arc flow variable for each non-root terminal and each directed source edge. The source witness is exactly the first `m` target variables, so solution extraction is a prefix read. Keep the paper work in a separate batch after the rule, exports, and example witness are stable.

**Tech Stack:** Rust, `good_lp` via `ilp-highs`, Typst, repo example-db exports, GitHub pipeline scripts

---

## Batch 1: Rule, Tests, and Canonical Example

### Task 1: Write the red tests and wire the module into the rule registry

**Files:**
- Create: `src/unit_tests/rules/steinertree_ilp.rs`
- Modify: `src/rules/mod.rs`
- Reference: `src/rules/minimummultiwaycut_ilp.rs`
- Reference: `src/unit_tests/rules/minimummultiwaycut_ilp.rs`
- Reference: `src/models/graph/steiner_tree.rs`
- Reference: `src/models/algebraic/ilp.rs`

**Step 1: Write the failing test file**

Create `src/unit_tests/rules/steinertree_ilp.rs` with a canonical instance helper and these tests:

```rust
use super::*;
use crate::models::algebraic::ObjectiveSense;
use crate::solvers::{BruteForce, ILPSolver};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::SolutionSize;

fn canonical_instance() -> SteinerTree<SimpleGraph, i32> {
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (1, 3), (3, 4), (0, 3), (3, 2), (2, 4)],
    );
    SteinerTree::new(graph, vec![2, 2, 1, 1, 5, 5, 6], vec![0, 2, 4])
}

#[test]
fn test_reduction_creates_expected_ilp_shape() {
    let problem = canonical_instance();
    let reduction: ReductionSteinerTreeToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    assert_eq!(ilp.num_vars, 35);
    assert_eq!(ilp.constraints.len(), 38);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);
    assert_eq!(
        ilp.objective,
        vec![(0, 2.0), (1, 2.0), (2, 1.0), (3, 1.0), (4, 5.0), (5, 5.0), (6, 6.0)],
    );
}

#[test]
fn test_steinertree_to_ilp_closed_loop() {
    let problem = canonical_instance();
    let reduction: ReductionSteinerTreeToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let bf = BruteForce::new();
    let ilp_solver = ILPSolver::new();
    let best_source = bf.find_all_best(&problem);
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    assert_eq!(problem.evaluate(&best_source[0]), SolutionSize::Valid(6));
    assert_eq!(problem.evaluate(&extracted), SolutionSize::Valid(6));
    assert!(problem.is_valid_solution(&extracted));
}

#[test]
fn test_solution_extraction_reads_edge_selector_prefix() {
    let problem = canonical_instance();
    let reduction: ReductionSteinerTreeToILP = ReduceTo::<ILP<bool>>::reduce_to(&problem);

    let target_solution = vec![
        1, 1, 1, 1, 0, 0, 0,
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ];

    assert_eq!(reduction.extract_solution(&target_solution), vec![1, 1, 1, 1, 0, 0, 0]);
}

#[test]
fn test_solve_reduced_uses_new_rule() {
    let problem = canonical_instance();
    let solution = ILPSolver::new()
        .solve_reduced(&problem)
        .expect("solve_reduced should find the Steiner tree via ILP");
    assert_eq!(problem.evaluate(&solution), SolutionSize::Valid(6));
}

#[test]
#[should_panic(expected = "SteinerTree -> ILP requires nonnegative edge weights")]
fn test_reduction_rejects_negative_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = SteinerTree::new(graph, vec![1, -2, 3], vec![0, 1]);
    let _ = ReduceTo::<ILP<bool>>::reduce_to(&problem);
}
```

Do not use `assert_optimization_round_trip_from_optimization_target` here; the reduced ILP has 35 binary variables, so the target must be solved with `ILPSolver`, not brute force.

**Step 2: Register the new module so the tests compile red**

Modify `src/rules/mod.rs`:

```rust
#[cfg(feature = "ilp-solver")]
pub(crate) mod steinertree_ilp;
```

And extend the example-spec collector inside the `#[cfg(feature = "ilp-solver")]` block:

```rust
specs.extend(steinertree_ilp::canonical_rule_example_specs());
```

**Step 3: Run the targeted test command and verify RED**

Run:

```bash
cargo test --features "ilp-highs example-db" steinertree_ilp -- --include-ignored
```

Expected: FAIL because `src/rules/steinertree_ilp.rs` and `ReductionSteinerTreeToILP` do not exist yet.

**Step 4: Do not commit yet**

Leave the branch in the red state and move directly to Task 2.

### Task 2: Implement the reduction module and make the targeted tests green

**Files:**
- Create: `src/rules/steinertree_ilp.rs`
- Modify: `src/rules/mod.rs`
- Test: `src/unit_tests/rules/steinertree_ilp.rs`

**Step 1: Create the rule module with the same shape as other direct ILP reductions**

Start from the `minimummultiwaycut_ilp.rs` pattern:

```rust
use crate::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
use crate::models::graph::SteinerTree;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

#[derive(Debug, Clone)]
pub struct ReductionSteinerTreeToILP {
    target: ILP<bool>,
    num_edges: usize,
}

impl ReductionResult for ReductionSteinerTreeToILP {
    type Source = SteinerTree<SimpleGraph, i32>;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_edges].to_vec()
    }
}
```

**Step 2: Implement the rooted multi-commodity variable layout**

Use this exact indexing scheme:

```rust
let m = self.num_edges();
let terminals = self.terminals();
let non_root = &terminals[1..];

let edge_var = |edge_idx: usize| edge_idx;
let flow_var = |terminal_pos: usize, edge_idx: usize, dir: usize| {
    m + terminal_pos * 2 * m + 2 * edge_idx + dir
};
```

Interpretation:
- `dir = 0` means the edge orientation as stored in `graph.edges()[edge_idx]`, `u -> v`
- `dir = 1` means the reverse orientation, `v -> u`
- total variables: `m + 2 * m * (k - 1)`

**Step 3: Implement the ILP constraints**

Use the issue's exact formulation:

```rust
#[reduction(
    overhead = {
        num_vars = "num_edges + 2 * num_edges * (num_terminals - 1)",
        num_constraints = "num_vertices * (num_terminals - 1) + 2 * num_edges * (num_terminals - 1)",
    }
)]
impl ReduceTo<ILP<bool>> for SteinerTree<SimpleGraph, i32> {
    type Result = ReductionSteinerTreeToILP;

    fn reduce_to(&self) -> Self::Result {
        assert!(
            self.edge_weights().iter().all(|&w| w >= 0),
            "SteinerTree -> ILP requires nonnegative edge weights",
        );

        // build:
        // 1. one flow-conservation equality per (terminal, vertex)
        // 2. two capacity-linking inequalities per (terminal, edge)
        // 3. objective on y_e only
    }
}
```

For each non-root terminal `t` and each vertex `v`, add one equality:
- root `r = terminals[0]`: incoming minus outgoing `= -1`
- sink `v = t`: incoming minus outgoing `= 1`
- otherwise `= 0`

For each non-root terminal and each undirected edge `e = (u, v)`:
- `f_t(u,v) <= y_e`
- `f_t(v,u) <= y_e`

Keep the target domain binary (`ILP<bool>`). Even though the issue writes `f^t_(u,v) in [0,1]`, a binary flow variable is valid here because each commodity can be routed on a simple root-to-terminal path inside an optimal tree, and the codebase does not expose a continuous LP model.

**Step 4: Implement the canonical example in the rule module**

Add `#[cfg(feature = "example-db")]` `canonical_rule_example_specs()` in the new rule file. Reuse the exact issue-123 instance:

```rust
let source = SteinerTree::new(
    SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (1, 3), (3, 4), (0, 3), (3, 2), (2, 4)],
    ),
    vec![2, 2, 1, 1, 5, 5, 6],
    vec![0, 2, 4],
);
```

Use this canonical witness pair:

```rust
SolutionPair {
    source_config: vec![1, 1, 1, 1, 0, 0, 0],
    target_config: vec![
        1, 1, 1, 1, 0, 0, 0,
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
}
```

That target witness corresponds to:
- root `0`
- commodity `2`: path `0 -> 1 -> 2`
- commodity `4`: path `0 -> 1 -> 3 -> 4`

**Step 5: Run the targeted tests and verify GREEN**

Run:

```bash
cargo test --features "ilp-highs example-db" steinertree_ilp -- --include-ignored
```

Expected: PASS for all `steinertree_ilp` tests.

**Step 6: Commit the green implementation**

Run:

```bash
git add src/rules/mod.rs src/rules/steinertree_ilp.rs src/unit_tests/rules/steinertree_ilp.rs
git commit -m "Add SteinerTree to ILP reduction"
```

### Task 3: Run rule-level exports and verify the example path before touching the paper

**Files:**
- Modify: `src/rules/steinertree_ilp.rs` if export/example issues appear
- Reference: `src/rules/mod.rs`

**Step 1: Regenerate the example export and graph metadata**

Run:

```bash
cargo run --features "example-db" --example export_examples
cargo run --example export_graph
cargo run --example export_schemas
```

Expected:
- `export_examples` includes the new `SteinerTree -> ILP` canonical rule entry
- `export_graph` sees the new edge
- `export_schemas` remains clean

**Step 2: Re-run the targeted rule tests after exports**

Run:

```bash
cargo test --features "ilp-highs example-db" steinertree_ilp -- --include-ignored
```

Expected: PASS again.

**Step 3: If the export/example step exposed mismatches, fix them before Batch 2**

Typical fixes:
- wrong `target_config` ordering in the example witness
- missing `canonical_rule_example_specs()` registration in `src/rules/mod.rs`
- overhead mismatch caused by the wrong variable count formula

Do not start the paper until the exported example data is stable.

## Batch 2: Paper Entry and Final Verification

### Task 4: Add the Typst paper entry and bibliography

**Files:**
- Modify: `docs/paper/reductions.typ`
- Modify: `docs/paper/references.bib`
- Reference: `docs/paper/reductions.typ` (`KColoring -> QUBO`, `MinimumMultiwayCut -> ILP`)

**Step 1: Add the missing bibliography entries with corrected DOIs**

Add entries for:
- Wong, 1984, `10.1007/BF02612335`
- Koch and Martin, 1998, `10.1002/(SICI)1097-0037(199810)32:3<207::AID-NET5>3.0.CO;2-O`

Use stable keys that match the rest of the bibliography, for example:
- `wong1984steiner`
- `kochmartin1998steiner`

**Step 2: Load the canonical example data near the ILP section**

Add a local binding before the new theorem:

```typst
#let st_ilp = load-example("SteinerTree", "ILP")
#let st_ilp_sol = st_ilp.solutions.at(0)
```

**Step 3: Write the new `reduction-rule("SteinerTree", "ILP", ...)` entry**

Place it near the other network-flow ILP formulations, preferably adjacent to `MinimumMultiwayCut -> ILP`.

The theorem body should state:
- this is the standard multi-commodity flow ILP formulation
- target size is `m + 2m(k - 1)` variables and `n(k - 1) + 2m(k - 1)` constraints
- citations point to the corrected Wong/Koch-Martin references

The proof body should include:
- `_Construction._` root choice, `y_e`, arc-flow variables, conservation/linking constraints, minimize edge weights
- `_Correctness._` why any Steiner tree induces feasible flows, and why any optimal feasible solution gives a minimum-cost connected subgraph whose selected edges can be read as a Steiner tree under the nonnegative-weight assumption
- `_Solution extraction._` read the first `m` variables as edge selectors

**Step 4: Add a worked example block driven by JSON data**

Set `example: true` and use the issue's 5-vertex, 7-edge, 3-terminal instance. The extra block should:
- show the source graph, terminals, and root `0`
- explain `7 + 2 * 7 * 2 = 35` variables and `5 * 2 + 2 * 7 * 2 = 38` constraints
- walk through the two commodity paths encoded by `st_ilp_sol.target_config`
- verify that the selected edges are `{(0,1), (1,2), (1,3), (3,4)}` with total cost `6`
- state that the fixture stores one canonical witness

Do not hardcode the final counts or objective; pull them from `st_ilp.source.instance`, `st_ilp.target.instance`, and `st_ilp_sol`.

**Step 5: Build the paper and verify GREEN**

Run:

```bash
make paper
```

Expected: PASS with no new completeness warnings for missing `SteinerTree -> ILP` coverage.

**Step 6: Commit the paper/docs batch**

Run:

```bash
git add docs/paper/reductions.typ docs/paper/references.bib
git commit -m "Document SteinerTree to ILP reduction"
```

### Task 5: Final verification for the full issue branch

**Files:**
- Verify: `src/rules/steinertree_ilp.rs`
- Verify: `src/unit_tests/rules/steinertree_ilp.rs`
- Verify: `src/rules/mod.rs`
- Verify: `docs/paper/reductions.typ`
- Verify: `docs/paper/references.bib`

**Step 1: Run the focused regression checks**

Run:

```bash
cargo test --features "ilp-highs example-db" steinertree_ilp -- --include-ignored
```

Expected: PASS.

**Step 2: Run the repo-wide required checks**

Run:

```bash
make clippy
make test
```

Expected:
- `make clippy` passes with `-D warnings`
- `make test` passes with `--features "ilp-highs example-db" -- --include-ignored`

**Step 3: Confirm the working tree only has intended tracked changes**

Run:

```bash
git status --short
```

Expected:
- tracked changes only in the rule/test/docs files above
- generated paper/example outputs remain ignored

**Step 4: Commit any final fixups**

If verification forced additional source changes, run:

```bash
git add -A
git commit -m "Polish SteinerTree to ILP implementation"
```

If verification is clean, do not add an extra commit.
