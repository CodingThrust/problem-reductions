//! Undirected flow with lower bounds problem implementation.
//!
//! Given an undirected graph with per-edge lower and upper capacities, a
//! source, a sink, and a required net flow value, determine whether there
//! exists an orientation and feasible directed flow meeting all bounds.
//!
//! The configuration space stores one binary orientation choice per edge in the
//! graph's edge order:
//! - `0` means orient the stored edge `(u, v)` as `u -> v`
//! - `1` means orient it as `v -> u`
//!
//! For a fixed orientation, feasibility reduces to a directed circulation with
//! lower bounds, so the registered exact complexity matches brute-force
//! enumeration over the `2^|E|` edge orientations.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "UndirectedFlowLowerBounds",
        display_name: "Undirected Flow with Lower Bounds",
        aliases: &[],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Graph,
        module_path: module_path!(),
        description: "Determine whether an undirected lower-bounded flow of value at least R exists",
        fields: UndirectedFlowLowerBoundsCreateSpec::FIELDS,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndirectedFlowLowerBounds {
    graph: SimpleGraph,
    capacities: Vec<i64>,
    lower_bounds: Vec<i64>,
    source: usize,
    sink: usize,
    requirement: i64,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct UndirectedFlowLowerBoundsCreateSpec {
    /// Undirected graph.
    graph: SimpleGraph,
    /// Upper capacities in graph edge order.
    capacities: Vec<i64>,
    /// Lower bounds in graph edge order.
    lower_bounds: Vec<i64>,
    /// Source vertex.
    source: usize,
    /// Sink vertex.
    sink: usize,
    /// Required net inflow at the sink.
    requirement: i64,
}
impl TryFrom<UndirectedFlowLowerBoundsCreateSpec> for UndirectedFlowLowerBounds {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: UndirectedFlowLowerBoundsCreateSpec) -> Result<Self, Self::Error> {
        let edges = spec.graph.num_edges();
        if spec.capacities.len() != edges {
            return Err(format!(
                "capacities has {} entries, expected {edges}",
                spec.capacities.len()
            )
            .into());
        }
        if spec.lower_bounds.len() != edges {
            return Err(format!(
                "lower_bounds has {} entries, expected {edges}",
                spec.lower_bounds.len()
            )
            .into());
        }
        let vertices = spec.graph.num_vertices();
        if spec.source >= vertices || spec.sink >= vertices {
            return Err("source and sink must be valid graph vertices"
                .to_string()
                .into());
        }
        if spec.source == spec.sink {
            return Err("source and sink must be distinct".to_string().into());
        }
        if spec.requirement == 0 {
            return Err("requirement must be at least 1".to_string().into());
        }
        if let Some((index, _)) = spec
            .lower_bounds
            .iter()
            .zip(&spec.capacities)
            .enumerate()
            .find(|(_, (&lower, &upper))| lower > upper)
        {
            return Err(format!("lower bound at edge {index} exceeds its capacity").into());
        }
        Ok(Self::new(
            spec.graph,
            spec.capacities,
            spec.lower_bounds,
            spec.source,
            spec.sink,
            spec.requirement,
        ))
    }
}

impl UndirectedFlowLowerBounds {
    pub fn new(
        graph: SimpleGraph,
        capacities: Vec<i64>,
        lower_bounds: Vec<i64>,
        source: usize,
        sink: usize,
        requirement: i64,
    ) -> Self {
        assert_eq!(
            capacities.len(),
            graph.num_edges(),
            "capacities length must match graph num_edges"
        );
        assert_eq!(
            lower_bounds.len(),
            graph.num_edges(),
            "lower_bounds length must match graph num_edges"
        );

        let num_vertices = graph.num_vertices();
        assert!(
            source < num_vertices,
            "source must be less than num_vertices ({num_vertices})"
        );
        assert!(
            sink < num_vertices,
            "sink must be less than num_vertices ({num_vertices})"
        );
        assert!(source != sink, "source and sink must be distinct");
        assert!(requirement >= 1, "requirement must be at least 1");

        for (edge_index, (&lower, &upper)) in lower_bounds.iter().zip(&capacities).enumerate() {
            assert!(
                lower <= upper,
                "lower bound at edge {edge_index} must be at most its capacity"
            );
        }

        Self {
            graph,
            capacities,
            lower_bounds,
            source,
            sink,
            requirement,
        }
    }

