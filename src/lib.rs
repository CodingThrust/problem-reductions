//! # Problem Reductions
//!
//! NP-hard problem definitions and reductions.
//! See the [user guide](https://codingthrust.github.io/problem-reductions/) for tutorials and examples.
//!
//! ## API Overview
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`models`] | Problem types — [`graph`](models::graph), [`formula`](models::formula), [`set`](models::set), [`algebraic`](models::algebraic), [`misc`](models::misc) |
//! | [`rules`] | Reduction rules, [`ReductionGraph`](rules::ReductionGraph) for path search |
//! | [`solvers`] | [`BruteForce`] and [`ILPSolver`](solvers::ILPSolver) |
//! | [`topology`] | Graph types — [`SimpleGraph`](topology::SimpleGraph), [`UnitDiskGraph`](topology::UnitDiskGraph), etc. |
//! | [`traits`] | Core traits — [`Problem`] |
//! | [`types`] | [`Max`], [`Min`], [`Extremum`], [`ExtremumSense`], [`ProblemSize`], [`WeightElement`] |
//! | [`variant`] | Variant parameter system for problem type parameterization |
//!
//! Use [`prelude`] for convenient imports.

extern crate self as problemreductions;

pub(crate) mod big_o;
pub mod config;
pub mod error;
#[cfg(feature = "example-db")]
pub mod example_db;
pub mod export;
pub mod expr;
// Growth is an explicit terminal projection for complexity display. Exact and certified
// size propagation never re-enters this domain.
pub mod growth;
pub mod io;
pub mod models;
pub mod random;
pub mod registry;
pub mod rules;
pub mod size;
pub mod solvers;
pub mod topology;
pub mod traits;
#[allow(dead_code)]
pub(crate) mod truth_table;
pub mod types;
pub mod variant;

/// Prelude module for convenient imports.
pub mod prelude {
    // Problem types
    pub use crate::models::algebraic::{
        AlgebraicEquationsOverGF2, ConsecutiveOnesMatrixAugmentation,
        MinimumWeightSolutionToLinearEquations, QuadraticAssignment, QuadraticCongruences,
        SimultaneousIncongruences, SparseMatrixCompression, BMF, QUBO,
    };
    pub use crate::models::formula::{
        CNFClause, CircuitSAT, KSatisfiability, Maximum2Satisfiability, NAESatisfiability,
        NonTautology, OneInThreeSatisfiability, Planar3Satisfiability, QuantifiedBooleanFormulas,
        Satisfiability,
    };
    pub use crate::models::graph::{
        AcyclicPartition, BalancedCompleteBipartiteSubgraph, BicliqueCover,
        BiconnectivityAugmentation, BottleneckTravelingSalesman, BoundedComponentSpanningForest,
        DegreeConstrainedSpanningTree, DirectedTwoCommodityIntegralFlow, DisjointConnectingPaths,
        GeneralizedHex, GraphPartitioning, HamiltonianCircuit, HamiltonianPath,
        HamiltonianPathBetweenTwoVertices, IntegralFlowBundles, IntegralFlowHomologousArcs,
        IntegralFlowWithMultipliers, IsomorphicSpanningTree, KClique, Kernel, KthBestSpanningTree,
        LengthBoundedDisjointPaths, LongestPath, MixedChinesePostman, SpinGlass, SteinerTree,
        StrongConnectivityAugmentation, SubgraphIsomorphism,
    };
    pub use crate::models::graph::{
        KColoring, LongestCircuit, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
        MaximumLeafSpanningTree, MaximumMatching, MinMaxMulticenter, MinimumCutIntoBoundedSets,
        MinimumDominatingSet, MinimumDummyActivitiesPert, MinimumFeedbackArcSet,
        MinimumFeedbackVertexSet, MinimumGeometricConnectedDominatingSet, MinimumGraphBandwidth,
        MinimumMultiwayCut, MinimumSumMulticenter, MinimumVertexCover, MonochromaticTriangle,
        MultipleChoiceBranching, MultipleCopyFileAllocation, OptimalLinearArrangement,
        PartialFeedbackEdgeSet, PartitionIntoCliques, PartitionIntoPathsOfLength2,
        PartitionIntoTriangles, PathConstrainedNetworkFlow, RootedTreeArrangement, RuralPostman,
        ShortestWeightConstrainedPath, SteinerTreeInGraphs, TravelingSalesman,
        UndirectedFlowLowerBounds, UndirectedTwoCommodityIntegralFlow,
    };
    pub use crate::models::misc::{
        AdditionalKey, BinPacking, BoyceCoddNormalFormViolation, CapacityAssignment, CbqRelation,
        ConjunctiveBooleanQuery, ConjunctiveQueryFoldability, ConsistencyOfDatabaseFrequencyTables,
        CosineProductIntegration, EnsembleComputation, ExpectedRetrievalCost, Factoring,
        FlowShopScheduling, GroupingBySwapping, IntegerExpressionMembership, JobShopScheduling,
        Knapsack, LongestCommonSubsequence, MinimumTardinessSequencing, MultiprocessorScheduling,
        OpenShopScheduling, PaintShop, Partition, PreemptiveScheduling, ProductionPlanning,
        QueryArg, RectilinearPictureCompression, ResourceConstrainedScheduling,
        SchedulingWithIndividualDeadlines, SequencingToMinimizeMaximumCumulativeCost,
        SequencingToMinimizeTardyTaskWeight, SequencingToMinimizeWeightedCompletionTime,
        SequencingToMinimizeWeightedTardiness, SequencingWithDeadlinesAndSetUpTimes,
        SequencingWithReleaseTimesAndDeadlines, SequencingWithinIntervals,
        ShortestCommonSupersequence, StackerCrane, StaffScheduling, StringToStringCorrection,
        SubsetProduct, SubsetSum, SumOfSquaresPartition, Term, ThreePartition, TimetableDesign,
    };
    pub use crate::models::set::{
        ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, IntegerKnapsack,
        MaximumSetPacking, MinimumCardinalityKey, MinimumHittingSet, MinimumSetCovering,
        PrimeAttributeName, RootedTreeStorageAssignment, SetBasis, SetSplitting,
        ThreeMatroidIntersection,
    };

