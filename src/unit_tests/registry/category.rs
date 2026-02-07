use super::*;

#[test]
fn test_category_path() {
    let cat = ProblemCategory::Graph(GraphSubcategory::Independent);
    assert_eq!(cat.path(), "graph/independent");
    assert_eq!(cat.name(), "graph");
    assert_eq!(cat.subcategory_name(), "independent");
}

#[test]
fn test_category_display() {
    let cat = ProblemCategory::Satisfiability(SatisfiabilitySubcategory::Sat);
    assert_eq!(format!("{}", cat), "satisfiability/sat");
}

#[test]
fn test_all_subcategories() {
    // Graph
    assert_eq!(GraphSubcategory::Coloring.name(), "coloring");
    assert_eq!(GraphSubcategory::Covering.name(), "covering");
    assert_eq!(GraphSubcategory::Independent.name(), "independent");
    assert_eq!(GraphSubcategory::Paths.name(), "paths");
    assert_eq!(GraphSubcategory::Structure.name(), "structure");
    assert_eq!(GraphSubcategory::Trees.name(), "trees");
    assert_eq!(GraphSubcategory::Matching.name(), "matching");

    // Satisfiability
    assert_eq!(SatisfiabilitySubcategory::Sat.name(), "sat");
    assert_eq!(SatisfiabilitySubcategory::Circuit.name(), "circuit");
    assert_eq!(SatisfiabilitySubcategory::Qbf.name(), "qbf");

    // Set
    assert_eq!(SetSubcategory::Covering.name(), "covering");
    assert_eq!(SetSubcategory::Packing.name(), "packing");
    assert_eq!(SetSubcategory::Partition.name(), "partition");
    assert_eq!(SetSubcategory::Matching.name(), "matching");

    // Optimization
    assert_eq!(OptimizationSubcategory::Quadratic.name(), "quadratic");
    assert_eq!(OptimizationSubcategory::Linear.name(), "linear");
    assert_eq!(OptimizationSubcategory::Constraint.name(), "constraint");

    // Scheduling
    assert_eq!(SchedulingSubcategory::Machine.name(), "machine");
    assert_eq!(SchedulingSubcategory::Sequencing.name(), "sequencing");
    assert_eq!(SchedulingSubcategory::Resource.name(), "resource");

    // Network
    assert_eq!(NetworkSubcategory::Flow.name(), "flow");
    assert_eq!(NetworkSubcategory::Routing.name(), "routing");
    assert_eq!(NetworkSubcategory::Connectivity.name(), "connectivity");

    // String
    assert_eq!(StringSubcategory::Sequence.name(), "sequence");
    assert_eq!(StringSubcategory::Matching.name(), "matching");
    assert_eq!(StringSubcategory::Compression.name(), "compression");

    // Specialized
    assert_eq!(SpecializedSubcategory::Geometry.name(), "geometry");
    assert_eq!(SpecializedSubcategory::Number.name(), "number");
    assert_eq!(SpecializedSubcategory::Game.name(), "game");
    assert_eq!(SpecializedSubcategory::Other.name(), "other");
}

#[test]
fn test_all_category_paths() {
    // Test ProblemCategory name() and subcategory_name() for all variants
    let categories = [
        ProblemCategory::Graph(GraphSubcategory::Coloring),
        ProblemCategory::Satisfiability(SatisfiabilitySubcategory::Sat),
        ProblemCategory::Set(SetSubcategory::Covering),
        ProblemCategory::Optimization(OptimizationSubcategory::Quadratic),
        ProblemCategory::Scheduling(SchedulingSubcategory::Machine),
        ProblemCategory::Network(NetworkSubcategory::Flow),
        ProblemCategory::String(StringSubcategory::Sequence),
        ProblemCategory::Specialized(SpecializedSubcategory::Geometry),
    ];

    let expected_names = [
        "graph",
        "satisfiability",
        "set",
        "optimization",
        "scheduling",
        "network",
        "string",
        "specialized",
    ];

    let expected_subcategories = [
        "coloring",
        "sat",
        "covering",
        "quadratic",
        "machine",
        "flow",
        "sequence",
        "geometry",
    ];

    for (i, cat) in categories.iter().enumerate() {
        assert_eq!(cat.name(), expected_names[i]);
        assert_eq!(cat.subcategory_name(), expected_subcategories[i]);
        assert!(!cat.path().is_empty());
        // Test Display
        let display = format!("{}", cat);
        assert!(display.contains('/'));
    }
}
