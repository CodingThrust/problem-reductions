use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;

/// A parsed problem specification: name + optional variant values.
#[derive(Debug, Clone)]
pub struct ProblemSpec {
    /// Resolved canonical problem name.
    pub name: String,
    /// Positional variant values (e.g., ["UnitDiskGraph", "i32"]).
    pub variant_values: Vec<String>,
}

/// Resolve a short alias to the canonical problem name.
///
/// Searches both variant-level aliases (e.g., `"3SAT"` → `KSatisfiability`) and
/// problem-level aliases (e.g., `"MIS"` → `MaximumIndependentSet`). When a
/// variant-level alias is matched, only the canonical name is returned here.
/// The older pass-through behavior where `3SAT` resolved to `"3SAT"` has been
/// intentionally replaced by `3SAT` resolving to `"KSatisfiability"` so aliases
/// behave consistently. Callers that need variant semantics such as
/// `3SAT` → `KSatisfiability { k = K3 }` should use [`parse_problem_spec`] or
/// [`resolve_problem_ref`].
pub fn resolve_alias(input: &str) -> String {
    if input.eq_ignore_ascii_case("UndirectedFlowLowerBounds") {
        return "UndirectedFlowLowerBounds".to_string();
    }
    if input.eq_ignore_ascii_case("GroupingBySwapping") {
        return "GroupingBySwapping".to_string();
    }
    if input.eq_ignore_ascii_case("MinimumWeightAndOrGraph") {
        return "MinimumWeightAndOrGraph".to_string();
    }
    if input.eq_ignore_ascii_case("MinimumFaultDetectionTestSet") {
        return "MinimumFaultDetectionTestSet".to_string();
    }
    if input.eq_ignore_ascii_case("MinimumCodeGenerationUnlimitedRegisters") {
        return "MinimumCodeGenerationUnlimitedRegisters".to_string();
    }
    if input.eq_ignore_ascii_case("MinimumCodeGenerationParallelAssignments") {
        return "MinimumCodeGenerationParallelAssignments".to_string();
    }
    if input.eq_ignore_ascii_case("ThreeMatroidIntersection") {
        return "ThreeMatroidIntersection".to_string();
    }
    if input.eq_ignore_ascii_case("GraphPartitioning") {
        return "GraphPartitioning".to_string();
    }
    if let Some((entry, _)) = problemreductions::registry::find_variant_by_alias(input) {
        return entry.name.to_string();
    }
    if let Some(pt) = problemreductions::registry::find_problem_type_by_alias(input) {
        return pt.canonical_name.to_string();
    }
    input.to_string()
}

/// Return the short aliases for a canonical problem name, if any.
pub fn aliases_for(canonical: &str) -> Vec<&'static str> {
    problemreductions::registry::find_problem_type(canonical)
        .map(|pt| pt.aliases.to_vec())
        .unwrap_or_default()
}

/// Resolve a problem spec against the catalog schema only (no graph required).
///
/// Returns a typed `ProblemRef` validated against the catalog's declared
/// dimensions and allowed values. Does NOT check reduction graph reachability.
#[cfg_attr(not(test), allow(dead_code))]
pub fn resolve_catalog_problem_ref(
    input: &str,
) -> anyhow::Result<problemreductions::registry::ProblemRef> {
    problemreductions::registry::parse_catalog_problem_ref(input)
        .map_err(|e| anyhow::anyhow!("{e}"))
}

/// Parse a problem spec string like "MIS/UnitDiskGraph/i32" into name + variant values.
///
/// Resolution order:
/// 1. **Variant-level alias** (`"3SAT"` → `KSatisfiability` + variant tokens `["K3"]`):
///    injects the variant tokens *before* any user-supplied tokens from the slash spec.
/// 2. **Problem-level alias** (`"MIS"` → `MaximumIndependentSet`): canonical-name only;
///    downstream default-variant resolution fills in the variant dimensions.
pub fn parse_problem_spec(input: &str) -> anyhow::Result<ProblemSpec> {
    let parts: Vec<&str> = input.split('/').collect();
    let raw_name = parts[0];
    let user_tokens: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

    if let Some((entry, variant_map)) = problemreductions::registry::find_variant_by_alias(raw_name)
    {
        // Prepend the alias's own variant values; the slash-spec resolver handles
        // additional user tokens (and errors on dimension collisions).
        let mut variant_values: Vec<String> = variant_map.into_values().collect();
        variant_values.extend(user_tokens);
        return Ok(ProblemSpec {
            name: entry.name.to_string(),
            variant_values,
        });
    }

    let name = resolve_alias(raw_name);

    Ok(ProblemSpec {
        name,
        variant_values: user_tokens,
    })
}

