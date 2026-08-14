//! Explicit variant registration via inventory.

use std::any::Any;
use std::collections::BTreeMap;

use crate::registry::dyn_problem::{DynProblem, SolveValueFn, SolveWitnessFn};
use crate::registry::FieldInfo;

/// Reusable syntax used to transport one construction input.
///
/// `Auto` asks a frontend to choose the codec from `type_name`. The explicit
/// variants are for Rust types whose compact external syntax is ambiguous.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CreateInputCodec {
    /// Infer the transport syntax from the Rust value type.
    #[default]
    Auto,
    /// A single scalar value.
    Scalar,
    /// A JSON value.
    Json,
    /// Comma-separated values.
    CommaSeparated,
    /// Semicolon-separated rows or groups.
    SemicolonSeparated,
    /// Undirected edges such as `0-1,1-2`.
    EdgeList,
    /// Directed arcs such as `0>1,1>2`.
    ArcList,
    /// Bipartite-local edges such as `0-0,0-1`.
    BipartiteEdgeList,
    /// Equality-linked index pairs such as `2=5;4=3`.
    EqualityPairList,
    /// Functional dependencies such as `0,1:2;2:3,4`.
    FunctionalDependencyList,
    /// Semicolon-separated character strings sharing one inferred alphabet.
    CharacterRows,
}

/// A user-facing input accepted when constructing a problem instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreateInputInfo {
    /// Input name in snake_case. Frontends may render it in their native style.
    pub name: &'static str,
    /// Concrete Rust value type accepted by the construction spec.
    pub type_name: &'static str,
    /// Human-readable input description.
    pub description: &'static str,
    /// Whether the input must be present.
    pub required: bool,
    /// Reusable transport syntax for this input.
    pub codec: CreateInputCodec,
}

impl CreateInputInfo {
    /// Promote catalog field metadata into a required construction input.
    pub const fn from_field(field: FieldInfo) -> Self {
        Self {
            name: field.name,
            type_name: field.type_name,
            description: field.description,
            required: true,
            codec: CreateInputCodec::Auto,
        }
    }
}

/// Static construction-input metadata generated from a typed create spec.
pub trait CreateSpec {
    /// Construction-facing field metadata used by the problem catalog.
    const FIELDS: &'static [FieldInfo];
    /// Inputs accepted by this construction spec.
    const INPUTS: &'static [CreateInputInfo];

    /// Deserialize normalized construction inputs into the typed specification.
    fn deserialize_inputs(data: serde_json::Value) -> Result<Self, serde_json::Error>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        serde_json::from_value(data)
    }
}

/// Failure while validating or applying a model construction contract.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConstructionError {
    /// No concrete variant matches the requested problem reference.
    #[error("no registered variant for `{name}` with variant {variant:?}")]
    UnregisteredVariant {
        /// Canonical problem name.
        name: String,
        /// Exact requested variant.
        variant: BTreeMap<String, String>,
    },
    /// Construction values must be supplied as a named JSON object.
    #[error("construction inputs must be a JSON object")]
    ExpectedObject,
    /// A construction contract declared the same input more than once.
    #[error("construction input `{0}` is declared more than once")]
    DuplicateInput(String),
    /// The caller supplied values outside the declared construction contract.
    #[error("unknown construction input(s): {}", .0.join(", "))]
    UnknownInputs(Vec<String>),
    /// The caller omitted required construction values.
    #[error("missing required construction input(s): {}", .0.join(", "))]
    MissingInputs(Vec<String>),
    /// Normalized values could not be deserialized into the direct model or create spec.
    #[error("invalid construction input: {0}")]
    InvalidInput(String),
    /// A typed create spec failed to convert into the problem model.
    #[error("problem construction failed: {0}")]
    Conversion(String),
}

/// Type-erased problem constructor used by dynamic frontends.
pub type ConstructProblemFn =
    fn(serde_json::Value) -> Result<Box<dyn DynProblem>, ConstructionError>;

/// Random-generation contract for one concrete problem variant.
#[derive(Clone, Copy)]
pub struct RandomRegistration {
    /// Inputs accepted by the generator.
    pub inputs: &'static [CreateInputInfo],
    /// Generate a concrete problem from normalized inputs.
    pub generate: ConstructProblemFn,
}

/// A concrete problem type that can generate itself from typed random inputs.
pub trait RandomGenerate: DynProblem + Sized {
    /// Inputs accepted by this model's random generator.
    const INPUTS: &'static [CreateInputInfo];

