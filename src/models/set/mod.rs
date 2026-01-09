//! Set-based optimization problems.
//!
//! This module contains NP-hard problems based on set operations:
//! - [`SetCovering`]: Minimum weight set cover
//! - [`SetPacking`]: Maximum weight set packing

mod set_covering;
mod set_packing;

pub use set_covering::SetCovering;
pub use set_packing::SetPacking;

// Validation utilities
pub use set_covering::is_set_cover;
pub use set_packing::is_set_packing;
