//! Circuit SAT problem implementation.
//!
//! CircuitSAT represents a boolean circuit satisfiability problem.
//! The goal is to find variable assignments that satisfy the circuit constraints.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

inventory::submit! {
    ProblemSchemaEntry {
        name: "CircuitSAT",
        description: "Find satisfying input to a boolean circuit",
        fields: &[
            FieldInfo { name: "circuit", type_name: "Circuit", description: "The boolean circuit" },
            FieldInfo { name: "variables", type_name: "Vec<String>", description: "Circuit variable names" },
        ],
    }
}

/// Boolean expression node types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BooleanOp {
    /// Variable reference
    Var(String),
    /// Boolean constant
    Const(bool),
    /// NOT operation
    Not(Box<BooleanExpr>),
    /// AND operation
    And(Vec<BooleanExpr>),
    /// OR operation
    Or(Vec<BooleanExpr>),
    /// XOR operation
    Xor(Vec<BooleanExpr>),
}

/// A boolean expression tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BooleanExpr {
    pub op: BooleanOp,
}

impl BooleanExpr {
    /// Create a variable reference.
    pub fn var(name: &str) -> Self {
        BooleanExpr {
            op: BooleanOp::Var(name.to_string()),
        }
    }

    /// Create a boolean constant.
    pub fn constant(value: bool) -> Self {
        BooleanExpr {
            op: BooleanOp::Const(value),
        }
    }

    /// Create a NOT expression.
    #[allow(clippy::should_implement_trait)]
    pub fn not(expr: BooleanExpr) -> Self {
        BooleanExpr {
            op: BooleanOp::Not(Box::new(expr)),
        }
    }

    /// Create an AND expression.
    pub fn and(args: Vec<BooleanExpr>) -> Self {
        BooleanExpr {
            op: BooleanOp::And(args),
        }
    }

    /// Create an OR expression.
    pub fn or(args: Vec<BooleanExpr>) -> Self {
        BooleanExpr {
            op: BooleanOp::Or(args),
        }
    }

    /// Create an XOR expression.
    pub fn xor(args: Vec<BooleanExpr>) -> Self {
        BooleanExpr {
            op: BooleanOp::Xor(args),
        }
    }

    /// Extract all variable names from this expression.
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.extract_variables(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }

    fn extract_variables(&self, vars: &mut Vec<String>) {
        match &self.op {
            BooleanOp::Var(name) => vars.push(name.clone()),
            BooleanOp::Const(_) => {}
            BooleanOp::Not(inner) => inner.extract_variables(vars),
            BooleanOp::And(args) | BooleanOp::Or(args) | BooleanOp::Xor(args) => {
                for arg in args {
                    arg.extract_variables(vars);
                }
            }
        }
    }

    /// Evaluate the expression given variable assignments.
    pub fn evaluate(&self, assignments: &HashMap<String, bool>) -> bool {
        match &self.op {
            BooleanOp::Var(name) => *assignments.get(name).unwrap_or(&false),
            BooleanOp::Const(value) => *value,
            BooleanOp::Not(inner) => !inner.evaluate(assignments),
            BooleanOp::And(args) => args.iter().all(|a| a.evaluate(assignments)),
            BooleanOp::Or(args) => args.iter().any(|a| a.evaluate(assignments)),
            BooleanOp::Xor(args) => args
                .iter()
                .fold(false, |acc, a| acc ^ a.evaluate(assignments)),
        }
    }
}

/// An assignment in a circuit: outputs = expr.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignment {
    /// Output variable names.
    pub outputs: Vec<String>,
    /// The expression to evaluate.
    pub expr: BooleanExpr,
}

impl Assignment {
    /// Create a new assignment.
    pub fn new(outputs: Vec<String>, expr: BooleanExpr) -> Self {
        Self { outputs, expr }
    }

    /// Get all variables referenced (both outputs and inputs).
    pub fn variables(&self) -> Vec<String> {
        let mut vars = self.outputs.clone();
        vars.extend(self.expr.variables());
        vars.sort();
        vars.dedup();
        vars
    }

    /// Check if the assignment is satisfied given variable assignments.
    pub fn is_satisfied(&self, assignments: &HashMap<String, bool>) -> bool {
        let result = self.expr.evaluate(assignments);
        self.outputs
            .iter()
            .all(|o| assignments.get(o).copied().unwrap_or(false) == result)
    }
}

/// A boolean circuit as a sequence of assignments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Circuit {
    /// The assignments in the circuit.
    pub assignments: Vec<Assignment>,
}

impl Circuit {
    /// Create a new circuit from assignments.
    pub fn new(assignments: Vec<Assignment>) -> Self {
        Self { assignments }
    }

    /// Get all variables in the circuit.
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        for assign in &self.assignments {
            vars.extend(assign.variables());
        }
        vars.sort();
        vars.dedup();
        vars
    }

    /// Get the number of assignments.
    pub fn num_assignments(&self) -> usize {
        self.assignments.len()
    }
}

/// The Circuit SAT problem.
///
/// Given a boolean circuit, find variable assignments that satisfy
/// all circuit constraints.
///
/// # Example
///
/// ```
/// use problemreductions::models::specialized::{CircuitSAT, BooleanExpr, Assignment, Circuit};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Create a simple circuit: c = x AND y
/// let circuit = Circuit::new(vec![
///     Assignment::new(
///         vec!["c".to_string()],
///         BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")])
///     ),
/// ]);
///
/// let problem = CircuitSAT::new(circuit);
/// let solver = BruteForce::new();
/// let solutions = solver.find_all_satisfying(&problem);
///
/// // Multiple satisfying assignments exist
/// assert!(!solutions.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitSAT {
    /// The circuit.
    circuit: Circuit,
    /// Variables in order.
    variables: Vec<String>,
}

impl CircuitSAT {
    /// Create a new CircuitSAT problem.
    pub fn new(circuit: Circuit) -> Self {
        let variables = circuit.variables();
        Self { circuit, variables }
    }

    /// Get the circuit.
    pub fn circuit(&self) -> &Circuit {
        &self.circuit
    }

    /// Get the variable names.
    pub fn variable_names(&self) -> &[String] {
        &self.variables
    }

    /// Convert a configuration to variable assignments.
    fn config_to_assignments(&self, config: &[usize]) -> HashMap<String, bool> {
        self.variables
            .iter()
            .enumerate()
            .map(|(i, name)| (name.clone(), config.get(i).copied().unwrap_or(0) == 1))
            .collect()
    }

    /// Count how many assignments are satisfied.
    fn count_satisfied(&self, config: &[usize]) -> usize {
        let assignments = self.config_to_assignments(config);
        self.circuit
            .assignments
            .iter()
            .filter(|a| a.is_satisfied(&assignments))
            .count()
    }
}

/// Check if a circuit assignment is satisfying.
pub fn is_circuit_satisfying(circuit: &Circuit, assignments: &HashMap<String, bool>) -> bool {
    circuit
        .assignments
        .iter()
        .all(|a| a.is_satisfied(assignments))
}

impl Problem for CircuitSAT {
    const NAME: &'static str = "CircuitSAT";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.variables.len()]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.count_satisfied(config) == self.circuit.num_assignments()
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![("graph", "SimpleGraph")]
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/specialized/circuit.rs"]
mod tests;
