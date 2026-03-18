//! Algebraic problems.
//!
//! Problems whose input is a matrix, linear system, or lattice:
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`BMF`]: Boolean Matrix Factorization
//! - [`QuadraticAssignment`]: Quadratic Assignment Problem

pub(crate) mod bmf;
pub(crate) mod closest_vector_problem;
pub(crate) mod ilp;
pub(crate) mod quadratic_assignment;
pub(crate) mod qubo;

pub use bmf::BMF;
pub use closest_vector_problem::{ClosestVectorProblem, VarBounds};
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VariableDomain, ILP};
pub use quadratic_assignment::QuadraticAssignment;
pub use qubo::QUBO;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(qubo::canonical_model_example_specs());
    specs.extend(ilp::canonical_model_example_specs());
    specs.extend(closest_vector_problem::canonical_model_example_specs());
    specs.extend(bmf::canonical_model_example_specs());
    specs.extend(quadratic_assignment::canonical_model_example_specs());
    specs
}
