//! King's Subgraph (KSG) mapping module.
//!
//! Maps arbitrary graphs to King's Subgraph (8-connected grid graphs).
//! Supports both unweighted and weighted modes.

pub mod gadgets;
pub mod gadgets_weighted;

/// Spacing between copy lines for KSG mapping.
pub const SPACING: usize = 4;

/// Padding around the grid for KSG mapping.
pub const PADDING: usize = 2;
