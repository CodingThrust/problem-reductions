# TravelingSalesman → ILP Reduction Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement a reduction from TravelingSalesman to ILP using position-based variables with McCormick linearization.

**Architecture:** Introduce binary variables x_{v,k} (vertex v at position k) and auxiliary variables y_{v,w,k} for linearizing products. Constraints enforce a valid permutation (assignment), prohibit non-edge consecutive positions, and McCormick constraints linearize the objective. Solution extraction reads the tour permutation from x variables and maps back to edge selection.

**Tech Stack:** Rust, `#[cfg(feature = "ilp")]` gated, `ILPSolver` for solving, `BruteForce` for cross-validation.

---

## Background

The issue says "HamiltonianCycle" but the model was renamed to `TravelingSalesman` during development. The source problem is `TravelingSalesman<SimpleGraph, i32>`.

**Source model** (`src/models/graph/traveling_salesman.rs`):
- Variables: one binary per edge (0 = not in cycle, 1 = in cycle)
- Metric: `SolutionSize<W>` (minimize total edge weight)
- Key methods: `num_vertices()`, `num_edges()`, `edges() -> Vec<(usize, usize, W)>`, `graph() -> &G`

**Target model** (`src/models/optimization/ilp.rs`):
- `ILP::new(num_vars, bounds, constraints, objective, sense)`
- `VarBounds::binary()`, `LinearConstraint::le/ge/eq(terms, rhs)`

**Reference reduction with similar structure:** `src/rules/coloring_ilp.rs` (vertex × color variables, assignment constraints, non-trivial extraction)

## ILP Formulation

Given graph G=(V,E) with n=|V|, m=|E|, edge weights w.

**Main variables:** x_{v,k} ∈ {0,1} for v ∈ V, k ∈ {0,...,n-1}. Meaning: vertex v is at position k in the tour. Index: `v * n + k`. Total: n².

**Auxiliary variables:** For each undirected edge (u,v) ∈ E (with u < v) and each position k ∈ {0,...,n-1}, introduce two auxiliary variables for the two directions:
- y_{u,v,k} linearizes x_{u,k} · x_{v,(k+1) mod n} (edge traversed u→v at position k)
- y_{v,u,k} linearizes x_{v,k} · x_{u,(k+1) mod n} (edge traversed v→u at position k)

Index auxiliary as: n² + edge_idx * 2n + 2k + direction (0 for u→v, 1 for v→u). Total auxiliary: 2mn.

**Constraints:**
1. Each vertex has exactly one position: Σ_k x_{v,k} = 1 for all v. (n constraints)
2. Each position has exactly one vertex: Σ_v x_{v,k} = 1 for all k. (n constraints)
3. Non-edge consecutive prohibition: For each ordered pair (v,w) where {v,w} ∉ E (and v ≠ w), for each k: x_{v,k} + x_{w,(k+1) mod n} ≤ 1. Count: n · (n(n-1) - 2m) constraints.
4. McCormick linearization (3 constraints per auxiliary variable):
   - y ≤ x_{v,k}
   - y ≤ x_{w,(k+1) mod n}
   - y ≥ x_{v,k} + x_{w,(k+1) mod n} - 1
   Total: 3 · 2mn = 6mn constraints.

**Objective:** Minimize Σ_{(u,v)∈E} w(u,v) · Σ_k (y_{u,v,k} + y_{v,u,k})

**Solution extraction:**
1. Read x_{v,k} from ILP solution: for each position k, find vertex v where x_{v,k} = 1 → get tour permutation π
2. Convert tour to edge selection: for each consecutive pair (π(k), π((k+1) mod n)), find the edge index in the source graph and set it to 1

**Size overhead:**
- num_vars: n² + 2mn
- num_constraints: 2n + n(n(n-1) - 2m) + 6mn

---

### Task 1: Implement TravelingSalesman → ILP reduction

