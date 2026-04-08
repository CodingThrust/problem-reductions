//! Reduction from KSatisfiability (3-SAT) to Feasible Register Assignment.
//!
//! This follows Sethi's Reduction 3 / Theorem 5.11:
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
}

impl ReductionResult for Reduction3SATToFeasibleRegisterAssignment {
    type Source = KSatisfiability<K3>;
    type Target = FeasibleRegisterAssignment;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        (0..self.num_vars)
            .map(|var| {
                usize::from(
                    target_solution[s_pos_idx(var)]
                        < target_solution[s_neg_idx(self.num_vars, var)],
                )
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_vertices = "2 * num_vars + 12 * num_clauses",
    num_arcs = "15 * num_clauses",
    num_registers = "num_vars + 9 * num_clauses",
})]
impl ReduceTo<FeasibleRegisterAssignment> for KSatisfiability<K3> {
    type Result = Reduction3SATToFeasibleRegisterAssignment;

    fn reduce_to(&self) -> Self::Result {
        let num_vars = self.num_vars();
        let num_clauses = self.num_clauses();
        let num_vertices = 2 * num_vars + 12 * num_clauses;
        let num_registers = num_vars + 9 * num_clauses;
        let mut assignment = vec![0usize; num_vertices];
        let mut arcs = Vec::with_capacity(15 * num_clauses);

        for var in 0..num_vars {
            assignment[s_pos_idx(var)] = var;
            assignment[s_neg_idx(num_vars, var)] = var;
        }

        for (clause_idx, clause) in self.clauses().iter().enumerate() {
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

            for (literal_pos, &literal) in clause.literals.iter().enumerate() {
                let var = literal.unsigned_abs() as usize - 1;
                let (literal_leaf, opposite_leaf) = if literal > 0 {
                    (s_pos_idx(var), s_neg_idx(num_vars, var))
                } else {
                    (s_neg_idx(num_vars, var), s_pos_idx(var))
                };
                arcs.push((r_idx(num_vars, clause_idx, literal_pos), literal_leaf));
                arcs.push((rbar_idx(num_vars, clause_idx, literal_pos), opposite_leaf));
            }
        }

        Reduction3SATToFeasibleRegisterAssignment {
            target: FeasibleRegisterAssignment::new(num_vertices, arcs, num_registers, assignment),
            num_vars,
        }
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
                <KSatisfiability<K3> as ReduceTo<FeasibleRegisterAssignment>>::reduce_to(&source);
            let to_ilp = <FeasibleRegisterAssignment as ReduceTo<ILP<i32>>>::reduce_to(
                to_fra.target_problem(),
            );
            let ilp_solution = ILPSolver::new()
                .solve(to_ilp.target_problem())
                .expect("canonical FRA example must reduce to a feasible ILP");
            let target_config = to_ilp.extract_solution(&ilp_solution);
            let source_config = to_fra.extract_solution(&target_config);
            crate::example_db::specs::assemble_rule_example(
                &source,
                to_fra.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_feasibleregisterassignment.rs"]
mod tests;
