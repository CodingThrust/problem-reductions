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
    BicliqueCover, BoundedComponentSpanningForest, DirectedTwoCommodityIntegralFlow,
    GraphPartitioning, HamiltonianPath, IsomorphicSpanningTree, KColoring,
    LengthBoundedDisjointPaths, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinimumDominatingSet, MinimumFeedbackArcSet, MinimumFeedbackVertexSet,
    MinimumSumMulticenter, MinimumVertexCover, MultipleChoiceBranching, OptimalLinearArrangement,
    PartitionIntoTriangles, RuralPostman, SpinGlass, SteinerTree, SubgraphIsomorphism,
    TravelingSalesman, UndirectedTwoCommodityIntegralFlow,
};
pub use misc::{
    BinPacking, Factoring, FlowShopScheduling, Knapsack, LongestCommonSubsequence,
    MinimumTardinessSequencing, PaintShop, SequencingWithinIntervals, ShortestCommonSupersequence,
    SubsetSum,
};
pub use set::{
    ExactCoverBy3Sets, MaximumSetPacking, MinimumCardinalityKey, MinimumSetCovering, SetBasis,
};
