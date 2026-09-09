//! ILP (Integer Linear Programming) solver module.
//!
//! This module provides an ILP solver using the HiGHS solver via the `good_lp` crate.

pub(super) mod adapter;
mod solver;

pub use solver::{ILPSolveError, ILPSolver};
