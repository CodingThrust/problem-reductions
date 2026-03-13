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
    BicliqueCover, GraphPartitioning, KColoring, MaxCut, MaximalIS, MaximumClique,
    MaximumIndependentSet, MaximumMatching, MinimumDominatingSet, MinimumFeedbackVertexSet,
    MinimumVertexCover, RuralPostman, SpinGlass, TravelingSalesman,
};
pub use misc::{BinPacking, Factoring, Knapsack, LongestCommonSubsequence, PaintShop, SubsetSum};
pub use set::{MaximumSetPacking, MinimumSetCovering};
