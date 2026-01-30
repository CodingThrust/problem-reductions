//! Path decomposition algorithms for graph embedding.
//!
//! This module provides algorithms to compute path decompositions of graphs,
//! which are used to determine optimal vertex orderings for the copy-line embedding.
//! The pathwidth of a graph determines the grid height needed for the embedding.
//!
//! Two methods are provided:
//! - `Greedy`: Fast heuristic with random restarts
//! - `MinhThiTrick`: Branch-and-bound algorithm for optimal pathwidth
//!
//! Reference for branch-and-bound:
//! Coudert, D., Mazauric, D., & Nisse, N. (2014).
//! Experimental evaluation of a branch and bound algorithm for computing pathwidth.
//! <https://doi.org/10.1007/978-3-319-07959-2_5>

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

/// A layout representing a partial path decomposition.
///
/// The layout tracks:
/// - `vertices`: The ordered list of vertices added so far
/// - `vsep`: The maximum vertex separation (pathwidth) seen so far
/// - `neighbors`: Vertices not yet added but adjacent to some added vertex
/// - `disconnected`: Vertices not yet added and not adjacent to any added vertex
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layout {
    /// Ordered list of vertices in the decomposition.
    pub vertices: Vec<usize>,
    /// Maximum vertex separation (pathwidth).
    pub vsep: usize,
    /// Vertices adjacent to the current frontier but not yet added.
    pub neighbors: Vec<usize>,
    /// Vertices not adjacent to any added vertex.
    pub disconnected: Vec<usize>,
}

impl Layout {
    /// Create a new layout for a graph starting with given vertices.
    ///
    /// # Arguments
    /// * `num_vertices` - Total number of vertices in the graph
    /// * `edges` - List of edges as (u, v) pairs
    /// * `vertices` - Initial ordered list of vertices
    pub fn new(num_vertices: usize, edges: &[(usize, usize)], vertices: Vec<usize>) -> Self {
        let (vsep, neighbors) = vsep_and_neighbors(num_vertices, edges, &vertices);
        let vertices_set: HashSet<usize> = vertices.iter().copied().collect();
        let neighbors_set: HashSet<usize> = neighbors.iter().copied().collect();
        let disconnected: Vec<usize> = (0..num_vertices)
            .filter(|v| !vertices_set.contains(v) && !neighbors_set.contains(v))
            .collect();
        Layout {
            vertices,
            vsep,
            neighbors,
            disconnected,
        }
    }

    /// Create an empty layout for a graph.
    pub fn empty(num_vertices: usize) -> Self {
        Layout {
            vertices: Vec::new(),
            vsep: 0,
            neighbors: Vec::new(),
            disconnected: (0..num_vertices).collect(),
        }
    }

    /// Get the vertex separation (pathwidth) of this layout.
    pub fn vsep(&self) -> usize {
        self.vsep
    }

    /// Get the current frontier size (number of neighbors).
    pub fn vsep_last(&self) -> usize {
        self.neighbors.len()
    }
}

/// Compute the vertex separation and final neighbors for a given vertex ordering.
///
/// The vertex separation is the maximum number of vertices that are:
/// - Not yet added to the ordering
/// - But adjacent to some vertex already in the ordering
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - List of edges
/// * `vertices` - Ordered list of vertices
///
/// # Returns
/// (vsep, neighbors) where vsep is the maximum vertex separation and
/// neighbors is the final neighbor set after all vertices are added.
fn vsep_and_neighbors(
    num_vertices: usize,
    edges: &[(usize, usize)],
    vertices: &[usize],
) -> (usize, Vec<usize>) {
    // Build adjacency list
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); num_vertices];
    for &(u, v) in edges {
        adj[u].insert(v);
        adj[v].insert(u);
    }

    let mut vsep = 0;
    let mut neighbors: HashSet<usize> = HashSet::new();

    for i in 0..vertices.len() {
        let s: HashSet<usize> = vertices[0..=i].iter().copied().collect();

        // neighbors = vertices not in S but adjacent to some vertex in S
        neighbors = (0..num_vertices)
            .filter(|&v| !s.contains(&v) && adj[v].iter().any(|&u| s.contains(&u)))
            .collect();

        let vsi = neighbors.len();
        if vsi > vsep {
            vsep = vsi;
        }
    }

    (vsep, neighbors.into_iter().collect())
}

