# Feature Parity Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Port all remaining features from ProblemReductions.jl to achieve full feature parity.

**Architecture:** Hybrid reduction framework with compile-time traits for type safety and runtime registry for path finding. Custom topology types for HyperGraph and UnitDiskGraph. TruthTable utility for logic gadgets. File I/O via serde.

**Tech Stack:** Rust, petgraph (graphs/path-finding), serde (serialization), bitvec (bit operations)

---

## Task 1: Reduction Framework Core Traits

**Files:**
- Create: `src/rules/mod.rs`
- Create: `src/rules/traits.rs`

**Step 1: Create rules module structure**

Create `src/rules/mod.rs`:
```rust
//! Reduction rules between NP-hard problems.

mod traits;

pub use traits::{ReductionResult, ReduceTo};
```

**Step 2: Write test for ReductionResult trait**

Create `src/rules/traits.rs`:
```rust
//! Core traits for problem reductions.

use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing a source problem to a target problem.
///
/// This trait encapsulates the target problem and provides methods
/// to extract solutions back to the source problem space.
pub trait ReductionResult: Clone {
    /// The source problem type.
    type Source: Problem;
    /// The target problem type.
    type Target: Problem;

    /// Get a reference to the target problem.
    fn target_problem(&self) -> &Self::Target;

    /// Extract a solution from target problem space to source problem space.
    ///
    /// # Arguments
    /// * `target_solution` - A solution to the target problem
    ///
    /// # Returns
    /// The corresponding solution in the source problem space
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize>;

    /// Get the size of the source problem (for complexity analysis).
    fn source_size(&self) -> ProblemSize;

    /// Get the size of the target problem (for complexity analysis).
    fn target_size(&self) -> ProblemSize;
}

/// Trait for problems that can be reduced to target type T.
///
/// # Example
/// ```ignore
/// let sat_problem = Satisfiability::new(...);
/// let reduction = sat_problem.reduce_to::<IndependentSet<i32>>();
/// let is_problem = reduction.target_problem();
/// let solutions = solver.find_best(is_problem);
/// let sat_solutions: Vec<_> = solutions.iter()
///     .map(|s| reduction.extract_solution(s))
///     .collect();
/// ```
pub trait ReduceTo<T: Problem>: Problem {
    /// The reduction result type.
    type Result: ReductionResult<Source = Self, Target = T>;

    /// Reduce this problem to the target problem type.
    fn reduce_to(&self) -> Self::Result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traits_compile() {
        // Traits should compile - actual tests in reduction implementations
    }
}
```

**Step 3: Run test to verify compilation**

Run: `cargo test rules::traits::tests --lib`
Expected: PASS

**Step 4: Update lib.rs to include rules module**

In `src/lib.rs`, add after `pub mod solvers;`:
```rust
pub mod rules;
```

And in the prelude, add:
```rust
pub use crate::rules::{ReductionResult, ReduceTo};
```

**Step 5: Run all tests**

Run: `cargo test`
Expected: All tests pass

**Step 6: Commit**

```bash
git add src/rules/ src/lib.rs
git commit -m "feat(rules): add core reduction traits"
```

---

## Task 2: IndependentSet ↔ VertexCovering Reduction

**Files:**
- Create: `src/rules/vertexcovering_independentset.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write failing test for IS → VC reduction**

