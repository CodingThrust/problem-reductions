//! Reduction from CircuitSAT to Satisfiability via Tseitin encoding.

use crate::models::formula::{
    Assignment, BooleanExpr, BooleanOp, CNFClause, CircuitSAT, Satisfiability,
};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum NormalizedExpr {
    Var(String),
    Const(bool),
    Not(Box<NormalizedExpr>),
    And(Box<NormalizedExpr>, Box<NormalizedExpr>),
    Or(Box<NormalizedExpr>, Box<NormalizedExpr>),
    Xor(Box<NormalizedExpr>, Box<NormalizedExpr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EncodedTerm {
    Const(bool),
    Var(i32),
}

#[derive(Debug, Clone)]
struct TseitinEncoding {
    num_vars: usize,
    clauses: Vec<CNFClause>,
}

#[derive(Debug)]
struct TseitinEncoder {
    source_var_ids: HashMap<String, i32>,
    clauses: Vec<CNFClause>,
    next_var: i32,
}

impl TseitinEncoder {
    fn new(source: &CircuitSAT) -> Self {
        let source_var_ids = source
            .variable_names()
            .iter()
            .enumerate()
            .map(|(index, name)| (name.clone(), index as i32 + 1))
            .collect();
        Self {
            source_var_ids,
            clauses: Vec::new(),
            next_var: source.num_variables() as i32 + 1,
        }
    }

    fn encode_problem(mut self, source: &CircuitSAT) -> TseitinEncoding {
        for assignment in &source.circuit().assignments {
            self.encode_assignment(assignment);
        }

        TseitinEncoding {
            num_vars: (self.next_var - 1) as usize,
            clauses: self.clauses,
        }
    }

    fn encode_assignment(&mut self, assignment: &Assignment) {
        if assignment.outputs.is_empty() {
            return;
        }

        let root = self.encode_expr(&normalize_expr(&assignment.expr));
        match root {
            EncodedTerm::Const(value) => {
                let literal_sign = if value { 1 } else { -1 };
                for output in &assignment.outputs {
                    let output_var = self.source_var(output);
                    self.push_clause(vec![literal_sign * output_var]);
                }
            }
            EncodedTerm::Var(root_var) => {
                for output in &assignment.outputs {
                    let output_var = self.source_var(output);
                    self.push_equivalence(output_var, root_var);
                }
            }
        }
    }

    fn encode_expr(&mut self, expr: &NormalizedExpr) -> EncodedTerm {
        match expr {
            NormalizedExpr::Var(name) => EncodedTerm::Var(self.source_var(name)),
            NormalizedExpr::Const(value) => EncodedTerm::Const(*value),
            NormalizedExpr::Not(inner) => match self.encode_expr(inner) {
                EncodedTerm::Const(value) => EncodedTerm::Const(!value),
                EncodedTerm::Var(child_var) => {
                    let gate_var = self.allocate_auxiliary_var();
                    self.push_clause(vec![-gate_var, -child_var]);
                    self.push_clause(vec![gate_var, child_var]);
                    EncodedTerm::Var(gate_var)
                }
            },
            NormalizedExpr::And(left, right) => {
                let left_term = self.encode_expr(left);
                let right_term = self.encode_expr(right);
                let left_var = self.expect_var(left_term, "AND left input");
                let right_var = self.expect_var(right_term, "AND right input");
                let gate_var = self.allocate_auxiliary_var();
                self.push_clause(vec![-gate_var, left_var]);
                self.push_clause(vec![-gate_var, right_var]);
                self.push_clause(vec![gate_var, -left_var, -right_var]);
                EncodedTerm::Var(gate_var)
            }
            NormalizedExpr::Or(left, right) => {
                let left_term = self.encode_expr(left);
                let right_term = self.encode_expr(right);
                let left_var = self.expect_var(left_term, "OR left input");
                let right_var = self.expect_var(right_term, "OR right input");
                let gate_var = self.allocate_auxiliary_var();
                self.push_clause(vec![gate_var, -left_var]);
                self.push_clause(vec![gate_var, -right_var]);
                self.push_clause(vec![-gate_var, left_var, right_var]);
                EncodedTerm::Var(gate_var)
            }
            NormalizedExpr::Xor(left, right) => {
                let left_term = self.encode_expr(left);
                let right_term = self.encode_expr(right);
                let left_var = self.expect_var(left_term, "XOR left input");
                let right_var = self.expect_var(right_term, "XOR right input");
                let gate_var = self.allocate_auxiliary_var();
                self.push_clause(vec![-left_var, -right_var, -gate_var]);
                self.push_clause(vec![left_var, right_var, -gate_var]);
                self.push_clause(vec![left_var, -right_var, gate_var]);
                self.push_clause(vec![-left_var, right_var, gate_var]);
                EncodedTerm::Var(gate_var)
            }
        }
    }

    fn expect_var(&self, term: EncodedTerm, context: &str) -> i32 {
        match term {
            EncodedTerm::Var(var) => var,
            EncodedTerm::Const(_) => {
                panic!("normalized Tseitin encoding produced a constant for {context}")
            }
        }
    }

    fn source_var(&self, name: &str) -> i32 {
        *self
            .source_var_ids
            .get(name)
            .unwrap_or_else(|| panic!("CircuitSAT variable {name:?} missing from source ordering"))
    }

    fn allocate_auxiliary_var(&mut self) -> i32 {
        let var = self.next_var;
        self.next_var += 1;
        var
    }

    fn push_equivalence(&mut self, left: i32, right: i32) {
        self.push_clause(vec![-left, right]);
        self.push_clause(vec![left, -right]);
    }

    fn push_clause(&mut self, literals: Vec<i32>) {
        self.clauses.push(CNFClause::new(literals));
    }
}

fn make_and(left: NormalizedExpr, right: NormalizedExpr) -> NormalizedExpr {
    NormalizedExpr::And(Box::new(left), Box::new(right))
}

fn make_or(left: NormalizedExpr, right: NormalizedExpr) -> NormalizedExpr {
    NormalizedExpr::Or(Box::new(left), Box::new(right))
}

fn make_xor(left: NormalizedExpr, right: NormalizedExpr) -> NormalizedExpr {
    NormalizedExpr::Xor(Box::new(left), Box::new(right))
}

fn build_balanced_expr(
    mut items: Vec<NormalizedExpr>,
    combine: fn(NormalizedExpr, NormalizedExpr) -> NormalizedExpr,
) -> NormalizedExpr {
    if items.len() == 1 {
        return items.pop().expect("single item exists");
    }

    let right = items.split_off(items.len() / 2);
    combine(
        build_balanced_expr(items, combine),
        build_balanced_expr(right, combine),
    )
}

fn normalize_expr(expr: &BooleanExpr) -> NormalizedExpr {
    match &expr.op {
        BooleanOp::Var(name) => NormalizedExpr::Var(name.clone()),
        BooleanOp::Const(value) => NormalizedExpr::Const(*value),
        BooleanOp::Not(inner) => match normalize_expr(inner) {
            NormalizedExpr::Const(value) => NormalizedExpr::Const(!value),
            NormalizedExpr::Not(grandchild) => *grandchild,
            normalized => NormalizedExpr::Not(Box::new(normalized)),
        },
        BooleanOp::And(args) => normalize_nary_gate(args, false, true, make_and),
        BooleanOp::Or(args) => normalize_nary_gate(args, true, false, make_or),
        BooleanOp::Xor(args) => {
            let mut parity = false;
            let mut normalized_args = Vec::new();

            for arg in args {
                match normalize_expr(arg) {
                    NormalizedExpr::Const(value) => parity ^= value,
                    normalized => normalized_args.push(normalized),
                }
            }

            match normalized_args.len() {
                0 => NormalizedExpr::Const(parity),
                1 => {
                    let base = normalized_args.pop().expect("single item exists");
                    if parity {
                        NormalizedExpr::Not(Box::new(base))
                    } else {
                        base
                    }
                }
                _ => {
                    let base = build_balanced_expr(normalized_args, make_xor);
                    if parity {
                        NormalizedExpr::Not(Box::new(base))
                    } else {
                        base
                    }
                }
            }
        }
    }
}

fn normalize_nary_gate(
    args: &[BooleanExpr],
    absorbing_value: bool,
    identity_value: bool,
    combine: fn(NormalizedExpr, NormalizedExpr) -> NormalizedExpr,
) -> NormalizedExpr {
    let mut normalized_args = Vec::new();

    for arg in args {
        match normalize_expr(arg) {
            NormalizedExpr::Const(value) if value == absorbing_value => {
                return NormalizedExpr::Const(absorbing_value)
            }
            NormalizedExpr::Const(value) if value == identity_value => {}
            normalized => normalized_args.push(normalized),
        }
    }

    match normalized_args.len() {
        0 => NormalizedExpr::Const(identity_value),
        1 => normalized_args.pop().expect("single item exists"),
        _ => build_balanced_expr(normalized_args, combine),
    }
}

fn build_tseitin_encoding(source: &CircuitSAT) -> TseitinEncoding {
    TseitinEncoder::new(source).encode_problem(source)
}

impl CircuitSAT {
    pub fn tseitin_num_vars(&self) -> usize {
        build_tseitin_encoding(self).num_vars
    }

    pub fn tseitin_num_clauses(&self) -> usize {
        build_tseitin_encoding(self).clauses.len()
    }
}

/// Result of reducing CircuitSAT to SAT.
#[derive(Debug, Clone)]
pub struct ReductionCircuitSATToSAT {
    target: Satisfiability,
    source_var_count: usize,
}

impl ReductionResult for ReductionCircuitSATToSAT {
    type Source = CircuitSAT;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        target_solution
            .iter()
            .take(self.source_var_count)
            .copied()
            .collect()
    }
}

#[reduction(
    overhead = {
        num_vars = "tseitin_num_vars",
        num_clauses = "tseitin_num_clauses",
    }
)]
impl ReduceTo<Satisfiability> for CircuitSAT {
    type Result = ReductionCircuitSATToSAT;

