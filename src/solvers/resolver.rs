//! Shared deterministic solver dispatch.

#[cfg(feature = "ilp-solver")]
use super::registry::CompiledIlpPipeline;
use super::registry::{
    solver_capability_registry, ExactProblemKey, NativeSolverRegistration, RegistryBuildError,
};
use crate::registry::LoadedDynProblem;
use serde::Serialize;

/// Public solver override. Omission is represented by [`SolverRequest::Default`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SolverRequest {
    #[default]
    Default,
    Ilp,
    BruteForce,
}

/// Information about the backend execution that produced a solve result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SolverExecution {
    Native { implementation: &'static str },
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
    #[error("native solver found no solution for {problem}")]
    NativeNoSolution { problem: String },
    #[cfg(feature = "ilp-solver")]
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

fn solve_native(
    problem: &LoadedDynProblem,
    registration: &'static NativeSolverRegistration,
) -> Result<DeterministicSolveResult, DeterministicSolveError> {
    let config = (registration.solve_fn)(problem.as_any()).ok_or_else(|| {
        DeterministicSolveError::NativeNoSolution {
            problem: problem_key(problem).label(),
        }
    })?;
    let evaluation = problem.evaluate_dyn(&config);
    Ok(DeterministicSolveResult {
        solver: SolverExecution::Native {
            implementation: registration.implementation,
        },
        config: Some(config),
        evaluation,
    })
}

#[cfg(feature = "ilp-solver")]
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
/// Default dispatch is native, then the registered fixed ILP pipeline, then
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
        SolverRequest::Ilp => {
            let pipeline = capabilities
                .ilp
                .ok_or_else(|| DeterministicSolveError::MissingIlpCapability(key.label()))?;
            #[cfg(feature = "ilp-solver")]
            {
                solve_ilp(problem, pipeline)
            }
            #[cfg(not(feature = "ilp-solver"))]
            {
                let _ = pipeline;
                Err(DeterministicSolveError::MissingIlpCapability(key.label()))
            }
        }
        SolverRequest::Default => {
            if let Some(native) = capabilities.native {
                return solve_native(problem, native);
            }
            if let Some(pipeline) = capabilities.ilp {
                #[cfg(feature = "ilp-solver")]
                {
                    return solve_ilp(problem, pipeline);
                }
                #[cfg(not(feature = "ilp-solver"))]
                let _ = pipeline;
            }
            Ok(solve_brute_force(problem))
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/solvers/resolver.rs"]
mod tests;