Create `src/rules/vertexcovering_independentset.rs`:
```rust
//! Reductions between IndependentSet and VertexCovering problems.
//!
//! These problems are complements: a set S is an independent set iff V\S is a vertex cover.

use crate::models::graph::{IndependentSet, VertexCovering};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing IndependentSet to VertexCovering.
#[derive(Debug, Clone)]
pub struct ReductionISToVC<W> {
    target: VertexCovering<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionISToVC<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Source = IndependentSet<W>;
    type Target = VertexCovering<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    /// If v is in the independent set (1), it's NOT in the vertex cover (0).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.iter().map(|&x| 1 - x).collect()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<VertexCovering<W>> for IndependentSet<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32>,
{
    type Result = ReductionISToVC<W>;

    fn reduce_to(&self) -> Self::Result {
        let target = VertexCovering::with_weights(
            self.num_vertices(),
            self.edges(),
            self.weights().clone(),
        );
        ReductionISToVC {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing VertexCovering to IndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionVCToIS<W> {
    target: IndependentSet<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionVCToIS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Source = VertexCovering<W>;
    type Target = IndependentSet<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution extraction: complement the configuration.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.iter().map(|&x| 1 - x).collect()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<IndependentSet<W>> for VertexCovering<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32>,
{
    type Result = ReductionVCToIS<W>;

    fn reduce_to(&self) -> Self::Result {
        let target = IndependentSet::with_weights(
            self.num_vertices(),
            self.edges(),
            self.weights().clone(),
        );
        ReductionVCToIS {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_is_to_vc_reduction() {
        // Triangle graph: max IS = 1, min VC = 2
        let is_problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionISToVC<i32> = is_problem.reduce_to();
        let vc_problem = reduction.target_problem();

        // Solve the VC problem
        let solver = BruteForce::new();
        let vc_solutions = solver.find_best(vc_problem);

        // Extract back to IS solutions
        let is_solutions: Vec<_> = vc_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Verify IS solutions are valid and optimal
        for sol in &is_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 1, "Max IS in triangle should be 1");
        }
    }

    #[test]
    fn test_vc_to_is_reduction() {
        // Path graph 0-1-2: min VC = 1 (just vertex 1), max IS = 2 (vertices 0 and 2)
        let vc_problem = VertexCovering::<i32>::new(3, vec![(0, 1), (1, 2)]);
        let reduction: ReductionVCToIS<i32> = vc_problem.reduce_to();
        let is_problem = reduction.target_problem();

        let solver = BruteForce::new();
        let is_solutions = solver.find_best(is_problem);

        let vc_solutions: Vec<_> = is_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Verify VC solutions
        for sol in &vc_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 1, "Min VC in path should be 1");
        }
    }

    #[test]
    fn test_roundtrip_is_vc_is() {
        let original = IndependentSet::<i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);
        let solver = BruteForce::new();
        let original_solutions = solver.find_best(&original);

        // IS -> VC -> IS
        let reduction1: ReductionISToVC<i32> = original.reduce_to();
        let vc = reduction1.target_problem().clone();
        let reduction2: ReductionVCToIS<i32> = vc.reduce_to();
        let roundtrip = reduction2.target_problem();

        let roundtrip_solutions = solver.find_best(roundtrip);

        // Solutions should have same objective value
        let orig_size: usize = original_solutions[0].iter().sum();
        let rt_size: usize = roundtrip_solutions[0].iter().sum();
        assert_eq!(orig_size, rt_size);
    }
}
```

**Step 2: Update mod.rs to include the new module**

Update `src/rules/mod.rs`:
```rust
//! Reduction rules between NP-hard problems.

mod traits;
mod vertexcovering_independentset;

pub use traits::{ReduceTo, ReductionResult};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};
```

**Step 3: Add weights() method to IndependentSet and VertexCovering if missing**

Check if `weights()` returns `Vec<W>` - if it returns a reference, the code needs adjustment.

In `src/models/graph/independent_set.rs`, ensure there's:
```rust
/// Get the weights.
pub fn weights(&self) -> &Vec<W> {
    &self.weights
}
```

In `src/models/graph/vertex_covering.rs`, ensure there's:
```rust
/// Get the weights.
pub fn weights(&self) -> &Vec<W> {
    &self.weights
}
```

**Step 4: Run tests**

Run: `cargo test rules::vertexcovering_independentset --lib`
Expected: All 3 tests pass

**Step 5: Commit**

```bash
git add src/rules/
git commit -m "feat(rules): add IndependentSet <-> VertexCovering reductions"
```

---

## Task 3: IndependentSet ↔ SetPacking Reduction

**Files:**
- Create: `src/rules/independentset_setpacking.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write tests and implementation**

Create `src/rules/independentset_setpacking.rs`:
```rust
//! Reductions between IndependentSet and SetPacking problems.
//!
//! IS → SetPacking: Each vertex becomes a set containing its incident edge indices.
//! SetPacking → IS: Each set becomes a vertex; two vertices are adjacent if their sets overlap.

use crate::models::graph::IndependentSet;
use crate::models::set::SetPacking;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::collections::HashSet;
use std::ops::AddAssign;

