//! Shared deterministic solver dispatch.

use super::registry::CompiledIlpPipeline;
use super::registry::{solver_capability_registry, CustomizedSolverRegistration, ExactProblemKey};
use super::SolveOutcome;
use crate::registry::LoadedDynProblem;
use serde::Serialize;

/// Public solver override. Omission is represented by [`SolverRequest::Default`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SolverRequest {
    #[default]
    Default,
    Customized,
    Ilp,
    BruteForce,
}

/// Information about the backend execution that produced a solve result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SolverExecution {
    External,
    Customized { implementation: &'static str },
    Ilp { reduction_path: Vec<String> },
    BruteForce,
}

/// Type-erased result returned by deterministic solver dispatch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolveResult {
    pub solver: SolverExecution,
    pub outcome: SolveOutcome,
}

fn problem_key(problem: &LoadedDynProblem) -> ExactProblemKey {
    ExactProblemKey::new(problem.problem_name(), problem.variant_map())
}

fn solve_customized(
    problem: &LoadedDynProblem,
    registration: &'static CustomizedSolverRegistration,
) -> Result<SolveResult, super::SolveError> {
    let outcome = (registration.solve_fn)(problem.as_any())?;
    Ok(SolveResult {
        solver: SolverExecution::Customized {
            implementation: registration.implementation,
        },
        outcome,
    })
}

fn solve_ilp(
    problem: &LoadedDynProblem,
    pipeline: &CompiledIlpPipeline,
) -> Result<SolveResult, super::SolveError> {
    let outcome = pipeline
        .solve(
            problem.as_any(),
            &super::ilp::adapter::HighsAdapter::new(None),
        )
        .map_err(|source| super::SolveError::IlpSolve {
            problem: problem_key(problem).label(),
            source,
        })?;
    Ok(SolveResult {
        solver: SolverExecution::Ilp {
            reduction_path: pipeline.path_labels(),
        },
        outcome,
    })
}

fn solve_brute_force(
    problem: &LoadedDynProblem,
    registration: &'static super::BruteForceRegistration,
) -> Result<SolveResult, super::SolveError> {
    let outcome = match (registration.solve_fn)(problem.as_any())? {
        Some((solution, evaluation)) => SolveOutcome::Optimal {
            solution,
            evaluation,
        },
        None => SolveOutcome::Infeasible,
    };
    Ok(SolveResult {
        solver: SolverExecution::BruteForce,
        outcome,
    })
}

/// Solve a loaded problem using deterministic exact-variant dispatch.
///
/// Default dispatch is customized, then the registered fixed ILP pipeline, then
/// brute force. Once selected, backend failure is returned without fallback.
pub fn solve(
    problem: &LoadedDynProblem,
    request: SolverRequest,
) -> Result<SolveResult, super::SolveError> {
    let registry = solver_capability_registry().map_err(super::SolveError::InvalidRegistry)?;
    let key = problem_key(problem);
    let capabilities = registry.lookup(&key);

    match request {
        SolverRequest::BruteForce => solve_brute_force(
            problem,
            capabilities
                .brute_force
                .ok_or_else(|| super::SolveError::MissingRegistration(key.label()))?,
        ),
        SolverRequest::Customized => {
            let registration = capabilities
                .customized
                .ok_or_else(|| super::SolveError::MissingCustomizedCapability(key.label()))?;
            solve_customized(problem, registration)
        }
        SolverRequest::Ilp => {
            let pipeline = capabilities
                .ilp
                .ok_or_else(|| super::SolveError::MissingIlpCapability(key.label()))?;
            solve_ilp(problem, pipeline)
        }
        SolverRequest::Default => {
            if let Some(customized) = capabilities.customized {
                return solve_customized(problem, customized);
            }
            if let Some(pipeline) = capabilities.ilp {
                return solve_ilp(problem, pipeline);
            }
            solve_brute_force(
                problem,
                capabilities
                    .brute_force
                    .ok_or_else(|| super::SolveError::MissingRegistration(key.label()))?,
            )
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/resolver.rs"]
mod tests;