/// Compute the updated vsep if vertex v is added to the layout.
///
/// This is an efficient incremental computation that doesn't create a new layout.
fn vsep_updated(
    num_vertices: usize,
    edges: &[(usize, usize)],
    layout: &Layout,
    v: usize,
) -> usize {
    // Build adjacency list
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); num_vertices];
    for &(u, w) in edges {
        adj[u].insert(w);
        adj[w].insert(u);
    }

    let mut vs = layout.vsep_last();

    // If v is in neighbors, removing it decreases frontier by 1
    if layout.neighbors.contains(&v) {
        vs -= 1;
    }

    // For each neighbor of v, if not in vertices and not in neighbors, it becomes a neighbor
    let vertices_set: HashSet<usize> = layout.vertices.iter().copied().collect();
    let neighbors_set: HashSet<usize> = layout.neighbors.iter().copied().collect();

    for &w in &adj[v] {
        if !vertices_set.contains(&w) && !neighbors_set.contains(&w) {
            vs += 1;
        }
    }

    vs.max(layout.vsep)
}

/// Compute the updated vsep, neighbors, and disconnected if vertex v is added.
///
/// Returns (new_vsep, new_neighbors, new_disconnected).
fn vsep_updated_neighbors(
    num_vertices: usize,
    edges: &[(usize, usize)],
    layout: &Layout,
    v: usize,
) -> (usize, Vec<usize>, Vec<usize>) {
    // Build adjacency list
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); num_vertices];
    for &(u, w) in edges {
        adj[u].insert(w);
        adj[w].insert(u);
    }

    let mut vs = layout.vsep_last();
    let mut nbs: Vec<usize> = layout.neighbors.clone();
    let mut disc: Vec<usize> = layout.disconnected.clone();

    if let Some(pos) = nbs.iter().position(|&x| x == v) {
        nbs.remove(pos);
        vs -= 1;
    } else if let Some(pos) = disc.iter().position(|&x| x == v) {
        disc.remove(pos);
    }

    let vertices_set: HashSet<usize> = layout.vertices.iter().copied().collect();
    let nbs_set: HashSet<usize> = nbs.iter().copied().collect();

    for &w in &adj[v] {
        if !vertices_set.contains(&w) && !nbs_set.contains(&w) {
            vs += 1;
            nbs.push(w);
            if let Some(pos) = disc.iter().position(|&x| x == w) {
                disc.remove(pos);
            }
        }
    }

    let vs = vs.max(layout.vsep);
    (vs, nbs, disc)
}

/// Extend a layout by adding a vertex.
///
/// This is the ⊙ operator from the Julia implementation.
fn extend(num_vertices: usize, edges: &[(usize, usize)], layout: &Layout, v: usize) -> Layout {
    let mut vertices = layout.vertices.clone();
    vertices.push(v);

    let (vs_new, neighbors_new, disconnected) =
        vsep_updated_neighbors(num_vertices, edges, layout, v);

    Layout {
        vertices,
        vsep: vs_new,
        neighbors: neighbors_new,
        disconnected,
    }
}

/// Apply greedy exact rules that don't increase pathwidth.
///
/// This adds vertices that can be added without increasing the vertex separation:
/// 1. Vertices whose all neighbors are already in vertices or neighbors (safe to add)
/// 2. Neighbor vertices that would add exactly one new neighbor (maintains separation)
fn greedy_exact(num_vertices: usize, edges: &[(usize, usize)], mut layout: Layout) -> Layout {
    // Build adjacency list
    let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); num_vertices];
    for &(u, v) in edges {
        adj[u].insert(v);
        adj[v].insert(u);
    }

    let mut keep_going = true;
    while keep_going {
        keep_going = false;

        // Rule 1: Add vertices whose all neighbors are in vertices ∪ neighbors
        for list in [&layout.disconnected.clone(), &layout.neighbors.clone()] {
            for &v in list {
                let vertices_set: HashSet<usize> = layout.vertices.iter().copied().collect();
                let neighbors_set: HashSet<usize> = layout.neighbors.iter().copied().collect();

                let all_neighbors_covered = adj[v]
                    .iter()
                    .all(|&nb| vertices_set.contains(&nb) || neighbors_set.contains(&nb));

                if all_neighbors_covered {
                    layout = extend(num_vertices, edges, &layout, v);
                    keep_going = true;
                }
            }
        }

        // Rule 2: Add neighbor vertices that would add exactly one new neighbor
        for &v in &layout.neighbors.clone() {
            let vertices_set: HashSet<usize> = layout.vertices.iter().copied().collect();
            let neighbors_set: HashSet<usize> = layout.neighbors.iter().copied().collect();

            let new_neighbors_count = adj[v]
                .iter()
                .filter(|&&nb| !vertices_set.contains(&nb) && !neighbors_set.contains(&nb))
                .count();

            if new_neighbors_count == 1 {
                layout = extend(num_vertices, edges, &layout, v);
                keep_going = true;
            }
        }
    }

    layout
}

