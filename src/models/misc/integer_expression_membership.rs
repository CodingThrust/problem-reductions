//! Integer Expression Membership problem implementation.
//!
//! Given a recursive integer expression tree built from singleton positive integers
//! combined with union (∪) and Minkowski sum (+) operations, and a target integer K,
//! decide whether K belongs to the set represented by the expression.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Or;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "IntegerExpressionMembership",
        display_name: "Integer Expression Membership",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Decide whether a target integer belongs to the set represented by an expression tree over union and Minkowski sum",
        fields: &[
            FieldInfo { name: "expression", type_name: "IntExpr", description: "Recursive expression tree" },
            FieldInfo { name: "target", type_name: "u64", description: "Target integer K" },
        ],
    }
}

/// A recursive integer expression tree.
///
/// Represents a set of positive integers built from:
/// - `Atom(n)`: the singleton set {n}
/// - `Union(f, g)`: set union F ∪ G
/// - `Sum(f, g)`: Minkowski sum {m + n : m ∈ F, n ∈ G}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntExpr {
    /// Singleton set {n} for a positive integer n.
    Atom(u64),
    /// Set union: F ∪ G.
    Union(Box<IntExpr>, Box<IntExpr>),
    /// Minkowski sum: {m + n : m ∈ F, n ∈ G}.
    Sum(Box<IntExpr>, Box<IntExpr>),
}

impl IntExpr {
    /// Returns true if all atoms in the expression are positive (> 0).
    pub fn all_atoms_positive(&self) -> bool {
        match self {
            IntExpr::Atom(n) => *n > 0,
            IntExpr::Union(l, r) | IntExpr::Sum(l, r) => {
                l.all_atoms_positive() && r.all_atoms_positive()
            }
        }
    }

    /// Count the total number of nodes in the expression tree.
    pub fn size(&self) -> usize {
        match self {
            IntExpr::Atom(_) => 1,
            IntExpr::Union(l, r) | IntExpr::Sum(l, r) => 1 + l.size() + r.size(),
        }
    }

    /// Count the number of Union nodes in the expression tree.
    pub fn count_union_nodes(&self) -> usize {
        match self {
            IntExpr::Atom(_) => 0,
            IntExpr::Union(l, r) => 1 + l.count_union_nodes() + r.count_union_nodes(),
            IntExpr::Sum(l, r) => l.count_union_nodes() + r.count_union_nodes(),
        }
    }

    /// Count the number of Atom nodes in the expression tree.
    pub fn count_atoms(&self) -> usize {
        match self {
            IntExpr::Atom(_) => 1,
            IntExpr::Union(l, r) | IntExpr::Sum(l, r) => l.count_atoms() + r.count_atoms(),
        }
    }

    /// Compute the depth of the expression tree (0 for a single Atom).
    pub fn depth(&self) -> usize {
        match self {
            IntExpr::Atom(_) => 0,
            IntExpr::Union(l, r) | IntExpr::Sum(l, r) => 1 + l.depth().max(r.depth()),
        }
    }

    /// Evaluate the expression given union choices from config.
    ///
    /// `counter` tracks which union node we are at (DFS order).
    /// Returns `Some(value)` if the config is valid, `None` otherwise.
    fn evaluate_with_config(&self, config: &[usize], counter: &mut usize) -> Option<u64> {
        match self {
            IntExpr::Atom(n) => Some(*n),
            IntExpr::Union(left, right) => {
                let idx = *counter;
                *counter += 1;
                if idx >= config.len() {
                    return None;
                }
                match config[idx] {
                    0 => left.evaluate_with_config(config, counter),
                    1 => right.evaluate_with_config(config, counter),
                    _ => None,
                }
            }
            IntExpr::Sum(left, right) => {
                let l = left.evaluate_with_config(config, counter)?;
                let r = right.evaluate_with_config(config, counter)?;
                l.checked_add(r)
            }
        }
    }
}

