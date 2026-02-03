//! Variant attribute utilities.

use std::any::type_name;

/// Extract short type name from full path.
/// e.g., "problemreductions::graph_types::SimpleGraph" -> "SimpleGraph"
pub fn short_type_name<T: 'static>() -> &'static str {
    let full = type_name::<T>();
    full.rsplit("::").next().unwrap_or(full)
}

#[cfg(test)]
mod tests {
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
    fn test_variant_for_problems() {
        use crate::models::graph::{
            Coloring, DominatingSet, IndependentSet, Matching, MaxCut, MaximalIS, VertexCovering,
        };
        use crate::models::optimization::{SpinGlass, QUBO};
        use crate::models::satisfiability::{KSatisfiability, Satisfiability};
        use crate::models::set::{SetCovering, SetPacking};
        use crate::models::specialized::{BicliqueCover, CircuitSAT, Factoring, PaintShop, BMF};
        use crate::topology::SimpleGraph;
        use crate::traits::Problem;

        // Test IndependentSet variants
        let v = IndependentSet::<SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].0, "graph");
        assert_eq!(v[0].1, "SimpleGraph");
        assert_eq!(v[1].0, "weight");
        assert_eq!(v[1].1, "i32");

        let v = IndependentSet::<SimpleGraph, f64>::variant();
        assert_eq!(v[1].1, "f64");

        // Test VertexCovering
        let v = VertexCovering::<SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].1, "SimpleGraph");
        assert_eq!(v[1].1, "i32");

        // Test DominatingSet
        let v = DominatingSet::<SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].1, "SimpleGraph");

        // Test Matching
        let v = Matching::<SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].1, "SimpleGraph");

        // Test MaxCut
        let v = MaxCut::<SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].1, "SimpleGraph");

        let v = MaxCut::<SimpleGraph, f64>::variant();
        assert_eq!(v[1].1, "f64");

        // Test Coloring (no weight parameter)
        let v = Coloring::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].1, "SimpleGraph");

        // Test MaximalIS (no weight parameter)
        let v = MaximalIS::<SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].1, "SimpleGraph");

        // Test Satisfiability
        let v = Satisfiability::<i32>::variant();
        assert_eq!(v.len(), 2);

        // Test KSatisfiability
        let v = KSatisfiability::<3, i32>::variant();
        assert_eq!(v.len(), 2);

        // Test SetPacking
        let v = SetPacking::<i32>::variant();
        assert_eq!(v.len(), 2);

        // Test SetCovering
        let v = SetCovering::<i32>::variant();
        assert_eq!(v.len(), 2);

        // Test SpinGlass
        let v = SpinGlass::<f64>::variant();
        assert_eq!(v.len(), 2);
        assert_eq!(v[1].1, "f64");

        let v = SpinGlass::<i32>::variant();
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
}
