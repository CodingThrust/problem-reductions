//! Reduction from KSatisfiability (3-SAT) to RegisterSufficiency.
//!
//! This is Sethi's Reduction I / Theorem 3.11 (STOC 1973), with native
//! empty-formula/empty-clause boundary targets, compact variables, and literal
//! repetition for nonempty short clauses. The corrected extraction
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
use std::collections::BTreeSet;

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
    arc_capacity: usize,
    register_bound: usize,
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
impl SethiRegisterLayout {
    fn new(num_vars: usize, num_clauses: usize) -> Result<Self, crate::rules::ReductionError> {
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<KSatisfiability<K3>, RegisterSufficiency>(
                "computing Sethi layout dimensions",
            )
        };
        let twice_vars = num_vars.checked_mul(2).ok_or_else(overflow)?;
        let square = num_vars.checked_mul(num_vars).ok_or_else(overflow)?;
        let b_padding = twice_vars.saturating_sub(num_clauses);
        let total_vertices = square
            .checked_mul(3)
            .and_then(|size| size.checked_add(num_vars.checked_mul(9)?))
            .and_then(|size| size.checked_add(num_clauses.checked_mul(4)?))
            .and_then(|size| size.checked_add(b_padding))
            .and_then(|size| size.checked_add(4))
            .ok_or_else(overflow)?;
        let arc_capacity = square
            .checked_mul(6)
            .and_then(|size| size.checked_add(num_vars.checked_mul(19)?))
            .and_then(|size| size.checked_add(num_clauses.checked_mul(16)?))
            .and_then(|size| size.checked_add(b_padding.checked_mul(2)?))
            .and_then(|size| size.checked_add(1))
            .ok_or_else(overflow)?;
        let register_bound = num_clauses
            .checked_mul(3)
            .and_then(|size| size.checked_add(num_vars.checked_mul(4)?))
            .and_then(|size| size.checked_add(b_padding))
            .and_then(|size| size.checked_add(1))
            .ok_or_else(overflow)?;
        // Every following prefix is a sum of nonnegative block sizes bounded
        // by the checked total; twice_vars also bounds the per-variable indices.
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
        debug_assert_eq!(next, total_vertices);

        Ok(Self {
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
            total_vertices,
            arc_capacity,
            register_bound,
        })
    }

    fn total_vertices(&self) -> usize {
        self.total_vertices
    }

    fn bound(&self) -> usize {
        self.register_bound
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
    #[cfg(any(test, feature = "example-db"))]
    fn schedule_for_assignment(&self, assignment: &[bool]) -> Vec<usize> {
        assert_eq!(assignment.len(), self.num_vars);
        let mut order = Vec::with_capacity(self.total_vertices);
        order.extend((0..2 * self.num_vars + 1).map(|i| self.a(i)));
        order.extend((0..self.b_padding).map(|i| self.bnode(i)));
        for clause in 0..self.num_clauses {
            order.extend((0..3).map(|position| self.f(clause, position)));
        }
        for variable in 0..self.num_vars {
            order.extend((0..2).map(|slot| self.u(variable, slot)));
        }
        order.push(self.initial());
        for (variable, &positive) in assignment.iter().enumerate() {
            order.extend((0..2 * self.num_vars - 2 * variable).map(|i| self.r(variable, i)));
            order.push(self.z(variable));
            order.extend((0..2 * self.num_vars - 2 * variable - 1).map(|i| {
                if positive {
                    self.s(variable, i)
                } else {
                    self.t(variable, i)
                }
            }));
            order.push(if positive {
                self.x_pos(variable)
            } else {
                self.x_neg(variable)
            });
            order.push(self.w(variable));
        }
        order.extend((0..self.num_clauses).map(|clause| self.c(clause)));
        order.push(self.d());
        for (variable, &positive) in assignment.iter().enumerate() {
            order.extend((0..2 * self.num_vars - 2 * variable - 1).map(|i| {
                if positive {
                    self.t(variable, i)
                } else {
                    self.s(variable, i)
                }
            }));
            order.push(if positive {
                self.x_neg(variable)
            } else {
                self.x_pos(variable)
            });
        }
        order.push(self.final_node());
        assert_eq!(order.len(), self.total_vertices);
        let mut positions = vec![0; self.total_vertices];
        for (position, vertex) in order.into_iter().enumerate() {
            positions[vertex] = position;
        }
        positions
    }
}

