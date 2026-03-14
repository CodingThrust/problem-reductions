use crate::example_db::{build_model_db, build_rule_db, find_model_example, find_rule_example};
use crate::export::{ProblemRef, EXAMPLE_DB_VERSION};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn test_build_model_db_contains_curated_examples() {
    let db = build_model_db().expect("model db should build");
    assert_eq!(db.version, EXAMPLE_DB_VERSION);
    assert!(!db.models.is_empty(), "model db should not be empty");
    assert!(
        db.models
            .iter()
            .any(|model| model.problem == "MaximumIndependentSet"),
        "model db should include a canonical MaximumIndependentSet example"
    );
}

#[test]
fn test_find_model_example_mis_simplegraph_i32() {
    let problem = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };

    let example = find_model_example(&problem).expect("MIS example should exist");
    assert_eq!(example.problem, "MaximumIndependentSet");
    assert_eq!(example.variant, problem.variant);
    assert!(example.instance.is_object());
    assert!(
        !example.optimal.is_empty(),
        "canonical example should include optima"
    );
}

#[test]
fn test_find_rule_example_mvc_to_mis_contains_full_problem_json() {
    let source = ProblemRef {
        name: "MinimumVertexCover".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };

    let example = find_rule_example(&source, &target).unwrap();
    assert!(example.source.instance.get("graph").is_some());
    assert!(example.target.instance.get("graph").is_some());
}

#[test]
fn test_find_rule_example_sat_to_kcoloring_contains_full_instances() {
    let source = ProblemRef {
        name: "Satisfiability".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "KColoring".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("k".to_string(), "K3".to_string()),
        ]),
    };

    let example = find_rule_example(&source, &target).unwrap();
    assert!(
        example.source.instance.get("clauses").is_some(),
        "SAT source should have clauses field"
    );
    assert!(
        example.target.instance.get("graph").is_some(),
        "KColoring target should have graph field"
    );
}

#[test]
fn test_build_rule_db_has_unique_structural_keys() {
    let db = build_rule_db().expect("rule db should build");
    let mut seen = BTreeSet::new();
    for rule in &db.rules {
        let key = (rule.source.problem_ref(), rule.target.problem_ref());
        assert!(
            seen.insert(key.clone()),
            "Duplicate rule key: {} {:?} -> {} {:?}",
            key.0.name,
            key.0.variant,
            key.1.name,
            key.1.variant
        );
    }
}

#[test]
fn test_build_model_db_has_unique_structural_keys() {
    let db = build_model_db().expect("model db should build");
    let mut seen = BTreeSet::new();
    for model in &db.models {
        let key = model.problem_ref();
        assert!(
            seen.insert(key.clone()),
            "Duplicate model key: {} {:?}",
            key.name,
            key.variant
        );
    }
}

#[test]
fn test_build_rule_db_count_is_42() {
    let db = build_rule_db().expect("rule db should build");
    assert_eq!(db.rules.len(), 42, "expected 42 canonical rule examples");
}

#[test]
fn test_build_model_db_count_is_28() {
    let db = build_model_db().expect("model db should build");
    assert_eq!(
        db.models.len(),
        28,
        "expected 28 canonical model examples"
    );
}
