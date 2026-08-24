//! Minimum-Cost Circulation problem implementation.
//!
//! Given a directed multigraph `G = (V, A)` with arc capacities `c(a)`
//! and **signed** arc costs `a(a)`, find a circulation `g` that
//! minimizes the total cost
//!
//! `sum_{a in A} a(a) * g(a)`,
//!
//! subject to
//!
//! 1. `0 <= g(a) <= c(a)` for every arc `a`, and
//! 2. inflow equals outflow at **every** vertex `v in V` (there is no
//!    distinguished source or sink — this is the defining feature of a
//!    circulation).
//!
//! Negative costs are explicitly allowed: with finite capacities every
//! circulation has bounded cost, and negative-cost cycles are exactly
//! what the standard reduction from min-cost max-flow uses (a single
//! sufficiently negative return arc from the sink to the source).
//!
//! # Integral-circulation restriction
//!
//! The mathematical formulation in issue #1030 uses continuous flows
//! `g: A -> R_{>= 0}`, but the [`Problem`] trait requires a discrete
//! configuration space via [`Problem::dims`]. Following the same
//! precedent as [`MinimumEdgeCostFlow`](super::MinimumEdgeCostFlow) and
//! the recently added [`MinimumCostMaximumFlow`](super::MinimumCostMaximumFlow),
//! we therefore restrict to **integer** circulations: each variable
//! `g(a)` ranges over `{0, 1, ..., c(a)}`. When capacities and costs are
//! integral, the standard minimum-cost flow theory (see e.g. the MIT
//! 6.854 notes, Ahuja-Magnanti-Orlin) guarantees that an integral
//! optimum exists, so this restriction does not change the optimal value
//! on integer instances.

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCostCirculation",
        display_name: "Minimum-Cost Circulation",
        aliases: &["MCC"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Integral circulation on a directed multigraph minimizing total signed arc cost",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed multigraph G = (V, A); loops and parallel arcs allowed" },
            FieldInfo { name: "capacities", type_name: "Vec<i64>", description: "Arc capacity c(a) in graph arc order (non-negative)" },
            FieldInfo { name: "costs", type_name: "Vec<i64>", description: "Signed arc cost a(a) in graph arc order (negative values allowed)" },
        ],
    }
}

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MinimumCostCirculation",
        fields: &["num_vertices", "num_arcs"],
    }
}

/// Minimum-Cost Circulation problem.
///
/// # Variables
///
/// `|A|` variables: variable `a` ranges over `{0, ..., c(a)}`
/// representing the integral circulation on arc `a`.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumCostCirculation;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // Two competing cycles 0->1->0 and 0->2->0; the cheaper-per-unit
/// // cycle 0->2->0 has lower capacity, but pushing both to capacity is
/// // optimal.
/// let graph = DirectedGraph::new(3, vec![
///     (0, 1), (1, 0), (0, 2), (2, 0),
/// ]);
/// let problem = MinimumCostCirculation::new(
///     graph,
///     vec![2, 2, 1, 1],   // capacities
///     vec![2, -3, 1, -4], // costs (signed)
/// );
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem).unwrap().unwrap();
/// // Optimal cost = 2*2 + 2*(-3) + 1*1 + 1*(-4) = -5.
/// assert_eq!(problem.total_cost(&witness).unwrap(), -5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumCostCirculation {
    /// The directed multigraph G = (V, A).
    graph: DirectedGraph,
    /// Capacity c(a) for each arc.
    capacities: Vec<i64>,
    /// Signed cost a(a) for each arc.
    costs: Vec<i64>,
}

