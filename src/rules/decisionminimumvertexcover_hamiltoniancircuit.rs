//! Reduction from Decision Minimum Vertex Cover to Hamiltonian Circuit.
//!
//! This implements the gadget construction from Garey & Johnson, Theorem 3.4,
//! on the unit-weight `Decision<MinimumVertexCover<SimpleGraph, i32>>` model.

use crate::models::decision::Decision;
use crate::models::graph::{HamiltonianCircuit, MinimumVertexCover};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::Problem;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
enum ConstructionKind {
    FixedYes { source_cover: Vec<usize> },
    FixedNo { num_source_vertices: usize },
    Theorem(TheoremConstruction),
}

#[derive(Debug, Clone)]
struct TheoremConstruction {
    num_source_vertices: usize,
    selector_count: usize,
    edges: Vec<(usize, usize)>,
    incident_edges: Vec<Vec<usize>>,
}

impl TheoremConstruction {
    fn active_vertices(&self) -> impl Iterator<Item = usize> + '_ {
        self.incident_edges
            .iter()
            .enumerate()
            .filter(|(_, edges)| !edges.is_empty())
            .map(|(v, _)| v)
    }

    fn gadget_base(&self, edge_idx: usize) -> usize {
        self.selector_count + 12 * edge_idx
    }

    fn gadget_vertex(&self, edge_idx: usize, vertex: usize, position: usize) -> usize {
        let (u, v) = self.edges[edge_idx];
        let side = if vertex == u {
            0
        } else if vertex == v {
            1
        } else {
            panic!(
                "vertex {vertex} is not incident on edge {:?}",
                self.edges[edge_idx]
            );
        };

        self.gadget_base(edge_idx) + side * 6 + (position - 1)
    }

    fn path_endpoints(&self, vertex: usize) -> Option<(usize, usize)> {
        let incident = self.incident_edges.get(vertex)?;
        let first = *incident.first()?;
        let last = *incident.last()?;
        Some((
            self.gadget_vertex(first, vertex, 1),
            self.gadget_vertex(last, vertex, 6),
        ))
    }

    fn covers_all_edges(&self, selected: &[usize]) -> bool {
        self.edges
            .iter()
            .all(|&(u, v)| selected.get(u) == Some(&1) || selected.get(v) == Some(&1))
    }

    #[cfg(any(test, feature = "example-db"))]
    fn exact_selected_vertices(&self, source_cover: &[usize]) -> Option<Vec<usize>> {
        if source_cover.len() != self.num_source_vertices || !self.covers_all_edges(source_cover) {
            return None;
        }

        let mut selected: Vec<usize> = self
            .active_vertices()
            .filter(|&v| source_cover[v] == 1)
            .collect();

        if selected.len() > self.selector_count {
            return None;
        }

        for v in self.active_vertices() {
            if selected.len() == self.selector_count {
                break;
            }
            if source_cover[v] == 0 {
                selected.push(v);
            }
        }

        (selected.len() == self.selector_count).then_some(selected)
    }

    #[cfg(any(test, feature = "example-db"))]
    fn gadget_segment(
        &self,
        edge_idx: usize,
        vertex: usize,
        selected_exact: &BTreeSet<usize>,
    ) -> Vec<usize> {
        let (u, v) = self.edges[edge_idx];
        let other = if vertex == u {
            v
        } else if vertex == v {
            u
        } else {
            panic!(
                "vertex {vertex} is not incident on edge {:?}",
                self.edges[edge_idx]
            );
        };

        if selected_exact.contains(&other) {
            return (1..=6)
                .map(|position| self.gadget_vertex(edge_idx, vertex, position))
                .collect();
        }

        if vertex == u {
            vec![
                self.gadget_vertex(edge_idx, u, 1),
                self.gadget_vertex(edge_idx, u, 2),
                self.gadget_vertex(edge_idx, u, 3),
                self.gadget_vertex(edge_idx, v, 1),
                self.gadget_vertex(edge_idx, v, 2),
                self.gadget_vertex(edge_idx, v, 3),
                self.gadget_vertex(edge_idx, v, 4),
                self.gadget_vertex(edge_idx, v, 5),
                self.gadget_vertex(edge_idx, v, 6),
                self.gadget_vertex(edge_idx, u, 4),
                self.gadget_vertex(edge_idx, u, 5),
                self.gadget_vertex(edge_idx, u, 6),
            ]
        } else {
            vec![
                self.gadget_vertex(edge_idx, v, 1),
                self.gadget_vertex(edge_idx, v, 2),
                self.gadget_vertex(edge_idx, v, 3),
                self.gadget_vertex(edge_idx, u, 1),
                self.gadget_vertex(edge_idx, u, 2),
                self.gadget_vertex(edge_idx, u, 3),
                self.gadget_vertex(edge_idx, u, 4),
                self.gadget_vertex(edge_idx, u, 5),
                self.gadget_vertex(edge_idx, u, 6),
                self.gadget_vertex(edge_idx, v, 4),
                self.gadget_vertex(edge_idx, v, 5),
                self.gadget_vertex(edge_idx, v, 6),
            ]
        }
    }

    #[cfg(any(test, feature = "example-db"))]
    fn vertex_path(&self, vertex: usize, selected_exact: &BTreeSet<usize>) -> Vec<usize> {
        let mut path = Vec::new();
        for &edge_idx in &self.incident_edges[vertex] {
            path.extend(self.gadget_segment(edge_idx, vertex, selected_exact));
        }
        path
    }

    #[cfg(any(test, feature = "example-db"))]
    fn build_target_witness(&self, source_cover: &[usize]) -> Vec<usize> {
        let Some(selected_vertices) = self.exact_selected_vertices(source_cover) else {
            return Vec::new();
        };

        let selected_exact: BTreeSet<usize> = selected_vertices.iter().copied().collect();
        let mut witness = Vec::with_capacity(self.selector_count + 12 * self.edges.len());

        for (selector, &vertex) in selected_vertices.iter().enumerate() {
            witness.push(selector);
            witness.extend(self.vertex_path(vertex, &selected_exact));
        }

        witness
    }

    fn extract_solution(
        &self,
        target_problem: &HamiltonianCircuit<SimpleGraph>,
        target_solution: &[usize],
    ) -> Vec<usize> {
        let mut source_cover = vec![0; self.num_source_vertices];
        if !target_problem.evaluate(target_solution).0 {
            return source_cover;
        }

        let mut positions = vec![usize::MAX; target_solution.len()];
        for (idx, &vertex) in target_solution.iter().enumerate() {
            if vertex >= positions.len() || positions[vertex] != usize::MAX {
                return vec![0; self.num_source_vertices];
            }
            positions[vertex] = idx;
        }

        let len = target_solution.len();
        let touches_selector = |vertex: usize| {
            let idx = positions[vertex];
            let prev = target_solution[(idx + len - 1) % len];
            let next = target_solution[(idx + 1) % len];
            prev < self.selector_count || next < self.selector_count
        };

        for vertex in self.active_vertices() {
            let Some((start, end)) = self.path_endpoints(vertex) else {
                continue;
            };
            if touches_selector(start) && touches_selector(end) {
                source_cover[vertex] = 1;
            }
        }

        let selected_count = source_cover.iter().filter(|&&x| x == 1).count();
        if selected_count != self.selector_count || !self.covers_all_edges(&source_cover) {
            return vec![0; self.num_source_vertices];
        }

        source_cover
    }
}

