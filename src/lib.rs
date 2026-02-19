//! # Problem Reductions
//!
//! NP-hard problem definitions and reductions.
//! See the [user guide](https://codingthrust.github.io/problem-reductions/) for tutorials and examples.
//!
//! ## API Overview
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`models`] | Problem types — [`graph`](models::graph), [`satisfiability`](models::satisfiability), [`set`](models::set), [`optimization`](models::optimization), [`specialized`](models::specialized) |
//! | [`rules`] | Reduction rules, [`ReductionGraph`](rules::ReductionGraph) for path search |
//! | [`solvers`] | [`BruteForce`] and [`ILPSolver`](solvers::ILPSolver) |
//! | [`topology`] | Graph types — [`SimpleGraph`](topology::SimpleGraph), [`HyperGraph`](topology::HyperGraph), [`UnitDiskGraph`](topology::UnitDiskGraph), etc. |
//! | [`traits`] | Core traits — [`Problem`], [`OptimizationProblem`], [`SatisfactionProblem`] |
//! | [`types`] | [`SolutionSize`], [`Direction`], [`ProblemSize`], [`WeightElement`] |
//! | [`variant`] | Variant parameter system for problem type parameterization |
//!
//! Use [`prelude`] for convenient imports.

pub mod config;
pub mod error;
pub mod export;
pub mod io;
pub mod models;
pub(crate) mod polynomial;
pub mod registry;
pub mod rules;
pub mod solvers;
pub mod testing;
pub mod topology;
pub mod traits;
#[allow(dead_code)]
pub(crate) mod truth_table;
pub mod types;
pub mod variant;

/// Prelude module for convenient imports.
pub mod prelude {
    // Problem types
    pub use crate::models::graph::{
        KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching,
        MinimumDominatingSet, MinimumVertexCover, TravelingSalesman,
    };
    pub use crate::models::optimization::{SpinGlass, QUBO};
    pub use crate::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
    pub use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
    pub use crate::models::specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};

    // Core traits
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::solvers::{BruteForce, Solver};
    pub use crate::traits::{problem_size, OptimizationProblem, Problem, SatisfactionProblem};

    // Types
    pub use crate::error::{ProblemError, Result};
    pub use crate::types::{Direction, One, ProblemSize, SolutionSize, Unweighted};
}

// Re-export commonly used items at crate root
pub use error::{ProblemError, Result};
pub use registry::{ComplexityClass, ProblemInfo};
pub use solvers::{BruteForce, Solver};
pub use traits::{problem_size, OptimizationProblem, Problem, SatisfactionProblem};
pub use types::{
    Direction, NumericSize, One, ProblemSize, SolutionSize, Unweighted, WeightElement,
};

// Re-export proc macro for reduction registration
pub use problemreductions_macros::reduction;

#[cfg(test)]
#[path = "unit_tests/graph_models.rs"]
mod test_graph_models;
#[cfg(test)]
#[path = "unit_tests/problem_size.rs"]
mod test_problem_size;
#[cfg(test)]
#[path = "unit_tests/property.rs"]
mod test_property;
#[cfg(test)]
#[path = "unit_tests/reduction_graph.rs"]
mod test_reduction_graph;
#[cfg(test)]
#[path = "unit_tests/trait_consistency.rs"]
mod test_trait_consistency;
#[cfg(test)]
#[path = "unit_tests/unitdiskmapping_algorithms/mod.rs"]
mod test_unitdiskmapping_algorithms;
