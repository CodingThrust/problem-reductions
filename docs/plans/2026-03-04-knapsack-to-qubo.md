# Knapsack to QUBO Reduction Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a reduction rule from Knapsack to QUBO that encodes the capacity constraint as a quadratic penalty using binary slack variables.

**Architecture:** The capacity inequality ∑w_i·x_i ≤ C is converted to equality by adding B = ⌊log₂C⌋+1 binary slack variables. The QUBO objective combines −∑v_i·x_i (value) with P·(constraint violation)² where P > ∑v_i. The target QUBO has n+B variables; solution extraction takes only the first n variables.

**Tech Stack:** Rust, `QUBO<f64>`, `Knapsack`, `BruteForce` solver

**Reference:** Lucas 2014 (*Ising formulations of many NP problems*), Glover et al. 2019

---

### Task 1: Implement the reduction rule

**Files:**
- Create: `src/rules/knapsack_qubo.rs`

**Step 1: Write the failing test**

Create `src/unit_tests/rules/knapsack_qubo.rs` with a closed-loop test:

```rust
use super::*;
use crate::solvers::BruteForce;

#[test]
fn test_knapsack_to_qubo_closed_loop() {
    // 4 items: weights=[2,3,4,5], values=[3,4,5,7], capacity=7
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    // n=4 items + B=floor(log2(7))+1=3 slack vars = 7 total
    assert_eq!(qubo.num_vars(), 7);

    let solver = BruteForce::new();
    let best_source = solver.find_all_best(&knapsack);
    let best_target = solver.find_all_best(qubo);

    // Extract source solutions from target solutions
    let extracted: std::collections::HashSet<Vec<usize>> = best_target
        .iter()
        .map(|t| reduction.extract_solution(t))
        .collect();
    let source_set: std::collections::HashSet<Vec<usize>> =
        best_source.into_iter().collect();

    // Every extracted solution must be optimal for the source
    assert!(extracted.is_subset(&source_set));
    assert!(!extracted.is_empty());
}

#[test]
fn test_knapsack_to_qubo_single_item() {
    // Edge case: 1 item, weight=1, value=1, capacity=1
    let knapsack = Knapsack::new(vec![1], vec![1], 1);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    // n=1 + B=floor(log2(1))+1=1 = 2 vars
    assert_eq!(qubo.num_vars(), 2);

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(qubo);
    let extracted = reduction.extract_solution(&best_target[0]);
    assert_eq!(extracted, vec![1]); // take the item
}

#[test]
fn test_knapsack_to_qubo_infeasible_rejected() {
    // Verify penalty is strong enough: no infeasible QUBO solution beats feasible
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(qubo);

    for sol in &best_target {
        let source_sol = reduction.extract_solution(sol);
        let eval = knapsack.evaluate(&source_sol);
        assert!(eval.is_valid(), "Optimal QUBO solution maps to infeasible knapsack solution");
    }
}

#[test]
fn test_knapsack_to_qubo_empty() {
    // Edge case: capacity 0, nothing fits
    let knapsack = Knapsack::new(vec![1, 2], vec![3, 4], 0);
    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    // B = floor(log2(0))+1 — but log2(0) is undefined.
    // For capacity=0, B=1 (need at least 1 slack bit to encode 0).
    // n=2 + B=1 = 3 vars
    assert_eq!(qubo.num_vars(), 3);

    let solver = BruteForce::new();
    let best_target = solver.find_all_best(qubo);
    let extracted = reduction.extract_solution(&best_target[0]);
    // Nothing should be selected
    assert_eq!(extracted, vec![0, 0]);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test knapsack_to_qubo -- --nocapture 2>&1 | head -20`
