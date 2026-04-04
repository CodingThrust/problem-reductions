//! Minimum Edge-Cost Flow problem implementation.
//!
//! Given a directed graph G = (V, A) with arc capacities c(a) and prices p(a),
//! a source s, a sink t, and a flow requirement R, find an integral flow of
//! value at least R that minimizes the total edge cost — the sum of prices of
//! arcs carrying nonzero flow.
//!
//! This is NP-hard: it generalizes Minimum-Weight Satisfiability via reduction
//! from Minimum Edge-Cost Flow on DAGs (Amaldi et al., 2011).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::Problem;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumEdgeCostFlow",
        display_name: "Minimum Edge-Cost Flow",
        aliases: &["MECF"],
        dimensions: &[],
        module_path: module_path!(),
        description: "Integral flow minimizing the number of arcs with nonzero flow (weighted by price)",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed graph G = (V, A)" },
            FieldInfo { name: "prices", type_name: "Vec<i64>", description: "Price p(a) for each arc" },
            FieldInfo { name: "capacities", type_name: "Vec<i64>", description: "Capacity c(a) for each arc" },
            FieldInfo { name: "source", type_name: "usize", description: "Source vertex s" },
            FieldInfo { name: "sink", type_name: "usize", description: "Sink vertex t" },
            FieldInfo { name: "required_flow", type_name: "i64", description: "Flow requirement R" },
        ],
    }
}

/// Minimum Edge-Cost Flow problem.
///
/// Given a directed graph G = (V, A) with arc capacities c(a) and prices p(a),
/// source s, sink t, and flow requirement R, find an integral flow f: A -> Z_0^+
/// of value at least R minimizing the total edge cost sum_{a: f(a)>0} p(a).
///
/// # Variables
///
/// |A| variables: variable a ranges over {0, ..., c(a)} representing the flow
/// on arc a.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::MinimumEdgeCostFlow;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 5-vertex network: s=0, t=4, R=3
/// let graph = DirectedGraph::new(5, vec![
///     (0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4),
/// ]);
/// let problem = MinimumEdgeCostFlow::new(
///     graph,
///     vec![3, 1, 2, 0, 0, 0], // prices
///     vec![2, 2, 2, 2, 2, 2], // capacities
///     0, 4, 3,
/// );
/// let solver = BruteForce::new();
/// let witness = solver.find_witness(&problem).unwrap();
/// assert_eq!(problem.evaluate(&witness), problemreductions::types::Min(Some(3)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumEdgeCostFlow {
    /// The directed graph G = (V, A).
    graph: DirectedGraph,
    /// Price p(a) for each arc.
    prices: Vec<i64>,
    /// Capacity c(a) for each arc.
    capacities: Vec<i64>,
    /// Source vertex s.
    source: usize,
    /// Sink vertex t.
    sink: usize,
    /// Flow requirement R.
    required_flow: i64,
}

