# Problem-to-QUBO Reductions Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement 7 reductions from NP-hard problems to QUBO, with tests, examples, and paper documentation (Issue #18).

**Architecture:** Each reduction creates a `QUBO<f64>` matrix encoding the source problem's objective + constraints as penalty terms. All reductions follow the existing pattern in `src/rules/spinglass_qubo.rs`: a result struct implementing `ReductionResult`, a `ReduceTo` impl with `#[reduction]` macro, unit tests via `#[path]`, and integration tests in `tests/suites/reductions.rs`.

**Tech Stack:** Rust, `#[reduction]` proc macro, `inventory` for registration, `BruteForce` solver for tests. Ground truth JSON in `tests/data/qubo/` (already generated via PR #29).

**Branch:** `issue-18-qubo-reductions` (already exists, PR #29)

---

### Task 1: IndependentSet → QUBO

Maximize weighted IS = minimize `-Σ w_i·x_i + P·Σ_{(i,j)∈E} x_i·x_j` where `P > Σ w_i`.

**Files:**
- Create: `src/rules/independentset_qubo.rs`
- Create: `src/unit_tests/rules/independentset_qubo.rs`
- Modify: `src/rules/mod.rs` — add `mod independentset_qubo;` + `pub use`

**Step 1: Write unit test**

File: `src/unit_tests/rules/independentset_qubo.rs`

```rust
use super::*;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_independentset_to_qubo_closed_loop() {
    // Path graph: 0-1-2-3 (4 vertices, 3 edges)
    // Maximum IS = {0, 2} or {1, 3} (size 2)
    let is = IndependentSet::<SimpleGraph, Unweighted>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.solution_size(&extracted).is_valid);
        // IS of size 2
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 2);
    }
}

#[test]
fn test_independentset_to_qubo_triangle() {
    // Triangle: 0-1-2 (complete graph K3)
    // Maximum IS = any single vertex (size 1)
    let is = IndependentSet::<SimpleGraph, Unweighted>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&is);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_best(qubo);

    for sol in &qubo_solutions {
        let extracted = reduction.extract_solution(sol);
        assert!(is.solution_size(&extracted).is_valid);
        assert_eq!(extracted.iter().filter(|&&x| x == 1).count(), 1);
    }
}
```

**Step 2: Run test, verify it fails**

Run: `cargo test --all-features test_independentset_to_qubo`
Expected: compilation error (module not found)

**Step 3: Write reduction implementation**

File: `src/rules/independentset_qubo.rs`

```rust
//! Reduction from IndependentSet to QUBO.
//!
//! Maximize Σ w_i·x_i s.t. x_i·x_j = 0 for (i,j) ∈ E
//! = Minimize -Σ w_i·x_i + P·Σ_{(i,j)∈E} x_i·x_j
//!
//! Q[i][i] = -w_i, Q[i][j] = P for edges. P = 1 + Σ w_i.

use crate::models::graph::IndependentSet;
use crate::models::optimization::QUBO;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::{ProblemSize, Unweighted};

#[derive(Debug, Clone)]
pub struct ReductionISToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionISToQUBO {
    type Source = IndependentSet<SimpleGraph, Unweighted>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target { &self.target }
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
    fn source_size(&self) -> ProblemSize { self.source_size.clone() }
    fn target_size(&self) -> ProblemSize { self.target.problem_size() }
}

#[reduction(
    source_graph = "SimpleGraph",
    overhead = { ReductionOverhead::new(vec![("num_vars", poly!(num_vertices))]) }
)]
impl ReduceTo<QUBO<f64>> for IndependentSet<SimpleGraph, Unweighted> {
    type Result = ReductionISToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();
        let penalty = 1.0 + n as f64; // P > sum of unit weights

        let mut matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            matrix[i][i] = -1.0; // -w_i (unit weight)
        }
        for (u, v) in &edges {
            let (i, j) = if u < v { (*u, *v) } else { (*v, *u) };
            matrix[i][j] += penalty;
        }

        ReductionISToQUBO {
            target: QUBO::from_matrix(matrix),
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/independentset_qubo.rs"]
mod tests;
```

**Step 4: Register in `src/rules/mod.rs`**

Add after `mod spinglass_qubo;`:
```rust
mod independentset_qubo;
```

Add after `pub use spinglass_qubo::...`:
```rust
pub use independentset_qubo::ReductionISToQUBO;
```

**Step 5: Run tests**

Run: `cargo test --all-features test_independentset_to_qubo`
Expected: PASS

**Step 6: Run clippy + full test suite**

Run: `make test clippy`
Expected: all pass, no warnings

**Step 7: Commit**

```bash
git add src/rules/independentset_qubo.rs src/unit_tests/rules/independentset_qubo.rs src/rules/mod.rs
git commit -m "feat: add IndependentSet → QUBO reduction"
```

---

### Task 2: VertexCovering → QUBO

Minimize `Σ w_i·x_i + P·Σ_{(i,j)∈E} (1-x_i)(1-x_j)`. Expanding: `Q[i][i] = w_i - P·deg(i)`, `Q[i][j] = P`.

**Files:**
- Create: `src/rules/vertexcovering_qubo.rs`
- Create: `src/unit_tests/rules/vertexcovering_qubo.rs`
- Modify: `src/rules/mod.rs`

Same pattern as Task 1. Key differences:
- VC minimizes (same as QUBO), so no sign flip on objective
- Penalty enforces: every edge has at least one endpoint selected
- `Q[i][i] = 1.0 - penalty * degree(i)`, `Q[i][j] = penalty` for edges
- Penalty `P = 1 + n` (unit weights)
- Test: cycle graph C4 (4 vertices, 4 edges) → min VC = 2 vertices

**Step 1: Write test** (same structure as Task 1)
**Step 2: Verify fails**
**Step 3: Implement** — struct `ReductionVCToQUBO`, same boilerplate
**Step 4: Register in mod.rs**
**Step 5-6: Test + clippy**
**Step 7: Commit** `"feat: add VertexCovering → QUBO reduction"`

---

### Task 3: MaxCut → QUBO

Maximize cut = Σ_{(i,j)∈E} w_ij·(x_i⊕x_j). Minimize negative: `Q[i][i] = -Σ_j w_ij`, `Q[i][j] = 2·w_ij` (upper triangular).

Note: MaxCut edges carry weights. Use `self.edges()` which returns `Vec<(usize, usize, W)>`.

**Files:**
- Create: `src/rules/maxcut_qubo.rs`
- Create: `src/unit_tests/rules/maxcut_qubo.rs`
- Modify: `src/rules/mod.rs`

Key: MaxCut is `MaxCut<SimpleGraph, W>` with edge weights. For unweighted, use `MaxCut::unweighted(n, edges)`.

- `Q[i][j] = 2·w_ij` for i < j (upper triangular; the `w_ij(x_i + x_j - 2x_ix_j)` formula)
- `Q[i][i] = -Σ_{j:(i,j)∈E} w_ij`
- Test: cycle C4 → max cut = 4 (all edges cut by bipartition)
- No penalty needed — MaxCut is unconstrained

**Step 1-7:** Same flow. Commit: `"feat: add MaxCut → QUBO reduction"`

---

### Task 4: Coloring (KColoring) → QUBO

One-hot encoding: `x_{v,c} = 1` iff vertex v gets color c. QUBO index: `v*K + c`.

- One-hot penalty: `P₁·Σ_v (1 - Σ_c x_{v,c})²`
- Edge penalty: `P₂·Σ_{(u,v)∈E} Σ_c x_{u,c}·x_{v,c}`
- QUBO has `n·K` variables

**Special:** `KColoring<const K: usize, G, W>` uses const generic. For the reduction, we implement for a specific K (e.g., `K=3`). Or better: implement for generic K using the existing pattern.

Actually, looking at `coloring_ilp.rs`, there are two reductions:
- `ReductionColoringToILP` for `Coloring<SimpleGraph, W>` (deprecated Coloring type?)
- `ReductionKColoringToILP<const K: usize, W>` for `KColoring<K, SimpleGraph, W>`

We should implement for `KColoring<K, SimpleGraph, Unweighted>`. The `extract_solution` decodes one-hot: for each vertex, find which color bit is 1.

The struct needs to store `num_vertices` and `K` for extraction.

**Files:**
- Create: `src/rules/coloring_qubo.rs`
- Create: `src/unit_tests/rules/coloring_qubo.rs`
- Modify: `src/rules/mod.rs`

**Test:** Triangle K3, 3 colors → exactly 6 valid colorings (3! permutations).

**Step 1-7:** Same flow. Commit: `"feat: add KColoring → QUBO reduction"`

---

### Task 5: SetPacking → QUBO

Same structure as IS on intersection graph: `Q[i][i] = -w_i`, `Q[i][j] = P` for overlapping pairs.

Use `self.overlapping_pairs()` to get conflicting set pairs.

**Files:**
- Create: `src/rules/setpacking_qubo.rs`
- Create: `src/unit_tests/rules/setpacking_qubo.rs`
- Modify: `src/rules/mod.rs`

**Test:** 3 sets with some overlaps → verify max packing found.

**Step 1-7:** Same flow. Commit: `"feat: add SetPacking → QUBO reduction"`

---

### Task 6: KSatisfiability (K=2) → QUBO

Max-2-SAT penalty formulation. Each clause contributes to Q based on literal signs.

For clause `(l₁ ∨ l₂)` where `l = x` or `l = ¬x`:
- `(x_i ∨ x_j)`: penalty `(1-x_i)(1-x_j)` = `1 - x_i - x_j + x_ix_j`
- `(¬x_i ∨ x_j)`: penalty `x_i(1-x_j)` = `x_i - x_ix_j`
- `(x_i ∨ ¬x_j)`: penalty `(1-x_i)x_j` = `x_j - x_ix_j`
- `(¬x_i ∨ ¬x_j)`: penalty `x_ix_j`

CNFClause uses 1-indexed signed integers: positive = variable, negative = negated. E.g., `[1, -2]` = `(x₁ ∨ ¬x₂)`.

**Files:**
- Create: `src/rules/ksatisfiability_qubo.rs`
- Create: `src/unit_tests/rules/ksatisfiability_qubo.rs`
- Modify: `src/rules/mod.rs`

**Test:** 3 vars, 4 clauses → verify all clauses satisfied by extracted solution.

**Step 1-7:** Same flow. Commit: `"feat: add KSatisfiability(K=2) → QUBO reduction"`

---

### Task 7: ILP (binary) → QUBO

Binary ILP: `min c^T x s.t. Ax ≤ b`. Feature-gated behind `ilp`.

Formulation: `Q[i][i] += c_i` (objective) + `P·Σ_k (Σ_j a_{kj}·x_j - b_k)²` (constraint penalties).

Expanding the quadratic penalty for constraint k:
- `Q[i][j] += P·a_{ki}·a_{kj}` for i ≤ j
- `Q[i][i] += P·a_{ki}·(a_{ki} - 2·b_k)` (diagonal adjustment)

ILP fields are public: `self.constraints`, `self.objective`, `self.sense`, `self.bounds`, `self.num_vars`.

Only valid for binary ILP (all bounds = [0,1]). Should assert this.

For Maximize objectives, negate the objective coefficients (QUBO minimizes).

**Files:**
- Create: `src/rules/ilp_qubo.rs` (with `#[cfg(feature = "ilp")]`)
- Create: `src/unit_tests/rules/ilp_qubo.rs`
- Modify: `src/rules/mod.rs`

**Test:** Binary ILP with 3 vars, 2 constraints → verify feasible optimal found.

**Step 1-7:** Same flow. Commit: `"feat: add ILP (binary) → QUBO reduction"`

---

### Task 8: Integration Tests

Add integration tests in `tests/suites/reductions.rs` that load JSON ground truth from `tests/data/qubo/` and compare against Rust reductions.

**Files:**
- Modify: `tests/suites/reductions.rs`

For each reduction, add a module like:
```rust
mod is_qubo_reductions {
    use super::*;

    #[test]
    fn test_is_to_qubo_ground_truth() {
        // Load JSON, create source problem, reduce, verify QUBO matrix and optimal match
    }
}
```

**Commit:** `"test: add integration tests for QUBO reductions against ground truth"`

---

### Task 9: Example Program

Create `examples/qubo_reductions.rs` demonstrating all 7 reductions with practical stories.

**File:** Create `examples/qubo_reductions.rs`

Each demo:
1. Create a small practical instance (e.g., "Find the largest non-conflicting set of wireless towers")
2. Reduce to QUBO
3. Solve with BruteForce
4. Extract and explain the solution

Run: `cargo run --example qubo_reductions --features ilp`

**Commit:** `"docs: add QUBO reductions example program"`

---

### Task 10: Paper Documentation

Update `docs/paper/reductions.typ` with 7 new theorems.

**File:** Modify `docs/paper/reductions.typ`

For each reduction:
1. Add theorem in Section 3.1 (trivial reductions — these are standard penalty formulations)
2. Add proof with the QUBO formulation
3. Add Rust code example (from `examples/qubo_reductions.rs`)
4. Update summary table with overhead and reference

Also update `@def:qubo` to list new "Reduces from" links.

Run: `make export-graph && make paper`

**Commit:** `"docs: add QUBO reduction theorems and examples to paper"`

---

### Task 11: Final Verification

```bash
make test          # All tests pass
make clippy        # No warnings
make export-graph  # Reduction graph updated
make paper         # Paper compiles
make coverage      # >95% for new code
```

**Commit:** any final fixups

---

## Key Reference Files

| Purpose | Path |
|---------|------|
| Model pattern | `src/rules/spinglass_qubo.rs` |
| Test pattern | `src/unit_tests/rules/spinglass_qubo.rs` |
| Module registry | `src/rules/mod.rs` |
| Integration tests | `tests/suites/reductions.rs` |
| ILP feature gate | `src/rules/mod.rs:28-45` (example) |
| Ground truth JSON | `tests/data/qubo/*.json` |
| Paper | `docs/paper/reductions.typ` |
| IS model | `src/models/graph/independent_set.rs` |
| VC model | `src/models/graph/vertex_covering.rs` |
| MaxCut model | `src/models/graph/max_cut.rs` |
| KColoring model | `src/models/graph/kcoloring.rs` |
| SetPacking model | `src/models/set/set_packing.rs` |
| KSat model | `src/models/satisfiability/ksat.rs` |
| ILP model | `src/models/optimization/ilp.rs` |
