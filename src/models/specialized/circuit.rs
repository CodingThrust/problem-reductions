//! Circuit SAT problem implementation.
//!
//! CircuitSAT represents a boolean circuit satisfiability problem.
//! The goal is to find variable assignments that satisfy the circuit constraints.

use crate::traits::Problem;
use crate::variant::short_type_name;
use crate::types::{EnergyMode, ProblemSize, SolutionSize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
/// as many assignments as possible (or all of them).
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
/// let problem = CircuitSAT::<i32>::new(circuit);
/// let solver = BruteForce::new();
/// let solutions = solver.find_best(&problem);
///
/// // Multiple satisfying assignments exist
/// assert!(!solutions.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitSAT<W = i32> {
    /// The circuit.
    circuit: Circuit,
    /// Variables in order.
    variables: Vec<String>,
    /// Weights for each assignment.
    weights: Vec<W>,
}

impl<W: Clone + Default + From<i32>> CircuitSAT<W> {
    /// Create a new CircuitSAT problem with unit weights.
    pub fn new(circuit: Circuit) -> Self {
        let variables = circuit.variables();
        let weights = vec![W::from(1); circuit.num_assignments()];
        Self {
            circuit,
            variables,
            weights,
        }
    }
}

impl<W> CircuitSAT<W> {
    /// Create a CircuitSAT problem with custom weights.
    pub fn with_weights(circuit: Circuit, weights: Vec<W>) -> Self {
        assert_eq!(weights.len(), circuit.num_assignments());
        let variables = circuit.variables();
        Self {
            circuit,
            variables,
            weights,
        }
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

impl<W> Problem for CircuitSAT<W>
where
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "CircuitSAT";

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", "SimpleGraph"),
            ("weight", short_type_name::<W>()),
        ]
    }

    type Size = W;

    fn num_variables(&self) -> usize {
        self.variables.len()
    }

    fn num_flavors(&self) -> usize {
        2 // Binary
    }

    fn problem_size(&self) -> ProblemSize {
        ProblemSize::new(vec![
            ("num_variables", self.variables.len()),
            ("num_assignments", self.circuit.num_assignments()),
        ])
    }

    fn energy_mode(&self) -> EnergyMode {
        EnergyMode::LargerSizeIsBetter // Maximize satisfied assignments
    }

    fn solution_size(&self, config: &[usize]) -> SolutionSize<Self::Size> {
        let assignments = self.config_to_assignments(config);
        let mut total = W::zero();

        for (i, assign) in self.circuit.assignments.iter().enumerate() {
            if assign.is_satisfied(&assignments) {
                total += self.weights[i].clone();
            }
        }

        // Valid if all assignments are satisfied
        let is_valid = self.count_satisfied(config) == self.circuit.num_assignments();
        SolutionSize::new(total, is_valid)
    }
}

