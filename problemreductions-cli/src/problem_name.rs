use std::collections::BTreeMap;

/// A parsed problem specification: name + optional variant values.
#[derive(Debug, Clone)]
pub struct ProblemSpec {
    /// Resolved canonical problem name.
    pub name: String,
    /// Positional variant values (e.g., ["UnitDiskGraph", "i32"]).
    pub variant_values: Vec<String>,
}

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
}
