//! Minimum-Cost Maximum-Flow problem implementation.
//!
//! Given a directed graph `G = (V, A)` with arc capacities `c(a)` and
//! arc costs `cost(a)`, a source `s`, and a sink `t`, find a flow `f`
//! that
//!
//! 1. first maximizes the flow value `|f| = sum_{a in delta^+(s)} f(a)
//!    - sum_{a in delta^-(s)} f(a)`, and then
//! 2. among all maximum-value flows, minimizes the total arc cost
//!    `sum_a cost(a) * f(a)`.
//!
//! The objective is lexicographic: maximum-value first, ties broken by
//! lower total cost.
//!
//! # Integral-flow restriction
//!
//! The mathematical CellRouter formulation in issue #1029 uses
//! continuous flows `f: A -> R_{>= 0}`, but this model's registered reference
//! solver uses a finite Cartesian space. Following the same precedent as
//! [`MinimumEdgeCostFlow`](super::MinimumEdgeCostFlow) (see
//! `src/models/graph/minimum_edge_cost_flow.rs`), we therefore restrict
//! to **integer** flows: each variable `f(a)` ranges over
//! `{0, 1, ..., c(a)}`. When capacities and costs are integral, the
//! standard minimum-cost flow theory (see e.g. the MIT 6.854 notes,
//! Ahuja-Magnanti-Orlin) guarantees that an integral optimum exists, so
//! this restriction does not change the optimal value on integer
//! instances.
//!
//! # Lexicographic encoding
//!
//! The lexicographic objective `(max |f|, min cost(f))` is encoded as a
//! single scalar score
//!
//! `score = M * (max_possible_flow - |f|) + cost(f)`
//!
//! where `M = sum_e c(e) * cost(e) + 1` strictly exceeds any feasible
//! cost. Minimizing this scalar therefore minimizes
//! `max_possible_flow - |f|` first (i.e. maximizes `|f|`), then breaks
//! ties by `cost(f)`. The optimum is always non-negative, and a smaller
//! score is strictly better in the lex order.

use crate::registry::{ConstructionError, FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumCostMaximumFlow",
        display_name: "Minimum-Cost Maximum-Flow",
        aliases: &["MCMF"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Integral flow that lexicographically maximizes value then minimizes total arc cost",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed graph G = (V, A)" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t" },
            FieldInfo { name: "capacities", type_name: "Vec<i64>", description: "Arc capacity c(a) in graph arc order (non-negative)" },
            FieldInfo { name: "costs", type_name: "Vec<i64>", description: "Arc cost cost(a) in graph arc order (non-negative)" },
        ],
    }
}

/// Minimum-Cost Maximum-Flow problem.
///
/// # Variables
///
/// `|A|` variables: variable `a` ranges over `{0, ..., c(a)}`
/// representing the integral flow on arc `a`.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumCostMaximumFlow;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, BruteForce};
///
/// // Diamond network from the canonical example.
/// let graph = DirectedGraph::new(4, vec![
///     (0, 1), (0, 2), (1, 2), (1, 3), (2, 3),
/// ]);
/// let problem = MinimumCostMaximumFlow::new(
///     graph,
///     0, 3,
///     vec![2, 1, 1, 1, 2], // capacities
///     vec![1, 0, 0, 1, 2], // costs
/// );
/// let solver = BruteForce::new();
/// let witness = solver.solve(&problem).unwrap().unwrap();
/// // Optimal flow has value 3 and cost 7.
/// assert_eq!(problem.flow_value(&witness).unwrap(), 3);
/// assert_eq!(problem.total_cost(&witness).unwrap(), 7);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MinimumCostMaximumFlow {
    /// The directed graph G = (V, A).
    graph: DirectedGraph,
    /// Source vertex s.
    source: usize,
    /// Sink vertex t.
    sink: usize,
    /// Capacity c(a) for each arc.
    capacities: Vec<i64>,
    /// Cost cost(a) for each arc.
    costs: Vec<i64>,
}

#[derive(Deserialize)]
struct MinimumCostMaximumFlowSerde {
    graph: DirectedGraph,
    source: usize,
    sink: usize,
    capacities: Vec<i64>,
    costs: Vec<i64>,
}

impl TryFrom<MinimumCostMaximumFlowSerde> for MinimumCostMaximumFlow {
    type Error = ConstructionError;

    fn try_from(value: MinimumCostMaximumFlowSerde) -> Result<Self, Self::Error> {
        Self::try_new(
            value.graph,
            value.source,
            value.sink,
            value.capacities,
            value.costs,
        )
    }
}

impl<'de> Deserialize<'de> for MinimumCostMaximumFlow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        MinimumCostMaximumFlowSerde::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

