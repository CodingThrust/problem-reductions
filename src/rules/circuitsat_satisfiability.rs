//! Reduction from CircuitSAT to Satisfiability via the Tseitin transformation.
//!
//! Each gate in the boolean circuit is converted to definitional CNF clauses
//! that preserve satisfiability. The resulting SAT instance has one variable
//! per circuit variable plus auxiliary variables for intermediate gate outputs.

use crate::models::formula::{Assignment, BooleanExpr, BooleanOp, CNFClause, CircuitSAT, Satisfiability};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use std::collections::HashMap;

/// Result of reducing CircuitSAT to Satisfiability.
#[derive(Debug, Clone)]
pub struct ReductionCircuitSATToSatisfiability {
    target: Satisfiability,
    /// Maps circuit variable name -> SAT variable index (1-indexed).
    var_map: HashMap<String, i32>,
    /// Sorted circuit variable names (defines the circuit config ordering).
    circuit_var_names: Vec<String>,
}

impl ReductionResult for ReductionCircuitSATToSatisfiability {
    type Source = CircuitSAT;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Satisfiability {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // The circuit variable names define the config ordering.
        // For each circuit variable, look up its SAT index and read the value.
        self.circuit_var_names
            .iter()
            .map(|name| {
                let sat_idx = self.var_map[name]; // 1-indexed
                target_solution[(sat_idx as usize) - 1]
            })
            .collect()
    }
}

/// State for the Tseitin transformation.
struct TseitinBuilder {
    var_map: HashMap<String, i32>,
    next_var: i32,
    clauses: Vec<CNFClause>,
}

impl TseitinBuilder {
    fn new(circuit_var_names: &[String]) -> Self {
        let mut var_map = HashMap::new();
        let mut next_var = 1i32;
        // Assign indices to circuit variables first, preserving sorted order.
        for name in circuit_var_names {
            var_map.insert(name.clone(), next_var);
            next_var += 1;
        }
        Self {
            var_map,
            next_var,
            clauses: Vec::new(),
        }
    }

    fn get_var(&mut self, name: &str) -> i32 {
        if let Some(&v) = self.var_map.get(name) {
            v
        } else {
            let v = self.next_var;
            self.var_map.insert(name.to_string(), v);
            self.next_var += 1;
            v
        }
    }

    fn fresh_var(&mut self) -> i32 {
        let v = self.next_var;
        self.next_var += 1;
        v
    }

    /// Walk the expression tree, creating definitional clauses.
    /// Returns the SAT variable index representing this subexpression.
    fn tseitin(&mut self, expr: &BooleanExpr) -> i32 {
        match &expr.op {
            BooleanOp::Var(name) => self.get_var(name),
            BooleanOp::Const(value) => {
                let v = self.fresh_var();
                if *value {
                    self.clauses.push(CNFClause::new(vec![v]));
                } else {
                    self.clauses.push(CNFClause::new(vec![-v]));
                }
                v
            }
            BooleanOp::Not(inner) => {
                let a = self.tseitin(inner);
                let v = self.fresh_var();
                // v <=> NOT a
                self.clauses.push(CNFClause::new(vec![-v, -a]));
                self.clauses.push(CNFClause::new(vec![v, a]));
                v
            }
            BooleanOp::And(children) => {
                let child_vars: Vec<i32> = children.iter().map(|c| self.tseitin(c)).collect();
                if child_vars.len() == 1 {
                    return child_vars[0];
                }
                // Build binary tree of AND gates
                let mut result = child_vars[0];
                for &b in &child_vars[1..] {
                    let a = result;
                    let v = self.fresh_var();
                    // v <=> a AND b
                    self.clauses.push(CNFClause::new(vec![-v, a]));
                    self.clauses.push(CNFClause::new(vec![-v, b]));
                    self.clauses.push(CNFClause::new(vec![v, -a, -b]));
                    result = v;
                }
                result
            }
            BooleanOp::Or(children) => {
                let child_vars: Vec<i32> = children.iter().map(|c| self.tseitin(c)).collect();
                if child_vars.len() == 1 {
                    return child_vars[0];
                }
                // Build binary tree of OR gates
                let mut result = child_vars[0];
                for &b in &child_vars[1..] {
                    let a = result;
                    let v = self.fresh_var();
                    // v <=> a OR b
                    self.clauses.push(CNFClause::new(vec![v, -a]));
                    self.clauses.push(CNFClause::new(vec![v, -b]));
                    self.clauses.push(CNFClause::new(vec![-v, a, b]));
                    result = v;
                }
                result
            }
            BooleanOp::Xor(children) => {
                let child_vars: Vec<i32> = children.iter().map(|c| self.tseitin(c)).collect();
                if child_vars.len() == 1 {
                    return child_vars[0];
                }
                // Build binary tree of XOR gates
                let mut result = child_vars[0];
                for &b in &child_vars[1..] {
                    let a = result;
                    let v = self.fresh_var();
                    // v <=> a XOR b
                    self.clauses.push(CNFClause::new(vec![-v, -a, -b]));
                    self.clauses.push(CNFClause::new(vec![-v, a, b]));
                    self.clauses.push(CNFClause::new(vec![v, -a, b]));
                    self.clauses.push(CNFClause::new(vec![v, a, -b]));
                    result = v;
                }
                result
            }
        }
    }

    fn process_assignment(&mut self, assign: &Assignment) {
        let root_var = self.tseitin(&assign.expr);
        // Each output must equal the expression result
        for out_name in &assign.outputs {
            let out_var = self.get_var(out_name);
            // out_var <=> root_var
            self.clauses.push(CNFClause::new(vec![-out_var, root_var]));
            self.clauses.push(CNFClause::new(vec![out_var, -root_var]));
        }
    }

    fn num_vars(&self) -> usize {
        (self.next_var - 1) as usize
    }
}

#[reduction(
    overhead = {
        num_vars = "num_variables + num_assignments",
        num_clauses = "3 * num_assignments + num_variables",
    }
)]
impl ReduceTo<Satisfiability> for CircuitSAT {
    type Result = ReductionCircuitSATToSatisfiability;

    fn reduce_to(&self) -> Self::Result {
        let circuit_var_names: Vec<String> = self.variable_names().to_vec();

        let mut builder = TseitinBuilder::new(&circuit_var_names);

        for assign in &self.circuit().assignments {
            builder.process_assignment(assign);
        }

        let num_vars = builder.num_vars();
        let target = Satisfiability::new(num_vars, builder.clauses.clone());

        ReductionCircuitSATToSatisfiability {
            target,
            var_map: builder.var_map,
            circuit_var_names,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::{BooleanExpr, Circuit};

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "circuitsat_to_satisfiability",
        build: || {
            // c = x AND y, d = c OR z
            let circuit = Circuit::new(vec![
                Assignment::new(
                    vec!["c".to_string()],
                    BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
                ),
                Assignment::new(
                    vec!["d".to_string()],
                    BooleanExpr::or(vec![BooleanExpr::var("c"), BooleanExpr::var("z")]),
                ),
            ]);
            let source = CircuitSAT::new(circuit);

            // Find a valid solution pair by brute force
            let reduction = <CircuitSAT as ReduceTo<Satisfiability>>::reduce_to(&source);
            let solver = crate::solvers::BruteForce::new();
            let target_solutions = solver.find_all_witnesses(reduction.target_problem());
            let target_config = target_solutions
                .into_iter()
                .next()
                .expect("YES instance must have a solution");
            let source_config = reduction.extract_solution(&target_config);

            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/circuitsat_satisfiability.rs"]
mod tests;