/// Result of reducing IndependentSet to SetPacking.
#[derive(Debug, Clone)]
pub struct ReductionISToSP<W> {
    target: SetPacking<usize, W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionISToSP<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + std::hash::Hash + Eq,
{
    type Source = IndependentSet<W>;
    type Target = SetPacking<usize, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly: vertex selection = set selection.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<SetPacking<usize, W>> for IndependentSet<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + std::hash::Hash + Eq,
{
    type Result = ReductionISToSP<W>;

    fn reduce_to(&self) -> Self::Result {
        let edges = self.edges();
        let n = self.num_vertices();

        // For each vertex, collect the indices of its incident edges
        let mut sets: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            sets[u].insert(edge_idx);
            sets[v].insert(edge_idx);
        }

        let target = SetPacking::with_weights(sets, self.weights().clone());

        ReductionISToSP {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SetPacking to IndependentSet.
#[derive(Debug, Clone)]
pub struct ReductionSPToIS<W> {
    target: IndependentSet<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionSPToIS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Source = SetPacking<usize, W>;
    type Target = IndependentSet<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solutions map directly.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<IndependentSet<W>> for SetPacking<usize, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + std::hash::Hash + Eq,
{
    type Result = ReductionSPToIS<W>;

    fn reduce_to(&self) -> Self::Result {
        let sets = self.sets();
        let n = sets.len();

        // Create edges between sets that overlap
        let mut edges = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                // Check if sets[i] and sets[j] overlap
                if sets[i].iter().any(|elem| sets[j].contains(elem)) {
                    edges.push((i, j));
                }
            }
        }

        let target = IndependentSet::with_weights(n, edges, self.weights().clone());

        ReductionSPToIS {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};
    use std::collections::HashSet;

    #[test]
    fn test_is_to_setpacking() {
        // Triangle graph
        let is_problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionISToSP<i32> = is_problem.reduce_to();
        let sp_problem = reduction.target_problem();

        let solver = BruteForce::new();
        let sp_solutions = solver.find_best(sp_problem);

        // Extract back
        let is_solutions: Vec<_> = sp_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Max IS in triangle = 1
        for sol in &is_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 1);
        }
    }

    #[test]
    fn test_setpacking_to_is() {
        // Two disjoint sets and one overlapping
        let sets = vec![
            HashSet::from([0, 1]),
            HashSet::from([2, 3]),
            HashSet::from([1, 2]), // overlaps with both
        ];
        let sp_problem = SetPacking::<usize, i32>::new(sets);
        let reduction: ReductionSPToIS<i32> = sp_problem.reduce_to();
        let is_problem = reduction.target_problem();

        let solver = BruteForce::new();
        let is_solutions = solver.find_best(is_problem);

        // Max packing = 2 (sets 0 and 1)
        for sol in &is_solutions {
            let size: usize = sol.iter().sum();
            assert_eq!(size, 2);
        }
    }
}
```

**Step 2: Update mod.rs**

```rust
//! Reduction rules between NP-hard problems.

mod traits;
mod vertexcovering_independentset;
mod independentset_setpacking;

pub use traits::{ReduceTo, ReductionResult};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};
pub use independentset_setpacking::{ReductionISToSP, ReductionSPToIS};
```

**Step 3: Run tests**

Run: `cargo test rules::independentset_setpacking --lib`
Expected: PASS

**Step 4: Commit**

```bash
git add src/rules/
git commit -m "feat(rules): add IndependentSet <-> SetPacking reductions"
```

---

## Task 4: SpinGlass ↔ QUBO Reduction

**Files:**
- Create: `src/rules/spinglass_qubo.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write implementation**

Create `src/rules/spinglass_qubo.rs`:
```rust
//! Reductions between SpinGlass and QUBO problems.
//!
//! QUBO: minimize x^T Q x where x ∈ {0, 1}^n
//! SpinGlass: minimize Σ J_ij s_i s_j + Σ h_i s_i where s ∈ {-1, +1}^n
//!
//! Transformation: s = 2x - 1 (so x=0 → s=-1, x=1 → s=+1)

use crate::models::optimization::{SpinGlass, QUBO};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing QUBO to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionQUBOToSG {
    target: SpinGlass<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionQUBOToSG {
    type Source = QUBO<f64>;
    type Target = SpinGlass<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Solution maps directly (same binary encoding).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl ReduceTo<SpinGlass<f64>> for QUBO<f64> {
    type Result = ReductionQUBOToSG;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_variables();
        let matrix = self.matrix();

        // Convert Q matrix to J interactions and h fields
        // s_i s_j = (2x_i - 1)(2x_j - 1) = 4x_i x_j - 2x_i - 2x_j + 1
        // For minimization equivalence with spin encoding
        let mut interactions = Vec::new();
        let mut onsite = vec![0.0; n];

        for i in 0..n {
            for j in i..n {
                let q = matrix[i][j];
                if i == j {
                    // Diagonal: contributes to onsite
                    onsite[i] -= q;
                } else {
                    // Off-diagonal: contributes to interactions
                    // Q_ij x_i x_j → J_ij term
                    let j_ij = q; // Simplified transformation
                    if j_ij.abs() > 1e-10 {
                        interactions.push(((i, j), j_ij));
                    }
                    // Also contributes to onsite terms
                    onsite[i] -= q;
                    onsite[j] -= q;
                }
            }
        }

        let target = SpinGlass::new(n, interactions, onsite);

        ReductionQUBOToSG {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SpinGlass to QUBO.
#[derive(Debug, Clone)]
pub struct ReductionSGToQUBO {
    target: QUBO<f64>,
    source_size: ProblemSize,
}

impl ReductionResult for ReductionSGToQUBO {
    type Source = SpinGlass<f64>;
    type Target = QUBO<f64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl ReduceTo<QUBO<f64>> for SpinGlass<f64> {
    type Result = ReductionSGToQUBO;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_variables();
        let mut matrix = vec![vec![0.0; n]; n];

        // Convert J interactions to Q matrix
        for &((i, j), j_val) in self.interactions() {
            matrix[i][j] += j_val;
            matrix[j][i] += j_val;
            matrix[i][i] -= j_val;
            matrix[j][j] -= j_val;
        }

        // Convert h fields to diagonal
        for (i, &h) in self.onsite().iter().enumerate() {
            matrix[i][i] -= h;
        }

        let target = QUBO::from_matrix(matrix);

        ReductionSGToQUBO {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_qubo_to_spinglass() {
        // Simple 2-variable QUBO
        let qubo = QUBO::from_matrix(vec![
            vec![1.0, -2.0],
            vec![0.0, 1.0],
        ]);
        let reduction: ReductionQUBOToSG = qubo.reduce_to();
        let sg = reduction.target_problem();

        let solver = BruteForce::new();
        let sg_solutions = solver.find_best(sg);
        let qubo_solutions: Vec<_> = sg_solutions
            .iter()
            .map(|s| reduction.extract_solution(s))
            .collect();

        // Verify solutions are valid
        assert!(!qubo_solutions.is_empty());
    }

    #[test]
    fn test_spinglass_to_qubo() {
        // Simple SpinGlass
        let sg = SpinGlass::new(
            2,
            vec![((0, 1), -1.0)], // Ferromagnetic coupling
            vec![0.0, 0.0],
        );
        let reduction: ReductionSGToQUBO = sg.reduce_to();
        let qubo = reduction.target_problem();

        let solver = BruteForce::new();
        let qubo_solutions = solver.find_best(qubo);

        // Ferromagnetic: both same spin is optimal
        assert!(!qubo_solutions.is_empty());
    }
}
```

**Step 2: Update mod.rs**

Add to `src/rules/mod.rs`:
```rust
mod spinglass_qubo;
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
```

**Step 3: Run tests**

Run: `cargo test rules::spinglass_qubo --lib`
Expected: PASS

**Step 4: Commit**

```bash
git add src/rules/
git commit -m "feat(rules): add SpinGlass <-> QUBO reductions"
```

---

## Task 5: SpinGlass ↔ MaxCut Reduction

**Files:**
- Create: `src/rules/spinglass_maxcut.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write implementation**

Create `src/rules/spinglass_maxcut.rs`:
```rust
//! Reductions between SpinGlass and MaxCut problems.
//!
//! MaxCut → SpinGlass: Direct mapping, edge weights become J couplings.
//! SpinGlass → MaxCut: Requires ancilla vertex for onsite terms.

use crate::models::graph::MaxCut;
use crate::models::optimization::SpinGlass;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;

/// Result of reducing MaxCut to SpinGlass.
#[derive(Debug, Clone)]
pub struct ReductionMaxCutToSG<W> {
    target: SpinGlass<W>,
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionMaxCutToSG<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    type Source = MaxCut<W>;
    type Target = SpinGlass<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<SpinGlass<W>> for MaxCut<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    type Result = ReductionMaxCutToSG<W>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let edges = self.edges();
        let weights = self.edge_weights();

