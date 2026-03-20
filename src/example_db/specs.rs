//! Shared example specification types and helper functions.
//!
//! These types describe canonical model and rule examples with metadata
//! that can be validated against the catalog and reduction registry.

use crate::export::{ProblemSide, RuleExample, SolutionPair};
use crate::prelude::{Problem, ReduceTo, ReductionResult};
use crate::registry::DynProblem;
use serde::Serialize;

/// Specification for a canonical model example.
///
/// Stores a concrete problem instance and its known optimal solution.
/// The instance is type-erased via `DynProblem` for heterogeneous collection.
#[allow(dead_code)] // `id` field is only read in tests
pub struct ModelExampleSpec {
    /// Unique example identifier (used by uniqueness tests).
    pub id: &'static str,
    /// The concrete problem instance (type-erased).
    pub instance: Box<dyn DynProblem>,
    /// One known optimal configuration.
    pub optimal_config: Vec<usize>,
    /// The optimal value as a serializable JSON value.
    pub optimal_value: serde_json::Value,
}

/// Specification for a canonical rule example.
#[allow(dead_code)] // `id` field is only read in tests
pub struct RuleExampleSpec {
    /// Unique example identifier.
    pub id: &'static str,
    /// Builder function that produces the full exported example.
    pub build: fn() -> RuleExample,
}

// ---- Rule example helpers ----

pub fn assemble_rule_example<S, T>(
    source: &S,
    target: &T,
    solutions: Vec<SolutionPair>,
) -> RuleExample
where
    S: Problem + Serialize,
    T: Problem + Serialize,
{
    RuleExample {
        source: ProblemSide::from_problem(source),
        target: ProblemSide::from_problem(target),
        solutions,
    }
}

/// Assemble a rule example from a source and its reduction, with a pre-stored solution pair.
pub fn rule_example_with_witness<S, T>(source: S, solution: SolutionPair) -> RuleExample
where
    S: Problem + Serialize + ReduceTo<T>,
    T: Problem + Serialize,
    <S as ReduceTo<T>>::Result: ReductionResult<Source = S, Target = T>,
{
    let reduction = source.reduce_to();
    let target = reduction.target_problem();
    assemble_rule_example(&source, target, vec![solution])
}
