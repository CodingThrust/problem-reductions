//! Reduction from NAESatisfiability to MaxCut.
//!
//! Classical reduction from Not-All-Equal SAT to Maximum Cut (Karp 1972,
//! Garey & Johnson ND16). For each variable x_i, create two vertices
//! (positive literal 2i, negative literal 2i+1) connected by a heavy
//! "variable edge" of weight M = m+1 where m is the number of clauses.
//! For each clause, add weight-1 edges between all pairs of literal vertices
//! in that clause. An optimal max-cut of value n*M + sum(k_j - 1) exists
//! if and only if the NAE-SAT formula is satisfiable.
//!
//! Reference: Garey & Johnson, *Computers and Intractability*, ND16, p.210

use crate::models::formula::NAESatisfiability;
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing NAESatisfiability to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToMaxCut {
    target: MaxCut<SimpleGraph, i32>,
    source_num_vars: usize,
}

impl ReductionResult for ReductionNAESATToMaxCut {
    type Source = NAESatisfiability;
    type Target = MaxCut<SimpleGraph, i32>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a NAE-SAT assignment from a MaxCut partition.
    ///
    /// Variable x_i is assigned based on vertex 2*i: if it is in set 0
    /// (config[2*i] == 0), set x_i = false (config value 0); if in set 1,
    /// set x_i = true (config value 1).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.source_num_vars)
            .map(|i| target_solution[2 * i])
            .collect()
    }
}

/// Map a literal to its vertex index.
///
/// Positive literal l (l > 0): vertex 2*(l-1)
/// Negative literal l (l < 0): vertex 2*((-l)-1) + 1
fn literal_vertex(lit: i32) -> usize {
    let var_idx = lit.unsigned_abs() as usize - 1;
    if lit > 0 {
        2 * var_idx
    } else {
        2 * var_idx + 1
    }
}

#[reduction(
    overhead = {
        num_vertices = "2 * num_vars",
        num_edges = "num_vars + num_literal_pairs",
    }
)]
impl ReduceTo<MaxCut<SimpleGraph, i32>> for NAESatisfiability {
    type Result = ReductionNAESATToMaxCut;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vars();
        let m = self.num_clauses();
        let total_vertices = 2 * n;
        let big_m = (m + 1) as i32;

        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut weights: Vec<i32> = Vec::new();

        // Step 1: Variable edges — connect (2*i, 2*i+1) with weight M = m+1
        for i in 0..n {
            edges.push((2 * i, 2 * i + 1));
            weights.push(big_m);
        }

        // Step 2: Clause edges — for each clause, add weight-1 edges between
        // all pairs of literal vertices
        for clause in self.clauses() {
            let lits = &clause.literals;
            for a in 0..lits.len() {
                for b in (a + 1)..lits.len() {
                    edges.push((literal_vertex(lits[a]), literal_vertex(lits[b])));
                    weights.push(1);
                }
            }
        }

        let graph = SimpleGraph::new(total_vertices, edges);
        let target = MaxCut::new(graph, weights);

        ReductionNAESATToMaxCut {
            target,
            source_num_vars: n,
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
            // 3 variables, 2 clauses:
            //   C1 = (x1, x2, ~x3)
            //   C2 = (~x1, x3, x2)
            // NAE-satisfying: x1=T, x2=F, x3=T
            let source = NAESatisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, -3]),
                    CNFClause::new(vec![-1, 3, 2]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i32>>(
                source,
                SolutionPair {
                    // x1=T(1), x2=F(0), x3=T(1)
                    source_config: vec![1, 0, 1],
                    // Vertices: x1(0)=1, ~x1(1)=0, x2(2)=0, ~x2(3)=1, x3(4)=1, ~x3(5)=0
                    // All variable edges cross (weight M=3 each) -> 3*3=9
                    // C1=(x1,x2,~x3): vertices 0,2,5 -> sides {1},{0,0} -> edges (0,2) crosses, (0,5) crosses, (2,5) doesn't -> +2
                    // C2=(~x1,x3,x2): vertices 1,4,2 -> sides {0},{1,0} -> edges (1,4) crosses, (1,2) doesn't, (4,2) crosses -> +2
                    // Total = 9 + 2 + 2 = 13
                    target_config: vec![1, 0, 0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_maxcut.rs"]
mod tests;
