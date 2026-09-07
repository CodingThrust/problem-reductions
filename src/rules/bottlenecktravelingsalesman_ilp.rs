//! Bottleneck TSP to ILP using cyclic positions and a selected maximum edge.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::BottleneckTravelingSalesman;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::Graph;

/// A tour is encoded by positions and distinct directed uses of source edges.
/// One selected maximum-weight edge carries the exact objective coefficient.
#[derive(Debug, Clone)]
pub struct ReductionBTSPToILP {
    target: ILP<i64>,
    num_vertices: usize,
    num_edges: usize,
}

impl ReductionResult for ReductionBTSPToILP {
    type Source = BottleneckTravelingSalesman;
    type Target = ILP<i64>;

    fn target_problem(&self) -> &ILP<i64> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.is_valid() {
            return Err(crate::rules::ExtractionError::invalid(
                "target ILP assignment is infeasible",
            ));
        }
        let n = self.num_vertices;
        Ok((0..self.num_edges)
            .map(|edge| {
                (0..2 * n).any(|offset| target_solution[n * n + 2 * n * edge + offset] == 1)
            })
            .collect())
    }
}

impl ReductionBTSPToILP {
    fn dimensions(
        n: usize,
        m: usize,
    ) -> Result<(usize, usize, usize, usize), crate::rules::ReductionError> {
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<BottleneckTravelingSalesman, ILP<i64>>(
                "sizing the cyclic edge-selection formulation",
            )
        };
        let x = n.checked_mul(n).ok_or_else(overflow)?;
        let z = n
            .checked_mul(m)
            .and_then(|v| v.checked_mul(2))
            .ok_or_else(overflow)?;
        let vars = x
            .checked_add(z)
            .and_then(|v| v.checked_add(m))
            .ok_or_else(overflow)?;
        let constraints = vars
            .checked_add(z)
            .and_then(|v| v.checked_add(z))
            .and_then(|v| {
                n.checked_add(m)
                    .and_then(|extra| extra.checked_mul(3))
                    .and_then(|extra| v.checked_add(extra))
            })
            .and_then(|v| v.checked_add(1))
            .ok_or_else(overflow)?;
        <BottleneckTravelingSalesman as ReduceTo<ILP<i64>>>::exact_i64(
            vars,
            "bounding binary constraint accumulation",
        )?;
        Ok((x, z, vars, constraints))
    }
}

#[reduction(
    transform = exact {
        num_vars = "num_vertices^2 + 2 * num_edges * num_vertices + num_edges",
        num_constraints = "num_vertices^2 + 6 * num_edges * num_vertices + 4 * num_edges + 3 * num_vertices + 1",
    },
    unavailable = {
        num_nonzeros = "threshold comparisons depend on the ordering of edge weights",
    }
)]
impl ReduceTo<ILP<i64>> for BottleneckTravelingSalesman {
    type Result = ReductionBTSPToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let edges = self.graph().edges();
        let m = edges.len();
        let weights = self.weights();
        if weights.len() != m {
            return Err(
                crate::rules::ReductionError::invalid_target::<Self, ILP<i64>>(
                    "edge weights must match the source edges",
                ),
            );
        }
        let (num_x, num_z, num_vars, num_constraints) = ReductionBTSPToILP::dimensions(n, m)?;
        let x = |vertex: usize, position: usize| vertex * n + position;
        let z = |edge: usize, position: usize, direction: usize| {
            num_x + 2 * (edge * n + position) + direction
        };
        let q = |edge: usize| num_x + num_z + edge;
        let uses = |edge: usize| {
            (0..n)
                .flat_map(move |p| [(z(edge, p, 0), 1), (z(edge, p, 1), 1)])
                .collect::<Vec<_>>()
        };
        let mut constraints = Vec::with_capacity(num_constraints);
        // ILP<i64> variables are nonnegative. Check binary bounds before sums.
        for variable in 0..num_vars {
            constraints.push(LinearConstraint::le(vec![(variable, 1)], 1));
        }
        for vertex in 0..n {
            constraints.push(LinearConstraint::eq(
                (0..n).map(|p| (x(vertex, p), 1)).collect(),
                1,
            ));
        }
        for p in 0..n {
            constraints.push(LinearConstraint::eq(
                (0..n).map(|vertex| (x(vertex, p), 1)).collect(),
                1,
            ));
        }
        // Choose one actual edge at each cyclic step. Parallel edges remain
        // independent choices, and the two orientations of a loop are choices.
        for (edge, &(u, v)) in edges.iter().enumerate() {
            for p in 0..n {
                for (direction, a, b) in [(0, u, v), (1, v, u)] {
                    constraints.push(LinearConstraint::le(
                        vec![(z(edge, p, direction), 1), (x(a, p), -1)],
                        0,
                    ));
                    constraints.push(LinearConstraint::le(
                        vec![(z(edge, p, direction), 1), (x(b, (p + 1) % n), -1)],
                        0,
                    ));
                }
            }
        }
        for p in 0..n {
            constraints.push(LinearConstraint::eq(
                (0..m)
                    .flat_map(|edge| [(z(edge, p, 0), 1), (z(edge, p, 1), 1)])
                    .collect(),
                1,
            ));
        }
        for edge in 0..m {
            constraints.push(LinearConstraint::le(uses(edge), 1));
        }
        // The selector is a used edge whose weight dominates every used edge.
        // We compare weights without subtraction or negation, including MIN.
        constraints.push(LinearConstraint::eq(
            (0..m).map(|edge| (q(edge), 1)).collect(),
            1,
        ));
        for edge in 0..m {
            let mut threshold = uses(edge);
            threshold.extend(
                (0..m)
                    .filter(|&other| weights[other] >= weights[edge])
                    .map(|other| (q(other), -1)),
            );
            constraints.push(LinearConstraint::le(threshold, 0));
            let mut selected = vec![(q(edge), 1)];
            selected.extend(uses(edge).into_iter().map(|(var, _)| (var, -1)));
            constraints.push(LinearConstraint::le(selected, 0));
        }
        let objective = weights
            .into_iter()
            .enumerate()
            .map(|(edge, weight)| (q(edge), weight))
            .collect();
        let target = ILP::new(num_vars, constraints, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(ReductionBTSPToILP {
            target,
            num_vertices: n,
            num_edges: m,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "bottlenecktravelingsalesman_to_ilp",
        build: || {
            // C4 with varying weights
            let source = BottleneckTravelingSalesman::new(
                crate::topology::SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3), (3, 0)]),
                vec![1, 2, 3, 4],
            );
            crate::example_db::specs::rule_example_via_ilp::<_, i64>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/bottlenecktravelingsalesman_ilp.rs"]
mod tests;
