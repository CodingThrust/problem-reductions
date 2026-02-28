//! Optimization problems.
//!
//! This module contains optimization problems:
//! - [`BinPacking`]: Bin Packing (minimize bins)
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming

mod bin_packing;
mod ilp;
mod qubo;
mod spin_glass;

pub use bin_packing::BinPacking;
pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VarBounds, ILP};
pub use qubo::QUBO;
pub use spin_glass::SpinGlass;
