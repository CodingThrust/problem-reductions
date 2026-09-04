//! Deterministic solver capabilities for exact problem variants.

use crate::registry::VariantEntry;
use crate::rules::registry::{reduction_entries, AggregateReduceFn, ReduceFn, ReductionEntry};
use crate::rules::DynReductionResult;
use serde::Serialize;
use std::any::Any;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

/// Canonical identity of one concrete problem variant.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct ExactProblemKey {
    pub name: String,
    pub variant: BTreeMap<String, String>,
}

impl ExactProblemKey {
    pub fn new(name: impl Into<String>, variant: BTreeMap<String, String>) -> Self {
        Self {
            name: name.into(),
            variant,
        }
    }

    fn from_static(step: &StaticProblemStep) -> Self {
        Self::new(
            step.name,
            step.variant
                .iter()
                .map(|&(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        )
    }

    /// Format the key using the catalog's canonical problem notation.
    pub fn label(&self) -> String {
        if self.variant.is_empty() {
            return self.name.clone();
        }
        let values = self
            .variant
            .values()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}<{values}>", self.name)
    }

    fn is_supported_ilp(&self) -> bool {
        self.name == "ILP"
            && matches!(
                self.variant.get("variable").map(String::as_str),
                Some("bool" | "i64")
            )
            && self.variant.get("coefficient").map(String::as_str) == Some("f64")
    }
}

/// A compile-time path node used by fixed ILP pipeline declarations.
#[derive(Clone, Copy)]
pub(crate) struct StaticProblemStep {
    pub name: &'static str,
    pub variant: &'static [(&'static str, &'static str)],
}

/// A fixed ILP pipeline declaration.
///
/// Every adjacent pair is resolved to one exact witness reduction while the
/// registry is constructed. Runtime solving executes the resolved function
/// pointers and never searches the reduction graph.
pub(crate) struct IlpPipelineRegistration {
    pub(crate) path: &'static [StaticProblemStep],
}

inventory::collect!(IlpPipelineRegistration);

type CustomizedSolveFn = fn(&dyn Any) -> Result<Option<serde_json::Value>, super::SolveError>;

/// A dedicated solver registered for one exact problem variant.
#[derive(Debug)]
pub(crate) struct CustomizedSolverRegistration {
    pub(crate) source_name: &'static str,
    pub(crate) source_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    pub(crate) implementation: &'static str,
    pub(crate) solve_fn: CustomizedSolveFn,
}

impl CustomizedSolverRegistration {
    fn source_key(&self) -> ExactProblemKey {
        ExactProblemKey::new(
            self.source_name,
            (self.source_variant_fn)()
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        )
    }
}

inventory::collect!(CustomizedSolverRegistration);

#[derive(Debug)]
pub(crate) struct CompiledIlpPipeline {
    path: Vec<ExactProblemKey>,
    reducers: Vec<(ReduceFn, Option<AggregateReduceFn>)>,
}

impl CompiledIlpPipeline {
    pub(crate) fn path(&self) -> &[ExactProblemKey] {
        &self.path
    }

    pub(crate) fn path_labels(&self) -> Vec<String> {
        self.path.iter().map(ExactProblemKey::label).collect()
    }