impl MinimumEdgeCostFlow {
    /// Create a new Minimum Edge-Cost Flow problem.
    ///
    /// # Arguments
    ///
    /// * `graph` - Directed graph G = (V, A)
    /// * `prices` - Price p(a) for each arc (one per arc)
    /// * `capacities` - Capacity c(a) for each arc (one per arc, all non-negative)
    /// * `source` - Source vertex index
    /// * `sink` - Sink vertex index
    /// * `required_flow` - Minimum flow requirement R
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `prices.len() != graph.num_arcs()`
    /// - `capacities.len() != graph.num_arcs()`
    /// - `source >= graph.num_vertices()`
    /// - `sink >= graph.num_vertices()`
    /// - `source == sink`
    /// - Any capacity is negative
    pub fn new(
        graph: DirectedGraph,
        prices: Vec<i64>,
        capacities: Vec<i64>,
        source: usize,
        sink: usize,
        required_flow: i64,
    ) -> Self {
        let n = graph.num_vertices();
        let m = graph.num_arcs();
        assert_eq!(
            prices.len(),
            m,
            "prices length ({}) must match num_arcs ({m})",
            prices.len()
        );
        assert_eq!(
            capacities.len(),
            m,
            "capacities length ({}) must match num_arcs ({m})",
            capacities.len()
        );
        assert!(source < n, "source ({source}) >= num_vertices ({n})");
        assert!(sink < n, "sink ({sink}) >= num_vertices ({n})");
        assert_ne!(source, sink, "source and sink must be distinct");
        for (i, &c) in capacities.iter().enumerate() {
            assert!(c >= 0, "capacity[{i}] = {c} is negative");
        }
        Self {
            graph,
            prices,
            capacities,
            source,
            sink,
            required_flow,
        }
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get a reference to the prices.
    pub fn prices(&self) -> &[i64] {
        &self.prices
    }

    /// Get a reference to the capacities.
    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    /// Get the source vertex.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the sink vertex.
    pub fn sink(&self) -> usize {
        self.sink
    }

    /// Get the flow requirement.
    pub fn required_flow(&self) -> i64 {
        self.required_flow
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges (arcs).
    pub fn num_edges(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Get the maximum capacity across all arcs (0 if empty).
    pub fn max_capacity(&self) -> i64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }

    /// Get a reference to the edges (arcs).
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph.arcs()
    }

    /// Check whether a flow assignment is feasible.
    ///
    /// A flow is feasible if:
    /// 1. Each arc's flow does not exceed its capacity
    /// 2. Flow is conserved at every non-terminal vertex
    /// 3. Net flow into the sink is at least the required flow
    pub fn is_feasible(&self, config: &[usize]) -> bool {
        let m = self.graph.num_arcs();
        if config.len() != m {
            return false;
        }
        let arcs = self.graph.arcs();

        // (1) Capacity constraints
        for (flow, cap) in config.iter().zip(self.capacities.iter()) {
            if (*flow as i64) > *cap {
                return false;
            }
        }

        // (2) Flow conservation at non-terminal vertices
        let n = self.graph.num_vertices();
        let mut balance = vec![0_i64; n];
        for (a, &(u, v)) in arcs.iter().enumerate() {
            let flow = config[a] as i64;
            balance[u] -= flow;
            balance[v] += flow;
        }

        for (v, &bal) in balance.iter().enumerate() {
            if v != self.source && v != self.sink && bal != 0 {
                return false;
            }
        }

        // (3) Flow requirement: net flow into sink >= R
        if balance[self.sink] < self.required_flow {
            return false;
        }

        true
    }

    /// Compute the edge cost for a feasible flow: sum of prices of arcs with
    /// nonzero flow.
    pub fn edge_cost(&self, config: &[usize]) -> i64 {
        config
            .iter()
            .enumerate()
            .filter(|(_, &f)| f > 0)
            .map(|(a, _)| self.prices[a])
            .sum()
    }
}

impl Problem for MinimumEdgeCostFlow {
    const NAME: &'static str = "MinimumEdgeCostFlow";
    type Value = crate::types::Min<i64>;

    fn dims(&self) -> Vec<usize> {
        self.capacities.iter().map(|&c| (c as usize) + 1).collect()
    }

    fn evaluate(&self, config: &[usize]) -> crate::types::Min<i64> {
        if self.is_feasible(config) {
            crate::types::Min(Some(self.edge_cost(config)))
        } else {
            crate::types::Min(None)
        }
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

crate::declare_variants! {
    default MinimumEdgeCostFlow => "(max_capacity + 1)^num_edges",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "minimum_edge_cost_flow",
        instance: Box::new(MinimumEdgeCostFlow::new(
            crate::topology::DirectedGraph::new(
                5,
                vec![(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4)],
            ),
            vec![3, 1, 2, 0, 0, 0], // prices
            vec![2, 2, 2, 2, 2, 2], // capacities
            0,
            4,
            3,
        )),
        // Optimal: route 1 unit via v2 and 2 units via v3 → cost = 1 + 2 = 3
        // config = [0, 1, 2, 0, 1, 2]
        optimal_config: vec![0, 1, 2, 0, 1, 2],
        optimal_value: serde_json::json!(3),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_edge_cost_flow.rs"]
mod tests;
