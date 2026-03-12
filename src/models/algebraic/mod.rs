//! Algebraic problems.
//!
//! Problems whose input is a matrix, linear system, or lattice:
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`BMF`]: Boolean Matrix Factorization
//! - [`MultivariateQuadratic`]: Multivariate Quadratic equations over finite fields

pub(crate) mod bmf;
mod closest_vector_problem;
mod ilp;
mod multivariate_quadratic;
mod qubo;

pub use bmf::BMF;
pub use closest_vector_problem::{ClosestVectorProblem, VarBounds};
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VariableDomain, ILP};
pub use multivariate_quadratic::{MultivariateQuadratic, QuadraticPoly};
pub use qubo::QUBO;
