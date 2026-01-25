//! ILP (Integer Linear Programming) solver module.
//!
//! This module provides an ILP solver using the HiGHS solver via the `good_lp` crate.
//! It is only available when the `ilp` feature is enabled.
//!
//! # Example
//!
//! ```rust,ignore
//! use problemreductions::prelude::*;
//! use problemreductions::solvers::ILPSolver;
//!
//! let problem: IndependentSetT = IndependentSetT::new(4, vec![(0, 1), (1, 2), (2, 3)]);
//! let solver = ILPSolver::new();
//! let solution = solver.solve(&problem);
//! ```

mod solver;
mod traits;

pub use solver::ILPSolver;
pub use traits::{ILPFormulation, ObjectiveSense, ToILP};
