//! Problem model implementations.
//!
//! Each sub-module groups related problem types. See individual modules for details.

pub mod graph;
pub mod optimization;
pub mod satisfiability;
pub mod set;
pub mod specialized;

// Re-export commonly used types
pub use graph::{
    KColoring, MaxCut, MaximalIS, MaximumIndependentSet, MaximumMatching, MinimumDominatingSet,
    MinimumVertexCover, TravelingSalesman,
};
pub use optimization::{SpinGlass, QUBO};
pub use satisfiability::{CNFClause, Satisfiability};
pub use set::{MaximumSetPacking, MinimumSetCovering};
pub use specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
