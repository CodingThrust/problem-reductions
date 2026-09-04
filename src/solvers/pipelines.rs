//! Fixed ILP pipelines registered for exact problem variants.
//!
//! These declarations are executable production metadata. Runtime solving
//! resolves them once to exact reduction function pointers and never searches
//! the reduction graph.

use super::registry::{IlpPipelineRegistration, StaticProblemStep};

macro_rules! register_ilp_pipeline {
    ($(($name:literal, [$(($key:literal, $value:literal)),* $(,)?])),+ $(,)?) => {
        inventory::submit! {
            IlpPipelineRegistration {
                path: &[
                    $(StaticProblemStep {
                        name: $name,
                        variant: &[$(($key, $value)),*],
                    }),+
                ],
            }
        }
    };
}

register_ilp_pipeline! {
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("AcyclicPartition", [("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("BMF", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("BalancedCompleteBipartiteSubgraph", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("BicliqueCover", []),
    ("BMF", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("BiconnectivityAugmentation", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("BinPacking", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("BottleneckTravelingSalesman", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("BoundedComponentSpanningForest", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("CapacityAssignment", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("CircuitSAT", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ClosestString", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ClosestSubstring", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("Clustering", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ConsecutiveBlockMinimization", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ConsecutiveOnesMatrixAugmentation", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ConsecutiveOnesSubmatrix", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ConsistencyOfDatabaseFrequencyTables", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("DecisionMinimumDominatingSet", [("graph", "SimpleGraph"), ("weight", "One")]),
    ("MinimumSumMulticenter", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("DecisionMinimumDominatingSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MinimumDominatingSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("DecisionMinimumVertexCover", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MinimumVertexCover", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MinimumSetCovering", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("DecisionOptimalLinearArrangement", [("graph", "SimpleGraph")]),
    ("OptimalLinearArrangement", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("DirectedHamiltonianPath", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("DirectedTwoCommodityIntegralFlow", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("DisjointConnectingPaths", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("EulerianPath", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ExactCoverBy3Sets", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ExpectedRetrievalCost", []),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("Factoring", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("FeasibleRegisterAssignment", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("FlowShopScheduling", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("GraphPartitioning", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("HamiltonianCircuit", [("graph", "SimpleGraph")]),
    ("LongestCircuit", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("HamiltonianPath", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("HighlyConnectedDeletion", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

// This exact variant also has a customized backend. Default dispatch selects the
// customized registration, while an explicit ILP override executes this pipeline.
register_ilp_pipeline! {
    ("RootedTreeArrangement", [("graph", "SimpleGraph")]),
    ("RootedTreeStorageAssignment", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("IntegralFlowBundles", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("IntegralFlowHomologousArcs", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("IntegralFlowWithMultipliers", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("IsomorphicSpanningTree", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("KClique", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("KColoring", [("graph", "SimpleGraph"), ("k", "KN")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("KColoring", [("graph", "SimpleGraph"), ("k", "K3")]),
    ("Clustering", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("KSatisfiability", [("k", "KN")]),
    ("Satisfiability", []),
    ("NAESatisfiability", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("Knapsack", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("LengthBoundedDisjointPaths", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("LongestCircuit", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("LongestCommonSubsequence", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("LongestPath", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximalIS", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("Maximum2Satisfiability", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumClique", [("graph", "SimpleGraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumSetPacking", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumClique", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumCoKPlex", [("graph", "SimpleGraph"), ("k", "KN"), ("weight", "One")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumCoKPlex", [("graph", "SimpleGraph"), ("k", "KN"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumCommonEdgeSubgraph", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumContactMapOverlap", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumDomaticNumber", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumEdgeWeightedKClique", [("weight", "f64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumEdgeWeightedKClique", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumSetPacking", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumIndependentSet", [("graph", "KingsSubgraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "UnitDiskGraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumClique", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumSetPacking", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumIndependentSet", [("graph", "UnitDiskGraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "One")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumClique", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumIndependentSet", [("graph", "KingsSubgraph"), ("weight", "i64")]),
    ("MaximumIndependentSet", [("graph", "UnitDiskGraph"), ("weight", "i64")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumClique", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumIndependentSet", [("graph", "TriangularSubgraph"), ("weight", "i64")]),
    ("MaximumIndependentSet", [("graph", "UnitDiskGraph"), ("weight", "i64")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumClique", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumIndependentSet", [("graph", "UnitDiskGraph"), ("weight", "i64")]),
    ("MaximumIndependentSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MaximumClique", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumLeafSpanningTree", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumLikelihoodRanking", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumMatching", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumSetPacking", [("weight", "One")]),
    ("MaximumSetPacking", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumSetPacking", [("weight", "f64")]),
    ("QUBO", [("weight", "f64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MaximumSetPacking", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinMaxMulticenter", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumCapacitatedSpanningTree", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumCoveringByCliques", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumCutIntoBoundedSets", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumDiscretePlanarInverseKinematics", []),
    ("QUBO", [("weight", "f64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumDominatingSet", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumEdgeCostFlow", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumExternalMacroDataCompression", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumFaultDetectionTestSet", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumFeedbackVertexSet", [("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumGraphBandwidth", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumHittingSet", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumInternalMacroDataCompression", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumMatrixCover", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumMaximalMatching", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumMetricDimension", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumMultiwayCut", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumSetCovering", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumSumMulticenter", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumTardinessSequencing", [("weight", "One")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumTardinessSequencing", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumVertexCover", [("graph", "SimpleGraph"), ("weight", "One")]),
    ("MinimumHittingSet", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumVertexCover", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("MinimumSetCovering", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MinimumWeightDecoding", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MixedChinesePostman", [("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MonochromaticTriangle", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MultipleCopyFileAllocation", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("MultiprocessorScheduling", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("NAESatisfiability", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("Numerical3DimensionalMatching", []),
    ("NumericalMatchingWithTargetSums", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("NumericalMatchingWithTargetSums", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("OpenShopScheduling", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("OptimalLinearArrangement", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("OptimumCommunicationSpanningTree", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PaintShop", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PartiallyOrderedKnapsack", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("Partition", []),
    ("MultiprocessorScheduling", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PartitionIntoCliques", [("graph", "SimpleGraph")]),
    ("MinimumCoveringByCliques", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PartitionIntoPathsOfLength2", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PartitionIntoTriangles", [("graph", "SimpleGraph")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PathConstrainedNetworkFlow", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PrecedenceConstrainedScheduling", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("PreemptiveScheduling", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("QUBO", [("weight", "f64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("QUBO", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("QuadraticAssignment", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("RectilinearPictureCompression", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("RegisterSufficiency", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ResourceConstrainedScheduling", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("RootedTreeStorageAssignment", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("RuralPostman", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("Satisfiability", []),
    ("NAESatisfiability", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SchedulingToMinimizeWeightedCompletionTime", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SchedulingWithIndividualDeadlines", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SequencingToMinimizeMaximumCumulativeCost", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SequencingToMinimizeTardyTaskWeight", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SequencingToMinimizeWeightedTardiness", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SequencingWithDeadlinesAndSetUpTimes", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SequencingWithReleaseTimesAndDeadlines", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SequencingWithinIntervals", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SetSplitting", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ShortestCommonSupersequence", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ShortestWeightConstrainedPath", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SparseMatrixCompression", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SpinGlass", [("graph", "SimpleGraph"), ("weight", "f64")]),
    ("QUBO", [("weight", "f64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SpinGlass", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("QUBO", [("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("StackerCrane", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("StringToStringCorrection", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("StrongConnectivityAugmentation", [("weight", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SubgraphIsomorphism", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("SumOfSquaresPartition", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ThreeDimensionalMatching", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("ThreePartition", []),
    ("ResourceConstrainedScheduling", []),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("TravelingSalesman", [("graph", "SimpleGraph"), ("weight", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "i64")]),
    ("ILP", [("variable", "bool"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("UndirectedFlowLowerBounds", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}

register_ilp_pipeline! {
    ("UndirectedTwoCommodityIntegralFlow", []),
    ("ILP", [("variable", "i64"), ("coefficient", "i64")]),
    ("ILP", [("variable", "i64"), ("coefficient", "f64")]),
}