/// Perform one greedy step by choosing the best vertex from a list.
///
/// Selects randomly among vertices that minimize the new vsep.
fn greedy_step(
    num_vertices: usize,
    edges: &[(usize, usize)],
    layout: &Layout,
    list: &[usize],
) -> Layout {
    let layouts: Vec<Layout> = list
        .iter()
        .map(|&v| extend(num_vertices, edges, layout, v))
        .collect();

    let costs: Vec<usize> = layouts.iter().map(|l| l.vsep()).collect();
    let best_cost = *costs.iter().min().unwrap();

    let best_indices: Vec<usize> = costs
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == best_cost)
        .map(|(i, _)| i)
        .collect();

    let mut rng = rand::thread_rng();
    let &chosen_idx = best_indices.as_slice().choose(&mut rng).unwrap();

    layouts.into_iter().nth(chosen_idx).unwrap()
}

/// Compute a path decomposition using the greedy algorithm.
///
/// This combines exact rules (that don't increase pathwidth) with
/// greedy choices when exact rules don't apply.
pub fn greedy_decompose(num_vertices: usize, edges: &[(usize, usize)]) -> Layout {
    let mut layout = Layout::empty(num_vertices);

    loop {
        layout = greedy_exact(num_vertices, edges, layout);

        if !layout.neighbors.is_empty() {
            layout = greedy_step(num_vertices, edges, &layout, &layout.neighbors.clone());
        } else if !layout.disconnected.is_empty() {
            layout = greedy_step(num_vertices, edges, &layout, &layout.disconnected.clone());
        } else {
            break;
        }
    }

    layout
}

/// Compute a path decomposition using branch and bound.
///
/// This finds the optimal (minimum) pathwidth decomposition.
pub fn branch_and_bound(num_vertices: usize, edges: &[(usize, usize)]) -> Layout {
    let initial = Layout::empty(num_vertices);
    let full_layout = Layout::new(num_vertices, edges, (0..num_vertices).collect());
    let mut visited: HashMap<Vec<usize>, bool> = HashMap::new();

    branch_and_bound_internal(num_vertices, edges, initial, full_layout, &mut visited)
}

/// Internal branch and bound implementation.
fn branch_and_bound_internal(
    num_vertices: usize,
    edges: &[(usize, usize)],
    p: Layout,
    mut best: Layout,
    visited: &mut HashMap<Vec<usize>, bool>,
) -> Layout {
    if p.vsep() < best.vsep() && !visited.contains_key(&p.vertices) {
        let p2 = greedy_exact(num_vertices, edges, p.clone());
        let vsep_p2 = p2.vsep();

        // Check if P2 is complete
        let mut sorted_vertices = p2.vertices.clone();
        sorted_vertices.sort();
        let all_vertices: Vec<usize> = (0..num_vertices).collect();

        if sorted_vertices == all_vertices && vsep_p2 < best.vsep() {
            return p2;
        } else {
            let current = best.vsep();
            let mut remaining: Vec<usize> = p2.neighbors.clone();
            remaining.extend(p2.disconnected.iter());

            // Sort by increasing vsep_updated
            let mut vsep_order: Vec<(usize, usize)> = remaining
                .iter()
                .map(|&v| (vsep_updated(num_vertices, edges, &p2, v), v))
                .collect();
            vsep_order.sort_by_key(|&(cost, _)| cost);

            for (_, v) in vsep_order {
                if vsep_updated(num_vertices, edges, &p2, v) < best.vsep() {
                    let extended = extend(num_vertices, edges, &p2, v);
                    let l3 = branch_and_bound_internal(num_vertices, edges, extended, best.clone(), visited);
                    if l3.vsep() < best.vsep() {
                        best = l3;
                    }
                }
            }

            // Update visited table
            visited.insert(p.vertices.clone(), !(best.vsep() < current && p.vsep() == best.vsep()));
        }
    }

    best
}

