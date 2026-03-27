//! ILP (Integer Linear Programming) solver module.
//!
//! This module provides an ILP solver using the HiGHS solver via direct `highs-sys` calls.
//! It is only available when the `ilp-highs` feature is enabled.
//!
//! # Example
//!
//! ```rust,ignore
//! use problemreductions::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
//! use problemreductions::solvers::ILPSolver;
//!
//! // Create a simple binary ILP: maximize x0 + 2*x1 subject to x0 + x1 <= 1
//! let ilp = ILP::<bool>::new(
//!     2,
//!     vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
//!     vec![(0, 1.0), (1, 2.0)],
//!     ObjectiveSense::Maximize,
//! );
//!
//! let solver = ILPSolver::new();
//! let solution = solver.solve(&ilp);
//! ```

pub(crate) mod highs_raw;
mod solver;

pub use solver::ILPSolver;
pub use solver::SolveViaReductionError;
