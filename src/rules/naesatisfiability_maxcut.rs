//! Reduction from NAE-Satisfiability to MaxCut.
//!
//! For an NAE-3SAT instance with `n` variables and `m` clauses, create:
//! - a heavy edge `(v_i, v_i')` of weight `M = 2m + 1` for every variable
//! - a unit-weight triangle on the three literal vertices of every clause
//!
//! The heavy variable edges force `v_i` and `v_i'` onto opposite sides in any
//! optimum. Once that is fixed, each clause triangle contributes `2` iff its
//! three literals are not all equal.

use crate::models::formula::NAESatisfiability;
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ReductionNAESATToMaxCut {
    target: MaxCut<SimpleGraph, i32>,
    num_source_variables: usize,
}

impl ReductionResult for ReductionNAESATToMaxCut {
    type Source = NAESatisfiability;
    type Target = MaxCut<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let required_vertices = 2 * self.num_source_variables;
        assert!(
            target_solution.len() >= required_vertices,
            "MaxCut solution has {} vertices but source requires {}",
            target_solution.len(),
            required_vertices,
        );

        (0..self.num_source_variables)
            .map(|var_index| target_solution[2 * var_index])
            .collect()
    }
}

fn variable_gadget_weight(num_clauses: usize) -> i32 {
    i32::try_from(num_clauses)
        .ok()
        .and_then(|m| m.checked_mul(2))
        .and_then(|twice_m| twice_m.checked_add(1))
        .expect("NAESatisfiability -> MaxCut penalty exceeds i32 range")
}

fn literal_vertex(literal: i32, num_vars: usize) -> usize {
    let var = literal.unsigned_abs() as usize;
    assert!(
        (1..=num_vars).contains(&var),
        "NAESatisfiability -> MaxCut literal {literal} is out of range for {num_vars} variables",
    );

    let var_index = var - 1;
    if literal > 0 {
        2 * var_index
    } else {
        2 * var_index + 1
    }
}

fn clause_literal_vertices(
    clause: &crate::models::formula::CNFClause,
    num_vars: usize,
) -> [usize; 3] {
    match clause.literals.as_slice() {
        [a, b, c] => [
            literal_vertex(*a, num_vars),
            literal_vertex(*b, num_vars),
            literal_vertex(*c, num_vars),
        ],
        _ => panic!("NAESatisfiability -> MaxCut requires every clause to have exactly 3 literals"),
    }
}

fn accumulate_edge_weight(
    edge_weights: &mut BTreeMap<(usize, usize), i32>,
    u: usize,
    v: usize,
    delta: i32,
) {
    if u == v {
        // Repeated literals induce self-loops in the multigraph view, but they
        // never contribute to a cut, so we drop them when collapsing to
        // `SimpleGraph`.
        return;
    }

    let edge = if u < v { (u, v) } else { (v, u) };
    let weight = edge_weights.entry(edge).or_insert(0);
    *weight = weight
        .checked_add(delta)
        .expect("NAESatisfiability -> MaxCut edge weight overflow");
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vars",
        num_edges = "num_vars + 3 * num_clauses",
    }
)]
impl ReduceTo<MaxCut<SimpleGraph, i32>> for NAESatisfiability {
    type Result = ReductionNAESATToMaxCut;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vars();
        let penalty = variable_gadget_weight(self.num_clauses());
        let mut edge_weights = BTreeMap::new();

        for var_index in 0..num_vars {
            accumulate_edge_weight(&mut edge_weights, 2 * var_index, 2 * var_index + 1, penalty);
        }

        for clause in self.clauses() {
            let [a, b, c] = clause_literal_vertices(clause, num_vars);
            accumulate_edge_weight(&mut edge_weights, a, b, 1);
            accumulate_edge_weight(&mut edge_weights, b, c, 1);
            accumulate_edge_weight(&mut edge_weights, a, c, 1);
        }

        let (edges, weights): (Vec<_>, Vec<_>) = edge_weights.into_iter().unzip();
        let target = MaxCut::new(SimpleGraph::new(2 * num_vars, edges), weights);

        ReductionNAESATToMaxCut {
            target,
            num_source_variables: num_vars,
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
const ISSUE_EXAMPLE_SOURCE_CONFIG: [usize; 3] = [1, 0, 1];

#[cfg(any(test, feature = "example-db"))]
const ISSUE_EXAMPLE_TARGET_CONFIG: [usize; 6] = [1, 0, 0, 1, 1, 0];

#[cfg(any(test, feature = "example-db"))]
fn issue_example() -> NAESatisfiability {
    use crate::models::formula::CNFClause;

    NAESatisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),
            CNFClause::new(vec![1, 2, -3]),
        ],
    )
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_maxcut",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i32>>(
                issue_example(),
                SolutionPair {
                    source_config: ISSUE_EXAMPLE_SOURCE_CONFIG.to_vec(),
                    target_config: ISSUE_EXAMPLE_TARGET_CONFIG.to_vec(),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_maxcut.rs"]
mod tests;
