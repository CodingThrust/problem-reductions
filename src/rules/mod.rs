//! Reduction rules between NP-hard problems.

mod graph;
mod traits;
mod independentset_setpacking;
mod spinglass_maxcut;
mod spinglass_qubo;
mod vertexcovering_independentset;

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

pub use graph::{ReductionGraph, ReductionPath};
pub use traits::{ReduceTo, ReductionResult};
pub use independentset_setpacking::{ReductionISToSP, ReductionSPToIS};
pub use spinglass_maxcut::{ReductionMaxCutToSG, ReductionSGToMaxCut};
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};

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