/// The Integer Expression Membership problem.
///
/// Given an integer expression `e` over union (∪) and Minkowski sum (+)
/// operations on singleton positive integers, and a target integer `K`,
/// decide whether `K ∈ eval(e)`.
///
/// # Configuration
///
/// Each Union node has a binary variable (0 = left, 1 = right).
/// A configuration assigns a branch choice to every Union node in DFS order.
/// The expression then collapses to a chain of Sum and Atom nodes,
/// evaluating to a single integer.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::{IntegerExpressionMembership, IntExpr};
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // e = (1 ∪ 4) + (3 ∪ 6) + (2 ∪ 5), target K = 12
/// let expr = IntExpr::Sum(
///     Box::new(IntExpr::Sum(
///         Box::new(IntExpr::Union(
///             Box::new(IntExpr::Atom(1)),
///             Box::new(IntExpr::Atom(4)),
///         )),
///         Box::new(IntExpr::Union(
///             Box::new(IntExpr::Atom(3)),
///             Box::new(IntExpr::Atom(6)),
///         )),
///     )),
///     Box::new(IntExpr::Union(
///         Box::new(IntExpr::Atom(2)),
///         Box::new(IntExpr::Atom(5)),
///     )),
/// );
/// let problem = IntegerExpressionMembership::new(expr, 12);
/// let solver = BruteForce::new();
/// let solution = solver.find_witness(&problem);
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerExpressionMembership {
    /// The recursive expression tree.
    expression: IntExpr,
    /// The target integer K.
    target: u64,
}

impl IntegerExpressionMembership {
    /// Create a new IntegerExpressionMembership instance.
    ///
    /// # Arguments
    /// * `expression` - The integer expression tree
    /// * `target` - The target integer K
    pub fn new(expression: IntExpr, target: u64) -> Self {
        assert!(target > 0, "target must be a positive integer (got 0)");
        assert!(
            expression.all_atoms_positive(),
            "all Atom values must be positive (> 0)"
        );
        Self { expression, target }
    }

    /// Returns a reference to the expression tree.
    pub fn expression(&self) -> &IntExpr {
        &self.expression
    }

    /// Returns the target integer K.
    pub fn target(&self) -> u64 {
        self.target
    }

    /// Returns the total number of nodes in the expression tree.
    pub fn expression_size(&self) -> usize {
        self.expression.size()
    }

    /// Returns the number of Union nodes in the expression tree.
    pub fn num_union_nodes(&self) -> usize {
        self.expression.count_union_nodes()
    }

    /// Returns the number of Atom nodes in the expression tree.
    pub fn num_atoms(&self) -> usize {
        self.expression.count_atoms()
    }

    /// Returns the depth of the expression tree.
    pub fn expression_depth(&self) -> usize {
        self.expression.depth()
    }

    /// Evaluate the expression for a given config and return the resulting integer.
    ///
    /// Returns `Some(value)` if the config is valid, `None` otherwise.
    pub fn evaluate_config(&self, config: &[usize]) -> Option<u64> {
        let mut counter = 0;
        self.expression.evaluate_with_config(config, &mut counter)
    }
}

impl Problem for IntegerExpressionMembership {
    const NAME: &'static str = "IntegerExpressionMembership";
    type Value = Or;

    fn dims(&self) -> Vec<usize> {
        vec![2; self.num_union_nodes()]
    }

    fn evaluate(&self, config: &[usize]) -> Or {
        Or({
            if config.len() != self.num_union_nodes() {
                return Or(false);
            }
            if config.iter().any(|&v| v >= 2) {
                return Or(false);
            }
            match self.evaluate_config(config) {
                Some(value) => value == self.target,
                None => false,
            }
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default IntegerExpressionMembership => "2^num_union_nodes",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // e = (1 ∪ 4) + (3 ∪ 6) + (2 ∪ 5), K = 12
    // 3 union nodes → 8 configs. Set = {6, 9, 12, 15}.
    // Witness: choose right(4), right(6), left(2) → 4+6+2=12, config=[1,1,0]
    let expr = IntExpr::Sum(
        Box::new(IntExpr::Sum(
            Box::new(IntExpr::Union(
                Box::new(IntExpr::Atom(1)),
                Box::new(IntExpr::Atom(4)),
            )),
            Box::new(IntExpr::Union(
                Box::new(IntExpr::Atom(3)),
                Box::new(IntExpr::Atom(6)),
            )),
        )),
        Box::new(IntExpr::Union(
            Box::new(IntExpr::Atom(2)),
            Box::new(IntExpr::Atom(5)),
        )),
    );
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "integer_expression_membership",
        instance: Box::new(IntegerExpressionMembership::new(expr, 12)),
        optimal_config: vec![1, 1, 0],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/integer_expression_membership.rs"]
mod tests;
