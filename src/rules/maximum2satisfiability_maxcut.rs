//! Reduction from Maximum 2-Satisfiability (MAX-2-SAT) to MaxCut.
//!
//! The reduction uses one reference vertex `s` plus one vertex per Boolean
//! variable. For a partition of the target graph, a variable is interpreted as
//! true exactly when its vertex lies on the same side of the cut as `s`.
//!
//! For each 2-literal clause `(l_1 \/ l_2)`, we add the doubled affine form of
//! its satisfaction indicator:
//! `2 * sat(C) = K_C + w(s,a) cut(s,a) + w(s,b) cut(s,b) + w(a,b) cut(a,b)`.
//! Summing over clauses yields
//! `2 * satisfied(phi, x) = C_0 + cut_value(G_phi, partition)`, so every
//! optimal cut extracts to an optimal MAX-2-SAT assignment.

use crate::models::formula::Maximum2Satisfiability;
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use std::collections::BTreeMap;

/// Result of reducing Maximum2Satisfiability to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionMaximum2SatisfiabilityToMaxCut {
    target: MaxCut<SimpleGraph, i32>,
    source_num_vars: usize,
}

impl ReductionResult for ReductionMaximum2SatisfiabilityToMaxCut {
    type Source = Maximum2Satisfiability;
    type Target = MaxCut<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let reference_side = target_solution[0];
        (0..self.source_num_vars)
            .map(|i| usize::from(target_solution[i + 1] == reference_side))
            .collect()
    }
}

fn add_edge_weight(weights: &mut BTreeMap<(usize, usize), i32>, u: usize, v: usize, delta: i32) {
    let edge = if u < v { (u, v) } else { (v, u) };
    *weights.entry(edge).or_insert(0) += delta;
}

fn literal_polarity(lit: i32) -> i32 {
    if lit > 0 {
        1
    } else {
        -1
    }
}

#[reduction(
    overhead = {
        num_vertices = "num_vars + 1",
        num_edges = "num_vars + num_clauses",
    }
)]
impl ReduceTo<MaxCut<SimpleGraph, i32>> for Maximum2Satisfiability {
    type Result = ReductionMaximum2SatisfiabilityToMaxCut;

    fn reduce_to(&self) -> Self::Result {
        let mut accumulated = BTreeMap::new();

        for clause in self.clauses() {
            let literals = &clause.literals;
            let (lit_a, lit_b) = (literals[0], literals[1]);
            let var_a = lit_a.unsigned_abs() as usize;
            let var_b = lit_b.unsigned_abs() as usize;
            let sigma_a = literal_polarity(lit_a);
            let sigma_b = literal_polarity(lit_b);

            add_edge_weight(&mut accumulated, 0, var_a, -sigma_a);
            add_edge_weight(&mut accumulated, 0, var_b, -sigma_b);
            if var_a != var_b {
                add_edge_weight(&mut accumulated, var_a, var_b, sigma_a * sigma_b);
            }
        }

        let (edges, weights): (Vec<_>, Vec<_>) = accumulated
            .into_iter()
            .filter(|(_, weight)| *weight != 0)
            .unzip();

        let target = MaxCut::new(SimpleGraph::new(self.num_vars() + 1, edges), weights);

        ReductionMaximum2SatisfiabilityToMaxCut {
            target,
            source_num_vars: self.num_vars(),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "maximum2satisfiability_to_maxcut",
        build: || {
            let source = Maximum2Satisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2]),
                    CNFClause::new(vec![-1, 3]),
                    CNFClause::new(vec![2, -3]),
                    CNFClause::new(vec![-1, -2]),
                    CNFClause::new(vec![1, 3]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i32>>(
                source,
                SolutionPair {
                    // x1=F, x2=T, x3=T satisfies all five clauses.
                    source_config: vec![0, 1, 1],
                    // Vertex 0 is the reference vertex s. Variables are true
                    // exactly when they share s's side of the cut.
                    target_config: vec![0, 1, 0, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/maximum2satisfiability_maxcut.rs"]
mod tests;
