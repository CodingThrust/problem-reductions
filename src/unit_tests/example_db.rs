use crate::example_db::{build_model_db, build_rule_db, find_model_example, find_rule_example};
use crate::export::{lookup_overhead, ProblemRef, EXAMPLE_DB_VERSION};
use std::collections::{BTreeMap, BTreeSet, HashSet};

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
fn test_path_based_rule_example_does_not_require_direct_overhead() {
    let source = ProblemRef {
        name: "MaximumIndependentSet".to_string(),
        variant: BTreeMap::from([
            ("graph".to_string(), "SimpleGraph".to_string()),
            ("weight".to_string(), "i32".to_string()),
        ]),
    };
    let target = ProblemRef {
        name: "ILP".to_string(),
        variant: BTreeMap::from([("variable".to_string(), "bool".to_string())]),
    };

    let example = find_rule_example(&source, &target).expect("path example should exist");
    assert!(
        !example.overhead.is_empty(),
        "path example should carry composed overhead"
    );
    assert!(
        lookup_overhead(&source.name, &source.variant, &target.name, &target.variant).is_none(),
        "path example should not require a direct-edge overhead entry"
    );
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
fn test_build_rule_db_nonempty() {
    let db = build_rule_db().expect("rule db should build");
    assert!(!db.rules.is_empty(), "rule db should not be empty");
}

#[test]
fn test_build_model_db_nonempty() {
    let db = build_model_db().expect("model db should build");
    assert!(!db.models.is_empty(), "model db should not be empty");
}

#[test]
fn canonical_model_example_ids_are_unique() {
    let specs = crate::models::graph::canonical_model_example_specs();
    let specs: Vec<_> = specs
        .into_iter()
        .chain(crate::models::formula::canonical_model_example_specs())
        .chain(crate::models::set::canonical_model_example_specs())
        .chain(crate::models::algebraic::canonical_model_example_specs())
        .chain(crate::models::misc::canonical_model_example_specs())
        .collect();
    let mut seen = HashSet::new();
    for spec in &specs {
        assert!(
            seen.insert(spec.id),
            "Duplicate model example id: {}",
            spec.id
        );
    }
}

#[test]
fn canonical_rule_example_ids_are_unique() {
    let specs = crate::rules::canonical_rule_example_specs();
    let mut seen = HashSet::new();
    for spec in &specs {
        assert!(
            seen.insert(spec.id),
            "Duplicate rule example id: {}",
            spec.id
        );
    }
}

// ---- Error path tests for example_db ----

#[test]
fn find_rule_example_nonexistent_returns_error() {
    let source = ProblemRef {
        name: "NonExistentProblem".to_string(),
        variant: BTreeMap::new(),
    };
    let target = ProblemRef {
        name: "AlsoNonExistent".to_string(),
        variant: BTreeMap::new(),
    };
    let result = find_rule_example(&source, &target);
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("No canonical rule example"),
        "error should mention no canonical rule: {err_msg}"
    );
}

#[test]
fn find_model_example_nonexistent_returns_error() {
    let problem = ProblemRef {
        name: "NonExistentModel".to_string(),
        variant: BTreeMap::from([("graph".to_string(), "SimpleGraph".to_string())]),
    };
    let result = find_model_example(&problem);
    assert!(result.is_err());
    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("No canonical model example"),
        "error should mention no canonical model: {err_msg}"
    );
}

#[test]
fn default_generated_dir_returns_path() {
    use crate::example_db::default_generated_dir;
    let dir = default_generated_dir();
    // Should return a valid path (either from env or the default)
    assert!(!dir.as_os_str().is_empty());
}
