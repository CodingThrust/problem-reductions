use std::collections::BTreeMap;
use std::ffi::OsStr;

/// A parsed problem specification: name + optional variant values.
#[derive(Debug, Clone)]
pub struct ProblemSpec {
    /// Resolved canonical problem name.
    pub name: String,
    /// Positional variant values (e.g., ["UnitDiskGraph", "i32"]).
    pub variant_values: Vec<String>,
}

/// Alias entries: (alias, canonical_name). Only includes short aliases,
/// not the lowercase identity mappings.
/// NOTE: This table is a legacy fallback. Models with catalog metadata
/// (aliases in ProblemSchemaEntry) are resolved through the catalog first.
/// This table will be removed once all models declare their aliases.
pub const ALIASES: &[(&str, &str)] = &[
    ("MIS", "MaximumIndependentSet"),
    ("MVC", "MinimumVertexCover"),
    ("SAT", "Satisfiability"),
    ("3SAT", "KSatisfiability"),
    ("KSAT", "KSatisfiability"),
    ("TSP", "TravelingSalesman"),
    ("CVP", "ClosestVectorProblem"),
    ("RPP", "RuralPostman"),
    ("LCS", "LongestCommonSubsequence"),
    ("MaxMatching", "MaximumMatching"),
    ("OLA", "OptimalLinearArrangement"),
    ("FVS", "MinimumFeedbackVertexSet"),
    ("SCS", "ShortestCommonSupersequence"),
    ("FAS", "MinimumFeedbackArcSet"),
    ("pmedian", "MinimumSumMulticenter"),
];

/// Resolve a short alias to the canonical problem name.
///
/// Tries the catalog first (ProblemSchemaEntry aliases), then falls back
/// to the legacy ALIASES table and lowercase match table.
pub fn resolve_alias(input: &str) -> String {
    // Try catalog first
    if let Some(pt) = problemreductions::registry::find_problem_type_by_alias(input) {
        return pt.canonical_name.to_string();
    }

    // Legacy fallback for models that haven't declared catalog aliases yet
    match input.to_lowercase().as_str() {
        "mis" => "MaximumIndependentSet".to_string(),
        "mvc" | "minimumvertexcover" => "MinimumVertexCover".to_string(),
        "sat" | "satisfiability" => "Satisfiability".to_string(),
        "3sat" => "KSatisfiability".to_string(),
        "ksat" | "ksatisfiability" => "KSatisfiability".to_string(),
        "qubo" => "QUBO".to_string(),
        "graphpartitioning" => "GraphPartitioning".to_string(),
        "isomorphicspanningtree" => "IsomorphicSpanningTree".to_string(),
        "maxcut" => "MaxCut".to_string(),
        "spinglass" => "SpinGlass".to_string(),
        "ilp" => "ILP".to_string(),
        "circuitsat" => "CircuitSAT".to_string(),
        "factoring" => "Factoring".to_string(),
        "maximumindependentset" => "MaximumIndependentSet".to_string(),
        "maximumclique" => "MaximumClique".to_string(),
        "maxmatching" | "maximummatching" => "MaximumMatching".to_string(),
        "minimumdominatingset" => "MinimumDominatingSet".to_string(),
        "minimumsetcovering" => "MinimumSetCovering".to_string(),
        "maximumsetpacking" => "MaximumSetPacking".to_string(),
        "kcoloring" => "KColoring".to_string(),
        "maximalis" => "MaximalIS".to_string(),
        "travelingsalesman" | "tsp" => "TravelingSalesman".to_string(),
        "ruralpostman" | "rpp" => "RuralPostman".to_string(),
        "paintshop" => "PaintShop".to_string(),
        "bmf" => "BMF".to_string(),
        "bicliquecover" => "BicliqueCover".to_string(),
        "binpacking" => "BinPacking".to_string(),
        "cvp" | "closestvectorproblem" => "ClosestVectorProblem".to_string(),
        "knapsack" => "Knapsack".to_string(),
        "optimallineararrangement" | "ola" => "OptimalLinearArrangement".to_string(),
        "subgraphisomorphism" => "SubgraphIsomorphism".to_string(),
        "partitionintotriangles" => "PartitionIntoTriangles".to_string(),
        "lcs" | "longestcommonsubsequence" => "LongestCommonSubsequence".to_string(),
        "fvs" | "minimumfeedbackvertexset" => "MinimumFeedbackVertexSet".to_string(),
        "flowshopscheduling" => "FlowShopScheduling".to_string(),
        "fas" | "minimumfeedbackarcset" => "MinimumFeedbackArcSet".to_string(),
        "minimumsummulticenter" | "pmedian" => "MinimumSumMulticenter".to_string(),
        "subsetsum" => "SubsetSum".to_string(),
        "scs" | "shortestcommonsupersequence" => "ShortestCommonSupersequence".to_string(),
        "hamiltonianpath" => "HamiltonianPath".to_string(),
        _ => input.to_string(), // pass-through for exact names
    }
}

