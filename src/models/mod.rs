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
    BalancedCompleteBipartiteSubgraph, BicliqueCover, BiconnectivityAugmentation,
    BoundedComponentSpanningForest, DirectedTwoCommodityIntegralFlow, GraphPartitioning,
    HamiltonianPath, IsomorphicSpanningTree, KColoring, KthBestSpanningTree,
    LengthBoundedDisjointPaths, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinimumDominatingSet, MinimumFeedbackArcSet, MinimumFeedbackVertexSet,
    MinimumMultiwayCut, MinimumSumMulticenter, MinimumVertexCover, MultipleChoiceBranching,
    OptimalLinearArrangement, PartitionIntoTriangles, RuralPostman, SpinGlass, SteinerTree,
    StrongConnectivityAugmentation, SubgraphIsomorphism, TravelingSalesman,
    UndirectedTwoCommodityIntegralFlow,
};
pub use misc::{
    BinPacking, Factoring, FlowShopScheduling, Knapsack, LongestCommonSubsequence,
    MinimumTardinessSequencing, PaintShop, SequencingWithinIntervals, ShortestCommonSupersequence,
    SubsetSum,
};
pub use set::{ExactCoverBy3Sets, MaximumSetPacking, MinimumSetCovering, SetBasis};
