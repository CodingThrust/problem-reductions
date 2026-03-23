//! Generalized Hex problem implementation.
//!
//! Generalized Hex asks whether the first player has a forced win in the
//! vertex-claiming Shannon switching game on an undirected graph.

use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::registry::{FieldInfo, ProblemSchemaEntry, VariantDimension};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{Problem, SatisfactionProblem};
use crate::variant::VariantParam;

inventory::submit! {
    ProblemSchemaEntry {
        name: "GeneralizedHex",
        display_name: "Generalized Hex",
        aliases: &[],
        dimensions: &[
            VariantDimension::new("graph", "SimpleGraph", &["SimpleGraph"]),
        ],
        module_path: module_path!(),
        description: "Determine whether Player 1 has a forced blue path between two terminals",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "source", type_name: "usize", description: "The source terminal s" },
            FieldInfo { name: "target", type_name: "usize", description: "The target terminal t" },
        ],
    }
}

/// Generalized Hex on an undirected graph.
///
/// The problem is represented as a zero-variable decision problem: the graph
/// instance fully determines the question, so `evaluate([])` runs a memoized
/// game-tree search from the initial empty board.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "G: serde::Deserialize<'de>"))]
pub struct GeneralizedHex<G> {
    graph: G,
    source: usize,
    target: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ClaimState {
    Unclaimed,
    Blue,
    Red,
}

impl<G: Graph> GeneralizedHex<G> {
    /// Create a new Generalized Hex instance.
    pub fn new(graph: G, source: usize, target: usize) -> Self {
        let num_vertices = graph.num_vertices();
        assert!(source < num_vertices, "source must be a valid graph vertex");
        assert!(target < num_vertices, "target must be a valid graph vertex");
        assert_ne!(source, target, "source and target must be distinct");
        Self {
            graph,
            source,
            target,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the source terminal.
    pub fn source(&self) -> usize {
        self.source
    }

    /// Get the target terminal.
    pub fn target(&self) -> usize {
        self.target
    }

    /// Get the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get the number of playable non-terminal vertices.
    pub fn num_playable_vertices(&self) -> usize {
        self.graph.num_vertices().saturating_sub(2)
    }

    /// Check whether the first player has a forced win from the initial state.
    pub fn is_valid_solution(&self, config: &[usize]) -> bool {
        if !config.is_empty() {
            return false;
        }
        let playable_vertices = self.playable_vertices();
        let vertex_to_state_index = self.vertex_to_state_index(&playable_vertices);
        let mut state = vec![ClaimState::Unclaimed; playable_vertices.len()];
        let mut memo = HashMap::new();
        self.first_player_wins(&mut state, &vertex_to_state_index, &mut memo)
    }

    fn playable_vertices(&self) -> Vec<usize> {
        (0..self.graph.num_vertices())
            .filter(|&vertex| vertex != self.source && vertex != self.target)
            .collect()
    }

    fn vertex_to_state_index(&self, playable_vertices: &[usize]) -> Vec<Option<usize>> {
        let mut index = vec![None; self.graph.num_vertices()];
        for (state_idx, &vertex) in playable_vertices.iter().enumerate() {
            index[vertex] = Some(state_idx);
        }
        index
    }

    fn first_player_wins(
        &self,
        state: &mut [ClaimState],
        vertex_to_state_index: &[Option<usize>],
        memo: &mut HashMap<Vec<ClaimState>, bool>,
    ) -> bool {
        if self.has_path(state, vertex_to_state_index, |claim| {
            matches!(claim, ClaimState::Blue)
        }) {
            return true;
        }
        if !self.has_path(state, vertex_to_state_index, |claim| {
            claim != ClaimState::Red
        }) {
            return false;
        }
        if let Some(&cached) = memo.get(state) {
            return cached;
        }

        let blue_turn = state
            .iter()
            .filter(|&&claim| !matches!(claim, ClaimState::Unclaimed))
            .count()
            % 2
            == 0;

        let result = if blue_turn {
            let mut winning_move_found = false;
            for idx in 0..state.len() {
                if !matches!(state[idx], ClaimState::Unclaimed) {
                    continue;
                }
                state[idx] = ClaimState::Blue;
                if self.first_player_wins(state, vertex_to_state_index, memo) {
                    winning_move_found = true;
                    state[idx] = ClaimState::Unclaimed;
                    break;
                }
                state[idx] = ClaimState::Unclaimed;
            }
            winning_move_found
        } else {
            let mut all_red_moves_still_win = true;
            for idx in 0..state.len() {
                if !matches!(state[idx], ClaimState::Unclaimed) {
                    continue;
                }
                state[idx] = ClaimState::Red;
                if !self.first_player_wins(state, vertex_to_state_index, memo) {
                    all_red_moves_still_win = false;
                    state[idx] = ClaimState::Unclaimed;
                    break;
                }
                state[idx] = ClaimState::Unclaimed;
            }
            all_red_moves_still_win
        };

        memo.insert(state.to_vec(), result);
        result
    }

    fn has_path<F>(
        &self,
        state: &[ClaimState],
        vertex_to_state_index: &[Option<usize>],
        allow_claim: F,
    ) -> bool
    where
        F: Fn(ClaimState) -> bool,
    {
        let mut visited = vec![false; self.graph.num_vertices()];
        let mut queue = VecDeque::from([self.source]);
        visited[self.source] = true;

        while let Some(vertex) = queue.pop_front() {
            if vertex == self.target {
                return true;
            }

            for neighbor in self.graph.neighbors(vertex) {
                if visited[neighbor]
                    || !self.vertex_is_allowed(neighbor, state, vertex_to_state_index, &allow_claim)
                {
                    continue;
                }
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }

        false
    }

    fn vertex_is_allowed<F>(
        &self,
        vertex: usize,
        state: &[ClaimState],
        vertex_to_state_index: &[Option<usize>],
        allow_claim: &F,
    ) -> bool
    where
        F: Fn(ClaimState) -> bool,
    {
        if vertex == self.source || vertex == self.target {
            return true;
        }
        vertex_to_state_index[vertex]
            .and_then(|state_idx| state.get(state_idx).copied())
            .is_some_and(allow_claim)
    }
}

impl<G> Problem for GeneralizedHex<G>
where
    G: Graph + VariantParam,
{
    const NAME: &'static str = "GeneralizedHex";
    type Value = bool;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G]
    }

    fn dims(&self) -> Vec<usize> {
        vec![]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if !config.is_empty() {
            return false;
        }
        let playable_vertices = self.playable_vertices();
        let vertex_to_state_index = self.vertex_to_state_index(&playable_vertices);
        let mut state = vec![ClaimState::Unclaimed; playable_vertices.len()];
        let mut memo = HashMap::new();
        self.first_player_wins(&mut state, &vertex_to_state_index, &mut memo)
    }
}

impl<G> SatisfactionProblem for GeneralizedHex<G> where G: Graph + VariantParam {}

crate::declare_variants! {
    default sat GeneralizedHex<SimpleGraph> => "3^num_playable_vertices",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "generalized_hex_simplegraph",
        instance: Box::new(GeneralizedHex::new(
            SimpleGraph::new(
                6,
                vec![(0, 1), (0, 2), (0, 3), (1, 4), (2, 4), (3, 4), (4, 5)],
            ),
            0,
            5,
        )),
        optimal_config: vec![],
        optimal_value: serde_json::json!(true),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/generalized_hex.rs"]
mod tests;