/// Return the short aliases for a canonical problem name, if any.
///
/// Checks catalog aliases first, then supplements from the legacy ALIASES table.
pub fn aliases_for(canonical: &str) -> Vec<&'static str> {
    // Try catalog first
    if let Some(pt) = problemreductions::registry::find_problem_type(canonical) {
        if !pt.aliases.is_empty() {
            return pt.aliases.to_vec();
        }
    }

    // Fallback to legacy table
    ALIASES
        .iter()
        .filter(|(_, name)| *name == canonical)
        .map(|(alias, _)| *alias)
        .collect()
}

/// Resolve a problem spec against the catalog schema only (no graph required).
///
/// Returns a typed `ProblemRef` validated against the catalog's declared
/// dimensions and allowed values. Does NOT check reduction graph reachability.
pub fn resolve_catalog_problem_ref(
    input: &str,
) -> anyhow::Result<problemreductions::registry::ProblemRef> {
    problemreductions::registry::parse_catalog_problem_ref(input)
        .map_err(|e| anyhow::anyhow!("{e}"))
}

/// Parse a problem spec string like "MIS/UnitDiskGraph/i32" into name + variant values.
pub fn parse_problem_spec(input: &str) -> anyhow::Result<ProblemSpec> {
    let parts: Vec<&str> = input.split('/').collect();
    let raw_name = parts[0];
    let mut variant_values: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

    let name = resolve_alias(raw_name);

    // Special case: "3SAT" implies K3 variant
    if raw_name.to_lowercase() == "3sat" && variant_values.is_empty() {
        variant_values.push("K3".to_string());
    }

    Ok(ProblemSpec {
        name,
        variant_values,
    })
}

/// Build a variant BTreeMap by matching specified values against a problem's
/// known variants from the reduction graph. Uses value-based matching:
/// each specified value must appear as a value in the variant map.
pub fn resolve_variant(
    spec: &ProblemSpec,
    known_variants: &[BTreeMap<String, String>],
) -> anyhow::Result<BTreeMap<String, String>> {
    if spec.variant_values.is_empty() {
        // Return the first (default) variant, or empty
        return Ok(known_variants.first().cloned().unwrap_or_default());
    }

    // Value-based matching: find variant containing ALL specified values
    let matches: Vec<_> = known_variants
        .iter()
        .filter(|v| {
            spec.variant_values
                .iter()
                .all(|sv| v.values().any(|vv| vv == sv))
        })
        .collect();

    match matches.len() {
        1 => Ok(matches[0].clone()),
        0 => anyhow::bail!(
            "No variant of {} matches values {:?}. Known variants: {:?}",
            spec.name,
            spec.variant_values,
            known_variants
        ),
        _ => {
            // When ambiguous, use the same default ranking as the reduction graph:
            // variants whose remaining (unmatched) fields are closest to defaults
            // (SimpleGraph, One, KN) win. This matches variants_for() sort order.
            let default_rank = |v: &BTreeMap<String, String>| -> usize {
                v.values()
                    .filter(|val| {
                        !spec.variant_values.contains(val)
                            && !["SimpleGraph", "One", "KN"].contains(&val.as_str())
                    })
                    .count()
            };
            let min_rank = matches.iter().map(|v| default_rank(v)).min().unwrap();
            let best: Vec<_> = matches
                .iter()
                .filter(|v| default_rank(v) == min_rank)
                .collect();
            if best.len() == 1 {
                return Ok((*best[0]).clone());
            }
            anyhow::bail!(
                "Ambiguous variant for {} with values {:?}. Matches: {:?}",
                spec.name,
                spec.variant_values,
                matches
            )
        }
    }
}