impl MinimumCostMaximumFlow {
    /// Create a new Minimum-Cost Maximum-Flow problem.
    ///
    /// # Panics
    ///
    /// Panics if any of the following holds:
    /// - `capacities.len() != graph.num_arcs()`
    /// - `costs.len() != graph.num_arcs()`
    /// - `source >= graph.num_vertices()`
    /// - `sink >= graph.num_vertices()`
    /// - `source == sink`
    /// - Any capacity is negative
    /// - Any cost is negative
    pub fn new(
        graph: DirectedGraph,
        source: usize,
        sink: usize,
        capacities: Vec<i64>,
        costs: Vec<i64>,
    ) -> Self {
        Self::try_new(graph, source, sink, capacities, costs)
            .unwrap_or_else(|error| panic!("{error}"))
    }

    fn try_new(
        graph: DirectedGraph,
        source: usize,
        sink: usize,
        capacities: Vec<i64>,
        costs: Vec<i64>,
    ) -> Result<Self, ConstructionError> {
        let n = graph.num_vertices();
        let m = graph.num_arcs();
        if capacities.len() != m {
            return Err(format!(
                "capacities length ({}) must match num_arcs ({m})",
                capacities.len()
            )
            .into());
        }
        if costs.len() != m {
            return Err(format!("costs length ({}) must match num_arcs ({m})", costs.len()).into());
        }
        if source >= n {
            return Err(format!("source ({source}) >= num_vertices ({n})").into());
        }
        if sink >= n {
            return Err(format!("sink ({sink}) >= num_vertices ({n})").into());
        }
        if source == sink {
            return Err("source and sink must be distinct".into());
        }
        if let Some((index, capacity)) = capacities
            .iter()
            .enumerate()
            .find(|(_, capacity)| **capacity < 0)
        {
            return Err(format!("capacity[{index}] = {capacity} is negative").into());
        }
        if let Some((index, cost)) = costs.iter().enumerate().find(|(_, cost)| **cost < 0) {
            return Err(format!("cost[{index}] = {cost} is negative").into());
        }
        Ok(Self {
            graph,
            source,
            sink,
            capacities,
            costs,
        })
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the sink vertex.
    pub fn sink(&self) -> usize {
        self.sink
    }

    /// Get a reference to the arc capacities.
    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    /// Get a reference to the arc costs.
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

    /// Check whether a flow assignment is feasible.
    ///
    /// A flow is feasible iff
    /// 1. `config.len() == num_arcs`,
    /// 2. each `0 <= f(a) <= c(a)`, and
    /// 3. flow is conserved at every non-terminal vertex.
    pub fn is_feasible(&self, config: &[usize]) -> Result<bool, crate::traits::EvaluationError> {
        let m = self.graph.num_arcs();
        if config.len() != m {
            return Ok(false);
        }
        // (1) Capacity constraints
        for (flow, cap) in config.iter().zip(self.capacities.iter()) {
            let flow = i64::try_from(*flow).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting maximum-flow configuration value".to_string(),
                )
            })?;
            if flow > *cap {
                return Ok(false);
            }
        }
        // (2) Flow conservation at non-terminal vertices
        let n = self.graph.num_vertices();
        let mut balance = vec![0_i64; n];
        for (a, &(u, v)) in self.graph.arcs().iter().enumerate() {
            let flow = i64::try_from(config[a]).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting maximum-flow configuration value".to_string(),
                )
            })?;
            balance[u] = balance[u].checked_sub(flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing maximum-flow vertex balance".to_string(),
                )
            })?;
            balance[v] = balance[v].checked_add(flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing maximum-flow vertex balance".to_string(),
                )
            })?;
        }
        for (v, &bal) in balance.iter().enumerate() {
            if v != self.source && v != self.sink && bal != 0 {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Compute the flow value `|f|` = net outflow from the source for a
    /// feasible configuration. Result is meaningless if `config` is not
    /// feasible.
    pub fn flow_value(&self, config: &[usize]) -> Result<i64, crate::traits::EvaluationError> {
        let mut net_out: i64 = 0;
        for (a, &(u, v)) in self.graph.arcs().iter().enumerate() {
            let f = i64::try_from(config[a]).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting maximum-flow configuration value".to_string(),
                )
            })?;
            if u == self.source {
                net_out = net_out.checked_add(f).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "computing maximum-flow value".to_string(),
                    )
                })?;
            }
            if v == self.source {
                net_out = net_out.checked_sub(f).ok_or_else(|| {
                    crate::traits::EvaluationError::IntegerOverflow(
                        "computing maximum-flow value".to_string(),
                    )
                })?;
            }
        }
        Ok(net_out)
    }

    /// Compute the total cost `sum_a cost(a) * f(a)` of a flow.
    pub fn total_cost(&self, config: &[usize]) -> Result<i64, crate::traits::EvaluationError> {
        let mut total = 0_i64;
        for (&flow, &cost) in config.iter().zip(self.costs.iter()) {
            let flow = i64::try_from(flow).map_err(|_| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "converting maximum-flow configuration value".to_string(),
                )
            })?;
            let term = flow.checked_mul(cost).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "multiplying maximum-flow arc cost".to_string(),
                )
            })?;
            total = total.checked_add(term).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing maximum-flow costs".to_string(),
                )
            })?;
        }
        Ok(total)
    }

    /// Upper bound on the integral flow value: `sum_a c(a)` (a trivial
    /// but valid bound, since `|f|` is bounded by the total capacity).
    fn max_possible_flow(&self) -> Result<i64, crate::traits::EvaluationError> {
        self.capacities.iter().try_fold(0_i64, |total, &capacity| {
            total.checked_add(capacity).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing maximum-flow capacities".to_string(),
                )
            })
        })
    }

    /// Strict upper bound on any feasible cost, used as the
    /// lex-multiplier `M` so that the scalar `score = M * (B - |f|)
    /// + cost(f)` orders by `(max |f|, min cost(f))`.
    fn cost_multiplier(&self) -> Result<i64, crate::traits::EvaluationError> {
        let mut total = 0_i64;
        for (&capacity, &cost) in self.capacities.iter().zip(self.costs.iter()) {
            let term = capacity.checked_mul(cost).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "multiplying maximum-flow capacity by cost".to_string(),
                )
            })?;
            total = total.checked_add(term).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing maximum-flow cost bounds".to_string(),
                )
            })?;
        }
        total.checked_add(1).ok_or_else(|| {
            crate::traits::EvaluationError::IntegerOverflow(
                "forming maximum-flow cost multiplier".to_string(),
            )
        })
    }
}

