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
pub use ksg::MappingResult;

// ============================================================================
// BACKWARD COMPATIBILITY EXPORTS (deprecated - use ksg:: and triangular:: instead)
// ============================================================================

// Old function names pointing to new locations
pub use ksg::embed_graph;
pub use ksg::map_unweighted as map_graph;
pub use ksg::map_unweighted_with_method as map_graph_with_method;
pub use ksg::map_unweighted_with_order as map_graph_with_order;
pub use ksg::{PADDING as SQUARE_PADDING, SPACING as SQUARE_SPACING};

pub use triangular::map_weighted as map_graph_triangular;
pub use triangular::map_weighted_with_method as map_graph_triangular_with_method;
pub use triangular::map_weighted_with_order as map_graph_triangular_with_order;
pub use triangular::{PADDING as TRIANGULAR_PADDING, SPACING as TRIANGULAR_SPACING};

// Old gadget names
pub use ksg::{
    KsgBranch as Branch, KsgBranchFix as BranchFix, KsgBranchFixB as BranchFixB, KsgCross as Cross,
    KsgDanglingLeg as DanglingLeg, KsgEndTurn as EndTurn, KsgPattern as SquarePattern,
    KsgReflectedGadget as ReflectedGadget, KsgRotatedGadget as RotatedGadget, KsgTCon as TCon,
    KsgTapeEntry as TapeEntry, KsgTrivialTurn as TrivialTurn, KsgTurn as Turn, KsgWTurn as WTurn,
    Mirror,
};

pub use triangular::{
    WeightedTriBranch as TriBranch, WeightedTriBranchFix as TriBranchFix,
    WeightedTriBranchFixB as TriBranchFixB, WeightedTriCross as TriCross,
    WeightedTriEndTurn as TriEndTurn, WeightedTriTConDown as TriTConDown,
    WeightedTriTConLeft as TriTConLeft, WeightedTriTConUp as TriTConUp,
    WeightedTriTapeEntry as TriangularTapeEntry, WeightedTriTrivialTurnLeft as TriTrivialTurnLeft,
    WeightedTriTrivialTurnRight as TriTrivialTurnRight, WeightedTriTurn as TriTurn,
    WeightedTriWTurn as TriWTurn, WeightedTriangularGadget as TriangularGadget,
};

// Additional exports for weighted mode utilities
pub use copyline::{copyline_weighted_locations_triangular, mis_overhead_copyline_triangular};
pub use triangular::weighted_ruleset as triangular_weighted_ruleset;
pub use weighted::{map_weights, trace_centers, Weightable};

// KSG gadget application functions
pub use ksg::{
    apply_crossing_gadgets, apply_simplifier_gadgets, apply_weighted_crossing_gadgets,
    apply_weighted_simplifier_gadgets, tape_entry_mis_overhead, weighted_tape_entry_mis_overhead,
    WeightedKsgTapeEntry,
};

// KSG weighted gadget types for testing
pub use ksg::{
    WeightedKsgBranch, WeightedKsgBranchFix, WeightedKsgBranchFixB, WeightedKsgCross,
    WeightedKsgDanglingLeg, WeightedKsgEndTurn, WeightedKsgPattern, WeightedKsgTCon,
    WeightedKsgTrivialTurn, WeightedKsgTurn, WeightedKsgWTurn,
};

// Triangular gadget application functions
pub use triangular::{
    apply_crossing_gadgets as apply_triangular_crossing_gadgets,
    apply_simplifier_gadgets as apply_triangular_simplifier_gadgets,
    tape_entry_mis_overhead as triangular_tape_entry_mis_overhead,
};
