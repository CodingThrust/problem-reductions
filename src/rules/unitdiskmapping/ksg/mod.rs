//! King's Subgraph (KSG) mapping module.
//!
//! Maps arbitrary graphs to King's Subgraph (8-connected grid graphs).
//! Supports both unweighted and weighted modes.
//!
//! # Example
//!
//! ```rust,ignore
//! use problemreductions::rules::unitdiskmapping::ksg;
//!
//! let edges = vec![(0, 1), (1, 2), (0, 2)];
//!
//! // Unweighted mapping
//! let result = ksg::map_unweighted(3, &edges);
//!
//! // Weighted mapping
//! let weighted_result = ksg::map_weighted(3, &edges);
//! ```

pub mod gadgets;
pub mod gadgets_weighted;
pub mod mapping;

// Re-export all public items for convenient access
pub use gadgets::{
    apply_crossing_gadgets, apply_simplifier_gadgets, crossing_ruleset_indices,
    tape_entry_mis_overhead, KsgBranch, KsgBranchFix, KsgBranchFixB, KsgCross, KsgDanglingLeg,
    KsgEndTurn, KsgPattern, KsgPatternBoxed, KsgReflectedGadget, KsgRotatedGadget, KsgTCon,
    KsgTapeEntry, KsgTrivialTurn, KsgTurn, KsgWTurn, Mirror,
};

pub use gadgets_weighted::{
    apply_weighted_crossing_gadgets, apply_weighted_simplifier_gadgets,
    weighted_tape_entry_mis_overhead, WeightedKsgBranch, WeightedKsgBranchFix,
    WeightedKsgBranchFixB, WeightedKsgCross, WeightedKsgDanglingLeg, WeightedKsgEndTurn,
    WeightedKsgPattern, WeightedKsgTCon, WeightedKsgTapeEntry, WeightedKsgTrivialTurn,
    WeightedKsgTurn, WeightedKsgWTurn,
};

pub use mapping::{
    embed_graph, map_config_copyback, map_unweighted, map_unweighted_with_method,
    map_unweighted_with_order, map_weighted, map_weighted_with_method, map_weighted_with_order,
    trace_centers, unapply_gadgets, unapply_weighted_gadgets, GridKind, MappingResult,
};

/// Spacing between copy lines for KSG mapping.
pub const SPACING: usize = 4;

/// Padding around the grid for KSG mapping.
pub const PADDING: usize = 2;
