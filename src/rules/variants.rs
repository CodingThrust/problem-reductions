//! Concrete variant registrations for problems that exist as valid graph nodes
//! but aren't the source or target of any explicit reduction rule.

use crate::rules::registry::ConcreteVariantEntry;

// Unit-weighted graph problems — valid variants that need nodes in the reduction graph
// for natural weight-promotion edges (e.g., MIS/One → MIS/i32).
inventory::submit! {
    ConcreteVariantEntry {
        name: "MaximumIndependentSet",
        variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
    }
}
inventory::submit! {
    ConcreteVariantEntry {
        name: "MinimumVertexCover",
        variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
    }
}
inventory::submit! {
    ConcreteVariantEntry {
        name: "MaxCut",
        variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
    }
}
inventory::submit! {
    ConcreteVariantEntry {
        name: "SpinGlass",
        variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
    }
}
inventory::submit! {
    ConcreteVariantEntry {
        name: "MaximumMatching",
        variant_fn: || vec![("graph", "SimpleGraph"), ("weight", "One")],
    }
}
inventory::submit! {
    ConcreteVariantEntry {
        name: "MaximumSetPacking",
        variant_fn: || vec![("weight", "One")],
    }
}
inventory::submit! {
    ConcreteVariantEntry {
        name: "MinimumSetCovering",
        variant_fn: || vec![("weight", "One")],
    }
}