**Files:**
- Create: `src/rules/travelingsalesman_ilp.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Create the reduction file**

Create `src/rules/travelingsalesman_ilp.rs`:

```rust
//! Reduction from TravelingSalesman to ILP (Integer Linear Programming).
//!
//! Uses position-based variables x_{v,k} with McCormick linearization.
//! - Variables: x_{v,k} for vertex v at position k (binary), plus auxiliary y variables
//! - Constraints: assignment, non-edge consecutive, McCormick
//! - Objective: minimize total edge weight of the tour

use crate::models::graph::TravelingSalesman;
use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::poly;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing TravelingSalesman to ILP.
#[derive(Debug, Clone)]
pub struct ReductionTSPToILP {
    target: ILP,
    /// Number of vertices in the source graph.
    num_vertices: usize,
    /// Edges of the source graph (for solution extraction).
    source_edges: Vec<(usize, usize)>,
}

impl ReductionTSPToILP {
    /// Variable index for x_{v,k}: vertex v at position k.
    fn x_index(&self, v: usize, k: usize) -> usize {
        v * self.num_vertices + k
    }
}

impl ReductionResult for ReductionTSPToILP {
    type Source = TravelingSalesman<SimpleGraph, i32>;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    /// Extract solution: read tour permutation from x variables,
    /// then map to edge selection for the source problem.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;

        // Read tour: for each position k, find vertex v with x_{v,k} = 1
        let mut tour = vec![0usize; n];
        for k in 0..n {
            for v in 0..n {
                if target_solution[self.x_index(v, k)] == 1 {
                    tour[k] = v;
                    break;
                }
            }
        }

        // Map tour to edge selection
        let mut edge_selection = vec![0usize; self.source_edges.len()];
        for k in 0..n {
            let u = tour[k];
            let v = tour[(k + 1) % n];
            // Find the edge index for (u, v) or (v, u)
            for (idx, &(a, b)) in self.source_edges.iter().enumerate() {
                if (a == u && b == v) || (a == v && b == u) {
                    edge_selection[idx] = 1;
                    break;
                }
            }
        }

        edge_selection
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_vertices ^ 2) + poly!(num_vertices ^ 2 * num_edges)),
            ("num_constraints", poly!(num_vertices) + poly!(num_vertices ^ 3) + poly!(num_vertices * num_edges)),
        ])
    }
)]
impl ReduceTo<ILP> for TravelingSalesman<SimpleGraph, i32> {
    type Result = ReductionTSPToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let graph = self.graph();
        let edges_with_weights = self.edges();
        let source_edges: Vec<(usize, usize)> = edges_with_weights.iter().map(|&(u, v, _)| (u, v)).collect();
        let edge_weights: Vec<f64> = edges_with_weights.iter().map(|&(_, _, w)| w as f64).collect();
        let m = source_edges.len();

        // Variable layout:
        // [0, n²): x_{v,k} = vertex v at position k
        // [n², n² + 2mn): auxiliary y variables for McCormick linearization
        //   For edge_idx e and position k:
        //     y_{forward} at n² + e * 2n + 2k      (x_{u,k} * x_{v,(k+1)%n})
        //     y_{reverse} at n² + e * 2n + 2k + 1  (x_{v,k} * x_{u,(k+1)%n})
        let num_x = n * n;
        let num_y = 2 * m * n;
        let num_vars = num_x + num_y;

        let x_idx = |v: usize, k: usize| -> usize { v * n + k };
        let y_idx = |edge: usize, k: usize, dir: usize| -> usize { num_x + edge * 2 * n + 2 * k + dir };

        let bounds = vec![VarBounds::binary(); num_vars];
        let mut constraints = Vec::new();

