use crate::variant::{CastToParent, KValue, VariantParam};

// Test types for the new system
#[derive(Clone, Debug)]
struct TestRoot;
#[derive(Clone, Debug)]
struct TestChild;

impl_variant_param!(TestRoot, "test_cat");
impl_variant_param!(TestChild, "test_cat", parent: TestRoot, cast: |_| TestRoot);

#[test]
fn test_variant_param_root() {
    assert_eq!(TestRoot::CATEGORY, "test_cat");
    assert_eq!(TestRoot::VALUE, "TestRoot");
    assert_eq!(TestRoot::PARENT_VALUE, None);
}

#[test]
fn test_variant_param_child() {
    assert_eq!(TestChild::CATEGORY, "test_cat");
    assert_eq!(TestChild::VALUE, "TestChild");
    assert_eq!(TestChild::PARENT_VALUE, Some("TestRoot"));
}

#[test]
fn test_cast_to_parent() {
    let child = TestChild;
    let _parent: TestRoot = child.cast_to_parent();
}

#[derive(Clone, Debug)]
struct TestKRoot;
#[derive(Clone, Debug)]
struct TestKChild;

impl_variant_param!(TestKRoot, "test_k", k: None);
impl_variant_param!(TestKChild, "test_k", parent: TestKRoot, cast: |_| TestKRoot, k: Some(3));

#[test]
fn test_kvalue_via_macro_root() {
    assert_eq!(TestKRoot::CATEGORY, "test_k");
    assert_eq!(TestKRoot::VALUE, "TestKRoot");
    assert_eq!(TestKRoot::PARENT_VALUE, None);
    assert_eq!(TestKRoot::K, None);
}

#[test]
fn test_kvalue_via_macro_child() {
    assert_eq!(TestKChild::CATEGORY, "test_k");
    assert_eq!(TestKChild::VALUE, "TestKChild");
    assert_eq!(TestKChild::PARENT_VALUE, Some("TestKRoot"));
    assert_eq!(TestKChild::K, Some(3));
}

#[test]
fn test_variant_params_macro_empty() {
    let v: Vec<(&str, &str)> = variant_params![];
    assert!(v.is_empty());
}

#[test]
fn test_variant_params_macro_single() {
    fn check<T: VariantParam>() -> Vec<(&'static str, &'static str)> {
        variant_params![T]
    }
    let v = check::<TestRoot>();
    assert_eq!(v, vec![("test_cat", "TestRoot")]);
}

#[test]
fn test_variant_params_macro_multiple() {
    fn check<A: VariantParam, B: VariantParam>() -> Vec<(&'static str, &'static str)> {
        variant_params![A, B]
    }
    let v = check::<TestRoot, TestChild>();
    assert_eq!(v, vec![("test_cat", "TestRoot"), ("test_cat", "TestChild")]);
}

