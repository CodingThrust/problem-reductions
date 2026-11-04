//! Reduction from Boyce-Codd Normal Form Violation to Satisfiability.
//!
//! The target unrolls the least functional-dependency closure of the selected
//! attribute set into `N` synchronous layers. Every gate is a full equivalence,
//! so the final layer is exactly the closure `X⁺` of the selection `X`.
//! Two final clauses demand a gained target attribute and a missing one.
//!
//! Variables, in order (one-based): selection bits `x_a` for `a ∈ R`; closure
//! bits `c[t][a]` for `t = 0..=N` and every attribute `a` (round-major);
//! dependency triggers `g[t][d]` for `t < N` (round-major); gain bits `w_a`.

use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::misc::BoyceCoddNormalFormViolation;
use crate::reduction;
use crate::registry::ConstructionError;
use crate::rules::ReductionError;
use crate::rules::traits::{ReduceTo, ReductionResult};

/// Result of reducing a Boyce-Codd normal form violation instance to SAT.
#[derive(Debug, Clone)]
pub struct ReductionBCNFViolationToSAT {
    target: Satisfiability,
    num_selection_bits: usize,
}

impl ReductionResult for ReductionBCNFViolationToSAT {
    type Source = BoyceCoddNormalFormViolation;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Satisfiability {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &Vec<bool>,
    ) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            target_solution,
            |value| value.0,
            "target assignment does not satisfy the closure encoding",
        )?;
        Ok(target_solution[..self.num_selection_bits].to_vec())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionBCNFViolationToSAT {}

/// Number of SAT variables `N(N+1) + N·D + 2r`, checked against the signed
/// literal range (`i64::MAX`).
fn variable_count(n: usize, d: usize, r: usize) -> Result<usize, ConstructionError> {
    let total = n
        .checked_add(1)
        .and_then(|n1| n.checked_mul(n1))
        .and_then(|layers| layers.checked_add(n.checked_mul(d)?))
        .and_then(|sum| sum.checked_add(r.checked_mul(2)?))
        .ok_or_else(|| {
            ConstructionError::from(
                "BoyceCoddNormalFormViolation -> Satisfiability variable count overflows",
            )
        })?;
    if u64::try_from(total).map_or(true, |t| t > i64::MAX as u64) {
        return Err(format!(
            "BoyceCoddNormalFormViolation -> Satisfiability needs {total} variables; SAT variable numbers are limited to {}",
            i64::MAX
        )
        .into());
    }
    Ok(total)
}

/// Number of clauses `N + 4r + N(L + B + D + 2N) + 2`, checked.
fn clause_count(n: usize, d: usize, r: usize, l: usize, b: usize) -> Option<usize> {
    let per_round = l
        .checked_add(b)?
        .checked_add(d)?
        .checked_add(n.checked_mul(2)?)?;
    n.checked_add(r.checked_mul(4)?)?
        .checked_add(n.checked_mul(per_round)?)?
        .checked_add(2)
}

#[reduction(transform = {
    exact {
        num_vars = "num_attributes * (num_attributes + 1) + num_attributes * num_functional_deps + 2 * num_target_attributes",
    },
    upper_bound {
        num_clauses = "num_attributes + 4 * num_target_attributes + num_attributes * (2 * num_attributes * num_functional_deps + num_functional_deps + 2 * num_attributes) + 2",
        num_literals = "num_attributes + 12 * num_target_attributes + num_attributes * (6 * num_attributes * num_functional_deps + num_functional_deps + 4 * num_attributes)",
    },
})]
impl ReduceTo<Satisfiability> for BoyceCoddNormalFormViolation {
    type Result = ReductionBCNFViolationToSAT;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let construction = <Self as ReduceTo<Satisfiability>>::target_construction;
        let n = self.num_attributes();
        let fds = self.functional_deps();
        let d = fds.len();
        let target_set = self.target_subset();
        let r = target_set.len();
        let num_vars = variable_count(n, d, r).map_err(construction)?;
        let lhs_total = fds.iter().map(|(l, _)| l.len()).sum::<usize>();
        let rhs_total = fds.iter().map(|(_, rr)| rr.len()).sum::<usize>();
        let num_clauses = clause_count(n, d, r, lhs_total, rhs_total).ok_or_else(|| {
            construction(
                "BoyceCoddNormalFormViolation -> Satisfiability clause count overflows".into(),
            )
        })?;
        let mut clauses: Vec<CNFClause> = Vec::new();
        clauses
            .try_reserve_exact(num_clauses)
            .map_err(|_| construction(format!("cannot allocate {num_clauses} clauses").into()))?;

