//! Reduction from Satisfiability (SAT) to 3-Coloring.
//!
//! The reduction works as follows:
//! 1. Create 3 special vertices: TRUE, FALSE, AUX (connected as a triangle)
//! 2. For each variable x, create vertices for x and NOT_x:
//!    - Connect x to AUX, NOT_x to AUX (they can't be auxiliary color)
//!    - Connect x to NOT_x (they must have different colors)
//! 3. For each clause, build an OR-gadget that forces output to be TRUE color
//!    - The OR-gadget is built recursively for multi-literal clauses

use crate::models::graph::Coloring;
use crate::models::satisfiability::Satisfiability;
use crate::rules::sat_independentset::BoolVar;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
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
        assert!(!literals.is_empty(), "Clause must have at least one literal");

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

    /// Build the final Coloring problem.
    fn build_coloring(&self) -> Coloring {
        Coloring::new(self.num_vertices, 3, self.edges.clone())
    }
}

/// Result of reducing Satisfiability to Coloring.
///
/// This struct contains:
/// - The target Coloring problem (3-coloring)
/// - Mappings from variable indices to vertex indices
/// - Information about the source problem
#[derive(Debug, Clone)]
pub struct ReductionSATToColoring<W> {
    /// The target Coloring problem.
    target: Coloring,
    /// Mapping from variable index (0-indexed) to positive literal vertex index.
    pos_vertices: Vec<usize>,
    /// Mapping from variable index (0-indexed) to negative literal vertex index.
    neg_vertices: Vec<usize>,
    /// Number of variables in the source SAT problem.
    num_source_variables: usize,
    /// Number of clauses in the source SAT problem.
    num_clauses: usize,
    /// Size of the source problem.
    source_size: ProblemSize,
    /// Phantom data to tie this reduction to the source type's weight parameter.
    _phantom: PhantomData<W>,
}

impl<W> ReductionResult for ReductionSATToColoring<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = Satisfiability<W>;
    type Target = Coloring;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a SAT solution from a Coloring solution.
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

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
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