#[derive(Debug, Clone)]
pub struct Reduction3SATToRegisterSufficiency {
    target: RegisterSufficiency,
    layout: Option<SethiRegisterLayout>,
    source_num_vars: usize,
    source_variables: Vec<usize>,
}

impl ReductionResult for Reduction3SATToRegisterSufficiency {
    type Source = KSatisfiability<K3>;
    type Target = RegisterSufficiency;

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
                "target ordering does not satisfy the register bound and dependencies",
            ));
        }
        let mut assignment = vec![false; self.source_num_vars];
        let Some(layout) = &self.layout else {
            // Only the empty-conjunction target has a feasible witness here.
            return Ok(assignment);
        };
        let cutoff = target_solution[layout.w(layout.num_vars - 1)];
        for (variable, &original) in self.source_variables.iter().enumerate() {
            let positive = target_solution[layout.x_pos(variable)] < cutoff;
            let negative = target_solution[layout.x_neg(variable)] < cutoff;
            if positive && negative {
                return Err(crate::rules::ExtractionError::invalid(format!(
                    "both literals of variable {original} precede the extraction cutoff"
                )));
            }
            assignment[original] = positive;
        }
        Ok(assignment)
    }
}

#[reduction(
    transform = upper_bound {
        num_vertices = "3 * num_vars^2 + 11 * num_vars + 4 * num_clauses + 4",
        num_arcs = "6 * num_vars^2 + 23 * num_vars + 16 * num_clauses + 1",
        bound = "6 * num_vars + 3 * num_clauses + 1",
        num_sinks = "1",
    }
)]
impl ReduceTo<RegisterSufficiency> for KSatisfiability<K3> {
    type Result = Reduction3SATToRegisterSufficiency;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let empty_clause = self
            .clauses()
            .iter()
            .any(|clause| clause.literals.is_empty());
        if empty_clause || self.num_clauses() == 0 {
            // Zero vertices need zero registers (YES); one output vertex
            // cannot be computed with zero registers (NO).
            return Ok(Reduction3SATToRegisterSufficiency {
                target: RegisterSufficiency::new(usize::from(empty_clause), Vec::new(), 0),
                layout: None,
                source_num_vars: self.num_vars(),
                source_variables: Vec::new(),
            });
        }
        let source_variables: Vec<_> = self
            .clauses()
            .iter()
            .flat_map(|clause| &clause.literals)
            .map(|literal| {
                usize::try_from(literal.unsigned_abs())
                    .expect("native SAT variable indices fit usize")
                    - 1
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let clauses: Vec<_> = self
            .clauses()
            .iter()
            .map(|clause| {
                let mut literals = [0i64; 3];
                for (position, &literal) in clause.literals.iter().enumerate() {
                    let original = usize::try_from(literal.unsigned_abs())
                        .expect("native SAT variable indices fit usize")
                        - 1;
                    let compact = source_variables
                        .binary_search(&original)
                        .expect("all appearing SAT variables were collected");
                    let variable = i64::try_from(compact + 1).expect("compact SAT indices fit i64");
                    literals[position] = if literal > 0 { variable } else { -variable };
                }
                // Repeating a literal preserves a nonempty disjunction. Every
                // slot has a real literal and the original first-true lock proof.
                for position in clause.literals.len()..3 {
                    literals[position] = literals[0];
                }
                literals
            })
            .collect();
        let num_vars = source_variables.len();
        let num_clauses = clauses.len();
        let layout = SethiRegisterLayout::new(num_vars, num_clauses)?;
        let mut arcs = Vec::with_capacity(layout.arc_capacity);

        for index in 0..(2 * num_vars + 1) {
            arcs.push((layout.initial(), layout.a(index)));
        }
        for index in 0..layout.b_padding {
            arcs.push((layout.initial(), layout.bnode(index)));
        }
        for clause in 0..num_clauses {
            for literal_pos in 0..3 {
                arcs.push((layout.initial(), layout.f(clause, literal_pos)));
            }
        }
        for var in 0..num_vars {
            arcs.push((layout.initial(), layout.u(var, 0)));
            arcs.push((layout.initial(), layout.u(var, 1)));
        }

        for clause in 0..num_clauses {
            arcs.push((layout.c(clause), layout.initial()));
        }
        for var in 0..num_vars {
            for index in 0..(2 * num_vars - 2 * var) {
                arcs.push((layout.r(var, index), layout.initial()));
            }
            for index in 0..(2 * num_vars - 2 * var - 1) {
                arcs.push((layout.s(var, index), layout.initial()));
                arcs.push((layout.t(var, index), layout.initial()));
            }
            arcs.push((layout.w(var), layout.initial()));
        }

        for var in 0..num_vars {
            arcs.push((layout.final_node(), layout.w(var)));
            arcs.push((layout.final_node(), layout.x_pos(var)));
            arcs.push((layout.final_node(), layout.x_neg(var)));
            arcs.push((layout.final_node(), layout.z(var)));
        }
        arcs.push((layout.final_node(), layout.initial()));
        arcs.push((layout.final_node(), layout.d()));

        for var in 0..num_vars {
            arcs.push((layout.x_pos(var), layout.z(var)));
            arcs.push((layout.x_neg(var), layout.z(var)));
            arcs.push((layout.x_pos(var), layout.u(var, 0)));
            arcs.push((layout.x_neg(var), layout.u(var, 1)));
        }

        for var in 0..num_vars {
            arcs.push((layout.w(var), layout.u(var, 0)));
            arcs.push((layout.w(var), layout.u(var, 1)));
        }

        for var in 0..num_vars {
            for index in 0..(2 * num_vars - 2 * var - 1) {
                arcs.push((layout.x_pos(var), layout.s(var, index)));
                arcs.push((layout.x_neg(var), layout.t(var, index)));
            }
            for index in 0..(2 * num_vars - 2 * var) {
                arcs.push((layout.z(var), layout.r(var, index)));
            }
        }

        for var in 1..num_vars {
            arcs.push((layout.z(var), layout.w(var - 1)));
            arcs.push((layout.z(var), layout.z(var - 1)));
        }
        if num_vars > 0 {
            let last_var = num_vars - 1;
            for clause in 0..num_clauses {
                arcs.push((layout.c(clause), layout.w(last_var)));
                arcs.push((layout.c(clause), layout.z(last_var)));
            }
        }

        for clause in 0..num_clauses {
            for literal_pos in 0..3 {
                arcs.push((layout.c(clause), layout.f(clause, literal_pos)));
            }
        }

        for index in 0..layout.b_padding {
            arcs.push((layout.d(), layout.bnode(index)));
        }
        for clause in 0..num_clauses {
            arcs.push((layout.d(), layout.c(clause)));
        }

        for (clause_idx, clause) in clauses.iter().enumerate() {
            let mut lit_nodes = [0usize; 3];
            let mut neg_nodes = [0usize; 3];

            for (literal_pos, &literal) in clause.iter().enumerate() {
                let var = usize::try_from(literal.unsigned_abs())
                    .expect("compact SAT variable indices fit usize")
                    - 1;
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

        Ok(Reduction3SATToRegisterSufficiency {
            target: RegisterSufficiency::new(layout.total_vertices(), arcs, layout.bound()),
            layout: Some(layout),
            source_num_vars: self.num_vars(),
            source_variables,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

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
                <KSatisfiability<K3> as ReduceTo<RegisterSufficiency>>::reduce_to(&source)
                    .expect("reduction should succeed");

            let target_config = to_registers
                .layout
                .as_ref()
                .expect("canonical formula has nonempty clauses")
                .schedule_for_assignment(&[true, true, true]);
            let source_config = to_registers.extract_solution(&target_config).unwrap();

            crate::example_db::specs::assemble_rule_example(
                &source,
                to_registers.target_problem(),
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
#[path = "../unit_tests/rules/ksatisfiability_registersufficiency.rs"]
mod tests;
