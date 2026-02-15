//! Graph to grid graph mapping.
//!
//! This module implements reductions from arbitrary graphs to unit disk grid graphs
//! using the copy-line technique from UnitDiskMapping.jl.
//!
//! # Modules
//!
//! - `ksg`: King's Subgraph (8-connected square grid) mapping
//! - `triangular`: Triangular lattice mapping
//!
//! # Example
//!
//! ```rust
//! use problemreductions::rules::unitdiskmapping::{ksg, triangular};
//!
//! let edges = vec![(0, 1), (1, 2), (0, 2)];
//!
//! // Map to King's Subgraph (unweighted)
//! let result = ksg::map_unweighted(3, &edges);
//!
//! // Map to King's Subgraph (weighted)
//! let weighted_result = ksg::map_weighted(3, &edges);
//!
//! // Map to triangular lattice (weighted)
//! let tri_result = triangular::map_weighted(3, &edges);
//! ```

pub mod alpha_tensor;
mod copyline;
mod grid;
pub mod ksg;
pub mod pathdecomposition;
mod traits;
pub mod triangular;
mod weighted;

// Re-export shared types
pub use copyline::{create_copylines, mis_overhead_copyline, remove_order, CopyLine};
pub use grid::{CellState, MappingGrid};
pub use pathdecomposition::{pathwidth, Layout, PathDecompositionMethod};
pub use traits::{apply_gadget, pattern_matches, unapply_gadget, Pattern, PatternCell};

// Re-export commonly used items from submodules for convenience
pub use ksg::{GridKind, MappingResult};

// Re-exports from private modules (not accessible via ksg:: or triangular::)
pub use copyline::{copyline_weighted_locations_triangular, mis_overhead_copyline_triangular};
pub use weighted::{map_weights, trace_centers, Weightable};
