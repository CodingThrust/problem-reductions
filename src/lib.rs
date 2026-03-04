//! # Problem Reductions
//!
//! NP-hard problem definitions and reductions.
//! See the [user guide](https://codingthrust.github.io/problem-reductions/) for tutorials and examples.
//!
//! ## API Overview
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`models`] | Problem types — [`graph`](models::graph), [`formula`](models::formula), [`set`](models::set), [`algebraic`](models::algebraic), [`misc`](models::misc) |
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
pub(crate) mod expr;
pub mod io;
pub mod models;
pub mod registry;
pub mod rules;
pub mod solvers;
pub mod topology;
pub mod traits;
#[allow(dead_code)]
pub(crate) mod truth_table;
pub mod types;
pub mod variant;

/// Prelude module for convenient imports.
pub mod prelude {
    // Problem types
    // Types
    pub use crate::error::{ProblemError, Result};
    // Core traits
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::{
        models::{
            algebraic::{BMF, QUBO},
            formula::{CNFClause, CircuitSAT, KSatisfiability, Satisfiability},
            graph::{
                BicliqueCover, KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
                MaximumMatching, MinimumDominatingSet, MinimumVertexCover, SpinGlass,
                TravelingSalesman,
            },
            misc::{BinPacking, Factoring, LongestCommonSubsequence, PaintShop},
            set::{MaximumSetPacking, MinimumSetCovering},
        },
        solvers::{BruteForce, Solver},
        traits::{OptimizationProblem, Problem, SatisfactionProblem},
        types::{Direction, One, ProblemSize, SolutionSize, Unweighted},
    };
}

// Re-export commonly used items at crate root
pub use error::{ProblemError, Result};
// Re-export inventory so `declare_variants!` can use `$crate::inventory::submit!`
pub use inventory;
// Re-export proc macros for reduction registration and variant declaration
pub use problemreductions_macros::{declare_variants, reduction};
pub use registry::{ComplexityClass, ProblemInfo};
pub use solvers::{BruteForce, Solver};
pub use traits::{OptimizationProblem, Problem, SatisfactionProblem};
pub use types::{
    Direction, NumericSize, One, ProblemSize, SolutionSize, Unweighted, WeightElement,
};

#[cfg(test)]
#[path = "unit_tests/graph_models.rs"]
mod test_graph_models;
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