    fn solve_with<R>(
        &self,
        source: &dyn Any,
        solver: &super::ILPSolver,
        finish: impl FnOnce(
            Box<dyn Any>,
            Option<&dyn DynReductionResult>,
        ) -> Result<R, super::ILPSolveError>,
    ) -> Result<Option<R>, super::ILPSolveError> {
        if self.reducers.is_empty() {
            return finish(Box::new(solver.solve_dyn(source)?), None).map(Some);
        }

        let mut reductions: Vec<Box<dyn DynReductionResult>> = Vec::new();
        for (reducer, _) in &self.reducers {
            let input = reductions
                .last()
                .map(|step| step.target_problem_any())
                .unwrap_or(source);
            reductions.push(reducer(input)?);
        }

        let target = reductions
            .last()
            .expect("non-empty fixed pipeline must produce a target")
            .target_problem_any();
        let solution = solver.solve_dyn(target)?;
        let mut source_solution: Box<dyn Any> = Box::new(solution);
        for (index, step) in reductions.iter().enumerate().rev() {
            if let Some(reduce) = self.reducers[index].1 {
                let input = if index == 0 {
                    source
                } else {
                    reductions[index - 1].target_problem_any()
                };
                let aggregate = reduce(input)?;
                // Downstream reductions have recovered an optimal target witness.
                // Its aggregate may still prove that the source decision is NO.
                let value = aggregate.extract_value_from_solution_dyn(source_solution.as_ref())?;
                if value.downcast_ref::<crate::types::Or>() == Some(&crate::types::Or(false)) {
                    return Ok(None);
                }
            }
            source_solution = step.extract_solution_dyn(source_solution.as_ref())?;
        }
        finish(source_solution, Some(reductions[0].as_ref())).map(Some)
    }

    pub(crate) fn solve(
        &self,
        source: &dyn Any,
        solver: &super::ILPSolver,
    ) -> Result<Option<serde_json::Value>, super::ILPSolveError> {
        self.solve_with(source, solver, |solution, first_reduction| {
            if let Some(reduction) = first_reduction {
                return reduction
                    .source_solution_json(solution.as_ref())
                    .map_err(super::ILPSolveError::from);
            }
            Ok(serde_json::to_value(
                *solution
                    .downcast::<Vec<i64>>()
                    .expect("ILP backend returned the wrong solution type"),
            )
            .expect("ILP solution serialization failed"))
        })
    }

    pub(crate) fn solve_typed<S: 'static>(
        &self,
        source: &dyn Any,
        solver: &super::ILPSolver,
    ) -> Result<Option<S>, super::ILPSolveError> {
        self.solve_with(source, solver, |solution, _| {
            solution
                .downcast::<S>()
                .map(|solution| *solution)
                .map_err(|_| {
                    super::ILPSolveError::PipelineTypeMismatch(
                        self.path
                            .first()
                            .expect("compiled pipeline has a source")
                            .label(),
                    )
                })
        })
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RegisteredSolverCapabilities<'a> {
    pub(crate) customized: Option<&'static CustomizedSolverRegistration>,
    pub(crate) ilp: Option<&'a CompiledIlpPipeline>,
    pub(crate) brute_force: Option<&'static super::BruteForceRegistration>,
}

impl std::fmt::Debug for RegisteredSolverCapabilities<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolverCapabilities")
            .field(
                "customized",
                &self.customized.map(|entry| entry.implementation),
            )
            .field("ilp", &self.ilp.map(CompiledIlpPipeline::path))
            .field("brute_force", &self.brute_force.is_some())
            .finish()
    }
}

/// Read-only metadata for a registered customized solver.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CustomizedSolverCapability {
    pub implementation: &'static str,
}

/// Read-only metadata for a registered fixed ILP pipeline.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct IlpSolverCapability {
    path: Vec<ExactProblemKey>,
}

impl IlpSolverCapability {
    pub fn path(&self) -> &[ExactProblemKey] {
        &self.path
    }

    pub fn path_labels(&self) -> Vec<String> {
        self.path.iter().map(ExactProblemKey::label).collect()
    }
}

/// Read-only solver capabilities for one exact problem variant.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SolverCapabilities {
    pub customized: Option<CustomizedSolverCapability>,
    pub ilp: Option<IlpSolverCapability>,
    pub brute_force: bool,
}

#[derive(Debug, Default)]
pub(crate) struct SolverCapabilityRegistry {
    customized: BTreeMap<ExactProblemKey, &'static CustomizedSolverRegistration>,
    ilp: BTreeMap<ExactProblemKey, CompiledIlpPipeline>,
    brute_force: BTreeMap<ExactProblemKey, &'static super::BruteForceRegistration>,
}

