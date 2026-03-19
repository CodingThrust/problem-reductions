# ClosestVectorProblem to QUBO Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the `ClosestVectorProblem<i32> -> QUBO<f64>` reduction, validate it against brute-force CVP/QUBO solving on bounded instances, and document the construction in the paper.

**Architecture:** Follow the paper's Gram-matrix expansion `x^T G x - 2 h^T x`, but adapt the integer encoding to the repository's bounded `VarBounds` model. Use an exact in-range bit basis per variable so every QUBO assignment reconstructs a legal CVP coefficient without needing a separate out-of-range penalty term. Store the per-variable offsets and bit spans in the reduction result for `extract_solution()`, and drive correctness with round-trip brute-force tests plus a canonical example fixture.

**Tech Stack:** Rust workspace, `cargo test`, `cargo run --example export_graph`, `cargo run --example export_schemas`, `make regenerate-fixtures`, `make paper`

---

## Chunk 1: Rule Implementation And Validation

### Task 1: Add bounded-encoding helpers to `ClosestVectorProblem`

**Files:**
- Modify: `src/models/algebraic/closest_vector_problem.rs`
- Test: `src/unit_tests/models/algebraic/closest_vector_problem.rs`

- [ ] **Step 1: Write failing model tests for the new overhead/helper API**

Add focused tests for helper methods that the reduction will depend on:

```rust
#[test]
fn test_num_encoding_bits_uses_var_bounds_ranges() {
    let problem = ClosestVectorProblem::new(
        vec![vec![2, 0], vec![1, 2]],
        vec![2.8, 1.5],
        vec![VarBounds::bounded(-2, 4), VarBounds::bounded(0, 5)],
    );

    assert_eq!(problem.num_encoding_bits(), 6);
}
```

Also add a small helper test that exercises the exact bounded basis for one range size where plain powers of two would overrun, such as 5 or 6 values.

- [ ] **Step 2: Run the focused model tests to verify they fail**

Run: `cargo test num_encoding_bits --lib`

Expected: FAIL because `ClosestVectorProblem` does not yet expose the helper(s).

- [ ] **Step 3: Add the minimal helper API to the model**

Implement small, reduction-oriented helpers on `ClosestVectorProblem<T>`:

```rust
pub fn num_encoding_bits(&self) -> usize { ... }
```

and a private/shared helper that derives the exact bounded bit weights for one `VarBounds` range. Use the encoding basis:

- powers of two for the low-order bits
- a capped final weight so the maximum representable offset is exactly `hi - lo`
- zero bits when `lo == hi`

This lets the reduction use a single overhead getter (`num_encoding_bits`) while keeping the encoding math centralized near the model.

- [ ] **Step 4: Run the focused model tests to verify they pass**

Run: `cargo test closest_vector_problem --lib`

Expected: PASS for the updated model test module.

- [ ] **Step 5: Commit the helper-only slice**

```bash
git add src/models/algebraic/closest_vector_problem.rs src/unit_tests/models/algebraic/closest_vector_problem.rs
git commit -m "feat(cvp): add bounded encoding helpers for QUBO reduction"
```

### Task 2: Implement the reduction and rule tests

**Files:**
- Create: `src/rules/closestvectorproblem_qubo.rs`
- Modify: `src/rules/mod.rs`
- Create: `src/unit_tests/rules/closestvectorproblem_qubo.rs`

- [ ] **Step 1: Write failing rule tests before touching the reduction**

Cover three behaviors:

```rust
#[test]
fn test_closestvectorproblem_to_qubo_closed_loop() { ... }

#[test]
fn test_closestvectorproblem_to_qubo_example_matrix_matches_issue_fixture() { ... }

#[test]
fn test_extract_solution_ignores_duplicate_exact_range_encodings() { ... }
```

The closed-loop test should:
- build the canonical bounded `ClosestVectorProblem<i32>` instance from the issue
- reduce it to `QUBO<f64>`
- solve the target with `BruteForce`
- extract the source solution
- verify the extracted assignment is source-optimal

The matrix-shape test should assert the target has 6 QUBO variables for the canonical `[-2,4]` x `[-2,4]` example and that key diagonal/off-diagonal coefficients match the issue's worked example or the implementation's exact-range variant, whichever the final construction uses.

- [ ] **Step 2: Run the focused rule tests to verify they fail**

Run: `cargo test closestvectorproblem_qubo --lib`

Expected: FAIL because the rule module is missing.

- [ ] **Step 3: Implement `src/rules/closestvectorproblem_qubo.rs`**

Follow `add-rule` Step 1 and mirror the structure of other `*_qubo.rs` reductions:

- `ReductionClosestVectorProblemToQUBO`
- `ReductionResult` impl storing:
  - `target: QUBO<f64>`
  - per-variable lower bounds
  - per-variable bit spans or start indices needed for reconstruction
- `#[reduction(overhead = { num_vars = "num_encoding_bits" })]`
- `impl ReduceTo<QUBO<f64>> for ClosestVectorProblem<i32>`

