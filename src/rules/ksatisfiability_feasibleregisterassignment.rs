//! Reduction from KSatisfiability (3-SAT) to Feasible Register Assignment.
//!
//! This follows Sethi's Reduction 3 / Theorem 5.11 (STOC 1973):
//! Nonempty short clauses are padded by literal repetition, an empty clause
//! maps to a fixed infeasible DAG, and appearing variables are compacted with
//! an inverse map. No ordering or distinct-variable hypothesis is needed.
//! - Variable leaf pairs `s_pos[k], s_neg[k]` share register `S[k]`
//! - Each literal occurrence adds `p[i,j], q[i,j], r[i,j], rbar[i,j]`
//! - `r[i,j]` and `rbar[i,j]` share register `R[i,j]`
//! - Clause gadgets are linked cyclically through `(q[i,1], rbar[i,2])`,
//!   `(q[i,2], rbar[i,3])`, `(q[i,3], rbar[i,1])`
//! - A realization yields a truth assignment by comparing the order of
//!   `s_pos[k]` and `s_neg[k]`

use crate::models::formula::KSatisfiability;
use crate::models::misc::FeasibleRegisterAssignment;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;
use std::collections::BTreeSet;

fn s_pos_idx(var: usize) -> usize {
    var
}

fn s_neg_idx(num_vars: usize, var: usize) -> usize {
    num_vars + var
}

fn literal_base(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    2 * num_vars + 12 * clause_idx + 4 * literal_pos
}

fn p_idx(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    literal_base(num_vars, clause_idx, literal_pos)
}

fn q_idx(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    literal_base(num_vars, clause_idx, literal_pos) + 1
}

fn r_idx(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    literal_base(num_vars, clause_idx, literal_pos) + 2
}

fn rbar_idx(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    literal_base(num_vars, clause_idx, literal_pos) + 3
}

fn p_register(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    num_vars + 3 * (3 * clause_idx + literal_pos)
}

fn q_register(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    p_register(num_vars, clause_idx, literal_pos) + 1
}

fn r_register(num_vars: usize, clause_idx: usize, literal_pos: usize) -> usize {
    p_register(num_vars, clause_idx, literal_pos) + 2
}

#[derive(Debug, Clone)]
pub struct Reduction3SATToFeasibleRegisterAssignment {
    target: FeasibleRegisterAssignment,
    num_vars: usize,
    source_variables: Vec<usize>,
}

impl ReductionResult for Reduction3SATToFeasibleRegisterAssignment {
    type Source = KSatisfiability<K3>;
    type Target = FeasibleRegisterAssignment;

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
                "target configuration is not a feasible register assignment realization",
            ));
        }
        let mut assignment = vec![false; self.num_vars];
        let compact_vars = self.source_variables.len();
        for (compact, &original) in self.source_variables.iter().enumerate() {
            assignment[original] = target_solution[s_pos_idx(compact)]
                < target_solution[s_neg_idx(compact_vars, compact)];
        }
        Ok(assignment)
    }
}