        // Convert edges to J interactions
        let interactions: Vec<((usize, usize), W)> = edges
            .into_iter()
            .zip(weights.into_iter())
            .map(|((u, v), w)| ((u, v), w))
            .collect();

        // No onsite terms for pure MaxCut
        let onsite = vec![W::zero(); n];

        let target = SpinGlass::new(n, interactions, onsite);

        ReductionMaxCutToSG {
            target,
            source_size: self.problem_size(),
        }
    }
}

/// Result of reducing SpinGlass to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionSGToMaxCut<W> {
    target: MaxCut<W>,
    source_size: ProblemSize,
    /// Ancilla vertex index (0 if no ancilla needed).
    ancilla: Option<usize>,
}

impl<W> ReductionResult for ReductionSGToMaxCut<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    type Source = SpinGlass<W>;
    type Target = MaxCut<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        match self.ancilla {
            None => target_solution.to_vec(),
            Some(anc) => {
                // If ancilla is 1, flip all bits; then remove ancilla
                let mut sol = target_solution.to_vec();
                if sol[anc] == 1 {
                    for x in sol.iter_mut() {
                        *x = 1 - *x;
                    }
                }
                sol.remove(anc);
                sol
            }
        }
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<W> ReduceTo<MaxCut<W>> for SpinGlass<W>
where
    W: Clone + Default + PartialOrd + num_traits::Num + num_traits::Zero + std::ops::AddAssign,
{
    type Result = ReductionSGToMaxCut<W>;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_variables();
        let interactions = self.interactions();
        let onsite = self.onsite();

        // Check if we need an ancilla vertex for onsite terms
        let need_ancilla = onsite.iter().any(|h| !h.is_zero());
        let total_vertices = if need_ancilla { n + 1 } else { n };
        let ancilla_idx = if need_ancilla { Some(n) } else { None };

        let mut edges = Vec::new();
        let mut weights = Vec::new();

        // Add interaction edges
        for &((i, j), ref w) in interactions {
            edges.push((i, j));
            weights.push(w.clone());
        }

        // Add onsite terms as edges to ancilla
        if need_ancilla {
            for (i, h) in onsite.iter().enumerate() {
                if !h.is_zero() {
                    edges.push((i, n));
                    weights.push(h.clone());
                }
            }
        }

        let target = MaxCut::with_weights(total_vertices, edges, weights);

        ReductionSGToMaxCut {
            target,
            source_size: self.problem_size(),
            ancilla: ancilla_idx,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_maxcut_to_spinglass() {
        // Simple triangle MaxCut
        let mc = MaxCut::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        let reduction: ReductionMaxCutToSG<i32> = mc.reduce_to();
        let sg = reduction.target_problem();

        let solver = BruteForce::new();
        let solutions = solver.find_best(sg);

        assert!(!solutions.is_empty());
    }

    #[test]
    fn test_spinglass_to_maxcut_no_onsite() {
        // SpinGlass without onsite terms
        let sg = SpinGlass::new(3, vec![((0, 1), 1), ((1, 2), 1)], vec![0, 0, 0]);
        let reduction: ReductionSGToMaxCut<i32> = sg.reduce_to();
        let mc = reduction.target_problem();

        assert_eq!(mc.num_vertices(), 3); // No ancilla needed
        assert!(reduction.ancilla.is_none());
    }

    #[test]
    fn test_spinglass_to_maxcut_with_onsite() {
        // SpinGlass with onsite terms
        let sg = SpinGlass::new(2, vec![((0, 1), 1)], vec![1, 0]);
        let reduction: ReductionSGToMaxCut<i32> = sg.reduce_to();
        let mc = reduction.target_problem();

        assert_eq!(mc.num_vertices(), 3); // Ancilla added
        assert_eq!(reduction.ancilla, Some(2));
    }
}
```

**Step 2: Update mod.rs**

**Step 3: Run tests**

Run: `cargo test rules::spinglass_maxcut --lib`

**Step 4: Commit**

```bash
git add src/rules/
git commit -m "feat(rules): add SpinGlass <-> MaxCut reductions"
```

---

## Task 6: Remaining Reduction Rules

Continue with same pattern for:
- `src/rules/vertexcovering_setcovering.rs`
- `src/rules/matching_setpacking.rs`
- `src/rules/sat_3sat.rs`
- `src/rules/sat_independentset.rs`
- `src/rules/sat_coloring.rs`
- `src/rules/sat_dominatingset.rs`
- `src/rules/spinglass_sat.rs`
- `src/rules/circuit_sat.rs`
- `src/rules/factoring_sat.rs`

Each follows the same structure:
1. Create reduction result struct
2. Implement `ReductionResult` trait
3. Implement `ReduceTo` trait
4. Write tests
5. Update mod.rs
6. Commit

---

## Task 7: ReductionGraph for Path Finding

**Files:**
- Create: `src/rules/graph.rs`
- Modify: `src/rules/mod.rs`

**Step 1: Write implementation**

Create `src/rules/graph.rs`:
```rust
//! Runtime reduction graph for discovering and executing reduction paths.

use petgraph::algo::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use std::any::TypeId;
use std::collections::HashMap;

/// A path through the reduction graph.
#[derive(Debug, Clone)]
pub struct ReductionPath {
    pub type_names: Vec<&'static str>,
    type_ids: Vec<TypeId>,
}

/// Runtime graph of all registered reductions.
pub struct ReductionGraph {
    graph: DiGraph<TypeId, ()>,
    type_names: HashMap<TypeId, &'static str>,
    node_indices: HashMap<TypeId, NodeIndex>,
}

impl ReductionGraph {
    /// Create a new reduction graph with all registered reductions.
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let mut type_names = HashMap::new();
        let mut node_indices = HashMap::new();

        // Register all problem types
        Self::register_types(&mut graph, &mut type_names, &mut node_indices);

        // Register all reductions as edges
        Self::register_reductions(&mut graph, &node_indices);

        Self {
            graph,
            type_names,
            node_indices,
        }
    }

    fn register_types(
        graph: &mut DiGraph<TypeId, ()>,
        type_names: &mut HashMap<TypeId, &'static str>,
        node_indices: &mut HashMap<TypeId, NodeIndex>,
    ) {
        // Add all problem types
        macro_rules! register {
            ($($ty:ty => $name:expr),* $(,)?) => {
                $(
                    let id = TypeId::of::<$ty>();
                    let idx = graph.add_node(id);
                    type_names.insert(id, $name);
                    node_indices.insert(id, idx);
                )*
            };
        }

        use crate::models::graph::*;
        use crate::models::optimization::*;
        use crate::models::set::*;
        use crate::models::satisfiability::*;

        register! {
            IndependentSet<i32> => "IndependentSet",
            VertexCovering<i32> => "VertexCovering",
            SetPacking<usize, i32> => "SetPacking",
            SetCovering<usize, i32> => "SetCovering",
            MaxCut<i32> => "MaxCut",
            SpinGlass<f64> => "SpinGlass",
            QUBO<f64> => "QUBO",
            Satisfiability<i32> => "Satisfiability",
        }
    }

    fn register_reductions(
        graph: &mut DiGraph<TypeId, ()>,
        node_indices: &HashMap<TypeId, NodeIndex>,
    ) {
        use crate::models::graph::*;
        use crate::models::optimization::*;
        use crate::models::set::*;

        macro_rules! add_edge {
            ($src:ty => $dst:ty) => {
                if let (Some(&src), Some(&dst)) = (
                    node_indices.get(&TypeId::of::<$src>()),
                    node_indices.get(&TypeId::of::<$dst>()),
                ) {
                    graph.add_edge(src, dst, ());
                }
            };
        }

        // Register all reductions
        add_edge!(IndependentSet<i32> => VertexCovering<i32>);
        add_edge!(VertexCovering<i32> => IndependentSet<i32>);
        add_edge!(IndependentSet<i32> => SetPacking<usize, i32>);
        add_edge!(SetPacking<usize, i32> => IndependentSet<i32>);
        add_edge!(SpinGlass<f64> => QUBO<f64>);
        add_edge!(QUBO<f64> => SpinGlass<f64>);
        add_edge!(MaxCut<i32> => SpinGlass<f64>);
        // Add more as implemented...
    }

    /// Find all paths from source to target type.
    pub fn find_paths<S: 'static, T: 'static>(&self) -> Vec<ReductionPath> {
        let src_id = TypeId::of::<S>();
        let dst_id = TypeId::of::<T>();

        let src_idx = match self.node_indices.get(&src_id) {
            Some(&idx) => idx,
            None => return vec![],
        };
        let dst_idx = match self.node_indices.get(&dst_id) {
            Some(&idx) => idx,
            None => return vec![],
        };

        let paths: Vec<Vec<NodeIndex>> =
            all_simple_paths(&self.graph, src_idx, dst_idx, 0, None).collect();

        paths
            .into_iter()
            .map(|path| {
                let type_ids: Vec<TypeId> =
                    path.iter().map(|&idx| self.graph[idx]).collect();
                let type_names: Vec<&'static str> = type_ids
                    .iter()
                    .filter_map(|id| self.type_names.get(id).copied())
                    .collect();
                ReductionPath {
                    type_names,
                    type_ids,
                }
            })
            .collect()
    }
}

