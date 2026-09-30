//! Binary cumulative evaluation and live-value indicators for register sufficiency.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::RegisterSufficiency;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionRegisterSufficiencyToILP {
    target: ILP<bool>,
    num_vertices: usize,
}

impl ReductionResult for ReductionRegisterSufficiencyToILP {
    type Source = RegisterSufficiency;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

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

        let n = self.num_vertices;
        (0..n)
            .map(|v| {
                target_solution[v * n..(v + 1) * n]
                    .iter()
                    .position(|&computed| computed != 0)
                    .ok_or_else(|| {
                        crate::rules::ExtractionError::invalid("vertex is never computed")
                    })
            })
            .collect()
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionRegisterSufficiencyToILP {}

#[reduction(transform = {
    exact {
        num_vars = "2 * num_vertices^2 - num_vertices * num_sinks",
        num_constraints = "num_vertices^2 + 2 * num_vertices * num_arcs + num_vertices",
        num_nonzeros = "4 * num_vertices^2 - 2 * num_vertices + 5 * num_vertices * num_arcs - num_arcs",
    },
    upper_bound {
        max_constraint_magnitude_bits = "num_vertices + 1",
    },
})]
impl ReduceTo<ILP<bool>> for RegisterSufficiency {
    type Result = ReductionRegisterSufficiencyToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(
                "counting cumulative scheduling variables",
            )
        };
        let square = n.checked_mul(n).ok_or_else(overflow)?;
        let mut has_dependent = vec![false; n];
        for &(_, u) in self.arcs() {
            has_dependent[u] = true;
        }
        let non_sinks = has_dependent.iter().filter(|&&value| value).count();
        let num_vars = n
            .checked_mul(non_sinks)
            .and_then(|count| square.checked_add(count))
            .ok_or_else(overflow)?;
        let horizon = Self::exact_i64(n, "bounding schedule row sums")?;
        let bound = Self::exact_i64(self.bound(), "representing the register bound")?.min(horizon);
        let computed = |v: usize, t: usize| v * n + t;
        let mut live_offset: Vec<_> = (0..n).map(|v| v * n).collect();
        let mut next = square;
        for (v, &needed) in has_dependent.iter().enumerate() {
            if needed {
                live_offset[v] = next;
                next += n;
            }
        }
        let mut constraints = Vec::new();
        for v in 0..n {
            for t in 0..n.saturating_sub(1) {
                constraints.push(LinearConstraint::le(
                    vec![(computed(v, t), 1), (computed(v, t + 1), -1)],
                    0,
                ));
            }
        }
        for t in 0..n {
            constraints.push(LinearConstraint::eq(
                (0..n).map(|v| (computed(v, t), 1)).collect(),
                Self::exact_i64(t + 1, "counting completed vertices")?,
            ));
        }
        for &(w, u) in self.arcs() {
            for t in 0..n {
                let mut precedence = vec![(computed(w, t), 1)];
                if t > 0 {
                    precedence.push((computed(u, t - 1), -1));
                }
                constraints.push(LinearConstraint::le(precedence, 0));
                constraints.push(LinearConstraint::ge(
                    vec![
                        (live_offset[u] + t, 1),
                        (computed(u, t), -1),
                        (computed(w, t), 1),
                    ],
                    0,
                ));
            }
        }
        for t in 0..n {
            // For sinks, computed bits are also their exact live indicators.
            constraints.push(LinearConstraint::le(
                live_offset.iter().map(|&offset| (offset + t, 1)).collect(),
                bound,
            ));
        }

        Ok(ReductionRegisterSufficiencyToILP {
            target: ILP::new(num_vars, constraints, vec![], ObjectiveSense::Minimize)
                .map_err(Self::target_construction)?,
            num_vertices: n,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "registersufficiency_to_ilp",
        build: || {
            let source = RegisterSufficiency::new(
                7,
                vec![
                    (2, 0),
                    (2, 1),
                    (3, 1),
                    (4, 2),
                    (4, 3),
                    (5, 0),
                    (6, 4),
                    (6, 5),
                ],
                3,
            );
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/registersufficiency_ilp.rs"]
mod tests;
