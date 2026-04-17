use crate::registry::variant::{validate_variant_aliases, variant_label};
use std::collections::BTreeMap;

#[test]
fn variant_alias_inventory_is_valid() {
    if let Err(conflicts) = validate_variant_aliases() {
        panic!("variant alias validation failed:\n{}", conflicts.join("\n"));
    }
}

// --- validate_aliases_inner unit tests ---

use crate::registry::variant::validate_aliases_inner;

fn empty_problem_names() -> BTreeMap<String, Vec<String>> {
    BTreeMap::new()
}

#[test]
fn validate_inner_accepts_valid_aliases() {
    let entries = vec![
        ("Foo {k=K3}".to_string(), &["3FOO"][..]),
        ("Foo {k=K2}".to_string(), &["2FOO"][..]),
    ];
    assert!(validate_aliases_inner(&empty_problem_names(), &entries).is_ok());
}

#[test]
fn validate_inner_rejects_empty_alias() {
    let entries = vec![("Foo {k=K3}".to_string(), &[""][..])];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(
        err[0].contains("empty or whitespace-only"),
        "expected empty alias error, got: {}",
        err[0]
    );
}

#[test]
fn validate_inner_rejects_whitespace_only_alias() {
    let entries = vec![("Foo".to_string(), &["  \t"][..])];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert!(err[0].contains("empty or whitespace-only"));
}

#[test]
fn validate_inner_rejects_collision_with_canonical_name() {
    let mut names = BTreeMap::new();
    names
        .entry("bar".to_string())
        .or_insert_with(Vec::new)
        .push("canonical problem name `Bar`".to_string());

    let entries = vec![("Foo {k=K3}".to_string(), &["BAR"][..])];
    let err = validate_aliases_inner(&names, &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("conflicts with canonical problem name"));
}

#[test]
fn validate_inner_rejects_collision_with_problem_level_alias() {
    let mut names = BTreeMap::new();
    names
        .entry("baz".to_string())
        .or_insert_with(Vec::new)
        .push("problem-level alias `BAZ` for `Bazinga`".to_string());

    let entries = vec![("Foo".to_string(), &["baz"][..])];
    let err = validate_aliases_inner(&names, &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(err[0].contains("conflicts with problem-level alias"));
}

#[test]
fn validate_inner_rejects_duplicate_variant_aliases() {
    let entries = vec![
        ("Foo {k=K3}".to_string(), &["DUP"][..]),
        ("Bar {k=K2}".to_string(), &["dup"][..]),
    ];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert_eq!(err.len(), 1);
    assert!(
        err[0].contains("duplicate variant-level alias"),
        "expected duplicate error, got: {}",
        err[0]
    );
}

#[test]
fn validate_inner_reports_multiple_conflicts() {
    let entries = vec![
        ("A".to_string(), &[""][..]),
        ("B".to_string(), &["X"][..]),
        ("C".to_string(), &["x"][..]),
    ];
    let err = validate_aliases_inner(&empty_problem_names(), &entries).unwrap_err();
    assert_eq!(err.len(), 2, "expected 2 conflicts, got: {err:?}");
}

// --- variant_label unit tests ---

#[test]
fn variant_label_bare_problem() {
    // Find a VariantEntry with no variant dimensions (empty variant list).
    // QUBO is a standalone problem with no variants.
    let entry = inventory::iter::<crate::registry::VariantEntry>()
        .find(|e| e.variant().is_empty())
        .expect("expected at least one VariantEntry with empty variant");
    let label = variant_label(entry);
    assert_eq!(label, entry.name);
}

#[test]
fn variant_label_with_variant_dimensions() {
    let entry = inventory::iter::<crate::registry::VariantEntry>()
        .find(|e| e.name == "KSatisfiability" && e.aliases.contains(&"3SAT"))
        .expect("expected KSatisfiability<K3> VariantEntry");
    let label = variant_label(entry);
    assert!(
        label.contains("k=K3"),
        "expected label to include k=K3, got: {label}"
    );
}