impl<W> ReduceTo<Coloring> for Satisfiability<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
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
            source_size: self.problem_size(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::satisfiability::CNFClause;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_constructor_basic_structure() {
        let constructor = SATColoringConstructor::new(2);

        // Should have 2*2 + 3 = 7 vertices
        assert_eq!(constructor.num_vertices, 7);

        // Check pos_vertices and neg_vertices
        assert_eq!(constructor.pos_vertices, vec![3, 4]);
        assert_eq!(constructor.neg_vertices, vec![5, 6]);

        // Check vmap
        assert_eq!(constructor.vmap[&(0, false)], 3);
        assert_eq!(constructor.vmap[&(0, true)], 5);
        assert_eq!(constructor.vmap[&(1, false)], 4);
        assert_eq!(constructor.vmap[&(1, true)], 6);
    }

    #[test]
    fn test_special_vertex_accessors() {
        let constructor = SATColoringConstructor::new(1);
        assert_eq!(constructor.true_vertex(), 0);
        assert_eq!(constructor.false_vertex(), 1);
        assert_eq!(constructor.aux_vertex(), 2);
    }

    #[test]
    fn test_simple_sat_to_coloring() {
        // Simple SAT: (x1) - one clause with one literal
        let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);
        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        let coloring = reduction.target_problem();

        // Should have 2*1 + 3 = 5 base vertices
        // Plus edges to set x1 to TRUE (attached to AUX and FALSE)
        assert!(coloring.num_vertices() >= 5);
    }

    #[test]
    fn test_reduction_structure() {
        // Satisfiable formula: (x1 OR x2) AND (NOT x1 OR x2)
        // Just verify the reduction builds the correct structure
        let sat = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 2])],
        );

        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        let coloring = reduction.target_problem();

        // Base vertices: 3 (TRUE, FALSE, AUX) + 2*2 (pos and neg for each var) = 7
        // Each 2-literal clause adds 5 vertices for OR gadget = 2 * 5 = 10
        // Total: 7 + 10 = 17 vertices
        assert_eq!(coloring.num_vertices(), 17);
        assert_eq!(coloring.num_colors(), 3);
        assert_eq!(reduction.pos_vertices().len(), 2);
        assert_eq!(reduction.neg_vertices().len(), 2);
    }

    #[test]
    fn test_unsatisfiable_formula() {
        // Unsatisfiable: (x1) AND (NOT x1)
        let sat = Satisfiability::<i32>::new(
            1,
            vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
        );

        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        let coloring = reduction.target_problem();

        // Solve the coloring problem
        let solver = BruteForce::new();
        let solutions = solver.find_best(coloring);

        // For an unsatisfiable formula, the coloring should have no valid solutions
        // OR no valid coloring exists that extracts to a satisfying SAT assignment
        let mut found_satisfying = false;
        for sol in &solutions {
            if coloring.solution_size(sol).is_valid {
                let sat_sol = reduction.extract_solution(sol);
                let assignment: Vec<bool> = sat_sol.iter().map(|&v| v == 1).collect();
                if sat.is_satisfying(&assignment) {
                    found_satisfying = true;
                    break;
                }
            }
        }

        // The coloring should not yield a satisfying SAT assignment
        // because the formula is unsatisfiable
        // Note: The coloring graph itself may still be colorable,
        // but the constraints should make it impossible for both
        // x1 and NOT x1 to be TRUE color simultaneously
        // Actually, let's check if ANY coloring solution produces a valid SAT solution
        // If the formula is unsat, no valid coloring should extract to a satisfying assignment
        assert!(
            !found_satisfying,
            "Unsatisfiable formula should not produce satisfying assignment"
        );
    }

    #[test]
    fn test_three_literal_clause_structure() {
        // (x1 OR x2 OR x3)
        let sat = Satisfiability::<i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        let coloring = reduction.target_problem();

        // Base vertices: 3 + 2*3 = 9
        // 3-literal clause needs 2 OR gadgets (x1 OR x2, then result OR x3)
        // Each OR gadget adds 5 vertices, so 2*5 = 10
        // Total: 9 + 10 = 19 vertices
        assert_eq!(coloring.num_vertices(), 19);
        assert_eq!(coloring.num_colors(), 3);
        assert_eq!(reduction.pos_vertices().len(), 3);
        assert_eq!(reduction.neg_vertices().len(), 3);
    }

    #[test]
    fn test_source_and_target_size() {
        let sat = Satisfiability::<i32>::new(
            3,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
        );
        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vars"), Some(3));
        assert_eq!(source_size.get("num_clauses"), Some(2));
        assert!(target_size.get("num_vertices").is_some());
        assert!(target_size.get("num_colors").unwrap() == 3);
    }

    #[test]
    fn test_extract_solution_basic() {
        // Simple case: one variable, one clause (x1)
        let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);
        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);

        // Manually construct a valid coloring where x1 has TRUE color
        // Vertices: 0=TRUE, 1=FALSE, 2=AUX, 3=x1, 4=NOT_x1
        // Colors: TRUE=0, FALSE=1, AUX=2
        // For x1 to be true, pos_vertex[0]=3 should have color 0 (TRUE)

        // A valid coloring that satisfies x1=TRUE:
        // - Vertex 0 (TRUE): color 0
        // - Vertex 1 (FALSE): color 1
        // - Vertex 2 (AUX): color 2
        // - Vertex 3 (x1): color 0 (TRUE) - connected to AUX(2), NOT_x1(4)
        // - Vertex 4 (NOT_x1): color 1 (FALSE) - connected to AUX(2), x1(3)

        // However, the actual coloring depends on the full graph structure
        // Let's just verify the extraction logic works by checking type signatures
        assert_eq!(reduction.pos_vertices().len(), 1);
        assert_eq!(reduction.neg_vertices().len(), 1);
    }

    #[test]
    fn test_complex_formula_structure() {
        // (x1 OR x2) AND (NOT x1 OR x3) AND (NOT x2 OR NOT x3)
        let sat = Satisfiability::<i32>::new(
            3,
            vec![
                CNFClause::new(vec![1, 2]),     // x1 OR x2
                CNFClause::new(vec![-1, 3]),    // NOT x1 OR x3
                CNFClause::new(vec![-2, -3]),   // NOT x2 OR NOT x3
            ],
        );

        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        let coloring = reduction.target_problem();

        // Base vertices: 3 + 2*3 = 9
        // 3 clauses each with 2 literals, each needs 1 OR gadget = 3*5 = 15
        // Total: 9 + 15 = 24 vertices
        assert_eq!(coloring.num_vertices(), 24);
        assert_eq!(coloring.num_colors(), 3);
        assert_eq!(reduction.num_clauses(), 3);
    }

    #[test]
    fn test_single_literal_clauses() {
        // (x1) AND (x2) - both must be true
        let sat = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1]), CNFClause::new(vec![2])],
        );

        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        let coloring = reduction.target_problem();

        let solver = BruteForce::new();
        let solutions = solver.find_best(coloring);

        let mut found_correct = false;
        for sol in &solutions {
            if coloring.solution_size(sol).is_valid {
                let sat_sol = reduction.extract_solution(sol);
                if sat_sol == vec![1, 1] {
                    found_correct = true;
                    break;
                }
            }
        }

        assert!(
            found_correct,
            "Should find solution where both x1 and x2 are true"
        );
    }

    #[test]
    fn test_empty_sat() {
        // Empty SAT (trivially satisfiable)
        let sat = Satisfiability::<i32>::new(0, vec![]);
        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);

        assert_eq!(reduction.num_clauses(), 0);
        assert!(reduction.pos_vertices().is_empty());
        assert!(reduction.neg_vertices().is_empty());

        let coloring = reduction.target_problem();
        // Just the 3 special vertices
        assert_eq!(coloring.num_vertices(), 3);
    }

    #[test]
    fn test_num_clauses_accessor() {
        let sat = Satisfiability::<i32>::new(
            2,
            vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
        );
        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        assert_eq!(reduction.num_clauses(), 2);
    }

    #[test]
    fn test_or_gadget_construction() {
        // Test that OR gadget is correctly added
        let mut constructor = SATColoringConstructor::new(2);
        let initial_vertices = constructor.num_vertices;

        // Add an OR gadget
        let input1 = constructor.pos_vertices[0]; // x1
        let input2 = constructor.pos_vertices[1]; // x2
        let output = constructor.add_or_gadget(input1, input2);

        // Should add 5 vertices
        assert_eq!(constructor.num_vertices, initial_vertices + 5);

        // Output should be the last added vertex
        assert_eq!(output, constructor.num_vertices - 1);
    }

    #[test]
    fn test_manual_coloring_extraction() {
        // Test solution extraction with a manually constructed coloring solution
        // for a simple 1-variable SAT problem: (x1)
        let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);
        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);
        let coloring = reduction.target_problem();

        // The graph structure for (x1) with set_true:
        // - Vertices 0, 1, 2: TRUE, FALSE, AUX (triangle)
        // - Vertex 3: x1 (pos)
        // - Vertex 4: NOT x1 (neg)
        // After set_true(3): x1 is connected to AUX and FALSE
        // So x1 must have TRUE color

        // A valid 3-coloring where x1 has TRUE color:
        // TRUE=0, FALSE=1, AUX=2
        // x1 must have color 0 (connected to 1 and 2)
        // NOT_x1 must have color 1 (connected to 2 and x1=0)
        let valid_coloring = vec![0, 1, 2, 0, 1];

        assert_eq!(coloring.num_vertices(), 5);
        let extracted = reduction.extract_solution(&valid_coloring);
        // x1 should be true (1) because vertex 3 has color 0 which equals TRUE vertex's color
        assert_eq!(extracted, vec![1]);
    }

    #[test]
    fn test_extraction_with_different_color_assignment() {
        // Test that extraction works with different color assignments
        // (colors may be permuted but semantics preserved)
        let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);
        let reduction = ReduceTo::<Coloring>::reduce_to(&sat);

        // Different valid coloring: TRUE=2, FALSE=0, AUX=1
        // x1 must have color 2 (TRUE), NOT_x1 must have color 0 (FALSE)
        let coloring_permuted = vec![2, 0, 1, 2, 0];
        let extracted = reduction.extract_solution(&coloring_permuted);
        // x1 should still be true because its color equals TRUE vertex's color
        assert_eq!(extracted, vec![1]);

        // Another permutation: TRUE=1, FALSE=2, AUX=0
        // x1 has color 1 (TRUE), NOT_x1 has color 2 (FALSE)
        let coloring_permuted2 = vec![1, 2, 0, 1, 2];
        let extracted2 = reduction.extract_solution(&coloring_permuted2);
        assert_eq!(extracted2, vec![1]);
    }
}