    fn reduce_to(&self) -> Self::Result {
        let encoding = build_tseitin_encoding(self);
        ReductionCircuitSATToSAT {
            target: Satisfiability::new(encoding.num_vars, encoding.clauses),
            source_var_count: self.num_variables(),
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
fn issue_example_source() -> CircuitSAT {
    use crate::models::formula::Circuit;

    CircuitSAT::new(Circuit::new(vec![Assignment::new(
        vec!["r".to_string()],
        BooleanExpr::or(vec![
            BooleanExpr::and(vec![BooleanExpr::var("x1"), BooleanExpr::var("x2")]),
            BooleanExpr::and(vec![
                BooleanExpr::not(BooleanExpr::var("x3")),
                BooleanExpr::var("x4"),
            ]),
        ]),
    )]))
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::solvers::BruteForce;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "circuitsat_to_satisfiability",
        build: || {
            let source = issue_example_source();
            let source_config = vec![1, 1, 1, 0, 1];
            let reduction = ReduceTo::<Satisfiability>::reduce_to(&source);
            let target_config = BruteForce::new()
                .find_all_witnesses(reduction.target_problem())
                .into_iter()
                .find(|candidate| reduction.extract_solution(candidate) == source_config)
                .expect("canonical CircuitSAT -> Satisfiability example must be satisfiable");

            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/circuit_sat.rs"]
mod tests;
