use super::*;

#[test]
fn test_collect_schemas_returns_all_problems() {
    let schemas = collect_schemas();
    // We have 17 registered problems
    assert!(schemas.len() >= 17, "Expected at least 17 schemas, got {}", schemas.len());
}

#[test]
fn test_collect_schemas_sorted_by_name() {
    let schemas = collect_schemas();
    for w in schemas.windows(2) {
        assert!(w[0].name <= w[1].name, "Schemas not sorted: {} > {}", w[0].name, w[1].name);
    }
}

#[test]
fn test_collect_schemas_known_problems() {
    let schemas = collect_schemas();
    let names: Vec<&str> = schemas.iter().map(|s| s.name.as_str()).collect();
    for expected in &["MaximumIndependentSet", "MinimumVertexCover", "QUBO", "SpinGlass", "Satisfiability", "KColoring"] {
        assert!(names.contains(expected), "Missing schema for {}", expected);
    }
}

#[test]
fn test_schema_fields_populated() {
    let schemas = collect_schemas();
    let is_schema = schemas.iter().find(|s| s.name == "MaximumIndependentSet").unwrap();
    assert_eq!(is_schema.category, "graph");
    assert!(!is_schema.fields.is_empty(), "MaximumIndependentSet should have fields");
    let field_names: Vec<&str> = is_schema.fields.iter().map(|f| f.name.as_str()).collect();
    assert!(field_names.contains(&"graph"), "MaximumIndependentSet should have 'graph' field");
    assert!(field_names.contains(&"weights"), "MaximumIndependentSet should have 'weights' field");
}

#[test]
fn test_schema_json_serialization() {
    let schemas = collect_schemas();
    let json = serde_json::to_string(&schemas).expect("Schemas should serialize to JSON");
    assert!(json.contains("MaximumIndependentSet"));
    assert!(json.contains("graph"));
}

#[test]
fn test_field_info_json_fields() {
    let schemas = collect_schemas();
    let sg = schemas.iter().find(|s| s.name == "SpinGlass").unwrap();
    assert_eq!(sg.fields.len(), 3);
    let field_names: Vec<&str> = sg.fields.iter().map(|f| f.name.as_str()).collect();
    assert!(field_names.contains(&"graph"));
    assert!(field_names.contains(&"couplings"));
    assert!(field_names.contains(&"fields"));
    for f in &sg.fields {
        assert!(!f.type_name.is_empty());
        assert!(!f.description.is_empty());
    }
}
