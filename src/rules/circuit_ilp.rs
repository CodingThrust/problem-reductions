//! Reduction from CircuitSAT to ILP via gate constraint encoding.
//!
//! Each boolean gate is encoded as linear constraints over binary variables.
//! The expression tree is flattened by introducing an auxiliary variable per
//! internal node (Tseitin-style).
//!
//! ## Gate Encodings (all variables binary)
//! - NOT(a) = c:           c + a = 1
//! - AND(a₁,...,aₖ) = c:  c ≤ aᵢ (∀i), c ≥ Σaᵢ - (k-1)
//! - OR(a₁,...,aₖ) = c:   c ≥ aᵢ (∀i), c ≤ Σaᵢ
//! - XOR(a, b) = c:        c ≤ a+b, c ≥ a-b, c ≥ b-a, c ≤ 2-a-b
//! - Const(v) = c:          c = v
//!
//! ## Objective
//! Trivial (minimize 0): any feasible ILP solution is a satisfying assignment.

use crate::models::optimization::{LinearConstraint, ObjectiveSense, VarBounds, ILP};
use crate::models::specialized::{BooleanExpr, BooleanOp, CircuitSAT};
use crate::poly;
use crate::reduction;
use crate::rules::registry::ReductionOverhead;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::HashMap;

/// Result of reducing CircuitSAT to ILP.
#[derive(Debug, Clone)]
pub struct ReductionCircuitToILP {
    target: ILP,
    source_variables: Vec<String>,
    variable_map: HashMap<String, usize>,
}

impl ReductionResult for ReductionCircuitToILP {
    type Source = CircuitSAT;
    type Target = ILP;

    fn target_problem(&self) -> &ILP {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.source_variables
            .iter()
            .map(|name| target_solution[self.variable_map[name]])
            .collect()
    }
}

/// Builder that accumulates ILP variables and constraints.
struct ILPBuilder {
    num_vars: usize,
    constraints: Vec<LinearConstraint>,
    variable_map: HashMap<String, usize>,
}

impl ILPBuilder {
    fn new() -> Self {
        Self {
            num_vars: 0,
            constraints: Vec::new(),
            variable_map: HashMap::new(),
        }
    }

    /// Get or create a variable index for a named circuit variable.
    fn get_or_create_var(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.variable_map.get(name) {
            idx
        } else {
            let idx = self.num_vars;
            self.variable_map.insert(name.to_string(), idx);
            self.num_vars += 1;
            idx
        }
    }

    /// Allocate an anonymous auxiliary variable.
    fn alloc_aux(&mut self) -> usize {
        let idx = self.num_vars;
        self.num_vars += 1;
        idx
    }

    /// Recursively process a BooleanExpr, returning the ILP variable index
    /// that holds the expression's value.
    fn process_expr(&mut self, expr: &BooleanExpr) -> usize {
        match &expr.op {
            BooleanOp::Var(name) => self.get_or_create_var(name),
            BooleanOp::Const(value) => {
                let c = self.alloc_aux();
                let v = if *value { 1.0 } else { 0.0 };
                self.constraints
                    .push(LinearConstraint::eq(vec![(c, 1.0)], v));
                c
            }
            BooleanOp::Not(inner) => {
                let a = self.process_expr(inner);
                let c = self.alloc_aux();
                // c + a = 1
                self.constraints
                    .push(LinearConstraint::eq(vec![(c, 1.0), (a, 1.0)], 1.0));
                c
            }
            BooleanOp::And(args) => {
                let inputs: Vec<usize> = args.iter().map(|a| self.process_expr(a)).collect();
                let c = self.alloc_aux();
                let k = inputs.len() as f64;
                // c ≤ a_i for all i
                for &a_i in &inputs {
                    self.constraints
                        .push(LinearConstraint::le(vec![(c, 1.0), (a_i, -1.0)], 0.0));
                }
                // c ≥ Σa_i - (k - 1)
                let mut terms: Vec<(usize, f64)> = vec![(c, 1.0)];
                for &a_i in &inputs {
                    terms.push((a_i, -1.0));
                }
                self.constraints
                    .push(LinearConstraint::ge(terms, -(k - 1.0)));
                c
            }
            BooleanOp::Or(args) => {
                let inputs: Vec<usize> = args.iter().map(|a| self.process_expr(a)).collect();
                let c = self.alloc_aux();
                // c ≥ a_i for all i
                for &a_i in &inputs {
                    self.constraints
                        .push(LinearConstraint::ge(vec![(c, 1.0), (a_i, -1.0)], 0.0));
                }
                // c ≤ Σa_i
                let mut terms: Vec<(usize, f64)> = vec![(c, 1.0)];
                for &a_i in &inputs {
                    terms.push((a_i, -1.0));
                }
                self.constraints.push(LinearConstraint::le(terms, 0.0));
                c
            }
            BooleanOp::Xor(args) => {
                // Chain pairwise: XOR(a1, a2, a3) = XOR(XOR(a1, a2), a3)
                let inputs: Vec<usize> = args.iter().map(|a| self.process_expr(a)).collect();
                assert!(!inputs.is_empty());
                let mut result = inputs[0];
                for &next in &inputs[1..] {
                    let c = self.alloc_aux();
                    let a = result;
                    let b = next;
                    // c ≤ a + b
                    self.constraints.push(LinearConstraint::le(
                        vec![(c, 1.0), (a, -1.0), (b, -1.0)],
                        0.0,
                    ));
                    // c ≥ a - b
                    self.constraints.push(LinearConstraint::ge(
                        vec![(c, 1.0), (a, -1.0), (b, 1.0)],
                        0.0,
                    ));
                    // c ≥ b - a
                    self.constraints.push(LinearConstraint::ge(
                        vec![(c, 1.0), (a, 1.0), (b, -1.0)],
                        0.0,
                    ));
                    // c ≤ 2 - a - b
                    self.constraints.push(LinearConstraint::le(
                        vec![(c, 1.0), (a, 1.0), (b, 1.0)],
                        2.0,
                    ));
                    result = c;
                }
                result
            }
        }
    }
}

#[reduction(
    overhead = {
        ReductionOverhead::new(vec![
            ("num_vars", poly!(num_variables) + poly!(num_assignments)),
            ("num_constraints", poly!(num_variables) + poly!(num_assignments)),
        ])
    }
)]
impl ReduceTo<ILP> for CircuitSAT {
    type Result = ReductionCircuitToILP;

    fn reduce_to(&self) -> Self::Result {
        let mut builder = ILPBuilder::new();

        // Pre-register all circuit variables to preserve ordering
        for name in self.variable_names() {
            builder.get_or_create_var(name);
        }

        // Process each assignment
        for assignment in &self.circuit().assignments {
            let expr_var = builder.process_expr(&assignment.expr);
            // Constrain each output to equal the expression result
            for output_name in &assignment.outputs {
                let out_var = builder.get_or_create_var(output_name);
                if out_var != expr_var {
                    // out = expr_var
                    builder.constraints.push(LinearConstraint::eq(
                        vec![(out_var, 1.0), (expr_var, -1.0)],
                        0.0,
                    ));
                }
            }
        }

        let bounds = vec![VarBounds::binary(); builder.num_vars];
        // Trivial objective: minimize 0 (satisfaction problem)
        let objective = vec![];
        let target = ILP::new(
            builder.num_vars,
            bounds,
            builder.constraints,
            objective,
            ObjectiveSense::Minimize,
        );

        ReductionCircuitToILP {
            target,
            source_variables: self.variable_names().to_vec(),
            variable_map: builder.variable_map,
        }
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/circuit_ilp.rs"]
mod tests;