impl SolverCapabilityRegistry {
    pub(crate) fn lookup(&self, key: &ExactProblemKey) -> RegisteredSolverCapabilities<'_> {
        RegisteredSolverCapabilities {
            customized: self.customized.get(key).copied(),
            ilp: self.ilp.get(key),
            brute_force: self.brute_force.get(key).copied(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryBuildError {
    #[error("solver registration references unknown exact variant {0}")]
    UnknownVariant(String),
    #[error("duplicate customized solver registration for {0}")]
    DuplicateCustomized(String),
    #[error("duplicate ILP pipeline registration for {0}")]
    DuplicateIlp(String),
    #[error("duplicate brute-force registration for {0}")]
    DuplicateBruteForce(String),
    #[error("exact variant {0} has no registered solver capability")]
    MissingSolverCapability(String),
    #[error("ILP pipeline must contain at least one node")]
    EmptyPipeline,
    #[error("ILP pipeline for {0} does not end at an f64-coefficient ILP")]
    UnsupportedTarget(String),
    #[error("ILP pipeline for {0} continues after reaching a supported ILP node")]
    ContinuesAfterIlp(String),
    #[error("ILP pipeline edge {source_label} -> {target_label} resolves to {matches} witness reductions")]
    InvalidEdge {
        source_label: String,
        target_label: String,
        matches: usize,
    },
}

fn registered_variant_keys() -> BTreeSet<ExactProblemKey> {
    inventory::iter::<VariantEntry>()
        .map(|entry| ExactProblemKey::new(entry.name, entry.variant_map()))
        .collect()
}

fn edge_key(entry: &ReductionEntry, source: bool) -> ExactProblemKey {
    let (name, variant) = if source {
        (entry.source_name, entry.source_variant())
    } else {
        (entry.target_name, entry.target_variant())
    };
    ExactProblemKey::new(
        name,
        variant
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),
    )
}

fn build_registry(
    variants: &BTreeSet<ExactProblemKey>,
    customized_entries: impl IntoIterator<Item = &'static CustomizedSolverRegistration>,
    pipeline_entries: impl IntoIterator<Item = &'static IlpPipelineRegistration>,
    brute_force_entries: impl IntoIterator<Item = &'static super::BruteForceRegistration>,
    reductions: &[&'static ReductionEntry],
) -> Result<SolverCapabilityRegistry, RegistryBuildError> {
    let mut registry = SolverCapabilityRegistry::default();
    let mut reduction_index =
        BTreeMap::<(ExactProblemKey, ExactProblemKey), Vec<&'static ReductionEntry>>::new();
    for entry in reductions
        .iter()
        .copied()
        .filter(|entry| entry.reduce_fn.is_some())
    {
        reduction_index
            .entry((edge_key(entry, true), edge_key(entry, false)))
            .or_default()
            .push(entry);
    }

    for customized in customized_entries {
        let source = customized.source_key();
        if !variants.contains(&source) {
            return Err(RegistryBuildError::UnknownVariant(source.label()));
        }
        if registry
            .customized
            .insert(source.clone(), customized)
            .is_some()
        {
            return Err(RegistryBuildError::DuplicateCustomized(source.label()));
        }
    }

    for brute_force in brute_force_entries {
        let source = ExactProblemKey::new(
            brute_force.source_name,
            crate::export::variant_to_map((brute_force.source_variant_fn)()),
        );
        if !variants.contains(&source) {
            return Err(RegistryBuildError::UnknownVariant(source.label()));
        }
        if registry
            .brute_force
            .insert(source.clone(), brute_force)
            .is_some()
        {
            return Err(RegistryBuildError::DuplicateBruteForce(source.label()));
        }
    }

    for registration in pipeline_entries {
        let path = registration
            .path
            .iter()
            .map(ExactProblemKey::from_static)
            .collect::<Vec<_>>();
        let source = path
            .first()
            .cloned()
            .ok_or(RegistryBuildError::EmptyPipeline)?;

        for step in &path {
            if !variants.contains(step) {
                return Err(RegistryBuildError::UnknownVariant(step.label()));
            }
        }
        if !path.last().is_some_and(ExactProblemKey::is_supported_ilp) {
            return Err(RegistryBuildError::UnsupportedTarget(source.label()));
        }
        if path[..path.len() - 1]
            .iter()
            .any(ExactProblemKey::is_supported_ilp)
        {
            return Err(RegistryBuildError::ContinuesAfterIlp(source.label()));
        }

        let mut reducers = Vec::with_capacity(path.len().saturating_sub(1));
        for pair in path.windows(2) {
            let matches = reduction_index
                .get(&(pair[0].clone(), pair[1].clone()))
                .map(Vec::as_slice)
                .unwrap_or_default();
            if matches.len() != 1 {
                return Err(RegistryBuildError::InvalidEdge {
                    source_label: pair[0].label(),
                    target_label: pair[1].label(),
                    matches: matches.len(),
                });
            }
            reducers.push((
                matches[0]
                    .reduce_fn
                    .expect("indexed only entries with reduce_fn"),
                matches[0].reduce_aggregate_fn,
            ));
        }

        if registry
            .ilp
            .insert(source.clone(), CompiledIlpPipeline { path, reducers })
            .is_some()
        {
            return Err(RegistryBuildError::DuplicateIlp(source.label()));
        }
    }

    for variant in variants {
        if !registry.customized.contains_key(variant)
            && !registry.ilp.contains_key(variant)
            && !registry.brute_force.contains_key(variant)
        {
            return Err(RegistryBuildError::MissingSolverCapability(variant.label()));
        }
    }

    Ok(registry)
}

static REGISTRY: OnceLock<Result<SolverCapabilityRegistry, RegistryBuildError>> = OnceLock::new();

pub(crate) fn solver_capability_registry(
) -> Result<&'static SolverCapabilityRegistry, &'static RegistryBuildError> {
    REGISTRY
        .get_or_init(|| {
            build_registry(
                &registered_variant_keys(),
                inventory::iter::<CustomizedSolverRegistration>(),
                inventory::iter::<IlpPipelineRegistration>(),
                inventory::iter::<super::BruteForceRegistration>(),
                &reduction_entries(),
            )
        })
        .as_ref()
}

/// Return read-only solver metadata for one exact problem variant.
pub fn solver_capabilities(
    key: &ExactProblemKey,
) -> Result<SolverCapabilities, &'static RegistryBuildError> {
    let registered = solver_capability_registry()?.lookup(key);
    Ok(SolverCapabilities {
        customized: registered
            .customized
            .map(|entry| CustomizedSolverCapability {
                implementation: entry.implementation,
            }),
        ilp: registered.ilp.map(|pipeline| IlpSolverCapability {
            path: pipeline.path.clone(),
        }),
        brute_force: registered.brute_force.is_some(),
    })
}

pub(crate) fn brute_force_registration(
    key: &ExactProblemKey,
) -> Result<Option<&'static super::BruteForceRegistration>, &'static RegistryBuildError> {
    Ok(solver_capability_registry()?.lookup(key).brute_force)
}

/// Return the finite Cartesian dimensions registered for a loaded problem.
#[doc(hidden)]
pub fn brute_force_dimensions(
    problem: &crate::registry::LoadedDynProblem,
) -> Result<Option<Vec<usize>>, &'static RegistryBuildError> {
    let key = ExactProblemKey::new(problem.problem_name(), problem.variant_map());
    Ok(brute_force_registration(&key)?
        .map(|registration| (registration.dimensions_fn)(problem.as_any())))
}

#[cfg(test)]
#[path = "../unit_tests/solvers/registry.rs"]
mod tests;
