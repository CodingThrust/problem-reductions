//! Customized solver module.
//!
//! Provides exact witness recovery for problems that have dedicated
//! structure-exploiting backends, without requiring ILP reduction paths.

mod solver;

pub use solver::CustomizedSolver;