Implementation details:
- build the exact bounded encoding coefficients for each CVP variable from its `VarBounds`
- compute `G = A^T A` and `h = A^T t`
- expand the objective after substituting `x_i = lo_i + sum_j w_(i,j) b_(i,j)`
- drop only the constant term
- populate the upper-triangular QUBO matrix using the repo's `QUBO::from_matrix` convention
- reconstruct `x` in `extract_solution()` by summing the selected encoding weights and adding each lower bound

- [ ] **Step 4: Register the rule module**

Add the new module in `src/rules/mod.rs` in the same area as the other QUBO reductions.

- [ ] **Step 5: Run the focused rule tests to verify they pass**

Run:

```bash
cargo test closestvectorproblem_qubo --lib
cargo test closest_vector_problem --lib
```

Expected: PASS for the new rule tests and no regressions in the CVP model tests.

- [ ] **Step 6: Commit the rule slice**

```bash
git add src/rules/closestvectorproblem_qubo.rs src/rules/mod.rs src/unit_tests/rules/closestvectorproblem_qubo.rs
git commit -m "feat(rules): add ClosestVectorProblem to QUBO reduction"
```

### Task 3: Add the canonical example and regenerate machine-readable data

**Files:**
- Modify: `src/rules/closestvectorproblem_qubo.rs`
- Generated/updated by commands: `docs/src/reductions/reduction_graph.json`, `docs/src/reductions/problem_schemas.json`, `src/example_db/fixtures/examples.json`

- [ ] **Step 1: Add the canonical example spec next to the rule**

Reuse the issue's 2D bounded instance:

- basis columns `[(2,0), (1,2)]`
- target `(2.8, 1.5)`
- bounds `[-2,4]` for both coefficients
- source witness `(1,1)` encoded as the target bit vector produced by the exact bounded encoding used in the implementation

Keep this in the rule file's `#[cfg(feature = "example-db")] canonical_rule_example_specs()` block, following other reductions that colocate the example with the rule.

- [ ] **Step 2: Run exports so the example and reduction graph exist for later paper work**

Run:

```bash
cargo run --example export_graph
cargo run --example export_schemas
make regenerate-fixtures
```

Expected:
- the new `ClosestVectorProblem -> QUBO` edge appears in the reduction graph export
- the example fixture contains the new rule example entry

- [ ] **Step 3: Run a broader implementation verification pass**

Run:

```bash
make test
make clippy
```

Expected: PASS before moving to the paper batch.

- [ ] **Step 4: Commit the example/export slice**

```bash
git add src/rules/closestvectorproblem_qubo.rs docs/src/reductions/reduction_graph.json docs/src/reductions/problem_schemas.json src/example_db/fixtures/examples.json
git commit -m "chore(exports): add CVP to QUBO example data"
```

## Chunk 2: Paper Entry

### Task 4: Document the reduction in `docs/paper/reductions.typ`

**Files:**
- Modify: `docs/paper/reductions.typ`

- [ ] **Step 1: Add the display/loading glue near the other QUBO examples if needed**

Introduce a `load-example("ClosestVectorProblem", "QUBO")` binding near the QUBO reduction section so the worked example can reuse the exported JSON fixture instead of hardcoding data.

- [ ] **Step 2: Add the `reduction-rule("ClosestVectorProblem", "QUBO", ...)` entry**

Mirror the structure of the `KColoring -> QUBO` writeup, but tailor it to the CVP construction:
- explain the bounded coefficient encoding
- show `G = A^T A` and `h = A^T t`
- explain how the quadratic form becomes a QUBO over the encoding bits
- make the solution-extraction step explicit

For the example block, use the exported canonical fixture values and walk through:
- the two bounded CVP variables
- the number of QUBO bits
- the concrete encoding of `(1,1)`
- the resulting QUBO objective value and how it maps back to the CVP distance

- [ ] **Step 3: Build the paper and fix any export/notation mismatches**

Run: `make paper`

Expected: PASS with no missing-example or uncovered-rule errors.

- [ ] **Step 4: Commit the paper slice**

```bash
git add docs/paper/reductions.typ
git commit -m "docs(paper): document ClosestVectorProblem to QUBO"
```

### Task 5: Final verification and handoff

**Files:**
- Modify: `docs/plans/2026-03-19-closestvectorproblem-qubo.md` (delete after execution in the issue-to-pr workflow)

- [ ] **Step 1: Run the final verification bundle**

Run:

```bash
make test
make clippy
make paper
```

Expected: PASS.

- [ ] **Step 2: Prepare the implementation summary for the PR comment**

Summarize:
- the new rule module
- the CVP helper added for overhead/encoding
- the new tests and canonical example
- the paper entry
- the implementation choice for bounded encodings (exact in-range bit basis rather than a separate invalid-range penalty)

- [ ] **Step 3: Remove this plan file after implementation**

```bash
git rm docs/plans/2026-03-19-closestvectorproblem-qubo.md
git commit -m "chore: remove plan file after implementation"
```