        // Constraint 1: Each vertex has exactly one position
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|k| (x_idx(v, k), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Constraint 2: Each position has exactly one vertex
        for k in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|v| (x_idx(v, k), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Constraint 3: Non-edge consecutive prohibition
        // For each ordered pair (v, w) where {v, w} ∉ E and v ≠ w:
        //   x_{v,k} + x_{w,(k+1) mod n} <= 1 for all k
        for v in 0..n {
            for w in 0..n {
                if v == w {
                    continue;
                }
                if graph.has_edge(v, w) {
                    continue;
                }
                for k in 0..n {
                    constraints.push(LinearConstraint::le(
                        vec![(x_idx(v, k), 1.0), (x_idx(w, (k + 1) % n), 1.0)],
                        1.0,
                    ));
                }
            }
        }

        // Constraint 4: McCormick linearization for auxiliary variables
        // For each edge (u, v) at index e:
        //   Forward (dir=0): y = x_{u,k} * x_{v,(k+1)%n}
        //   Reverse (dir=1): y = x_{v,k} * x_{u,(k+1)%n}
        for (e, &(u, v)) in source_edges.iter().enumerate() {
            for k in 0..n {
                let k_next = (k + 1) % n;

                // Forward: y_{e,k,0} = x_{u,k} * x_{v,k_next}
                let y_fwd = y_idx(e, k, 0);
                let xu = x_idx(u, k);
                let xv_next = x_idx(v, k_next);
                constraints.push(LinearConstraint::le(vec![(y_fwd, 1.0), (xu, -1.0)], 0.0));
                constraints.push(LinearConstraint::le(vec![(y_fwd, 1.0), (xv_next, -1.0)], 0.0));
                constraints.push(LinearConstraint::ge(
                    vec![(y_fwd, 1.0), (xu, -1.0), (xv_next, -1.0)],
                    -1.0,
                ));

                // Reverse: y_{e,k,1} = x_{v,k} * x_{u,k_next}
                let y_rev = y_idx(e, k, 1);
                let xv = x_idx(v, k);
                let xu_next = x_idx(u, k_next);
                constraints.push(LinearConstraint::le(vec![(y_rev, 1.0), (xv, -1.0)], 0.0));
                constraints.push(LinearConstraint::le(vec![(y_rev, 1.0), (xu_next, -1.0)], 0.0));
                constraints.push(LinearConstraint::ge(
                    vec![(y_rev, 1.0), (xv, -1.0), (xu_next, -1.0)],
                    -1.0,
                ));
            }
        }

        // Objective: minimize Σ_{e=(u,v)} w_e * Σ_k (y_{e,k,0} + y_{e,k,1})
        let mut objective: Vec<(usize, f64)> = Vec::new();
        for (e, &w) in edge_weights.iter().enumerate() {
            for k in 0..n {
                objective.push((y_idx(e, k, 0), w));
                objective.push((y_idx(e, k, 1), w));
            }
        }

        let target = ILP::new(num_vars, bounds, constraints, objective, ObjectiveSense::Minimize);

        ReductionTSPToILP {
            target,
            num_vertices: n,
            source_edges,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/travelingsalesman_ilp.rs"]
mod tests;
```

**Step 2: Register in `src/rules/mod.rs`**

Add after the existing ILP reduction registrations (after `mod minimumvertexcover_ilp;`):

```rust
#[cfg(feature = "ilp")]
mod travelingsalesman_ilp;
```

And after the existing `pub use minimumvertexcover_ilp::...` line:

```rust
#[cfg(feature = "ilp")]
pub use travelingsalesman_ilp::ReductionTSPToILP;
```

**Step 3: Run build to verify compilation**

Run: `cargo build --features ilp`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add src/rules/travelingsalesman_ilp.rs src/rules/mod.rs
git commit -m "feat: add TravelingSalesman to ILP reduction (#52)"
```

---

### Task 2: Write unit tests for the reduction

**Files:**
- Create: `src/unit_tests/rules/travelingsalesman_ilp.rs`

**Step 1: Create the unit test file**

Create `src/unit_tests/rules/travelingsalesman_ilp.rs`:

```rust
use super::*;
use crate::solvers::{BruteForce, ILPSolver};
use crate::traits::Problem;
use crate::types::SolutionSize;

#[test]
fn test_reduction_creates_valid_ilp_c4() {
    // C4 cycle: 4 vertices, 4 edges. Unique Hamiltonian cycle (the cycle itself).
    let problem = TravelingSalesman::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (1, 2), (2, 3), (3, 0)],
    );
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // n=4, m=4: num_vars = 16 + 2*4*4 = 48
    assert_eq!(ilp.num_vars, 48);
    assert_eq!(ilp.sense, ObjectiveSense::Minimize);

    // All variables should be binary
    for bound in &ilp.bounds {
        assert_eq!(*bound, VarBounds::binary());
    }
}

#[test]
fn test_reduction_c4_closed_loop() {
    // C4 cycle with unit weights: optimal tour cost = 4
    let problem = TravelingSalesman::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (1, 2), (2, 3), (3, 0)],
    );
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Verify extracted solution is valid on source problem
    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid(), "Extracted solution must be valid");
    assert_eq!(metric, SolutionSize::Valid(4));
}

#[test]
fn test_reduction_k4_weighted_closed_loop() {
    // K4 weighted: find minimum weight Hamiltonian cycle
    let problem = TravelingSalesman::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10), (0, 2, 15), (0, 3, 20),
            (1, 2, 35), (1, 3, 25), (2, 3, 30),
        ],
    );

    // Solve via ILP reduction
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Solve via brute force for cross-check
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_best(&problem);
    let bf_metric = problem.evaluate(&bf_solutions[0]);
    let ilp_metric = problem.evaluate(&extracted);

    assert!(ilp_metric.is_valid());
    assert_eq!(ilp_metric, bf_metric, "ILP and brute force must agree on optimal cost");
}

#[test]
fn test_reduction_c5_unweighted_closed_loop() {
    // C5 cycle with unit weights
    let problem = TravelingSalesman::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    );

    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    let metric = problem.evaluate(&extracted);
    assert!(metric.is_valid());
    assert_eq!(metric, SolutionSize::Valid(5));
}

#[test]
fn test_no_hamiltonian_cycle_infeasible() {
    // Path graph 0-1-2-3: no Hamiltonian cycle exists
    let problem = TravelingSalesman::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (1, 2), (2, 3)],
    );

    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();
    let ilp_solver = ILPSolver::new();
    let result = ilp_solver.solve(ilp);

    assert!(result.is_none(), "Path graph should have no Hamiltonian cycle (infeasible ILP)");
}

#[test]
fn test_solution_extraction_structure() {
    // C4 cycle: verify extraction produces correct edge selection format
    let problem = TravelingSalesman::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (1, 2), (2, 3), (3, 0)],
    );
    let reduction: ReductionTSPToILP = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");
    let extracted = reduction.extract_solution(&ilp_solution);

    // Should have one value per edge
    assert_eq!(extracted.len(), 4);
    // All edges should be selected (C4 has unique cycle = all edges)
    assert_eq!(extracted.iter().sum::<usize>(), 4);
}

#[test]
fn test_solve_reduced() {
    // Test via ILPSolver::solve_reduced
    let problem = TravelingSalesman::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10), (0, 2, 15), (0, 3, 20),
            (1, 2, 35), (1, 3, 25), (2, 3, 30),
        ],
    );

    let ilp_solver = ILPSolver::new();
    let solution = ilp_solver.solve_reduced(&problem).expect("solve_reduced should work");

    let metric = problem.evaluate(&solution);
    assert!(metric.is_valid());

    // Cross-check with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_best(&problem);
    assert_eq!(metric, problem.evaluate(&bf_solutions[0]));
}
```

**Step 2: Run tests**

Run: `cargo test --features ilp travelingsalesman_ilp -- --nocapture`
Expected: All 7 tests pass

**Step 3: Commit**

```bash
git add src/unit_tests/rules/travelingsalesman_ilp.rs
git commit -m "test: add unit tests for TravelingSalesman to ILP reduction (#52)"
```

---

### Task 3: Write example program

**Files:**
- Create: `examples/reduction_travelingsalesman_to_ilp.rs`
- Modify: `tests/suites/examples.rs`

**Step 1: Create example file**

Create `examples/reduction_travelingsalesman_to_ilp.rs`:

```rust
// # Traveling Salesman to ILP Reduction
//
// ## Mathematical Formulation
// Variables: x_{v,k} in {0,1} for vertex v and position k;
// auxiliary y variables for McCormick linearization of products.
// Constraints: assignment, non-edge consecutive prohibition, McCormick.
// Objective: minimize total edge weight of the tour.
//
// ## This Example
// - Instance: K4 complete graph with weights
// - Source: TravelingSalesman with 4 vertices, 6 edges
// - Target: ILP with position-based binary variables
//
// ## Output
// Exports `docs/paper/examples/travelingsalesman_to_ilp.json` and `travelingsalesman_to_ilp.result.json`.

use problemreductions::export::*;
use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

pub fn run() {
    // 1. Create TSP instance: K4 with weights
    let problem = TravelingSalesman::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10), (0, 2, 15), (0, 3, 20),
            (1, 2, 35), (1, 3, 25), (2, 3, 30),
        ],
    );

    // 2. Reduce to ILP
    let reduction = ReduceTo::<ILP>::reduce_to(&problem);
    let ilp = reduction.target_problem();

    // 3. Print transformation
    println!("\n=== Problem Transformation ===");
    println!(
        "Source: TravelingSalesman with {} variables ({} edges)",
        problem.num_variables(),
        problem.num_edges()
    );
    println!(
        "Target: ILP with {} variables, {} constraints",
        ilp.num_vars,
        ilp.constraints.len()
    );

    // 4. Solve target ILP
    let ilp_solver = ILPSolver::new();
    let ilp_solution = ilp_solver.solve(ilp).expect("ILP should be solvable");

    // 5. Extract source solution
    let tsp_solution = reduction.extract_solution(&ilp_solution);
    println!("\n=== Solution ===");
    println!("Edge selection: {:?}", tsp_solution);

    // 6. Verify
    let metric = problem.evaluate(&tsp_solution);
    println!("Tour cost: {:?}", metric);
    assert!(metric.is_valid());

    // Cross-check with brute force
    let bf = BruteForce::new();
    let bf_solutions = bf.find_all_best(&problem);
    let bf_metric = problem.evaluate(&bf_solutions[0]);
    assert_eq!(metric, bf_metric, "ILP must match brute force optimum");
    println!("Brute force confirms optimality");

    // 7. Collect solutions and export JSON
    let mut solutions = Vec::new();
    solutions.push(SolutionPair {
        source_config: tsp_solution.clone(),
        target_config: ilp_solution,
    });

    let overhead = lookup_overhead_or_empty("TravelingSalesman", "ILP");
    let edges: Vec<(usize, usize)> = problem.edges().iter().map(|&(u, v, _)| (u, v)).collect();

    let data = ReductionData {
        source: ProblemSide {
            problem: TravelingSalesman::<SimpleGraph, i32>::NAME.to_string(),
            variant: variant_to_map(TravelingSalesman::<SimpleGraph, i32>::variant()),
            instance: serde_json::json!({
                "num_vertices": problem.num_vertices(),
                "num_edges": problem.num_edges(),
                "edges": edges,
            }),
        },
        target: ProblemSide {
            problem: ILP::NAME.to_string(),
            variant: variant_to_map(ILP::variant()),
            instance: serde_json::json!({
                "num_vars": ilp.num_vars,
                "num_constraints": ilp.constraints.len(),
            }),
        },
        overhead: overhead_to_json(&overhead),
    };

    let results = ResultData { solutions };
    let name = "travelingsalesman_to_ilp";
    write_example(name, &data, &results);
}

fn main() {
    run()
}
```

**Step 2: Register in `tests/suites/examples.rs`**

Add after existing example registrations:

```rust
example_test!(reduction_travelingsalesman_to_ilp);
```

And the corresponding test function:

```rust
example_fn!(
    test_travelingsalesman_to_ilp,
    reduction_travelingsalesman_to_ilp
);
```

**Step 3: Run example test**

Run: `cargo test --features ilp test_travelingsalesman_to_ilp -- --nocapture`
Expected: PASS

**Step 4: Commit**

```bash
git add examples/reduction_travelingsalesman_to_ilp.rs tests/suites/examples.rs
git commit -m "feat: add TravelingSalesman to ILP example (#52)"
```

---

### Task 4: Document in paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add reduction-rule entry**

Add the following `reduction-rule` entry in `docs/paper/reductions.typ` in the appropriate location (near other ILP reductions):

```typst
#reduction-rule("TravelingSalesman", "ILP",
  example: true,
  example-caption: [Weighted $K_4$: the optimal tour $0 arrow 1 arrow 3 arrow 2 arrow 0$ with cost 80 is found by position-based ILP.],
)[
  The traveling salesman problem reduces to binary ILP with $n^2 + 2 m n$ variables via position-based encoding with McCormick linearization.
][
  _Construction._ For graph $G = (V, E)$ with $n = |V|$ and $m = |E|$:

  _Variables:_ Binary $x_(v,k) in {0, 1}$ for each vertex $v in V$ and position $k in {0, ..., n-1}$. Interpretation: $x_(v,k) = 1$ iff vertex $v$ is at position $k$ in the tour.

  _Auxiliary variables:_ For each edge $(u,v) in E$ and position $k$, introduce $y_(u,v,k)$ and $y_(v,u,k)$ to linearize the products $x_(u,k) dot x_(v,(k+1) mod n)$ and $x_(v,k) dot x_(u,(k+1) mod n)$ respectively.

  _Constraints:_ (1) Each vertex has exactly one position: $sum_(k=0)^(n-1) x_(v,k) = 1$ for all $v in V$. (2) Each position has exactly one vertex: $sum_(v in V) x_(v,k) = 1$ for all $k$. (3) Non-edge consecutive prohibition: if ${v,w} in.not E$, then $x_(v,k) + x_(w,(k+1) mod n) <= 1$ for all $k$. (4) McCormick: $y <= x_(v,k)$, $y <= x_(w,(k+1) mod n)$, $y >= x_(v,k) + x_(w,(k+1) mod n) - 1$.

  _Objective:_ Minimize $sum_((u,v) in E) w(u,v) dot sum_k (y_(u,v,k) + y_(v,u,k))$.

  _Solution extraction._ For each position $k$, find vertex $v$ with $x_(v,k) = 1$ to recover the tour permutation; then select edges between consecutive positions.
]
```

**Step 2: Commit**

```bash
git add docs/paper/reductions.typ
git commit -m "docs: add TravelingSalesman to ILP reduction rule in paper (#52)"
```

---

### Task 5: Regenerate reduction graph and verify

**Step 1: Regenerate the reduction graph**

Run: `cargo run --features ilp --example export_graph`
Expected: Updates `docs/src/reductions/reduction_graph.json` with TravelingSalesman → ILP edge

**Step 2: Run full test suite**

Run: `make test clippy`
Expected: All tests pass, no clippy warnings

**Step 3: Run coverage check**

Run: `make coverage`
Expected: Coverage >95% for new code

**Step 4: Commit any generated files**

```bash
git add docs/src/reductions/reduction_graph.json
git commit -m "chore: regenerate reduction graph with TravelingSalesman to ILP edge (#52)"
```

---

## Notes

- The `poly!()` macro expressions for overhead are approximate upper bounds. Verify the exact polynomial forms compile correctly; adjust if the macro doesn't support the needed expressions.
- The `ILPSolver::solve()` returns `Option<Vec<usize>>` — `None` means infeasible, which is the expected result for graphs without Hamiltonian cycles.
- For small test instances (n ≤ 5), the ILP solver should complete quickly. Avoid K5 or larger complete graphs in tests as the ILP grows rapidly.
- The `#[reduction(...)]` proc macro must be on the `impl ReduceTo<ILP>` block. Check that `poly!` supports the `^` operator or use multiplication: `poly!(num_vertices * num_vertices)`.
