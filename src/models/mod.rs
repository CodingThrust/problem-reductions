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
    BicliqueCover, DirectedTwoCommodityIntegralFlow, GraphPartitioning, HamiltonianPath,
    IsomorphicSpanningTree, KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinimumDominatingSet, MinimumFeedbackArcSet, MinimumFeedbackVertexSet,
    MinimumSumMulticenter, MinimumVertexCover, OptimalLinearArrangement, PartitionIntoTriangles,
    RuralPostman, SpinGlass, SteinerTree, SubgraphIsomorphism, TravelingSalesman,
};
pub use misc::{
    BinPacking, Factoring, FlowShopScheduling, Knapsack, LongestCommonSubsequence,
    MinimumTardinessSequencing, PaintShop, ShortestCommonSupersequence, SubsetSum,
};
pub use set::{ExactCoverBy3Sets, MaximumSetPacking, MinimumSetCovering};
