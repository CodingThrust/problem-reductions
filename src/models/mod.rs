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
    CNFClause, CircuitSAT, KSatisfiability, NAESatisfiability, QuantifiedBooleanFormulas,
    Quantifier, Satisfiability,
};
pub use graph::{
    BalancedCompleteBipartiteSubgraph, BicliqueCover, BiconnectivityAugmentation,
    BoundedComponentSpanningForest, DirectedTwoCommodityIntegralFlow, GeneralizedHex,
    GraphPartitioning, HamiltonianCircuit, HamiltonianPath, IsomorphicSpanningTree, KClique,
    KColoring, KthBestSpanningTree, LengthBoundedDisjointPaths, MaxCut, MaximalIS, MaximumClique,
    MaximumIndependentSet, MaximumMatching, MinMaxMulticenter, MinimumCutIntoBoundedSets,
    MinimumDominatingSet, MinimumFeedbackArcSet, MinimumFeedbackVertexSet, MinimumMultiwayCut,
    MinimumSumMulticenter, MinimumVertexCover, MultipleChoiceBranching, MultipleCopyFileAllocation,
    OptimalLinearArrangement, PartitionIntoPathsOfLength2, PartitionIntoTriangles, RuralPostman,
    ShortestWeightConstrainedPath, SpinGlass, SteinerTree, SteinerTreeInGraphs,
    StrongConnectivityAugmentation, SubgraphIsomorphism, TravelingSalesman,
    UndirectedTwoCommodityIntegralFlow,
};
pub use misc::PartiallyOrderedKnapsack;
pub use misc::{
    AdditionalKey, BinPacking, CbqRelation, ConjunctiveBooleanQuery, ConjunctiveQueryFoldability,
    ConsistencyOfDatabaseFrequencyTables, Factoring, FlowShopScheduling, Knapsack,
    LongestCommonSubsequence, MinimumTardinessSequencing, MultiprocessorScheduling, PaintShop,
    Partition, PrecedenceConstrainedScheduling, QueryArg, RectilinearPictureCompression,
    ResourceConstrainedScheduling, SchedulingWithIndividualDeadlines,
    SequencingToMinimizeMaximumCumulativeCost, SequencingToMinimizeWeightedCompletionTime,
    SequencingToMinimizeWeightedTardiness, SequencingWithReleaseTimesAndDeadlines,
    SequencingWithinIntervals, ShortestCommonSupersequence, StaffScheduling,
    StringToStringCorrection, SubsetSum, SumOfSquaresPartition, Term, TimetableDesign,
};
pub use set::{
    ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, MaximumSetPacking,
    MinimumCardinalityKey, MinimumHittingSet, MinimumSetCovering, PrimeAttributeName, SetBasis,
    TwoDimensionalConsecutiveSets,
};
