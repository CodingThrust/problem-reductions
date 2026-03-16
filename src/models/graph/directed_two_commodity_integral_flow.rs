//! Directed Two-Commodity Integral Flow problem implementation.
//!
//! Given a directed graph with arc capacities and two source-sink pairs with
//! flow requirements, determine whether two integral flow functions exist that
//! jointly satisfy capacity, conservation, and requirement constraints.
//!
//! NP-complete even with unit capacities (Even, Itai, and Shamir, 1976).

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::DirectedGraph;
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "DirectedTwoCommodityIntegralFlow",
        display_name: "Directed Two-Commodity Integral Flow",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Two-commodity integral flow feasibility on a directed graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "DirectedGraph", description: "Directed graph G = (V, A)" },
            FieldInfo { name: "capacities", type_name: "Vec<u64>", description: "Capacity c(a) for each arc" },
            FieldInfo { name: "source_1", type_name: "usize", description: "Source vertex s_1 for commodity 1" },
            FieldInfo { name: "sink_1", type_name: "usize", description: "Sink vertex t_1 for commodity 1" },
            FieldInfo { name: "source_2", type_name: "usize", description: "Source vertex s_2 for commodity 2" },
            FieldInfo { name: "sink_2", type_name: "usize", description: "Sink vertex t_2 for commodity 2" },
            FieldInfo { name: "requirement_1", type_name: "u64", description: "Flow requirement R_1 for commodity 1" },
            FieldInfo { name: "requirement_2", type_name: "u64", description: "Flow requirement R_2 for commodity 2" },
        ],
    }
}

/// Directed Two-Commodity Integral Flow problem.
///
/// Given a directed graph G = (V, A) with arc capacities c(a), two source-sink
/// pairs (s_1, t_1) and (s_2, t_2), and requirements R_1, R_2, determine
/// whether two integral flow functions f_1, f_2: A -> Z_0^+ exist such that:
/// 1. Joint capacity: f_1(a) + f_2(a) <= c(a) for all a in A
/// 2. Flow conservation: for each commodity i, flow is conserved at every
///    vertex except the four terminals
/// 3. Requirements: net flow into t_i under f_i is at least R_i
///
/// # Variables
///
/// 2|A| variables: first |A| for commodity 1's flow on each arc,
/// next |A| for commodity 2's flow on each arc. Variable j for arc a
/// of commodity i ranges over {0, ..., c(a)}.
///
/// # Example
///
/// ```
/// use problemreductions::models::graph::DirectedTwoCommodityIntegralFlow;
/// use problemreductions::topology::DirectedGraph;
/// use problemreductions::{Problem, Solver, BruteForce};
///
/// // 6-vertex network: s1=0, s2=1, t1=4, t2=5
/// let graph = DirectedGraph::new(6, vec![
///     (0, 2), (0, 3), (1, 2), (1, 3),
///     (2, 4), (2, 5), (3, 4), (3, 5),
/// ]);
/// let problem = DirectedTwoCommodityIntegralFlow::new(
///     graph, vec![1; 8], 0, 4, 1, 5, 1, 1,
/// );
/// let solver = BruteForce::new();
/// assert!(solver.find_satisfying(&problem).is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedTwoCommodityIntegralFlow {
    /// The directed graph G = (V, A).
    graph: DirectedGraph,
    /// Capacity c(a) for each arc.
    capacities: Vec<u64>,
    /// Source vertex s_1 for commodity 1.
    source_1: usize,
    /// Sink vertex t_1 for commodity 1.
    sink_1: usize,
    /// Source vertex s_2 for commodity 2.
    source_2: usize,
    /// Sink vertex t_2 for commodity 2.
    sink_2: usize,
    /// Flow requirement R_1 for commodity 1.
    requirement_1: u64,
    /// Flow requirement R_2 for commodity 2.
    requirement_2: u64,
}

