//! Reduction from Satisfiability (SAT) to MaximumIndependentSet.
//!
//! The reduction creates one vertex for each literal occurrence in each clause.
//! Edges are added:
//! 1. Between all literals within the same clause (forming a clique per clause)
//! 2. Between complementary literals (x and NOT x) across different clauses
//!
//! A satisfying assignment corresponds to an independent set of size = num_clauses,
//! where we pick exactly one literal from each clause.

use crate::models::graph::MaximumIndependentSet;
use crate::models::satisfiability::Satisfiability;
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// A literal in the SAT problem, representing a variable or its negation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolVar {
    /// The variable name/index (0-indexed).
    pub name: usize,
    /// Whether this literal is negated.
    pub neg: bool,
}

impl BoolVar {
    /// Create a new literal.
    pub fn new(name: usize, neg: bool) -> Self {
        Self { name, neg }
    }

    /// Create a literal from a signed integer (1-indexed, as in DIMACS format).
    /// Positive means the variable, negative means its negation.
    pub fn from_literal(lit: i32) -> Self {
        let name = lit.unsigned_abs() as usize - 1; // Convert to 0-indexed
        let neg = lit < 0;
        Self { name, neg }
    }

    /// Check if this literal is the complement of another.
    pub fn is_complement(&self, other: &BoolVar) -> bool {
        self.name == other.name && self.neg != other.neg
    }
}

/// Result of reducing Satisfiability to MaximumIndependentSet.
///
/// This struct contains:
/// - The target MaximumIndependentSet problem
/// - A mapping from vertex indices to literals
/// - The list of source variable indices
/// - The number of clauses in the original SAT problem
#[derive(Debug, Clone)]
pub struct ReductionSATToIS {
    /// The target MaximumIndependentSet problem.
    target: MaximumIndependentSet<SimpleGraph, i32>,
    /// Mapping from vertex index to the literal it represents.
    literals: Vec<BoolVar>,
    /// The number of variables in the source SAT problem.
    num_source_variables: usize,
    /// The number of clauses in the source SAT problem.
    num_clauses: usize,
}

impl ReductionResult for ReductionSATToIS {
    type Source = Satisfiability;
    type Target = MaximumIndependentSet<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a SAT solution from an MaximumIndependentSet solution.
    ///
    /// For each selected vertex (representing a literal), we set the corresponding
    /// variable to make that literal true. Variables not covered by any selected
    /// literal default to false.
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let mut assignment = vec![0usize; self.num_source_variables];
        let mut covered = vec![false; self.num_source_variables];

        for (vertex_idx, &selected) in target_solution.iter().enumerate() {
            if selected == 1 {
                let literal = &self.literals[vertex_idx];
                // If the literal is positive (neg=false), variable should be true (1)
                // If the literal is negated (neg=true), variable should be false (0)
                assignment[literal.name] = if literal.neg { 0 } else { 1 };
                covered[literal.name] = true;
            }
        }

        // Variables not covered can be assigned any value (we use 0)
        // They are already initialized to 0
        assignment
    }
}

impl ReductionSATToIS {
    /// Get the number of clauses in the source SAT problem.
    pub fn num_clauses(&self) -> usize {
        self.num_clauses
    }

    /// Get a reference to the literals mapping.
    pub fn literals(&self) -> &[BoolVar] {
        &self.literals
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vertices", poly!(num_literals)),
            ("num_edges", poly!(num_literals ^ 2)),
        ])
    }
)]
impl ReduceTo<MaximumIndependentSet<SimpleGraph, i32>> for Satisfiability {
    type Result = ReductionSATToIS;

    fn reduce_to(&self) -> Self::Result {
        let mut literals: Vec<BoolVar> = Vec::new();
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut vertex_count = 0;

        // First pass: add vertices for each literal in each clause
        // and add clique edges within each clause
        for clause in self.clauses() {
            let clause_start = vertex_count;

            // Add vertices for each literal in this clause
            for &lit in &clause.literals {
                literals.push(BoolVar::from_literal(lit));
                vertex_count += 1;
            }

            // Add clique edges within this clause
            for i in clause_start..vertex_count {
                for j in (i + 1)..vertex_count {
                    edges.push((i, j));
                }
            }
        }

        // Second pass: add edges between complementary literals across clauses
        // Since we only add clique edges within clauses in the first pass,
        // complementary literals in different clauses won't already have an edge
        for i in 0..vertex_count {
            for j in (i + 1)..vertex_count {
                if literals[i].is_complement(&literals[j]) {
                    edges.push((i, j));
                }
            }
        }

        let target = MaximumIndependentSet::new(SimpleGraph::new(vertex_count, edges), vec![1i32; vertex_count]);

        ReductionSATToIS {
            target,
            literals,
            num_source_variables: self.num_vars(),
            num_clauses: self.num_clauses(),
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_maximumindependentset.rs"]
mod tests;
