//! Customized solver module.
//!
//! Provides exact witness recovery for problems that have dedicated
//! structure-exploiting backends, without requiring ILP reduction paths.

pub(crate) mod fd_subset_search;
mod solver;

pub use solver::CustomizedSolver;
