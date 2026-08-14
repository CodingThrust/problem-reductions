//! Automatic reduction registration via inventory.

use crate::expr::Expr;
use crate::rules::traits::{DynAggregateReductionResult, DynReductionResult};
use crate::size::{SizeRelation, SizeTransform, SizeTransformError};
use crate::types::ProblemSize;
use std::any::Any;
use std::collections::HashSet;

/// One target field whose size cannot be propagated through a reduction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct UnavailableSizeField {
    pub field: &'static str,
    pub reason: &'static str,
}

/// Raw symbolic declaration emitted by the reduction proc macro.
#[derive(Clone, Debug, Default)]
pub struct ReductionSizeDeclarations {
    pub relation: Option<SizeRelation>,
    pub fields: Vec<(&'static str, Expr)>,
    pub unavailable: Vec<UnavailableSizeField>,
}

/// Validated size metadata for one reduction edge.
#[derive(Clone, Debug)]
pub struct ReductionSizeContract {
    transform: Option<SizeTransform>,
    unavailable: Vec<UnavailableSizeField>,
}

impl ReductionSizeContract {
    pub fn new(
        edge: impl Into<Box<str>>,
        declarations: ReductionSizeDeclarations,
    ) -> Result<Self, SizeContractError> {
        let edge = edge.into();
        let formula_names: HashSet<_> = declarations
            .fields
            .iter()
            .map(|(field, _)| *field)
            .collect();
        let mut unavailable_names = HashSet::new();
        for unavailable in &declarations.unavailable {
            if unavailable.reason.trim().is_empty() {
                return Err(SizeContractError::EmptyUnavailableReason {
                    edge,
                    field: unavailable.field.into(),
                });
            }
            if !unavailable_names.insert(unavailable.field)
                || formula_names.contains(unavailable.field)
            {
                return Err(SizeContractError::DuplicateClassification {
                    edge,
                    field: unavailable.field.into(),
                });
            }
        }
        let transform = match (declarations.relation, declarations.fields.is_empty()) {
            (Some(relation), false) => {
                Some(SizeTransform::new(edge, relation, declarations.fields)?)
            }
            (None, true) if !declarations.unavailable.is_empty() => None,
            (None, true) => return Err(SizeContractError::EmptyContract { edge }),
            (Some(_), true) => return Err(SizeContractError::EmptyTransform { edge }),
            (None, false) => return Err(SizeContractError::MissingRelation { edge }),
        };
        Ok(Self {
            transform,
            unavailable: declarations.unavailable,
        })
    }

    pub fn transform(&self) -> Option<&SizeTransform> {
        self.transform.as_ref()
    }

    pub fn unavailable(&self) -> &[UnavailableSizeField] {
        &self.unavailable
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SizeContractError {
    Transform(SizeTransformError),
    EmptyContract { edge: Box<str> },
    EmptyTransform { edge: Box<str> },
    MissingRelation { edge: Box<str> },
    DuplicateClassification { edge: Box<str>, field: Box<str> },
    EmptyUnavailableReason { edge: Box<str>, field: Box<str> },
}

impl std::fmt::Display for SizeContractError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transform(error) => write!(formatter, "invalid size transform: {error}"),
            Self::EmptyContract { edge } => write!(
                formatter,
                "reduction `{edge}` has no size formulas or unavailable fields"
            ),
            Self::EmptyTransform { edge } => {
                write!(
                    formatter,
                    "reduction `{edge}` declares an empty size transform"
                )
            }
            Self::MissingRelation { edge } => write!(
                formatter,
                "reduction `{edge}` declares size formulas without a relation"
            ),
            Self::DuplicateClassification { edge, field } => {
                write!(
                    formatter,
                    "reduction `{edge}` classifies target field `{field}` more than once"
                )
            }
            Self::EmptyUnavailableReason { edge, field } => write!(
                formatter,
                "reduction `{edge}` marks target field `{field}` unavailable without a reason"
            ),
        }
    }
}

impl std::error::Error for SizeContractError {}

impl From<SizeTransformError> for SizeContractError {
    fn from(error: SizeTransformError) -> Self {
        Self::Transform(error)
    }
}

/// Witness/config reduction executor stored in the inventory.
pub type ReduceFn = fn(&dyn Any) -> Box<dyn DynReductionResult>;

/// Aggregate/value reduction executor stored in the inventory.
pub type AggregateReduceFn = fn(&dyn Any) -> Box<dyn DynAggregateReductionResult>;

