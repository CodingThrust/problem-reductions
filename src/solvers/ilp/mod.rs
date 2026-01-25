//! ILP (Integer Linear Programming) solver module.
//!
//! This module provides an ILP solver using the HiGHS solver via the `good_lp` crate.
//! It is only available when the `ilp` feature is enabled.
//!
//! # Example
//!
//! ```rust,ignore
//! use problemreductions::models::optimization::{ILP, VarBounds, LinearConstraint, ObjectiveSense};
//! use problemreductions::solvers::ILPSolver;
//!
//! // Create a simple ILP: maximize x0 + 2*x1 subject to x0 + x1 <= 1
//! let ilp = ILP::binary(
//!     2,
//!     vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 1.0)],
//!     vec![(0, 1.0), (1, 2.0)],
//!     ObjectiveSense::Maximize,
//! );
//!
//! let solver = ILPSolver::new();
//! let solution = solver.solve(&ilp);
//! ```

mod solver;

pub use solver::ILPSolver;
