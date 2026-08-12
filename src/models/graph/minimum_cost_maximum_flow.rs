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
//! continuous flows `f: A -> R_{>= 0}`, but the [`Problem`] trait
//! requires a discrete configuration space via [`Problem::dims`].
//! Following the same precedent as
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

use crate::registry::{FieldInfo, ProblemSchemaEntry, ProblemSizeFieldEntry};
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

inventory::submit! {
    ProblemSizeFieldEntry {
        name: "MinimumCostMaximumFlow",
        fields: &["num_vertices", "num_arcs"],
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
/// use problemreductions::{Problem, Solver, BruteForce};
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
/// let witness = solver.find_witness(&problem).unwrap();
/// // Optimal flow has value 3 and cost 7.
/// assert_eq!(problem.flow_value(&witness), 3);
/// assert_eq!(problem.total_cost(&witness), 7);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let n = graph.num_vertices();
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
        assert!(source < n, "source ({source}) >= num_vertices ({n})");
        assert!(sink < n, "sink ({sink}) >= num_vertices ({n})");
        assert_ne!(source, sink, "source and sink must be distinct");
        for (i, &c) in capacities.iter().enumerate() {
            assert!(c >= 0, "capacity[{i}] = {c} is negative");
        }
        for (i, &c) in costs.iter().enumerate() {
            assert!(c >= 0, "cost[{i}] = {c} is negative");
        }
        Self {
            graph,
            source,
            sink,
            capacities,
            costs,
        }
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
    pub fn is_feasible(&self, config: &[usize]) -> bool {
        let m = self.graph.num_arcs();
        if config.len() != m {
            return false;
        }
        // (1) Capacity constraints
        for (flow, cap) in config.iter().zip(self.capacities.iter()) {
            if (*flow as i64) > *cap {
                return false;
            }
        }
        // (2) Flow conservation at non-terminal vertices
        let n = self.graph.num_vertices();
        let mut balance = vec![0_i64; n];
        for (a, &(u, v)) in self.graph.arcs().iter().enumerate() {
            let flow = config[a] as i64;
            balance[u] -= flow;
            balance[v] += flow;
        }
        for (v, &bal) in balance.iter().enumerate() {
            if v != self.source && v != self.sink && bal != 0 {
                return false;
            }
        }
        true
    }

    /// Compute the flow value `|f|` = net outflow from the source for a
    /// feasible configuration. Result is meaningless if `config` is not
    /// feasible.
    pub fn flow_value(&self, config: &[usize]) -> i64 {
        let mut net_out: i64 = 0;
        for (a, &(u, v)) in self.graph.arcs().iter().enumerate() {
            let f = config[a] as i64;
            if u == self.source {
                net_out += f;
            }
            if v == self.source {
                net_out -= f;
            }
        }
        net_out
    }

    /// Compute the total cost `sum_a cost(a) * f(a)` of a flow.
    pub fn total_cost(&self, config: &[usize]) -> i64 {
        config
            .iter()
            .zip(self.costs.iter())
            .map(|(&f, &c)| (f as i64) * c)
            .sum()
    }

    /// Upper bound on the integral flow value: `sum_a c(a)` (a trivial
    /// but valid bound, since `|f|` is bounded by the total capacity).
    fn max_possible_flow(&self) -> i64 {
        self.capacities.iter().sum()
    }

    /// Strict upper bound on any feasible cost, used as the
    /// lex-multiplier `M` so that the scalar `score = M * (B - |f|)
    /// + cost(f)` orders by `(max |f|, min cost(f))`.
    fn cost_multiplier(&self) -> i64 {
        self.capacities
            .iter()
            .zip(self.costs.iter())
            .map(|(&c, &k)| c * k)
            .sum::<i64>()
            + 1
    }
}

impl Problem for MinimumCostMaximumFlow {
    const NAME: &'static str = "MinimumCostMaximumFlow";
    type Value = crate::types::Min<i64>;

    fn dims(&self) -> Vec<usize> {
        self.capacities.iter().map(|&c| (c as usize) + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Min<i64> {
        if !self.is_feasible(config) {
            return crate::types::Min(None);
        }
        let m = self.cost_multiplier();
        let value = self.flow_value(config);
        let cost = self.total_cost(config);
        let bound = self.max_possible_flow();
        // score = M * (max_possible_flow - |f|) + cost(f)
        crate::types::Min(Some(m * (bound - value) + cost))
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default MinimumCostMaximumFlow => "(num_vertices + num_arcs)^6",
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
    let optimal_config = vec![2_usize, 1, 1, 1, 2];
    let optimal_value = problem.evaluate(&optimal_config);
    let scalar = match optimal_value {
        crate::types::Min(Some(v)) => v,
        crate::types::Min(None) => panic!("canonical example must be feasible"),
    };
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_cost_maximum_flow",
        instance: Box::new(problem),
        optimal_config,
        optimal_value: serde_json::json!(scalar),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_cost_maximum_flow.rs"]
mod tests;