    // Core traits
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::solvers::BruteForce;
    pub use crate::traits::Problem;

    // Types
    pub use crate::error::{ProblemError, Result};
    pub use crate::types::{And, Extremum, ExtremumSense, Max, Min, One, Or, ProblemSize, Sum};
}

// Re-export commonly used items at crate root
pub use big_o::big_o_normal_form;
pub use error::{ProblemError, Result};
pub use expr::{
    evaluate_approximate, ApproximationError, AsymptoticAnalysisError, Expr, ParseError,
};
pub use growth::Growth;
pub use registry::{ComplexityClass, ProblemInfo};
pub use solvers::BruteForce;
pub use traits::Problem;
pub use types::{
    And, Extremum, ExtremumSense, Max, Min, NumericSize, One, Or, ProblemSize, Sum, WeightElement,
};

// Re-export proc macros for reduction registration and variant declaration
pub use problemreductions_macros::{declare_variants, reduction, register_brute_force, CreateSpec};

// Re-export inventory so `declare_variants!` can use `$crate::inventory::submit!`
pub use inventory;

#[cfg(all(test, feature = "example-db"))]
#[path = "unit_tests/symbolic_size_contracts.rs"]
mod symbolic_size_contracts;
#[cfg(test)]
#[path = "unit_tests/graph_models.rs"]
mod test_graph_models;
#[cfg(test)]
#[path = "unit_tests/prelude.rs"]
mod test_prelude;
#[cfg(test)]
#[path = "unit_tests/problem_size.rs"]
mod test_problem_size;
#[cfg(test)]
#[path = "unit_tests/property.rs"]
mod test_property;
#[cfg(test)]
#[path = "unit_tests/reduction_graph.rs"]
mod test_reduction_graph;
#[cfg(test)]
#[path = "unit_tests/unitdiskmapping_algorithms/mod.rs"]
mod test_unitdiskmapping_algorithms;
