//! ILP (Integer Linear Programming) solver module.
//!
//! This module provides an ILP solver using the HiGHS solver through its native Rust bindings.

pub(super) mod adapter;
mod solver;

pub use solver::{ILPSolveError, ILPSolver};
