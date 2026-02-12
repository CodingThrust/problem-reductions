use super::*;

#[test]
fn test_complexity_class() {
    assert_eq!(ComplexityClass::NpComplete.name(), "NP-complete");
    assert!(ComplexityClass::NpComplete.is_hard());
    assert!(ComplexityClass::NpHard.is_hard());
    assert!(!ComplexityClass::P.is_hard());
}

#[test]
fn test_problem_info_builder() {
    let info = ProblemInfo::new("Independent Set", "Find a maximum weight independent set")
        .with_aliases(&["MIS", "MWIS"])
        .with_complexity(ComplexityClass::NpComplete)
        .with_reduction_from("3-SAT")
        .with_reference("https://en.wikipedia.org/wiki/Independent_set_(graph_theory)");

    assert_eq!(info.name, "Independent Set");
    assert_eq!(info.aliases, &["MIS", "MWIS"]);
    assert!(info.is_np_complete());
    assert_eq!(info.canonical_reduction_from, Some("3-SAT"));
    assert_eq!(info.all_names(), vec!["Independent Set", "MIS", "MWIS"]);
}

#[test]
fn test_problem_info_display() {
    let info = ProblemInfo::new("Vertex Cover", "Find a minimum vertex cover");
    assert_eq!(format!("{}", info), "Vertex Cover (NP-complete)");
}

#[test]
fn test_problem_info_versions() {
    let decision_only =
        ProblemInfo::new("Decision Problem", "A yes/no problem").with_optimization(false);
    assert!(decision_only.decision_version);
    assert!(!decision_only.optimization_version);

    let opt_only =
        ProblemInfo::new("Optimization Problem", "An optimization problem").with_decision(false);
    assert!(!opt_only.decision_version);
    assert!(opt_only.optimization_version);
}

#[test]
fn test_problem_info_with_fields() {
    const FIELDS: &[FieldInfo] = &[
        FieldInfo {
            name: "graph",
            type_name: "G",
            description: "The graph",
        },
        FieldInfo {
            name: "weights",
            type_name: "Vec<W>",
            description: "Vertex weights",
        },
    ];
    let info = ProblemInfo::new("Test", "Test problem").with_fields(FIELDS);
    assert_eq!(info.fields.len(), 2);
    assert_eq!(info.fields[0].name, "graph");
    assert_eq!(info.fields[1].type_name, "Vec<W>");
}
