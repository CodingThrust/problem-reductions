//! Reduction from Satisfiability (SAT) to 3-Coloring.
//!
//! The reduction works as follows:
//! 1. Create 3 special vertices: TRUE, FALSE, AUX (connected as a triangle)
//! 2. For each variable x, create vertices for x and NOT_x:
//!    - Connect x to AUX, NOT_x to AUX (they can't be auxiliary color)
//!    - Connect x to NOT_x (they must have different colors)
//! 3. For each clause, build an OR-gadget that forces output to be TRUE color
//!    - The OR-gadget is built recursively for multi-literal clauses

use crate::models::graph::KColoring;
use crate::models::satisfiability::Satisfiability;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::sat_maximumindependentset::BoolVar;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use num_traits::{Bounded, Num, Zero};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::AddAssign;

/// Helper struct for constructing the graph for the SAT to 3-Coloring reduction.
struct SATColoringConstructor {
    /// The edges of the graph being constructed.
    edges: Vec<(usize, usize)>,
    /// Current number of vertices.
    num_vertices: usize,
    /// Mapping from positive variable index (0-indexed) to vertex index.
    pos_vertices: Vec<usize>,
    /// Mapping from negative variable index (0-indexed) to vertex index.
    neg_vertices: Vec<usize>,
    /// Mapping from BoolVar to vertex index.
    vmap: HashMap<(usize, bool), usize>,
}

impl SATColoringConstructor {
    /// Create a new SATColoringConstructor for `num_vars` variables.
    ///
    /// Initial graph structure:
    /// - Vertices 0, 1, 2: TRUE, FALSE, AUX (forming a triangle)
    /// - For each variable i (0-indexed):
    ///   - Vertex 3 + i: positive literal (x_i)
    ///   - Vertex 3 + num_vars + i: negative literal (NOT x_i)
    fn new(num_vars: usize) -> Self {
        let num_vertices = 2 * num_vars + 3;
        let mut edges = Vec::new();

        // Create triangle: TRUE(0), FALSE(1), AUX(2)
        edges.push((0, 1));
        edges.push((0, 2));
        edges.push((1, 2));

        // Create variable vertices and edges
        let mut pos_vertices = Vec::with_capacity(num_vars);
        let mut neg_vertices = Vec::with_capacity(num_vars);
        let mut vmap = HashMap::new();

        for i in 0..num_vars {
            let pos_v = 3 + i;
            let neg_v = 3 + num_vars + i;
            pos_vertices.push(pos_v);
            neg_vertices.push(neg_v);

            // Connect to AUX (they can't be auxiliary color)
            edges.push((pos_v, 2));
            edges.push((neg_v, 2));

            // Connect pos and neg of the same variable (they must have different colors)
            edges.push((pos_v, neg_v));

            // Build vmap
            vmap.insert((i, false), pos_v); // positive literal
            vmap.insert((i, true), neg_v); // negative literal
        }

        Self {
            edges,
            num_vertices,
            pos_vertices,
            neg_vertices,
            vmap,
        }
    }

    /// Get the TRUE vertex index.
    fn true_vertex(&self) -> usize {
        0
    }

    /// Get the FALSE vertex index.
    fn false_vertex(&self) -> usize {
        1
    }

    /// Get the AUX (ancilla) vertex index.
    fn aux_vertex(&self) -> usize {
        2
    }

    /// Add edge to connect vertex to AUX.
    fn attach_to_aux(&mut self, idx: usize) {
        self.add_edge(idx, self.aux_vertex());
    }

    /// Add edge to connect vertex to FALSE.
    fn attach_to_false(&mut self, idx: usize) {
        self.add_edge(idx, self.false_vertex());
    }

    /// Add edge to connect vertex to TRUE.
    fn attach_to_true(&mut self, idx: usize) {
        self.add_edge(idx, self.true_vertex());
    }

    /// Add an edge between two vertices.
    fn add_edge(&mut self, u: usize, v: usize) {
        self.edges.push((u, v));
    }

    /// Add vertices to the graph.
    fn add_vertices(&mut self, n: usize) -> Vec<usize> {
        let start = self.num_vertices;
        self.num_vertices += n;
        (start..self.num_vertices).collect()
    }

    /// Force a vertex to have the TRUE color.
    /// This is done by connecting it to both AUX and FALSE.
    fn set_true(&mut self, idx: usize) {
        self.attach_to_aux(idx);
        self.attach_to_false(idx);
    }

    /// Get the vertex index for a literal.
    fn get_vertex(&self, var: &BoolVar) -> usize {
        self.vmap[&(var.name, var.neg)]
    }

    /// Add a clause to the graph.
    /// For a single-literal clause, just set the literal to TRUE.
    /// For multi-literal clauses, build OR-gadgets recursively.
    fn add_clause(&mut self, literals: &[i32]) {
        assert!(
            !literals.is_empty(),
            "Clause must have at least one literal"
        );

        let first_var = BoolVar::from_literal(literals[0]);
        let mut output_node = self.get_vertex(&first_var);

        // Build OR-gadget chain for remaining literals
        for &lit in &literals[1..] {
            let var = BoolVar::from_literal(lit);
            let input2 = self.get_vertex(&var);
            output_node = self.add_or_gadget(output_node, input2);
        }

        // Force the output to be TRUE
        self.set_true(output_node);
    }

