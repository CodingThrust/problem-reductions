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

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, ILP};
use crate::models::formula::{BooleanExpr, BooleanOp, CircuitSAT};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::HashMap;

/// Result of reducing CircuitSAT to ILP.
#[derive(Debug, Clone)]
pub struct ReductionCircuitToILP {
    target: ILP<bool>,
    source_variables: Vec<String>,
    variable_map: HashMap<String, usize>,
}

impl ReductionResult for ReductionCircuitToILP {
    type Source = CircuitSAT;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            self.source_variables
                .iter()
                .map(|name| target_solution[self.variable_map[name]] == 1)
                .collect()
        })
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
    fn process_expr(&mut self, expr: &BooleanExpr) -> Result<usize, std::num::TryFromIntError> {
        Ok(match &expr.op {
            BooleanOp::Var(name) => self.get_or_create_var(name),
            BooleanOp::Const(value) => {
                let c = self.alloc_aux();
                let v = if *value { 1 } else { 0 };
                self.constraints.push(LinearConstraint::eq(vec![(c, 1)], v));
                c
            }
            BooleanOp::Not(inner) => {
                let a = self.process_expr(inner)?;
                let c = self.alloc_aux();
                // c + a = 1
                self.constraints
                    .push(LinearConstraint::eq(vec![(c, 1), (a, 1)], 1));
                c
            }
            BooleanOp::And(args) => {
                let inputs: Vec<usize> = args
                    .iter()
                    .map(|arg| self.process_expr(arg))
                    .collect::<Result<_, _>>()?;
                let c = self.alloc_aux();
                let k = i64::try_from(inputs.len())?;
                // c ≤ a_i for all i
                for &a_i in &inputs {
                    self.constraints
                        .push(LinearConstraint::le(vec![(c, 1), (a_i, -1)], 0));
                }
                // c ≥ Σa_i - (k - 1)
                let mut terms: Vec<(usize, i64)> = vec![(c, 1)];
                for &a_i in &inputs {
                    terms.push((a_i, -1));
                }
                self.constraints.push(LinearConstraint::ge(terms, 1 - k));
                c
            }
            BooleanOp::Or(args) => {
                let inputs: Vec<usize> = args
                    .iter()
                    .map(|arg| self.process_expr(arg))
                    .collect::<Result<_, _>>()?;
                let c = self.alloc_aux();
                // c ≥ a_i for all i
                for &a_i in &inputs {
                    self.constraints
                        .push(LinearConstraint::ge(vec![(c, 1), (a_i, -1)], 0));
                }
                // c ≤ Σa_i
                let mut terms: Vec<(usize, i64)> = vec![(c, 1)];
                for &a_i in &inputs {
                    terms.push((a_i, -1));
                }
                self.constraints.push(LinearConstraint::le(terms, 0));
                c
            }
            BooleanOp::Xor(args) => {
                // Chain pairwise: XOR(a1, a2, a3) = XOR(XOR(a1, a2), a3)
                let inputs: Vec<usize> = args
                    .iter()
                    .map(|arg| self.process_expr(arg))
                    .collect::<Result<_, _>>()?;
                assert!(!inputs.is_empty());
                let mut result = inputs[0];
                for &next in &inputs[1..] {
                    let c = self.alloc_aux();
                    let a = result;
                    let b = next;
                    // c ≤ a + b
                    self.constraints
                        .push(LinearConstraint::le(vec![(c, 1), (a, -1), (b, -1)], 0));
                    // c ≥ a - b
                    self.constraints
                        .push(LinearConstraint::ge(vec![(c, 1), (a, -1), (b, 1)], 0));
                    // c ≥ b - a
                    self.constraints
                        .push(LinearConstraint::ge(vec![(c, 1), (a, 1), (b, -1)], 0));
                    // c ≤ 2 - a - b
                    self.constraints
                        .push(LinearConstraint::le(vec![(c, 1), (a, 1), (b, 1)], 2));
                    result = c;
                }
                result
            }
        })
    }
}

#[reduction(
    transform = upper_bound {
        num_vars = "num_variables + num_expression_nodes",
        num_constraints = "5 * num_expression_nodes + num_assignment_outputs",
    },
    unavailable = {
        num_nonzeros = "the exact target parameter is not represented by this reduction's symbolic transform",
    }
)]
impl ReduceTo<ILP<bool>> for CircuitSAT {
    type Result = ReductionCircuitToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let mut builder = ILPBuilder::new();

        // Pre-register all circuit variables to preserve ordering
        for name in self.variable_names() {
            builder.get_or_create_var(name);
        }

        // Process each assignment
        for assignment in &self.circuit().assignments {
            let expr_var = builder.process_expr(&assignment.expr).map_err(|_| {
                crate::rules::ReductionError::integer_overflow::<CircuitSAT, ILP<bool>>(
                    "encoding a circuit gate arity",
                )
            })?;
            // Constrain each output to equal the expression result
            for output_name in &assignment.outputs {
                let out_var = builder.get_or_create_var(output_name);
                if out_var != expr_var {
                    // out = expr_var
                    builder
                        .constraints
                        .push(LinearConstraint::eq(vec![(out_var, 1), (expr_var, -1)], 0));
                }
            }
        }

        // Trivial objective: minimize 0 (satisfaction problem)
        let objective = vec![];
        let target = ILP::new(
            builder.num_vars,
            builder.constraints,
            objective,
            ObjectiveSense::Minimize,
        )
        .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;

        Ok(ReductionCircuitToILP {
            target,
            source_variables: self.variable_names().to_vec(),
            variable_map: builder.variable_map,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::{Assignment, BooleanExpr, Circuit};

    fn full_adder_circuit_sat() -> CircuitSAT {
        let circuit = Circuit::new(vec![
            Assignment::new(
                vec!["t".to_string()],
                BooleanExpr::xor(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
            ),
            Assignment::new(
                vec!["sum".to_string()],
                BooleanExpr::xor(vec![BooleanExpr::var("t"), BooleanExpr::var("cin")]),
            ),
            Assignment::new(
                vec!["ab".to_string()],
                BooleanExpr::and(vec![BooleanExpr::var("a"), BooleanExpr::var("b")]),
            ),
            Assignment::new(
                vec!["cin_t".to_string()],
                BooleanExpr::and(vec![BooleanExpr::var("cin"), BooleanExpr::var("t")]),
            ),
            Assignment::new(
                vec!["cout".to_string()],
                BooleanExpr::or(vec![BooleanExpr::var("ab"), BooleanExpr::var("cin_t")]),
            ),
        ]);
        CircuitSAT::new(circuit)
    }

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "circuitsat_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<
                _,
                crate::models::algebraic::ILP<bool>,
            >(
                full_adder_circuit_sat(),
                SolutionPair {
                    source_config: serde_json::json!(vec![
                        false, false, false, false, false, false, false, false
                    ]),
                    target_config: serde_json::json!(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/circuit_ilp.rs"]
mod tests;
