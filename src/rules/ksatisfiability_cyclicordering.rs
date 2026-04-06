//! Reduction from KSatisfiability (3-SAT) to CyclicOrdering.
//!
//! Galil and Megiddo's construction associates each variable with three
//! elements `(alpha_i, beta_i, gamma_i)`. A satisfying assignment is encoded by
//! which of the two cyclic orientations `(alpha_i, beta_i, gamma_i)` and
//! `(alpha_i, gamma_i, beta_i)` is derived by the final cyclic order. Each
//! clause contributes five fresh auxiliary elements and ten cyclic-ordering
//! triples enforcing that at least one literal orientation must be the
//! "true" one.
//!
//! Reference: Galil and Megiddo, "Cyclic ordering is NP-complete", 1977.

use crate::models::formula::KSatisfiability;
use crate::models::misc::CyclicOrdering;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;

#[derive(Debug, Clone)]
pub struct Reduction3SATToCyclicOrdering {
    target: CyclicOrdering,
    source_num_vars: usize,
}

impl ReductionResult for Reduction3SATToCyclicOrdering {
    type Source = KSatisfiability<K3>;
    type Target = CyclicOrdering;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.source_num_vars)
            .map(|var_idx| {
                let (alpha, beta, gamma) = variable_triple(var_idx);
                usize::from(!is_cyclic_order(
                    target_solution[alpha],
                    target_solution[beta],
                    target_solution[gamma],
                ))
            })
            .collect()
    }
}

fn variable_triple(var_idx: usize) -> (usize, usize, usize) {
    let base = 3 * var_idx;
    (base, base + 1, base + 2)
}

fn literal_triple(literal: i32) -> (usize, usize, usize) {
    let (alpha, beta, gamma) = variable_triple((literal.unsigned_abs() as usize) - 1);
    if literal > 0 {
        (alpha, beta, gamma)
    } else {
        (alpha, gamma, beta)
    }
}

#[allow(clippy::nonminimal_bool)]
fn is_cyclic_order(a: usize, b: usize, c: usize) -> bool {
    (a < b && b < c) || (b < c && c < a) || (c < a && a < b)
}

#[reduction(
    overhead = {
        num_elements = "3 * num_vars + 5 * num_clauses",
        num_triples = "10 * num_clauses",
    }
)]
impl ReduceTo<CyclicOrdering> for KSatisfiability<K3> {
    type Result = Reduction3SATToCyclicOrdering;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vars();
        let num_clauses = self.num_clauses();
        let num_elements = 3 * num_vars + 5 * num_clauses;
        let mut triples = Vec::with_capacity(10 * num_clauses);

        for (clause_idx, clause) in self.clauses().iter().enumerate() {
            let (a, b, c) = literal_triple(clause.literals[0]);
            let (d, e, f) = literal_triple(clause.literals[1]);
            let (g, h, i) = literal_triple(clause.literals[2]);

            let base = 3 * num_vars + 5 * clause_idx;
            let j = base;
            let k = base + 1;
            let l = base + 2;
            let m = base + 3;
            let n = base + 4;

            triples.extend([
                (a, c, j),
                (b, j, k),
                (c, k, l),
                (d, f, j),
                (e, j, l),
                (f, l, m),
                (g, i, k),
                (h, k, m),
                (i, m, n),
                (n, m, l),
            ]);
        }

        Reduction3SATToCyclicOrdering {
            target: CyclicOrdering::new(num_elements, triples),
            source_num_vars: num_vars,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_cyclicordering",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, CyclicOrdering>(
                KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])]),
                SolutionPair {
                    source_config: vec![1, 1, 1],
                    target_config: vec![0, 11, 1, 9, 12, 10, 6, 13, 7, 2, 3, 4, 8, 5],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_cyclicordering.rs"]
mod tests;