    /// Add an OR-gadget that computes OR of two inputs.
    ///
    /// The OR-gadget ensures that if any input has TRUE color, the output can have TRUE color.
    /// If both inputs have FALSE color, the output must have FALSE color.
    ///
    /// The gadget adds 5 vertices: ancilla1, ancilla2, entrance1, entrance2, output
    ///
    /// Returns the output vertex index.
    fn add_or_gadget(&mut self, input1: usize, input2: usize) -> usize {
        // Add 5 new vertices
        let new_vertices = self.add_vertices(5);
        let ancilla1 = new_vertices[0];
        let ancilla2 = new_vertices[1];
        let entrance1 = new_vertices[2];
        let entrance2 = new_vertices[3];
        let output = new_vertices[4];

        // Connect output to AUX
        self.attach_to_aux(output);

        // Connect ancilla1 to TRUE
        self.attach_to_true(ancilla1);

        // Build the gadget structure (based on Julia implementation)
        // (ancilla1, ancilla2), (ancilla2, input1), (ancilla2, input2),
        // (entrance1, entrance2), (output, ancilla1), (input1, entrance2),
        // (input2, entrance1), (entrance1, output), (entrance2, output)
        self.add_edge(ancilla1, ancilla2);
        self.add_edge(ancilla2, input1);
        self.add_edge(ancilla2, input2);
        self.add_edge(entrance1, entrance2);
        self.add_edge(output, ancilla1);
        self.add_edge(input1, entrance2);
        self.add_edge(input2, entrance1);
        self.add_edge(entrance1, output);
        self.add_edge(entrance2, output);

        output
    }

    /// Build the final KColoring problem.
    fn build_coloring(&self) -> KColoring<3, SimpleGraph, i32> {
        KColoring::<3, SimpleGraph, i32>::new(self.num_vertices, self.edges.clone())
    }
}

/// Result of reducing Satisfiability to KColoring.
///
/// This struct contains:
/// - The target KColoring problem (3-coloring)
/// - Mappings from variable indices to vertex indices
/// - Information about the source problem
#[derive(Debug, Clone)]
pub struct ReductionSATToColoring<W> {
    /// The target KColoring problem.
    target: KColoring<3, SimpleGraph, i32>,
    /// Mapping from variable index (0-indexed) to positive literal vertex index.
    pos_vertices: Vec<usize>,
    /// Mapping from variable index (0-indexed) to negative literal vertex index.
    neg_vertices: Vec<usize>,
    /// Number of variables in the source SAT problem.
    num_source_variables: usize,
    /// Number of clauses in the source SAT problem.
    num_clauses: usize,
    /// Phantom data to tie this reduction to the source type's weight parameter.
    _phantom: PhantomData<W>,
}

impl<W> ReductionResult for ReductionSATToColoring<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + 'static,
{
    type Source = Satisfiability<W>;
    type Target = KColoring<3, SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a SAT solution from a KColoring solution.
    ///
    /// The coloring solution maps each vertex to a color (0, 1, or 2).
    /// - Color 0: TRUE
    /// - Color 1: FALSE
    /// - Color 2: AUX
    ///
    /// For each variable, we check if its positive literal vertex has TRUE color (0).
    /// If so, the variable is assigned true (1); otherwise false (0).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // First determine which color is TRUE, FALSE, and AUX
        // Vertices 0, 1, 2 are TRUE, FALSE, AUX respectively
        assert!(
            target_solution.len() >= 3,
            "Invalid solution: coloring must have at least 3 vertices"
        );
        let true_color = target_solution[0];
        let false_color = target_solution[1];
        let aux_color = target_solution[2];

        // Sanity checks
        assert!(
            true_color != false_color && true_color != aux_color,
            "Invalid coloring solution: special vertices must have distinct colors"
        );

        let mut assignment = vec![0usize; self.num_source_variables];

        for (i, &pos_vertex) in self.pos_vertices.iter().enumerate() {
            let vertex_color = target_solution[pos_vertex];

            // Sanity check: variable vertices should not have AUX color
            assert!(
                vertex_color != aux_color,
                "Invalid coloring solution: variable vertex has auxiliary color"
            );

            // If positive literal has TRUE color, variable is true (1)
            // Otherwise, variable is false (0)
            assignment[i] = if vertex_color == true_color { 1 } else { 0 };
        }

        assignment
    }
}

impl<W> ReductionSATToColoring<W> {
    /// Get the number of clauses in the source SAT problem.
    pub fn num_clauses(&self) -> usize {
        self.num_clauses
    }

    /// Get the positive vertices mapping.
    pub fn pos_vertices(&self) -> &[usize] {
        &self.pos_vertices
    }

    /// Get the negative vertices mapping.
    pub fn neg_vertices(&self) -> &[usize] {
        &self.neg_vertices
    }
}

#[reduction(
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            // 2*num_vars + 3 (base) + 5*(num_literals - num_clauses) (OR gadgets)
            ("num_vertices", poly!(2 * num_vars) + poly!(5 * num_literals) + poly!(num_clauses).scale(-5.0) + poly!(3)),
            ("num_colors", poly!(3)),
        ])
    }
)]
impl<W> ReduceTo<KColoring<3, SimpleGraph, i32>> for Satisfiability<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + Bounded + AddAssign + From<i32> + 'static,
{
    type Result = ReductionSATToColoring<W>;

    fn reduce_to(&self) -> Self::Result {
        let mut constructor = SATColoringConstructor::new(self.num_vars());

        // Add each clause to the graph
        for clause in self.clauses() {
            constructor.add_clause(&clause.literals);
        }

        let target = constructor.build_coloring();

        ReductionSATToColoring {
            target,
            pos_vertices: constructor.pos_vertices,
            neg_vertices: constructor.neg_vertices,
            num_source_variables: self.num_vars(),
            num_clauses: self.num_clauses(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_coloring.rs"]
mod tests;
