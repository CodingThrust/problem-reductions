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
mod maximumindependentset_qubo;
mod maximumindependentset_maximumsetpacking;
mod ksatisfiability_qubo;
mod maximummatching_maximumsetpacking;
mod sat_coloring;
mod sat_minimumdominatingset;
mod sat_maximumindependentset;
mod sat_ksat;
mod maximumsetpacking_qubo;
mod spinglass_maxcut;
mod spinglass_qubo;
mod traits;
mod minimumvertexcover_maximumindependentset;
mod minimumvertexcover_qubo;
mod minimumvertexcover_minimumsetcovering;

pub mod unitdiskmapping;

#[cfg(feature = "ilp")]
mod maximumclique_ilp;
#[cfg(feature = "ilp")]
mod ilp_qubo;
#[cfg(feature = "ilp")]
mod coloring_ilp;
#[cfg(feature = "ilp")]
mod minimumdominatingset_ilp;
#[cfg(feature = "ilp")]
mod factoring_ilp;
#[cfg(feature = "ilp")]
mod maximumindependentset_ilp;
#[cfg(feature = "ilp")]
mod maximummatching_ilp;
#[cfg(feature = "ilp")]
mod minimumsetcovering_ilp;
#[cfg(feature = "ilp")]
mod maximumsetpacking_ilp;
#[cfg(feature = "ilp")]
mod minimumvertexcover_ilp;

pub use circuit_spinglass::{
    and_gadget, not_gadget, or_gadget, set0_gadget, set1_gadget, xor_gadget, LogicGadget,
    ReductionCircuitToSG,
};
pub use factoring_circuit::ReductionFactoringToCircuit;
pub use graph::{
    EdgeJson, NodeJson, ReductionEdge, ReductionGraph, ReductionGraphJson, ReductionPath,
};
pub use coloring_qubo::ReductionKColoringToQUBO;
pub use maximumindependentset_qubo::ReductionISToQUBO;
pub use maximumindependentset_maximumsetpacking::{ReductionISToSP, ReductionSPToIS};
pub use ksatisfiability_qubo::{ReductionKSatToQUBO, ReductionK3SatToQUBO};
pub use maximummatching_maximumsetpacking::ReductionMatchingToSP;
pub use sat_coloring::ReductionSATToColoring;
pub use maximumsetpacking_qubo::ReductionSPToQUBO;
pub use sat_minimumdominatingset::ReductionSATToDS;
pub use sat_maximumindependentset::{BoolVar, ReductionSATToIS};
pub use sat_ksat::{ReductionKSATToSAT, ReductionSATToKSAT};
pub use spinglass_maxcut::{ReductionMaxCutToSG, ReductionSGToMaxCut};
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
pub use traits::{ReduceTo, ReductionResult};
pub use minimumvertexcover_maximumindependentset::{ReductionISToVC, ReductionVCToIS};
pub use minimumvertexcover_qubo::ReductionVCToQUBO;
pub use minimumvertexcover_minimumsetcovering::ReductionVCToSC;

#[cfg(feature = "ilp")]
pub use maximumclique_ilp::ReductionCliqueToILP;
#[cfg(feature = "ilp")]
pub use coloring_ilp::{ReductionColoringToILP, ReductionKColoringToILP};
#[cfg(feature = "ilp")]
pub use minimumdominatingset_ilp::ReductionDSToILP;
#[cfg(feature = "ilp")]
pub use ilp_qubo::ReductionILPToQUBO;
#[cfg(feature = "ilp")]
pub use factoring_ilp::ReductionFactoringToILP;
#[cfg(feature = "ilp")]
pub use maximumindependentset_ilp::ReductionISToILP;
#[cfg(feature = "ilp")]
pub use maximummatching_ilp::ReductionMatchingToILP;
#[cfg(feature = "ilp")]
pub use minimumsetcovering_ilp::ReductionSCToILP;
#[cfg(feature = "ilp")]
pub use maximumsetpacking_ilp::ReductionSPToILP;
#[cfg(feature = "ilp")]
pub use minimumvertexcover_ilp::ReductionVCToILP;
