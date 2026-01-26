//! Reduction rules between NP-hard problems.

pub mod cost;
pub mod registry;
pub use cost::{CustomCost, Minimize, MinimizeLexicographic, MinimizeMax, MinimizeSteps, MinimizeWeighted, PathCostFn};
pub use registry::{ReductionEntry, ReductionOverhead};

mod circuit_spinglass;
mod graph;
mod traits;
mod factoring_circuit;
mod independentset_setpacking;
mod matching_setpacking;
mod sat_coloring;
mod sat_dominatingset;
mod sat_independentset;
mod sat_ksat;
mod spinglass_maxcut;
mod spinglass_qubo;
mod vertexcovering_independentset;
mod vertexcovering_setcovering;

#[cfg(feature = "ilp")]
mod clique_ilp;
#[cfg(feature = "ilp")]
mod dominatingset_ilp;
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

pub use graph::{EdgeJson, NodeJson, ReductionGraph, ReductionGraphJson, ReductionPath};
pub use traits::{ReduceTo, ReductionResult};
pub use independentset_setpacking::{ReductionISToSP, ReductionSPToIS};
pub use matching_setpacking::ReductionMatchingToSP;
pub use sat_coloring::ReductionSATToColoring;
pub use sat_dominatingset::ReductionSATToDS;
pub use sat_independentset::{BoolVar, ReductionSATToIS};
pub use spinglass_maxcut::{ReductionMaxCutToSG, ReductionSGToMaxCut};
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};
pub use vertexcovering_setcovering::ReductionVCToSC;
pub use sat_ksat::{ReductionKSATToSAT, ReductionSATToKSAT};
pub use factoring_circuit::ReductionFactoringToCircuit;
pub use circuit_spinglass::{
    LogicGadget, ReductionCircuitToSG,
    and_gadget, or_gadget, not_gadget, xor_gadget, set0_gadget, set1_gadget,
};

#[cfg(feature = "ilp")]
pub use clique_ilp::ReductionCliqueToILP;
#[cfg(feature = "ilp")]
pub use dominatingset_ilp::ReductionDSToILP;
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
