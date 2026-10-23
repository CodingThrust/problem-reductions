//! Binary decision ILP to SAT using ripple adders and Tseitin gate clauses.

use crate::models::algebraic::{Comparison, ObjectiveSense, ILP};
use crate::models::{CNFClause, Decision, Satisfiability};
use crate::registry::ConstructionError;
use crate::rules::sat_helpers::SatVariableAllocator;
use crate::rules::{ReduceTo, ReductionError, ReductionResult};
use num_bigint::BigInt;
use num_traits::Zero;

/// A SAT witness retains the binary ILP assignment in its first coordinates.
#[derive(Debug, Clone)]
pub struct ReductionDecisionILPToSAT {
    target: Satisfiability,
    num_source_vars: usize,
}

impl ReductionResult for ReductionDecisionILPToSAT {
    type Source = Decision<ILP<bool>>;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Satisfiability {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<i64>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.0,
            "target assignment does not satisfy the binary ILP encoding",
        )?;
        Ok(solution[..self.num_source_vars]
            .iter()
            .map(|&bit| i64::from(bit))
            .collect())
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionDecisionILPToSAT {}

// Checked signed totals bound each row sum by 2^64-1: <=64 bits,
// 5 gates per adder cell, <=64 comparator gates/units, plus the objective row.
#[crate::reduction(transform = upper_bound {
    num_vars = "num_vars + 1 + (num_constraints + 1) * 64 * (5 * num_vars + 1)",
    num_clauses = "1 + (num_constraints + 1) * (1280 * num_vars + 321)",
    num_literals = "3 * (1 + (num_constraints + 1) * (1280 * num_vars + 321))",
})]
impl ReduceTo<Satisfiability> for Decision<ILP<bool>> {
    type Result = ReductionDecisionILPToSAT;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let source = self.inner();
        let construction = <Self as ReduceTo<Satisfiability>>::target_construction;
        let mut variables =
            SatVariableAllocator::new("DecisionILP -> Satisfiability", source.num_vars())
                .map_err(construction)?;
        let false_literal = variables.allocate().map_err(construction)?;
        let mut encoder = Encoder {
            variables,
            false_literal,
            clauses: vec![CNFClause::new(vec![-false_literal])],
        };
        for row in source.constraints() {
            encoder.row(row.terms(), row.comparison(), row.rhs())?;
        }
        let comparison = match source.sense() {
            ObjectiveSense::Minimize => Comparison::Le,
            ObjectiveSense::Maximize => Comparison::Ge,
        };
        encoder.row(source.objective(), comparison, *self.bound())?;
        Ok(ReductionDecisionILPToSAT {
            target: Satisfiability::try_new(encoder.variables.num_vars(), encoder.clauses)
                .map_err(construction)?,
            num_source_vars: source.num_vars(),
        })
    }
}

#[derive(Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

struct Encoder {
    variables: SatVariableAllocator,
    false_literal: i64,
    clauses: Vec<CNFClause>,
}

impl Encoder {
    fn gate(&mut self, gate: Gate, a: i64, b: i64) -> Result<i64, ConstructionError> {
        let f = self.false_literal;
        if a == f || a == -f {
            return Ok(match (gate, a == f) {
                (Gate::And, true) => f,
                (Gate::And, false) => b,
                (Gate::Or, true) => b,
                (Gate::Or, false) => -f,
                (Gate::Xor, true) => b,
                (Gate::Xor, false) => -b,
            });
        }
        if b == f || b == -f {
            return self.gate(gate, b, a);
        }
        let z = self.variables.allocate()?;
        let clauses = match gate {
            Gate::And => vec![vec![-z, a], vec![-z, b], vec![z, -a, -b]],
            Gate::Or => vec![vec![z, -a], vec![z, -b], vec![-z, a, b]],
            Gate::Xor => vec![
                vec![-a, -b, -z],
                vec![a, b, -z],
                vec![a, -b, z],
                vec![-a, b, z],
            ],
        };
        self.clauses.extend(clauses.into_iter().map(CNFClause::new));
        Ok(z)
    }

