//! Problem category types for classification and discovery.
//!
//! This module defines a hierarchical category system for NP-complete problems.
//! Each problem belongs to a top-level category (e.g., Graph, Satisfiability)
//! and a subcategory (e.g., Independent, Coloring).
//!
//! # Category Hierarchy
//!
//! ```text
//! ProblemCategory
//! ├── Graph
//! │   ├── Coloring      (3-Coloring, Chromatic Number)
//! │   ├── Covering      (Vertex Cover, Dominating Set)
//! │   ├── Independent   (Independent Set, Clique)
//! │   ├── Paths         (Hamiltonian Path, TSP)
//! │   ├── Structure     (Graph Partition)
//! │   ├── Trees         (Steiner Tree)
//! │   └── Matching      (3D Matching)
//! ├── Satisfiability
//! │   ├── Sat           (SAT, 3-SAT, Max-SAT)
//! │   ├── Circuit       (Circuit SAT)
//! │   └── Qbf           (QBF)
//! ├── Set
//! │   ├── Covering      (Set Cover, Exact Cover)
//! │   ├── Packing       (Bin Packing, Knapsack)
//! │   ├── Partition     (Partition, Subset Sum)
//! │   └── Matching      (Hitting Set)
//! ├── Optimization
//! │   ├── Quadratic     (QUBO, Max-Cut)
//! │   ├── Linear        (ILP)
//! │   └── Constraint    (CSP)
//! ├── Scheduling
//! │   ├── Machine       (Job Shop)
//! │   ├── Sequencing    (Sequencing)
//! │   └── Resource      (Resource Allocation)
//! ├── Network
//! │   ├── Flow          (Network Flow)
//! │   ├── Routing       (Routing)
//! │   └── Connectivity  (k-Connectivity)
//! ├── String
//! │   ├── Sequence      (Shortest Superstring)
//! │   ├── Matching      (String Matching)
//! │   └── Compression   (Grammar Compression)
//! └── Specialized
//!     ├── Geometry      (Protein Folding)
//!     ├── Number        (Factoring)
//!     ├── Game          (Game Theory)
//!     └── Other
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Top-level problem category.
///
/// Problems are organized into a two-level hierarchy: category and subcategory.
/// Use [`path()`](ProblemCategory::path) to get the full path (e.g., "graph/independent").
///
/// # Example
///
/// ```rust
/// use problemreductions::registry::{ProblemCategory, GraphSubcategory};
///
/// let cat = ProblemCategory::Graph(GraphSubcategory::Independent);
/// assert_eq!(cat.name(), "graph");
/// assert_eq!(cat.subcategory_name(), "independent");
/// assert_eq!(cat.path(), "graph/independent");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProblemCategory {
    /// Graph-based problems (coloring, covering, paths, etc.)
    Graph(GraphSubcategory),
    /// Boolean satisfiability problems
    Satisfiability(SatisfiabilitySubcategory),
    /// Set-based problems (covering, packing, partition)
    Set(SetSubcategory),
    /// Optimization problems (quadratic, linear, constraint)
    Optimization(OptimizationSubcategory),
    /// Scheduling and resource allocation
    Scheduling(SchedulingSubcategory),
    /// Network flow and routing problems
    Network(NetworkSubcategory),
    /// String and sequence problems
    String(StringSubcategory),
    /// Specialized domain-specific problems
    Specialized(SpecializedSubcategory),
}

impl ProblemCategory {
    /// Get the top-level category name.
    pub fn name(&self) -> &'static str {
        match self {
            ProblemCategory::Graph(_) => "graph",
            ProblemCategory::Satisfiability(_) => "satisfiability",
            ProblemCategory::Set(_) => "set",
            ProblemCategory::Optimization(_) => "optimization",
            ProblemCategory::Scheduling(_) => "scheduling",
            ProblemCategory::Network(_) => "network",
            ProblemCategory::String(_) => "string",
            ProblemCategory::Specialized(_) => "specialized",
        }
    }

    /// Get the subcategory name.
    pub fn subcategory_name(&self) -> &'static str {
        match self {
            ProblemCategory::Graph(sub) => sub.name(),
            ProblemCategory::Satisfiability(sub) => sub.name(),
            ProblemCategory::Set(sub) => sub.name(),
            ProblemCategory::Optimization(sub) => sub.name(),
            ProblemCategory::Scheduling(sub) => sub.name(),
            ProblemCategory::Network(sub) => sub.name(),
            ProblemCategory::String(sub) => sub.name(),
            ProblemCategory::Specialized(sub) => sub.name(),
        }
    }

    /// Get the full path as "category/subcategory".
    pub fn path(&self) -> String {
        format!("{}/{}", self.name(), self.subcategory_name())
    }
}

