//! Deterministic solver capabilities for exact problem variants.

use crate::registry::VariantEntry;
use crate::rules::registry::{reduction_entries, ReduceFn, ReductionEntry};
#[cfg(feature = "ilp-solver")]
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
                Some("bool" | "i32")
            )
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

type NativeSolveFn = fn(&dyn Any) -> Option<Vec<usize>>;

/// A dedicated solver registered for one exact problem variant.
#[derive(Debug)]
pub(crate) struct NativeSolverRegistration {
    pub(crate) source_name: &'static str,
    pub(crate) source_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    pub(crate) implementation: &'static str,
    pub(crate) solve_fn: NativeSolveFn,
}

impl NativeSolverRegistration {
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

inventory::collect!(NativeSolverRegistration);

#[derive(Debug)]
pub(crate) struct CompiledIlpPipeline {
    path: Vec<ExactProblemKey>,
    reducers: Vec<ReduceFn>,
}

impl CompiledIlpPipeline {
    pub(crate) fn path(&self) -> &[ExactProblemKey] {
        &self.path
    }

    pub(crate) fn path_labels(&self) -> Vec<String> {
        self.path.iter().map(ExactProblemKey::label).collect()
    }

    #[cfg(feature = "ilp-solver")]
    pub(crate) fn solve(
        &self,
        source: &dyn Any,
        solver: &super::ILPSolver,
    ) -> Result<Vec<usize>, super::ILPSolveError> {
        if self.reducers.is_empty() {
            return solver.solve_dyn(source);
        }

        let mut reductions: Vec<Box<dyn DynReductionResult>> = Vec::new();
        for reducer in &self.reducers {
            let input = reductions
                .last()
                .map(|step| step.target_problem_any())
                .unwrap_or(source);
            reductions.push(reducer(input));
        }

        let target = reductions
            .last()
            .expect("non-empty fixed pipeline must produce a target")
            .target_problem_any();
        let solution = solver.solve_dyn(target)?;
        Ok(reductions.iter().rev().fold(solution, |current, step| {
            step.extract_solution_dyn(&current)
        }))
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RegisteredSolverCapabilities<'a> {
    pub(crate) native: Option<&'static NativeSolverRegistration>,
    pub(crate) ilp: Option<&'a CompiledIlpPipeline>,
}

impl std::fmt::Debug for RegisteredSolverCapabilities<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolverCapabilities")
            .field("native", &self.native.map(|entry| entry.implementation))
            .field("ilp", &self.ilp.map(CompiledIlpPipeline::path))
            .finish()
    }
}

/// Read-only metadata for a registered native solver.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NativeSolverCapability {
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
    pub native: Option<NativeSolverCapability>,
    pub ilp: Option<IlpSolverCapability>,
}

#[derive(Debug, Default)]
pub(crate) struct SolverCapabilityRegistry {
    native: BTreeMap<ExactProblemKey, &'static NativeSolverRegistration>,
    ilp: BTreeMap<ExactProblemKey, CompiledIlpPipeline>,
}

impl SolverCapabilityRegistry {
    pub(crate) fn lookup(&self, key: &ExactProblemKey) -> RegisteredSolverCapabilities<'_> {
        RegisteredSolverCapabilities {
            native: self.native.get(key).copied(),
            ilp: self.ilp.get(key),
        }
    }

    #[cfg(test)]
    pub(crate) fn native_entries(
        &self,
    ) -> impl Iterator<Item = (&ExactProblemKey, &'static NativeSolverRegistration)> + '_ {
        self.native.iter().map(|(key, entry)| (key, *entry))
    }

    #[cfg(test)]
    pub(crate) fn ilp_entries(
        &self,
    ) -> impl Iterator<Item = (&ExactProblemKey, &CompiledIlpPipeline)> {
        self.ilp.iter()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryBuildError {
    #[error("solver registration references unknown exact variant {0}")]
    UnknownVariant(String),
    #[error("duplicate native solver registration for {0}")]
    DuplicateNative(String),
    #[error("duplicate ILP pipeline registration for {0}")]
    DuplicateIlp(String),
    #[error("ILP pipeline must contain at least one node")]
    EmptyPipeline,
    #[error("ILP pipeline for {0} does not end at ILP<bool> or ILP<i32>")]
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
    native_entries: impl IntoIterator<Item = &'static NativeSolverRegistration>,
    pipeline_entries: impl IntoIterator<Item = &'static IlpPipelineRegistration>,
    reductions: &[&'static ReductionEntry],
) -> Result<SolverCapabilityRegistry, RegistryBuildError> {
    let mut registry = SolverCapabilityRegistry::default();
    let mut reduction_index =
        BTreeMap::<(ExactProblemKey, ExactProblemKey), Vec<&'static ReductionEntry>>::new();
    for entry in reductions
        .iter()
        .copied()
        .filter(|entry| entry.capabilities.witness && entry.reduce_fn.is_some())
    {
        reduction_index
            .entry((edge_key(entry, true), edge_key(entry, false)))
            .or_default()
            .push(entry);
    }

    for native in native_entries {
        let source = native.source_key();
        if !variants.contains(&source) {
            return Err(RegistryBuildError::UnknownVariant(source.label()));
        }
        if registry.native.insert(source.clone(), native).is_some() {
            return Err(RegistryBuildError::DuplicateNative(source.label()));
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
            reducers.push(
                matches[0]
                    .reduce_fn
                    .expect("indexed only entries with reduce_fn"),
            );
        }

        if registry
            .ilp
            .insert(source.clone(), CompiledIlpPipeline { path, reducers })
            .is_some()
        {
            return Err(RegistryBuildError::DuplicateIlp(source.label()));
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
                inventory::iter::<NativeSolverRegistration>(),
                inventory::iter::<IlpPipelineRegistration>(),
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
        native: registered.native.map(|entry| NativeSolverCapability {
            implementation: entry.implementation,
        }),
        ilp: registered.ilp.map(|pipeline| IlpSolverCapability {
            path: pipeline.path.clone(),
        }),
    })
}

#[cfg(test)]
#[path = "../unit_tests/solvers/registry.rs"]
mod tests;
