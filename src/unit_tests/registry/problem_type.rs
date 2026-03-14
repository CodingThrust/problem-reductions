use crate::registry::{
    find_problem_type, find_problem_type_by_alias, parse_catalog_problem_ref, problem_types,
    ProblemRef, ProblemSchemaEntry,
};
use std::collections::HashMap;

#[test]
fn typed_problem_ref_fills_declared_defaults() {
    let problem = find_problem_type("MaximumIndependentSet").unwrap();
    let problem_ref = ProblemRef::from_values(&problem, ["i32"]).unwrap();
    assert_eq!(
        problem_ref.variant().get("graph").map(|s| s.as_str()),
        Some("SimpleGraph")
    );
    assert_eq!(
        problem_ref.variant().get("weight").map(|s| s.as_str()),
        Some("i32")
    );
}

#[test]
fn catalog_rejects_unknown_dimension_values() {
    let problem = find_problem_type("MaximumIndependentSet").unwrap();
    let err = ProblemRef::from_values(&problem, ["HyperGraph"]).unwrap_err();
    assert!(
        err.contains("Known variants"),
        "error should mention known variants: {err}"
    );
}

#[test]
fn catalog_alias_lookup_is_case_insensitive() {
    let problem = find_problem_type_by_alias("mis").unwrap();
    assert_eq!(problem.canonical_name, "MaximumIndependentSet");
}

#[test]
fn find_problem_type_returns_none_for_unknown() {
    assert!(find_problem_type("NonExistentProblem").is_none());
}

#[test]
fn find_problem_type_by_alias_matches_canonical_name() {
    let problem = find_problem_type_by_alias("MaximumIndependentSet").unwrap();
    assert_eq!(problem.canonical_name, "MaximumIndependentSet");
}

#[test]
fn problem_types_returns_all_registered() {
    let types = problem_types();
    assert!(
        types.len() > 10,
        "expected many problem types, got {}",
        types.len()
    );
    // Should include MIS
    assert!(types
        .iter()
        .any(|t| t.canonical_name == "MaximumIndependentSet"));
}

#[test]
fn problem_ref_from_values_no_values_uses_all_defaults() {
    let problem = find_problem_type("MaximumIndependentSet").unwrap();
    let problem_ref = ProblemRef::from_values(&problem, Vec::<&str>::new()).unwrap();
    assert_eq!(
        problem_ref.variant().get("graph").map(|s| s.as_str()),
        Some("SimpleGraph")
    );
    assert_eq!(
        problem_ref.variant().get("weight").map(|s| s.as_str()),
        Some("One")
    );
}

#[test]
fn problem_ref_from_values_graph_override() {
    let problem = find_problem_type("MaximumIndependentSet").unwrap();
    let problem_ref = ProblemRef::from_values(&problem, ["UnitDiskGraph", "i32"]).unwrap();
    assert_eq!(
        problem_ref.variant().get("graph").map(|s| s.as_str()),
        Some("UnitDiskGraph")
    );
    assert_eq!(
        problem_ref.variant().get("weight").map(|s| s.as_str()),
        Some("i32")
    );
}

#[test]
fn parse_catalog_problem_ref_bare_mis() {
    let r = parse_catalog_problem_ref("MIS").unwrap();
    assert_eq!(r.name(), "MaximumIndependentSet");
    assert_eq!(
        r.variant().get("graph").map(|s| s.as_str()),
        Some("SimpleGraph")
    );
    assert_eq!(r.variant().get("weight").map(|s| s.as_str()), Some("One"));
}

#[test]
fn parse_catalog_problem_ref_with_value() {
    let r = parse_catalog_problem_ref("MIS/UnitDiskGraph").unwrap();
    assert_eq!(r.name(), "MaximumIndependentSet");
    assert_eq!(
        r.variant().get("graph").map(|s| s.as_str()),
        Some("UnitDiskGraph")
    );
}

#[test]
fn parse_catalog_problem_ref_rejects_unknown() {
    let err = parse_catalog_problem_ref("NonExistent").unwrap_err();
    assert!(err.contains("Unknown problem type"));
}

#[test]
fn problem_ref_to_export_ref() {
    let problem = find_problem_type("MaximumIndependentSet").unwrap();
    let problem_ref = ProblemRef::from_values(&problem, ["i32"]).unwrap();
    let export_ref = problem_ref.to_export_ref();
    assert_eq!(export_ref.name, "MaximumIndependentSet");
    assert_eq!(
        export_ref.variant.get("weight").map(|s| s.as_str()),
        Some("i32")
    );
}

// ---- Catalog invariant tests ----

#[test]
fn every_public_problem_schema_has_display_name() {
    for entry in inventory::iter::<ProblemSchemaEntry> {
        assert!(
            !entry.display_name.is_empty(),
            "Problem {} has empty display_name",
            entry.name
        );
    }
}

#[test]
fn every_public_problem_schema_has_dimension_defaults() {
    for entry in inventory::iter::<ProblemSchemaEntry> {
        for dim in entry.dimensions {
            assert!(
                dim.allowed_values.contains(&dim.default_value),
                "Problem {} dimension '{}' default '{}' not in allowed values {:?}",
                entry.name,
                dim.key,
                dim.default_value,
                dim.allowed_values,
            );
        }
    }
}

#[test]
fn every_alias_is_globally_unique() {
    let mut seen: HashMap<String, &str> = HashMap::new();
    for entry in inventory::iter::<ProblemSchemaEntry> {
        for alias in entry.aliases {
            let lower = alias.to_lowercase();
            if let Some(prev) = seen.get(&lower) {
                panic!(
                    "Alias '{}' is used by both {} and {}",
                    alias, prev, entry.name,
                );
            }
            seen.insert(lower, entry.name);
        }
    }
}

#[test]
fn catalog_dimensions_cover_all_declared_variants() {
    use crate::registry::variant::VariantEntry;

    for entry in inventory::iter::<ProblemSchemaEntry> {
        if entry.dimensions.is_empty() {
            continue;
        }

        // Collect all variant entries for this problem
        let variants: Vec<_> = inventory::iter::<VariantEntry>
            .into_iter()
            .filter(|v| v.name == entry.name)
            .collect();

        for ve in &variants {
            let variant_pairs = ve.variant();
            for (key, value) in &variant_pairs {
                if let Some(dim) = entry.dimensions.iter().find(|d| d.key == *key) {
                    assert!(
                        dim.allowed_values.contains(value),
                        "Problem {} declared variant value '{}' for dimension '{}' \
                         is not in catalog allowed_values {:?}",
                        entry.name,
                        value,
                        key,
                        dim.allowed_values,
                    );
                }
            }
        }
    }
}

#[test]
fn graph_defaults_are_catalog_defaults_for_registered_variants() {
    let graph = crate::rules::ReductionGraph::new();

    for pt in problem_types() {
        if pt.dimensions.is_empty() {
            continue;
        }

        let catalog_default = pt.default_variant();
        if let Some(graph_default) = graph.default_variant_for(pt.canonical_name) {
            // Every catalog default dimension should match the graph default
            for (key, cat_val) in &catalog_default {
                if let Some(graph_val) = graph_default.get(key) {
                    assert_eq!(
                        cat_val, graph_val,
                        "Problem {} dimension '{}': catalog default '{}' != graph default '{}'",
                        pt.canonical_name, key, cat_val, graph_val,
                    );
                }
            }
        }
    }
}