impl DirectedTwoCommodityIntegralFlow {
    /// Create a new Directed Two-Commodity Integral Flow problem.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `capacities.len() != graph.num_arcs()`
    /// - Any terminal vertex index >= `graph.num_vertices()`
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: DirectedGraph,
        capacities: Vec<u64>,
        source_1: usize,
        sink_1: usize,
        source_2: usize,
        sink_2: usize,
        requirement_1: u64,
        requirement_2: u64,
    ) -> Self {
        let n = graph.num_vertices();
        assert_eq!(
            capacities.len(),
            graph.num_arcs(),
            "capacities length must match graph num_arcs"
        );
        assert!(source_1 < n, "source_1 ({source_1}) >= num_vertices ({n})");
        assert!(sink_1 < n, "sink_1 ({sink_1}) >= num_vertices ({n})");
        assert!(source_2 < n, "source_2 ({source_2}) >= num_vertices ({n})");
        assert!(sink_2 < n, "sink_2 ({sink_2}) >= num_vertices ({n})");
        Self {
            graph,
            capacities,
            source_1,
            sink_1,
            source_2,
            sink_2,
            requirement_1,
            requirement_2,
        }
    }

    /// Get a reference to the underlying directed graph.
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    /// Get a reference to the capacities.
    pub fn capacities(&self) -> &[u64] {
        &self.capacities
    }

    /// Get source vertex for commodity 1.
    pub fn source_1(&self) -> usize {
        self.source_1
    }

    /// Get sink vertex for commodity 1.
    pub fn sink_1(&self) -> usize {
        self.sink_1
    }

    /// Get source vertex for commodity 2.
    pub fn source_2(&self) -> usize {
        self.source_2
    }

    /// Get sink vertex for commodity 2.
    pub fn sink_2(&self) -> usize {
        self.sink_2
    }

    /// Get requirement for commodity 1.
    pub fn requirement_1(&self) -> u64 {
        self.requirement_1
    }

    /// Get requirement for commodity 2.
    pub fn requirement_2(&self) -> u64 {
        self.requirement_2
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of arcs.
    pub fn num_arcs(&self) -> usize {
        self.graph.num_arcs()
    }

    /// Get the maximum capacity across all arcs.
    pub fn max_capacity(&self) -> u64 {
        self.capacities.iter().copied().max().unwrap_or(0)
    }

    /// Check whether a flow assignment is feasible.
    ///
    /// `config` has 2*|A| entries: first |A| for commodity 1, next |A| for commodity 2.
    pub fn is_feasible(&self, config: &[usize]) -> bool {
        let m = self.graph.num_arcs();
        if config.len() != 2 * m {
            return false;
        }
        let arcs = self.graph.arcs();
        let terminals = [self.source_1, self.sink_1, self.source_2, self.sink_2];

        // (1) Joint capacity constraint
        for a in 0..m {
            let f1 = config[a] as u64;
            let f2 = config[m + a] as u64;
            if f1 + f2 > self.capacities[a] {
                return false;
            }
        }

        // (2) Flow conservation for each commodity at non-terminal vertices
        // Per the definition, conservation applies at every vertex EXCEPT
        // the four terminals {s_1, s_2, t_1, t_2} for BOTH commodities.
        let n = self.graph.num_vertices();
        for commodity in 0..2usize {
            let offset = commodity * m;
            for v in 0..n {
                if terminals.contains(&v) {
                    continue;
                }
                let mut flow_in: i64 = 0;
                let mut flow_out: i64 = 0;
                for (a, &(u, w)) in arcs.iter().enumerate() {
                    if w == v {
                        flow_in += config[offset + a] as i64;
                    }
                    if u == v {
                        flow_out += config[offset + a] as i64;
                    }
                }
                if flow_in != flow_out {
                    return false;
                }
            }
        }

        // (3) Requirement constraints
        // Net flow into sink for commodity i = (total flow into sink) - (total flow out of sink)
        for commodity in 0..2usize {
            let offset = commodity * m;
            let snk = if commodity == 0 {
                self.sink_1
            } else {
                self.sink_2
            };
            let req = if commodity == 0 {
                self.requirement_1
            } else {
                self.requirement_2
            };
            let mut net_flow_in: i64 = 0;
            for (a, &(u, w)) in arcs.iter().enumerate() {
                if w == snk {
                    net_flow_in += config[offset + a] as i64;
                }
                if u == snk {
                    net_flow_in -= config[offset + a] as i64;
                }
            }
            if (net_flow_in as u64) < req {
                return false;
            }
        }

        true
    }
}

impl Problem for DirectedTwoCommodityIntegralFlow {
    const NAME: &'static str = "DirectedTwoCommodityIntegralFlow";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        self.capacities
            .iter()
            .chain(self.capacities.iter())
            .map(|&c| (c as usize) + 1)
            .collect()
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        self.is_feasible(config)
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for DirectedTwoCommodityIntegralFlow {}

crate::declare_variants! {
    default sat DirectedTwoCommodityIntegralFlow => "(max_capacity + 1)^(2 * num_arcs)",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "directed_two_commodity_integral_flow",
        build: || {
            let graph = DirectedGraph::new(
                6,
                vec![
                    (0, 2),
                    (0, 3),
                    (1, 2),
                    (1, 3),
                    (2, 4),
                    (2, 5),
                    (3, 4),
                    (3, 5),
                ],
            );
            let problem =
                DirectedTwoCommodityIntegralFlow::new(graph, vec![1; 8], 0, 4, 1, 5, 1, 1);
            // Solution: commodity 1 path 0->2->4 (arcs 0,4), commodity 2 path 1->3->5 (arcs 3,7)
            // config = [f1(a0)..f1(a7), f2(a0)..f2(a7)]
            //        = [1,0,0,0,1,0,0,0, 0,0,0,1,0,0,0,1]
            crate::example_db::specs::satisfaction_example(
                problem,
                vec![vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1]],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/directed_two_commodity_integral_flow.rs"]
mod tests;