/// Method for computing path decomposition.
#[derive(Debug, Clone, Copy, Default)]
pub enum PathDecompositionMethod {
    /// Greedy method with random restarts.
    Greedy {
        /// Number of random restarts.
        nrepeat: usize,
    },
    /// Branch and bound method for optimal pathwidth.
    /// Named in memory of Minh-Thi Nguyen, one of the main developers.
    #[default]
    MinhThiTrick,
}

impl PathDecompositionMethod {
    /// Create a greedy method with default 10 restarts.
    pub fn greedy() -> Self {
        PathDecompositionMethod::Greedy { nrepeat: 10 }
    }

    /// Create a greedy method with specified number of restarts.
    pub fn greedy_with_restarts(nrepeat: usize) -> Self {
        PathDecompositionMethod::Greedy { nrepeat }
    }
}

/// Compute a path decomposition of a graph.
///
/// Returns a Layout containing the vertex ordering and pathwidth.
///
/// # Arguments
/// * `num_vertices` - Number of vertices in the graph
/// * `edges` - List of edges as (u, v) pairs
/// * `method` - The decomposition method to use
///
/// # Example
/// ```
/// use problemreductions::rules::unitdiskmapping::pathdecomposition::{pathwidth, PathDecompositionMethod};
///
/// // Path graph: 0-1-2
/// let edges = vec![(0, 1), (1, 2)];
/// let layout = pathwidth(3, &edges, PathDecompositionMethod::greedy());
/// assert_eq!(layout.vertices.len(), 3);
/// assert_eq!(layout.vsep(), 1); // Path graph has pathwidth 1
/// ```
pub fn pathwidth(
    num_vertices: usize,
    edges: &[(usize, usize)],
    method: PathDecompositionMethod,
) -> Layout {
    match method {
        PathDecompositionMethod::Greedy { nrepeat } => {
            let mut best: Option<Layout> = None;
            for _ in 0..nrepeat {
                let layout = greedy_decompose(num_vertices, edges);
                if best.is_none() || layout.vsep() < best.as_ref().unwrap().vsep() {
                    best = Some(layout);
                }
            }
            best.unwrap_or_else(|| Layout::empty(num_vertices))
        }
        PathDecompositionMethod::MinhThiTrick => branch_and_bound(num_vertices, edges),
    }
}

