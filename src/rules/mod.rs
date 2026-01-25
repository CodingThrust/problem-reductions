//! Reduction rules between NP-hard problems.

mod traits;
mod independentset_setpacking;
mod spinglass_maxcut;
mod spinglass_qubo;
mod vertexcovering_independentset;

pub use traits::{ReduceTo, ReductionResult};
pub use independentset_setpacking::{ReductionISToSP, ReductionSPToIS};
pub use spinglass_maxcut::{ReductionMaxCutToSG, ReductionSGToMaxCut};
pub use spinglass_qubo::{ReductionQUBOToSG, ReductionSGToQUBO};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};
