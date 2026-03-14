use crate::registry::{
    find_problem_type, find_problem_type_by_alias, parse_catalog_problem_ref, problem_types,
    ProblemRef,
};

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
    assert!(types.len() > 10, "expected many problem types, got {}", types.len());
    // Should include MIS
    assert!(types.iter().any(|t| t.canonical_name == "MaximumIndependentSet"));
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
    let problem_ref =
        ProblemRef::from_values(&problem, ["UnitDiskGraph", "i32"]).unwrap();
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
    assert_eq!(
        r.variant().get("weight").map(|s| s.as_str()),
        Some("One")
    );
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