fn format_variant(variant: &BTreeMap<String, String>) -> String {
    let parts = variant
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{{{parts}}}")
}

fn dimension_values(
    known_variants: &[BTreeMap<String, String>],
) -> BTreeMap<String, BTreeSet<String>> {
    let mut by_dimension = BTreeMap::new();
    for variant in known_variants {
        for (dimension, value) in variant {
            by_dimension
                .entry(dimension.clone())
                .or_insert_with(BTreeSet::new)
                .insert(value.clone());
        }
    }
    by_dimension
}

fn resolve_variant_updates(
    spec: &ProblemSpec,
    default_variant: &BTreeMap<String, String>,
    known_variants: &[BTreeMap<String, String>],
) -> anyhow::Result<BTreeMap<String, String>> {
    if spec.variant_values.is_empty() {
        return Ok(default_variant.clone());
    }

    let token_index = dimension_values(known_variants);
    let mut resolved = default_variant.clone();
    let mut updated_dimensions = BTreeSet::new();

    for token in &spec.variant_values {
        let matching_dimensions = token_index
            .iter()
            .filter(|(_, values)| values.contains(token))
            .map(|(dimension, _)| dimension.clone())
            .collect::<Vec<_>>();

        match matching_dimensions.as_slice() {
            [] => anyhow::bail!("Unknown variant token \"{}\" for {}", token, spec.name),
            [dimension] => {
                if !updated_dimensions.insert(dimension.clone()) {
                    anyhow::bail!(
                        "Variant dimension \"{}\" was specified more than once",
                        dimension
                    );
                }
                resolved.insert(dimension.clone(), token.clone());
            }
            _ => {
                let dimensions = matching_dimensions.join(" and ");
                anyhow::bail!(
                    "Token \"{}\" is ambiguous for {}; matches dimensions {}",
                    token,
                    spec.name,
                    dimensions
                );
            }
        }
    }

    if known_variants.iter().any(|variant| variant == &resolved) {
        Ok(resolved)
    } else {
        anyhow::bail!(
            "Resolved variant {} is not declared for {}",
            format_variant(&resolved),
            spec.name
        )
    }
}

/// Parse the problem name from a spec string, resolving aliases.
///
/// Accepts both bare names ("MIS") and slash specs ("MIS/UnitDiskGraph").
/// Returns just the canonical name (alias-resolved).
#[cfg(test)]
pub fn parse_problem_type(input: &str) -> anyhow::Result<String> {
    let parts: Vec<&str> = input.split('/').collect();
    Ok(resolve_alias(parts[0]))
}

/// Resolve a problem spec to a specific graph node using declared defaults.
///
/// For bare names (no slash), returns the declared default variant.
/// For slash specs, resolves variant values against known variants.
pub fn resolve_problem_ref(
    input: &str,
    graph: &problemreductions::rules::ReductionGraph,
) -> anyhow::Result<ProblemRef> {
    let spec = parse_problem_spec(input)?;
    let known_variants = graph.variants_for(&spec.name);

    if known_variants.is_empty() {
        anyhow::bail!("{}", unknown_problem_error(&spec.name));
    }

    let default_variant = graph
        .default_variant_for(&spec.name)
        .ok_or_else(|| anyhow::anyhow!("No default variant declared for {}", spec.name))?;

    let resolved = resolve_variant_updates(&spec, &default_variant, &known_variants)?;
    Ok(ProblemRef {
        name: spec.name,
        variant: resolved,
    })
}

use problemreductions::export::ProblemRef;

/// A value parser that accepts any string but provides problem names as
/// completion candidates for shell completion scripts.
#[derive(Clone)]
pub struct ProblemNameParser;

impl clap::builder::TypedValueParser for ProblemNameParser {
    type Value = String;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &OsStr,
    ) -> Result<String, clap::Error> {
        Ok(value.to_string_lossy().to_string())
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue>>> {
        let mut names = Vec::new();
        for pt in problemreductions::registry::problem_types() {
            names.push(pt.canonical_name);
            for alias in pt.aliases {
                names.push(alias);
            }
        }

        names.sort();
        names.dedup();
        Some(Box::new(
            names.into_iter().map(clap::builder::PossibleValue::new),
        ))
    }
}