/// Result of reducing Decision<MinimumVertexCover<SimpleGraph, i32>> to
/// HamiltonianCircuit<SimpleGraph>.
#[derive(Debug, Clone)]
pub struct ReductionDecisionMinimumVertexCoverToHamiltonianCircuit {
    target: HamiltonianCircuit<SimpleGraph>,
    construction: ConstructionKind,
}

impl ReductionDecisionMinimumVertexCoverToHamiltonianCircuit {
    #[cfg(any(test, feature = "example-db"))]
    fn build_target_witness(&self, source_cover: &[usize]) -> Vec<usize> {
        match &self.construction {
            ConstructionKind::FixedYes { .. } => vec![0, 1, 2],
            ConstructionKind::FixedNo { .. } => Vec::new(),
            ConstructionKind::Theorem(construction) => {
                construction.build_target_witness(source_cover)
            }
        }
    }
}

impl ReductionResult for ReductionDecisionMinimumVertexCoverToHamiltonianCircuit {
    type Source = Decision<MinimumVertexCover<SimpleGraph, i32>>;
    type Target = HamiltonianCircuit<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        match &self.construction {
            ConstructionKind::FixedYes { source_cover } => {
                if self.target.evaluate(target_solution).0 {
                    source_cover.clone()
                } else {
                    vec![0; source_cover.len()]
                }
            }
            ConstructionKind::FixedNo {
                num_source_vertices,
            } => vec![0; *num_source_vertices],
            ConstructionKind::Theorem(construction) => {
                construction.extract_solution(&self.target, target_solution)
            }
        }
    }
}

fn normalize_edges(edges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut normalized: Vec<_> = edges
        .into_iter()
        .map(|(u, v)| if u < v { (u, v) } else { (v, u) })
        .collect();
    normalized.sort_unstable();
    normalized
}

fn insert_edge(edges: &mut BTreeSet<(usize, usize)>, a: usize, b: usize) {
    let edge = if a < b { (a, b) } else { (b, a) };
    edges.insert(edge);
}

#[reduction(
    overhead = {
        num_vertices = "12 * num_edges + k",
        num_edges = "16 * num_edges - num_vertices + 2 * k * num_vertices",
    }
)]
impl ReduceTo<HamiltonianCircuit<SimpleGraph>> for Decision<MinimumVertexCover<SimpleGraph, i32>> {
    type Result = ReductionDecisionMinimumVertexCoverToHamiltonianCircuit;

