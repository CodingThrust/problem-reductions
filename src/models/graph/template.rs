//! Generic graph problem template.
//!
//! This module provides a template for defining binary graph problems with minimal
//! boilerplate. Instead of writing ~400 lines per problem, you can define a new
//! graph problem in ~15 lines by implementing [`GraphConstraint`].
//!
//! # Overview
//!
//! Binary graph problems share a common structure:
//! - Each vertex can be selected (1) or not selected (0)
//! - Edges impose constraints on which pairs can be simultaneously selected
//! - The objective is to maximize or minimize the total weight of selected vertices
//!
//! This template captures this pattern via:
//! - [`GraphConstraint`] - Trait defining problem-specific semantics
//! - [`GraphProblem<C, W>`] - Generic struct implementing all standard traits
//!
//! # Quick Start
//!
//! ```rust
//! use problemreductions::models::graph::{GraphConstraint, GraphProblem};
//! use problemreductions::registry::GraphSubcategory;
//! use problemreductions::types::EnergyMode;
//!
//! // Step 1: Define the constraint
//! #[derive(Debug, Clone, Copy)]
//! pub struct MyConstraint;
//!
//! impl GraphConstraint for MyConstraint {
//!     const NAME: &'static str = "My Problem";
//!     const DESCRIPTION: &'static str = "Description of my problem";
//!     const ENERGY_MODE: EnergyMode = EnergyMode::LargerSizeIsBetter;
//!     const SUBCATEGORY: GraphSubcategory = GraphSubcategory::Independent;
//!
//!     fn edge_constraint_spec() -> [bool; 4] {
//!         // [neither, only_v, only_u, both] selected
//!         [true, true, true, false]  // At most one endpoint
//!     }
//! }
//!
//! // Step 2: Create a type alias
//! pub type MyProblem<W = i32> = GraphProblem<MyConstraint, W>;
//!
//! // Step 3: Use it!
//! let problem = MyProblem::<i32>::new(3, vec![(0, 1), (1, 2)]);
//! ```
//!
//! # Built-in Problem Types
//!
//! | Type Alias | Constraint | Energy Mode | Edge Spec |
//! |------------|------------|-------------|-----------|
//! | [`IndependentSetT`] | At most one selected | Maximize | `[T,T,T,F]` |
//! | [`VertexCoverT`] | At least one selected | Minimize | `[F,T,T,T]` |
//! | [`CliqueT`] | For complement graph | Maximize | `[T,T,T,F]` |
//!
//! # Edge Constraint Specification
//!
//! The [`edge_constraint_spec`](GraphConstraint::edge_constraint_spec) method returns
//! a 4-element array `[bool; 4]` indexed by `(u_selected * 2 + v_selected)`:
//!
//! | Index | u | v | Meaning |
//! |-------|---|---|---------|
//! | `0` | `0` | `0` | Neither endpoint selected |
//! | `1` | `0` | `1` | Only v selected |
//! | `2` | `1` | `0` | Only u selected |
//! | `3` | `1` | `1` | Both endpoints selected |
//!
//! Common patterns:
//! - **Independent Set**: `[true, true, true, false]` - at most one selected
//! - **Vertex Cover**: `[false, true, true, true]` - at least one selected
//! - **Perfect Matching**: Define on edge graph with exactly one selected

use crate::registry::{ComplexityClass, GraphSubcategory, ProblemCategory, ProblemInfo, ProblemMetadata};
use crate::traits::{ConstraintSatisfactionProblem, Problem};
use crate::types::{EnergyMode, LocalConstraint, LocalSolutionSize, ProblemSize, SolutionSize};
use num_traits::{Num, Zero};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::ops::AddAssign;

