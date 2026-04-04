//! Algebraic problems.
//!
//! Problems whose input is a matrix, linear system, or lattice:
//! - [`AlgebraicEquationsOverGF2`]: Polynomial equations over GF(2)
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`IntegerExpressionMembership`]: Pick one integer from each choice set to hit a target sum
//! - [`BMF`]: Boolean Matrix Factorization
//! - [`ConsecutiveBlockMinimization`]: Consecutive Block Minimization
//! - [`ConsecutiveOnesSubmatrix`]: Consecutive Ones Submatrix (column selection with C1P)
//! - [`MinimumWeightSolutionToLinearEquations`]: Sparse binary solution to a linear system
//! - [`QuadraticAssignment`]: Quadratic Assignment Problem
//! - [`SparseMatrixCompression`]: Sparse Matrix Compression by row overlay
//! - [`SimultaneousIncongruences`]: Find an integer avoiding a family of residue classes

pub(crate) mod algebraic_equations_over_gf2;
pub(crate) mod bmf;
pub(crate) mod closest_vector_problem;
pub(crate) mod consecutive_block_minimization;
pub(crate) mod consecutive_ones_matrix_augmentation;
pub(crate) mod consecutive_ones_submatrix;
pub(crate) mod ilp;
pub(crate) mod integer_expression_membership;
pub(crate) mod minimum_weight_solution_to_linear_equations;
pub(crate) mod quadratic_assignment;
pub(crate) mod qubo;
pub(crate) mod simultaneous_incongruences;
pub(crate) mod sparse_matrix_compression;

pub use algebraic_equations_over_gf2::AlgebraicEquationsOverGF2;
pub use bmf::BMF;
pub use closest_vector_problem::{ClosestVectorProblem, VarBounds};
pub use consecutive_block_minimization::ConsecutiveBlockMinimization;
pub use consecutive_ones_matrix_augmentation::ConsecutiveOnesMatrixAugmentation;
pub use consecutive_ones_submatrix::ConsecutiveOnesSubmatrix;
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VariableDomain, ILP};
pub use integer_expression_membership::IntegerExpressionMembership;
pub use minimum_weight_solution_to_linear_equations::MinimumWeightSolutionToLinearEquations;
pub use quadratic_assignment::QuadraticAssignment;
pub use qubo::QUBO;
pub use simultaneous_incongruences::SimultaneousIncongruences;
pub use sparse_matrix_compression::SparseMatrixCompression;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(algebraic_equations_over_gf2::canonical_model_example_specs());
    specs.extend(qubo::canonical_model_example_specs());
    specs.extend(ilp::canonical_model_example_specs());
    specs.extend(closest_vector_problem::canonical_model_example_specs());
    specs.extend(integer_expression_membership::canonical_model_example_specs());
    specs.extend(minimum_weight_solution_to_linear_equations::canonical_model_example_specs());
    specs.extend(bmf::canonical_model_example_specs());
    specs.extend(consecutive_block_minimization::canonical_model_example_specs());
    specs.extend(consecutive_ones_matrix_augmentation::canonical_model_example_specs());
    specs.extend(consecutive_ones_submatrix::canonical_model_example_specs());
    specs.extend(quadratic_assignment::canonical_model_example_specs());
    specs.extend(simultaneous_incongruences::canonical_model_example_specs());
    specs.extend(sparse_matrix_compression::canonical_model_example_specs());
    specs
}
