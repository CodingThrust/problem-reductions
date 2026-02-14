//! Reduction rules between NP-hard problems.

pub mod cost;
pub mod registry;
pub use cost::{
    CustomCost, Minimize, MinimizeLexicographic, MinimizeMax, MinimizeSteps, MinimizeWeighted,
    PathCostFn,
};
pub use registry::{ReductionEntry, ReductionOverhead};

mod circuit_spinglass;
mod coloring_qubo;
mod factoring_circuit;
mod graph;
mod ksatisfiability_qubo;
mod maximumindependentset_gridgraph;
mod maximumindependentset_maximumsetpacking;
mod maximumindependentset_qubo;
mod maximumindependentset_triangular;
mod maximummatching_maximumsetpacking;
mod maximumsetpacking_qubo;
mod minimumvertexcover_maximumindependentset;
mod minimumvertexcover_minimumsetcovering;
mod minimumvertexcover_qubo;
mod natural;
mod sat_coloring;
mod sat_ksat;
mod sat_maximumindependentset;
mod sat_minimumdominatingset;
mod spinglass_maxcut;
mod spinglass_qubo;
mod traits;

pub mod unitdiskmapping;

#[cfg(feature = "ilp")]
mod coloring_ilp;
#[cfg(feature = "ilp")]
mod factoring_ilp;
#[cfg(feature = "ilp")]
mod ilp_qubo;
#[cfg(feature = "ilp")]
mod maximumclique_ilp;
#[cfg(feature = "ilp")]
mod maximumindependentset_ilp;
#[cfg(feature = "ilp")]
mod maximummatching_ilp;
#[cfg(feature = "ilp")]
mod maximumsetpacking_ilp;
#[cfg(feature = "ilp")]
mod minimumdominatingset_ilp;
#[cfg(feature = "ilp")]
mod minimumsetcovering_ilp;
#[cfg(feature = "ilp")]
mod minimumvertexcover_ilp;
#[cfg(feature = "ilp")]
mod travelingsalesman_ilp;

pub use circuit_spinglass::{
    and_gadget, not_gadget, or_gadget, set0_gadget, set1_gadget, xor_gadget, LogicGadget,
    ReductionCircuitToSG,
};
pub use coloring_qubo::ReductionKColoringToQUBO;
pub use factoring_circuit::ReductionFactoringToCircuit;
pub use graph::{
    EdgeJson, EdgeKind, NodeJson, ReductionEdge, ReductionGraph, ReductionGraphJson, ReductionPath,
    ReductionStep, ResolvedPath,
};
pub use ksatisfiability_qubo::{Reduction3SATToQUBO, ReductionKSatToQUBO};
pub use maximumindependentset_gridgraph::{ReductionISSimpleToGrid, ReductionISUnitDiskToGrid};
pub use maximumindependentset_maximumsetpacking::{ReductionISToSP, ReductionSPToIS};
pub use maximumindependentset_qubo::ReductionISToQUBO;
pub use maximumindependentset_triangular::ReductionISSimpleToTriangular;
pub use maximummatching_maximumsetpacking::ReductionMatchingToSP;
pub use maximumsetpacking_qubo::ReductionSPToQUBO;
pub use minimumvertexcover_maximumindependentset::{ReductionISToVC, ReductionVCToIS};
pub use minimumvertexcover_minimumsetcovering::ReductionVCToSC;
pub use minimumvertexcover_qubo::ReductionVCToQUBO;
pub use sat_coloring::ReductionSATToColoring;
pub use sat_ksat::{ReductionKSATToSAT, ReductionSATToKSAT};
pub use sat_maximumindependentset::{BoolVar, ReductionSATToIS};
pub use sat_minimumdominatingset::ReductionSATToDS;
pub use spinglass_maxcut::{ReductionMaxCutToSG, ReductionSGToMaxCut};
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
pub use traits::{ReduceTo, ReductionAutoCast, ReductionResult};

/// Generates a natural-edge `ReduceTo` impl for graph subtype relaxation.
///
/// When graph type `$SubGraph` is a subtype of `$SuperGraph`, a problem on
/// the subgraph can be trivially solved as the same problem on the supergraph.
/// This macro stamps out the concrete `#[reduction]` impl with identity overhead
/// and uses [`ReductionAutoCast`] for the identity solution mapping.
///
/// # Example
///
/// ```text
/// impl_natural_reduction!(MaximumIndependentSet, Triangular, SimpleGraph, i32);
/// // Generates: ReduceTo<MIS<SimpleGraph, i32>> for MIS<Triangular, i32>
/// ```
#[macro_export]
macro_rules! impl_natural_reduction {
    ($Problem:ident, $SubGraph:ty, $SuperGraph:ty, $Weight:ty) => {
        #[reduction(
                                    overhead = {
                                        $crate::rules::registry::ReductionOverhead::new(vec![
                                            ("num_vertices", $crate::poly!(num_vertices)),
                                            ("num_edges", $crate::poly!(num_edges)),
                                        ])
                                    }
                                )]
        impl $crate::rules::ReduceTo<$Problem<$SuperGraph, $Weight>>
            for $Problem<$SubGraph, $Weight>
        {
            type Result = $crate::rules::ReductionAutoCast<Self, $Problem<$SuperGraph, $Weight>>;

            fn reduce_to(&self) -> Self::Result {
                use $crate::topology::GraphCast;
                let graph: $SuperGraph = self.graph().cast_graph();
                let target = $Problem::from_graph(graph, self.weights());
                $crate::rules::ReductionAutoCast::new(target)
            }
        }
    };
}

#[cfg(feature = "ilp")]
pub use coloring_ilp::{ReductionColoringToILP, ReductionKColoringToILP};
#[cfg(feature = "ilp")]
pub use factoring_ilp::ReductionFactoringToILP;
#[cfg(feature = "ilp")]
pub use ilp_qubo::ReductionILPToQUBO;
#[cfg(feature = "ilp")]
pub use maximumclique_ilp::ReductionCliqueToILP;
#[cfg(feature = "ilp")]
pub use maximumindependentset_ilp::ReductionISToILP;
#[cfg(feature = "ilp")]
pub use maximummatching_ilp::ReductionMatchingToILP;
#[cfg(feature = "ilp")]
pub use maximumsetpacking_ilp::ReductionSPToILP;
#[cfg(feature = "ilp")]
pub use minimumdominatingset_ilp::ReductionDSToILP;
#[cfg(feature = "ilp")]
pub use minimumsetcovering_ilp::ReductionSCToILP;
#[cfg(feature = "ilp")]
pub use minimumvertexcover_ilp::ReductionVCToILP;
#[cfg(feature = "ilp")]
pub use travelingsalesman_ilp::ReductionTSPToILP;
