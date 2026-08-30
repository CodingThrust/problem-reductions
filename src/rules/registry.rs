//! Automatic reduction registration via inventory.

use crate::expr::Expr;
use crate::parameters::{ParameterRelation, ParameterTransform, ParameterTransformError};
use crate::rules::traits::{DynAggregateReductionResult, DynReductionResult};
use std::any::Any;
use std::collections::HashSet;

/// One target parameter that cannot be propagated through a reduction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct UnavailableParameterField {
    pub field: &'static str,
    pub reason: &'static str,
}

/// Raw symbolic declaration emitted by the reduction proc macro.
#[derive(Clone, Debug, Default)]
pub struct ReductionParameterDeclarations {
    pub relation: Option<ParameterRelation>,
    pub fields: Vec<(&'static str, Expr)>,
    pub unavailable: Vec<UnavailableParameterField>,
}

/// Validated parameter metadata for one reduction edge.
#[derive(Clone, Debug)]
pub struct ReductionParameterContract {
    transform: Option<ParameterTransform>,
    unavailable: Vec<UnavailableParameterField>,
}

impl ReductionParameterContract {
    pub fn new(
        edge: impl Into<Box<str>>,
        declarations: ReductionParameterDeclarations,
    ) -> Result<Self, ParameterContractError> {
        let edge = edge.into();
        let formula_names: HashSet<_> = declarations
            .fields
            .iter()
            .map(|(field, _)| *field)
            .collect();
        let mut unavailable_names = HashSet::new();
        for unavailable in &declarations.unavailable {
            if unavailable.reason.trim().is_empty() {
                return Err(ParameterContractError::EmptyUnavailableReason {
                    edge,
                    field: unavailable.field.into(),
                });
            }
            if !unavailable_names.insert(unavailable.field)
                || formula_names.contains(unavailable.field)
            {
                return Err(ParameterContractError::DuplicateClassification {
                    edge,
                    field: unavailable.field.into(),
                });
            }
        }
        let transform = match (declarations.relation, declarations.fields.is_empty()) {
            (Some(relation), false) => Some(ParameterTransform::new(
                edge,
                relation,
                declarations.fields,
            )?),
            (None, true) if !declarations.unavailable.is_empty() => None,
            (None, true) => return Err(ParameterContractError::EmptyContract { edge }),
            (Some(_), true) => return Err(ParameterContractError::EmptyTransform { edge }),
            (None, false) => return Err(ParameterContractError::MissingRelation { edge }),
        };
        Ok(Self {
            transform,
            unavailable: declarations.unavailable,
        })
    }

    pub fn transform(&self) -> Option<&ParameterTransform> {
        self.transform.as_ref()
    }

    pub fn unavailable(&self) -> &[UnavailableParameterField] {
        &self.unavailable
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParameterContractError {
    Transform(ParameterTransformError),
    EmptyContract { edge: Box<str> },
    EmptyTransform { edge: Box<str> },
    MissingRelation { edge: Box<str> },
    DuplicateClassification { edge: Box<str>, field: Box<str> },
    EmptyUnavailableReason { edge: Box<str>, field: Box<str> },
}

impl std::fmt::Display for ParameterContractError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transform(error) => write!(formatter, "invalid parameter transform: {error}"),
            Self::EmptyContract { edge } => write!(
                formatter,
                "reduction `{edge}` has no parameter formulas or unavailable fields"
            ),
            Self::EmptyTransform { edge } => {
                write!(
                    formatter,
                    "reduction `{edge}` declares an empty parameter transform"
                )
            }
            Self::MissingRelation { edge } => write!(
                formatter,
                "reduction `{edge}` declares parameter formulas without a relation"
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

impl std::error::Error for ParameterContractError {}

impl From<ParameterTransformError> for ParameterContractError {
    fn from(error: ParameterTransformError) -> Self {
        Self::Transform(error)
    }
}

/// Witness/config reduction executor stored in the inventory.
pub type ReduceFn =
    fn(&dyn Any) -> Result<Box<dyn DynReductionResult>, crate::rules::ReductionError>;

/// Aggregate/value reduction executor stored in the inventory.
pub type AggregateReduceFn =
    fn(&dyn Any) -> Result<Box<dyn DynAggregateReductionResult>, crate::rules::ReductionError>;

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
    /// The rule's single parameter relation, formulas, and unavailable target fields.
    pub parameter_declarations_fn: fn() -> ReductionParameterDeclarations,
    /// Module path where the reduction is defined (from `module_path!()`).
    pub module_path: &'static str,
    /// Type-erased reduction executor.
    /// Takes a `&dyn Any` (must be `&SourceType`), calls `ReduceTo::reduce_to()`,
    /// and returns either a boxed `DynReductionResult` or the edge's `ReductionError`.
    pub reduce_fn: Option<ReduceFn>,
    /// Type-erased aggregate reduction executor.
    /// Takes a `&dyn Any` (must be `&SourceType`), calls
    /// `ReduceToAggregate::reduce_to_aggregate()`, and returns either a boxed
    /// `DynAggregateReductionResult` or the edge's `ReductionError`.
    pub reduce_aggregate_fn: Option<AggregateReduceFn>,
    /// Whether this is a Turing (multi-query) reduction.
    pub turing: bool,
}

impl ReductionEntry {
    pub fn parameter_contract(&self) -> Result<ReductionParameterContract, ParameterContractError> {
        let edge: Box<str> = format!("{} -> {}", self.source_name, self.target_name).into();
        ReductionParameterContract::new(edge, (self.parameter_declarations_fn)())
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
            .field("parameter_contract", &self.parameter_contract())
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

/// Validate reduction parameter expressions against problem-owned endpoint schemas.
pub fn validate_reduction_parameter_schemas() -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for entry in inventory::iter::<ReductionEntry> {
        let source_variant = crate::export::variant_to_map(entry.source_variant());
        let target_variant = crate::export::variant_to_map(entry.target_variant());
        let Some(source) = crate::registry::find_variant_entry(entry.source_name, &source_variant)
        else {
            errors.push(format!(
                "{} -> {} references an unregistered source variant {source_variant:?}",
                entry.source_name, entry.target_name
            ));
            continue;
        };
        let Some(target) = crate::registry::find_variant_entry(entry.target_name, &target_variant)
        else {
            errors.push(format!(
                "{} -> {} references an unregistered target variant {target_variant:?}",
                entry.source_name, entry.target_name
            ));
            continue;
        };

        let source_fields = source
            .parameter_names()
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let target_fields = target
            .parameter_names()
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let declarations = (entry.parameter_declarations_fn)();

        for field in declarations
            .fields
            .iter()
            .flat_map(|(_, expression)| expression.variables())
        {
            if !source_fields.contains(field) {
                errors.push(format!(
                    "{} -> {} references unknown source parameter `{field}`; declared: {source_fields:?}",
                    entry.source_name, entry.target_name
                ));
            }
        }

        let declared_target_fields = declarations
            .fields
            .iter()
            .map(|(field, _)| *field)
            .chain(declarations.unavailable.iter().map(|field| field.field))
            .collect::<std::collections::BTreeSet<_>>();
        for field in &declared_target_fields {
            if !target_fields.contains(field) {
                errors.push(format!(
                    "{} -> {} declares unknown target parameter `{field}`; declared: {target_fields:?}",
                    entry.source_name, entry.target_name
                ));
            }
        }
        for field in target_fields.difference(&declared_target_fields) {
            errors.push(format!(
                "{} -> {} omits target parameter `{field}`",
                entry.source_name, entry.target_name
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        errors.sort();
        Err(errors)
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/registry.rs"]
mod tests;
