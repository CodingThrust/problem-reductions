//! Reduction from KSatisfiability (3-SAT) to RegisterSufficiency.
//!
//! This is Sethi's Reduction I / Theorem 3.11 with the corrected extraction
//! rule from issue #872:
//! - the snapshot is taken immediately after `w[n]`
//! - `x_k = true` iff `x_pos[k]` has been computed by that snapshot
//! - at most one of `x_pos[k]`, `x_neg[k]` can have been computed by then
//! - the literal/clause edges keep Sethi's original orientation

use crate::models::formula::KSatisfiability;
use crate::models::misc::RegisterSufficiency;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct SethiRegisterLayout {
    num_vars: usize,
    num_clauses: usize,
    b_padding: usize,
    a_start: usize,
    b_start: usize,
    c_start: usize,
    f_start: usize,
    initial_idx: usize,
    d_idx: usize,
    final_idx: usize,
    r_starts: Vec<usize>,
    s_starts: Vec<usize>,
    t_starts: Vec<usize>,
    u_start: usize,
    w_start: usize,
    x_pos_start: usize,
    x_neg_start: usize,
    z_start: usize,
    total_vertices: usize,
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
impl SethiRegisterLayout {
    fn new(num_vars: usize, num_clauses: usize) -> Self {
        let b_padding = (2 * num_vars).saturating_sub(num_clauses);
        let mut next = 0usize;

        let a_start = next;
        next += 2 * num_vars + 1;

        let b_start = next;
        next += b_padding;

        let c_start = next;
        next += num_clauses;

        let f_start = next;
        next += 3 * num_clauses;

        let initial_idx = next;
        let d_idx = next + 1;
        let final_idx = next + 2;
        next += 3;

        let mut r_starts = Vec::with_capacity(num_vars);
        for var in 0..num_vars {
            r_starts.push(next);
            next += 2 * num_vars - 2 * var;
        }

        let mut s_starts = Vec::with_capacity(num_vars);
        for var in 0..num_vars {
            s_starts.push(next);
            next += 2 * num_vars - 2 * var - 1;
        }

        let mut t_starts = Vec::with_capacity(num_vars);
        for var in 0..num_vars {
            t_starts.push(next);
            next += 2 * num_vars - 2 * var - 1;
        }

        let u_start = next;
        next += 2 * num_vars;

        let w_start = next;
        next += num_vars;

        let x_pos_start = next;
        next += num_vars;

        let x_neg_start = next;
        next += num_vars;

        let z_start = next;
        next += num_vars;

        Self {
            num_vars,
            num_clauses,
            b_padding,
            a_start,
            b_start,
            c_start,
            f_start,
            initial_idx,
            d_idx,
            final_idx,
            r_starts,
            s_starts,
            t_starts,
            u_start,
            w_start,
            x_pos_start,
            x_neg_start,
            z_start,
            total_vertices: next,
        }
    }

    fn total_vertices(&self) -> usize {
        self.total_vertices
    }

    fn bound(&self) -> usize {
        3 * self.num_clauses + 4 * self.num_vars + 1 + self.b_padding
    }

    fn initial(&self) -> usize {
        self.initial_idx
    }

    fn d(&self) -> usize {
        self.d_idx
    }

    fn final_node(&self) -> usize {
        self.final_idx
    }

    fn a(&self, index: usize) -> usize {
        self.a_start + index
    }

    fn bnode(&self, index: usize) -> usize {
        self.b_start + index
    }

    fn c(&self, clause: usize) -> usize {
        self.c_start + clause
    }

    fn f(&self, clause: usize, literal_pos: usize) -> usize {
        self.f_start + 3 * clause + literal_pos
    }

    fn r(&self, var: usize, index: usize) -> usize {
        self.r_starts[var] + index
    }

    fn s(&self, var: usize, index: usize) -> usize {
        self.s_starts[var] + index
    }

    fn t(&self, var: usize, index: usize) -> usize {
        self.t_starts[var] + index
    }

    fn u(&self, var: usize, slot: usize) -> usize {
        self.u_start + 2 * var + slot
    }

    fn w(&self, var: usize) -> usize {
        self.w_start + var
    }

    fn x_pos(&self, var: usize) -> usize {
        self.x_pos_start + var
    }

    fn x_neg(&self, var: usize) -> usize {
        self.x_neg_start + var
    }

    fn z(&self, var: usize) -> usize {
        self.z_start + var
    }
}

#[derive(Debug, Clone)]
pub struct Reduction3SATToRegisterSufficiency {
    target: RegisterSufficiency,
    layout: SethiRegisterLayout,
}

impl ReductionResult for Reduction3SATToRegisterSufficiency {
    type Source = KSatisfiability<K3>;
    type Target = RegisterSufficiency;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        if self.layout.num_vars == 0 {
            return Vec::new();
        }

        let cutoff = target_solution[self.layout.w(self.layout.num_vars - 1)];
        (0..self.layout.num_vars)
            .map(|var| {
                let x_pos_before = target_solution[self.layout.x_pos(var)] < cutoff;
                let x_neg_before = target_solution[self.layout.x_neg(var)] < cutoff;
                debug_assert!(
                    !(x_pos_before && x_neg_before),
                    "Sethi extraction expects at most one of x_pos/x_neg before w[n]",
                );
                usize::from(x_pos_before)
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_vertices = "3 * num_vars^2 + 9 * num_vars + 4 * num_clauses + register_sufficiency_padding + 4",
    num_arcs = "6 * num_vars^2 + 19 * num_vars + 16 * num_clauses + 2 * register_sufficiency_padding + 1",
    bound = "3 * num_clauses + 4 * num_vars + 1 + register_sufficiency_padding",
})]
impl ReduceTo<RegisterSufficiency> for KSatisfiability<K3> {
    type Result = Reduction3SATToRegisterSufficiency;

    fn reduce_to(&self) -> Self::Result {
        let layout = SethiRegisterLayout::new(self.num_vars(), self.num_clauses());
        let mut arcs = Vec::with_capacity(
            6 * self.num_vars() * self.num_vars()
                + 19 * self.num_vars()
                + 16 * self.num_clauses()
                + 2 * layout.b_padding
                + 1,
        );

        for index in 0..(2 * self.num_vars() + 1) {
            arcs.push((layout.initial(), layout.a(index)));
        }
        for index in 0..layout.b_padding {
            arcs.push((layout.initial(), layout.bnode(index)));
        }
        for clause in 0..self.num_clauses() {
            for literal_pos in 0..3 {
                arcs.push((layout.initial(), layout.f(clause, literal_pos)));
            }
        }
        for var in 0..self.num_vars() {
            arcs.push((layout.initial(), layout.u(var, 0)));
            arcs.push((layout.initial(), layout.u(var, 1)));
        }

        for clause in 0..self.num_clauses() {
            arcs.push((layout.c(clause), layout.initial()));
        }
        for var in 0..self.num_vars() {
            for index in 0..(2 * self.num_vars() - 2 * var) {
                arcs.push((layout.r(var, index), layout.initial()));
            }
            for index in 0..(2 * self.num_vars() - 2 * var - 1) {
                arcs.push((layout.s(var, index), layout.initial()));
                arcs.push((layout.t(var, index), layout.initial()));
            }
            arcs.push((layout.w(var), layout.initial()));
        }

        for var in 0..self.num_vars() {
            arcs.push((layout.final_node(), layout.w(var)));
            arcs.push((layout.final_node(), layout.x_pos(var)));
            arcs.push((layout.final_node(), layout.x_neg(var)));
            arcs.push((layout.final_node(), layout.z(var)));
        }
        arcs.push((layout.final_node(), layout.initial()));
        arcs.push((layout.final_node(), layout.d()));

        for var in 0..self.num_vars() {
            arcs.push((layout.x_pos(var), layout.z(var)));
            arcs.push((layout.x_neg(var), layout.z(var)));
            arcs.push((layout.x_pos(var), layout.u(var, 0)));
            arcs.push((layout.x_neg(var), layout.u(var, 1)));
        }

        for var in 0..self.num_vars() {
            arcs.push((layout.w(var), layout.u(var, 0)));
            arcs.push((layout.w(var), layout.u(var, 1)));
        }

        for var in 0..self.num_vars() {
            for index in 0..(2 * self.num_vars() - 2 * var - 1) {
                arcs.push((layout.x_pos(var), layout.s(var, index)));
                arcs.push((layout.x_neg(var), layout.t(var, index)));
            }
            for index in 0..(2 * self.num_vars() - 2 * var) {
                arcs.push((layout.z(var), layout.r(var, index)));
            }
        }

        for var in 1..self.num_vars() {
            arcs.push((layout.z(var), layout.w(var - 1)));
            arcs.push((layout.z(var), layout.z(var - 1)));
        }
        if self.num_vars() > 0 {
            let last_var = self.num_vars() - 1;
            for clause in 0..self.num_clauses() {
                arcs.push((layout.c(clause), layout.w(last_var)));
                arcs.push((layout.c(clause), layout.z(last_var)));
            }
        }

        for clause in 0..self.num_clauses() {
            for literal_pos in 0..3 {
                arcs.push((layout.c(clause), layout.f(clause, literal_pos)));
            }
        }

        for index in 0..layout.b_padding {
            arcs.push((layout.d(), layout.bnode(index)));
        }
        for clause in 0..self.num_clauses() {
            arcs.push((layout.d(), layout.c(clause)));
        }

        for (clause_idx, clause) in self.clauses().iter().enumerate() {
            let mut lit_nodes = [0usize; 3];
            let mut neg_nodes = [0usize; 3];

            for (literal_pos, &literal) in clause.literals.iter().enumerate() {
                let var = literal.unsigned_abs() as usize - 1;
                if literal > 0 {
                    lit_nodes[literal_pos] = layout.x_pos(var);
                    neg_nodes[literal_pos] = layout.x_neg(var);
                } else {
                    lit_nodes[literal_pos] = layout.x_neg(var);
                    neg_nodes[literal_pos] = layout.x_pos(var);
                }
                arcs.push((lit_nodes[literal_pos], layout.f(clause_idx, literal_pos)));
            }

            for (earlier, &neg_node) in neg_nodes.iter().enumerate() {
                for later in (earlier + 1)..3 {
                    arcs.push((neg_node, layout.f(clause_idx, later)));
                }
            }
        }

        Reduction3SATToRegisterSufficiency {
            target: RegisterSufficiency::new(layout.total_vertices(), arcs, layout.bound()),
            layout,
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
        id: "ksatisfiability_to_registersufficiency",
        build: || {
            let source = KSatisfiability::<K3>::new(
                3,
                vec![
                    CNFClause::new(vec![1, -2, 3]),
                    CNFClause::new(vec![-1, 2, -3]),
                ],
            );
            let to_registers =
                <KSatisfiability<K3> as ReduceTo<RegisterSufficiency>>::reduce_to(&source);
            let to_ilp = <RegisterSufficiency as ReduceTo<ILP<i32>>>::reduce_to(
                to_registers.target_problem(),
            );
            let ilp_solution = ILPSolver::new()
                .solve(to_ilp.target_problem())
                .expect("canonical RegisterSufficiency example must reduce to a feasible ILP");
            let target_config = to_ilp.extract_solution(&ilp_solution);
            let source_config = to_registers.extract_solution(&target_config);
            crate::example_db::specs::assemble_rule_example(
                &source,
                to_registers.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_registersufficiency.rs"]
mod tests;