#[test]
fn test_variant_for_problems() {
    use crate::models::algebraic::{BMF, QUBO};
    use crate::models::formula::{CircuitSAT, KSatisfiability, Satisfiability};
    use crate::models::graph::{BicliqueCover, SpinGlass};
    use crate::models::graph::{
        KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching,
        MinimumDominatingSet, MinimumVertexCover,
    };
    use crate::models::misc::{Factoring, PaintShop};
    use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    // Test MaximumIndependentSet variants
    let v = MaximumIndependentSet::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].0, "graph");
    assert_eq!(v[0].1, "SimpleGraph");
    assert_eq!(v[1].0, "weight");
    assert_eq!(v[1].1, "i32");

    // Test MinimumVertexCover
    let v = MinimumVertexCover::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");
    assert_eq!(v[1].1, "i32");

    // Test MinimumDominatingSet
    let v = MinimumDominatingSet::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test MaximumMatching
    let v = MaximumMatching::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test MaxCut
    let v = MaxCut::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test KColoring (has K and graph parameters)
    let v = KColoring::<K3, SimpleGraph>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], ("k", "K3"));
    assert_eq!(v[1], ("graph", "SimpleGraph"));

    // Test MaximalIS
    let v = MaximalIS::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test MaximumClique
    let v = MaximumClique::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test Satisfiability (no type parameters)
    let v = Satisfiability::variant();
    assert_eq!(v.len(), 0);

    // Test KSatisfiability (K type parameter only)
    let v = KSatisfiability::<K3>::variant();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], ("k", "K3"));

    // Test MaximumSetPacking (weight parameter only)
    let v = MaximumSetPacking::<i32>::variant();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], ("weight", "i32"));

    // Test MinimumSetCovering (weight parameter only)
    let v = MinimumSetCovering::<i32>::variant();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], ("weight", "i32"));

    // Test SpinGlass (graph + weight parameters)
    let v = SpinGlass::<SimpleGraph, f64>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[1].1, "f64");

    let v = SpinGlass::<SimpleGraph, i32>::variant();
    assert_eq!(v[1].1, "i32");

    // Test QUBO (weight parameter only)
    let v = QUBO::<f64>::variant();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], ("weight", "f64"));

    // Test CircuitSAT (no type parameters)
    let v = CircuitSAT::variant();
    assert_eq!(v.len(), 0);

    // Test Factoring (no type parameters)
    let v = Factoring::variant();
    assert_eq!(v.len(), 0);

    // Test BicliqueCover (no type parameters)
    let v = BicliqueCover::variant();
    assert_eq!(v.len(), 0);

    // Test BMF (no type parameters)
    let v = BMF::variant();
    assert_eq!(v.len(), 0);

    // Test PaintShop (no type parameters)
    let v = PaintShop::variant();
    assert_eq!(v.len(), 0);
}

// --- KValue concrete type tests ---

use crate::variant::{K1, K2, K3, K4, K5, KN};

#[test]
fn test_kvalue_k1() {
    assert_eq!(K1::CATEGORY, "k");
    assert_eq!(K1::VALUE, "K1");
    assert_eq!(K1::PARENT_VALUE, Some("KN"));
    assert_eq!(K1::K, Some(1));
}

#[test]
fn test_kvalue_k2() {
    assert_eq!(K2::CATEGORY, "k");
    assert_eq!(K2::VALUE, "K2");
    assert_eq!(K2::PARENT_VALUE, Some("KN"));
    assert_eq!(K2::K, Some(2));
}

#[test]
fn test_kvalue_k3() {
    assert_eq!(K3::CATEGORY, "k");
    assert_eq!(K3::VALUE, "K3");
    assert_eq!(K3::PARENT_VALUE, Some("KN"));
    assert_eq!(K3::K, Some(3));
}

#[test]
fn test_kvalue_k4() {
    assert_eq!(K4::CATEGORY, "k");
    assert_eq!(K4::VALUE, "K4");
    assert_eq!(K4::PARENT_VALUE, Some("KN"));
    assert_eq!(K4::K, Some(4));
}

#[test]
fn test_kvalue_k5() {
    assert_eq!(K5::CATEGORY, "k");
    assert_eq!(K5::VALUE, "K5");
    assert_eq!(K5::PARENT_VALUE, Some("KN"));
    assert_eq!(K5::K, Some(5));
}

#[test]
fn test_kvalue_kn() {
    assert_eq!(KN::CATEGORY, "k");
    assert_eq!(KN::VALUE, "KN");
    assert_eq!(KN::PARENT_VALUE, None);
    assert_eq!(KN::K, None);
}

// --- Graph type VariantParam tests ---

use crate::topology::{BipartiteGraph, Graph, PlanarGraph, SimpleGraph, UnitDiskGraph};

#[test]
fn test_simple_graph_variant_param() {
    assert_eq!(SimpleGraph::CATEGORY, "graph");
    assert_eq!(SimpleGraph::VALUE, "SimpleGraph");
    assert_eq!(SimpleGraph::PARENT_VALUE, None);
}

