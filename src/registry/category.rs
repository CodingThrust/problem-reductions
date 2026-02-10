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
//! │   ├── Independent   (Independent Set, MaximumClique)
//! │   ├── Paths         (Hamiltonian Path, TSP)
//! │   ├── Structure     (Graph Partition)
//! │   ├── Trees         (Steiner Tree)
//! │   └── MaximumMatching      (3D MaximumMatching)
//! ├── Satisfiability
//! │   ├── Sat           (SAT, 3-SAT, Max-SAT)
//! │   ├── Circuit       (Circuit SAT)
//! │   └── Qbf           (QBF)
//! ├── Set
//! │   ├── Covering      (Set Cover, Exact Cover)
//! │   ├── Packing       (Bin Packing, Knapsack)
//! │   ├── Partition     (Partition, Subset Sum)
//! │   └── MaximumMatching      (Hitting Set)
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
//! │   ├── MaximumMatching      (String MaximumMatching)
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
    /// MaximumMatching problems
    MaximumMatching,
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
            GraphSubcategory::MaximumMatching => "matching",
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
    MaximumMatching,
}

impl SetSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            SetSubcategory::Covering => "covering",
            SetSubcategory::Packing => "packing",
            SetSubcategory::Partition => "partition",
            SetSubcategory::MaximumMatching => "matching",
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
    MaximumMatching,
    /// Compression problems
    Compression,
}

impl StringSubcategory {
    /// Get the subcategory name.
    pub fn name(&self) -> &'static str {
        match self {
            StringSubcategory::Sequence => "sequence",
            StringSubcategory::MaximumMatching => "matching",
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
#[path = "../unit_tests/registry/category.rs"]
mod tests;
