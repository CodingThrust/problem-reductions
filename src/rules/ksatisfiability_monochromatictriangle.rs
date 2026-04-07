//! Reduction from KSatisfiability (3-SAT) to MonochromaticTriangle.
//!
//! For each variable, create positive/negative literal vertices joined by a
//! negation edge. Each clause adds three fresh intermediates that form a clause
//! triangle, plus six fan edges from the clause literals to those intermediates.
//! The resulting graph has a triangle-free 2-edge-coloring iff the source
//! formula is satisfiable.

use crate::models::formula::KSatisfiability;
use crate::models::graph::MonochromaticTriangle;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::variant::K3;
use std::collections::HashMap;

fn normalized_edge(u: usize, v: usize) -> (usize, usize) {
    if u < v {
        (u, v)
    } else {
        (v, u)
    }
}

fn literal_vertex(num_vars: usize, literal: i32) -> usize {
    if literal > 0 {
        literal as usize - 1
    } else {
        num_vars + literal.unsigned_abs() as usize - 1
    }
}

/// Result of reducing KSatisfiability<K3> to MonochromaticTriangle.
#[derive(Debug, Clone)]
pub struct Reduction3SATToMonochromaticTriangle {
    target: MonochromaticTriangle<SimpleGraph>,
    source: KSatisfiability<K3>,
    negation_edge_indices: Vec<usize>,
}

impl ReductionResult for Reduction3SATToMonochromaticTriangle {
    type Source = KSatisfiability<K3>;
    type Target = MonochromaticTriangle<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let direct: Vec<usize> = self
            .negation_edge_indices
            .iter()
            .map(
                |&edge_idx| match target_solution.get(edge_idx).copied().unwrap_or(1) {
                    0 => 1,
                    _ => 0,
                },
            )
            .collect();
        if self.source.evaluate(&direct).0 {
            return direct;
        }

        let complement: Vec<usize> = direct.iter().map(|&value| 1 - value).collect();
        if self.source.evaluate(&complement).0 {
            return complement;
        }

        direct
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vars + 3 * num_clauses",
        num_edges = "num_vars + 9 * num_clauses",
    }
)]
impl ReduceTo<MonochromaticTriangle<SimpleGraph>> for KSatisfiability<K3> {
    type Result = Reduction3SATToMonochromaticTriangle;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vars();
        let num_clauses = self.num_clauses();
        let mut edges = Vec::with_capacity(num_vars + 9 * num_clauses);

        for var in 0..num_vars {
            edges.push((var, num_vars + var));
        }

        for (clause_idx, clause) in self.clauses().iter().enumerate() {
            let clause_base = 2 * num_vars + 3 * clause_idx;
            let m12 = clause_base;
            let m13 = clause_base + 1;
            let m23 = clause_base + 2;
            let literal_vertices: Vec<usize> = clause
                .literals
                .iter()
                .map(|&literal| literal_vertex(num_vars, literal))
                .collect();
            let v1 = literal_vertices[0];
            let v2 = literal_vertices[1];
            let v3 = literal_vertices[2];

            edges.extend_from_slice(&[
                (v1, m12),
                (v2, m12),
                (v1, m13),
                (v3, m13),
                (v2, m23),
                (v3, m23),
                (m12, m13),
                (m12, m23),
                (m13, m23),
            ]);
        }

        let target =
            MonochromaticTriangle::new(SimpleGraph::new(2 * num_vars + 3 * num_clauses, edges));
        let edge_indices: HashMap<(usize, usize), usize> = target
            .edge_list()
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, (u, v))| (normalized_edge(u, v), idx))
            .collect();
        let negation_edge_indices = (0..num_vars)
            .map(|var| edge_indices[&normalized_edge(var, num_vars + var)])
            .collect();

        Reduction3SATToMonochromaticTriangle {
            target,
            source: self.clone(),
            negation_edge_indices,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;
    use crate::solvers::BruteForce;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_monochromatictriangle",
        build: || {
            let source = KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);
            let reduction =
                <KSatisfiability<K3> as ReduceTo<MonochromaticTriangle<SimpleGraph>>>::reduce_to(
                    &source,
                );
            let target_config = BruteForce::new()
                .find_witness(reduction.target_problem())
                .expect("canonical MonochromaticTriangle example must be feasible");
            let source_config = reduction.extract_solution(&target_config);
            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_monochromatictriangle.rs"]
mod tests;
