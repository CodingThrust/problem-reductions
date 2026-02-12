use super::*;

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

    // Test KColoring (has K, graph, and weight parameters)
    let v = KColoring::<3, SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], ("k", "3"));
    assert_eq!(v[1], ("graph", "SimpleGraph"));
    assert_eq!(v[2], ("weight", "i32"));

    // Test MaximalIS
    let v = MaximalIS::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test MaximumClique
    let v = MaximumClique::<SimpleGraph, i32>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test Satisfiability
    let v = Satisfiability::variant();
    assert_eq!(v.len(), 2);

    // Test KSatisfiability
    let v = KSatisfiability::<3>::variant();
    assert_eq!(v.len(), 2);

    // Test MaximumSetPacking
    let v = MaximumSetPacking::<i32>::variant();
    assert_eq!(v.len(), 2);

    // Test MinimumSetCovering
    let v = MinimumSetCovering::<i32>::variant();
    assert_eq!(v.len(), 2);

    // Test SpinGlass
    let v = SpinGlass::<SimpleGraph, f64>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[1].1, "f64");

    let v = SpinGlass::<SimpleGraph, i32>::variant();
    assert_eq!(v[1].1, "i32");

    // Test QUBO
    let v = QUBO::<f64>::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[1].1, "f64");

    // Test CircuitSAT
    let v = CircuitSAT::<i32>::variant();
    assert_eq!(v.len(), 2);

    // Test Factoring (no type parameters)
    let v = Factoring::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");
    assert_eq!(v[1].1, "i32");

    // Test BicliqueCover (no type parameters)
    let v = BicliqueCover::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test BMF (no type parameters)
    let v = BMF::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");

    // Test PaintShop (no type parameters)
    let v = PaintShop::variant();
    assert_eq!(v.len(), 2);
    assert_eq!(v[0].1, "SimpleGraph");
}