/// Trait defining the constraint semantics for a binary graph problem.
///
/// Implement this trait to define a new graph problem type. The trait specifies
/// how edges constrain the selection of vertices and what the optimization objective is.
///
/// # Example
///
/// ```rust,ignore
/// pub struct IndependentSetConstraint;
///
/// impl GraphConstraint for IndependentSetConstraint {
///     const NAME: &'static str = "Independent Set";
///     const DESCRIPTION: &'static str = "Find maximum weight set of non-adjacent vertices";
///     const ENERGY_MODE: EnergyMode = EnergyMode::LargerSizeIsBetter;
///     const SUBCATEGORY: GraphSubcategory = GraphSubcategory::Independent;
///
///     fn edge_constraint_spec() -> [bool; 4] {
///         [true, true, true, false]  // (0,0), (0,1), (1,0) OK; (1,1) invalid
///     }
/// }
/// ```
pub trait GraphConstraint: Clone + Send + Sync + 'static {
    /// The canonical name of this problem.
    const NAME: &'static str;

    /// Brief description of the problem.
    const DESCRIPTION: &'static str;

    /// Whether to maximize or minimize the objective.
    const ENERGY_MODE: EnergyMode;

    /// The graph subcategory this problem belongs to.
    const SUBCATEGORY: GraphSubcategory;

    /// Alternative names for this problem (default: empty).
    const ALIASES: &'static [&'static str] = &[];

    /// The problem this canonically reduces from (default: None).
    const REDUCES_FROM: Option<&'static str> = None;

    /// The edge constraint specification.
    ///
    /// Returns a 4-element array representing which (u_selected, v_selected)
    /// combinations are valid for an edge (u, v):
    /// - Index 0: (0, 0) - neither endpoint selected
    /// - Index 1: (0, 1) - only v selected
    /// - Index 2: (1, 0) - only u selected
    /// - Index 3: (1, 1) - both endpoints selected
    fn edge_constraint_spec() -> [bool; 4];

    /// Check if an edge is satisfied given endpoint selection states.
    ///
    /// Default implementation uses `edge_constraint_spec()`.
    fn is_edge_satisfied(u_selected: bool, v_selected: bool) -> bool {
        let spec = Self::edge_constraint_spec();
        let index = (u_selected as usize) * 2 + (v_selected as usize);
        spec[index]
    }

    /// Get the problem info for this constraint type.
    fn problem_info() -> ProblemInfo {
        ProblemInfo::new(Self::NAME, Self::DESCRIPTION)
            .with_aliases(Self::ALIASES)
            .with_complexity(ComplexityClass::NpComplete)
    }

    /// Get the problem category.
    fn category() -> ProblemCategory {
        ProblemCategory::Graph(Self::SUBCATEGORY)
    }
}

/// A generic graph problem parameterized by constraint type and weight type.
///
/// This struct provides a standard implementation for binary graph problems where:
/// - Each vertex can be either selected (1) or not selected (0)
/// - Edges impose constraints on which pairs of vertices can be simultaneously selected
/// - The objective is to maximize or minimize the total weight of selected vertices
///
/// # Type Parameters
///
/// - `C`: The constraint type implementing `GraphConstraint`
/// - `W`: The weight type (default: `i32`)
///
/// # Example
///
/// ```rust,ignore
/// // Define Independent Set as a type alias
/// pub type IndependentSet<W = i32> = GraphProblem<IndependentSetConstraint, W>;
///
/// // Create an instance
/// let problem = IndependentSet::new(4, vec![(0, 1), (1, 2), (2, 3)]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphProblem<C: GraphConstraint, W = i32> {
    /// The underlying graph structure.
    graph: UnGraph<(), ()>,
    /// Weights for each vertex.
    weights: Vec<W>,
    /// Phantom data to track the constraint type.
    #[serde(skip)]
    _constraint: PhantomData<C>,
}

impl<C: GraphConstraint, W: Clone + Default> GraphProblem<C, W> {
    /// Create a new graph problem with unit weights.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices in the graph
    /// * `edges` - List of edges as (u, v) pairs
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let mut graph = UnGraph::new_undirected();
        for _ in 0..num_vertices {
            graph.add_node(());
        }
        for (u, v) in edges {
            graph.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        let weights = vec![W::from(1); num_vertices];
        Self {
            graph,
            weights,
            _constraint: PhantomData,
        }
    }

    /// Create a new graph problem with custom weights.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices in the graph
    /// * `edges` - List of edges as (u, v) pairs
    /// * `weights` - Weight for each vertex
    ///
    /// # Panics
    /// Panics if `weights.len() != num_vertices`.
    pub fn with_weights(num_vertices: usize, edges: Vec<(usize, usize)>, weights: Vec<W>) -> Self {
        assert_eq!(
            weights.len(),
            num_vertices,
            "weights length must match num_vertices"
        );
        let mut graph = UnGraph::new_undirected();
        for _ in 0..num_vertices {
            graph.add_node(());
        }
        for (u, v) in edges {
            graph.add_edge(NodeIndex::new(u), NodeIndex::new(v), ());
        }
        Self {
            graph,
            weights,
            _constraint: PhantomData,
        }
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.edge_count()
    }

    /// Get the edges as a list of (u, v) pairs.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph
            .edge_references()
            .map(|e| (e.source().index(), e.target().index()))
            .collect()
    }

    /// Check if two vertices are adjacent.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.graph
            .find_edge(NodeIndex::new(u), NodeIndex::new(v))
            .is_some()
    }

    /// Check if a configuration satisfies all edge constraints.
    fn is_valid_config(&self, config: &[usize]) -> bool {
        for edge in self.graph.edge_references() {
            let u = edge.source().index();
            let v = edge.target().index();
            let u_selected = config.get(u).copied().unwrap_or(0) == 1;
            let v_selected = config.get(v).copied().unwrap_or(0) == 1;
            if !C::is_edge_satisfied(u_selected, v_selected) {
                return false;
            }
        }
        true
    }
}

