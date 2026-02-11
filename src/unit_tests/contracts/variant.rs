use crate::core::{from_legacy_variant, VariantDimension, VariantKey};

#[test]
fn test_variant_round_trip_json() {
    let dims = vec![
        VariantDimension::graph("SimpleGraph"),
        VariantDimension::weight("i32"),
        VariantDimension::new(VariantKey::ConstParam("k".to_string()), "3"),
    ];

    let json = serde_json::to_string(&dims).unwrap();
    let restored: Vec<VariantDimension> = serde_json::from_str(&json).unwrap();

    assert_eq!(dims, restored);
}

#[test]
fn test_variant_legacy_mapping() {
    let legacy = vec![
        ("graph", "SimpleGraph"),
        ("weight", "f64"),
        ("domain", "sat"),
    ];
    let dims = from_legacy_variant(&legacy);

    assert_eq!(dims[0].key, VariantKey::Graph);
    assert_eq!(dims[0].key.legacy_key(), "graph");
    assert_eq!(dims[0].value, "SimpleGraph");

    assert_eq!(dims[1].key, VariantKey::Weight);
    assert_eq!(dims[1].value, "f64");

    assert_eq!(dims[2].key, VariantKey::Custom("domain".to_string()));
    assert_eq!(dims[2].key.legacy_key(), "domain");
}
