//! Graph to grid graph mapping.
//!
//! This module implements reductions from arbitrary graphs to unit disk grid graphs
//! using the copy-line technique from UnitDiskMapping.jl.
//!
//! # Overview
//!
//! The mapping works by:
//! 1. Creating "copy lines" for each vertex (L-shaped paths on the grid)
//! 2. Resolving crossings using gadgets that preserve MIS properties
//! 3. The resulting grid graph has the property that a MIS solution can be
//!    mapped back to a MIS solution on the original graph
//!
//! # Example
//!
//! ```rust
//! use problemreductions::rules::mapping::{map_graph, map_graph_triangular};
//! use problemreductions::topology::Graph;
//!
//! // Map a triangle graph to a square lattice
//! let edges = vec![(0, 1), (1, 2), (0, 2)];
//! let result = map_graph(3, &edges);
//!
//! println!("Grid graph has {} vertices", result.grid_graph.num_vertices());
//! println!("MIS overhead: {}", result.mis_overhead);
//!
//! // Map the same graph to a triangular lattice
//! let tri_result = map_graph_triangular(3, &edges);
//! println!("Triangular grid has {} vertices", tri_result.grid_graph.num_vertices());
//! ```
//!
//! # Submodules
//!
//! - `copyline`: Copy line creation and manipulation
//! - `gadgets`: Crossing gadgets for resolving line intersections
//! - `grid`: Grid representation and cell state management
//! - `map_graph`: Main mapping functions for square lattices
//! - `triangular`: Mapping functions for triangular lattices

mod copyline;
mod gadgets;
mod grid;
mod map_graph;
pub mod pathdecomposition;
mod triangular;
mod weighted;

pub use copyline::{
    copyline_weighted_locations_triangular, create_copylines, mis_overhead_copyline,
    mis_overhead_copyline_triangular, remove_order, CopyLine,
};
pub use gadgets::{
    apply_crossing_gadgets, apply_gadget, apply_simplifier_gadgets, pattern_matches,
    tape_entry_mis_overhead, unapply_gadget, Branch, BranchFix, BranchFixB, Cross, DanglingLeg,
    EndTurn, Mirror, Pattern, PatternBoxed, PatternCell, ReflectedGadget, RotatedGadget, TCon,
    TapeEntry, TrivialTurn, Turn, WTurn,
};
pub use grid::{CellState, MappingGrid};
pub use map_graph::{embed_graph, map_graph, map_graph_with_method, map_graph_with_order, MappingResult};
pub use pathdecomposition::{pathwidth, Layout, PathDecompositionMethod};
pub use triangular::{
    map_graph_triangular, map_graph_triangular_with_method, map_graph_triangular_with_order,
    TriangularGadget, TriBranch, TriBranchFix, TriBranchFixB, TriCross, TriEndTurn, TriTConDown,
    TriTConLeft, TriTConUp, TriTrivialTurnLeft, TriTrivialTurnRight, TriTurn, TriWTurn,
};
pub use weighted::{
    map_weights, trace_centers, triangular_weighted_ruleset, WeightedGadget,
    WeightedTriangularGadget, Weightable,
};
