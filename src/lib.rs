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
//! use problemreductions::models::graph::IndependentSet;
//!
//! // Create an Independent Set problem on a triangle graph
//! let problem = IndependentSet::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
//!
//! // Solve with brute force
//! let solver = BruteForce::new();
//! let solutions = solver.find_best(&problem);
//!
//! // Maximum independent set in a triangle has size 1
//! assert!(solutions.iter().all(|s| s.iter().sum::<usize>() == 1));
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
//! - IndependentSet: Maximum weight independent set
//! - MaximalIS: Maximal independent set
//! - VertexCovering: Minimum weight vertex cover
//! - DominatingSet: Minimum dominating set
//! - Coloring: K-vertex coloring
//!
//! ### Set Problems
//! - SetCovering: Minimum weight set cover
//! - SetPacking: Maximum weight set packing
//!
//! ### Optimization Problems
//! - MaxCut: Maximum cut on weighted graphs
//! - SpinGlass: Ising model Hamiltonian
//! - QUBO: Quadratic unconstrained binary optimization
//! - Matching: Maximum weight matching
//!
//! ### Specialized Problems
//! - Paintshop: Minimize color switches
//! - BicliqueCover: Biclique cover on bipartite graphs
//! - BMF: Boolean matrix factorization

pub mod config;
pub mod error;
pub mod models;
pub mod registry;
pub mod rules;
pub mod solvers;
pub mod testing;
pub mod topology;
pub mod traits;
pub mod truth_table;
pub mod types;

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::config::{
        bits_to_config, config_to_bits, config_to_index, index_to_config, ConfigIterator,
    };
    pub use crate::error::{ProblemError, Result};
    pub use crate::models::graph::{
        Coloring, DominatingSet, IndependentSet, Matching, MaxCut, MaximalIS, VertexCovering,
    };
    pub use crate::models::optimization::{SpinGlass, QUBO};
    pub use crate::models::satisfiability::{CNFClause, Satisfiability};
    pub use crate::models::set::{SetCovering, SetPacking};
    pub use crate::models::specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
    pub use crate::registry::{
        ComplexityClass, GraphSubcategory, ProblemCategory, ProblemInfo, ProblemMetadata,
    };
    pub use crate::solvers::{BruteForce, Solver};
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::traits::{csp_solution_size, ConstraintSatisfactionProblem, Problem};
    pub use crate::types::{
        EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize,
    };
}

// Re-export commonly used items at crate root
pub use error::{ProblemError, Result};
pub use registry::{ComplexityClass, ProblemCategory, ProblemInfo};
pub use solvers::{BruteForce, Solver};
pub use traits::{ConstraintSatisfactionProblem, Problem};
pub use types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
