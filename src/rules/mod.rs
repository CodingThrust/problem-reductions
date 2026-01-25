//! Reduction rules between NP-hard problems.

mod traits;
mod vertexcovering_independentset;

pub use traits::{ReduceTo, ReductionResult};
pub use vertexcovering_independentset::{ReductionISToVC, ReductionVCToIS};