impl<C, W> Problem for GraphProblem<C, W>
where
    C: GraphConstraint,
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    type Size = W;

    fn num_variables(&self) -> usize {
        self.graph.node_count()
    }

    fn num_flavors(&self) -> usize {
        2 // Binary: 0 = not selected, 1 = selected
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_vertices", self.graph.node_count()),
            ("num_edges", self.graph.edge_count()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        C::ENERGY_MODE
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let is_valid = self.is_valid_config(config);
        let mut total = W::zero();
        for (i, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.weights[i].clone();
            }
        }
        SolutionSize::new(total, is_valid)
    }
}

impl<C, W> ConstraintSatisfactionProblem for GraphProblem<C, W>
where
    C: GraphConstraint,
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign,
{
    fn constraints(&self) -> Vec<LocalConstraint> {
        let spec = C::edge_constraint_spec();
        self.graph
            .edge_references()
            .map(|e| {
                LocalConstraint::new(
                    2,
                    vec![e.source().index(), e.target().index()],
                    spec.to_vec(),
                )
            })
            .collect()
    }

    fn objectives(&self) -> Vec<LocalSolutionSize<Self::Size>> {
        self.weights
            .iter()
            .enumerate()
            .map(|(i, w)| LocalSolutionSize::new(2, vec![i], vec![W::zero(), w.clone()]))
            .collect()
    }

    fn weights(&self) -> Vec<Self::Size> {
        self.weights.clone()
    }

    fn set_weights(&mut self, weights: Vec<Self::Size>) {
        assert_eq!(weights.len(), self.num_variables());
        self.weights = weights;
    }

    fn is_weighted(&self) -> bool {
        if self.weights.is_empty() {
            return false;
        }
        let first = &self.weights[0];
        !self.weights.iter().all(|w| w == first)
    }
}

impl<C, W> ProblemMetadata for GraphProblem<C, W>
where
    C: GraphConstraint,
    W: Clone + Default,
{
    fn problem_info() -> ProblemInfo {
        C::problem_info()
    }

    fn category() -> ProblemCategory {
        C::category()
    }
}

// ============================================================================
// Built-in constraint types for common graph problems
// ============================================================================

/// Constraint for the Independent Set problem.
///
/// At most one endpoint of each edge can be selected.
/// Objective: Maximize total weight.
#[derive(Debug, Clone, Copy)]
pub struct IndependentSetConstraint;

impl GraphConstraint for IndependentSetConstraint {
    const NAME: &'static str = "Independent Set";
    const DESCRIPTION: &'static str = "Find a maximum weight set of non-adjacent vertices";
    const ENERGY_MODE: EnergyMode = EnergyMode::LargerSizeIsBetter;
    const SUBCATEGORY: GraphSubcategory = GraphSubcategory::Independent;
    const ALIASES: &'static [&'static str] = &["MIS", "MWIS", "Stable Set"];
    const REDUCES_FROM: Option<&'static str> = Some("3-SAT");

    fn edge_constraint_spec() -> [bool; 4] {
        // (0,0), (0,1), (1,0) OK; (1,1) invalid
        [true, true, true, false]
    }
}

/// Constraint for the Vertex Cover problem.
///
/// At least one endpoint of each edge must be selected.
/// Objective: Minimize total weight.
#[derive(Debug, Clone, Copy)]
pub struct VertexCoverConstraint;

impl GraphConstraint for VertexCoverConstraint {
    const NAME: &'static str = "Vertex Cover";
    const DESCRIPTION: &'static str = "Find a minimum weight set of vertices covering all edges";
    const ENERGY_MODE: EnergyMode = EnergyMode::SmallerSizeIsBetter;
    const SUBCATEGORY: GraphSubcategory = GraphSubcategory::Covering;
    const ALIASES: &'static [&'static str] = &["VC", "Minimum Vertex Cover"];
    const REDUCES_FROM: Option<&'static str> = Some("Independent Set");

    fn edge_constraint_spec() -> [bool; 4] {
        // (0,0) invalid; (0,1), (1,0), (1,1) OK
        [false, true, true, true]
    }
}

/// Constraint for the Clique problem.
///
/// All pairs of selected vertices must be adjacent.
/// This is the complement of Independent Set on the complement graph.
/// Objective: Maximize size.
#[derive(Debug, Clone, Copy)]
pub struct CliqueConstraint;

