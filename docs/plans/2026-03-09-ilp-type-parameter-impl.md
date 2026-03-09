# ILP Type Parameter Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace `ILP` with `ILP<V>` where `V ∈ {bool, i32}` determines the variable domain, removing the `bounds` field.

**Architecture:** Define a sealed `VariableDomain` trait implemented for `bool` (binary, dims=2) and `i32` (integer, dims=2^31-1). The ILP struct becomes generic over `V: VariableDomain` with `PhantomData<V>`. All existing reductions target `ILP<bool>` except Factoring which targets `ILP<i32>`. A trivial cast reduction connects `ILP<bool>` → `ILP<i32>`.

**Tech Stack:** Rust, serde, inventory, proc-macro (`#[reduction]`), `variant_params!`, `declare_variants!`

**Design doc:** `docs/plans/2026-03-09-ilp-type-parameter-design.md`

---

### Task 1: Define `VariableDomain` trait and refactor `ILP<V>` struct

**Files:**
- Modify: `src/models/algebraic/ilp.rs`

**Step 1: Add `VariableDomain` trait after the `VarBounds` impl block (after line 100)**

```rust
/// Sealed trait for ILP variable domains.
///
/// `bool` = binary variables (0 or 1), `i32` = non-negative integers (0..2^31-1).
pub trait VariableDomain: 'static + Clone + std::fmt::Debug + Send + Sync {
    /// Number of possible values per variable (used by `dims()`).
    const DIMS_PER_VAR: usize;
    /// Name for the variant system (e.g., "bool", "i32").
    const NAME: &'static str;
}

impl VariableDomain for bool {
    const DIMS_PER_VAR: usize = 2;
    const NAME: &'static str = "bool";
}

impl VariableDomain for i32 {
    const DIMS_PER_VAR: usize = i32::MAX as usize;
    const NAME: &'static str = "i32";
}
```

**Step 2: Refactor `ILP` struct to `ILP<V>`**

Replace the struct definition (lines 213-225) with:

```rust
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ILP<V: VariableDomain = bool> {
    /// Number of variables.
    pub num_vars: usize,
    /// Linear constraints.
    pub constraints: Vec<LinearConstraint>,
    /// Sparse objective coefficients: (var_index, coefficient).
    pub objective: Vec<(usize, f64)>,
    /// Optimization direction.
    pub sense: ObjectiveSense,
    #[serde(skip)]
    _marker: PhantomData<V>,
}
```

Note: Default type parameter `= bool` preserves backward compatibility during migration.

**Step 3: Update constructors**

Replace `ILP::new()` (lines 239-254) and `ILP::binary()` (lines 259-267) and `ILP::empty()` (lines 270-278):

```rust
impl<V: VariableDomain> ILP<V> {
    /// Create a new ILP problem.
    pub fn new(
        num_vars: usize,
        constraints: Vec<LinearConstraint>,
        objective: Vec<(usize, f64)>,
        sense: ObjectiveSense,
    ) -> Self {
        Self {
            num_vars,
            constraints,
            objective,
            sense,
            _marker: PhantomData,
        }
    }

    /// Create an empty ILP with no variables.
    pub fn empty() -> Self {
        Self::new(0, vec![], vec![], ObjectiveSense::Minimize)
    }
}
```

**Step 4: Update helper methods**

Remove `bounds_satisfied()` and update `config_to_values()`, `is_feasible()`:

```rust
impl<V: VariableDomain> ILP<V> {
    /// Evaluate the objective function for given variable values.
    pub fn evaluate_objective(&self, values: &[i64]) -> f64 {
        self.objective
            .iter()
            .map(|&(var, coef)| coef * values.get(var).copied().unwrap_or(0) as f64)
            .sum()
    }

    /// Check if all constraints are satisfied for given variable values.
    pub fn constraints_satisfied(&self, values: &[i64]) -> bool {
        self.constraints.iter().all(|c| c.is_satisfied(values))
    }

    /// Check if a solution is feasible (satisfies constraints).
    pub fn is_feasible(&self, values: &[i64]) -> bool {
        values.len() == self.num_vars && self.constraints_satisfied(values)
    }

    /// Convert a configuration (Vec<usize>) to integer values (Vec<i64>).
    /// For bool: config 0→0, 1→1. For i32: config index = value.
    fn config_to_values(&self, config: &[usize]) -> Vec<i64> {
        config.iter().map(|&c| c as i64).collect()
    }

    /// Get the number of variables.
    pub fn num_variables(&self) -> usize {
        self.num_vars
    }

    /// Get the number of variables.
    pub fn num_vars(&self) -> usize {
        self.num_variables()
    }

    /// Get the number of constraints.
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }
}
```

