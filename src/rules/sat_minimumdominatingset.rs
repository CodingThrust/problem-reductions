//! Reduction from Satisfiability (SAT) to MinimumDominatingSet.
//!
//! The reduction follows this construction:
//! 1. For each occurring variable x_i, create a "variable gadget" with 3 vertices:
//!    - Vertex for positive literal x_i
//!    - Vertex for negative literal NOT x_i
//!    - A dummy vertex
//!      These 3 vertices form a complete triangle (clique).
//! 2. For each clause C_j, create a clause vertex.
//! 3. Connect each clause vertex to the literal vertices that appear in that clause.
//!
//! A dominating set of size = number of occurring variables corresponds to a satisfying assignment:
//! - Selecting the positive literal vertex means the variable is true
//! - Selecting the negative literal vertex means the variable is false
//! - Selecting the dummy vertex means the variable may be assigned either value

use crate::models::formula::Satisfiability;
use crate::models::graph::MinimumDominatingSet;
use crate::reduction;
use crate::rules::sat_maximumindependentset::BoolVar;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::types::{Min, Or};
use std::collections::BTreeMap;

/// Result of reducing Satisfiability to MinimumDominatingSet.
///
/// This struct contains:
/// - The target MinimumDominatingSet problem
/// - The number of literals (variables) in the source SAT problem
/// - The number of clauses in the source SAT problem
#[derive(Debug, Clone)]
pub struct ReductionSATToDS {
    /// The target MinimumDominatingSet problem.
    target: MinimumDominatingSet<SimpleGraph, i64>,
    /// The number of variables in the source SAT problem.
    num_literals: usize,
    /// The number of clauses in the source SAT problem.
    num_clauses: usize,
    /// Original variable indices mapped to dense triangle indices.
    variables: BTreeMap<usize, usize>,
    /// Exact minimum size certifying satisfiability.
    target_size: i64,
}

impl ReductionResult for ReductionSATToDS {
    type Source = Satisfiability;
    type Target = MinimumDominatingSet<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a SAT solution from a MinimumDominatingSet solution.
    ///
    /// Validate a dominating set of exactly one vertex per occurring variable.
    /// Each dense triangle starts with its positive literal; selecting it sets
    /// that original variable true. Negative, dummy and absent variables decode
    /// to false. The finite size certificate proves every clause is dominated
    /// by a selected literal and no clause vertex is selected.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        let certificate = crate::rules::AggregateReductionResult::extract_value(self, value);
        if !certificate.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target dominating set does not certify satisfiability",
            ));
        }

        let mut assignment = vec![false; self.num_literals];
        for (&variable, &gadget) in &self.variables {
            assignment[variable] = target_solution[3 * gadget];
        }

        Ok(assignment)
    }
}

impl crate::rules::AggregateReductionResult for ReductionSATToDS {
    type Source = Satisfiability;
    type Target = MinimumDominatingSet<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, target_value: Min<i64>) -> Or {
        Or(target_value == Min(Some(self.target_size)))
    }
}

impl ReductionSATToDS {
    /// Compute the graph dimensions and exact certificate before allocation.
    fn target_dimensions(
        num_variables: usize,
        num_clauses: usize,
    ) -> Result<(usize, i64), crate::rules::ReductionError> {
        let num_vertices = num_variables
            .checked_mul(3)
            .and_then(|base| base.checked_add(num_clauses))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    Satisfiability,
                    MinimumDominatingSet<SimpleGraph, i64>,
                >("counting dominating-set vertices")
            })?;
        // All vertices may be selected, so every count up to this total must
        // fit the target objective, not only the optimum certificate.
        <Satisfiability as ReduceTo<MinimumDominatingSet<SimpleGraph, i64>>>::exact_i64(
            num_vertices,
            "representing all dominating-set weights",
        )?;
        let target_size =
            <Satisfiability as ReduceTo<MinimumDominatingSet<SimpleGraph, i64>>>::exact_i64(
                num_variables,
                "representing the satisfying dominating-set cardinality",
            )?;
        Ok((num_vertices, target_size))
    }

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
    aggregate = custom,
    transform = upper_bound {
        num_vertices = "3 * num_vars + num_clauses",
        num_edges = "3 * num_vars + num_literals",
    }
)]
impl ReduceTo<MinimumDominatingSet<SimpleGraph, i64>> for Satisfiability {
    type Result = ReductionSATToDS;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        // Absent variables are free in SAT and need no target gadget. Keep
        // original indices for extraction, and assign dense indices in order.
        let mut variables: BTreeMap<usize, usize> = self
            .clauses()
            .iter()
            .flat_map(|clause| &clause.literals)
            .map(|&literal| (BoolVar::from_literal(literal).name, 0))
            .collect();
        for (index, gadget) in variables.values_mut().enumerate() {
            *gadget = index;
        }
        let num_variables = variables.len();
        let num_clauses = self.num_clauses();
        let (num_vertices, target_size) =
            ReductionSATToDS::target_dimensions(num_variables, num_clauses)?;

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
                // The literal's original index is present in the map because
                // the map was collected from these same validated clauses.
                let literal_vertex = 3 * variables[&var.name] + usize::from(var.neg);
                edges.push((literal_vertex, clause_vertex));
            }
        }

        let target = MinimumDominatingSet::new(
            SimpleGraph::new(num_vertices, edges),
            vec![1i64; num_vertices],
        );

        Ok(ReductionSATToDS {
            target,
            num_literals: self.num_vars(),
            num_clauses,
            variables,
            target_size,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_minimumdominatingset",
        build: || {
            let source = Satisfiability::new(
                5,
                vec![
                    CNFClause::new(vec![1, 2, -3]),
                    CNFClause::new(vec![-1, 3, 4]),
                    CNFClause::new(vec![2, -4, 5]),
                    CNFClause::new(vec![-2, 3, -5]),
                    CNFClause::new(vec![1, -3, 5]),
                    CNFClause::new(vec![-1, -2, 4]),
                    CNFClause::new(vec![3, -4, -5]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<
                _,
                MinimumDominatingSet<SimpleGraph, i64>,
            >(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, true, true, true]),
                    target_config: serde_json::json!(vec![
                        true, false, false, false, true, false, true, false, false, true, false,
                        false, true, false, false, false, false, false, false, false, false, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_minimumdominatingset.rs"]
mod tests;
