# ILP Reduction Paths Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add QUBO→ILP and CircuitSAT→ILP reductions so all problem types can reach ILP (fixes #83).

**Architecture:** Two independent reductions following existing patterns: McCormick linearization for QUBO→ILP, Tseitin-style gate encoding for CircuitSAT→ILP. Both are feature-gated behind `ilp-solver`.

**Tech Stack:** Rust, `#[reduction]` macro, `inventory` crate for registration, `ILP` model types from `src/models/optimization/ilp.rs`.

---

## Task 1: QUBO → ILP Reduction

**Files:**
- Create: `src/rules/qubo_ilp.rs`
- Modify: `src/rules/mod.rs` (add module declaration)
- Test: `src/unit_tests/rules/qubo_ilp.rs`

**Reference:** `src/rules/maximumindependentset_ilp.rs` for ILP reduction pattern, `src/rules/factoring_ilp.rs` for McCormick pattern.

### Step 1: Write the failing test

Create `src/unit_tests/rules/qubo_ilp.rs`:

```rust
use super::*;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

#[test]
fn test_qubo_to_ilp_closed_loop() {
    // QUBO: minimize 2*x0 - 3*x1 + x0*x1
    // Q = [[2, 1], [0, -3]]
    // x=0,0 -> 0, x=1,0 -> 2, x=0,1 -> -3, x=1,1 -> 0
    // Optimal: x = [0, 1] with obj = -3
    let qubo = QUBO::from_matrix(vec![vec![2.0, 1.0], vec![0.0, -3.0]]);
    let reduction = ReduceTo::<ILP>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(ilp);
    let best_source: HashSet<_> = solver.find_all_best(&qubo).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
}

#[test]
fn test_qubo_to_ilp_diagonal_only() {
    // No quadratic terms: minimize 3*x0 - 2*x1
    // Optimal: x = [0, 1] with obj = -2
    let qubo = QUBO::from_matrix(vec![vec![3.0, 0.0], vec![0.0, -2.0]]);
    let reduction = ReduceTo::<ILP>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    // No auxiliary variables when no off-diagonal terms
    assert_eq!(ilp.num_variables(), 2);
    assert!(ilp.constraints.is_empty());

    let solver = BruteForce::new();
    let best = solver.find_all_best(ilp);
    let extracted = reduction.extract_solution(&best[0]);
    assert_eq!(extracted, vec![0, 1]);
}

#[test]
fn test_qubo_to_ilp_3var() {
    // QUBO: minimize -x0 - x1 - x2 + 4*x0*x1 + 4*x1*x2
    // Penalty on adjacent pairs → optimal is [1, 0, 1]
    let qubo = QUBO::from_matrix(vec![
        vec![-1.0, 4.0, 0.0],
        vec![0.0, -1.0, 4.0],
        vec![0.0, 0.0, -1.0],
    ]);
    let reduction = ReduceTo::<ILP>::reduce_to(&qubo);
    let ilp = reduction.target_problem();

    // 3 original + 2 auxiliary (for two off-diagonal terms)
    assert_eq!(ilp.num_variables(), 5);
    // 3 constraints per auxiliary = 6
    assert_eq!(ilp.constraints.len(), 6);

    let solver = BruteForce::new();
    let best = solver.find_all_best(ilp);
    let extracted = reduction.extract_solution(&best[0]);
    assert_eq!(extracted, vec![1, 0, 1]);
}
```

### Step 2: Run test to verify it fails