**Step 5: Update `Problem` and `OptimizationProblem` impls**

```rust
impl<V: VariableDomain> Problem for ILP<V> {
    const NAME: &'static str = "ILP";
    type Metric = SolutionSize<f64>;

    fn dims(&self) -> Vec<usize> {
        vec![V::DIMS_PER_VAR; self.num_vars]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<f64> {
        let values = self.config_to_values(config);
        if !self.is_feasible(&values) {
            return SolutionSize::Invalid;
        }
        SolutionSize::Valid(self.evaluate_objective(&values))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("variable", V::NAME)]
    }
}

impl<V: VariableDomain> OptimizationProblem for ILP<V> {
    type Value = f64;

    fn direction(&self) -> Direction {
        match self.sense {
            ObjectiveSense::Maximize => Direction::Maximize,
            ObjectiveSense::Minimize => Direction::Minimize,
        }
    }
}
```

**Step 6: Update `declare_variants!` and `ProblemSchemaEntry`**

```rust
crate::declare_variants! {
    ILP<bool> => "2^num_vars",
    ILP<i32> => "num_vars^num_vars",
}
```

Update the `inventory::submit!` block to remove the `bounds` field entry.

**Step 7: Run `make check`**

Expected: compilation errors in all files that reference `ILP` (reductions, solver, CLI, tests). This is expected — we fix them in subsequent tasks.

**Step 8: Commit**

```
feat: refactor ILP to ILP<V> with VariableDomain trait

Remove bounds field. V=bool for binary (dims=2), V=i32 for
general integer (dims=2^31-1).
```

---

### Task 2: Update X → ILP reductions to target `ILP<bool>`

**Files:**
- Modify: `src/rules/maximumindependentset_ilp.rs`
- Modify: `src/rules/maximumclique_ilp.rs`
- Modify: `src/rules/maximummatching_ilp.rs`
- Modify: `src/rules/minimumdominatingset_ilp.rs`
- Modify: `src/rules/minimumsetcovering_ilp.rs`
- Modify: `src/rules/coloring_ilp.rs`
- Modify: `src/rules/travelingsalesman_ilp.rs`
- Modify: `src/rules/circuit_ilp.rs`
- Modify: `src/rules/qubo_ilp.rs`

**Step 1: Apply the same pattern to all 9 files**

For each file:
1. Change `use ... VarBounds` → remove VarBounds import (no longer needed)
2. Change `ILP` → `ILP<bool>` in `ReductionResult` type aliases and `ReduceTo` impls
3. Remove `let bounds = vec![VarBounds::binary(); ...]` lines
4. Change `ILP::new(num_vars, bounds, constraints, ...)` → `ILP::<bool>::new(num_vars, constraints, ...)`

Example diff for `maximumindependentset_ilp.rs`:

```diff
-use crate::models::algebraic::{LinearConstraint, ObjectiveSense, VarBounds, ILP, QUBO};
+use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};

 pub struct ReductionISToILP {
-    target: ILP,
+    target: ILP<bool>,
 }

 impl ReductionResult for ReductionISToILP {
     type Source = MaximumIndependentSet<SimpleGraph, i32>;
-    type Target = ILP;
+    type Target = ILP<bool>;

-impl ReduceTo<ILP> for MaximumIndependentSet<SimpleGraph, i32> {
+impl ReduceTo<ILP<bool>> for MaximumIndependentSet<SimpleGraph, i32> {

     fn reduce_to(&self) -> Self::Result {
         let num_vars = self.graph().num_vertices();
-        let bounds = vec![VarBounds::binary(); num_vars];
         // ... constraints and objective unchanged ...
-        let target = ILP::new(num_vars, bounds, constraints, objective, ObjectiveSense::Maximize);
+        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Maximize);
```