/// Execution capabilities carried by a reduction edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EdgeCapabilities {
    pub witness: bool,
    pub aggregate: bool,
    /// Turing (multi-query) reduction: solving the source requires multiple
    /// adaptive queries to the target (e.g., binary search over a decision bound).
    #[serde(default)]
    pub turing: bool,
}

impl EdgeCapabilities {
    pub(crate) const fn from_executors(
        reduce_fn: Option<ReduceFn>,
        reduce_aggregate_fn: Option<AggregateReduceFn>,
        turing: bool,
    ) -> Self {
        Self {
            witness: reduce_fn.is_some(),
            aggregate: reduce_aggregate_fn.is_some(),
            turing,
        }
    }
}

/// A registered reduction entry for static inventory registration.
/// Uses function pointers to lazily derive variant fields from `Problem::variant()`.
pub struct ReductionEntry {
    /// Base name of source problem (e.g., "MaximumIndependentSet").
    pub source_name: &'static str,
    /// Base name of target problem (e.g., "MinimumVertexCover").
    pub target_name: &'static str,
    /// Function to derive source variant attributes from `Problem::variant()`.
    pub source_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Function to derive target variant attributes from `Problem::variant()`.
    pub target_variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// The rule's single size relation, formulas, and unavailable target fields.
    pub size_declarations_fn: fn() -> ReductionSizeDeclarations,
    /// Module path where the reduction is defined (from `module_path!()`).
    pub module_path: &'static str,
    /// Type-erased reduction executor.
    /// Takes a `&dyn Any` (must be `&SourceType`), calls `ReduceTo::reduce_to()`,
    /// and returns the result as a boxed `DynReductionResult`.
    pub reduce_fn: Option<ReduceFn>,
    /// Type-erased aggregate reduction executor.
    /// Takes a `&dyn Any` (must be `&SourceType`), calls
    /// `ReduceToAggregate::reduce_to_aggregate()`, and returns the result as a
    /// boxed `DynAggregateReductionResult`.
    pub reduce_aggregate_fn: Option<AggregateReduceFn>,
    /// Whether this is a Turing (multi-query) reduction.
    pub turing: bool,
    /// Measure the source fields referenced by this rule's size formulas.
    pub source_size_measure_fn: fn(&dyn Any) -> ProblemSize,
    /// Measure the target fields declared by this rule's size contract.
    pub target_size_measure_fn: fn(&dyn Any) -> ProblemSize,
}

impl ReductionEntry {
    pub fn size_contract(&self) -> Result<ReductionSizeContract, SizeContractError> {
        let edge: Box<str> = format!("{} -> {}", self.source_name, self.target_name).into();
        ReductionSizeContract::new(edge, (self.size_declarations_fn)())
    }

    /// Get the source variant by calling the function.
    pub fn source_variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.source_variant_fn)()
    }

    /// Get the target variant by calling the function.
    pub fn target_variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.target_variant_fn)()
    }

    /// Return the modes backed by this entry's executors.
    pub fn capabilities(&self) -> EdgeCapabilities {
        EdgeCapabilities::from_executors(self.reduce_fn, self.reduce_aggregate_fn, self.turing)
    }

    /// Check if this reduction involves only the base (unweighted) variants.
    pub fn is_base_reduction(&self) -> bool {
        let source = self.source_variant();
        let target = self.target_variant();
        let source_unweighted = source
            .iter()
            .find(|(k, _)| *k == "weight")
            .map(|(_, v)| *v == "One")
            .unwrap_or(true);
        let target_unweighted = target
            .iter()
            .find(|(k, _)| *k == "weight")
            .map(|(_, v)| *v == "One")
            .unwrap_or(true);
        source_unweighted && target_unweighted
    }
}

impl std::fmt::Debug for ReductionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReductionEntry")
            .field("source_name", &self.source_name)
            .field("target_name", &self.target_name)
            .field("source_variant", &self.source_variant())
            .field("target_variant", &self.target_variant())
            .field("size_contract", &self.size_contract())
            .field("module_path", &self.module_path)
            .field("capabilities", &self.capabilities())
            .finish()
    }
}

inventory::collect!(ReductionEntry);

/// Return all registered reduction entries.
pub fn reduction_entries() -> Vec<&'static ReductionEntry> {
    inventory::iter::<ReductionEntry>().collect()
}

#[cfg(test)]
#[path = "../unit_tests/rules/registry.rs"]
mod tests;
