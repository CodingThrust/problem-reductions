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
    ClosestVectorProblem, ConsecutiveBlockMinimization, ConsecutiveOnesSubmatrix,
    QuadraticAssignment, BMF, ILP, QUBO,
};
pub use formula::{
    CNFClause, CircuitSAT, KSatisfiability, QuantifiedBooleanFormulas, Quantifier, Satisfiability,
};
pub use graph::{
    BalancedCompleteBipartiteSubgraph, BicliqueCover, BiconnectivityAugmentation,
    BoundedComponentSpanningForest, DirectedTwoCommodityIntegralFlow, GeneralizedHex,
    GraphPartitioning, HamiltonianCircuit, HamiltonianPath, IsomorphicSpanningTree, KColoring,
    KthBestSpanningTree, LengthBoundedDisjointPaths, MaxCut, MaximalIS, MaximumClique,
    MaximumIndependentSet, MaximumMatching, MinimumCutIntoBoundedSets, MinimumDominatingSet,
    MinimumFeedbackArcSet, MinimumFeedbackVertexSet, MinimumMultiwayCut, MinimumSumMulticenter,
    MinimumVertexCover, MultipleChoiceBranching, MultipleCopyFileAllocation,
    OptimalLinearArrangement, PartitionIntoPathsOfLength2, PartitionIntoTriangles, RuralPostman,
    SpinGlass, SteinerTree, SteinerTreeInGraphs, StrongConnectivityAugmentation,
    SubgraphIsomorphism, TravelingSalesman, UndirectedTwoCommodityIntegralFlow,
};
pub use misc::PartiallyOrderedKnapsack;
pub use misc::{
    AdditionalKey, BinPacking, CbqRelation, ConjunctiveBooleanQuery, ConjunctiveQueryFoldability,
    Factoring, FlowShopScheduling, Knapsack, LongestCommonSubsequence, MinimumTardinessSequencing,
    MultiprocessorScheduling, PaintShop, Partition, PrecedenceConstrainedScheduling, QueryArg,
    RectilinearPictureCompression, ResourceConstrainedScheduling,
    SchedulingWithIndividualDeadlines, SequencingToMinimizeMaximumCumulativeCost,
    SequencingToMinimizeWeightedCompletionTime, SequencingToMinimizeWeightedTardiness,
    SequencingWithReleaseTimesAndDeadlines, SequencingWithinIntervals, ShortestCommonSupersequence,
    StaffScheduling, StringToStringCorrection, SubsetSum, SumOfSquaresPartition, Term,
    TimetableDesign,
};
pub use set::{
    ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, MaximumSetPacking,
    MinimumCardinalityKey, MinimumSetCovering, PrimeAttributeName, SetBasis,
    TwoDimensionalConsecutiveSets,
};
