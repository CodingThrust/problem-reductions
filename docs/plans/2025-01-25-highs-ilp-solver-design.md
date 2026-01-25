# HiGHS ILP Solver Design

## Overview

Add an optional Integer Linear Programming (ILP) solver using HiGHS via the `good_lp` crate. This enables exact solving for small-medium instances and helps verify reduction correctness.

## Motivation

- BruteForce solver is exponential (O(2^n)) - only works for tiny instances
- ILP solvers can handle much larger instances efficiently
- Useful for testing and verification of reductions
- Aligns with user's quantum computing use case (verifying reductions)

## Design Decision

**Chosen approach: Trait-based (`ToILP`)**

Rationale:
1. Problems can provide efficient, mathematically clean ILP formulations
2. Rust-idiomatic (trait-based polymorphism)
3. Produces compact formulations (vs exponential truth-table encoding)
4. Extensible - new problems implement the trait

## Architecture

### Dependencies

```toml
[features]
default = []
ilp = ["good_lp"]

[dependencies]
good_lp = { version = "1.8", features = ["highs"], optional = true }
```

### ToILP Trait

```rust
// src/solvers/ilp/traits.rs

#[cfg(feature = "ilp")]
pub trait ToILP: Problem {
    /// Build the ILP model for this problem.
    fn to_ilp(&self, vars: &[Variable]) -> ILPFormulation;
}

#[cfg(feature = "ilp")]
pub struct ILPFormulation {
    /// Linear constraints (Ax <= b, Ax >= b, Ax == b)
    pub constraints: Vec<Constraint>,
    /// Objective expression to optimize
    pub objective: Expression,
    /// Whether to maximize or minimize
    pub sense: ObjectiveSense,
}
```

### ILPSolver

```rust
// src/solvers/ilp/solver.rs

#[cfg(feature = "ilp")]
pub struct ILPSolver {
    /// Time limit in seconds (None = no limit)
    pub time_limit: Option<f64>,
}

#[cfg(feature = "ilp")]
impl ILPSolver {
    pub fn new() -> Self { ... }
    pub fn with_time_limit(seconds: f64) -> Self { ... }

    /// Solve a problem that implements ToILP
    pub fn solve<P: ToILP>(&self, problem: &P) -> Option<Vec<usize>> { ... }

    /// Find all optimal solutions (if solver supports enumeration)
    pub fn find_all<P: ToILP>(&self, problem: &P) -> Vec<Vec<usize>> { ... }
}
```

### Problem Formulations

| Problem | Formulation |
|---------|-------------|
| IndependentSet | max Σ wᵢxᵢ s.t. xᵤ + xᵥ ≤ 1 ∀(u,v)∈E |
| VertexCover | min Σ wᵢxᵢ s.t. xᵤ + xᵥ ≥ 1 ∀(u,v)∈E |
| Clique | max Σ wᵢxᵢ s.t. xᵤ + xᵥ ≤ 1 ∀(u,v)∉E |
| SetPacking | max Σ wᵢxᵢ s.t. Σⱼ∈S xⱼ ≤ 1 ∀ overlapping S |
| SetCovering | min Σ wᵢxᵢ s.t. Σⱼ∈covering(e) xⱼ ≥ 1 ∀ elements e |
| SAT | feasibility: Σ literals ≥ 1 per clause |

### Implementation for Graph Problems

```rust
#[cfg(feature = "ilp")]
impl<G: Graph, W: Into<f64> + Clone> ToILP for IndependentSetT<G, W> {
    fn to_ilp(&self, vars: &[Variable]) -> ILPFormulation {
        let mut constraints = Vec::new();

        // For each edge (u, v): x_u + x_v <= 1
        for (u, v) in self.graph().edges() {
            constraints.push(vars[u] + vars[v] <= 1.0);
        }

        // Objective: maximize sum of weighted selections
        let objective = self.weights()
            .iter()
            .enumerate()
            .map(|(i, w)| w.clone().into() * vars[i])
            .sum();

        ILPFormulation {
            constraints,
            objective,
            sense: ObjectiveSense::Maximize,
        }
    }
}
```

## File Structure

```
src/solvers/
├── mod.rs              # Add ilp module export
├── brute_force.rs      # Existing
└── ilp/
    ├── mod.rs          # Module exports
    ├── traits.rs       # ToILP trait
    ├── solver.rs       # ILPSolver implementation
    └── formulations/
        ├── mod.rs
        ├── graph.rs    # Graph problem formulations
        ├── set.rs      # Set problem formulations
        └── sat.rs      # SAT formulation
```

## Testing Strategy

1. **Correctness tests**: Compare ILP results with BruteForce on small instances
2. **Performance tests**: Benchmark ILP on medium instances (20-50 variables)
3. **Edge cases**: Empty problems, disconnected graphs, infeasible instances

```rust
#[test]
#[cfg(feature = "ilp")]
fn test_ilp_matches_brute_force() {
    let problem: IndependentSetT = IndependentSetT::new(10, edges);

    let bf = BruteForce::new();
    let ilp = ILPSolver::new();

    let bf_solutions = bf.find_best(&problem);
    let ilp_solution = ilp.solve(&problem).unwrap();

    // ILP solution should have same objective value as brute force
    let bf_size = problem.solution_size(&bf_solutions[0]).size;
    let ilp_size = problem.solution_size(&ilp_solution).size;
    assert_eq!(bf_size, ilp_size);
}
```

## Implementation Plan

1. Add `good_lp` dependency with `highs` feature flag
2. Create `ToILP` trait in `src/solvers/ilp/traits.rs`
3. Implement `ILPSolver` in `src/solvers/ilp/solver.rs`
4. Implement `ToILP` for graph problems (IndependentSetT, VertexCoverT, CliqueT)
5. Implement `ToILP` for set problems (SetPacking, SetCovering)
6. Add integration tests comparing ILP vs BruteForce
7. Update documentation

## Not in Scope (for initial implementation)

- Solution enumeration (finding all optimal solutions)
- Warm starting from previous solutions
- Cut generation / branch-and-cut customization
- MIP callbacks
- MaxCut (requires quadratic linearization - more complex)
- Coloring (requires big-M constraints - more complex)