Expected: compilation error (module doesn't exist yet)

**Step 3: Implement the reduction**

Create `src/rules/knapsack_qubo.rs`:

```rust
//! Reduction from Knapsack to QUBO.
//!
//! Converts the capacity inequality ∑w_i·x_i ≤ C into equality using B = ⌊log₂C⌋+1
//! binary slack variables, then constructs a QUBO that combines the objective
//! −∑v_i·x_i with a quadratic penalty P·(∑w_i·x_i + ∑2^j·s_j − C)².
//! Penalty P > ∑v_i ensures any infeasible solution costs more than any feasible one.
//!
//! Reference: Lucas, 2014, "Ising formulations of many NP problems".

use crate::models::algebraic::QUBO;
use crate::models::misc::Knapsack;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing Knapsack to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionKnapsackToQUBO {
    target: QUBO<f64>,
    num_items: usize,
}

impl ReductionResult for ReductionKnapsackToQUBO {
    type Source = Knapsack;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution[..self.num_items].to_vec()
    }
}

/// Number of slack bits needed: ⌊log₂C⌋ + 1, or 1 if C = 0.
fn num_slack_bits(capacity: i64) -> usize {
    if capacity <= 0 {
        1
    } else {
        ((capacity as f64).log2().floor() as usize) + 1
    }
}

#[reduction(
    overhead = { num_vars = "num_items + num_slack_bits" }
)]
impl ReduceTo<QUBO<f64>> for Knapsack {
    type Result = ReductionKnapsackToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_items();
        let c = self.capacity();
        let b = num_slack_bits(c);
        let total = n + b;

        // Penalty must exceed sum of all values
        let sum_values: i64 = self.values().iter().sum();
        let penalty = (sum_values + 1) as f64;

        // Build QUBO matrix
        // H = -∑v_i·x_i + P·(∑w_i·x_i + ∑2^j·s_j − C)²
        //
        // Let a_i be the coefficient of variable i in the constraint:
        //   a_i = w_i for i < n (item variables)
        //   a_{n+j} = 2^j for j < B (slack variables)
        // Constraint: ∑a_i·z_i = C, where z = (x_0,...,x_{n-1}, s_0,...,s_{B-1})
        //
        // Expanding the penalty:
        //   P·(∑a_i·z_i − C)² = P·∑∑ a_i·a_j·z_i·z_j − 2P·C·∑a_i·z_i + P·C²
        // Since z_i is binary, z_i² = z_i, so diagonal terms become:
        //   Q[i][i] = P·a_i² − 2P·C·a_i  (from penalty)
        //   Q[i][i] -= v_i               (from objective, item vars only)
        // Off-diagonal terms (i < j):
        //   Q[i][j] = 2P·a_i·a_j
        // Constant P·C² is ignored (doesn't affect optimization).

        let mut coeffs = vec![0.0f64; total];
        for i in 0..n {
            coeffs[i] = self.weights()[i] as f64;
        }
        for j in 0..b {
            coeffs[n + j] = (1i64 << j) as f64;
        }

        let c_f = c as f64;
        let mut matrix = vec![vec![0.0f64; total]; total];

        // Diagonal: P·a_i² − 2P·C·a_i − v_i (for items)
        for i in 0..total {
            matrix[i][i] = penalty * coeffs[i] * coeffs[i] - 2.0 * penalty * c_f * coeffs[i];
            if i < n {
                matrix[i][i] -= self.values()[i] as f64;
            }
        }

        // Off-diagonal (upper triangular): 2P·a_i·a_j
        for i in 0..total {
            for j in (i + 1)..total {
                matrix[i][j] = 2.0 * penalty * coeffs[i] * coeffs[j];
            }
        }

        ReductionKnapsackToQUBO {
            target: QUBO::from_matrix(matrix),
            num_items: n,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/knapsack_qubo.rs"]
mod tests;
```

**Step 4: Register in mod.rs**

Add `mod knapsack_qubo;` to `src/rules/mod.rs` (alphabetical order, after `graph` module).

**Step 5: Add `num_slack_bits` getter to Knapsack**

The overhead expression references `num_slack_bits` as a getter on `Knapsack`. Add to `src/models/misc/knapsack.rs`:

```rust
/// Returns the number of binary slack bits needed for QUBO encoding: ⌊log₂C⌋ + 1.
pub fn num_slack_bits(&self) -> usize {
    if self.capacity <= 0 {
        1
    } else {
        ((self.capacity as f64).log2().floor() as usize) + 1
    }
}
```

**Step 6: Run tests**

Run: `cargo test knapsack_to_qubo -- --nocapture`
Expected: all 4 tests pass

**Step 7: Run clippy**

Run: `cargo clippy --all-targets -- -D warnings`
Expected: no warnings

**Step 8: Commit**

```bash
git add src/rules/knapsack_qubo.rs src/rules/mod.rs src/models/misc/knapsack.rs src/unit_tests/rules/knapsack_qubo.rs
git commit -m "feat: add Knapsack to QUBO reduction rule"
```

---

### Task 2: Write example program

**Files:**
- Create: `examples/reduction_knapsack_to_qubo.rs`
- Modify: `tests/suites/examples.rs`

**Step 1: Write the example**

Create `examples/reduction_knapsack_to_qubo.rs`:

```rust
// # Knapsack to QUBO Reduction
//
// ## Reduction Overview
// The 0-1 Knapsack capacity constraint ∑w_i·x_i ≤ C is converted to equality
// using B = ⌊log₂C⌋ + 1 binary slack variables. The QUBO objective combines
// −∑v_i·x_i with penalty P·(∑w_i·x_i + ∑2^j·s_j − C)² where P > ∑v_i.
//
// ## This Example
// - 4 items: weights=[2,3,4,5], values=[3,4,5,7], capacity=7
// - QUBO: 7 variables (4 items + 3 slack bits)
// - Optimal: items {0,3} (weight=7, value=10)
//
// ## Output
// Exports `docs/paper/examples/knapsack_to_qubo.json` and `knapsack_to_qubo.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;

pub fn run() {
    // Source: Knapsack with 4 items, capacity 7
    let knapsack = Knapsack::new(vec![2, 3, 4, 5], vec![3, 4, 5, 7], 7);

    let reduction = ReduceTo::<QUBO<f64>>::reduce_to(&knapsack);
    let qubo = reduction.target_problem();

    println!("\n=== Problem Transformation ===");
    println!(
        "Source: Knapsack with {} items, capacity {}",
        knapsack.num_items(),
        knapsack.capacity()
    );
    println!("Target: QUBO with {} variables", qubo.num_vars());

    let solver = BruteForce::new();
    let qubo_solutions = solver.find_all_best(qubo);
    println!("\n=== Solution ===");
    println!("Target solutions found: {}", qubo_solutions.len());

    let mut solutions = Vec::new();
    for target_sol in &qubo_solutions {
        let source_sol = reduction.extract_solution(target_sol);
        let eval = knapsack.evaluate(&source_sol);
        assert!(eval.is_valid());
        solutions.push(SolutionPair {
            source_config: source_sol.clone(),
            target_config: target_sol.clone(),
        });
    }

    let source_sol = reduction.extract_solution(&qubo_solutions[0]);
    println!("Source solution: {:?}", source_sol);
    println!("Source value: {:?}", knapsack.evaluate(&source_sol));
    println!("\nReduction verified successfully");

    // Export JSON
    let source_variant = variant_to_map(Knapsack::variant());
    let target_variant = variant_to_map(QUBO::<f64>::variant());
    let overhead = lookup_overhead("Knapsack", &source_variant, "QUBO", &target_variant)
        .expect("Knapsack -> QUBO overhead not found");

    let data = ReductionData {
        source: ProblemSide {
            problem: Knapsack::NAME.to_string(),
            variant: source_variant,
            instance: serde_json::json!({
                "num_items": knapsack.num_items(),
                "weights": knapsack.weights(),
                "values": knapsack.values(),
                "capacity": knapsack.capacity(),
            }),
        },
        target: ProblemSide {
            problem: QUBO::<f64>::NAME.to_string(),
            variant: target_variant,
            instance: serde_json::json!({
                "num_vars": qubo.num_vars(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    write_example("knapsack_to_qubo", &data, &results);
}

fn main() {
    run()
}
```

**Step 2: Register in examples.rs**

Add to `tests/suites/examples.rs` (alphabetical):

```rust
example_test!(reduction_knapsack_to_qubo);
// ... in the example_fn section:
example_fn!(test_knapsack_to_qubo, reduction_knapsack_to_qubo);
```

**Step 3: Run the example test**

Run: `cargo test test_knapsack_to_qubo -- --nocapture`
Expected: PASS, JSON files exported

**Step 4: Commit**

```bash
git add examples/reduction_knapsack_to_qubo.rs tests/suites/examples.rs
git commit -m "feat: add Knapsack to QUBO example program"
```

---

### Task 3: Regenerate exports and run full checks

**Step 1: Regenerate reduction graph and schemas**

```bash
cargo run --example export_graph
cargo run --example export_schemas
```

**Step 2: Run full test suite**

```bash
make test
```

**Step 3: Run clippy**

```bash
make clippy
```

**Step 4: Commit generated files**

```bash
git add docs/paper/examples/ docs/paper/reduction_graph.json schemas/
git commit -m "chore: regenerate exports after Knapsack->QUBO rule"
```

---

### Task 4: Document in paper

Use `/write-rule-in-paper` skill to add the reduction-rule entry in `docs/paper/reductions.typ`.

The entry should cover:
- Formal reduction statement (Knapsack → QUBO via slack variables + penalty)
- Proof sketch (penalty ensures feasibility, slack encodes unused capacity)
- Worked example (4 items, capacity 7, P=20, showing optimal/suboptimal/infeasible)
- Auto-derived overhead from JSON

**Step 1: Invoke skill**

Run: `/write-rule-in-paper` for Knapsack → QUBO

**Step 2: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add Knapsack to QUBO reduction in paper"
```

---

### Task 5: Final verification

**Step 1: Run review-implementation**

Run: `/review-implementation` to verify structural and semantic checks pass.

**Step 2: Fix any issues found**

Address review feedback.

**Step 3: Final commit and push**

```bash
make check  # fmt + clippy + test
git push
```