#[reduction(
    transform = upper_bound {
        num_vertices = "2 * num_vars + 12 * num_clauses",
        num_arcs = "15 * num_clauses",
        num_registers = "num_vars + 9 * num_clauses",
        num_same_register_pairs = "num_vars + 3 * num_clauses",
    }
)]
impl ReduceTo<FeasibleRegisterAssignment> for KSatisfiability<K3> {
    type Result = Reduction3SATToFeasibleRegisterAssignment;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        if self
            .clauses()
            .iter()
            .any(|clause| clause.literals.is_empty())
        {
            // Both predecessors must remain live until vertex 2, but they
            // share a register. This acyclic target has no realization.
            return Ok(Reduction3SATToFeasibleRegisterAssignment {
                target: FeasibleRegisterAssignment::new(3, vec![(2, 0), (2, 1)], 2, vec![0, 0, 1]),
                num_vars: self.num_vars(),
                source_variables: Vec::new(),
            });
        }
        let source_variables: Vec<_> = self
            .clauses()
            .iter()
            .flat_map(|clause| clause.literals.iter())
            .map(|literal| {
                usize::try_from(literal.unsigned_abs()).expect("native SAT indices fit usize") - 1
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let num_vars = source_variables.len();
        let num_clauses = self.num_clauses();
        let overflow = |operation| {
            crate::rules::ReductionError::integer_overflow::<Self, FeasibleRegisterAssignment>(
                operation,
            )
        };
        let variable_vertices = num_vars
            .checked_mul(2)
            .ok_or_else(|| overflow("counting variable leaves"))?;
        let clause_vertices = num_clauses
            .checked_mul(12)
            .ok_or_else(|| overflow("counting clause vertices"))?;
        let num_vertices = variable_vertices
            .checked_add(clause_vertices)
            .ok_or_else(|| overflow("counting target vertices"))?;
        let clause_registers = num_clauses
            .checked_mul(9)
            .ok_or_else(|| overflow("counting clause registers"))?;
        let num_registers = num_vars
            .checked_add(clause_registers)
            .ok_or_else(|| overflow("counting target registers"))?;
        let num_arcs = num_clauses
            .checked_mul(15)
            .ok_or_else(|| overflow("counting target arcs"))?;
        let mut assignment = vec![0usize; num_vertices];
        let mut arcs = Vec::with_capacity(num_arcs);

        for var in 0..num_vars {
            assignment[s_pos_idx(var)] = var;
            assignment[s_neg_idx(num_vars, var)] = var;
        }

        for (clause_idx, clause) in self.clauses().iter().enumerate() {
            // Repetition preserves a nonempty disjunction. Every occurrence
            // keeps its own gadget and register pair, even for repeated literals.
            let mut literals = clause.literals.clone();
            literals.resize(3, literals[0]);
            for literal_pos in 0..3 {
                assignment[p_idx(num_vars, clause_idx, literal_pos)] =
                    p_register(num_vars, clause_idx, literal_pos);
                assignment[q_idx(num_vars, clause_idx, literal_pos)] =
                    q_register(num_vars, clause_idx, literal_pos);
                assignment[r_idx(num_vars, clause_idx, literal_pos)] =
                    r_register(num_vars, clause_idx, literal_pos);
                assignment[rbar_idx(num_vars, clause_idx, literal_pos)] =
                    r_register(num_vars, clause_idx, literal_pos);

                arcs.push((
                    q_idx(num_vars, clause_idx, literal_pos),
                    p_idx(num_vars, clause_idx, literal_pos),
                ));
                arcs.push((
                    p_idx(num_vars, clause_idx, literal_pos),
                    r_idx(num_vars, clause_idx, literal_pos),
                ));
            }

            arcs.push((
                q_idx(num_vars, clause_idx, 0),
                rbar_idx(num_vars, clause_idx, 1),
            ));
            arcs.push((
                q_idx(num_vars, clause_idx, 1),
                rbar_idx(num_vars, clause_idx, 2),
            ));
            arcs.push((
                q_idx(num_vars, clause_idx, 2),
                rbar_idx(num_vars, clause_idx, 0),
            ));

            for (literal_pos, &literal) in literals.iter().enumerate() {
                let original = usize::try_from(literal.unsigned_abs())
                    .expect("native SAT indices fit usize")
                    - 1;
                let var = source_variables
                    .binary_search(&original)
                    .expect("all appearing variables were collected");
                let (literal_leaf, opposite_leaf) = if literal > 0 {
                    (s_pos_idx(var), s_neg_idx(num_vars, var))
                } else {
                    (s_neg_idx(num_vars, var), s_pos_idx(var))
                };
                arcs.push((r_idx(num_vars, clause_idx, literal_pos), literal_leaf));
                arcs.push((rbar_idx(num_vars, clause_idx, literal_pos), opposite_leaf));
            }
        }

        Ok(Reduction3SATToFeasibleRegisterAssignment {
            target: FeasibleRegisterAssignment::new(num_vertices, arcs, num_registers, assignment),
            num_vars: self.num_vars(),
            source_variables,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::algebraic::ILP;
    use crate::models::formula::CNFClause;
    use crate::solvers::ILPSolver;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_feasibleregisterassignment",
        build: || {
            let source = KSatisfiability::<K3>::new(
                3,
                vec![
                    CNFClause::new(vec![1, -2, 3]),
                    CNFClause::new(vec![-1, 2, -3]),
                ],
            );
            let to_fra =
                <KSatisfiability<K3> as ReduceTo<FeasibleRegisterAssignment>>::reduce_to(&source)
                    .expect("reduction should succeed");
            let to_ilp = <FeasibleRegisterAssignment as ReduceTo<ILP<i64>>>::reduce_to(
                to_fra.target_problem(),
            )
            .expect("reduction should succeed");
            let ilp_solution = ILPSolver::new()
                .solve(to_ilp.target_problem())
                .expect("canonical FRA example must reduce to a feasible ILP");
            let target_config = to_ilp.extract_solution(&ilp_solution).unwrap();
            let source_config = to_fra.extract_solution(&target_config).unwrap();
            crate::example_db::specs::assemble_rule_example(
                &source,
                to_fra.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::to_value(source_config)
                        .expect("solution serialization must succeed"),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_feasibleregisterassignment.rs"]
mod tests;
