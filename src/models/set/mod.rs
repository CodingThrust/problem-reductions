//! Set-based optimization problems.
//!
//! This module contains NP-hard problems based on set operations:
//! - [`MinimumSetCovering`]: Minimum weight set cover
//! - [`MaximumSetPacking`]: Maximum weight set packing

mod maximum_set_packing;
mod minimum_set_covering;

pub use maximum_set_packing::MaximumSetPacking;
pub use minimum_set_covering::MinimumSetCovering;

// Validation utilities
pub use maximum_set_packing::is_set_packing;
pub use minimum_set_covering::is_set_cover;