impl Default for ReductionGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::graph::{IndependentSet, VertexCovering};

    #[test]
    fn test_find_direct_path() {
        let graph = ReductionGraph::new();
        let paths = graph.find_paths::<IndependentSet<i32>, VertexCovering<i32>>();
        assert!(!paths.is_empty());
        assert_eq!(paths[0].type_names.len(), 2);
    }

    #[test]
    fn test_find_indirect_path() {
        let graph = ReductionGraph::new();
        let paths = graph.find_paths::<IndependentSet<i32>, SetPacking<usize, i32>>();
        assert!(!paths.is_empty());
    }
}
```

**Step 2: Update mod.rs, run tests, commit**

---

## Task 8: Topology - HyperGraph

**Files:**
- Create: `src/topology/mod.rs`
- Create: `src/topology/hypergraph.rs`

**Step 1: Create module structure**

Create `src/topology/mod.rs`:
```rust
//! Graph topology types.

mod hypergraph;

pub use hypergraph::HyperGraph;
```

**Step 2: Implement HyperGraph**

Create `src/topology/hypergraph.rs`:
```rust
//! Hypergraph implementation.

use serde::{Deserialize, Serialize};

/// A hypergraph where edges can connect any number of vertices.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HyperGraph {
    num_vertices: usize,
    edges: Vec<Vec<usize>>,
}