/// Type-level parser for the `show` command.
///
/// Resolves aliases but rejects slash suffixes — `show` operates on the
/// entire problem type, not a specific variant node.
pub fn parse_problem_type(input: &str) -> anyhow::Result<String> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() > 1 {
        anyhow::bail!(
            "`show` operates at the type level. Use `pred show {}` without variant suffixes.\n\
             To see a specific variant's details, use `pred to {0}` or `pred from {0}`.",
            parts[0]
        );
    }
    Ok(resolve_alias(input))
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

    // Get declared default variant
    let default_variant = graph
        .default_variant_for(&spec.name)
        .ok_or_else(|| anyhow::anyhow!("{}", unknown_problem_error(&spec.name)))?;

    if spec.variant_values.is_empty() {
        // Bare name: use the declared default
        return Ok(ProblemRef {
            name: spec.name,
            variant: default_variant,
        });
    }

    // Has slash tokens: apply them as updates to the default
    let known_variants = graph.variants_for(&spec.name);
    let resolved = resolve_variant(&spec, &known_variants)?;
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
        let graph = problemreductions::rules::ReductionGraph::new();
        let mut names: Vec<&'static str> = graph.problem_types();

        // Add catalog aliases
        for pt in problemreductions::registry::problem_types() {
            for alias in pt.aliases {
                names.push(alias);
            }
        }

        // Add legacy aliases for models without catalog metadata yet
        for (alias, _) in ALIASES {
            names.push(alias);
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
    let graph = problemreductions::rules::ReductionGraph::new();
    let all_names = graph.problem_types();

    let input_lower = input.to_lowercase();
    let mut suggestions: Vec<(String, usize)> = Vec::new();

    for name in all_names {
        let dist = edit_distance(&input_lower, &name.to_lowercase());
        if dist <= 3 {
            suggestions.push((name.to_string(), dist));
        }
    }

    // Also check aliases
    for (alias, canonical) in ALIASES {
        let dist = edit_distance(&input_lower, &alias.to_lowercase());
        if dist <= 2 {
            suggestions.push((canonical.to_string(), dist));
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
    for j in 0..=m {
        dp[0][j] = j;
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
        assert_eq!(resolve_alias("3SAT"), "KSatisfiability");
        assert_eq!(resolve_alias("QUBO"), "QUBO");
        assert_eq!(resolve_alias("MaxCut"), "MaxCut");
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
    fn test_parse_problem_spec_3sat_alias() {
        let spec = parse_problem_spec("3SAT").unwrap();
        assert_eq!(spec.name, "KSatisfiability");
        assert_eq!(spec.variant_values, vec!["K3"]);
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
    fn parse_problem_type_rejects_slash() {
        // Slash suffixes are rejected for type-level operations
        let err = parse_problem_type("MIS/UnitDiskGraph").unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("type level"),
            "error should mention type level: {msg}"
        );
        assert!(
            msg.contains("pred show MIS"),
            "error should suggest bare name: {msg}"
        );
    }

    #[test]
    fn parse_problem_type_3sat_alias() {
        // 3SAT resolves to KSatisfiability without injecting K3
        assert_eq!(parse_problem_type("3SAT").unwrap(), "KSatisfiability");
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
    fn parse_problem_type_rejects_variant_suffixes_before_graph_lookup() {
        // show command rejects slash suffixes at the type level
        let err = parse_problem_type("MIS/UnitDiskGraph").unwrap_err();
        assert!(
            err.to_string().contains("type level"),
            "error should mention type level"
        );
    }

    #[test]
    fn resolve_catalog_problem_ref_validates_against_schema() {
        // Schema-valid values should resolve
        let r = resolve_catalog_problem_ref("MIS/i32").unwrap();
        assert_eq!(r.name(), "MaximumIndependentSet");
        assert_eq!(
            r.variant().get("weight").map(|s| s.as_str()),
            Some("i32")
        );
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
        assert_eq!(
            r.variant().get("weight").map(|s| s.as_str()),
            Some("One")
        );
    }
}