Run: `cargo test --features ilp-solver test_qubo_to_ilp -- --nocapture 2>&1 | head -30`
Expected: Compilation error (module doesn't exist yet)

### Step 3: Write the reduction implementation

Create `src/rules/qubo_ilp.rs`:

```rust
//! Reduction from QUBO to ILP via McCormick linearization.
//!
//! QUBO minimizes x^T Q x where x ∈ {0,1}^n and Q is upper-triangular.
//!
//! ## Linearization
//! - Diagonal: Q_ii · x_i² = Q_ii · x_i (linear for binary x)
//! - Off-diagonal: For each non-zero Q_ij (i < j), introduce y_ij = x_i · x_j
//!   with McCormick constraints: y_ij ≤ x_i, y_ij ≤ x_j, y_ij ≥ x_i + x_j - 1
//!
//! ## Variables
//! - x_i ∈ {0,1} for i = 0..n-1 (original QUBO variables)
//! - y_k ∈ {0,1} for each non-zero off-diagonal Q_ij (auxiliary products)
//!
//! ## Objective
//! minimize Σ_i Q_ii · x_i + Σ_{i<j} Q_ij · y_{ij}

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP, QUBO};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing QUBO to ILP.
#[derive(Debug, Clone)]
pub struct ReductionQUBOToILP {
    target: ILP,
    num_original: usize,
}

impl ReductionResult for ReductionQUBOToILP {
    type Source = QUBO<f64>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_original].to_vec()
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_vars ^ 2)),
            ("num_constraints", poly!(num_vars ^ 2)),
        ])
    }
)]
impl ReduceTo<ILP> for QUBO<f64> {
    type Result = ReductionQUBOToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let matrix = self.matrix();

        // Collect non-zero off-diagonal entries (i < j)
        let mut off_diag: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let q_ij = matrix[i][j];
                if q_ij != 0.0 {
                    off_diag.push((i, j, q_ij));
                }
            }
        }

        let m = off_diag.len();
        let total_vars = n + m;

        // All variables are binary
        let bounds = vec![VarBounds::binary(); total_vars];

        // Objective: minimize Σ Q_ii · x_i + Σ Q_ij · y_k
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for i in 0..n {
            let q_ii = matrix[i][i];
            if q_ii != 0.0 {
                objective.push((i, q_ii));
            }
        }
        for (k, &(_, _, q_ij)) in off_diag.iter().enumerate() {
            objective.push((n + k, q_ij));
        }

        // McCormick constraints: 3 per auxiliary variable
        let mut constraints = Vec::with_capacity(3 * m);
        for (k, &(i, j, _)) in off_diag.iter().enumerate() {
            let y_k = n + k;
            // y_k ≤ x_i
            constraints.push(LinearConstraint::le(
                vec![(y_k, 1.0), (i, -1.0)],
                0.0,
            ));
            // y_k ≤ x_j
            constraints.push(LinearConstraint::le(
                vec![(y_k, 1.0), (j, -1.0)],
                0.0,
            ));
            // y_k ≥ x_i + x_j - 1
            constraints.push(LinearConstraint::ge(
                vec![(y_k, 1.0), (i, -1.0), (j, -1.0)],
                -1.0,
            ));
        }

        let target = ILP::new(total_vars, bounds, constraints, objective, ObjectiveSense::Minimize);
        ReductionQUBOToILP {
            target,
            num_original: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/qubo_ilp.rs"]
mod tests;
```

### Step 4: Register module in `src/rules/mod.rs`

Add after the existing `ilp_qubo` line:

```rust
#[cfg(feature = "ilp-solver")]
mod qubo_ilp;
```

### Step 5: Run tests to verify they pass

Run: `cargo test --features ilp-solver test_qubo_to_ilp -- --nocapture`
Expected: All 3 tests pass

### Step 6: Commit

```bash
git add src/rules/qubo_ilp.rs src/rules/mod.rs src/unit_tests/rules/qubo_ilp.rs
git commit -m "feat: add QUBO → ILP reduction via McCormick linearization

Closes the ILP path for QUBO, SpinGlass, and MaxCut (issue #83)."
```

---

## Task 2: CircuitSAT → ILP Reduction

**Files:**
- Create: `src/rules/circuit_ilp.rs`
- Modify: `src/rules/mod.rs` (add module declaration)
- Test: `src/unit_tests/rules/circuit_ilp.rs`

**Reference:** `src/rules/circuit_spinglass.rs` for expression tree walking, `src/rules/maximumindependentset_ilp.rs` for ILP reduction structure.

### Step 1: Write the failing test

Create `src/unit_tests/rules/circuit_ilp.rs`:

```rust
use super::*;
use crate::models::specialized::{Assignment, BooleanExpr, Circuit, CircuitSAT};
use crate::solvers::BruteForce;
use crate::traits::Problem;
use std::collections::HashSet;

#[test]
fn test_circuitsat_to_ilp_and_gate() {
    // c = x AND y, constrain c = true → only x=1, y=1 satisfies
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    let ilp = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(ilp);
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    assert!(!extracted.is_empty());
}

#[test]
fn test_circuitsat_to_ilp_or_gate() {
    // c = x OR y, constrain c = true → x=1,y=0 or x=0,y=1 or x=1,y=1
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::or(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);
    let ilp = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(ilp);
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
}

#[test]
fn test_circuitsat_to_ilp_xor_gate() {
    // c = x XOR y, constrain c = true → x=1,y=0 or x=0,y=1
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(reduction.target_problem());
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
    assert_eq!(extracted.len(), 2); // exactly x=1,y=0 and x=0,y=1
}

#[test]
fn test_circuitsat_to_ilp_nested() {
    // d = (x AND y) OR z, constrain d = true
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["d".to_string()],
        BooleanExpr::or(vec![
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
            BooleanExpr::var("z"),
        ]),
    )]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(reduction.target_problem());
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
}

#[test]
fn test_circuitsat_to_ilp_closed_loop() {
    // Multi-assignment circuit: a = x AND y, b = NOT a, constrain b = false
    // Satisfying: x=1, y=1 → a=true → b=false ✓
    //             x=0, y=0 → a=false → b=true ✗ (b must be false)
    // etc.
    let circuit = Circuit::new(vec![
        Assignment::new(
            vec!["a".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        ),
        Assignment::new(
            vec!["b".to_string()],
            BooleanExpr::not(BooleanExpr::var("a")),
        ),
    ]);
    let source = CircuitSAT::new(circuit);
    let reduction = ReduceTo::<ILP>::reduce_to(&source);

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(reduction.target_problem());
    let best_source: HashSet<_> = solver.find_all_satisfying(&source).into_iter().collect();

    let extracted: HashSet<_> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    assert!(extracted.is_subset(&best_source));
}
```

### Step 2: Run test to verify it fails

Run: `cargo test --features ilp-solver test_circuitsat_to_ilp -- --nocapture 2>&1 | head -30`
Expected: Compilation error

### Step 3: Write the reduction implementation

Create `src/rules/circuit_ilp.rs`:

```rust
//! Reduction from CircuitSAT to ILP via gate constraint encoding.
//!
//! Each boolean gate is encoded as linear constraints over binary variables.
//! The expression tree is flattened by introducing an auxiliary variable per
//! internal node (Tseitin-style).
//!
//! ## Gate Encodings (all variables binary)
//! - NOT(a) = c:           c + a = 1
//! - AND(a₁,...,aₖ) = c:  c ≤ aᵢ (∀i), c ≥ Σaᵢ - (k-1)
//! - OR(a₁,...,aₖ) = c:   c ≥ aᵢ (∀i), c ≤ Σaᵢ
//! - XOR(a, b) = c:        c ≤ a+b, c ≥ a-b, c ≥ b-a, c ≤ 2-a-b
//! - Const(v) = c:          c = v
//!
//! ## Objective
//! Trivial (minimize 0): any feasible ILP solution is a satisfying assignment.

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::models::specialized::{BooleanExpr, BooleanOp, CircuitSAT};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::HashMap;

/// Result of reducing CircuitSAT to ILP.
#[derive(Debug, Clone)]
pub struct ReductionCircuitToILP {
    target: ILP,
    source_variables: Vec<String>,
    variable_map: HashMap<String, usize>,
}

impl ReductionResult for ReductionCircuitToILP {
    type Source = CircuitSAT;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.source_variables
            .iter()
            .map(|name| target_solution[self.variable_map[name]])
            .collect()
    }
}

/// Builder that accumulates ILP variables and constraints.
struct ILPBuilder {
    num_vars: usize,
    constraints: Vec<LinearConstraint>,
    variable_map: HashMap<String, usize>,
}

impl ILPBuilder {
    fn new() -> Self {
        Self {
            num_vars: 0,
            constraints: Vec::new(),
            variable_map: HashMap::new(),
        }
    }

    /// Get or create a variable index for a named circuit variable.
    fn get_or_create_var(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.variable_map.get(name) {
            idx
        } else {
            let idx = self.num_vars;
            self.variable_map.insert(name.to_string(), idx);
            self.num_vars += 1;
            idx
        }
    }

    /// Allocate an anonymous auxiliary variable.
    fn alloc_aux(&mut self) -> usize {
        let idx = self.num_vars;
        self.num_vars += 1;
        idx
    }

    /// Recursively process a BooleanExpr, returning the ILP variable index
    /// that holds the expression's value.
    fn process_expr(&mut self, expr: &BooleanExpr) -> usize {
        match &expr.op {
            BooleanOp::Var(name) => self.get_or_create_var(name),
            BooleanOp::Const(value) => {
                let c = self.alloc_aux();
                let v = if *value { 1.0 } else { 0.0 };
                self.constraints.push(LinearConstraint::eq(vec![(c, 1.0)], v));
                c
            }
            BooleanOp::Not(inner) => {
                let a = self.process_expr(inner);
                let c = self.alloc_aux();
                // c + a = 1
                self.constraints
                    .push(LinearConstraint::eq(vec![(c, 1.0), (a, 1.0)], 1.0));
                c
            }
            BooleanOp::And(args) => {
                let inputs: Vec<usize> = args.iter().map(|a| self.process_expr(a)).collect();
                let c = self.alloc_aux();
                let k = inputs.len() as f64;
                // c ≤ a_i for all i
                for &a_i in &inputs {
                    self.constraints
                        .push(LinearConstraint::le(vec![(c, 1.0), (a_i, -1.0)], 0.0));
                }
                // c ≥ Σa_i - (k - 1)
                let mut terms: Vec<(usize, f64)> = vec![(c, 1.0)];
                for &a_i in &inputs {
                    terms.push((a_i, -1.0));
                }
                self.constraints
                    .push(LinearConstraint::ge(terms, -(k - 1.0)));
                c
            }
            BooleanOp::Or(args) => {
                let inputs: Vec<usize> = args.iter().map(|a| self.process_expr(a)).collect();
                let c = self.alloc_aux();
                // c ≥ a_i for all i
                for &a_i in &inputs {
                    self.constraints
                        .push(LinearConstraint::ge(vec![(c, 1.0), (a_i, -1.0)], 0.0));
                }
                // c ≤ Σa_i
                let mut terms: Vec<(usize, f64)> = vec![(c, 1.0)];
                for &a_i in &inputs {
                    terms.push((a_i, -1.0));
                }
                self.constraints.push(LinearConstraint::le(terms, 0.0));
                c
            }
            BooleanOp::Xor(args) => {
                // Chain pairwise: XOR(a1, a2, a3) = XOR(XOR(a1, a2), a3)
                let inputs: Vec<usize> = args.iter().map(|a| self.process_expr(a)).collect();
                assert!(!inputs.is_empty());
                let mut result = inputs[0];
                for &next in &inputs[1..] {
                    let c = self.alloc_aux();
                    let a = result;
                    let b = next;
                    // c ≤ a + b
                    self.constraints.push(LinearConstraint::le(
                        vec![(c, 1.0), (a, -1.0), (b, -1.0)],
                        0.0,
                    ));
                    // c ≥ a - b
                    self.constraints.push(LinearConstraint::ge(
                        vec![(c, 1.0), (a, -1.0), (b, 1.0)],
                        0.0,
                    ));
                    // c ≥ b - a
                    self.constraints.push(LinearConstraint::ge(
                        vec![(c, 1.0), (a, 1.0), (b, -1.0)],
                        0.0,
                    ));
                    // c ≤ 2 - a - b
                    self.constraints.push(LinearConstraint::le(
                        vec![(c, 1.0), (a, 1.0), (b, 1.0)],
                        2.0,
                    ));
                    result = c;
                }
                result
            }
        }
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_variables + num_assignments)),
            ("num_constraints", poly!(num_variables + num_assignments)),
        ])
    }
)]
impl ReduceTo<ILP> for CircuitSAT {
    type Result = ReductionCircuitToILP;

    fn reduce_to(&self) -> Self::Result {
        let mut builder = ILPBuilder::new();

        // Pre-register all circuit variables to preserve ordering
        for name in self.variable_names() {
            builder.get_or_create_var(name);
        }

        // Process each assignment
        for assignment in &self.circuit().assignments {
            let expr_var = builder.process_expr(&assignment.expr);
            // Constrain each output to equal the expression result
            for output_name in &assignment.outputs {
                let out_var = builder.get_or_create_var(output_name);
                if out_var != expr_var {
                    // out = expr_var
                    builder.constraints.push(LinearConstraint::eq(
                        vec![(out_var, 1.0), (expr_var, -1.0)],
                        0.0,
                    ));
                }
            }
        }

        let bounds = vec![VarBounds::binary(); builder.num_vars];
        // Trivial objective: minimize 0 (satisfaction problem)
        let objective = vec![];
        let target = ILP::new(
            builder.num_vars,
            bounds,
            builder.constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionCircuitToILP {
            target,
            source_variables: self.variable_names().to_vec(),
            variable_map: builder.variable_map,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/circuit_ilp.rs"]
mod tests;
```

### Step 4: Register module in `src/rules/mod.rs`

Add after the `coloring_ilp` line in the `#[cfg(feature = "ilp-solver")]` block:

```rust
#[cfg(feature = "ilp-solver")]
mod circuit_ilp;
```

### Step 5: Run tests to verify they pass

Run: `cargo test --features ilp-solver test_circuitsat_to_ilp -- --nocapture`
Expected: All 5 tests pass

### Step 6: Commit

```bash
git add src/rules/circuit_ilp.rs src/rules/mod.rs src/unit_tests/rules/circuit_ilp.rs
git commit -m "feat: add CircuitSAT → ILP reduction via gate constraint encoding

Direct 1-step path, more efficient than CircuitSAT→SpinGlass→QUBO→ILP (issue #83)."
```

---

## Task 3: Integration Verification

### Step 1: Run full test suite

Run: `make test clippy`
Expected: All tests pass, no clippy warnings

### Step 2: Verify reduction graph has new paths

Run: `cargo test --features ilp-solver test_reduction_graph -- --nocapture 2>&1 | grep -i "qubo\|circuit"` or check the graph test file at `src/unit_tests/rules/graph.rs` for existing assertions and add new ones if needed.

### Step 3: Test CLI integration (if on the cli-tool-design branch)

Run:
```bash
echo '{"problem":"QUBO","instance":{"matrix":[[2.0,1.0],[0.0,-3.0]]}}' > /tmp/qubo_test.json
cargo run --features ilp-solver -- solve /tmp/qubo_test.json
```
Expected: Solution found (not "No reduction path" error)

### Step 4: Commit any fixes

---

## Task 4: Examples and Documentation (Optional — can be a follow-up PR)

**Files:**
- Create: `examples/reduction_qubo_to_ilp.rs`
- Create: `examples/reduction_circuitsat_to_ilp.rs`
- Modify: `tests/suites/examples.rs` (register new examples)
- Modify: `docs/paper/reductions.typ` (add reduction-rule entries)

### Step 1: Create QUBO → ILP example

Follow pattern from `examples/reduction_ilp_to_qubo.rs`:
- Create a small QUBO instance (e.g., 3-variable)
- Reduce to ILP, print transformation details
- Solve with BruteForce, extract and verify
- Export JSON to `docs/paper/examples/`

### Step 2: Create CircuitSAT → ILP example

Follow pattern from `examples/reduction_circuitsat_to_spinglass.rs`:
- Create a small circuit (e.g., AND + NOT)
- Reduce to ILP, print transformation details
- Solve, extract and verify
- Export JSON

### Step 3: Register examples in test suite

Add to `tests/suites/examples.rs`:
```rust
example_test!(reduction_qubo_to_ilp);
example_test!(reduction_circuitsat_to_ilp);
example_fn!(test_qubo_to_ilp, reduction_qubo_to_ilp);
example_fn!(test_circuitsat_to_ilp, reduction_circuitsat_to_ilp);
```

### Step 4: Add paper entries

In `docs/paper/reductions.typ`, add `reduction-rule` entries for both new reductions.

### Step 5: Commit

```bash
git add examples/ tests/suites/examples.rs docs/
git commit -m "docs: add examples and paper entries for QUBO→ILP and CircuitSAT→ILP"
```