    /// Generate a concrete problem from normalized random inputs.
    fn generate(data: serde_json::Value) -> Result<Self, ConstructionError>;
}

/// Validate normalized values against a typed construction contract.
pub fn validate_create_inputs(
    inputs: &[CreateInputInfo],
    data: &serde_json::Value,
) -> Result<(), ConstructionError> {
    validate_input_contract(
        inputs.iter().map(|input| (input.name, input.required)),
        data,
    )
}

/// Validate the direct-construction path backed by catalog field metadata.
///
/// Direct models have no separate create DTO, so every catalog field is a
/// required construction input.
pub fn validate_direct_create_inputs(
    fields: &[FieldInfo],
    data: &serde_json::Value,
) -> Result<(), ConstructionError> {
    validate_input_contract(fields.iter().map(|field| (field.name, true)), data)
}

fn validate_input_contract<'a>(
    inputs: impl IntoIterator<Item = (&'a str, bool)>,
    data: &serde_json::Value,
) -> Result<(), ConstructionError> {
    let object = data.as_object().ok_or(ConstructionError::ExpectedObject)?;
    let mut declared = BTreeMap::new();
    for (name, required) in inputs {
        if declared.insert(name, required).is_some() {
            return Err(ConstructionError::DuplicateInput(name.to_string()));
        }
    }

    let unknown = object
        .keys()
        .filter(|name| !declared.contains_key(name.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if !unknown.is_empty() {
        return Err(ConstructionError::UnknownInputs(unknown));
    }

    let missing = declared
        .into_iter()
        .filter(|(name, required)| *required && !object.contains_key(*name))
        .map(|(name, _)| name.to_string())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(ConstructionError::MissingInputs(missing));
    }

    Ok(())
}

/// A registered problem variant entry.
///
/// Submitted by [`declare_variants!`] for each concrete problem type.
/// The reduction graph uses these entries to build nodes with complexity metadata.
pub struct VariantEntry {
    /// Problem name (from `Problem::NAME`).
    pub name: &'static str,
    /// Function returning variant key-value pairs (from `Problem::variant()`).
    pub variant_fn: fn() -> Vec<(&'static str, &'static str)>,
    /// Worst-case time complexity expression (e.g., `"2^num_vertices"`).
    pub complexity: &'static str,
    /// Compiled complexity evaluation function.
    /// Takes a `&dyn Any` (must be `&ProblemType`), calls getter methods directly,
    /// and returns the estimated worst-case time as f64.
    pub complexity_eval_fn: fn(&dyn Any) -> f64,
    /// Whether this entry is the declared default variant for its problem.
    pub is_default: bool,
    /// Variant-level aliases (e.g., `&["3SAT"]` for `KSatisfiability<K3>`).
    ///
    /// Unlike problem-level aliases (on `ProblemSchemaEntry`), these resolve to a
    /// specific reduction-graph node, not just to a canonical problem name. The CLI
    /// resolver tries variant-level aliases first and falls back to problem-level.
    pub aliases: &'static [&'static str],
    /// Custom construction inputs. `None` means the catalog schema fields are
    /// also the construction inputs through the direct path.
    pub create_inputs: Option<&'static [CreateInputInfo]>,
    /// Construct a validated concrete problem from normalized construction data.
    pub construct_fn: ConstructProblemFn,
    /// Model-owned random generator for this exact variant.
    pub random: Option<RandomRegistration>,
    /// Factory: deserialize JSON into a boxed dynamic problem.
    pub factory: fn(serde_json::Value) -> Result<Box<dyn DynProblem>, serde_json::Error>,
    /// Serialize: downcast `&dyn Any` and serialize to JSON.
    pub serialize_fn: fn(&dyn Any) -> Option<serde_json::Value>,
    /// Solve value: downcast `&dyn Any` and brute-force solve to an aggregate string.
    pub solve_value_fn: SolveValueFn,
    /// Solve witness: downcast `&dyn Any` and brute-force recover a witness when available.
    pub solve_witness_fn: SolveWitnessFn,
}

impl VariantEntry {
    /// Get the variant by calling the function.
    pub fn variant(&self) -> Vec<(&'static str, &'static str)> {
        (self.variant_fn)()
    }

    /// Get the variant as a `BTreeMap<String, String>`.
    pub fn variant_map(&self) -> BTreeMap<String, String> {
        self.variant()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}

/// Return every registered concrete problem variant.
pub fn variant_entries() -> Vec<&'static VariantEntry> {
    inventory::iter::<VariantEntry>().collect()
}

