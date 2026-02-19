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
pub const ALIASES: &[(&str, &str)] = &[
    ("MIS", "MaximumIndependentSet"),
    ("MVC", "MinimumVertexCover"),
    ("SAT", "Satisfiability"),
    ("3SAT", "KSatisfiability"),
    ("KSAT", "KSatisfiability"),
    ("TSP", "TravelingSalesman"),
];

/// Resolve a short alias to the canonical problem name.
pub fn resolve_alias(input: &str) -> String {
    match input.to_lowercase().as_str() {
        "mis" => "MaximumIndependentSet".to_string(),
        "mvc" | "minimumvertexcover" => "MinimumVertexCover".to_string(),
        "sat" | "satisfiability" => "Satisfiability".to_string(),
        "3sat" => "KSatisfiability".to_string(),
        "ksat" | "ksatisfiability" => "KSatisfiability".to_string(),
        "qubo" => "QUBO".to_string(),
        "maxcut" => "MaxCut".to_string(),
        "spinglass" => "SpinGlass".to_string(),
        "ilp" => "ILP".to_string(),
        "circuitsat" => "CircuitSAT".to_string(),
        "factoring" => "Factoring".to_string(),
        "maximumindependentset" => "MaximumIndependentSet".to_string(),
        "maximumclique" => "MaximumClique".to_string(),
        "maximummatching" => "MaximumMatching".to_string(),
        "minimumdominatingset" => "MinimumDominatingSet".to_string(),
        "minimumsetcovering" => "MinimumSetCovering".to_string(),
        "maximumsetpacking" => "MaximumSetPacking".to_string(),
        "kcoloring" => "KColoring".to_string(),
        "maximalis" => "MaximalIS".to_string(),
        "travelingsalesman" | "tsp" => "TravelingSalesman".to_string(),
        "paintshop" => "PaintShop".to_string(),
        "bmf" => "BMF".to_string(),
        "bicliquecover" => "BicliqueCover".to_string(),
        _ => input.to_string(), // pass-through for exact names
    }
}

/// Return the short aliases for a canonical problem name, if any.
pub fn aliases_for(canonical: &str) -> Vec<&'static str> {
    ALIASES
        .iter()
        .filter(|(_, name)| *name == canonical)
        .map(|(alias, _)| *alias)
        .collect()
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

/// Build a variant BTreeMap by matching positional values against a problem's
/// known variant keys from the reduction graph.
pub fn resolve_variant(
    spec: &ProblemSpec,
    known_variants: &[BTreeMap<String, String>],
) -> anyhow::Result<BTreeMap<String, String>> {
    if spec.variant_values.is_empty() {
        // Return the first (default) variant, or empty
        return Ok(known_variants.first().cloned().unwrap_or_default());
    }

    // Get the variant keys from the first known variant
    let keys: Vec<String> = known_variants
        .first()
        .map(|v| v.keys().cloned().collect())
        .unwrap_or_default();

    if spec.variant_values.len() > keys.len() {
        anyhow::bail!(
            "Too many variant values for {}: expected at most {} but got {}",
            spec.name,
            keys.len(),
            spec.variant_values.len()
        );
    }

    // Build the variant map: fill specified positions, use defaults for the rest
    let mut result = known_variants.first().cloned().unwrap_or_default();
    for (i, value) in spec.variant_values.iter().enumerate() {
        if let Some(key) = keys.get(i) {
            result.insert(key.clone(), value.clone());
        }
    }

    // Verify this variant exists
    if !known_variants.contains(&result) {
        anyhow::bail!(
            "Unknown variant for {}: {:?}. Known variants: {:?}",
            spec.name,
            result,
            known_variants
        );
    }

    Ok(result)
}

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
}
