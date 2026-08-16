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
    pub outcome: SolveOutcome,
}

/// Semantic result of a completed exact solve.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum SolveOutcome {
    /// The exact optimum was established. Aggregate-only problems do not
    /// provide a witness configuration.
    Optimal {
        #[serde(rename = "solution", skip_serializing_if = "Option::is_none")]
        config: Option<Vec<usize>>,
        evaluation: String,
    },
    /// The solver proved that the instance has no feasible configuration.
    Infeasible,
}

#[derive(Debug, thiserror::Error)]
pub enum DeterministicSolveError {
    #[error("solver capability registry is invalid: {0}")]
    InvalidRegistry(&'static RegistryBuildError),
    #[error("No ILP pipeline is registered for {0}")]
    MissingIlpCapability(String),
    #[error("No customized solver is registered for {0}")]
    MissingCustomizedCapability(String),
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
    let outcome = match (registration.solve_fn)(problem.as_any()) {
        Some(config) => SolveOutcome::Optimal {
            evaluation: problem.evaluate_dyn(&config),
            config: Some(config),
        },
        None => SolveOutcome::Infeasible,
    };
    Ok(DeterministicSolveResult {
        solver: SolverExecution::Customized {
            implementation: registration.implementation,
        },
        outcome,
    })
}

fn solve_ilp(
    problem: &LoadedDynProblem,
    pipeline: &CompiledIlpPipeline,
) -> Result<DeterministicSolveResult, DeterministicSolveError> {
    let outcome = match pipeline.solve(problem.as_any(), &super::ILPSolver::new()) {
        Ok(config) => SolveOutcome::Optimal {
            evaluation: problem.evaluate_dyn(&config),
            config: Some(config),
        },
        Err(super::ILPSolveError::Infeasible) => SolveOutcome::Infeasible,
        Err(source) => {
            return Err(DeterministicSolveError::IlpSolve {
                problem: problem_key(problem).label(),
                source,
            });
        }
    };
    Ok(DeterministicSolveResult {
        solver: SolverExecution::Ilp {
            reduction_path: pipeline.path_labels(),
        },
        outcome,
    })
}

fn solve_brute_force(problem: &LoadedDynProblem) -> DeterministicSolveResult {
    let outcome = match problem.solve_brute_force_witness() {
        Some((config, evaluation)) => SolveOutcome::Optimal {
            config: Some(config),
            evaluation,
        },
        None if problem.supports_witnesses_dyn() => SolveOutcome::Infeasible,
        None => SolveOutcome::Optimal {
            config: None,
            evaluation: problem.solve_brute_force_value(),
        },
    };
    DeterministicSolveResult {
        solver: SolverExecution::BruteForce,
        outcome,
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