impl Problem for MinimumCostMaximumFlow {
    const NAME: &'static str = "MinimumCostMaximumFlow";
    type Solution = Vec<usize>;
    type Value = crate::types::Min<i64>;

    crate::problem_parameters![("num_arcs", num_arcs), ("num_vertices", num_vertices),];

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Min<i64>, crate::traits::EvaluationError> {
        if config.len() != self.graph.num_arcs() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "flow vector length does not match the graph arcs".into(),
            ));
        }
        Ok({
            if !self.is_feasible(config)? {
                return Ok(crate::types::Min(None));
            }
            let m = self.cost_multiplier()?;
            let value = self.flow_value(config)?;
            let cost = self.total_cost(config)?;
            let bound = self.max_possible_flow()?;
            // score = M * (max_possible_flow - |f|) + cost(f)
            let remaining = bound.checked_sub(value).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "computing maximum-flow objective gap".to_string(),
                )
            })?;
            let penalty = m.checked_mul(remaining).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "multiplying maximum-flow objective penalty".to_string(),
                )
            })?;
            let score = penalty.checked_add(cost).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing maximum-flow objective".to_string(),
                )
            })?;
            crate::types::Min(Some(score))
        })
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl crate::solvers::BruteForceProblem for MinimumCostMaximumFlow {
    fn dimensions(&self) -> Vec<usize> {
        self.capacities.iter().map(|&c| (c as usize) + 1).collect()
    }
}

crate::declare_variants! {
    default MinimumCostMaximumFlow => "(num_vertices + num_arcs)^6",
}

crate::register_brute_force! {
    MinimumCostMaximumFlow,
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    let problem = MinimumCostMaximumFlow::new(
        crate::topology::DirectedGraph::new(4, vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]),
        0,
        3,
        vec![2, 1, 1, 1, 2],
        vec![1, 0, 0, 1, 2],
    );
    // Optimal flow has value 3, routed as:
    //   - 1 unit on 0->1->3        via arcs 0,3 (cost 1 + 1 = 2)
    //   - 1 unit on 0->1->2->3     via arcs 0,2,4 (cost 1 + 0 + 2 = 3)
    //   - 1 unit on 0->2->3        via arcs 1,4 (cost 0 + 2 = 2)
    // Arc flows sum to f = [2, 1, 1, 1, 2]: value = 3,
    // cost = 2*1 + 1*0 + 1*0 + 1*1 + 2*2 = 7.
    let optimal_config = vec![2, 1, 1, 1, 2];
    let optimal_value = problem
        .evaluate(&optimal_config)
        .expect("canonical example evaluation must succeed");
    let scalar = match optimal_value {
        crate::types::Min(Some(v)) => v,
        crate::types::Min(None) => panic!("canonical example must be feasible"),
    };
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_cost_maximum_flow",
        instance: Box::new(problem),
        optimal_config: serde_json::to_value(optimal_config)
            .expect("solution serialization must succeed"),
        optimal_value: serde_json::json!(scalar),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_cost_maximum_flow.rs"]
mod tests;