/// Check if a circuit assignment is satisfying.
pub fn is_circuit_satisfying(circuit: &Circuit, assignments: &HashMap<String, bool>) -> bool {
    circuit
        .assignments
        .iter()
        .all(|a| a.is_satisfied(assignments))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_boolean_expr_var() {
        let expr = BooleanExpr::var("x");
        let mut assignments = HashMap::new();
        assignments.insert("x".to_string(), true);
        assert!(expr.evaluate(&assignments));

        assignments.insert("x".to_string(), false);
        assert!(!expr.evaluate(&assignments));
    }

    #[test]
    fn test_boolean_expr_const() {
        let t = BooleanExpr::constant(true);
        let f = BooleanExpr::constant(false);
        let assignments = HashMap::new();
        assert!(t.evaluate(&assignments));
        assert!(!f.evaluate(&assignments));
    }

    #[test]
    fn test_boolean_expr_not() {
        let expr = BooleanExpr::not(BooleanExpr::var("x"));
        let mut assignments = HashMap::new();
        assignments.insert("x".to_string(), true);
        assert!(!expr.evaluate(&assignments));

        assignments.insert("x".to_string(), false);
        assert!(expr.evaluate(&assignments));
    }

    #[test]
    fn test_boolean_expr_and() {
        let expr = BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]);
        let mut assignments = HashMap::new();

        assignments.insert("x".to_string(), true);
        assignments.insert("y".to_string(), true);
        assert!(expr.evaluate(&assignments));

        assignments.insert("y".to_string(), false);
        assert!(!expr.evaluate(&assignments));
    }

    #[test]
    fn test_boolean_expr_or() {
        let expr = BooleanExpr::or(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]);
        let mut assignments = HashMap::new();

        assignments.insert("x".to_string(), false);
        assignments.insert("y".to_string(), false);
        assert!(!expr.evaluate(&assignments));

        assignments.insert("y".to_string(), true);
        assert!(expr.evaluate(&assignments));
    }

    #[test]
    fn test_boolean_expr_xor() {
        let expr = BooleanExpr::xor(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]);
        let mut assignments = HashMap::new();

        assignments.insert("x".to_string(), true);
        assignments.insert("y".to_string(), true);
        assert!(!expr.evaluate(&assignments)); // XOR(T, T) = F

        assignments.insert("y".to_string(), false);
        assert!(expr.evaluate(&assignments)); // XOR(T, F) = T
    }

    #[test]
    fn test_boolean_expr_variables() {
        let expr = BooleanExpr::and(vec![
            BooleanExpr::var("x"),
            BooleanExpr::or(vec![BooleanExpr::var("y"), BooleanExpr::var("z")]),
        ]);
        let vars = expr.variables();
        assert_eq!(vars, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_assignment_satisfied() {
        let assign = Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        );

        let mut assignments = HashMap::new();
        assignments.insert("x".to_string(), true);
        assignments.insert("y".to_string(), true);
        assignments.insert("c".to_string(), true);
        assert!(assign.is_satisfied(&assignments));

        assignments.insert("c".to_string(), false);
        assert!(!assign.is_satisfied(&assignments));
    }

    #[test]
    fn test_circuit_variables() {
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
        let vars = circuit.variables();
        assert_eq!(vars, vec!["c", "d", "x", "y", "z"]);
    }

    #[test]
    fn test_circuit_sat_creation() {
        let circuit = Circuit::new(vec![Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        )]);
        let problem = CircuitSAT::<i32>::new(circuit);
        assert_eq!(problem.num_variables(), 3); // c, x, y
        assert_eq!(problem.num_flavors(), 2);
    }

    #[test]
    fn test_circuit_sat_solution_size() {
        // c = x AND y
        let circuit = Circuit::new(vec![Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        )]);
        let problem = CircuitSAT::<i32>::new(circuit);

        // Variables sorted: c, x, y
        // c=1, x=1, y=1 -> c = 1 AND 1 = 1, valid
        let sol = problem.solution_size(&[1, 1, 1]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        // c=0, x=0, y=0 -> c = 0 AND 0 = 0, valid
        let sol = problem.solution_size(&[0, 0, 0]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 1);

        // c=1, x=0, y=0 -> c should be 0, but c=1, invalid
        let sol = problem.solution_size(&[1, 0, 0]);
        assert!(!sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_circuit_sat_brute_force() {
        // c = x AND y
        let circuit = Circuit::new(vec![Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        )]);
        let problem = CircuitSAT::<i32>::new(circuit);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All satisfying: c matches x AND y
        // 4 valid configs: (0,0,0), (0,0,1), (0,1,0), (1,1,1)
        assert_eq!(solutions.len(), 4);
        for sol in &solutions {
            assert!(problem.solution_size(sol).is_valid);
        }
    }

    #[test]
    fn test_circuit_sat_complex() {
        // c = x AND y
        // d = c OR z
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
        let problem = CircuitSAT::<i32>::new(circuit);
        let solver = BruteForce::new();

        let solutions = solver.find_best(&problem);
        // All valid solutions satisfy both assignments
        for sol in &solutions {
            let sol_size = problem.solution_size(sol);
            assert!(sol_size.is_valid);
            assert_eq!(sol_size.size, 2);
        }
    }

    #[test]
    fn test_is_circuit_satisfying() {
        let circuit = Circuit::new(vec![Assignment::new(
            vec!["c".to_string()],
            BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
        )]);

        let mut assignments = HashMap::new();
        assignments.insert("x".to_string(), true);
        assignments.insert("y".to_string(), true);
        assignments.insert("c".to_string(), true);
        assert!(is_circuit_satisfying(&circuit, &assignments));

        assignments.insert("c".to_string(), false);
        assert!(!is_circuit_satisfying(&circuit, &assignments));
    }

    #[test]
    fn test_problem_size() {
        let circuit = Circuit::new(vec![
            Assignment::new(vec!["c".to_string()], BooleanExpr::var("x")),
            Assignment::new(vec!["d".to_string()], BooleanExpr::var("y")),
        ]);
        let problem = CircuitSAT::<i32>::new(circuit);
        let size = problem.problem_size();
        assert_eq!(size.get("num_variables"), Some(4));
        assert_eq!(size.get("num_assignments"), Some(2));
    }

    #[test]
    fn test_energy_mode() {
        let circuit = Circuit::new(vec![]);
        let problem = CircuitSAT::<i32>::new(circuit);
        assert!(problem.energy_mode().is_maximization());
    }

    #[test]
    fn test_empty_circuit() {
        let circuit = Circuit::new(vec![]);
        let problem = CircuitSAT::<i32>::new(circuit);
        let sol = problem.solution_size(&[]);
        assert!(sol.is_valid);
        assert_eq!(sol.size, 0);
    }

    #[test]
    fn test_weighted_circuit_sat() {
        let circuit = Circuit::new(vec![
            Assignment::new(vec!["c".to_string()], BooleanExpr::var("x")),
            Assignment::new(vec!["d".to_string()], BooleanExpr::var("y")),
        ]);
        let problem = CircuitSAT::with_weights(circuit, vec![10, 1]);

        // Variables sorted: c, d, x, y
        // Config [1, 0, 1, 0]: c=1, d=0, x=1, y=0
        // c=x (1=1) satisfied (weight 10), d=y (0=0) satisfied (weight 1)
        let sol = problem.solution_size(&[1, 0, 1, 0]);
        assert_eq!(sol.size, 11); // Both satisfied: 10 + 1
        assert!(sol.is_valid);

        // Config [1, 0, 0, 0]: c=1, d=0, x=0, y=0
        // c=x (1!=0) not satisfied, d=y (0=0) satisfied (weight 1)
        let sol = problem.solution_size(&[1, 0, 0, 0]);
        assert_eq!(sol.size, 1); // Only d=y satisfied
        assert!(!sol.is_valid);

        // Config [0, 1, 0, 0]: c=0, d=1, x=0, y=0
        // c=x (0=0) satisfied (weight 10), d=y (1!=0) not satisfied
        let sol = problem.solution_size(&[0, 1, 0, 0]);
        assert_eq!(sol.size, 10); // Only c=x satisfied
        assert!(!sol.is_valid);
    }
}
