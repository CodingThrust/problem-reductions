//! Optimization problems.
//!
//! This module contains optimization problems:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`ClosestVectorProblem`]: Closest Vector Problem (minimize lattice distance)
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming

mod bin_packing;
mod closest_vector_problem;
mod ilp;
mod qubo;
mod spin_glass;

pub use bin_packing::BinPacking;
pub use closest_vector_problem::ClosestVectorProblem;
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VarBounds, ILP};
pub use qubo::QUBO;
pub use spin_glass::SpinGlass;
