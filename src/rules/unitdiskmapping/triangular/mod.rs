//! Triangular lattice mapping module.
//!
//! Maps arbitrary graphs to weighted triangular lattice graphs.
//!
//! # Example
//!
//! ```rust,ignore
//! use problemreductions::rules::unitdiskmapping::triangular;
//!
//! let edges = vec![(0, 1), (1, 2), (0, 2)];
//! let result = triangular::map_weighted(3, &edges).unwrap();
//! ```

pub mod gadgets;
pub mod mapping;

pub use super::weighted::{map_weights, trace_centers};
pub use gadgets::{
    SourceCell, WeightedTriBranch, WeightedTriBranchFix, WeightedTriBranchFixB, WeightedTriCross,
    WeightedTriEndTurn, WeightedTriTConDown, WeightedTriTConLeft, WeightedTriTConUp,
    WeightedTriTapeEntry, WeightedTriTrivialTurnLeft, WeightedTriTrivialTurnRight, WeightedTriTurn,
    WeightedTriWTurn, WeightedTriangularGadget, apply_crossing_gadgets, apply_simplifier_gadgets,
    tape_entry_mis_overhead,
};
pub use mapping::{
    PADDING, SPACING, map_config_back, map_unit_weights, map_weighted, map_weighted_with_method,
    map_weighted_with_order,
};