    pub fn graph(&self) -> &SimpleGraph {
        &self.graph
    }

    pub fn capacities(&self) -> &[i64] {
        &self.capacities
    }

    pub fn lower_bounds(&self) -> &[i64] {
        &self.lower_bounds
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn sink(&self) -> usize {
        self.sink
    }

    pub fn requirement(&self) -> i64 {
        self.requirement
    }

    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    pub fn is_valid_solution(
        &self,
        config: &[bool],
    ) -> Result<bool, crate::traits::EvaluationError> {
        if config.len() != self.num_edges() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-orientation length does not match the graph".into(),
            ));
        }
        self.has_feasible_orientation(config)
    }

    fn total_capacity(&self) -> Result<i64, crate::traits::EvaluationError> {
        self.capacities.iter().try_fold(0_i64, |total, &capacity| {
            total.checked_add(capacity).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing undirected flow capacities".into(),
                )
            })
        })
    }

    fn has_feasible_orientation(
        &self,
        config: &[bool],
    ) -> Result<bool, crate::traits::EvaluationError> {
        if config.len() != self.num_edges() {
            return Ok(false);
        }

        let total_capacity = self.total_capacity()?;
        let requirement = self.requirement;
        if requirement > total_capacity {
            return Ok(false);
        }

        let node_count = self.num_vertices();
        let super_source = node_count;
        let super_sink = node_count + 1;
        let mut network = ResidualNetwork::new(node_count + 2);
        let mut balances = vec![0_i64; node_count];

        for (edge_index, ((u, v), &orientation)) in self
            .graph
            .edges()
            .into_iter()
            .zip(config.iter())
            .enumerate()
        {
            let (from, to) = if orientation { (v, u) } else { (u, v) };
            let lower = self.lower_bounds[edge_index];
            let upper = self.capacities[edge_index];
            if !add_lower_bounded_edge(&mut network, &mut balances, from, to, lower, upper)? {
                return Ok(false);
            }
        }

        if !add_lower_bounded_edge(
            &mut network,
            &mut balances,
            self.sink,
            self.source,
            requirement,
            total_capacity,
        )? {
            return Ok(false);
        }

        let mut demand = 0_i64;
        for (vertex, balance) in balances.into_iter().enumerate() {
            if balance > 0 {
                demand = match demand.checked_add(balance) {
                    Some(value) => value,
                    None => {
                        return Err(crate::traits::EvaluationError::IntegerOverflow(
                            "summing lower-bound flow demand".into(),
                        ));
                    }
                };
                network.add_edge(super_source, vertex, balance);
            } else if balance < 0 {
                network.add_edge(
                    vertex,
                    super_sink,
                    balance.checked_neg().ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "negating lower-bound flow balance".into(),
                        )
                    })?,
                );
            }
        }

        Ok(network.max_flow(super_source, super_sink)? == demand)
    }
}

impl Problem for UndirectedFlowLowerBounds {
    const NAME: &'static str = "UndirectedFlowLowerBounds";
    type Solution = Vec<bool>;
    type Value = crate::types::Or;

    crate::problem_size![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<crate::types::Or, crate::traits::EvaluationError> {
        Ok(crate::types::Or(self.is_valid_solution(config)?))
    }
}

impl crate::solvers::BruteForceProblem for UndirectedFlowLowerBounds {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_edges()]
    }
}

crate::declare_variants! {
    default UndirectedFlowLowerBounds => "2^num_edges" create UndirectedFlowLowerBoundsCreateSpec,
}

crate::register_brute_force! {
    UndirectedFlowLowerBounds decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "undirected_flow_lower_bounds",
        instance: Box::new(UndirectedFlowLowerBounds::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 4), (3, 5), (4, 5)],
            ),
            vec![2, 2, 2, 2, 1, 3, 2],
            vec![1, 1, 0, 0, 1, 0, 1],
            0,
            5,
            3,
        )),
        optimal_config: serde_json::json!(vec![false, false, false, false, false, false, false]),
        optimal_value: serde_json::json!(true),
    }]
}