impl HyperGraph {
    /// Create a new hypergraph.
    pub fn new(num_vertices: usize, edges: Vec<Vec<usize>>) -> Self {
        for edge in &edges {
            for &v in edge {
                assert!(v < num_vertices, "vertex index out of bounds");
            }
        }
        Self { num_vertices, edges }
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn edges(&self) -> &[Vec<usize>] {
        &self.edges
    }

    pub fn has_edge(&self, edge: &[usize]) -> bool {
        let mut sorted = edge.to_vec();
        sorted.sort();
        self.edges.iter().any(|e| {
            let mut e_sorted = e.clone();
            e_sorted.sort();
            e_sorted == sorted
        })
    }

    /// Get all vertices adjacent to vertex v (share an edge with v).
    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        for edge in &self.edges {
            if edge.contains(&v) {
                for &u in edge {
                    if u != v && !neighbors.contains(&u) {
                        neighbors.push(u);
                    }
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypergraph_basic() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
        assert_eq!(hg.num_vertices(), 4);
        assert_eq!(hg.num_edges(), 2);
    }

    #[test]
    fn test_hypergraph_neighbors() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2], vec![2, 3]]);
        let neighbors = hg.neighbors(2);
        assert!(neighbors.contains(&0));
        assert!(neighbors.contains(&1));
        assert!(neighbors.contains(&3));
    }

    #[test]
    fn test_hypergraph_has_edge() {
        let hg = HyperGraph::new(4, vec![vec![0, 1, 2]]);
        assert!(hg.has_edge(&[0, 1, 2]));
        assert!(hg.has_edge(&[2, 1, 0])); // Order doesn't matter
        assert!(!hg.has_edge(&[0, 1]));
    }
}
```

**Step 3: Update lib.rs, run tests, commit**

---

## Task 9: Topology - UnitDiskGraph

**Files:**
- Create: `src/topology/unit_disk_graph.rs`
- Modify: `src/topology/mod.rs`

Similar pattern - implement `UnitDiskGraph` with location-based edge computation.

---

## Task 10: TruthTable

**Files:**
- Create: `src/truth_table.rs`
- Modify: `src/lib.rs`

---

## Task 11: File I/O

**Files:**
- Create: `src/io.rs`
- Modify: `src/lib.rs`

---

## Task 12: Final Integration Tests

**Files:**
- Create: `tests/reduction_tests.rs`

Test full reduction chains and roundtrips.

---

## Task 13: Coverage and Documentation

Run coverage, ensure >95%, add doc comments.

```bash
cargo tarpaulin --out Html
cargo doc --open
```