/// Get the vertex ordering from a layout (reversed for copy-line embedding).
///
/// The copy-line embedding expects vertices in reverse order of the path decomposition.
pub fn vertex_order_from_layout(layout: &Layout) -> Vec<usize> {
    layout.vertices.iter().rev().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_empty() {
        let layout = Layout::empty(5);
        assert_eq!(layout.vertices.len(), 0);
        assert_eq!(layout.vsep(), 0);
        assert_eq!(layout.disconnected.len(), 5);
        assert_eq!(layout.neighbors.len(), 0);
    }

    #[test]
    fn test_layout_new() {
        // Path graph: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let layout = Layout::new(3, &edges, vec![0, 1, 2]);
        assert_eq!(layout.vertices, vec![0, 1, 2]);
        assert_eq!(layout.vsep(), 1); // Path has pathwidth 1
    }

    #[test]
    fn test_vsep_and_neighbors_path() {
        // Path: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let (vsep, _) = vsep_and_neighbors(3, &edges, &[0, 1, 2]);
        assert_eq!(vsep, 1);
    }

    #[test]
    fn test_vsep_and_neighbors_star() {
        // Star: 0 connected to 1, 2, 3
        let edges = vec![(0, 1), (0, 2), (0, 3)];
        // Order: 0, 1, 2, 3 - after adding 0, all others become neighbors
        let (vsep, _) = vsep_and_neighbors(4, &edges, &[0, 1, 2, 3]);
        assert_eq!(vsep, 3); // After adding 0, neighbors = {1, 2, 3}
    }

    #[test]
    fn test_extend() {
        // Path: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let layout = Layout::empty(3);
        let layout = extend(3, &edges, &layout, 0);
        assert_eq!(layout.vertices, vec![0]);
        assert!(layout.neighbors.contains(&1));
        assert!(layout.disconnected.contains(&2));
    }

    #[test]
    fn test_greedy_decompose_path() {
        // Path: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let layout = greedy_decompose(3, &edges);
        assert_eq!(layout.vertices.len(), 3);
        assert_eq!(layout.vsep(), 1);
    }

    #[test]
    fn test_greedy_decompose_triangle() {
        // Triangle: 0-1, 1-2, 0-2
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let layout = greedy_decompose(3, &edges);
        assert_eq!(layout.vertices.len(), 3);
        assert_eq!(layout.vsep(), 2); // Triangle has pathwidth 2
    }

    #[test]
    fn test_greedy_decompose_k4() {
        // Complete graph K4
        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let layout = greedy_decompose(4, &edges);
        assert_eq!(layout.vertices.len(), 4);
        assert_eq!(layout.vsep(), 3); // K4 has pathwidth 3
    }

    #[test]
    fn test_branch_and_bound_path() {
        // Path: 0-1-2
        let edges = vec![(0, 1), (1, 2)];
        let layout = branch_and_bound(3, &edges);
        assert_eq!(layout.vertices.len(), 3);
        assert_eq!(layout.vsep(), 1);
    }

    #[test]
    fn test_branch_and_bound_triangle() {
        // Triangle
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let layout = branch_and_bound(3, &edges);
        assert_eq!(layout.vertices.len(), 3);
        assert_eq!(layout.vsep(), 2);
    }

    #[test]
    fn test_pathwidth_greedy() {
        let edges = vec![(0, 1), (1, 2)];
        let layout = pathwidth(3, &edges, PathDecompositionMethod::greedy());
        assert_eq!(layout.vertices.len(), 3);
        assert_eq!(layout.vsep(), 1);
    }

    #[test]
    fn test_pathwidth_minhthi() {
        let edges = vec![(0, 1), (1, 2)];
        let layout = pathwidth(3, &edges, PathDecompositionMethod::MinhThiTrick);
        assert_eq!(layout.vertices.len(), 3);
        assert_eq!(layout.vsep(), 1);
    }

    #[test]
    fn test_vertex_order_from_layout() {
        let layout = Layout {
            vertices: vec![0, 1, 2],
            vsep: 1,
            neighbors: vec![],
            disconnected: vec![],
        };
        let order = vertex_order_from_layout(&layout);
        assert_eq!(order, vec![2, 1, 0]);
    }

    #[test]
    fn test_petersen_graph_pathwidth() {
        // Petersen graph edges
        let edges = vec![
            (0, 1), (1, 2), (2, 3), (3, 4), (4, 0), // outer pentagon
            (5, 7), (7, 9), (9, 6), (6, 8), (8, 5), // inner star
            (0, 5), (1, 6), (2, 7), (3, 8), (4, 9), // connections
        ];

        let layout = pathwidth(10, &edges, PathDecompositionMethod::MinhThiTrick);
        assert_eq!(layout.vertices.len(), 10);
        // Petersen graph has pathwidth 5
        assert_eq!(layout.vsep(), 5);
    }

    #[test]
    fn test_cycle_graph_pathwidth() {
        // Cycle C5: 0-1-2-3-4-0
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
        let layout = pathwidth(5, &edges, PathDecompositionMethod::MinhThiTrick);
        assert_eq!(layout.vertices.len(), 5);
        // Cycle has pathwidth 2
        assert_eq!(layout.vsep(), 2);
    }

    #[test]
    fn test_disconnected_graph() {
        // Two disconnected edges: 0-1, 2-3
        let edges = vec![(0, 1), (2, 3)];
        let layout = pathwidth(4, &edges, PathDecompositionMethod::MinhThiTrick);
        assert_eq!(layout.vertices.len(), 4);
        // Pathwidth is 1 (each component has pathwidth 1)
        assert_eq!(layout.vsep(), 1);
    }

    #[test]
    fn test_empty_graph() {
        // No edges
        let edges: Vec<(usize, usize)> = vec![];
        let layout = pathwidth(5, &edges, PathDecompositionMethod::MinhThiTrick);
        assert_eq!(layout.vertices.len(), 5);
        assert_eq!(layout.vsep(), 0); // No edges means pathwidth 0
    }
}
