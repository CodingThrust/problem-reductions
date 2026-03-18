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
    HamiltonianCircuit, HamiltonianPath, IsomorphicSpanningTree, KColoring,
    LengthBoundedDisjointPaths, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinimumDominatingSet, MinimumFeedbackArcSet, MinimumFeedbackVertexSet,
    MinimumMultiwayCut, MinimumSumMulticenter, MinimumVertexCover, MultipleChoiceBranching,
    OptimalLinearArrangement, PartitionIntoTriangles, RuralPostman, SpinGlass, SteinerTree,
    SubgraphIsomorphism, TravelingSalesman, UndirectedTwoCommodityIntegralFlow,
};
pub use misc::{
    BinPacking, Factoring, FlowShopScheduling, Knapsack, LongestCommonSubsequence,
    MinimumTardinessSequencing, MultiprocessorScheduling, PaintShop,
    PrecedenceConstrainedScheduling, SequencingWithReleaseTimesAndDeadlines,
    SequencingWithinIntervals, ShortestCommonSupersequence,
    StaffScheduling, StringToStringCorrection, SubsetSum, SumOfSquaresPartition,
};
pub use set::{
    ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, MaximumSetPacking,
    MinimumCardinalityKey, MinimumSetCovering, SetBasis,
};
