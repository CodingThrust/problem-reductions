//! ILP (Integer Linear Programming) solver module.
//!
//! This module provides an ILP solver using the HiGHS solver via the `good_lp` crate.
//! It is only available when the `ilp` feature is enabled.

mod solver;

pub use solver::{ILPSolveError, ILPSolver};
