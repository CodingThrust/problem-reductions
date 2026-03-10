//! Problem model implementations.
//!
//! Each sub-module groups related problem types by input structure.

pub mod algebraic;
pub mod formula;
pub mod graph;
pub mod misc;
pub mod set;

// Re-export commonly used types
pub use algebraic::{ClosestVectorProblem, BMF, ILP, QUBO};
pub use formula::{CNFClause, CircuitSAT, KSatisfiability, Satisfiability};
pub use graph::{
    BicliqueCover, KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinimumDominatingSet, MinimumVertexCover, SpinGlass, TravelingSalesman,
};
pub use misc::{BinPacking, Factoring, Knapsack, PaintShop, SequencingWithinIntervals};
pub use set::{MaximumSetPacking, MinimumSetCovering};
