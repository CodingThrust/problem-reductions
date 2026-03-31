//! Reduction from MinimumGraphBandwidth to ILP (Integer Linear Programming).
//!
//! Position-assignment formulation with bandwidth variable:
//! - Binary x_{v,p}: vertex v gets position p
//! - Integer position variables pos_v = sum_p p * x_{v,p}
//! - Integer bandwidth variable B
//! - For each edge (u,v): pos_u - pos_v <= B, pos_v - pos_u <= B
//! - Objective: minimize B

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::MinimumGraphBandwidth;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};

/// Result of reducing MinimumGraphBandwidth to ILP.
///
/// Variable layout (ILP<i32>, non-negative integers):
/// - `x_{v,p}` at index `v * n + p`, bounded to {0,1}
/// - `pos_v` at index `n^2 + v`, integer position in {0, ..., n-1}
/// - `B` (bandwidth) at index `n^2 + n`
#[derive(Debug, Clone)]
pub struct ReductionMGBToILP {
    target: ILP<i32>,
    num_vertices: usize,
}

impl ReductionResult for ReductionMGBToILP {
    type Source = MinimumGraphBandwidth<SimpleGraph>;
    type Target = ILP<i32>;

    fn target_problem(&self) -> &ILP<i32> {
        &self.target
    }

    /// Extract: for each vertex v, output its position p (the unique p with x_{v,p} = 1).
    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let n = self.num_vertices;
        (0..n)
            .map(|v| {
                (0..n)
                    .find(|&p| target_solution[v * n + p] == 1)
                    .unwrap_or(0)
            })
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "num_vertices^2 + num_vertices + 1",
        num_constraints = "2 * num_vertices + num_vertices^2 + num_vertices + num_vertices + 1 + 2 * num_edges",
    }
)]
impl ReduceTo<ILP<i32>> for MinimumGraphBandwidth<SimpleGraph> {
    type Result = ReductionMGBToILP;

    fn reduce_to(&self) -> Self::Result {
        let n = self.num_vertices();
        let graph = self.graph();
        let edges = graph.edges();

        let num_x = n * n;
        let num_vars = num_x + n + 1;

        let x_idx = |v: usize, p: usize| -> usize { v * n + p };
        let pos_idx = |v: usize| -> usize { num_x + v };
        let b_idx = num_x + n;

        let mut constraints = Vec::new();

        // Assignment: each vertex in exactly one position
        for v in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|p| (x_idx(v, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Assignment: each position has exactly one vertex
        for p in 0..n {
            let terms: Vec<(usize, f64)> = (0..n).map(|v| (x_idx(v, p), 1.0)).collect();
            constraints.push(LinearConstraint::eq(terms, 1.0));
        }

        // Binary bounds for x variables (ILP<i32>)
        for v in 0..n {
            for p in 0..n {
                constraints.push(LinearConstraint::le(vec![(x_idx(v, p), 1.0)], 1.0));
            }
        }

        // Position variable linking: pos_v = sum_p p * x_{v,p}
        for v in 0..n {
            let mut terms: Vec<(usize, f64)> = vec![(pos_idx(v), 1.0)];
            for p in 0..n {
                terms.push((x_idx(v, p), -(p as f64)));
            }
            constraints.push(LinearConstraint::eq(terms, 0.0));
        }

        // Position bounds: 0 <= pos_v <= n-1
        for v in 0..n {
            constraints.push(LinearConstraint::le(
                vec![(pos_idx(v), 1.0)],
                (n - 1) as f64,
            ));
        }

        // Bandwidth upper bound: B <= n-1 (max possible position difference)
        constraints.push(LinearConstraint::le(vec![(b_idx, 1.0)], (n - 1) as f64));

        // Bandwidth constraints: for each edge (u,v):
        //   pos_u - pos_v <= B  =>  pos_u - pos_v - B <= 0
        //   pos_v - pos_u <= B  =>  pos_v - pos_u - B <= 0
        for &(u, v) in edges.iter() {
            constraints.push(LinearConstraint::le(
                vec![(pos_idx(u), 1.0), (pos_idx(v), -1.0), (b_idx, -1.0)],
                0.0,
            ));
            constraints.push(LinearConstraint::le(
                vec![(pos_idx(v), 1.0), (pos_idx(u), -1.0), (b_idx, -1.0)],
                0.0,
            ));
        }

        // Objective: minimize B
        let objective = vec![(b_idx, 1.0)];
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize);

        ReductionMGBToILP {
            target,
            num_vertices: n,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumgraphbandwidth_to_ilp",
        build: || {
            // Star S4: center 0 connected to 1, 2, 3
            let source =
                MinimumGraphBandwidth::new(SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]));
            crate::example_db::specs::rule_example_via_ilp::<_, i32>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/minimumgraphbandwidth_ilp.rs"]
mod tests;
