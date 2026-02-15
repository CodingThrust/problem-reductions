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

use crate::topology::HyperGraph;
use crate::topology::{Graph, SimpleGraph, UnitDiskGraph};

#[test]
fn test_simple_graph_variant_param() {
    assert_eq!(SimpleGraph::CATEGORY, "graph");
    assert_eq!(SimpleGraph::VALUE, "SimpleGraph");
    assert_eq!(SimpleGraph::PARENT_VALUE, Some("HyperGraph"));
}

#[test]
fn test_unit_disk_graph_variant_param() {
    assert_eq!(UnitDiskGraph::CATEGORY, "graph");
    assert_eq!(UnitDiskGraph::VALUE, "UnitDiskGraph");
    assert_eq!(UnitDiskGraph::PARENT_VALUE, Some("SimpleGraph"));
}

#[test]
fn test_hyper_graph_variant_param() {
    assert_eq!(HyperGraph::CATEGORY, "graph");
    assert_eq!(HyperGraph::VALUE, "HyperGraph");
    assert_eq!(HyperGraph::PARENT_VALUE, None);
}

#[test]
fn test_simple_graph_cast_to_parent() {
    let sg = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let hg: HyperGraph = sg.cast_to_parent();
    assert_eq!(hg.num_vertices(), 3);
    assert_eq!(hg.num_edges(), 2);
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
