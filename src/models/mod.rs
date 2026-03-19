//! Problem model implementations.
//!
//! Each sub-module groups related problem types by input structure.

pub mod algebraic;
pub mod formula;
pub mod graph;
pub mod misc;
pub mod set;

// Re-export commonly used types
pub use algebraic::{
    ClosestVectorProblem, ConsecutiveOnesSubmatrix, QuadraticAssignment, BMF, ILP, QUBO,
};
pub use formula::{CNFClause, CircuitSAT, KSatisfiability, Satisfiability};
pub use graph::{
    BalancedCompleteBipartiteSubgraph, BicliqueCover, BiconnectivityAugmentation,
    BoundedComponentSpanningForest, DirectedTwoCommodityIntegralFlow, GeneralizedHex,
    GraphPartitioning, HamiltonianCircuit, HamiltonianPath, IsomorphicSpanningTree, KColoring,
    KthBestSpanningTree, LengthBoundedDisjointPaths, MaxCut, MaximalIS, MaximumClique,
    MaximumIndependentSet, MaximumMatching, MinimumCutIntoBoundedSets, MinimumDominatingSet,
    MinimumFeedbackArcSet, MinimumFeedbackVertexSet, MinimumMultiwayCut, MinimumSumMulticenter,
    MinimumVertexCover, MultipleChoiceBranching, OptimalLinearArrangement,
    PartitionIntoPathsOfLength2, PartitionIntoTriangles, RuralPostman, SpinGlass, SteinerTree,
    StrongConnectivityAugmentation, SubgraphIsomorphism, TravelingSalesman,
    UndirectedTwoCommodityIntegralFlow,
};
pub use misc::PartiallyOrderedKnapsack;
pub use misc::{
    BinPacking, CbqRelation, ConjunctiveBooleanQuery, ConjunctiveQueryFoldability, Factoring,
    FlowShopScheduling, Knapsack, LongestCommonSubsequence, MinimumTardinessSequencing,
    MultiprocessorScheduling, PaintShop, Partition, PrecedenceConstrainedScheduling, QueryArg,
    RectilinearPictureCompression, ResourceConstrainedScheduling,
    SequencingToMinimizeMaximumCumulativeCost, SequencingWithReleaseTimesAndDeadlines,
    SequencingWithinIntervals, ShortestCommonSupersequence, StaffScheduling,
    StringToStringCorrection, SubsetSum, SumOfSquaresPartition, Term,
};
pub use set::{
    ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, MaximumSetPacking,
    MinimumCardinalityKey, MinimumSetCovering, PrimeAttributeName, SetBasis,
};
