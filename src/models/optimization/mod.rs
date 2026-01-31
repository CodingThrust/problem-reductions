//! Optimization problems.
//!
//! This module contains optimization problems:
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization
//! - [`ILP`]: Integer Linear Programming

mod ilp;
mod qubo;
mod spin_glass;

pub use ilp::{Comparison, LinearConstraint, ObjectiveSense, VarBounds, ILP};
pub use qubo::QUBO;
pub use spin_glass::SpinGlass;