#[test]
fn test_planar_graph_variant_param() {
    assert_eq!(PlanarGraph::CATEGORY, "graph");
    assert_eq!(PlanarGraph::VALUE, "PlanarGraph");
    assert_eq!(PlanarGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_bipartite_graph_variant_param() {
    assert_eq!(BipartiteGraph::CATEGORY, "graph");
    assert_eq!(BipartiteGraph::VALUE, "BipartiteGraph");
    assert_eq!(BipartiteGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_unit_disk_graph_variant_param() {
    assert_eq!(UnitDiskGraph::CATEGORY, "graph");
    assert_eq!(UnitDiskGraph::VALUE, "UnitDiskGraph");
    assert_eq!(UnitDiskGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_udg_cast_to_parent() {
    let udg = UnitDiskGraph::new(vec![(0.0, 0.0), (0.5, 0.0), (2.0, 0.0)], 1.0);
    let sg: SimpleGraph = udg.cast_to_parent();
    assert_eq!(sg.num_vertices(), 3);
    // Only the first two points are within distance 1.0
    assert!(sg.has_edge(0, 1));
    assert!(!sg.has_edge(0, 2));
}

// --- Weight type VariantParam tests ---

use crate::types::One;

#[test]
fn test_weight_f64_variant_param() {
    assert_eq!(<f64 as VariantParam>::CATEGORY, "weight");
    assert_eq!(<f64 as VariantParam>::VALUE, "f64");
    assert_eq!(<f64 as VariantParam>::PARENT_VALUE, None);
}

#[test]
fn test_weight_i32_variant_param() {
    assert_eq!(<i32 as VariantParam>::CATEGORY, "weight");
    assert_eq!(<i32 as VariantParam>::VALUE, "i32");
    assert_eq!(<i32 as VariantParam>::PARENT_VALUE, Some("f64"));
}

#[test]
fn test_weight_one_variant_param() {
    assert_eq!(One::CATEGORY, "weight");
    assert_eq!(One::VALUE, "One");
    assert_eq!(One::PARENT_VALUE, Some("i32"));
}

#[test]
fn test_weight_cast_chain() {
    let one = One;
    let i: i32 = one.cast_to_parent();
    assert_eq!(i, 1);
    let f: f64 = i.cast_to_parent();
    assert_eq!(f, 1.0);
}

// --- VariantSpec tests ---

use crate::variant::VariantSpec;

#[test]
fn variant_spec_basic_construction() {
    let spec = VariantSpec::try_from_pairs(vec![("graph", "SimpleGraph"), ("weight", "i32")])
        .expect("valid pairs should succeed");
    let map = spec.as_map();
    assert_eq!(map.len(), 2);
    assert_eq!(map["graph"], "SimpleGraph");
    assert_eq!(map["weight"], "i32");
}

#[test]
fn variant_spec_empty_construction() {
    let spec = VariantSpec::try_from_pairs(Vec::<(&str, &str)>::new())
        .expect("empty pairs should succeed");
    assert!(spec.as_map().is_empty());
}

#[test]
fn variant_spec_rejects_duplicate_dimensions() {
    let result =
        VariantSpec::try_from_pairs(vec![("graph", "SimpleGraph"), ("graph", "PlanarGraph")]);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("duplicate dimension"),
        "error should mention duplicate dimension, got: {err_msg}"
    );
}

#[test]
fn variant_spec_preserves_btreemap_order() {
    // BTreeMap sorts by key, so insertion order doesn't matter
    let spec = VariantSpec::try_from_pairs(vec![("weight", "i32"), ("graph", "SimpleGraph")])
        .expect("valid pairs");
    let keys: Vec<&String> = spec.as_map().keys().collect();
    assert_eq!(keys, vec!["graph", "weight"], "BTreeMap should sort keys");
}

#[test]
fn variant_spec_normalizes_empty_graph_to_simple_graph() {
    // A variant with graph="" should normalize to graph="SimpleGraph"
    let spec =
        VariantSpec::try_from_pairs(vec![("graph", ""), ("weight", "i32")]).expect("valid pairs");
    let normalized = spec.normalize();
    assert_eq!(
        normalized.as_map()["graph"],
        "SimpleGraph",
        "normalize() should fill in 'SimpleGraph' for empty graph dimension"
    );
}

#[test]
fn variant_spec_normalize_preserves_explicit_values() {
    // A variant with explicit values should not be changed by normalize
    let spec = VariantSpec::try_from_pairs(vec![("graph", "PlanarGraph"), ("weight", "f64")])
        .expect("valid pairs");
    let normalized = spec.normalize();
    assert_eq!(normalized.as_map()["graph"], "PlanarGraph");
    assert_eq!(normalized.as_map()["weight"], "f64");
}

#[test]
fn variant_spec_is_default_for_default_values() {
    // A variant with all default values (SimpleGraph, One) should be the default
    let spec = VariantSpec::try_from_pairs(vec![("graph", "SimpleGraph"), ("weight", "One")])
        .expect("valid pairs");
    assert!(
        spec.is_default(),
        "variant with SimpleGraph+One should be the default"
    );
}

#[test]
fn variant_spec_is_not_default_for_non_default_values() {
    // A variant with non-default values should NOT be the default
    let spec = VariantSpec::try_from_pairs(vec![("graph", "PlanarGraph"), ("weight", "i32")])
        .expect("valid pairs");
    assert!(
        !spec.is_default(),
        "variant with PlanarGraph+i32 should not be the default"
    );
}

#[test]
fn variant_spec_try_from_map() {
    let map = std::collections::BTreeMap::from([
        ("graph".to_string(), "SimpleGraph".to_string()),
        ("weight".to_string(), "i32".to_string()),
    ]);
    let spec = VariantSpec::try_from_map(map.clone()).expect("should succeed for valid map");
    assert_eq!(spec.as_map(), &map);
}

#[test]
fn variant_spec_into_map_returns_owned() {
    let spec = VariantSpec::try_from_pairs(vec![("graph", "SimpleGraph"), ("weight", "One")])
        .expect("valid pairs");
    let map = spec.into_map();
    assert_eq!(map.len(), 2);
    assert_eq!(map["graph"], "SimpleGraph");
    assert_eq!(map["weight"], "One");
}

#[test]
fn variant_spec_update_dimension_adds_new() {
    let mut spec =
        VariantSpec::try_from_pairs(vec![("graph", "SimpleGraph")]).expect("valid pairs");
    spec.update_dimension("weight", "i32");
    assert_eq!(spec.as_map().len(), 2);
    assert_eq!(spec.as_map()["weight"], "i32");
}

#[test]
fn variant_spec_update_dimension_overwrites_existing() {
    let mut spec = VariantSpec::try_from_pairs(vec![("graph", "SimpleGraph"), ("weight", "One")])
        .expect("valid pairs");
    spec.update_dimension("weight", "f64");
    assert_eq!(spec.as_map()["weight"], "f64");
}

#[test]
fn variant_spec_normalize_no_graph_dimension_unchanged() {
    // A variant without a "graph" dimension should not be changed
    let spec = VariantSpec::try_from_pairs(vec![("weight", "i32")]).expect("valid pairs");
    let normalized = spec.normalize();
    assert_eq!(normalized.as_map().len(), 1);
    assert_eq!(normalized.as_map()["weight"], "i32");
}

#[test]
fn variant_spec_is_default_empty_variant() {
    let spec = VariantSpec::try_from_pairs(Vec::<(&str, &str)>::new())
        .expect("empty pairs should succeed");
    assert!(
        spec.is_default(),
        "empty variant should be considered default"
    );
}

#[test]
fn variant_spec_is_default_kn() {
    let spec = VariantSpec::try_from_pairs(vec![("k", "KN")]).expect("valid pairs");
    assert!(
        spec.is_default(),
        "variant with KN should be considered default"
    );
}

#[test]
fn variant_spec_is_not_default_mixed() {
    let spec = VariantSpec::try_from_pairs(vec![("graph", "SimpleGraph"), ("weight", "i32")])
        .expect("valid pairs");
    assert!(
        !spec.is_default(),
        "variant with i32 weight should not be default"
    );
}
