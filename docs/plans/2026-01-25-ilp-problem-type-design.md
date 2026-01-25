# ILP as a Problem Type - Design Document

## Overview

Redesign ILP support by treating Integer Linear Programming as a first-class problem type with reductions from other problems, rather than a trait-based approach.

```
IndependentSet  ──┐
VertexCover     ──┤
Clique          ──┼──► ILP ──► ILPSolver
DominatingSet   ──┤
Matching        ──┤
SetPacking      ──┤
SetCovering     ──┘
```

## Benefits

- Fits the library's reduction framework naturally
- Only one solver needed (for ILP)
- Each problem implements `ReduceTo<ILP>` using existing traits
- Solution extraction via `ReductionResult::extract_solution`
- ILP itself is NP-complete, a natural "hub" problem

## Data Structures

### Variable Bounds

```rust
/// Variable bounds (None = unbounded in that direction)
#[derive(Debug, Clone, Copy, Default)]
pub struct VarBounds {
    pub lower: Option<i64>,  // None = -∞
    pub upper: Option<i64>,  // None = +∞
}

impl VarBounds {
    pub fn binary() -> Self { Self { lower: Some(0), upper: Some(1) } }
    pub fn non_negative() -> Self { Self { lower: Some(0), upper: None } }
    pub fn unbounded() -> Self { Self { lower: None, upper: None } }
    pub fn bounded(lo: i64, hi: i64) -> Self { Self { lower: Some(lo), upper: Some(hi) } }
}
```

### Linear Constraint

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Le,  // <=
    Ge,  // >=
    Eq,  // ==
}

/// A linear constraint: sum of (coefficient * variable) {<=, >=, ==} rhs
#[derive(Debug, Clone)]
pub struct LinearConstraint {
    pub terms: Vec<(usize, f64)>,  // sparse: (var_index, coefficient)
    pub cmp: Comparison,
    pub rhs: f64,
}
```

### ILP Problem

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveSense {
    Maximize,
    Minimize,
}

/// Integer Linear Programming problem
#[derive(Debug, Clone)]
pub struct ILP {
    pub num_vars: usize,
    pub bounds: Vec<VarBounds>,           // bounds[i] for variable i
    pub constraints: Vec<LinearConstraint>,
    pub objective: Vec<(usize, f64)>,     // sparse objective coefficients
    pub sense: ObjectiveSense,
}
```

## Problem Trait Implementation

```rust
impl Problem for ILP {
    type Size = f64;

    fn num_variables(&self) -> usize {
        self.num_vars
    }

    fn num_flavors(&self) -> usize {
        self.bounds.iter()
            .map(|b| match (b.lower, b.upper) {
                (Some(lo), Some(hi)) => (hi - lo + 1) as usize,
                _ => usize::MAX,
            })
            .max()
            .unwrap_or(2)
    }

    fn energy_mode(&self) -> EnergyMode {
        match self.sense {
            ObjectiveSense::Maximize => EnergyMode::LargerSizeIsBetter,
            ObjectiveSense::Minimize => EnergyMode::SmallerSizeIsBetter,
        }
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<f64> {
        let bounds_ok = config.iter().enumerate().all(|(i, &val)| {
            let b = &self.bounds[i];
            let val = val as i64;
            b.lower.map_or(true, |lo| val >= lo) &&
            b.upper.map_or(true, |hi| val <= hi)
        });

        let constraints_ok = self.constraints.iter().all(|c| {
            let lhs: f64 = c.terms.iter()
                .map(|&(var, coef)| coef * config[var] as f64)
                .sum();
            match c.cmp {
                Comparison::Le => lhs <= c.rhs + 1e-9,
                Comparison::Ge => lhs >= c.rhs - 1e-9,
                Comparison::Eq => (lhs - c.rhs).abs() < 1e-9,
            }
        });

        let obj: f64 = self.objective.iter()
            .map(|&(var, coef)| coef * config[var] as f64)
            .sum();

        SolutionSize::new(obj, bounds_ok && constraints_ok)
    }
}
```

## Reduction Pattern

```rust
/// Generic result for reductions to ILP
#[derive(Debug, Clone)]
pub struct ReductionToILP<Source: Problem> {
    pub target: ILP,
    pub source_size: ProblemSize,
    _marker: PhantomData<Source>,
}

impl<Source: Problem> ReductionResult for ReductionToILP<Source> {
    type Source = Source;
    type Target = ILP;

    fn target_problem(&self) -> &ILP { &self.target }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()  // 1:1 mapping for most problems
    }

    fn source_size(&self) -> ProblemSize { self.source_size.clone() }
    fn target_size(&self) -> ProblemSize { self.target.problem_size() }
}
```

## ILP Solver

```rust
#[derive(Debug, Clone, Default)]
pub struct ILPSolver {
    pub time_limit: Option<f64>,
}

impl ILPSolver {
    pub fn new() -> Self { Self::default() }

    pub fn solve(&self, problem: &ILP) -> Option<Vec<usize>> {
        // Uses good_lp with HiGHS backend
        // Creates integer variables with specified bounds
        // Adds all constraints
        // Solves and extracts solution
    }

    pub fn solve_reduced<P>(&self, problem: &P) -> Option<Vec<usize>>
    where P: ReduceTo<ILP>
    {
        let reduction = problem.reduce_to();
        let ilp_solution = self.solve(reduction.target_problem())?;
        Some(reduction.extract_solution(&ilp_solution))
    }
}
```

## ILP Formulations

| Problem | Variables | Constraints | Sense |
|---------|-----------|-------------|-------|
| IndependentSet | vertices (binary) | x_u + x_v ≤ 1 ∀(u,v)∈E | max |
| VertexCover | vertices (binary) | x_u + x_v ≥ 1 ∀(u,v)∈E | min |
| Clique | vertices (binary) | x_u + x_v ≤ 1 ∀(u,v)∉E | max |
| DominatingSet | vertices (binary) | x_v + Σ_{u∈N(v)} x_u ≥ 1 ∀v | min |
| Matching | edges (binary) | Σ_{e∋v} x_e ≤ 1 ∀v | max |
| SetPacking | sets (binary) | x_i + x_j ≤ 1 ∀overlap(i,j) | max |
| SetCovering | sets (binary) | Σ_{j covers e} x_j ≥ 1 ∀e | min |

## File Structure

```
src/models/optimization/
├── mod.rs              # add: pub mod ilp;
└── ilp.rs              # NEW: ILP problem type

src/solvers/
├── mod.rs              # update exports
└── ilp.rs              # REPLACE: simplified solver for ILP only

src/rules/
├── mod.rs              # add new reduction modules
├── independentset_ilp.rs    # NEW
├── vertexcovering_ilp.rs    # NEW
├── clique_ilp.rs            # NEW
├── dominatingset_ilp.rs     # NEW
├── matching_ilp.rs          # NEW
├── setpacking_ilp.rs        # NEW
└── setcovering_ilp.rs       # NEW
```

## Removals

- `src/solvers/ilp/traits.rs` - `ToILP` trait no longer needed
- `ILPConstraintSpec` and `ILPEdgeConstraint` from `src/models/graph/template.rs`
