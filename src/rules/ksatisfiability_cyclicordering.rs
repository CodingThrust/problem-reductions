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
//! Before applying the gadget, remove tautologies and repeated literals,
//! compact occurring variables, and expand short clauses with fresh variables.
//! Each resulting clause has three distinct variables in global index order.
//! This is the hypothesis needed to combine the paper's local cyclic orders.
//! Empty clauses and empty conjunctions map to fixed NO and YES targets.
//!
//! Reference: Galil and Megiddo, "Cyclic ordering is NP-complete", 1977.

use crate::models::formula::KSatisfiability;
use crate::models::misc::CyclicOrdering;
use crate::reduction;
use crate::rules::sat_helpers::SatVariableAllocator;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Reduction3SATToCyclicOrdering {
    target: CyclicOrdering,
    source_num_vars: usize,
    source_variables: Vec<usize>,
}

impl ReductionResult for Reduction3SATToCyclicOrdering {
    type Source = KSatisfiability<K3>;
    type Target = CyclicOrdering;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !value.0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target configuration is not a feasible cyclic ordering",
            ));
        }
        let mut assignment = vec![false; self.source_num_vars];
        for (compact, &original) in self.source_variables.iter().enumerate() {
            let (alpha, beta, gamma) = variable_triple(compact);
            assignment[original] = !is_cyclic_order(
                target_solution[alpha],
                target_solution[beta],
                target_solution[gamma],
            );
        }
        Ok(assignment)
    }
}

fn variable_triple(var_idx: usize) -> (usize, usize, usize) {
    let base = 3 * var_idx;
    (base, base + 1, base + 2)
}

fn literal_triple(literal: i64) -> (usize, usize, usize) {
    let (alpha, beta, gamma) = variable_triple(
        usize::try_from(literal.unsigned_abs()).expect("normalized literal indices fit usize") - 1,
    );
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

/// Three distinct, globally ordered variables per clause are required by
/// Galil--Megiddo's simultaneous-extension argument (Corollary 2).
struct NormalizedFormula {
    source_variables: Vec<usize>,
    num_vars: usize,
    clauses: Vec<[i64; 3]>,
}

fn normalize(
    source: &KSatisfiability<K3>,
) -> Result<NormalizedFormula, crate::rules::ReductionError> {
    let mut clauses = Vec::new();
    let mut variables = BTreeSet::new();
    for clause in source.clauses() {
        let mut literals = clause.literals.clone();
        literals.sort_unstable_by_key(|literal| (literal.unsigned_abs(), *literal));
        literals.dedup();
        if literals.windows(2).any(|pair| pair[0] == -pair[1]) {
            continue;
        }
        for literal in &literals {
            variables.insert(
                usize::try_from(literal.unsigned_abs()).expect("native SAT indices fit usize") - 1,
            );
        }
        clauses.push(literals);
    }
    let source_variables: Vec<_> = variables.into_iter().collect();
    let mut variables =
        SatVariableAllocator::new("KSatisfiability -> CyclicOrdering", source_variables.len())
            .map_err(<KSatisfiability<K3> as ReduceTo<CyclicOrdering>>::target_construction)?;
    let mut normalized = Vec::new();
    for clause in clauses {
        let literals: Vec<_> = clause
            .iter()
            .map(|literal| {
                let original = usize::try_from(literal.unsigned_abs())
                    .expect("native SAT indices fit usize")
                    - 1;
                let compact = source_variables
                    .binary_search(&original)
                    .expect("all retained variables were collected")
                    + 1;
                let index = i64::try_from(compact)
                    .expect("compaction cannot increase a valid source variable index");
                if *literal > 0 {
                    index
                } else {
                    -index
                }
            })
            .collect();
        match *literals.as_slice() {
            [a, b, c] => normalized.push([a, b, c]),
            [a, b] => {
                let u = variables.allocate()
                    .map_err(<KSatisfiability<K3> as ReduceTo<CyclicOrdering>>::target_construction)?;
                normalized.extend([[a, b, u], [a, b, -u]]);
            }
            [a] => {
                let u = variables.allocate()
                    .map_err(<KSatisfiability<K3> as ReduceTo<CyclicOrdering>>::target_construction)?;
                let v = variables.allocate()
                    .map_err(<KSatisfiability<K3> as ReduceTo<CyclicOrdering>>::target_construction)?;
                normalized.extend([[a, u, v], [a, u, -v], [a, -u, v], [a, -u, -v]]);
            }
            _ => unreachable!("empty clauses are handled before normalization; native clauses have at most three literals"),
        }
    }
    // Original compact indices precede all fresh indices. Every emitted
    // clause is therefore already sorted by absolute variable index.
    Ok(NormalizedFormula {
        source_variables,
        num_vars: variables.num_vars(),
        clauses: normalized,
    })
}

#[reduction(
    transform = upper_bound {
        num_elements = "3 * num_vars + 26 * num_clauses + 3",
        num_triples = "40 * num_clauses + 2",
    }
)]
impl ReduceTo<CyclicOrdering> for KSatisfiability<K3> {
    type Result = Reduction3SATToCyclicOrdering;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        if self
            .clauses()
            .iter()
            .any(|clause| clause.literals.is_empty())
        {
            return Ok(Reduction3SATToCyclicOrdering {
                target: CyclicOrdering::try_new(3, vec![(0, 1, 2), (0, 2, 1)])
                    .map_err(<Self as ReduceTo<CyclicOrdering>>::target_construction)?,
                source_num_vars: self.num_vars(),
                source_variables: Vec::new(),
            });
        }
        let normalized = normalize(self)?;
        let num_vars = normalized.num_vars;
        let num_clauses = normalized.clauses.len();
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, CyclicOrdering>(operation)
        };
        let variable_elements = num_vars
            .checked_mul(3)
            .ok_or_else(|| overflow("counting variable elements"))?;
        let clause_elements = num_clauses
            .checked_mul(5)
            .ok_or_else(|| overflow("counting clause elements"))?;
        let num_elements = variable_elements
            .checked_add(clause_elements)
            .ok_or_else(|| overflow("counting target elements"))?
            .max(1);
        let num_triples = num_clauses
            .checked_mul(10)
            .ok_or_else(|| overflow("counting target triples"))?;
        let mut triples = Vec::with_capacity(num_triples);

        for (clause_idx, clause) in normalized.clauses.iter().enumerate() {
            let (a, b, c) = literal_triple(clause[0]);
            let (d, e, f) = literal_triple(clause[1]);
            let (g, h, i) = literal_triple(clause[2]);

            let base = variable_elements + 5 * clause_idx;
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

        Ok(Reduction3SATToCyclicOrdering {
            target: CyclicOrdering::try_new(num_elements, triples)
                .map_err(<Self as ReduceTo<CyclicOrdering>>::target_construction)?,
            source_num_vars: self.num_vars(),
            source_variables: normalized.source_variables,
        })
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
                    source_config: serde_json::json!(vec![true, true, true]),
                    target_config: serde_json::json!(vec![
                        0, 11, 1, 9, 12, 10, 6, 13, 7, 2, 3, 4, 8, 5
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_cyclicordering.rs"]
mod tests;
