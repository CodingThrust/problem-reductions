//! Reduction from 3-SAT to Kernel.
//!
//! This is Chvatal's 1973 construction using variable digons and clause
//! 3-cycles with arcs to literal vertices.

use crate::models::formula::KSatisfiability;
use crate::models::graph::Kernel;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::DirectedGraph;
use crate::variant::K3;

/// Result of reducing 3-SAT to Kernel.
#[derive(Debug, Clone)]
pub struct Reduction3SatToKernel {
    target: Kernel,
    source_num_vars: usize,
}

impl ReductionResult for Reduction3SatToKernel {
    type Source = KSatisfiability<K3>;
    type Target = Kernel;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.source_num_vars)
            .map(|i| usize::from(target_solution.get(2 * i).copied().unwrap_or(0) == 1))
            .collect()
    }
}

fn literal_vertex(literal: i32) -> usize {
    let variable = literal.unsigned_abs() as usize - 1;
    if literal > 0 {
        2 * variable
    } else {
        2 * variable + 1
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vars + 3 * num_clauses",
        num_arcs = "2 * num_vars + 6 * num_clauses",
    }
)]
impl ReduceTo<Kernel> for KSatisfiability<K3> {
    type Result = Reduction3SatToKernel;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vars();
        let num_clauses = self.num_clauses();
        let mut arcs = Vec::with_capacity(2 * num_vars + 6 * num_clauses);

        for variable in 0..num_vars {
            let positive = 2 * variable;
            let negative = positive + 1;
            arcs.push((positive, negative));
            arcs.push((negative, positive));
        }

        for (clause_index, clause) in self.clauses().iter().enumerate() {
            let clause_base = 2 * num_vars + 3 * clause_index;
            arcs.push((clause_base, clause_base + 1));
            arcs.push((clause_base + 1, clause_base + 2));
            arcs.push((clause_base + 2, clause_base));

            for (literal_index, &literal) in clause.literals.iter().enumerate() {
                arcs.push((clause_base + literal_index, literal_vertex(literal)));
            }
        }

        Reduction3SatToKernel {
            target: Kernel::new(DirectedGraph::new(2 * num_vars + 3 * num_clauses, arcs)),
            source_num_vars: num_vars,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_kernel",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Kernel>(
                KSatisfiability::<K3>::new(
                    3,
                    vec![
                        CNFClause::new(vec![1, 2, 3]),
                        CNFClause::new(vec![-1, -2, 3]),
                    ],
                ),
                SolutionPair {
                    source_config: vec![1, 1, 1],
                    target_config: vec![1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0],
                },
            )
        },
    }]
}
#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_kernel.rs"]
mod tests;