Repeat this pattern for all 9 files. The only differences are the struct/type names and which fields they construct.

**Step 2: Run `make clippy`**

Expected: should compile (reduction files fixed). Tests/solver/CLI still broken.

**Step 3: Commit**

```
refactor: update 9 reductions to target ILP<bool>

Remove VarBounds::binary() construction from all reductions.
```

---

### Task 3: Update Factoring → ILP to target `ILP<i32>`

**Files:**
- Modify: `src/rules/factoring_ilp.rs`

**Step 1: Change target type and convert carry bounds to constraints**

1. Change `use ... VarBounds` → remove import
2. Change `ReduceTo<ILP>` → `ReduceTo<ILP<i32>>`
3. Change `ReductionResult::Target` to `ILP<i32>`
4. Remove the `bounds` construction (lines 132-145)
5. Add carry upper-bound constraints instead:

```rust
// Instead of VarBounds::bounded(0, carry_upper) for carries:
// Add explicit constraints: 0 <= c_k <= carry_upper
let carry_upper = min(m, n) as f64;
for k in 0..num_carries {
    let cv = carry_var(k);
    // c_k >= 0
    constraints.push(LinearConstraint::ge(vec![(cv, 1.0)], 0.0));
    // c_k <= min(m, n)
    constraints.push(LinearConstraint::le(vec![(cv, 1.0)], carry_upper));
}
```

6. Update overhead to account for additional constraints:

```rust
#[reduction(overhead = {
    num_vars = "2 * num_bits_first + 2 * num_bits_second + num_bits_first * num_bits_second",
    num_constraints = "3 * num_bits_first * num_bits_second + 3 * num_bits_first + 3 * num_bits_second + 1",
})]
```

Note: The `+ 2 * num_carries` for bound constraints. Since `num_carries = max(m+n, target_bits)` which is at most `m + n`, and the overhead expression uses `num_bits_first + num_bits_second` as proxy — verify the exact count after implementation.

7. Change constructor call: `ILP::new(num_vars, bounds, constraints, ...)` → `ILP::<i32>::new(num_vars, constraints, ...)`

**Step 2: Verify binary variables still work**

For `ILP<i32>`, binary variables (p_i, q_j, z_ij) are implicitly 0/1 because of the McCormick constraints (`z_ij ≤ p_i`, `z_ij ≤ q_j`, `z_ij ≥ p_i + q_j - 1`) combined with the bit equations. But to enforce that p_i and q_j are actually binary (not just non-negative integers), add:

```rust
// p_i ∈ {0,1}: add constraints p_i <= 1
for i in 0..m {
    constraints.push(LinearConstraint::le(vec![(p_var(i), 1.0)], 1.0));
}
// q_j ∈ {0,1}: add constraints q_j <= 1
for j in 0..n {
    constraints.push(LinearConstraint::le(vec![(q_var(j), 1.0)], 1.0));
}
// z_ij upper bound is already enforced by z_ij <= p_i and z_ij <= q_j
```

**Step 3: Run `make test` on factoring tests**

```bash
cargo test test_factoring --no-fail-fast
```

**Step 4: Commit**

```
refactor: update Factoring → ILP<i32> with carry bounds as constraints
```

---

### Task 4: Update ILP<bool> → QUBO reduction

**Files:**
- Modify: `src/rules/ilp_qubo.rs`

**Step 1: Change source type and remove runtime assert**

```diff
-impl ReductionResult for ReductionILPToQUBO {
-    type Source = ILP;
+impl ReductionResult for ReductionILPToQUBO {
+    type Source = ILP<bool>;

-impl ReduceTo<QUBO<f64>> for ILP {
+impl ReduceTo<QUBO<f64>> for ILP<bool> {
     fn reduce_to(&self) -> Self::Result {
         let n = self.num_vars;
-        // Verify all variables are binary
-        for (i, b) in self.bounds.iter().enumerate() { ... }
+        // All variables are binary by type — no runtime check needed.
```

