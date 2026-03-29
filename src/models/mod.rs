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
    ClosestVectorProblem, ConsecutiveBlockMinimization, ConsecutiveOnesMatrixAugmentation,
    ConsecutiveOnesSubmatrix, QuadraticAssignment, SparseMatrixCompression, BMF, ILP, QUBO,
};
pub use formula::{
    CNFClause, CircuitSAT, KSatisfiability, NAESatisfiability, QuantifiedBooleanFormulas,
    Quantifier, Satisfiability,
};
pub use graph::{
    AcyclicPartition, BalancedCompleteBipartiteSubgraph, BicliqueCover, BiconnectivityAugmentation,
    BottleneckTravelingSalesman, BoundedComponentSpanningForest, DirectedTwoCommodityIntegralFlow,
    DisjointConnectingPaths, GeneralizedHex, HamiltonianCircuit, HamiltonianPath,
    IntegralFlowBundles, IntegralFlowHomologousArcs, IntegralFlowWithMultipliers,
    IsomorphicSpanningTree, KClique, KColoring, KthBestSpanningTree, LengthBoundedDisjointPaths,
    LongestCircuit, LongestPath, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinMaxMulticenter, MinimumCutIntoBoundedSets, MinimumDominatingSet,
    MinimumDummyActivitiesPert, MinimumFeedbackArcSet, MinimumFeedbackVertexSet,
    MinimumMultiwayCut, MinimumSumMulticenter, MinimumVertexCover, MixedChinesePostman,
    MultipleChoiceBranching, MultipleCopyFileAllocation, OptimalLinearArrangement,
    PartialFeedbackEdgeSet, PartitionIntoPathsOfLength2, PartitionIntoTriangles,
    PathConstrainedNetworkFlow, RootedTreeArrangement, RuralPostman, ShortestWeightConstrainedPath,
    SpinGlass, SteinerTree, SteinerTreeInGraphs, StrongConnectivityAugmentation,
    SubgraphIsomorphism, TravelingSalesman, UndirectedFlowLowerBounds,
    UndirectedTwoCommodityIntegralFlow,
};
pub use misc::PartiallyOrderedKnapsack;
pub use misc::{
    AdditionalKey, BinPacking, CapacityAssignment, CbqRelation, ConjunctiveBooleanQuery,
    ConjunctiveQueryFoldability, ConsistencyOfDatabaseFrequencyTables, CosineProductIntegration,
    EnsembleComputation, ExpectedRetrievalCost, Factoring, FlowShopScheduling, GroupingBySwapping,
    JobShopScheduling, Knapsack, KthLargestMTuple, LongestCommonSubsequence,
    MinimumTardinessSequencing, MultiprocessorScheduling, PaintShop, Partition,
    PrecedenceConstrainedScheduling, ProductionPlanning, QueryArg, RectilinearPictureCompression,
    ResourceConstrainedScheduling,
    SchedulingWithIndividualDeadlines, SequencingToMinimizeMaximumCumulativeCost,
    SequencingToMinimizeWeightedCompletionTime, SequencingToMinimizeWeightedTardiness,
    SequencingWithReleaseTimesAndDeadlines, SequencingWithinIntervals, ShortestCommonSupersequence,
    StackerCrane, StaffScheduling, StringToStringCorrection, SubsetSum, SumOfSquaresPartition,
    Term, ThreePartition, TimetableDesign,
};
pub use set::{
    ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, MaximumSetPacking,
    MinimumCardinalityKey, MinimumHittingSet, MinimumSetCovering, PrimeAttributeName,
    RootedTreeStorageAssignment, SetBasis, TwoDimensionalConsecutiveSets,
};