impl MinimumCostCirculation {
    /// Create a new Minimum-Cost Circulation problem.
    ///
    /// # Panics
    ///
    /// Panics if any of the following holds:
    /// - `capacities.len() != graph.num_arcs()`
    /// - `costs.len() != graph.num_arcs()`
    /// - Any capacity is negative
    ///
    /// Note: costs are signed and **may be negative**.
    pub fn new(graph: DirectedGraph, capacities: Vec<i64>, costs: Vec<i64>) -> Self {
        let m = graph.num_arcs();
        assert_eq!(
            capacities.len(),
            m,
            "capacities length ({}) must match num_arcs ({m})",
            capacities.len()
        );
        assert_eq!(
            costs.len(),
            m,
            "costs length ({}) must match num_arcs ({m})",
            costs.len()
        );
        for (i, &c) in capacities.iter().enumerate() {
            assert!(c >= 0, "capacity[{i}] = {c} is negative");
        }
        Self {
            graph,
            capacities,
            costs,
        }
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get a reference to the arc capacities.
    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    /// Get a reference to the arc costs (signed).
    pub fn costs(&self) -> &[i64] {
        &self.costs
    }

    /// Get the number of vertices `|V|`.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs `|A|`.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Check whether a circulation assignment is feasible.
    ///
    /// A circulation is feasible iff
    /// 1. `config.len() == num_arcs`,
    /// 2. each `0 <= g(a) <= c(a)`, and
    /// 3. inflow equals outflow at **every** vertex (no exempt
    ///    terminals — this is what distinguishes a circulation from a
    ///    flow).
    pub fn is_feasible(&self, config: &[usize]) -> Result<bool, crate::traits::EvaluationError> {
        let m = self.graph.num_arcs();
        if config.len() != m {
            return Ok(false);
        }
        // (1) Capacity constraints
        for (flow, cap) in config.iter().zip(self.capacities.iter()) {
            let flow = i64::try_from(*flow).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting circulation configuration value".to_string(),
                )
            })?;
            if flow > *cap {
                return Ok(false);
            }
        }
        // (2) Flow conservation at every vertex
        let n = self.graph.num_vertices();
        let mut balance = vec![0_i64; n];
        for (a, &(u, v)) in self.graph.arcs().iter().enumerate() {
            let flow = i64::try_from(config[a]).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting circulation configuration value".to_string(),
                )
            })?;
            balance[u] = balance[u].checked_sub(flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing circulation vertex balance".to_string(),
                )
            })?;
            balance[v] = balance[v].checked_add(flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing circulation vertex balance".to_string(),
                )
            })?;
        }
        Ok(balance.iter().all(|&b| b == 0))
    }

    /// Compute the total cost `sum_a a(a) * g(a)` of a circulation.
    pub fn total_cost(&self, config: &[usize]) -> Result<i64, crate::traits::EvaluationError> {
        let mut total = 0_i64;
        for (&flow, &cost) in config.iter().zip(self.costs.iter()) {
            let flow = i64::try_from(flow).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting circulation configuration value".to_string(),
                )
            })?;
            let term = flow.checked_mul(cost).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "multiplying circulation arc cost".to_string(),
                )
            })?;
            total = total.checked_add(term).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing circulation costs".to_string(),
                )
            })?;
        }
        Ok(total)
    }
}

impl Problem for MinimumCostCirculation {
    const NAME: &'static str = "MinimumCostCirculation";
    type Value = crate::types::Min<i64>;

    fn dims(&self) -> Vec<usize> {
        self.capacities.iter().map(|&c| (c as usize) + 1).collect()
    }

    fn evaluate(
        &self,
        config: &[usize],
    ) -> Result<crate::types::Min<i64>, crate::traits::EvaluationError> {
        Ok({
            if !self.is_feasible(config)? {
                return Ok(crate::types::Min(None));
            }
            crate::types::Min(Some(self.total_cost(config)?))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default MinimumCostCirculation => "(num_vertices + num_arcs)^6",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // Two competing cycles on V = {0, 1, 2}:
    //   cycle A: 0->1->0  (cost per unit = 2 + (-3) = -1, capacity 2)
    //   cycle B: 0->2->0  (cost per unit = 1 + (-4) = -3, capacity 1)
    // Optimal: push both to capacity.
    //   arc 0 (0->1) = 2, arc 1 (1->0) = 2, arc 2 (0->2) = 1, arc 3 (2->0) = 1
    //   cost = 2*2 + 2*(-3) + 1*1 + 1*(-4) = 4 - 6 + 1 - 4 = -5
    let problem = MinimumCostCirculation::new(
        crate::topology::DirectedGraph::new(3, vec![(0, 1), (1, 0), (0, 2), (2, 0)]),
        vec![2, 2, 1, 1],
        vec![2, -3, 1, -4],
    );
    let optimal_config = vec![2, 2, 1, 1];
    let optimal_value = problem
        .evaluate(&optimal_config)
        .expect("canonical example evaluation must succeed");
    let scalar = match optimal_value {
        crate::types::Min(Some(v)) => v,
        crate::types::Min(None) => panic!("canonical example must be feasible"),
    };
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_cost_circulation",
        instance: Box::new(problem),
        optimal_config,
        optimal_value: serde_json::json!(scalar),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_cost_circulation.rs"]
mod tests;
