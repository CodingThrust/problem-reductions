use super::*;
use crate::variant::{CastToParent, KValue, VariantParam, VariantTypeEntry};

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

#[test]
fn test_variant_type_entry_registered() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "test_cat")
        .collect();
    assert!(entries
        .iter()
        .any(|e| e.value == "TestRoot" && e.parent.is_none()));
    assert!(entries
        .iter()
        .any(|e| e.value == "TestChild" && e.parent == Some("TestRoot")));
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
fn test_short_type_name_primitive() {
    assert_eq!(short_type_name::<i32>(), "i32");
    assert_eq!(short_type_name::<f64>(), "f64");
}

#[test]
fn test_short_type_name_struct() {
    struct MyStruct;
    assert_eq!(short_type_name::<MyStruct>(), "MyStruct");
}

#[test]
fn test_const_usize_str() {
    assert_eq!(const_usize_str::<1>(), "1");
    assert_eq!(const_usize_str::<2>(), "2");
    assert_eq!(const_usize_str::<3>(), "3");
    assert_eq!(const_usize_str::<4>(), "4");
    assert_eq!(const_usize_str::<5>(), "5");
    assert_eq!(const_usize_str::<6>(), "6");
    assert_eq!(const_usize_str::<7>(), "7");
    assert_eq!(const_usize_str::<8>(), "8");
    assert_eq!(const_usize_str::<9>(), "9");
    assert_eq!(const_usize_str::<10>(), "10");
    assert_eq!(const_usize_str::<11>(), "N");
    assert_eq!(const_usize_str::<100>(), "N");
}

#[test]
fn test_variant_for_problems() {
    use crate::models::graph::{
        KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching,
        MinimumDominatingSet, MinimumVertexCover,
    };
    use crate::models::optimization::{SpinGlass, QUBO};
    use crate::models::satisfiability::{KSatisfiability, Satisfiability};
    use crate::models::set::{MaximumSetPacking, MinimumSetCovering};
    use crate::models::specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
    use crate::topology::SimpleGraph;
    use crate::traits::Problem;

    // Test MaximumIndependentSet variants
    let v = MaximumIndependentSet::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].0, "graph");
    assert_eq!(v[0].1, "SimpleGraph");
    assert_eq!(v[1].0, "weight");
    assert_eq!(v[1].1, "i32");

    // Note: f64 variants removed because SolutionSize now requires Ord

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

    // Note: f64 variants removed because SolutionSize now requires Ord

    // Test KColoring (has K and graph parameters)
    let v = KColoring::<3, SimpleGraph>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], ("k", "3"));
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

    // Test KSatisfiability (const K parameter only)
    let v = KSatisfiability::<3>::variant();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], ("k", "3"));

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

use crate::variant::{K2, K3, KN};

#[test]
fn test_kvalue_k2() {
    assert_eq!(K2::CATEGORY, "k");
    assert_eq!(K2::VALUE, "K2");
    assert_eq!(K2::PARENT_VALUE, Some("K3"));
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
fn test_kvalue_kn() {
    assert_eq!(KN::CATEGORY, "k");
    assert_eq!(KN::VALUE, "KN");
    assert_eq!(KN::PARENT_VALUE, None);
    assert_eq!(KN::K, None);
}

#[test]
fn test_kvalue_cast_chain() {
    let k2 = K2;
    let k3: K3 = k2.cast_to_parent();
    let kn: KN = k3.cast_to_parent();
    assert_eq!(KN::K, None);
    let _ = kn; // use it
}

#[test]
fn test_kvalue_variant_entries() {
    let entries: Vec<_> = inventory::iter::<VariantTypeEntry>()
        .filter(|e| e.category == "k")
        .collect();
    assert!(entries.iter().any(|e| e.value == "KN" && e.parent.is_none()));
    assert!(entries
        .iter()
        .any(|e| e.value == "K3" && e.parent == Some("KN")));
    assert!(entries
        .iter()
        .any(|e| e.value == "K2" && e.parent == Some("K3")));
}
