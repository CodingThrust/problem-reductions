//! Optimization problems.
//!
//! This module contains optimization problems:
//! - [`SpinGlass`]: Ising model Hamiltonian
//! - [`QUBO`]: Quadratic Unconstrained Binary Optimization

mod qubo;
mod spin_glass;

pub use qubo::QUBO;
pub use spin_glass::SpinGlass;
