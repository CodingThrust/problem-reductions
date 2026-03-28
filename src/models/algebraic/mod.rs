//! Algebraic problems.
//!
//! Problems whose input is a matrix, linear system, or lattice:
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`BMF`]: Boolean Matrix Factorization
//! - [`ConsecutiveBlockMinimization`]: Consecutive Block Minimization
//! - [`ConsecutiveOnesSubmatrix`]: Consecutive Ones Submatrix (column selection with C1P)
//! - [`QuadraticAssignment`]: Quadratic Assignment Problem
//! - [`QuadraticDiophantineEquations`]: Decide ax² + by = c in positive integers
//! - [`SparseMatrixCompression`]: Sparse Matrix Compression by row overlay

pub(crate) mod bmf;
pub(crate) mod closest_vector_problem;
pub(crate) mod consecutive_block_minimization;
pub(crate) mod consecutive_ones_matrix_augmentation;
pub(crate) mod consecutive_ones_submatrix;
pub(crate) mod ilp;
pub(crate) mod quadratic_assignment;
pub(crate) mod quadratic_diophantine_equations;
pub(crate) mod qubo;
pub(crate) mod sparse_matrix_compression;

pub use bmf::BMF;
pub use closest_vector_problem::{ClosestVectorProblem, VarBounds};
pub use consecutive_block_minimization::ConsecutiveBlockMinimization;
pub use consecutive_ones_matrix_augmentation::ConsecutiveOnesMatrixAugmentation;
pub use consecutive_ones_submatrix::ConsecutiveOnesSubmatrix;
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VariableDomain, ILP};
pub use quadratic_assignment::QuadraticAssignment;
pub use quadratic_diophantine_equations::QuadraticDiophantineEquations;
pub use qubo::QUBO;
pub use sparse_matrix_compression::SparseMatrixCompression;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let mut specs = Vec::new();
    specs.extend(qubo::canonical_model_example_specs());
    specs.extend(ilp::canonical_model_example_specs());
    specs.extend(closest_vector_problem::canonical_model_example_specs());
    specs.extend(bmf::canonical_model_example_specs());
    specs.extend(consecutive_block_minimization::canonical_model_example_specs());
    specs.extend(consecutive_ones_matrix_augmentation::canonical_model_example_specs());
    specs.extend(consecutive_ones_submatrix::canonical_model_example_specs());
    specs.extend(quadratic_assignment::canonical_model_example_specs());
    specs.extend(quadratic_diophantine_equations::canonical_model_example_specs());
    specs.extend(sparse_matrix_compression::canonical_model_example_specs());
    specs
}
