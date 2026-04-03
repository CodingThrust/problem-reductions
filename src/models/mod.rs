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
    CNFClause, CircuitSAT, KSatisfiability, NAESatisfiability, NonTautology,
    QuantifiedBooleanFormulas, Quantifier, Satisfiability,
};
pub use graph::{
    AcyclicPartition, BalancedCompleteBipartiteSubgraph, BicliqueCover, BiconnectivityAugmentation,
    BottleneckTravelingSalesman, BoundedComponentSpanningForest, DegreeConstrainedSpanningTree,
    DirectedTwoCommodityIntegralFlow, DisjointConnectingPaths, GeneralizedHex, GraphPartitioning,
    HamiltonianCircuit, HamiltonianPath, IntegralFlowBundles, IntegralFlowHomologousArcs,
    IntegralFlowWithMultipliers, IsomorphicSpanningTree, KClique, KColoring, Kernel,
    KthBestSpanningTree, LengthBoundedDisjointPaths, LongestCircuit, LongestPath, MaxCut,
    MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching, MinMaxMulticenter,
    MinimumCutIntoBoundedSets, MinimumDominatingSet, MinimumDummyActivitiesPert,
    MinimumFeedbackArcSet, MinimumFeedbackVertexSet, MinimumMultiwayCut, MinimumSumMulticenter,
    MinimumVertexCover, MixedChinesePostman, MultipleChoiceBranching, MultipleCopyFileAllocation,
    OptimalLinearArrangement, PartialFeedbackEdgeSet, PartitionIntoCliques,
    PartitionIntoPathsOfLength2, PartitionIntoTriangles, PathConstrainedNetworkFlow,
    RootedTreeArrangement, RuralPostman, ShortestWeightConstrainedPath, SpinGlass, SteinerTree,
    SteinerTreeInGraphs, StrongConnectivityAugmentation, SubgraphIsomorphism, TravelingSalesman,
    UndirectedFlowLowerBounds, UndirectedTwoCommodityIntegralFlow,
};
pub use misc::PartiallyOrderedKnapsack;
pub use misc::{
    AdditionalKey, BinPacking, CapacityAssignment, CbqRelation, ConjunctiveBooleanQuery,
    ConjunctiveQueryFoldability, ConsistencyOfDatabaseFrequencyTables, EnsembleComputation,
    ExpectedRetrievalCost, Factoring, FlowShopScheduling, GroupingBySwapping, Knapsack,
    LongestCommonSubsequence, MinimumTardinessSequencing, MultiprocessorScheduling, PaintShop,
    Partition, PrecedenceConstrainedScheduling, QueryArg, RectilinearPictureCompression,
    ResourceConstrainedScheduling, SchedulingWithIndividualDeadlines,
    SequencingToMinimizeMaximumCumulativeCost, SequencingToMinimizeWeightedCompletionTime,
    SequencingToMinimizeWeightedTardiness, SequencingWithReleaseTimesAndDeadlines,
    SequencingWithinIntervals, ShortestCommonSupersequence, StackerCrane, StaffScheduling,
    StringToStringCorrection, SubsetProduct, SubsetSum, SumOfSquaresPartition, Term,
    TimetableDesign,
};
pub use set::{
    ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, MaximumSetPacking,
    MinimumCardinalityKey, MinimumHittingSet, MinimumSetCovering, PrimeAttributeName,
    RootedTreeStorageAssignment, SetBasis, SetSplitting, TwoDimensionalConsecutiveSets,
};
