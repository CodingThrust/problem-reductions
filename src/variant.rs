//! Variant attribute utilities.

use std::any::type_name;

/// Convert const generic usize to static str (for common values).
///
/// This is useful for including const generic parameters in problem variant IDs.
/// For values 1-10, returns the string representation. For other values, returns "N".
///
/// # Example
///
/// ```
/// use problemreductions::variant::const_usize_str;
///
/// assert_eq!(const_usize_str::<3>(), "3");
/// assert_eq!(const_usize_str::<10>(), "10");
/// assert_eq!(const_usize_str::<100>(), "N");
/// ```
pub const fn const_usize_str<const N: usize>() -> &'static str {
    match N {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        _ => "N",
    }
}

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
            DominatingSet, IndependentSet, KColoring, Matching, MaxCut, MaximalIS, VertexCovering,
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

        // Test KColoring (has K, graph, and weight parameters)
        let v = KColoring::<3, SimpleGraph, i32>::variant();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], ("k", "3"));
        assert_eq!(v[1], ("graph", "SimpleGraph"));
        assert_eq!(v[2], ("weight", "i32"));

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
}
