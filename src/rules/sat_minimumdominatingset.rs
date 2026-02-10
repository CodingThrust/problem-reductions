//! Reduction from Satisfiability (SAT) to MinimumDominatingSet.
//!
//! The reduction follows this construction:
//! 1. For each variable x_i, create a "variable gadget" with 3 vertices:
//!    - Vertex for positive literal x_i
//!    - Vertex for negative literal NOT x_i
//!    - A dummy vertex
//!      These 3 vertices form a complete triangle (clique).
//! 2. For each clause C_j, create a clause vertex.
//! 3. Connect each clause vertex to the literal vertices that appear in that clause.
//!
//! A dominating set of size = num_variables corresponds to a satisfying assignment:
//! - Selecting the positive literal vertex means the variable is true
//! - Selecting the negative literal vertex means the variable is false
//! - Selecting the dummy vertex means the variable can be either (unused in any clause)

use crate::models::graph::MinimumDominatingSet;
use crate::models::satisfiability::Satisfiability;
use crate::topology::SimpleGraph;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::sat_maximumindependentset::BoolVar;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing Satisfiability to MinimumDominatingSet.
///
/// This struct contains:
/// - The target MinimumDominatingSet problem
/// - The number of literals (variables) in the source SAT problem
/// - The number of clauses in the source SAT problem
#[derive(Debug, Clone)]
pub struct ReductionSATToDS<W> {
    /// The target MinimumDominatingSet problem.
    target: MinimumDominatingSet<SimpleGraph, W>,
    /// The number of variables in the source SAT problem.
    num_literals: usize,
    /// The number of clauses in the source SAT problem.
    num_clauses: usize,
    /// Size of the source problem.
    source_size: ProblemSize,
}

impl<W> ReductionResult for ReductionSATToDS<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + 'static,
{
    type Source = Satisfiability<W>;
    type Target = MinimumDominatingSet<SimpleGraph, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a SAT solution from a MinimumDominatingSet solution.
    ///
    /// The dominating set solution encodes variable assignments:
    /// - For each variable x_i (0-indexed), vertices are at positions 3*i, 3*i+1, 3*i+2
    ///   - 3*i: positive literal x_i (selecting means x_i = true)
    ///   - 3*i+1: negative literal NOT x_i (selecting means x_i = false)
    ///   - 3*i+2: dummy vertex (selecting means x_i can be either)
    ///
    /// If more than num_literals vertices are selected, the solution is invalid
    /// and we return a default assignment.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let selected_count: usize = target_solution.iter().sum();

        // If more vertices selected than variables, not a minimal dominating set
        // corresponding to a satisfying assignment
        if selected_count > self.num_literals {
            // Return default assignment (all false)
            return vec![0; self.num_literals];
        }

        let mut assignment = vec![0usize; self.num_literals];

        for (i, &value) in target_solution.iter().enumerate() {
            if value == 1 {
                // Only consider variable gadget vertices (first 3*num_literals vertices)
                if i >= 3 * self.num_literals {
                    continue; // Skip clause vertices
                }

                let var_index = i / 3;
                let vertex_type = i % 3;

                match vertex_type {
                    0 => {
                        // Positive literal selected: x_i = true
                        assignment[var_index] = 1;
                    }
                    1 => {
                        // Negative literal selected: x_i = false
                        assignment[var_index] = 0;
                    }
                    2 => {
                        // Dummy vertex selected: variable is unconstrained
                        // Default to false (already 0), but could be anything
                    }
                    _ => unreachable!(),
                }
            }
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

impl<W> ReductionSATToDS<W> {
    /// Get the number of literals (variables) in the source SAT problem.
    pub fn num_literals(&self) -> usize {
        self.num_literals
    }

    /// Get the number of clauses in the source SAT problem.
    pub fn num_clauses(&self) -> usize {
        self.num_clauses
    }
}

#[reduction(
    target_graph = "SimpleGraph",
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(3 * num_vars) + poly!(num_clauses)),
            ("num_edges", poly!(3 * num_vars) + poly!(num_literals)),
        ])
    }
)]
impl<W> ReduceTo<MinimumDominatingSet<SimpleGraph, W>> for Satisfiability<W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionSATToDS<W>;

    fn reduce_to(&self) -> Self::Result {
        let num_variables = self.num_vars();
        let num_clauses = self.num_clauses();

        // Total vertices: 3 per variable (for variable gadget) + 1 per clause
        let num_vertices = 3 * num_variables + num_clauses;

        let mut edges: Vec<(usize, usize)> = Vec::new();

        // Step 1: Create variable gadgets
        // For each variable i (0-indexed), vertices are at positions:
        //   3*i: positive literal x_i
        //   3*i+1: negative literal NOT x_i
        //   3*i+2: dummy vertex
        // These form a complete triangle (clique of 3)
        for i in 0..num_variables {
            let base = 3 * i;
            // Add all edges of the triangle
            edges.push((base, base + 1));
            edges.push((base, base + 2));
            edges.push((base + 1, base + 2));
        }

        // Step 2: Connect clause vertices to literal vertices
        // Clause j gets vertex at position 3*num_variables + j
        for (j, clause) in self.clauses().iter().enumerate() {
            let clause_vertex = 3 * num_variables + j;

            for &lit in &clause.literals {
                let var = BoolVar::from_literal(lit);
                // var.name is 0-indexed
                // If positive literal, connect to vertex 3*var.name
                // If negative literal, connect to vertex 3*var.name + 1
                let literal_vertex = if var.neg {
                    3 * var.name + 1
                } else {
                    3 * var.name
                };
                edges.push((literal_vertex, clause_vertex));
            }
        }

        let target = MinimumDominatingSet::new(num_vertices, edges);

        ReductionSATToDS {
            target,
            num_literals: num_variables,
            num_clauses,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_minimumdominatingset.rs"]
mod tests;
