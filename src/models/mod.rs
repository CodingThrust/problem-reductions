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
    AlgebraicEquationsOverGF2, ClosestVectorProblem, ConsecutiveBlockMinimization,
    ConsecutiveOnesMatrixAugmentation, ConsecutiveOnesSubmatrix, EquilibriumPoint,
    FeasibleBasisExtension, MinimumMatrixCover, MinimumMatrixDomination, MinimumWeightDecoding,
    MinimumWeightSolutionToLinearEquations, QuadraticAssignment, QuadraticCongruences,
    QuadraticDiophantineEquations, SimultaneousIncongruences, SparseMatrixCompression, BMF, ILP,
    QUBO,
};
pub use formula::{
    CNFClause, CircuitSAT, KSatisfiability, Maximum2Satisfiability, NAESatisfiability,
    NonTautology, OneInThreeSatisfiability, Planar3Satisfiability, QuantifiedBooleanFormulas,
    Quantifier, Satisfiability,
};
pub use graph::{
    AcyclicPartition, BalancedCompleteBipartiteSubgraph, BicliqueCover, BiconnectivityAugmentation,
    BottleneckTravelingSalesman, BoundedComponentSpanningForest, BoundedDiameterSpanningTree,
    DegreeConstrainedSpanningTree, DirectedHamiltonianPath, DirectedTwoCommodityIntegralFlow,
    DisjointConnectingPaths, GeneralizedHex, HamiltonianCircuit, HamiltonianPath,
    HamiltonianPathBetweenTwoVertices, IntegralFlowBundles, IntegralFlowHomologousArcs,
    IntegralFlowWithMultipliers, IsomorphicSpanningTree, KClique, KColoring, Kernel,
    KthBestSpanningTree, LengthBoundedDisjointPaths, LongestCircuit, LongestPath, MaxCut,
    MaximalIS, MaximumAchromaticNumber, MaximumClique, MaximumDomaticNumber, MaximumIndependentSet,
    MaximumLeafSpanningTree, MaximumMatching, MinMaxMulticenter, MinimumCoveringByCliques,
    MinimumCutIntoBoundedSets, MinimumDominatingSet, MinimumDummyActivitiesPert,
    MinimumEdgeCostFlow, MinimumFeedbackArcSet, MinimumFeedbackVertexSet,
    MinimumGeometricConnectedDominatingSet, MinimumGraphBandwidth, MinimumIntersectionGraphBasis,
    MinimumMaximalMatching, MinimumMultiwayCut, MinimumSumMulticenter, MinimumVertexCover,
    MixedChinesePostman, MonochromaticTriangle, MultipleChoiceBranching,
    MultipleCopyFileAllocation, OptimalLinearArrangement, PartialFeedbackEdgeSet,
    PartitionIntoCliques, PartitionIntoForests, PartitionIntoPathsOfLength2,
    PartitionIntoPerfectMatchings, PartitionIntoTriangles, PathConstrainedNetworkFlow,
    RootedTreeArrangement, RuralPostman, ShortestWeightConstrainedPath, SpinGlass, SteinerTree,
    SteinerTreeInGraphs, StrongConnectivityAugmentation, SubgraphIsomorphism, TravelingSalesman,
    UndirectedFlowLowerBounds, UndirectedTwoCommodityIntegralFlow,
};
pub use misc::PartiallyOrderedKnapsack;
pub use misc::{
    AdditionalKey, Betweenness, BinPacking, CapacityAssignment, CbqRelation, Clustering,
    ConjunctiveBooleanQuery, ConjunctiveQueryFoldability, ConsistencyOfDatabaseFrequencyTables,
    CosineProductIntegration, CyclicOrdering, DynamicStorageAllocation, EnsembleComputation,
    ExpectedRetrievalCost, Factoring, FeasibleRegisterAssignment, FlowShopScheduling,
    GroupingBySwapping, IntExpr, IntegerExpressionMembership, JobShopScheduling, Knapsack,
    KthLargestMTuple, LongestCommonSubsequence, MaximumLikelihoodRanking, MinimumAxiomSet,
    MinimumExternalMacroDataCompression, MinimumFaultDetectionTestSet,
    MinimumInternalMacroDataCompression, MinimumRegisterSufficiencyForLoops,
    MinimumTardinessSequencing, MinimumWeightAndOrGraph, MultiprocessorScheduling,
    NonLivenessFreePetriNet, Numerical3DimensionalMatching, NumericalMatchingWithTargetSums,
    OpenShopScheduling, OptimumCommunicationSpanningTree, PaintShop, Partition,
    PrecedenceConstrainedScheduling, PreemptiveScheduling, ProductionPlanning, QueryArg,
    RectilinearPictureCompression, RegisterSufficiency, ResourceConstrainedScheduling,
    SchedulingToMinimizeWeightedCompletionTime, SchedulingWithIndividualDeadlines,
    SequencingToMinimizeMaximumCumulativeCost, SequencingToMinimizeTardyTaskWeight,
    SequencingToMinimizeWeightedCompletionTime, SequencingToMinimizeWeightedTardiness,
    SequencingWithDeadlinesAndSetUpTimes, SequencingWithReleaseTimesAndDeadlines,
    SequencingWithinIntervals, ShortestCommonSupersequence, SquareTiling, StackerCrane,
    StaffScheduling, StringToStringCorrection, SubsetProduct, SubsetSum, SumOfSquaresPartition,
    Term, ThreePartition, TimetableDesign,
};
pub use set::{
    ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, IntegerKnapsack, MaximumSetPacking,
    MinimumCardinalityKey, MinimumHittingSet, MinimumSetCovering, PrimeAttributeName,
    RootedTreeStorageAssignment, SetBasis, SetSplitting, ThreeDimensionalMatching,
    TwoDimensionalConsecutiveSets,
};
