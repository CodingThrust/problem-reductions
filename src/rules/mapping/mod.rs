//! Graph to grid mapping functionality.
//!
//! This module provides tools for embedding arbitrary graphs into 2D grids
//! using the copy-line technique.

mod copyline;
mod gadgets;
mod grid;
mod map_graph;
mod triangular;

pub use copyline::{create_copylines, mis_overhead_copyline, remove_order, CopyLine};
pub use gadgets::*;
pub use grid::{CellState, MappingGrid};
pub use map_graph::{embed_graph, map_graph, map_graph_with_order, MappingResult};
pub use triangular::{
    map_graph_triangular, map_graph_triangular_with_order, TriBranch, TriCross, TriTurn,
};