impl GraphConstraint for CliqueConstraint {
    const NAME: &'static str = "Clique";
    const DESCRIPTION: &'static str = "Find a maximum clique (complete subgraph)";
    const ENERGY_MODE: EnergyMode = EnergyMode::LargerSizeIsBetter;
    const SUBCATEGORY: GraphSubcategory = GraphSubcategory::Independent;
    const ALIASES: &'static [&'static str] = &["Maximum Clique", "Max Clique"];
    const REDUCES_FROM: Option<&'static str> = Some("3-SAT");

    fn edge_constraint_spec() -> [bool; 4] {
        // For non-edges: if both are selected, invalid
        // Note: This constraint is applied to NON-EDGES in complement graph,
        // which means we use this on the original graph's edges to find cliques.
        // Actually for Clique, we should use the complement graph.
        // For now, this spec means: on non-edges, both selected is invalid.
        [true, true, true, false]
    }
}

// ============================================================================
// Type aliases for convenient usage
// ============================================================================

/// Independent Set problem using the generic template.
///
/// Find a maximum weight set of vertices where no two are adjacent.
pub type IndependentSetT<W = i32> = GraphProblem<IndependentSetConstraint, W>;

/// Vertex Cover problem using the generic template.
///
/// Find a minimum weight set of vertices that covers all edges.
pub type VertexCoverT<W = i32> = GraphProblem<VertexCoverConstraint, W>;

/// Clique problem using the generic template.
///
/// Note: For finding cliques, create the complement graph first.
pub type CliqueT<W = i32> = GraphProblem<CliqueConstraint, W>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_independent_set_template() {
        let problem = IndependentSetT::<i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);
        assert_eq!(problem.num_vertices(), 3);
        assert_eq!(problem.num_edges(), 3);
        assert!(problem.energy_mode().is_maximization());

        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        // Maximum IS in triangle is size 1
        assert_eq!(solutions.len(), 3);
        for sol in &solutions {
            assert_eq!(sol.iter().sum::<usize>(), 1);
        }
    }

    #[test]
    fn test_vertex_cover_template() {
        let problem = VertexCoverT::<i32>::new(3, vec![(0, 1), (1, 2)]);
        assert!(problem.energy_mode().is_minimization());

        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        // Minimum VC for path 0-1-2 is {1}
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_weighted_problem() {
        let problem = IndependentSetT::with_weights(3, vec![(0, 1), (1, 2)], vec![1, 100, 1]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        // Should select vertex 1 (weight 100) over 0+2 (weight 2)
        assert_eq!(solutions[0], vec![0, 1, 0]);
    }

    #[test]
    fn test_problem_metadata() {
        let info = IndependentSetT::<i32>::problem_info();
        assert_eq!(info.name, "Independent Set");
        assert!(info.aliases.contains(&"MIS"));

        let cat = IndependentSetT::<i32>::category();
        assert_eq!(cat.path(), "graph/independent");
    }

    #[test]
    fn test_constraint_specs() {
        // Independent Set: at most one selected per edge
        let is_spec = IndependentSetConstraint::edge_constraint_spec();
        assert!(is_spec[0]); // (0,0) OK
        assert!(is_spec[1]); // (0,1) OK
        assert!(is_spec[2]); // (1,0) OK
        assert!(!is_spec[3]); // (1,1) invalid

        // Vertex Cover: at least one selected per edge
        let vc_spec = VertexCoverConstraint::edge_constraint_spec();
        assert!(!vc_spec[0]); // (0,0) invalid
        assert!(vc_spec[1]); // (0,1) OK
        assert!(vc_spec[2]); // (1,0) OK
        assert!(vc_spec[3]); // (1,1) OK
    }

    #[test]
    fn test_csp_interface() {
        let problem = IndependentSetT::<i32>::new(3, vec![(0, 1), (1, 2)]);

        let constraints = problem.constraints();
        assert_eq!(constraints.len(), 2);

        let objectives = problem.objectives();
        assert_eq!(objectives.len(), 3);

        assert!(problem.is_satisfied(&[1, 0, 1]));
        assert!(!problem.is_satisfied(&[1, 1, 0]));
    }

    #[test]
    fn test_edges_and_adjacency() {
        let problem = IndependentSetT::<i32>::new(4, vec![(0, 1), (2, 3)]);
        assert!(problem.has_edge(0, 1));
        assert!(problem.has_edge(1, 0)); // Undirected
        assert!(!problem.has_edge(0, 2));

        let edges = problem.edges();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_empty_graph() {
        let problem = IndependentSetT::<i32>::new(3, vec![]);
        let solver = BruteForce::new();
        let solutions = solver.find_best(&problem);
        // All vertices can be selected
        assert_eq!(solutions[0], vec![1, 1, 1]);
    }

    #[test]
    fn test_set_weights() {
        let mut problem = IndependentSetT::<i32>::new(3, vec![(0, 1)]);
        assert!(!problem.is_weighted());
        problem.set_weights(vec![1, 2, 3]);
        assert!(problem.is_weighted());
        assert_eq!(problem.weights(), vec![1, 2, 3]);
    }
}