    fn reduce_to(&self) -> Self::Result {
        let weights = self.inner().weights();
        assert!(
            weights.iter().all(|&weight| weight == 1),
            "Garey-Johnson Theorem 3.4 requires unit vertex weights"
        );

        let num_source_vertices = self.inner().graph().num_vertices();
        let raw_bound = *self.bound();
        if raw_bound < 0 {
            return ReductionDecisionMinimumVertexCoverToHamiltonianCircuit {
                target: HamiltonianCircuit::new(SimpleGraph::path(3)),
                construction: ConstructionKind::FixedNo {
                    num_source_vertices,
                },
            };
        }

        let k = self.k();
        let edges = normalize_edges(self.inner().graph().edges());
        let mut incident_edges = vec![Vec::new(); num_source_vertices];
        for (edge_idx, &(u, v)) in edges.iter().enumerate() {
            incident_edges[u].push(edge_idx);
            incident_edges[v].push(edge_idx);
        }

        let active_vertices: Vec<_> = incident_edges
            .iter()
            .enumerate()
            .filter(|(_, incident)| !incident.is_empty())
            .map(|(vertex, _)| vertex)
            .collect();
        let active_count = active_vertices.len();

        if active_count == 0 || k >= active_count {
            let mut source_cover = vec![0; num_source_vertices];
            for vertex in active_vertices {
                source_cover[vertex] = 1;
            }
            return ReductionDecisionMinimumVertexCoverToHamiltonianCircuit {
                target: HamiltonianCircuit::new(SimpleGraph::cycle(3)),
                construction: ConstructionKind::FixedYes { source_cover },
            };
        }

        if k == 0 {
            return ReductionDecisionMinimumVertexCoverToHamiltonianCircuit {
                target: HamiltonianCircuit::new(SimpleGraph::path(3)),
                construction: ConstructionKind::FixedNo {
                    num_source_vertices,
                },
            };
        }

        let construction = TheoremConstruction {
            num_source_vertices,
            selector_count: k,
            edges,
            incident_edges,
        };

        let mut target_edges = BTreeSet::new();
        for (edge_idx, &(u, v)) in construction.edges.iter().enumerate() {
            for position in 1..6 {
                insert_edge(
                    &mut target_edges,
                    construction.gadget_vertex(edge_idx, u, position),
                    construction.gadget_vertex(edge_idx, u, position + 1),
                );
                insert_edge(
                    &mut target_edges,
                    construction.gadget_vertex(edge_idx, v, position),
                    construction.gadget_vertex(edge_idx, v, position + 1),
                );
            }

            insert_edge(
                &mut target_edges,
                construction.gadget_vertex(edge_idx, u, 3),
                construction.gadget_vertex(edge_idx, v, 1),
            );
            insert_edge(
                &mut target_edges,
                construction.gadget_vertex(edge_idx, v, 3),
                construction.gadget_vertex(edge_idx, u, 1),
            );
            insert_edge(
                &mut target_edges,
                construction.gadget_vertex(edge_idx, u, 6),
                construction.gadget_vertex(edge_idx, v, 4),
            );
            insert_edge(
                &mut target_edges,
                construction.gadget_vertex(edge_idx, v, 6),
                construction.gadget_vertex(edge_idx, u, 4),
            );
        }

        for vertex in construction.active_vertices() {
            let incident = &construction.incident_edges[vertex];
            for window in incident.windows(2) {
                insert_edge(
                    &mut target_edges,
                    construction.gadget_vertex(window[0], vertex, 6),
                    construction.gadget_vertex(window[1], vertex, 1),
                );
            }

            let (start, end) = construction
                .path_endpoints(vertex)
                .expect("active vertices have path endpoints");
            for selector in 0..construction.selector_count {
                insert_edge(&mut target_edges, selector, start);
                insert_edge(&mut target_edges, selector, end);
            }
        }

        let target = HamiltonianCircuit::new(SimpleGraph::new(
            construction.selector_count + 12 * construction.edges.len(),
            target_edges.into_iter().collect(),
        ));

        ReductionDecisionMinimumVertexCoverToHamiltonianCircuit {
            target,
            construction: ConstructionKind::Theorem(construction),
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::assemble_rule_example;
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "decisionminimumvertexcover_to_hamiltoniancircuit",
        build: || {
            let source = Decision::new(
                MinimumVertexCover::new(SimpleGraph::new(3, vec![(0, 1), (1, 2)]), vec![1, 1, 1]),
                1,
            );
            let source_config = vec![0, 1, 0];
            let reduction = ReduceTo::<HamiltonianCircuit<SimpleGraph>>::reduce_to(&source);
            let target_config = reduction.build_target_witness(&source_config);
            assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/decisionminimumvertexcover_hamiltoniancircuit.rs"]
mod tests;
