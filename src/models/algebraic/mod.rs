//! Algebraic problems.
//!
//! Problems whose input is a matrix, linear system, or lattice:
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`BMF`]: Boolean Matrix Factorization

pub(crate) mod bmf;
mod closest_vector_problem;
mod ilp;
mod qubo;

pub use bmf::BMF;
pub use closest_vector_problem::ClosestVectorProblem;
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VarBounds, ILP};
pub use qubo::QUBO;