impl fmt::Display for ProblemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path())
    }
}

/// Graph problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GraphSubcategory {
    /// Vertex/edge coloring problems
    Coloring,
    /// Vertex/edge covering problems
    Covering,
    /// Independent set and clique problems
    Independent,
    /// Path and cycle problems (Hamiltonian, TSP)
    Paths,
    /// Graph structure and partitioning
    Structure,
    /// Tree problems (Steiner, spanning)
    Trees,
    /// Matching problems
    Matching,
}

impl GraphSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            GraphSubcategory::Coloring => "coloring",
            GraphSubcategory::Covering => "covering",
            GraphSubcategory::Independent => "independent",
            GraphSubcategory::Paths => "paths",
            GraphSubcategory::Structure => "structure",
            GraphSubcategory::Trees => "trees",
            GraphSubcategory::Matching => "matching",
        }
    }
}

/// Satisfiability problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SatisfiabilitySubcategory {
    /// SAT and variants (3-SAT, Max-SAT)
    Sat,
    /// Circuit satisfiability
    Circuit,
    /// Quantified Boolean formulas
    Qbf,
}

impl SatisfiabilitySubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            SatisfiabilitySubcategory::Sat => "sat",
            SatisfiabilitySubcategory::Circuit => "circuit",
            SatisfiabilitySubcategory::Qbf => "qbf",
        }
    }
}

/// Set problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetSubcategory {
    /// Set covering and exact cover
    Covering,
    /// Set packing, bin packing, knapsack
    Packing,
    /// Partition and subset sum
    Partition,
    /// Set splitting and hitting set
    Matching,
}

impl SetSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            SetSubcategory::Covering => "covering",
            SetSubcategory::Packing => "packing",
            SetSubcategory::Partition => "partition",
            SetSubcategory::Matching => "matching",
        }
    }
}

/// Optimization problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationSubcategory {
    /// Quadratic optimization (QUBO, Max-Cut, Ising)
    Quadratic,
    /// Linear and integer programming
    Linear,
    /// Constraint-based optimization
    Constraint,
}

impl OptimizationSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            OptimizationSubcategory::Quadratic => "quadratic",
            OptimizationSubcategory::Linear => "linear",
            OptimizationSubcategory::Constraint => "constraint",
        }
    }
}

/// Scheduling problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchedulingSubcategory {
    /// Multiprocessor and machine scheduling
    Machine,
    /// Sequencing with constraints
    Sequencing,
    /// Resource allocation
    Resource,
}

impl SchedulingSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            SchedulingSubcategory::Machine => "machine",
            SchedulingSubcategory::Sequencing => "sequencing",
            SchedulingSubcategory::Resource => "resource",
        }
    }
}

/// Network problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkSubcategory {
    /// Network flow problems
    Flow,
    /// Routing and path problems
    Routing,
    /// Connectivity problems
    Connectivity,
}

impl NetworkSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            NetworkSubcategory::Flow => "flow",
            NetworkSubcategory::Routing => "routing",
            NetworkSubcategory::Connectivity => "connectivity",
        }
    }
}

/// String problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StringSubcategory {
    /// Sequence problems (superstring, subsequence)
    Sequence,
    /// String matching
    Matching,
    /// Compression problems
    Compression,
}

impl StringSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            StringSubcategory::Sequence => "sequence",
            StringSubcategory::Matching => "matching",
            StringSubcategory::Compression => "compression",
        }
    }
}

/// Specialized problem subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecializedSubcategory {
    /// Geometric problems
    Geometry,
    /// Number-theoretic problems
    Number,
    /// Game-theoretic problems
    Game,
    /// Other specialized problems
    Other,
}

impl SpecializedSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            SpecializedSubcategory::Geometry => "geometry",
            SpecializedSubcategory::Number => "number",
            SpecializedSubcategory::Game => "game",
            SpecializedSubcategory::Other => "other",
        }
    }
}

#[cfg(test)]
mod tests {
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
}
