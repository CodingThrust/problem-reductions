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
//! | [`traits`] | Core traits — [`Problem`], [`OptimizationProblem`], [`SatisfactionProblem`] |
//! | [`types`] | [`SolutionSize`], [`Direction`], [`ProblemSize`], [`WeightElement`] |
//! | [`variant`] | Variant parameter system for problem type parameterization |
//!
//! Use [`prelude`] for convenient imports.

extern crate self as problemreductions;

pub(crate) mod big_o;
pub(crate) mod canonical;
pub mod config;
pub mod error;
#[cfg(feature = "example-db")]
pub mod example_db;
pub mod export;
pub(crate) mod expr;
pub mod io;
pub mod models;
pub mod registry;
pub mod rules;
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
    pub use crate::models::algebraic::{QuadraticAssignment, BMF, QUBO};
    pub use crate::models::formula::{
        CNFClause, CircuitSAT, KSatisfiability, NAESatisfiability, QuantifiedBooleanFormulas,
        Satisfiability,
    };
    pub use crate::models::graph::{
        AcyclicPartition, BalancedCompleteBipartiteSubgraph, BicliqueCover,
        BiconnectivityAugmentation, BottleneckTravelingSalesman, BoundedComponentSpanningForest,
        DirectedTwoCommodityIntegralFlow, GeneralizedHex, GraphPartitioning, HamiltonianCircuit,
        HamiltonianPath, IntegralFlowWithMultipliers, IsomorphicSpanningTree, KClique,
        KthBestSpanningTree, LengthBoundedDisjointPaths, MixedChinesePostman, SpinGlass,
        SteinerTree, StrongConnectivityAugmentation, SubgraphIsomorphism,
    };
    pub use crate::models::graph::{
        KColoring, LongestCircuit, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
        MaximumMatching, MinMaxMulticenter, MinimumCutIntoBoundedSets, MinimumDominatingSet,
        MinimumFeedbackArcSet, MinimumFeedbackVertexSet, MinimumMultiwayCut, MinimumSumMulticenter,
        MinimumVertexCover, MultipleChoiceBranching, MultipleCopyFileAllocation,
        OptimalLinearArrangement, PartitionIntoPathsOfLength2, PartitionIntoTriangles,
        PathConstrainedNetworkFlow,
        RuralPostman, ShortestWeightConstrainedPath, SteinerTreeInGraphs, TravelingSalesman,
        UndirectedTwoCommodityIntegralFlow,
    };
    pub use crate::models::misc::{
        AdditionalKey, BinPacking, BoyceCoddNormalFormViolation, CbqRelation,
        ConjunctiveBooleanQuery, ConjunctiveQueryFoldability, ConsistencyOfDatabaseFrequencyTables,
        EnsembleComputation, Factoring, FlowShopScheduling, Knapsack, LongestCommonSubsequence,
        MinimumTardinessSequencing, MultiprocessorScheduling, PaintShop, Partition, QueryArg,
        RectilinearPictureCompression, ResourceConstrainedScheduling,
        SchedulingWithIndividualDeadlines, SequencingToMinimizeMaximumCumulativeCost,
        SequencingToMinimizeWeightedCompletionTime, SequencingToMinimizeWeightedTardiness,
        SequencingWithReleaseTimesAndDeadlines, SequencingWithinIntervals,
        ShortestCommonSupersequence, StackerCrane, StaffScheduling, StringToStringCorrection,
        SubsetSum, SumOfSquaresPartition, Term, TimetableDesign,
    };
    pub use crate::models::set::{
        ComparativeContainment, ConsecutiveSets, ExactCoverBy3Sets, MaximumSetPacking,
        MinimumCardinalityKey, MinimumHittingSet, MinimumSetCovering, PrimeAttributeName, SetBasis,
    };

    // Core traits
    pub use crate::rules::{ReduceTo, ReductionResult};
    pub use crate::solvers::{BruteForce, Solver};
    pub use crate::traits::{OptimizationProblem, Problem, SatisfactionProblem};

    // Types
    pub use crate::error::{ProblemError, Result};
    pub use crate::types::{Direction, One, ProblemSize, SolutionSize, Unweighted};
}

// Re-export commonly used items at crate root
pub use big_o::big_o_normal_form;
pub use canonical::canonical_form;
pub use error::{ProblemError, Result};
pub use expr::{asymptotic_normal_form, AsymptoticAnalysisError, CanonicalizationError, Expr};
pub use registry::{ComplexityClass, ProblemInfo};
pub use solvers::{BruteForce, Solver};
pub use traits::{OptimizationProblem, Problem, SatisfactionProblem};
pub use types::{
    Direction, NumericSize, One, ProblemSize, SolutionSize, Unweighted, WeightElement,
};

// Re-export proc macros for reduction registration and variant declaration
pub use problemreductions_macros::{declare_variants, reduction};

// Re-export inventory so `declare_variants!` can use `$crate::inventory::submit!`
pub use inventory;

#[cfg(test)]
#[path = "unit_tests/graph_models.rs"]
mod test_graph_models;
#[cfg(test)]
#[path = "unit_tests/prelude.rs"]
mod test_prelude;
#[cfg(test)]
#[path = "unit_tests/property.rs"]
mod test_property;
#[cfg(test)]
#[path = "unit_tests/reduction_graph.rs"]
mod test_reduction_graph;
#[cfg(test)]
#[path = "unit_tests/unitdiskmapping_algorithms/mod.rs"]
mod test_unitdiskmapping_algorithms;
