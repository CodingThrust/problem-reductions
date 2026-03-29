//! Reduction from NAESatisfiability to MaxCut.
//!
//! Given a NAE-3-SAT instance with n variables and m clauses (each with exactly
//! 3 literals), we construct a weighted MaxCut instance. Variable gadgets
//! (complementary vertex pairs with high-weight edges) force optimal solutions
//! to encode truth assignments. Clause gadgets (triangles on literal vertices)
//! ensure that NAE-satisfied clauses contribute exactly 2 edges to the cut.
//!
//! Reference: Garey, Johnson & Stockmeyer, "Some simplified NP-complete graph
//! problems," Theoretical Computer Science 1(3), 237–267 (1976).

use crate::models::formula::NAESatisfiability;
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;
use std::collections::BTreeMap;

/// Result of reducing NAESatisfiability to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToMaxCut {
    target: MaxCut<SimpleGraph, i32>,
    /// Number of variables in the source NAE-SAT instance.
    num_vars: usize,
}

impl ReductionResult for ReductionNAESATToMaxCut {
    type Source = NAESatisfiability;
    type Target = MaxCut<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Variable x_i corresponds to vertex 2*i (positive literal vertex).
        // The partition side of v_i directly encodes the truth value of x_i.
        (0..self.num_vars).map(|i| target_solution[2 * i]).collect()
    }
}

/// Map a 1-indexed signed literal to its vertex index.
///
/// Positive literal j → vertex 2*(j-1) (positive literal vertex).
/// Negative literal -j → vertex 2*(j-1)+1 (negative literal vertex).
fn literal_vertex(lit: i32) -> usize {
    let var_idx = (lit.unsigned_abs() as usize) - 1;
    if lit > 0 {
        2 * var_idx
    } else {
        2 * var_idx + 1
    }
}

#[reduction(overhead = {
    num_vertices = "2 * num_vars",
    num_edges = "num_vars + 3 * num_clauses",
})]
impl ReduceTo<MaxCut<SimpleGraph, i32>> for NAESatisfiability {
    type Result = ReductionNAESATToMaxCut;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let m = self.num_clauses();

        for (idx, clause) in self.clauses().iter().enumerate() {
            assert_eq!(
                clause.len(),
                3,
                "Clause {idx} has {} literals; NAE-3-SAT to MaxCut requires exactly 3",
                clause.len()
            );
        }

        // Forcing weight: M > max total clause contribution (2m), so all
        // variable-gadget edges are always cut in an optimal partition.
        let big_m: i32 = 2 * (m as i32) + 1;

        // Accumulate edges with merged weights (parallel edges sum).
        let mut edge_weights: BTreeMap<(usize, usize), i32> = BTreeMap::new();

        // Variable gadget edges: (v_i, v_i') with weight M.
        for i in 0..n {
            let key = (2 * i, 2 * i + 1);
            *edge_weights.entry(key).or_insert(0) += big_m;
        }

        // Clause triangle edges: for each 3-literal clause, add weight-1
        // edges between all pairs of literal vertices.
        for clause in self.clauses() {
            let lits = &clause.literals;
            for a in 0..lits.len() {
                for b in (a + 1)..lits.len() {
                    let u = literal_vertex(lits[a]);
                    let v = literal_vertex(lits[b]);
                    let key = if u < v { (u, v) } else { (v, u) };
                    *edge_weights.entry(key).or_insert(0) += 1;
                }
            }
        }

        let edges: Vec<(usize, usize)> = edge_weights.keys().copied().collect();
        let weights: Vec<i32> = edge_weights.values().copied().collect();

        let target = MaxCut::new(SimpleGraph::new(2 * n, edges), weights);

        ReductionNAESATToMaxCut {
            target,
            num_vars: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_maxcut",
        build: || {
            // NAE-3-SAT: n=3, m=2
            // C1 = (x1, x2, x3), C2 = (¬x1, ¬x2, ¬x3)
            let source = NAESatisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, -2, -3]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i32>>(
                source,
                SolutionPair {
                    source_config: vec![1, 0, 0],
                    target_config: vec![1, 0, 0, 1, 0, 1],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_maxcut.rs"]
mod tests;
