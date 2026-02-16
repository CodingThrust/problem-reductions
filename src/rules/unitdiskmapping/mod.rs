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

#[allow(dead_code)]
pub(crate) mod alpha_tensor;
mod copyline;
mod grid;
pub mod ksg;
pub(crate) mod pathdecomposition;
mod traits;
pub mod triangular;
mod weighted;

// Re-export commonly used items from submodules for convenience
pub use ksg::{GridKind, MappingResult};

// Re-exports for unit tests (only needed in test builds)
#[cfg(test)]
pub(crate) use copyline::{
    copyline_weighted_locations_triangular, create_copylines, mis_overhead_copyline, CopyLine,
};
#[cfg(test)]
pub(crate) use grid::{CellState, MappingGrid};
#[cfg(test)]
pub(crate) use traits::{apply_gadget, unapply_gadget, Pattern};
#[cfg(test)]
pub(crate) use weighted::{map_weights, trace_centers};

// Hidden re-exports for development tools (examples/export_mapping_stages.rs)
#[doc(hidden)]
pub mod _internal {
    pub use super::copyline::{
        create_copylines, mis_overhead_copyline, mis_overhead_copyline_triangular, CopyLine,
    };
    pub use super::grid::{CellState, MappingGrid};
}
