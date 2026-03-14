//! Problem model implementations.
//!
//! Each sub-module groups related problem types by input structure.

pub mod algebraic;
pub mod formula;
pub mod graph;
pub mod misc;
pub mod set;

// Re-export commonly used types
pub use algebraic::{ClosestVectorProblem, QuadraticAssignment, BMF, ILP, QUBO};
pub use formula::{CNFClause, CircuitSAT, KSatisfiability, Satisfiability};
pub use graph::{
    BicliqueCover, GraphPartitioning, HamiltonianPath, IsomorphicSpanningTree, KColoring, MaxCut,
    MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching, MinimumDominatingSet,
    MinimumFeedbackArcSet, MinimumFeedbackVertexSet, MinimumSumMulticenter, MinimumVertexCover,
    PartitionIntoTriangles, RuralPostman, SpinGlass, SubgraphIsomorphism, TravelingSalesman,
};
pub use misc::{
    BinPacking, Factoring, FlowShopScheduling, Knapsack, LongestCommonSubsequence, PaintShop,
    ShortestCommonSupersequence, SubsetSum,
};
pub use set::{MaximumSetPacking, MinimumSetCovering};