#[derive(Debug, Clone)]
struct ResidualEdge {
    to: usize,
    rev: usize,
    capacity: i64,
}

#[derive(Debug, Clone)]
struct ResidualNetwork {
    adjacency: Vec<Vec<ResidualEdge>>,
}

impl ResidualNetwork {
    fn new(num_vertices: usize) -> Self {
        Self {
            adjacency: vec![Vec::new(); num_vertices],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: i64) {
        let reverse_at_to = self.adjacency[to].len();
        let reverse_at_from = self.adjacency[from].len();
        self.adjacency[from].push(ResidualEdge {
            to,
            rev: reverse_at_to,
            capacity,
        });
        self.adjacency[to].push(ResidualEdge {
            to: from,
            rev: reverse_at_from,
            capacity: 0,
        });
    }

    fn max_flow(
        &mut self,
        source: usize,
        sink: usize,
    ) -> Result<i64, crate::traits::EvaluationError> {
        let mut total_flow = 0_i64;

        loop {
            let mut parent: Vec<Option<(usize, usize)>> = vec![None; self.adjacency.len()];
            let mut queue = VecDeque::new();
            queue.push_back(source);
            parent[source] = Some((source, usize::MAX));

            while let Some(vertex) = queue.pop_front() {
                if vertex == sink {
                    break;
                }

                for (edge_index, edge) in self.adjacency[vertex].iter().enumerate() {
                    if edge.capacity == 0 || parent[edge.to].is_some() {
                        continue;
                    }
                    parent[edge.to] = Some((vertex, edge_index));
                    queue.push_back(edge.to);
                }
            }

            if parent[sink].is_none() {
                return Ok(total_flow);
            }

            let mut path_flow = i64::MAX;
            let mut vertex = sink;
            while vertex != source {
                let (prev, edge_index) = parent[vertex].expect("sink is reachable");
                path_flow = path_flow.min(self.adjacency[prev][edge_index].capacity);
                vertex = prev;
            }

            let mut vertex = sink;
            while vertex != source {
                let (prev, edge_index) = parent[vertex].expect("sink is reachable");
                let reverse_edge = self.adjacency[prev][edge_index].rev;
                self.adjacency[prev][edge_index].capacity = self.adjacency[prev][edge_index]
                    .capacity
                    .checked_sub(path_flow)
                    .ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "subtracting residual path flow".into(),
                        )
                    })?;
                self.adjacency[vertex][reverse_edge].capacity = self.adjacency[vertex]
                    [reverse_edge]
                    .capacity
                    .checked_add(path_flow)
                    .ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "adding reverse residual path flow".into(),
                        )
                    })?;
                vertex = prev;
            }

            total_flow = total_flow.checked_add(path_flow).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow("summing maximum flow".into())
            })?;
        }
    }
}

fn add_lower_bounded_edge(
    network: &mut ResidualNetwork,
    balances: &mut [i64],
    from: usize,
    to: usize,
    lower: i64,
    upper: i64,
) -> Result<bool, crate::traits::EvaluationError> {
    if lower > upper {
        return Ok(false);
    }

    let residual = upper.checked_sub(lower).ok_or_else(|| {
        crate::traits::EvaluationError::IntegerOverflow(
            "subtracting lower bound from flow capacity".into(),
        )
    })?;
    if residual > 0 {
        network.add_edge(from, to, residual);
    }

    balances[from] = balances[from].checked_sub(lower).ok_or_else(|| {
        crate::traits::EvaluationError::IntegerOverflow(
            "subtracting lower bound from source balance".into(),
        )
    })?;
    balances[to] = balances[to].checked_add(lower).ok_or_else(|| {
        crate::traits::EvaluationError::IntegerOverflow(
            "adding lower bound to target balance".into(),
        )
    })?;
    Ok(true)
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/undirected_flow_lower_bounds.rs"]
mod tests;
