//! Reduction rules between NP-hard problems.

mod traits;
mod independentset_setpacking;
mod vertexcovering_independentset;

pub use traits::{ReduceTo, ReductionResult};
pub use independentset_setpacking::{ReductionISToSP, ReductionSPToIS};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};