/// Find the closest matching problem names using edit distance.
pub fn suggest_problem_name(input: &str) -> Vec<String> {
    let input_lower = input.to_lowercase();
    let mut suggestions: Vec<(String, usize)> = Vec::new();

    for problem_type in problemreductions::registry::problem_types() {
        let dist = edit_distance(&input_lower, &problem_type.canonical_name.to_lowercase());
        if dist <= 3 {
            suggestions.push((problem_type.canonical_name.to_string(), dist));
        }
        for alias in problem_type.aliases {
            let dist = edit_distance(&input_lower, &alias.to_lowercase());
            if dist <= 2 {
                suggestions.push((problem_type.canonical_name.to_string(), dist));
            }
        }
    }

    suggestions.sort_by_key(|(_, d)| *d);
    suggestions.dedup_by_key(|(n, _)| n.clone());
    suggestions.into_iter().map(|(n, _)| n).take(3).collect()
}

/// Simple Levenshtein edit distance.
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    for (i, row) in dp.iter_mut().enumerate().take(n + 1) {
        row[0] = i;
    }
    for (j, value) in dp[0].iter_mut().enumerate().take(m + 1) {
        *value = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[n][m]
}

/// Format an error message for an unknown problem name with suggestions.
pub fn unknown_problem_error(input: &str) -> String {
    let suggestions = suggest_problem_name(input);
    let mut msg = format!("Unknown problem: {input}");
    if !suggestions.is_empty() {
        msg.push_str(&format!("\n\nDid you mean: {}?", suggestions.join(", ")));
    }
    msg.push_str("\n\nRun `pred list` to see all available problems.");
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_resolution() {
        assert_eq!(resolve_alias("MIS"), "MaximumIndependentSet");
        assert_eq!(resolve_alias("mis"), "MaximumIndependentSet");
        assert_eq!(resolve_alias("MVC"), "MinimumVertexCover");
        assert_eq!(resolve_alias("SAT"), "Satisfiability");
        assert_eq!(resolve_alias("X3C"), "ExactCoverBy3Sets");
        assert_eq!(resolve_alias("3Partition"), "ThreePartition");
        assert_eq!(resolve_alias("3-partition"), "ThreePartition");
        // Variant-level aliases: resolve_alias only returns the canonical name;
        // parse_problem_spec recovers the variant tokens (see tests below).
        assert_eq!(resolve_alias("3SAT"), "KSatisfiability");
        assert_eq!(resolve_alias("3sat"), "KSatisfiability");
        assert_eq!(resolve_alias("2SAT"), "KSatisfiability");
        assert_eq!(resolve_alias("QUBO"), "QUBO");
        assert_eq!(resolve_alias("MaxCut"), "MaxCut");
        assert_eq!(
            resolve_alias("biconnectivityaugmentation"),
            "BiconnectivityAugmentation"
        );
        assert_eq!(resolve_alias("DMVC"), "DecisionMinimumVertexCover");
        // Pass-through for full names
        assert_eq!(
            resolve_alias("MaximumIndependentSet"),
            "MaximumIndependentSet"
        );
    }

    #[test]
    fn test_parse_problem_spec_bare() {
        let spec = parse_problem_spec("MIS").unwrap();
        assert_eq!(spec.name, "MaximumIndependentSet");
        assert!(spec.variant_values.is_empty());
    }

    #[test]
    fn test_parse_problem_spec_with_variants() {
        let spec = parse_problem_spec("MIS/UnitDiskGraph").unwrap();
        assert_eq!(spec.name, "MaximumIndependentSet");
        assert_eq!(spec.variant_values, vec!["UnitDiskGraph"]);
    }

    #[test]
    fn test_parse_problem_spec_two_variants() {
        let spec = parse_problem_spec("MIS/SimpleGraph/f64").unwrap();
        assert_eq!(spec.name, "MaximumIndependentSet");
        assert_eq!(spec.variant_values, vec!["SimpleGraph", "f64"]);
    }

    #[test]
    fn test_resolve_alias_pass_through_undirected_two_commodity_integral_flow() {
        assert_eq!(
            resolve_alias("UndirectedTwoCommodityIntegralFlow"),
            "UndirectedTwoCommodityIntegralFlow"
        );
    }

    #[test]
    fn test_resolve_alias_pass_through_undirected_flow_lower_bounds() {
        assert_eq!(
            resolve_alias("UndirectedFlowLowerBounds"),
            "UndirectedFlowLowerBounds"
        );
        assert_eq!(
            resolve_alias("undirectedflowlowerbounds"),
            "UndirectedFlowLowerBounds"
        );
    }

    #[test]
    fn test_parse_problem_spec_ksat_alias() {
        let spec = parse_problem_spec("KSAT").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert!(spec.variant_values.is_empty());
    }

    #[test]
    fn test_parse_problem_spec_ksat_k3() {
        let spec = parse_problem_spec("KSAT/K3").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K3"]);
    }

    #[test]
    fn test_parse_problem_spec_variant_alias_3sat() {
        // Variant-level alias: "3SAT" injects the K3 variant token.
        let spec = parse_problem_spec("3SAT").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K3"]);

        let spec = parse_problem_spec("3sat").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K3"]);
    }

    #[test]
    fn test_parse_problem_spec_variant_alias_2sat() {
        let spec = parse_problem_spec("2SAT").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K2"]);
    }

    #[test]
    fn test_resolve_problem_ref_variant_alias_3sat() {
        let graph = problemreductions::rules::ReductionGraph::new();
        let expected = ProblemRef {
            name: "KSatisfiability".to_string(),
            variant: BTreeMap::from([("k".to_string(), "K3".to_string())]),
        };

        assert_eq!(resolve_problem_ref("3SAT", &graph).unwrap(), expected);
    }

    #[test]
    fn test_resolve_problem_ref_variant_alias_2sat() {
        let graph = problemreductions::rules::ReductionGraph::new();
        let expected = ProblemRef {
            name: "KSatisfiability".to_string(),
            variant: BTreeMap::from([("k".to_string(), "K2".to_string())]),
        };

        assert_eq!(resolve_problem_ref("2SAT", &graph).unwrap(), expected);
    }

    #[test]
    fn test_resolve_problem_ref_3sat_k2_rejects_duplicate_dimension() {
        let spec = parse_problem_spec("3SAT/K2").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K3", "K2"]);

        let graph = problemreductions::rules::ReductionGraph::new();
        let err = resolve_problem_ref("3SAT/K2", &graph).unwrap_err();
        assert!(
            err.to_string().contains("specified more than once"),
            "expected duplicate-dimension error, got: {err}"
        );
    }

    #[test]
    fn test_resolve_problem_ref_3sat_simple_graph_rejects_unknown_token() {
        let spec = parse_problem_spec("3SAT/SimpleGraph").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K3", "SimpleGraph"]);

        let graph = problemreductions::rules::ReductionGraph::new();
        let err = resolve_problem_ref("3SAT/SimpleGraph", &graph).unwrap_err();
        assert!(
            err.to_string()
                .to_lowercase()
                .contains("unknown variant token"),
            "expected unknown-token error, got: {err}"
        );
    }

    #[test]
    fn test_parse_problem_spec_max2sat_problem_level() {
        // MAX2SAT is a problem-level alias on Maximum2Satisfiability (standalone problem,
        // no K variants) — no variant tokens injected.
        let spec = parse_problem_spec("MAX2SAT").unwrap();
        assert_eq!(spec.name, "Maximum2Satisfiability");
        assert!(spec.variant_values.is_empty());
    }

    #[test]
    fn test_suggest_problem_name_close() {
        // "MISs" is 1 edit from "MIS" alias -> should suggest MaximumIndependentSet
        let suggestions = suggest_problem_name("MISs");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_suggest_problem_name_far() {
        // Totally unrelated name should not match anything
        let suggestions = suggest_problem_name("xyzxyzxyz");
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_unknown_problem_error_with_suggestions() {
        let msg = unknown_problem_error("MISs");
        assert!(msg.contains("Unknown problem: MISs"));
        assert!(msg.contains("Did you mean"));
        assert!(msg.contains("pred list"));
    }

    #[test]
    fn test_unknown_problem_error_no_suggestions() {
        let msg = unknown_problem_error("xyzxyzxyz");
        assert!(msg.contains("Unknown problem: xyzxyzxyz"));
        assert!(!msg.contains("Did you mean"));
        assert!(msg.contains("pred list"));
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("abc", "ab"), 1);
        assert_eq!(edit_distance("abc", "axc"), 1);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }

    // ---- parse_problem_type ----

    #[test]
    fn parse_problem_type_bare_name() {
        // Bare name resolves alias
        assert_eq!(parse_problem_type("MIS").unwrap(), "MaximumIndependentSet");
        assert_eq!(parse_problem_type("QUBO").unwrap(), "QUBO");
    }

    #[test]
    fn parse_problem_type_with_slash() {
        // Slash specs extract the problem name portion
        assert_eq!(
            parse_problem_type("MIS/UnitDiskGraph").unwrap(),
            "MaximumIndependentSet"
        );
    }

    #[test]
    fn parse_problem_type_ksat_alias() {
        assert_eq!(parse_problem_type("KSAT").unwrap(), "KSatisfiability");
    }

    // ---- resolve_problem_ref ----

    #[test]
    fn resolve_problem_ref_bare_mis() {
        // Bare MIS should resolve to the declared default variant
        let graph = problemreductions::rules::ReductionGraph::new();
        let r = resolve_problem_ref("MIS", &graph).unwrap();
        assert_eq!(r.name, "MaximumIndependentSet");
        assert_eq!(
            r.variant.get("graph").map(|s| s.as_str()),
            Some("SimpleGraph")
        );
        assert_eq!(r.variant.get("weight").map(|s| s.as_str()), Some("One"));
    }

    #[test]
    fn resolve_problem_ref_with_slash_updates() {
        // Slash spec resolves to a specific variant
        let graph = problemreductions::rules::ReductionGraph::new();
        let r = resolve_problem_ref("MIS/UnitDiskGraph", &graph).unwrap();
        assert_eq!(r.name, "MaximumIndependentSet");
        assert_eq!(
            r.variant.get("graph").map(|s| s.as_str()),
            Some("UnitDiskGraph")
        );
    }

    #[test]
    fn resolve_problem_ref_rejects_duplicate_dimension_updates() {
        let graph = problemreductions::rules::ReductionGraph::new();
        let err = resolve_problem_ref("MIS/One/i32", &graph).unwrap_err();
        assert!(
            err.to_string().contains("specified more than once"),
            "expected duplicate-dimension error, got: {err}"
        );
    }

    #[test]
    fn resolve_problem_ref_unknown_problem() {
        let graph = problemreductions::rules::ReductionGraph::new();
        let err = resolve_problem_ref("NonExistent", &graph).unwrap_err();
        assert!(err.to_string().contains("Unknown problem"));
    }

    // ---- catalog-backed resolution ----

    #[test]
    fn resolve_problem_ref_bare_mis_uses_catalog_default() {
        // Bare MIS resolves through catalog to the declared default variant
        let graph = problemreductions::rules::ReductionGraph::new();
        let r = resolve_problem_ref("MIS", &graph).unwrap();
        assert_eq!(r.name, "MaximumIndependentSet");
        // Catalog declares SimpleGraph + One as defaults
        assert_eq!(
            r.variant.get("graph").map(|s| s.as_str()),
            Some("SimpleGraph")
        );
        assert_eq!(r.variant.get("weight").map(|s| s.as_str()), Some("One"));
    }

    #[test]
    fn parse_problem_type_extracts_name_from_variant_spec() {
        // parse_problem_type extracts just the problem name from a variant spec
        assert_eq!(
            parse_problem_type("MIS/UnitDiskGraph/i32").unwrap(),
            "MaximumIndependentSet"
        );
    }

    #[test]
    fn resolve_catalog_problem_ref_validates_against_schema() {
        // Schema-valid values should resolve
        let r = resolve_catalog_problem_ref("MIS/i32").unwrap();
        assert_eq!(r.name(), "MaximumIndependentSet");
        assert_eq!(r.variant().get("weight").map(|s| s.as_str()), Some("i32"));
    }

    #[test]
    fn resolve_catalog_problem_ref_rejects_schema_invalid_variant() {
        // HyperGraph is not in MIS's declared dimensions
        let err = resolve_catalog_problem_ref("MIS/HyperGraph").unwrap_err();
        assert!(
            err.to_string().contains("Known variants"),
            "error should mention known variants: {}",
            err
        );
    }

    #[test]
    fn resolve_catalog_problem_ref_fills_defaults() {
        // Bare MIS should fill in all defaults from catalog
        let r = resolve_catalog_problem_ref("MIS").unwrap();
        assert_eq!(
            r.variant().get("graph").map(|s| s.as_str()),
            Some("SimpleGraph")
        );
        assert_eq!(r.variant().get("weight").map(|s| s.as_str()), Some("One"));
    }
}
