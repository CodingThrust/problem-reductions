//! Shared deterministic solver dispatch.

use super::registry::CompiledIlpPipeline;
use super::registry::{
    solver_capability_registry, CustomizedSolverRegistration, ExactProblemKey, RegistryBuildError,
};
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
    Customized { implementation: &'static str },
    Ilp { reduction_path: Vec<String> },
    BruteForce,
}

/// Type-erased result returned by deterministic solver dispatch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeterministicSolveResult {
    pub solver: SolverExecution,
    pub config: Option<Vec<usize>>,
    pub evaluation: String,
}

#[derive(Debug, thiserror::Error)]
pub enum DeterministicSolveError {
    #[error("solver capability registry is invalid: {0}")]
    InvalidRegistry(&'static RegistryBuildError),
    #[error("No ILP pipeline is registered for {0}")]
    MissingIlpCapability(String),
    #[error("No customized solver is registered for {0}")]
    MissingCustomizedCapability(String),
    #[error("customized solver found no solution for {problem}")]
    CustomizedNoSolution { problem: String },
    #[error("ILP solver failed for {problem}: {source}")]
    IlpSolve {
        problem: String,
        #[source]
        source: super::ILPSolveError,
    },
}

fn problem_key(problem: &LoadedDynProblem) -> ExactProblemKey {
    ExactProblemKey::new(problem.problem_name(), problem.variant_map())
}

fn solve_customized(
    problem: &LoadedDynProblem,
    registration: &'static CustomizedSolverRegistration,
) -> Result<DeterministicSolveResult, DeterministicSolveError> {
    let config = (registration.solve_fn)(problem.as_any()).ok_or_else(|| {
        DeterministicSolveError::CustomizedNoSolution {
            problem: problem_key(problem).label(),
        }
    })?;
    let evaluation = problem.evaluate_dyn(&config);
    Ok(DeterministicSolveResult {
        solver: SolverExecution::Customized {
            implementation: registration.implementation,
        },
        config: Some(config),
        evaluation,
    })
}

fn solve_ilp(
    problem: &LoadedDynProblem,
    pipeline: &CompiledIlpPipeline,
) -> Result<DeterministicSolveResult, DeterministicSolveError> {
    let config = pipeline
        .solve(problem.as_any(), &super::ILPSolver::new())
        .map_err(|source| DeterministicSolveError::IlpSolve {
            problem: problem_key(problem).label(),
            source,
        })?;
    let evaluation = problem.evaluate_dyn(&config);
    Ok(DeterministicSolveResult {
        solver: SolverExecution::Ilp {
            reduction_path: pipeline.path_labels(),
        },
        config: Some(config),
        evaluation,
    })
}

fn solve_brute_force(problem: &LoadedDynProblem) -> DeterministicSolveResult {
    match problem.solve_brute_force_witness() {
        Some((config, evaluation)) => DeterministicSolveResult {
            solver: SolverExecution::BruteForce,
            config: Some(config),
            evaluation,
        },
        None => DeterministicSolveResult {
            solver: SolverExecution::BruteForce,
            config: None,
            evaluation: problem.solve_brute_force_value(),
        },
    }
}

/// Solve a loaded problem using deterministic exact-variant dispatch.
///
/// Default dispatch is customized, then the registered fixed ILP pipeline, then
/// brute force. Once selected, backend failure is returned without fallback.
pub fn solve_deterministically(
    problem: &LoadedDynProblem,
    request: SolverRequest,
) -> Result<DeterministicSolveResult, DeterministicSolveError> {
    if request == SolverRequest::BruteForce {
        return Ok(solve_brute_force(problem));
    }

    let registry =
        solver_capability_registry().map_err(DeterministicSolveError::InvalidRegistry)?;
    let key = problem_key(problem);
    let capabilities = registry.lookup(&key);

    match request {
        SolverRequest::BruteForce => unreachable!("handled before registry initialization"),
        SolverRequest::Customized => {
            let registration = capabilities
                .customized
                .ok_or_else(|| DeterministicSolveError::MissingCustomizedCapability(key.label()))?;
            solve_customized(problem, registration)
        }
        SolverRequest::Ilp => {
            let pipeline = capabilities
                .ilp
                .ok_or_else(|| DeterministicSolveError::MissingIlpCapability(key.label()))?;
            solve_ilp(problem, pipeline)
        }
        SolverRequest::Default => {
            if let Some(customized) = capabilities.customized {
                return solve_customized(problem, customized);
            }
            if let Some(pipeline) = capabilities.ilp {
                return solve_ilp(problem, pipeline);
            }
            Ok(solve_brute_force(problem))
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/resolver.rs"]
mod tests;
