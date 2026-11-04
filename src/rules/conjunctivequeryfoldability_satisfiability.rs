//! Reduction of exact-image conjunctive query foldability to CNF satisfiability.

use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::misc::{ConjunctiveQueryFoldability, Term};
use crate::rules::{ReduceTo, ReductionError, ReductionResult};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct ReductionConjunctiveQueryFoldabilityToSAT {
    target: Satisfiability,
    candidates: Vec<usize>,
    num_selectors: usize,
}

impl ReductionResult for ReductionConjunctiveQueryFoldabilityToSAT {
    type Source = ConjunctiveQueryFoldability;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Satisfiability {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.0,
            "target assignment does not satisfy the folding CNF",
        )?;
        Ok(solution[..self.num_selectors]
            .iter()
            .enumerate()
            .filter(|(_, chosen)| **chosen)
            .map(|(index, _)| self.candidates[index % self.candidates.len()])
            .collect())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionConjunctiveQueryFoldabilityToSAT {}

fn overflow(operation: &str) -> ReductionError {
    ReductionError::integer_overflow::<ConjunctiveQueryFoldability, Satisfiability>(operation)
}

fn variable_counts(
    u: usize,
    c: usize,
    p: usize,
    q: usize,
) -> Result<(usize, usize), ReductionError> {
    let selectors = u
        .checked_mul(c)
        .ok_or_else(|| overflow("counting selectors"))?;
    let matches = p
        .checked_mul(q)
        .ok_or_else(|| overflow("counting atom matches"))?;
    let total = selectors
        .checked_add(matches)
        .ok_or_else(|| overflow("counting SAT variables"))?;
    Ok((selectors, total))
}

#[crate::reduction(transform = upper_bound {
    num_vars = "num_undistinguished * (domain_size + num_distinguished + num_undistinguished) + num_conjuncts_q1 * num_conjuncts_q2",
    num_clauses = "num_undistinguished * (1 + (domain_size + num_distinguished + num_undistinguished) * (domain_size + num_distinguished + num_undistinguished - 1) / 2) + num_conjuncts_q1 * num_conjuncts_q2 * (num_undistinguished + 1) + num_conjuncts_q1 + num_conjuncts_q2",
    num_literals = "num_undistinguished * (domain_size + num_distinguished + num_undistinguished)^2 + num_conjuncts_q1 * num_conjuncts_q2 * (3 * num_undistinguished + 3)",
})]
impl ReduceTo<Satisfiability> for ConjunctiveQueryFoldability {
    type Result = ReductionConjunctiveQueryFoldabilityToSAT;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let d = self.domain_size();
        let u = self.num_undistinguished();
        let fixed = d
            .checked_add(self.num_distinguished())
            .ok_or_else(|| overflow("counting fixed terms"))?;
        let range = fixed
            .checked_add(u)
            .ok_or_else(|| overflow("counting all terms"))?;
        let index = |term: &Term| match *term {
            Term::Constant(i) => i,
            Term::Distinguished(i) => d + i,
            Term::Undistinguished(i) => fixed + i,
        };
        let mut terms = BTreeSet::new();
        if range > 0 {
            terms.insert(0);
        }
        for (_, args) in self.query2_conjuncts() {
            terms.extend(args.iter().map(index));
        }
        let candidates: Vec<_> = terms.into_iter().collect();
        let c = candidates.len();
        let p = self.num_conjuncts_q1();
        let q = self.num_conjuncts_q2();
        let (num_selectors, num_vars) = variable_counts(u, c, p, q)?;
        Self::exact_i64(num_vars, "representing SAT literal indices")?;
        // Every offset below is bounded by the checked total variable count.
        let selector = |y, t| Self::exact_i64(y * c + t + 1, "numbering selectors");
        let match_bit = |i, j| Self::exact_i64(num_selectors + i * q + j + 1, "numbering matches");
        let mut clauses = Vec::new();
        for y in 0..u {
            clauses.push(CNFClause::new(
                (0..c).map(|t| selector(y, t)).collect::<Result<_, _>>()?,
            ));
            for t in 0..c {
                for s in t + 1..c {
                    clauses.push(CNFClause::new(vec![-selector(y, t)?, -selector(y, s)?]));
                }
            }
        }
        for (i, (ra, aa)) in self.query1_conjuncts().iter().enumerate() {
            for (j, (rb, ab)) in self.query2_conjuncts().iter().enumerate() {
                let z = match_bit(i, j)?;
                let mut compatible = ra == rb;
                let mut requirements = BTreeMap::new();
                if compatible {
                    for (a, b) in aa.iter().zip(ab) {
                        let b = index(b);
                        match a {
                            Term::Undistinguished(y) => {
                                if let Some(previous) = requirements.insert(*y, b) {
                                    compatible &= previous == b;
                                }
                            }
                            _ => compatible &= index(a) == b,
                        }
                    }
                }
                if !compatible {
                    clauses.push(CNFClause::new(vec![-z]));
                    continue;
                }
                let mut reverse = vec![z];
                for (y, term) in requirements {
                    let t = candidates
                        .binary_search(&term)
                        .expect("target terms are candidates");
                    let e = selector(y, t)?;
                    clauses.push(CNFClause::new(vec![-z, e]));
                    reverse.push(-e);
                }
                clauses.push(CNFClause::new(reverse));
            }
        }
        for i in 0..p {
            clauses.push(CNFClause::new(
                (0..q).map(|j| match_bit(i, j)).collect::<Result<_, _>>()?,
            ));
        }
        for j in 0..q {
            clauses.push(CNFClause::new(
                (0..p).map(|i| match_bit(i, j)).collect::<Result<_, _>>()?,
            ));
        }
        let target =
            Satisfiability::try_new(num_vars, clauses).map_err(Self::target_construction)?;
        Ok(ReductionConjunctiveQueryFoldabilityToSAT {
            target,
            candidates,
            num_selectors,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use Term::{Distinguished as X, Undistinguished as Y};
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "conjunctivequeryfoldability_to_satisfiability",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                ConjunctiveQueryFoldability::new(
                    0,
                    1,
                    3,
                    vec![2],
                    vec![
                        (0, vec![X(0), Y(0)]),
                        (0, vec![Y(0), Y(1)]),
                        (0, vec![Y(1), X(0)]),
                        (0, vec![Y(0), Y(0)]),
                    ],
                    vec![
                        (0, vec![X(0), Y(2)]),
                        (0, vec![Y(2), Y(2)]),
                        (0, vec![Y(2), X(0)]),
                    ],
                ),
                SolutionPair {
                    source_config: serde_json::json!([3, 3, 3]),
                    target_config: serde_json::json!([
                        false, true, false, true, false, true, true, false, false, false, true,
                        false, false, false, true, false, true, false
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/conjunctivequeryfoldability_satisfiability.rs"]
mod tests;
