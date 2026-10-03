//! Bounded partition labels with exact crossing flags and occupied-part budgets.

use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::graph::AcyclicPartition;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionAcyclicPartitionToILP {
    target: ILP<i64, i64, Bounded>,
    n: usize,
    one_hot: bool,
}

impl ReductionResult for ReductionAcyclicPartitionToILP {
    type Source = AcyclicPartition<i64>;
    type Target = ILP<i64, i64, Bounded>;

    fn target_problem(&self) -> &ILP<i64, i64, Bounded> {
        &self.target
    }

    /// Decode one-hot memberships or return the direct binary part labels.
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;

        if self.one_hot {
            crate::rules::ilp_helpers::one_hot_decode_rows(target_solution, self.n, self.n, 0)
        } else {
            crate::rules::ilp_helpers::decode_usize_values(&target_solution[..self.n])
        }
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionAcyclicPartitionToILP {}

// Two separated heavy anchors, with a two-arc path through every other
// vertex, give a lower bound on crossing cost. A third part would cross both
// arcs of one path, rather than at least one, and pay the indicated extra.
fn two_part_anchors(problem: &AcyclicPartition<i64>) -> Option<(usize, usize)> {
    let n = problem.num_vertices();
    if n < 3
        || *problem.weight_bound() < 0
        || problem.vertex_weights().iter().any(|&weight| weight < 0)
        || problem.arc_costs().iter().any(|&cost| cost < 0)
    {
        return None;
    }
    let arcs = problem.graph().arcs();
    let mut incoming = vec![false; n];
    let mut outgoing = vec![false; n];
    for &(u, v) in &arcs {
        if u != v {
            outgoing[u] = true;
            incoming[v] = true;
        }
    }
    for root in (0..n).filter(|&v| !incoming[v]) {
        for sink in (0..n).filter(|&v| v != root && !outgoing[v]) {
            if i128::from(problem.vertex_weights()[root])
                + i128::from(problem.vertex_weights()[sink])
                <= i128::from(*problem.weight_bound())
            {
                continue;
            }
            let mut left = vec![None; n];
            let mut right = vec![None; n];
            let mut baseline = 0_i128;
            for (&(u, v), &cost) in arcs.iter().zip(problem.arc_costs()) {
                let cost = i128::from(cost);
                if u == root && v == sink {
                    baseline += cost;
                } else if u == root && v != root {
                    left[v] = Some(left[v].unwrap_or(0) + cost);
                } else if v == sink && u != sink {
                    right[u] = Some(right[u].unwrap_or(0) + cost);
                }
            }
            let mut extra = i128::MAX;
            for v in (0..n).filter(|&v| v != root && v != sink) {
                if let (Some(a), Some(b)) = (left[v], right[v]) {
                    baseline += a.min(b);
                    extra = extra.min(a.max(b));
                } else {
                    extra = 0;
                    break;
                }
            }
            if extra > 0 && i128::from(*problem.cost_bound()) - baseline < extra {
                return Some((root, sink));
            }
        }
    }
    None
}

// With two certified occupied parts, one binary label per vertex suffices.
// Substitute fixed anchor labels before summing weight and crossing rows.
fn two_part_reduction(
    problem: &AcyclicPartition<i64>,
    root: usize,
    sink: usize,
) -> Result<ReductionAcyclicPartitionToILP, crate::rules::ReductionError> {
    type Source = AcyclicPartition<i64>;
    type Target = ILP<i64, i64, Bounded>;
    let n = problem.num_vertices();
    let exact = |value, operation| {
        i64::try_from(value).map_err(|_| {
            crate::rules::ReductionError::integer_overflow::<Source, Target>(operation)
        })
    };
    let mut variables = vec![IntegerVariable::binary(); n];
    variables[root] = IntegerVariable::new(Some(0), Some(0))
        .map_err(<Source as ReduceTo<Target>>::target_construction)?;
    variables[sink] = IntegerVariable::new(Some(1), Some(1))
        .map_err(<Source as ReduceTo<Target>>::target_construction)?;
    let mut constraints = Vec::new();
    let mut costs = vec![0_i128; n];
    for (&(u, v), &cost) in problem.graph().arcs().iter().zip(problem.arc_costs()) {
        constraints.push(LinearConstraint::le(vec![(u, 1), (v, -1)], 0));
        costs[u] -= i128::from(cost);
        costs[v] += i128::from(cost);
    }
    let interior = |v: &usize| *v != root && *v != sink;
    let weights: Vec<_> = (0..n)
        .filter(interior)
        .map(|v| (v, problem.vertex_weights()[v]))
        .collect();
    let total_weight: i128 = weights.iter().map(|&(_, weight)| i128::from(weight)).sum();
    constraints.push(LinearConstraint::le(
        weights.clone(),
        exact(
            i128::from(*problem.weight_bound()) - i128::from(problem.vertex_weights()[sink]),
            "bounding the sink part weight",
        )?,
    ));
    constraints.push(LinearConstraint::ge(
        weights,
        exact(
            total_weight - i128::from(*problem.weight_bound())
                + i128::from(problem.vertex_weights()[root]),
            "bounding the root part weight",
        )?,
    ));
    let cost_terms = (0..n)
        .filter(interior)
        .map(|v| Ok((v, exact(costs[v], "summing crossing cost coefficients")?)))
        .collect::<Result<_, crate::rules::ReductionError>>()?;
    constraints.push(LinearConstraint::le(
        cost_terms,
        exact(
            i128::from(*problem.cost_bound()) - costs[sink],
            "substituting the sink crossing cost",
        )?,
    ));
    let selected = i128::from(*problem.weight_bound()) - i128::from(problem.vertex_weights()[root]);
    let unselected =
        i128::from(*problem.weight_bound()) - i128::from(problem.vertex_weights()[sink]);
    if selected > 0
        && selected + unselected == total_weight
        && (0..n)
            .filter(interior)
            .all(|v| problem.vertex_weights()[v] <= 1)
    {
        // Exactly k unit-weight vertices belong to the root part. A zero-weight
        // item with two such parents can be selected only with both parents.
        // Thus each selected parent has at most k-1 distinct selected neighbors.
        let mut parents = vec![Vec::new(); n];
        for (u, v) in problem.graph().arcs() {
            if interior(&u) && problem.vertex_weights()[u] == 1 {
                parents[v].push(u);
            }
        }
        let mut pairs = std::collections::BTreeMap::new();
        for v in (0..n).filter(interior) {
            parents[v].sort_unstable();
            parents[v].dedup();
            if problem.vertex_weights()[v] == 0 && parents[v].len() == 2 {
                pairs.entry((parents[v][0], parents[v][1])).or_insert(v);
            }
        }
        let mut incident = vec![Vec::new(); n];
        for ((u, v), item) in pairs {
            incident[u].push(item);
            incident[v].push(item);
        }
        let limit = exact(selected - 1, "bounding distinct selected neighbors")?;
        for (v, neighbors) in incident
            .into_iter()
            .enumerate()
            .filter(|(_, neighbors)| !neighbors.is_empty())
        {
            let degree = <Source as ReduceTo<Target>>::exact_i64(
                neighbors.len(),
                "counting distinct neighbors",
            )?;
            let mut terms: Vec<_> = neighbors.into_iter().map(|item| (item, 1)).collect();
            terms.push((v, -limit));
            constraints.push(LinearConstraint::ge(
                terms,
                exact(
                    i128::from(degree) - i128::from(limit),
                    "bounding selected incidence items",
                )?,
            ));
        }
    }
    let target = Target::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
        .map_err(<Source as ReduceTo<Target>>::target_construction)?;
    Ok(ReductionAcyclicPartitionToILP {
        target,
        n,
        one_hot: false,
    })
}

#[reduction(transform = upper_bound {
    num_vars = "num_vertices^2 + 2 * num_vertices + num_arcs",
    num_constraints = "num_vertices^2 + 4 * num_vertices + 2 * num_arcs + 1",
    max_constraint_magnitude_bits = "max_numeric_magnitude_bits + num_vertices + num_arcs + 1",
    num_nonzeros = "6 * num_vertices^2 + 2 * num_vertices + 7 * num_arcs",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for AcyclicPartition<i64> {
    type Result = ReductionAcyclicPartitionToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let arcs = self.graph().arcs();
        let m = arcs.len();
        if let Some((root, sink)) = two_part_anchors(self) {
            return two_part_reduction(self, root, sink);
        }
        let parts = n;

        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                "counting acyclic partition variables",
            )
        };
        let square = n.checked_mul(parts).ok_or_else(overflow)?;
        let labels = square.checked_add(parts).ok_or_else(overflow)?;
        let crossing = labels.checked_add(n).ok_or_else(overflow)?;
        let num_vars = crossing.checked_add(m).ok_or_else(overflow)?;
        let last_label = Self::exact_i64(parts.saturating_sub(1), "bounding partition labels")?;
        let x_idx = |v: usize, c: usize| v * parts + c;
        let empty_idx = |c: usize| square + c;
        let label_idx = |v: usize| labels + v;
        let y_idx = |t: usize| crossing + t;
        let mut constraints = Vec::new();
        let vertex_weights = self.vertex_weights();
        let arc_costs = self.arc_costs();
        let weight_bound = *self.weight_bound();
        let cost_bound = *self.cost_bound();

        // Assignment: Σ_c x_{v,c} = 1 for each vertex v.
        for v in 0..n {
            let terms: Vec<(usize, i64)> = (0..parts).map(|c| (x_idx(v, c), 1)).collect();
            constraints.push(LinearConstraint::eq(terms, 1));
            let mut label = vec![(label_idx(v), 1)];
            for c in 1..parts {
                label.push((
                    x_idx(v, c),
                    -Self::exact_i64(c, "representing a partition label")?,
                ));
            }
            constraints.push(LinearConstraint::eq(label, 0));
        }

        // Only occupied classes must meet the weight bound, which can be negative.
        for c in 0..parts {
            let mut membership = vec![(empty_idx(c), 1)];
            for v in 0..n {
                constraints.push(LinearConstraint::le(
                    vec![(x_idx(v, c), 1), (empty_idx(c), 1)],
                    1,
                ));
                membership.push((x_idx(v, c), 1));
            }
            constraints.push(LinearConstraint::ge(membership, 1));
            let mut terms: Vec<(usize, i64)> = vertex_weights
                .iter()
                .enumerate()
                .map(|(vertex, &weight)| (x_idx(vertex, c), weight))
                .collect();
            // Keep the bound on the RHS to preserve representable source sums.
            terms.push((empty_idx(c), weight_bound.min(0)));
            constraints.push(LinearConstraint::le(terms, weight_bound));
        }

        // A crossing arc increases its part label by at least one; an internal
        // arc has equal labels. This equivalence also handles negative costs.
        for (t, &(u, v)) in arcs.iter().enumerate() {
            constraints.push(LinearConstraint::ge(
                vec![(label_idx(v), 1), (label_idx(u), -1), (y_idx(t), -1)],
                0,
            ));
            constraints.push(LinearConstraint::le(
                vec![
                    (label_idx(v), 1),
                    (label_idx(u), -1),
                    (y_idx(t), -last_label),
                ],
                0,
            ));
        }

        // Cost bound: Σ_t cost(a_t) * y_t ≤ K.
        let cost_terms: Vec<(usize, i64)> = arc_costs
            .iter()
            .enumerate()
            .map(|(arc, &cost)| (y_idx(arc), cost))
            .collect();
        constraints.push(LinearConstraint::le(cost_terms, cost_bound));

        let mut variables = vec![IntegerVariable::binary(); num_vars];
        variables[labels..crossing].fill(
            IntegerVariable::new(Some(0), Some(last_label)).map_err(Self::target_construction)?,
        );
        let target = ILP::with_variables(variables, constraints, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;

        Ok(ReductionAcyclicPartitionToILP {
            target,
            n,
            one_hot: true,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::topology::DirectedGraph;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "acyclicpartition_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(AcyclicPartition::new(
                DirectedGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]),
                vec![1, 1, 1, 1],
                vec![1, 1, 1],
                3,
                2,
            ))
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/acyclicpartition_ilp.rs"]
mod tests;
