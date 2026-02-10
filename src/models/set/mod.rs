//! Set-based optimization problems.
//!
//! This module contains NP-hard problems based on set operations:
//! - [`MinimumSetCovering`]: Minimum weight set cover
//! - [`MaximumSetPacking`]: Maximum weight set packing

mod minimum_set_covering;
mod maximum_set_packing;

pub use minimum_set_covering::MinimumSetCovering;
pub use maximum_set_packing::MaximumSetPacking;

// Validation utilities
pub use minimum_set_covering::is_set_cover;
pub use maximum_set_packing::is_set_packing;
