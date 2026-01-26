//! Graph to grid mapping functionality.
//!
//! This module provides tools for embedding arbitrary graphs into 2D grids
//! using the copy-line technique.

mod copyline;
mod gadgets;

pub use copyline::{create_copylines, mis_overhead_copyline, remove_order, CopyLine};
pub use gadgets::*;
