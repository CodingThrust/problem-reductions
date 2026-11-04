//! Unit containment indicators encoded by penalized binary product chains.
use crate::models::{ComparativeContainment, Decision, QUBO};
use crate::rules::qubo_helpers::QuboBuilder;
use crate::rules::{ReduceTo, ReductionError, ReductionResult};
use crate::types::One;
use std::collections::HashSet;

type Builder = QuboBuilder<ComparativeContainment<One>, Decision<QUBO<i64>>>;

#[derive(Debug, Clone)]
pub struct ReductionComparativeContainmentToQUBO {
    target: Decision<QUBO<i64>>,
    universe_size: usize,
}

impl ReductionResult for ReductionComparativeContainmentToQUBO {
    type Source = ComparativeContainment<One>;
    type Target = Decision<QUBO<i64>>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, bits: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            bits,
            |value| value.0,
            "QUBO does not meet the containment threshold",
        )?;
        Ok(bits[..self.universe_size].to_vec())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionComparativeContainmentToQUBO {}

#[crate::reduction(transform = upper_bound {
    num_vars = "universe_size + universe_size * (num_r_sets + num_s_sets)",
    num_quadratic_terms = "6 * universe_size * (num_r_sets + num_s_sets) + num_r_sets + num_s_sets",
})]
impl ReduceTo<Decision<QUBO<i64>>> for ComparativeContainment<One> {
    type Result = ReductionComparativeContainmentToQUBO;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        construct(self, i64::MAX)
    }
}

fn checked_sizes(
    n: usize,
    r: usize,
    s: usize,
    g: usize,
    limit: i64,
) -> Result<(usize, i64), ReductionError> {
    let count = r.checked_add(s).ok_or_else(Builder::overflow)?;
    let num_vars = n.checked_add(g).ok_or_else(Builder::overflow)?;
    let r = i64::try_from(r).map_err(|_| Builder::overflow())?;
    let count = i64::try_from(count).map_err(|_| Builder::overflow())?;
    let g = i64::try_from(g).map_err(|_| Builder::overflow())?;
    let penalty = r.checked_add(1).ok_or_else(Builder::overflow)?;
    // Multiply by the gate count first: a gate-free target needs no 8*M.
    let certificate = penalty
        .checked_mul(g)
        .and_then(|v| v.checked_mul(8))
        .and_then(|v| v.checked_add(count))
        .ok_or_else(Builder::overflow)?;
    if penalty > limit || certificate > limit {
        return Err(Builder::overflow());
    }
    Ok((num_vars, penalty))
}

fn construct(
    source: &ComparativeContainment<One>,
    limit: i64,
) -> Result<ReductionComparativeContainmentToQUBO, ReductionError> {
    let n = source.universe_size();
    let mut g = 0usize;
    for entry in source.r_sets().iter().chain(source.s_sets()) {
        let distinct: HashSet<_> = entry.iter().collect();
        let k = n - distinct.len();
        g = g
            .checked_add(k.saturating_sub(1))
            .ok_or_else(Builder::overflow)?;
    }
    let (num_vars, penalty) = checked_sizes(n, source.num_r_sets(), source.num_s_sets(), g, limit)?;
    let p = i128::from(penalty);
    let mut q = Builder::new(num_vars);
    let mut next = n;
    for (family, sign) in [(source.r_sets(), -1i128), (source.s_sets(), 1)] {
        for entry in family {
            let present: HashSet<_> = entry.iter().copied().collect();
            let mut complement = (0..n).filter(|v| !present.contains(v));
            let Some(u) = complement.next() else {
                q.add_constant(sign)?;
                continue;
            };
            let Some(v) = complement.next() else {
                q.add_constant(sign)?;
                q.add(u, u, -sign)?;
                continue;
            };
            let mut z = next;
            next = next.checked_add(1).ok_or_else(Builder::overflow)?;
            // P(1-x_u,1-x_v,z) = 1-x_u-x_v+x_u*x_v-z+2*x_u*z+2*x_v*z.
            q.add_constant(p)?;
            for (a, b, c) in [
                (u, u, -p),
                (v, v, -p),
                (u, v, p),
                (z, z, -p),
                (u, z, 2 * p),
                (v, z, 2 * p),
            ] {
                q.add(a, b, c)?;
            }
            for v in complement {
                let previous = z;
                z = next;
                next = next.checked_add(1).ok_or_else(Builder::overflow)?;
                // P(w,1-x_v,z) = w-w*x_v-2*w*z+z+2*x_v*z.
                for (a, b, c) in [
                    (previous, previous, p),
                    (previous, v, -p),
                    (previous, z, -2 * p),
                    (z, z, p),
                    (v, z, 2 * p),
                ] {
                    q.add(a, b, c)?;
                }
            }
            q.add(z, z, sign)?;
        }
    }
    let (target, constant) = q.finish()?;
    let bound = constant.checked_neg().ok_or_else(Builder::overflow)?;
    Ok(ReductionComparativeContainmentToQUBO {
        target: Decision::new(target, bound),
        universe_size: n,
    })
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "comparativecontainment_qubo",
        build: || {
            let source = ComparativeContainment::<One>::new(
                4,
                vec![vec![0, 1], vec![1, 2], vec![1, 3]],
                vec![vec![0, 1, 2, 3]; 2],
            )
            .expect("canonical containment instance must be valid");
            crate::example_db::specs::rule_example_with_witness::<_, Decision<QUBO<i64>>>(
                source,
                crate::export::SolutionPair {
                    source_config: serde_json::json!([false, true, false, false]),
                    target_config: serde_json::json!([false, true, false, false, true, true, true]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/comparativecontainment_qubo.rs"]
mod tests;