        // All numbers below are at most `num_vars <= i64::MAX`.
        let lit = |v: usize| -> i64 { i64::try_from(v).expect("variable number fits i64") };
        let mut position = vec![None; n];
        for (i, &a) in target_set.iter().enumerate() {
            position[a] = Some(i);
        }
        let x = |pos: usize| lit(1 + pos);
        let c = |t: usize, a: usize| lit(1 + r + t * n + a);
        let g_base = r + (n + 1) * n;
        let g = |t: usize, dep: usize| lit(1 + g_base + t * d + dep);
        let w_base = g_base + n * d;
        let w = |pos: usize| lit(1 + w_base + pos);

        // Dependencies concluding each attribute.
        let mut concluding: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (dep, (_, rhs)) in fds.iter().enumerate() {
            for &a in rhs {
                concluding[a].push(dep);
            }
        }

        for (a, slot) in position.iter().enumerate() {
            match *slot {
                Some(pos) => {
                    clauses.push(CNFClause::new(vec![-c(0, a), x(pos)]));
                    clauses.push(CNFClause::new(vec![c(0, a), -x(pos)]));
                }
                None => clauses.push(CNFClause::new(vec![-c(0, a)])),
            }
        }
        for t in 0..n {
            for (dep, (lhs, _)) in fds.iter().enumerate() {
                let trigger = g(t, dep);
                for &a in lhs {
                    clauses.push(CNFClause::new(vec![-trigger, c(t, a)]));
                }
                let mut all = vec![trigger];
                all.extend(lhs.iter().map(|&a| -c(t, a)));
                clauses.push(CNFClause::new(all));
            }
            for (a, deps) in concluding.iter().enumerate() {
                let mut inputs = vec![c(t, a)];
                inputs.extend(deps.iter().map(|&dep| g(t, dep)));
                for &h in &inputs {
                    clauses.push(CNFClause::new(vec![-h, c(t + 1, a)]));
                }
                let mut rev = vec![-c(t + 1, a)];
                rev.extend(inputs);
                clauses.push(CNFClause::new(rev));
            }
        }
        for (pos, &a) in target_set.iter().enumerate() {
            clauses.push(CNFClause::new(vec![-w(pos), -x(pos)]));
            clauses.push(CNFClause::new(vec![-w(pos), c(n, a)]));
            clauses.push(CNFClause::new(vec![w(pos), x(pos), -c(n, a)]));
        }
        clauses.push(CNFClause::new((0..r).map(w).collect()));
        clauses.push(CNFClause::new(
            target_set.iter().map(|&a| -c(n, a)).collect(),
        ));

        let target = Satisfiability::try_new(num_vars, clauses).map_err(construction)?;
        Ok(ReductionBCNFViolationToSAT {
            target,
            num_selection_bits: r,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "boycecoddnormalformviolation_to_satisfiability",
        build: || {
            let source = BoyceCoddNormalFormViolation::new(
                4,
                vec![(vec![0], vec![1]), (vec![1], vec![2])],
                vec![0, 1, 3],
            );
            // X = {0}; layers 1000 1100 1110 1110 1110; triggers 10 11 11 11; gains 010.
            let bits = "100\
                1000110011101110\
                1110\
                10111111\
                010";
            let target_config: Vec<bool> = bits.chars().map(|ch| ch == '1').collect();
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                source,
                SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false]),
                    target_config: serde_json::json!(target_config),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/boycecoddnormalformviolation_satisfiability.rs"]
mod tests;
