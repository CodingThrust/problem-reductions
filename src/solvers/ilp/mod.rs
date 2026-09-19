//! ILP (Integer Linear Programming) solver module.
//!
//! This module provides an ILP solver using HiGHS.
//! Numerical backend details are isolated in the HiGHS adapter.

mod adapter;
mod solver;

pub use solver::{ILPSolveError, ILPSolver};
