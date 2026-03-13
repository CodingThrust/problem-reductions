//! Algebraic problems.
//!
//! Problems whose input is a matrix, linear system, or lattice:
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`BMF`]: Boolean Matrix Factorization
//! - [`QuadraticAssignment`]: Quadratic Assignment Problem

pub(crate) mod bmf;
mod closest_vector_problem;
mod ilp;
mod quadratic_assignment;
mod qubo;

pub use bmf::BMF;
pub use closest_vector_problem::{ClosestVectorProblem, VarBounds};
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VariableDomain, ILP};
pub use quadratic_assignment::QuadraticAssignment;
pub use qubo::QUBO;
