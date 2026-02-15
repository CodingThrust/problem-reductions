//! # Problem Reductions
//!
//! A Rust library for reducing NP-hard problems.
//!
//! This library provides implementations of various NP-hard computational problems
//! and reduction rules between them. It is designed for algorithm research,
//! education, and quantum optimization studies.
//!
//! ## Features
//!
//! - **Problem Definitions**: Implementations of 18+ NP-hard problems
//! - **Reduction Rules**: Transform problems into equivalent problems
//! - **Solvers**: Brute-force solver for testing and verification
//! - **Validation**: Utilities to verify solution validity
//!
//! ## Example
//!
//! ```rust
//! use problemreductions::prelude::*;
//! use problemreductions::models::graph::MaximumIndependentSet;
//! use problemreductions::topology::SimpleGraph;
//!
//! // Create an Independent Set problem on a triangle graph
//! let problem = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
//!
//! // Solve with brute force
//! let solver = BruteForce::new();
//! let solution = solver.find_best(&problem).unwrap();
//!
//! // Maximum independent set in a triangle has size 1
//! assert_eq!(solution.iter().sum::<usize>(), 1);
//! ```
//!
//! ## Problem Categories
//!
//! ### Satisfiability
//! - SAT: Boolean satisfiability with CNF clauses
//! - K-SAT: SAT restricted to k-literal clauses
//! - CircuitSAT: Boolean circuit satisfiability
//! - Factoring: Integer factorization
//!
//! ### Graph Problems
//! - MaximumIndependentSet: Maximum weight independent set
//! - MaximalIS: Maximal independent set
//! - MinimumVertexCover: Minimum weight vertex cover
//! - MinimumDominatingSet: Minimum dominating set
//! - Coloring: K-vertex coloring
//!
//! ### Set Problems
//! - MinimumSetCovering: Minimum weight set cover
//! - MaximumSetPacking: Maximum weight set packing
//!
//! ### Optimization Problems
//! - MaxCut: Maximum cut on weighted graphs
//! - SpinGlass: Ising model Hamiltonian
//! - QUBO: Quadratic unconstrained binary optimization
//! - MaximumMatching: Maximum weight matching
//!
//! ### Specialized Problems
//! - Paintshop: Minimize color switches
//! - BicliqueCover: Biclique cover on bipartite graphs
//! - BMF: Boolean matrix factorization

pub mod config;
pub mod error;
pub mod export;
pub mod graph_types;
pub mod io;
pub mod models;
pub mod polynomial;
pub mod registry;
pub mod rules;
pub mod solvers;
pub mod testing;
pub mod topology;
pub mod traits;
pub mod truth_table;
pub mod types;
pub mod variant;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::config::{
        bits_to_config, config_to_bits, config_to_index, index_to_config, ConfigIterator,
    };
    pub use crate::error::{ProblemError, Result};
    pub use crate::models::graph::{
        KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching,
        MinimumDominatingSet, MinimumVertexCover, TravelingSalesman,
    };
    pub use crate::models::optimization::{
        Comparison, LinearConstraint, ObjectiveSense, SpinGlass, VarBounds, ILP, QUBO,
    };
    pub use crate::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
    pub use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
    pub use crate::models::specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
    pub use crate::registry::{ComplexityClass, ProblemInfo, ProblemMetadata};
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::solvers::{BruteForce, Solver};
    pub use crate::traits::{OptimizationProblem, Problem, SatisfactionProblem};
    pub use crate::types::{
        Direction, NumericSize, One, ProblemSize, SolutionSize, Unweighted, WeightElement,
    };
    pub use crate::variant::{CastToParent, KValue, VariantParam, K1, K2, K3, K4, K5, KN};
}

// Re-export commonly used items at crate root
pub use error::{ProblemError, Result};
pub use registry::{ComplexityClass, ProblemInfo};
pub use solvers::{BruteForce, Solver};
pub use traits::{OptimizationProblem, Problem, SatisfactionProblem};
pub use types::{
    Direction, NumericSize, One, ProblemSize, SolutionSize, Unweighted, WeightElement,
};

// Re-export proc macro for reduction registration
pub use problemreductions_macros::reduction;

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