**Step 2: Update overhead expression**

The slack bits per inequality constraint are bounded by `ceil(log2(n + 1))` for binary variables (worst-case: all n variables with coefficient 1). Total slack ≤ `num_constraints * ceil(log2(num_vars + 1))`. As an upper bound:

```rust
#[reduction(overhead = {
    num_vars = "num_vars + num_constraints * num_vars",
})]
```

**Step 3: Update slack computation**

With binary variables, the slack range for `Ax <= b` is `b - min_lhs` where `min_lhs = Σ min(0, a_i)`. Since variables are binary, coefficients determine the range. The existing logic is correct — it just no longer needs the bounds check.

Remove the line `for (i, b) in self.bounds.iter().enumerate()` block (lines 46-54).

**Step 4: Commit**

```
refactor: ILP<bool> → QUBO with compile-time binary guarantee
```

---

### Task 5: Add `ILP<bool>` → `ILP<i32>` cast reduction

**Files:**
- Create: `src/rules/ilp_bool_ilp_i32.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write the reduction**

```rust
//! Natural embedding of binary ILP into general integer ILP.
//!
//! Every binary (0-1) variable is a valid non-negative integer variable.
//! The constraints carry over unchanged. Additional upper-bound constraints
//! (x_i <= 1) are added to preserve binary semantics.

use crate::models::algebraic::{LinearConstraint, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionBinaryILPToIntILP {
    target: ILP<i32>,
}

impl ReductionResult for ReductionBinaryILPToIntILP {
    type Source = ILP<bool>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }
}

#[reduction(overhead = {
    num_vars = "num_vars",
    num_constraints = "num_constraints + num_vars",
})]
impl ReduceTo<ILP<i32>> for ILP<bool> {
    type Result = ReductionBinaryILPToIntILP;

    fn reduce_to(&self) -> Self::Result {
        let mut constraints = self.constraints.clone();
        // Add x_i <= 1 for each variable to preserve binary domain
        for i in 0..self.num_vars {
            constraints.push(LinearConstraint::le(vec![(i, 1.0)], 1.0));
        }
        ReductionBinaryILPToIntILP {
            target: ILP::<i32>::new(
                self.num_vars,
                constraints,
                self.objective.clone(),
                self.sense,
            ),
        }
    }
}
```

**Step 2: Register in `src/rules/mod.rs`**

Add `mod ilp_bool_ilp_i32;` to the module list.

**Step 3: Commit**

```
feat: add ILP<bool> → ILP<i32> cast reduction
```

---

### Task 6: Update ILP solver

**Files:**
- Modify: `src/solvers/ilp/solver.rs`
- Modify: `src/solvers/ilp/mod.rs`

**Step 1: Refactor `solve()` to be generic over `V`**

```rust
use crate::models::algebraic::{Comparison, ObjectiveSense, VariableDomain, ILP};

impl ILPSolver {
    pub fn solve<V: VariableDomain>(&self, problem: &ILP<V>) -> Option<Vec<usize>> {
        let n = problem.num_vars;
        if n == 0 {
            return Some(vec![]);
        }

        let mut vars_builder = ProblemVariables::new();
        let vars: Vec<Variable> = (0..n)
            .map(|_| {
                let mut v = variable().integer();
                // Set bounds based on variable domain
                v = v.min(0.0);
                v = v.max((V::DIMS_PER_VAR - 1) as f64);
                vars_builder.add(v)
            })
            .collect();

        // ... objective and constraint building unchanged ...

        // Solution extraction: config index = value (no lower bound offset)
        let result: Vec<usize> = vars
            .iter()
            .map(|v| {
                let val = solution.value(*v);
                val.round().max(0.0) as usize
            })
            .collect();

        Some(result)
    }