/// Find a variant entry by exact problem name and exact variant map.
///
/// No alias resolution or default fallback. Both `name` and `variant` must match exactly.
pub fn find_variant_entry(
    name: &str,
    variant: &BTreeMap<String, String>,
) -> Option<&'static VariantEntry> {
    inventory::iter::<VariantEntry>()
        .find(|entry| entry.name == name && entry.variant_map() == *variant)
}

/// Find a variant entry by a variant-level alias (case-insensitive).
///
/// A variant-level alias points at a specific reduction-graph node (e.g., `"3SAT"` →
/// `KSatisfiability` with variant `{k: "K3"}`), unlike problem-level aliases which
/// resolve only to a canonical problem name.
///
/// Returns the matched entry along with its variant map. The first match in registration
/// order wins — duplicate variant-level aliases across problems are a declaration bug.
pub fn find_variant_by_alias(
    input: &str,
) -> Option<(&'static VariantEntry, BTreeMap<String, String>)> {
    let lower = input.to_lowercase();
    let entry = inventory::iter::<VariantEntry>()
        .find(|entry| entry.aliases.iter().any(|a| a.to_lowercase() == lower))?;
    Some((entry, entry.variant_map()))
}

/// Validate all variant-level aliases registered in inventory.
///
/// This is intended for explicit test-time or startup invocation. It rejects
/// duplicate variant-level aliases, aliases that collide with canonical
/// problem names or problem-level aliases, and empty aliases for manually
/// constructed [`VariantEntry`] values that bypass `declare_variants!`.
pub fn validate_variant_aliases() -> Result<(), Vec<String>> {
    let mut problem_names: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for problem in super::problem_type::problem_types() {
        problem_names
            .entry(problem.canonical_name.to_lowercase())
            .or_default()
            .push(format!(
                "canonical problem name `{}`",
                problem.canonical_name
            ));

        for alias in problem.aliases {
            problem_names
                .entry(alias.to_lowercase())
                .or_default()
                .push(format!(
                    "problem-level alias `{alias}` for `{}`",
                    problem.canonical_name
                ));
        }
    }

    let entries: Vec<_> = inventory::iter::<VariantEntry>()
        .map(|e| (variant_label(e), e.aliases))
        .collect();

    validate_aliases_inner(&problem_names, &entries)
}

/// Core validation logic, separated for testability with mock data.
///
/// - `problem_names`: lowercase key → list of human-readable sources (canonical names + problem-level aliases).
/// - `entries`: `(variant_label, aliases_slice)` per variant entry.
pub fn validate_aliases_inner(
    problem_names: &BTreeMap<String, Vec<String>>,
    entries: &[(String, &[&str])],
) -> Result<(), Vec<String>> {
    let mut conflicts = Vec::new();
    let mut variant_aliases: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();

    for (target, aliases) in entries {
        for alias in *aliases {
            if alias.trim().is_empty() {
                conflicts.push(format!(
                    "variant-level alias on {target} is empty or whitespace-only"
                ));
                continue;
            }

            let lower = alias.to_lowercase();
            if let Some(collisions) = problem_names.get(&lower) {
                for collision in collisions {
                    conflicts.push(format!(
                        "variant-level alias `{alias}` on {target} conflicts with {collision}"
                    ));
                }
            }

            variant_aliases
                .entry(lower)
                .or_default()
                .push((alias.to_string(), target.clone()));
        }
    }

    for (lower, registrations) in variant_aliases {
        if registrations.len() > 1 {
            let details = registrations
                .iter()
                .map(|(alias, target)| format!("`{alias}` on {target}"))
                .collect::<Vec<_>>()
                .join("; ");
            conflicts.push(format!(
                "duplicate variant-level alias `{lower}` (case-insensitive): {details}"
            ));
        }
    }

    if conflicts.is_empty() {
        Ok(())
    } else {
        conflicts.sort();
        Err(conflicts)
    }
}

pub fn variant_label(entry: &VariantEntry) -> String {
    let variant = entry.variant();
    if variant.is_empty() {
        return entry.name.to_string();
    }

    let parts = variant
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{} {{{parts}}}", entry.name)
}

impl std::fmt::Debug for VariantEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VariantEntry")
            .field("name", &self.name)
            .field("variant", &self.variant())
            .field("complexity", &self.complexity)
            .finish()
    }
}

inventory::collect!(VariantEntry);

#[cfg(test)]
#[path = "../unit_tests/registry/variant.rs"]
mod tests;