    fn row(
        &mut self,
        terms: &[(usize, i64)],
        comparison: Comparison,
        rhs: i64,
    ) -> Result<(), ReductionError> {
        let construction = <Decision<ILP<bool>> as ReduceTo<Satisfiability>>::target_construction;
        let overflow = || {
            ReductionError::integer_overflow::<Decision<ILP<bool>>, Satisfiability>(
                "bounding binary ILP row arithmetic",
            )
        };
        // Guarantee the source's checked i64 evaluator is defined for every assignment.
        let (positive, negative) = terms.iter().try_fold((0_i64, 0_i64), |(p, n), &(_, a)| {
            Ok::<_, ReductionError>((
                p.checked_add(a.max(0)).ok_or_else(overflow)?,
                n.checked_add(a.min(0)).ok_or_else(overflow)?,
            ))
        })?;
        let total = BigInt::from(positive) - BigInt::from(negative);
        let mut bound = BigInt::from(rhs) - BigInt::from(negative);
        let constant = match comparison {
            Comparison::Le if bound >= total => Some(true),
            Comparison::Le if bound < BigInt::zero() => Some(false),
            Comparison::Ge if bound <= BigInt::zero() => Some(true),
            Comparison::Ge if bound > total => Some(false),
            Comparison::Eq if bound < BigInt::zero() || bound > total => Some(false),
            Comparison::Eq if total.is_zero() => Some(true),
            _ => None,
        };
        if let Some(satisfied) = constant {
            if !satisfied {
                self.clauses.push(CNFClause::new(vec![]));
            }
            return Ok(());
        }
        let width = total.bits();
        let mut sum: Vec<_> = (0..width).map(|_| self.false_literal).collect();
        for &(index, coefficient) in terms {
            let variable = i64::try_from(index + 1).map_err(|_| overflow())?;
            let literal = if coefficient < 0 { -variable } else { variable };
            let weight = coefficient.unsigned_abs();
            let mut carry = self.false_literal;
            for (bit, sum_bit) in (0..width).zip(&mut sum) {
                let b = if weight & (1 << bit) != 0 {
                    literal
                } else {
                    self.false_literal
                };
                let a = *sum_bit;
                let p = self.gate(Gate::Xor, a, b).map_err(construction)?;
                *sum_bit = self.gate(Gate::Xor, p, carry).map_err(construction)?;
                let u = self.gate(Gate::And, a, b).map_err(construction)?;
                let v = self.gate(Gate::And, p, carry).map_err(construction)?;
                carry = self.gate(Gate::Or, u, v).map_err(construction)?;
            }
        }
        if comparison == Comparison::Ge {
            for bit in &mut sum {
                *bit = -*bit;
            }
            bound = (BigInt::from(1) << width) - 1 - bound;
        }
        let mut accepted = -self.false_literal;
        for (bit, &literal) in (0..width).zip(&sum) {
            let set = bound.bit(bit);
            if comparison == Comparison::Eq {
                self.clauses
                    .push(CNFClause::new(vec![if set { literal } else { -literal }]));
            } else {
                accepted = self
                    .gate(if set { Gate::Or } else { Gate::And }, -literal, accepted)
                    .map_err(construction)?;
            }
        }
        if comparison != Comparison::Eq {
            self.clauses.push(CNFClause::new(vec![accepted]));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionilp_sat.rs"]
mod tests;

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::{
        assemble_rule_example, rule_example_with_witness, RuleExampleSpec,
    };
    use crate::export::SolutionPair;
    use crate::models::algebraic::ilp::canonical_decision_example;

    vec![
        RuleExampleSpec {
            id: "decision_ilp_to_sat",
            build: || {
                let source = canonical_decision_example();
                let reduction = ReduceTo::<Satisfiability>::reduce_to(&source).unwrap();
                let target = reduction.target_problem();
                let witness = crate::solvers::ILPSolver::new().solve(target).unwrap();
                let decoded = reduction.extract_solution(&witness).unwrap();
                assemble_rule_example(
                    &source,
                    target,
                    vec![SolutionPair {
                        source_config: serde_json::json!(decoded),
                        target_config: serde_json::json!(witness),
                    }],
                )
            },
        },
        RuleExampleSpec {
            id: "decision_ilp_to_ilp",
            build: || {
                rule_example_with_witness::<_, ILP<bool>>(
                    canonical_decision_example(),
                    SolutionPair {
                        source_config: serde_json::json!([0, 0, 1]),
                        target_config: serde_json::json!([0, 0, 1]),
                    },
                )
            },
        },
    ]
}
