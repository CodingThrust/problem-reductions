//! Reduction rules between NP-hard problems.

pub mod cost;
pub mod registry;
pub use cost::{CustomCost, Minimize, MinimizeSteps, PathCostFn};
pub use registry::{ReductionEntry, ReductionOverhead};

mod circuit_spinglass;
mod coloring_qubo;
mod factoring_circuit;
mod graph;
mod kcoloring_casts;
mod ksatisfiability_casts;
mod ksatisfiability_qubo;
mod maximumindependentset_casts;
mod maximumindependentset_gridgraph;
mod maximumindependentset_maximumsetpacking;
mod maximumindependentset_qubo;
mod maximumindependentset_triangular;
mod maximummatching_maximumsetpacking;
mod maximumsetpacking_casts;
mod maximumsetpacking_qubo;
mod minimumvertexcover_maximumindependentset;
mod minimumvertexcover_minimumsetcovering;
mod minimumvertexcover_qubo;
mod sat_circuitsat;
mod sat_coloring;
mod sat_ksat;
mod sat_maximumindependentset;
mod sat_minimumdominatingset;
mod spinglass_casts;
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
    ChainedReduction, EdgeJson, ExecutablePath, NodeJson, ReductionGraph, ReductionGraphJson,
    ReductionPath, ReductionStep,
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
pub use sat_circuitsat::ReductionSATToCircuit;
pub use sat_coloring::ReductionSATToColoring;
pub use sat_ksat::{ReductionKSATToSAT, ReductionSATToKSAT};
pub use sat_maximumindependentset::{BoolVar, ReductionSATToIS};
pub use sat_minimumdominatingset::ReductionSATToDS;
pub use spinglass_maxcut::{ReductionMaxCutToSG, ReductionSGToMaxCut};
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
pub use traits::{ReduceTo, ReductionAutoCast, ReductionResult};

/// Generates a variant-cast `ReduceTo` impl with `#[reduction]` registration.
///
/// Variant casts convert a problem from one variant to another (e.g.,
/// `MIS<KingsSubgraph, i32>` -> `MIS<UnitDiskGraph, i32>`). The solution
/// mapping is identity -- vertex/element indices are preserved.
///
/// The problem name is specified once, followed by `<SourceParams> => <TargetParams>`.
/// This works with any number of type parameters.
///
/// # Example
///
/// ```text
/// impl_variant_reduction!(
///     MaximumIndependentSet,
///     <KingsSubgraph, i32> => <UnitDiskGraph, i32>,
///     fields: [num_vertices, num_edges],
///     |src| MaximumIndependentSet::new(
///         src.graph().cast_to_parent(), src.weights())
/// );
/// ```
#[macro_export]
macro_rules! impl_variant_reduction {
    ($problem:ident,
     < $($src_param:ty),+ > => < $($dst_param:ty),+ >,
     fields: [$($field:ident),+],
     |$src:ident| $body:expr) => {
        #[$crate::reduction(
            overhead = {
                $crate::rules::registry::ReductionOverhead::identity(
                    &[$(stringify!($field)),+]
                )
            }
        )]
        impl $crate::rules::ReduceTo<$problem<$($dst_param),+>>
            for $problem<$($src_param),+>
        {
            type Result = $crate::rules::ReductionAutoCast<
                $problem<$($src_param),+>,
                $problem<$($dst_param),+>,
            >;
            fn reduce_to(&self) -> Self::Result {
                let $src = self;
                $crate::rules::ReductionAutoCast::new($body)
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
