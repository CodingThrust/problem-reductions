//! Problem model implementations.
//!
//! This module contains implementations of various NP-hard problems.
//!
//! # Problem Categories
//!
//! - **Satisfiability**: SAT, K-SAT, CircuitSAT, Factoring
//! - **Graph**: IndependentSet, MaximalIS, VertexCovering, DominatingSet, Coloring, Matching
//! - **Set**: SetCovering, SetPacking
//! - **Optimization**: MaxCut, SpinGlass, QUBO
//! - **Specialized**: Paintshop, BicliqueCover, BMF

pub mod graph;
pub mod optimization;
pub mod satisfiability;
pub mod set;
pub mod specialized;

// Re-export commonly used types
pub use graph::{
    Coloring, DominatingSet, IndependentSet, Matching, MaxCut, MaximalIS, VertexCovering,
};
pub use optimization::{SpinGlass, QUBO};
pub use satisfiability::{CNFClause, Satisfiability};
pub use set::{SetCovering, SetPacking};
pub use specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
