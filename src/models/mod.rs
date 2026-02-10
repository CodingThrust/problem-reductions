//! Problem model implementations.
//!
//! This module contains implementations of various NP-hard problems.
//!
//! # Problem Categories
//!
//! - **Satisfiability**: SAT, K-SAT, CircuitSAT, Factoring
//! - **Graph**: MaximumIndependentSet, MaximalIS, MinimumVertexCover, MinimumDominatingSet, KColoring, MaximumMatching
//! - **Set**: MinimumSetCovering, MaximumSetPacking
//! - **Optimization**: MaxCut, SpinGlass, QUBO
//! - **Specialized**: Paintshop, BicliqueCover, BMF

pub mod graph;
pub mod optimization;
pub mod satisfiability;
pub mod set;
pub mod specialized;

// Re-export commonly used types
pub use graph::{
    MinimumDominatingSet, MaximumIndependentSet, KColoring, MaximumMatching, MaxCut, MaximalIS, MinimumVertexCover,
};
pub use optimization::{SpinGlass, QUBO};
pub use satisfiability::{CNFClause, Satisfiability};
pub use set::{MinimumSetCovering, MaximumSetPacking};
pub use specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
