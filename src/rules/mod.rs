//! Reduction rules between NP-hard problems.

pub mod cost;
pub mod registry;
pub use cost::{
    CustomCost, Minimize, MinimizeLexicographic, MinimizeMax, MinimizeSteps, MinimizeWeighted,
    PathCostFn,
};
pub use registry::{ReductionEntry, ReductionOverhead};

mod circuit_spinglass;
mod factoring_circuit;
mod graph;
mod independentset_setpacking;
mod matching_setpacking;
mod sat_coloring;
mod sat_dominatingset;
mod sat_independentset;
mod sat_ksat;
mod spinglass_maxcut;
mod spinglass_qubo;
mod traits;
mod vertexcovering_independentset;
mod vertexcovering_setcovering;

pub mod unitdiskmapping;

#[cfg(feature = "ilp")]
mod clique_ilp;
#[cfg(feature = "ilp")]
mod coloring_ilp;
#[cfg(feature = "ilp")]
mod dominatingset_ilp;
#[cfg(feature = "ilp")]
mod factoring_ilp;
#[cfg(feature = "ilp")]
mod independentset_ilp;
#[cfg(feature = "ilp")]
mod matching_ilp;
#[cfg(feature = "ilp")]
mod setcovering_ilp;
#[cfg(feature = "ilp")]
mod setpacking_ilp;
#[cfg(feature = "ilp")]
mod vertexcovering_ilp;

pub use circuit_spinglass::{
    and_gadget, not_gadget, or_gadget, set0_gadget, set1_gadget, xor_gadget, LogicGadget,
    ReductionCircuitToSG,
};
pub use factoring_circuit::ReductionFactoringToCircuit;
pub use graph::{
    EdgeJson, NodeJson, ReductionEdge, ReductionGraph, ReductionGraphJson, ReductionPath,
};
pub use independentset_setpacking::{ReductionISToSP, ReductionSPToIS};
pub use matching_setpacking::ReductionMatchingToSP;
pub use sat_coloring::ReductionSATToColoring;
pub use sat_dominatingset::ReductionSATToDS;
pub use sat_independentset::{BoolVar, ReductionSATToIS};
pub use sat_ksat::{ReductionKSATToSAT, ReductionSATToKSAT};
pub use spinglass_maxcut::{ReductionMaxCutToSG, ReductionSGToMaxCut};
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
pub use traits::{ReduceTo, ReductionResult};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};
pub use vertexcovering_setcovering::ReductionVCToSC;

#[cfg(feature = "ilp")]
pub use clique_ilp::ReductionCliqueToILP;
#[cfg(feature = "ilp")]
pub use coloring_ilp::ReductionColoringToILP;
#[cfg(feature = "ilp")]
pub use dominatingset_ilp::ReductionDSToILP;
#[cfg(feature = "ilp")]
pub use factoring_ilp::ReductionFactoringToILP;
#[cfg(feature = "ilp")]
pub use independentset_ilp::ReductionISToILP;
#[cfg(feature = "ilp")]
pub use matching_ilp::ReductionMatchingToILP;
#[cfg(feature = "ilp")]
pub use setcovering_ilp::ReductionSCToILP;
#[cfg(feature = "ilp")]
pub use setpacking_ilp::ReductionSPToILP;
#[cfg(feature = "ilp")]
pub use vertexcovering_ilp::ReductionVCToILP;