    pub fn solve_reduced<P>(&self, problem: &P) -> Option<Vec<usize>>
    where
        P: ReduceTo<ILP<bool>>,
    {
        let reduction = problem.reduce_to();
        let ilp_solution = self.solve(reduction.target_problem())?;
        Some(reduction.extract_solution(&ilp_solution))
    }
}
```

**Step 2: Update doc examples in solver.rs and mod.rs**

Replace `ILP::binary(...)` with `ILP::<bool>::new(...)` (without bounds param).

**Step 3: Commit**

```
refactor: update ILP solver for generic ILP<V>
```

---

### Task 7: Update module exports and CLI dispatch

**Files:**
- Modify: `src/models/algebraic/mod.rs` (line 16)
- Modify: `src/models/mod.rs` (line 12)
- Modify: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/commands/solve.rs`
- Modify: `problemreductions-cli/src/commands/create.rs`
- Modify: `problemreductions-cli/src/mcp/tools.rs`

**Step 1: Update re-exports**

`src/models/algebraic/mod.rs`:
```rust
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VarBounds, VariableDomain, ILP};
```

`src/models/mod.rs`:
```rust
pub use algebraic::{ClosestVectorProblem, BMF, ILP, QUBO};
```

**Step 2: Update CLI dispatch**

In `dispatch.rs`, change `ILP` references to `ILP<bool>` (the default variant for CLI):
```rust
"ILP" => deser_opt::<ILP<bool>>(data),
// ...
"ILP" => try_ser::<ILP<bool>>(any),
```

Similarly update `solve.rs`, `create.rs`, and `mcp/tools.rs`. Since `ILP` defaults to `ILP<bool>`, many references may work without change — check each.

**Step 3: Run `make cli-demo`**

Verify CLI still works end-to-end.

**Step 4: Commit**

```
refactor: update exports and CLI for ILP<V>
```

---

### Task 8: Update all tests

**Files:**
- Modify: `src/unit_tests/models/algebraic/ilp.rs`
- Modify: `src/unit_tests/rules/ilp_qubo.rs`
- Modify: `src/unit_tests/rules/qubo_ilp.rs`
- Modify: `src/unit_tests/rules/maximumindependentset_ilp.rs`
- Modify: `src/unit_tests/rules/maximumclique_ilp.rs`
- Modify: `src/unit_tests/rules/maximummatching_ilp.rs`
- Modify: `src/unit_tests/rules/minimumdominatingset_ilp.rs`
- Modify: `src/unit_tests/rules/minimumsetcovering_ilp.rs`
- Modify: `src/unit_tests/rules/coloring_ilp.rs`
- Modify: `src/unit_tests/rules/travelingsalesman_ilp.rs`
- Modify: `src/unit_tests/rules/factoring_ilp.rs`
- Modify: `src/unit_tests/solvers/ilp/solver.rs`
- Modify: `src/unit_tests/problem_size.rs`
- Modify: `src/unit_tests/unitdiskmapping_algorithms/common.rs`
- Modify: `src/unit_tests/unitdiskmapping_algorithms/weighted.rs`
- Modify: `src/unit_tests/reduction_graph.rs`
- Modify: `tests/suites/reductions.rs`
- Modify: `examples/reduction_ilp_to_qubo.rs`

**Step 1: Bulk replacement across all test files**

For each file:
1. `ILP::binary(n, constraints, objective, sense)` → `ILP::<bool>::new(n, constraints, objective, sense)`
2. `ILP::new(n, bounds, constraints, objective, sense)` → `ILP::<bool>::new(n, constraints, objective, sense)` (remove bounds arg)
3. Remove `VarBounds` imports where no longer needed
4. Remove assertions like `for bound in &ilp.bounds { assert_eq!(*bound, VarBounds::binary()); }`
5. Remove `assert!(ilp.bounds_satisfied(...))` calls (method removed)
6. Update `ILP` type annotations to `ILP<bool>` where needed for type inference

**Step 2: Update ILP model tests specifically**

`src/unit_tests/models/algebraic/ilp.rs`:
- Remove `test_ilp_bounds_satisfied` (method no longer exists)
- Remove `test_ilp_new_with_general_bounds` (ILP no longer has bounds)
- Update `test_ilp_new_basic` to use new constructor
- Update serialization tests (no `bounds` field in JSON)
- Keep constraint satisfaction tests
- Add new test for `ILP<i32>`:

```rust
#[test]
fn test_ilp_i32_dims() {
    let ilp = ILP::<i32>::new(3, vec![], vec![], ObjectiveSense::Minimize);
    assert_eq!(ilp.dims(), vec![i32::MAX as usize; 3]);
}
```

**Step 3: Update reduction graph test**

`src/unit_tests/reduction_graph.rs`: The graph now has two ILP variant nodes. Update node count and edge count expectations. Add check for ILP<bool> → ILP<i32> edge.

**Step 4: Update analysis allow-list**

`src/unit_tests/rules/analysis.rs`: The dominated-rule allow-list may change because:
- ILP → QUBO is now `ILP<bool>` → QUBO, and overhead is more accurate
- New ILP<bool> → ILP<i32> edge exists
- Factoring → ILP is now Factoring → ILP<i32>

Re-run the test with `--nocapture` to discover the new allow-list, then update.

**Step 5: Run `make check`**

Expected: all tests pass, clippy clean, fmt clean.

**Step 6: Commit**

```
test: update all tests for ILP<V> refactor
```

---

### Task 9: Remove UNTRUSTED_EDGES and update analysis

**Files:**
- Modify: `src/rules/analysis.rs`

**Step 1: Update UNTRUSTED_EDGES**

With `ILP<bool>` → QUBO having an accurate overhead, remove the entry:

```rust
const UNTRUSTED_EDGES: &[(&str, &str)] = &[];
```

**Step 2: Re-run dominated rule analysis**

```bash
cargo test test_find_dominated_rules_returns_known_set -- --nocapture
```

Update the allow-list in `src/unit_tests/rules/analysis.rs` based on new results. Some previously-unknown comparisons may become decidable now that ILP → QUBO paths are trusted.

**Step 3: Commit**

```
feat: remove ILP→QUBO from untrusted edges

ILP<bool>→QUBO overhead is now accurate (no slack ambiguity).
```

---

### Task 10: Update paper and generated artifacts

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Update ILP problem definition**

Update the `problem-def("ILP")` to mention the type parameter:
- `ILP<bool>`: binary integer linear programming
- `ILP<i32>`: general integer linear programming
- Add to `display-name` dict if variants need distinct display names

**Step 2: Regenerate artifacts**

```bash
make export-schemas    # Regenerate problem_schemas.json
make examples          # Regenerate example JSON
make doc               # Rebuild mdBook (exports reduction graph)
```

**Step 3: Verify paper compiles**

```bash
make paper
```

**Step 4: Commit**

```
docs: update paper and generated artifacts for ILP<V>
```

---

### Task 11: Final verification

**Step 1: Full check**

```bash
make check        # fmt + clippy + test
make cli-demo     # CLI end-to-end
make mcp-test     # MCP server tests
```

**Step 2: Coverage check**

```bash
make coverage
```

Verify >95% coverage on new code.

**Step 3: Commit any remaining fixes**

---

## Summary of Changes

| Category | Files | Change |
|----------|-------|--------|
| Core type | `ilp.rs` | `ILP` → `ILP<V>`, add `VariableDomain` trait, remove `bounds` |
| 9 reductions → ILP | `*_ilp.rs`, `qubo_ilp.rs` | Target `ILP<bool>`, remove bounds construction |
| Factoring → ILP | `factoring_ilp.rs` | Target `ILP<i32>`, carry bounds as constraints |
| ILP → QUBO | `ilp_qubo.rs` | Source `ILP<bool>`, remove runtime assert, fix overhead |
| New cast | `ilp_bool_ilp_i32.rs` | `ILP<bool>` → `ILP<i32>` |
| Solver | `solver.rs` | Generic over `V`, remove bounds references |
| CLI | `dispatch.rs`, etc. | Type annotations for `ILP<bool>` |
| Tests | 18+ test files | Remove bounds assertions, update constructors |
| Analysis | `analysis.rs` | Remove UNTRUSTED_EDGES entry |
| Paper | `reductions.typ` | Update problem definition |
