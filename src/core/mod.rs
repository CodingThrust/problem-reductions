//! v1 core abstractions for problem definitions and reductions.
//!
//! This module introduces a typed, stable contract surface that can coexist
//! with legacy traits during migration.

pub mod assignment;
pub mod evaluation;
pub mod objective;
pub mod problem;
pub mod reduction;
pub mod variant;

pub use assignment::Assignment;
pub use evaluation::Evaluation;
pub use objective::{ObjectiveDirection, ObjectiveValue};
pub use problem::{LegacyProblemAdapter, ProblemInstance, ProblemSpec};
pub use reduction::{LegacyReductionAdapter, Reduction};
pub use variant::{from_legacy_variant, VariantDimension, VariantKey};

#[cfg(test)]
#[path = "../unit_tests/contracts/mod.rs"]
mod tests;
