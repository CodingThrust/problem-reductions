//! Typed internal problem references with catalog-validated variants.

use super::problem_type::ProblemType;
use std::collections::BTreeMap;

/// A typed internal reference to a specific problem variant.
///
/// Unlike `export::ProblemRef` (a plain DTO), this type validates its
/// variant dimensions against the catalog at construction time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblemRef {
    /// Canonical problem name.
    name: String,
    /// Validated variant dimensions.
    variant: BTreeMap<String, String>,
}

impl ProblemRef {
    /// Create a `ProblemRef` from positional values, matching them against
    /// the problem type's declared dimensions.
    ///
    /// Values are matched by checking which dimension's allowed_values contains
    /// each positional value. Unmatched dimensions are filled with defaults.
    ///
    /// # Errors
    ///
    /// Returns an error if any value doesn't match a dimension's allowed values.
    pub fn from_values<I, S>(problem_type: &ProblemType, values: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        // Start with all defaults
        let mut variant: BTreeMap<String, String> = problem_type.default_variant();
        let mut matched_dims: Vec<bool> = vec![false; problem_type.dimensions.len()];

        for value in values {
            let val = value.as_ref();
            // Find which dimension this value belongs to
            let dim_idx = problem_type
                .dimensions
                .iter()
                .enumerate()
                .find(|(i, dim)| !matched_dims[*i] && dim.allowed_values.contains(&val))
                .map(|(i, _)| i);

            match dim_idx {
                Some(idx) => {
                    matched_dims[idx] = true;
                    let dim = &problem_type.dimensions[idx];
                    variant.insert(dim.key.to_string(), val.to_string());
                }
                None => {
                    let known: Vec<&str> = problem_type
                        .dimensions
                        .iter()
                        .flat_map(|d| d.allowed_values.iter().copied())
                        .collect();
                    return Err(format!(
                        "Unknown variant value \"{val}\" for {}. Known variants: {known:?}",
                        problem_type.canonical_name,
                    ));
                }
            }
        }

        Ok(Self {
            name: problem_type.canonical_name.to_string(),
            variant,
        })
    }

    /// Create a `ProblemRef` from an explicit variant map, validating against the catalog.
    pub fn from_map(
        problem_type: &ProblemType,
        variant: BTreeMap<String, String>,
    ) -> Result<Self, String> {
        // Validate all keys and values
        for (key, value) in &variant {
            let dim = problem_type
                .dimensions
                .iter()
                .find(|d| d.key == key.as_str())
                .ok_or_else(|| {
                    format!(
                        "Unknown dimension \"{key}\" for {}",
                        problem_type.canonical_name
                    )
                })?;
            if !dim.allowed_values.contains(&value.as_str()) {
                return Err(format!(
                    "Unknown value \"{value}\" for dimension \"{key}\" of {}. Known variants: {:?}",
                    problem_type.canonical_name, dim.allowed_values
                ));
            }
        }

        // Fill in defaults for missing dimensions
        let mut full_variant = problem_type.default_variant();
        full_variant.extend(variant);

        Ok(Self {
            name: problem_type.canonical_name.to_string(),
            variant: full_variant,
        })
    }

    /// Get the canonical problem name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the validated variant map.
    pub fn variant(&self) -> &BTreeMap<String, String> {
        &self.variant
    }

    /// Convert to an `export::ProblemRef` DTO.
    pub fn to_export_ref(&self) -> crate::export::ProblemRef {
        crate::export::ProblemRef {
            name: self.name.clone(),
            variant: self.variant.clone(),
        }
    }
}

/// Parse a slash-separated problem spec string against the catalog.
///
/// Only validates against catalog schema (names, aliases, dimensions).
/// Does NOT check reduction graph reachability.
pub fn parse_catalog_problem_ref(input: &str) -> Result<ProblemRef, String> {
    let parts: Vec<&str> = input.split('/').collect();
    let raw_name = parts[0];
    let values: Vec<&str> = parts[1..].to_vec();

    // Resolve name through catalog
    let problem_type = super::problem_type::find_problem_type_by_alias(raw_name)
        .ok_or_else(|| format!("Unknown problem type: \"{raw_name}\""))?;

    let effective_values: Vec<String> = values.iter().map(|s| s.to_string()).collect();

    ProblemRef::from_values(&problem_type, &effective_values)
}

/// Check whether a catalog-validated `ProblemRef` exists in the reduction graph.
///
/// Returns the export DTO if the variant is reachable, or an error describing
/// which graph variants exist for the problem.
pub fn require_graph_variant(
    graph: &crate::rules::ReductionGraph,
    problem_ref: &ProblemRef,
) -> Result<crate::export::ProblemRef, String> {
    let known_variants = graph.variants_for(problem_ref.name());
    if known_variants.iter().any(|v| v == problem_ref.variant()) {
        return Ok(problem_ref.to_export_ref());
    }

    Err(format!(
        "Variant {:?} of {} is schema-valid but not reachable in the reduction graph. \
         Known graph variants: {:?}",
        problem_ref.variant(),
        problem_ref.name(),
        known_variants
    ))
}
