//! Tests for the mapping module (src/rules/mapping/).
//!
//! This mirrors the source structure:
//! - map_graph.rs - tests for map_graph functionality
//! - triangular.rs - tests for triangular lattice mapping
//! - gadgets.rs - tests for gadget properties
//! - copyline.rs - tests for copyline functionality
//! - weighted.rs - tests for weighted mode
//! - mapping_result.rs - tests for MappingResult utility methods

mod common;
mod copyline;
mod gadgets;
mod gadgets_ground_truth;
mod julia_comparison;
mod map_graph;
mod mapping_result;
mod triangular;
mod weighted;
