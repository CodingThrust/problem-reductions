# HamiltonianCycle Model Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement the HamiltonianCycle optimization problem model — given a weighted undirected graph, find a minimum-weight cycle visiting every vertex exactly once.

**Architecture:** Follow the `MaximumMatching` model pattern (edge-based binary variables with edge weights). The struct `HamiltonianCycle<G, W>` stores a graph and edge weights. `dims()` returns `[2; num_edges]`. `evaluate()` checks if selected edges form a valid Hamiltonian cycle (degree-2 at every vertex, single connected cycle, exactly |V| edges) then returns the total weight. Direction is `Minimize`.

**Tech Stack:** Rust, serde, num_traits, inventory (for schema registration)

---

## Task 1: Write failing tests for HamiltonianCycle model

**Files:**
- Create: `src/unit_tests/models/graph/hamiltonian_cycle.rs`

**Step 1: Write the failing tests**

Create the test file with comprehensive tests covering creation, evaluation, brute-force solving, and edge cases. These tests follow the patterns in `src/unit_tests/models/graph/maximum_matching.rs` and use the four example instances from the issue.

```rust
use super::*;
use crate::solvers::BruteForce;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_hamiltonian_cycle_creation() {
    // K4 complete graph
    let problem = HamiltonianCycle::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10), (0, 2, 15), (0, 3, 20),
            (1, 2, 35), (1, 3, 25), (2, 3, 30),
        ],
    );
    assert_eq!(problem.num_vertices(), 4);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.dims().len(), 6);
}

#[test]
fn test_hamiltonian_cycle_unweighted() {
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    );
    assert!(!problem.is_weighted());
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 5);
}

#[test]
fn test_hamiltonian_cycle_weighted() {
    let problem = HamiltonianCycle::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10), (0, 2, 15), (0, 3, 20),
            (1, 2, 35), (1, 3, 25), (2, 3, 30),
        ],
    );
    assert!(problem.is_weighted());
}

#[test]
fn test_evaluate_valid_cycle() {
    // C5 cycle graph with unit weights: all 5 edges form the only Hamiltonian cycle
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    );
    // Select all edges → valid Hamiltonian cycle, cost = 5
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 1]), SolutionSize::Valid(5));
}

#[test]
fn test_evaluate_invalid_degree() {
    // K4: select 3 edges incident to vertex 0 → degree > 2 at vertex 0
    let problem = HamiltonianCycle::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10), (0, 2, 15), (0, 3, 20),
            (1, 2, 35), (1, 3, 25), (2, 3, 30),
        ],
    );
    // edges: 0-1, 0-2, 0-3, 1-2, 1-3, 2-3
    // Select first 3 edges (all incident to 0): degree(0)=3 → Invalid
    assert_eq!(problem.evaluate(&[1, 1, 1, 0, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_invalid_not_connected() {
    // 6 vertices, two disjoint triangles: 0-1-2-0 and 3-4-5-3
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        6,
        vec![
            (0, 1), (1, 2), (0, 2),
            (3, 4), (4, 5), (3, 5),
        ],
    );
    // Select all 6 edges: two disjoint cycles, not a single Hamiltonian cycle
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 1, 1]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_invalid_wrong_edge_count() {
    // C5 with only 4 edges selected → not enough edges
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    );
    assert_eq!(problem.evaluate(&[1, 1, 1, 1, 0]), SolutionSize::Invalid);
}

#[test]
fn test_evaluate_no_edges_selected() {
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    );
    assert_eq!(problem.evaluate(&[0, 0, 0, 0, 0]), SolutionSize::Invalid);
}

#[test]
fn test_brute_force_k4() {
    // Instance 1 from issue: K4 with weights
    let problem = HamiltonianCycle::<SimpleGraph, i32>::new(
        4,
        vec![
            (0, 1, 10), (0, 2, 15), (0, 3, 20),
            (1, 2, 35), (1, 3, 25), (2, 3, 30),
        ],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(!solutions.is_empty());
    // Optimal cycle: 0→1→3→2→0, cost = 10+25+30+15 = 80
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(80));
    }
}

#[test]
fn test_brute_force_path_graph_no_solution() {
    // Instance 2 from issue: path graph, no Hamiltonian cycle exists
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        4,
        vec![(0, 1), (1, 2), (2, 3)],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_brute_force_c5_unique_solution() {
    // Instance 3 from issue: C5 cycle graph, unique Hamiltonian cycle
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1, 1, 1]);
    assert_eq!(problem.evaluate(&solutions[0]), SolutionSize::Valid(5));
}

#[test]
fn test_brute_force_bipartite_no_solution() {
    // Instance 4 from issue: K_{2,3} bipartite, no Hamiltonian cycle
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        5,
        vec![(0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (1, 4)],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(solutions.is_empty());
}

#[test]
fn test_direction() {
    let problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        3,
        vec![(0, 1), (1, 2), (0, 2)],
    );
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_problem_name() {
    assert_eq!(
        <HamiltonianCycle<SimpleGraph, i32> as Problem>::NAME,
        "HamiltonianCycle"
    );
}

#[test]
fn test_is_hamiltonian_cycle_function() {
    // Triangle: selecting all 3 edges is a valid Hamiltonian cycle
    assert!(is_hamiltonian_cycle(
        3,
        &[(0, 1), (1, 2), (0, 2)],
        &[true, true, true]
    ));
    // Path: not a cycle
    assert!(!is_hamiltonian_cycle(
        3,
        &[(0, 1), (1, 2)],
        &[true, true]
    ));
}

#[test]
fn test_set_weights() {
    let mut problem = HamiltonianCycle::<SimpleGraph, i32>::unweighted(
        3,
        vec![(0, 1), (1, 2), (0, 2)],
    );
    problem.set_weights(vec![5, 10, 15]);
    assert_eq!(problem.weights(), vec![5, 10, 15]);
}

#[test]
fn test_edges() {
    let problem = HamiltonianCycle::<SimpleGraph, i32>::new(
        3,
        vec![(0, 1, 10), (1, 2, 20), (0, 2, 30)],
    );
    let edges = problem.edges();
    assert_eq!(edges.len(), 3);
}

#[test]
fn test_from_graph() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = HamiltonianCycle::<SimpleGraph, i32>::from_graph(graph, vec![10, 20, 30]);
    assert_eq!(problem.num_vertices(), 3);
    assert_eq!(problem.weights(), vec![10, 20, 30]);
}

#[test]
fn test_from_graph_unit_weights() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let problem = HamiltonianCycle::<SimpleGraph, i32>::from_graph_unit_weights(graph);
    assert_eq!(problem.weights(), vec![1, 1, 1]);
}

#[test]
fn test_brute_force_triangle_weighted() {
    // Triangle with weights: unique Hamiltonian cycle using all edges
    let problem = HamiltonianCycle::<SimpleGraph, i32>::new(
        3,
        vec![(0, 1, 5), (1, 2, 10), (0, 2, 15)],
    );
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert_eq!(solutions.len(), 1);
    assert_eq!(solutions[0], vec![1, 1, 1]);
    assert_eq!(problem.evaluate(&solutions[0]), SolutionSize::Valid(30));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test hamiltonian_cycle -- --no-run 2>&1 | head -20`
Expected: Compilation error — `HamiltonianCycle` type doesn't exist yet.

---

## Task 2: Implement HamiltonianCycle model

**Files:**
- Create: `src/models/graph/hamiltonian_cycle.rs`
- Modify: `src/models/graph/mod.rs`

**Step 1: Write the implementation**

Create `src/models/graph/hamiltonian_cycle.rs`:

```rust
//! Hamiltonian Cycle problem implementation.
//!
//! The Hamiltonian Cycle problem asks for a minimum-weight cycle
//! that visits every vertex exactly once.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "HamiltonianCycle",
        description: "Find minimum weight Hamiltonian cycle in a graph",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The underlying graph G=(V,E)" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R" },
        ],
    }
}

/// The Hamiltonian Cycle problem.
///
/// Given a weighted graph G = (V, E) with edge weights w_e,
/// find a cycle that visits every vertex exactly once and
/// minimizes the total edge weight.
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is not in the cycle
/// - 1: edge is in the cycle
///
/// A valid Hamiltonian cycle requires:
/// - Exactly 2 selected edges incident to each vertex (degree constraint)
/// - Selected edges form a single connected cycle (no subtours)
/// - Exactly |V| edges are selected
///
/// # Type Parameters
///
/// * `G` - The graph type (e.g., `SimpleGraph`, `GridGraph`)
/// * `W` - The weight type for edges (e.g., `i32`, `f64`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HamiltonianCycle<G, W> {
    /// The underlying graph.
    graph: G,
    /// Weights for each edge (in edge index order).
    edge_weights: Vec<W>,
}

impl<W: Clone + Default> HamiltonianCycle<SimpleGraph, W> {
    /// Create a new HamiltonianCycle problem.
    ///
    /// # Arguments
    /// * `num_vertices` - Number of vertices
    /// * `edges` - List of weighted edges as (u, v, weight) triples
    pub fn new(num_vertices: usize, edges: Vec<(usize, usize, W)>) -> Self {
        let mut edge_list = Vec::new();
        let mut edge_weights = Vec::new();
        for (u, v, w) in edges {
            edge_list.push((u, v));
            edge_weights.push(w);
        }
        let graph = SimpleGraph::new(num_vertices, edge_list);
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a HamiltonianCycle problem with unit weights.
    pub fn unweighted(num_vertices: usize, edges: Vec<(usize, usize)>) -> Self
    where
        W: From<i32>,
    {
        let edge_weights = vec![W::from(1); edges.len()];
        let graph = SimpleGraph::new(num_vertices, edges);
        Self {
            graph,
            edge_weights,
        }
    }
}

impl<G: Graph, W: Clone + Default> HamiltonianCycle<G, W> {
    /// Create a HamiltonianCycle problem from a graph with given edge weights.
    pub fn from_graph(graph: G, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        Self {
            graph,
            edge_weights,
        }
    }

    /// Create a HamiltonianCycle problem from a graph with unit weights.
    pub fn from_graph_unit_weights(graph: G) -> Self
    where
        W: From<i32>,
    {
        let edge_weights = vec![W::from(1); graph.num_edges()];
        Self {
            graph,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Get the number of edges.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Get all edges with their weights.
    pub fn edges(&self) -> Vec<(usize, usize, W)> {
        self.graph
            .edges()
            .into_iter()
            .zip(self.edge_weights.iter().cloned())
            .map(|((u, v), w)| (u, v, w))
            .collect()
    }

    /// Set new weights for the problem.
    pub fn set_weights(&mut self, weights: Vec<W>) {
        assert_eq!(weights.len(), self.graph.num_edges());
        self.edge_weights = weights;
    }

    /// Get the weights for the problem.
    pub fn weights(&self) -> Vec<W> {
        self.edge_weights.clone()
    }

    /// Check if the problem has non-uniform weights.
    pub fn is_weighted(&self) -> bool
    where
        W: PartialEq,
    {
        if self.edge_weights.is_empty() {
            return false;
        }
        let first = &self.edge_weights[0];
        !self.edge_weights.iter().all(|w| w == first)
    }

    /// Check if a configuration forms a valid Hamiltonian cycle.
    fn is_valid_hamiltonian_cycle(&self, config: &[usize]) -> bool {
        let n = self.graph.num_vertices();
        let edges = self.graph.edges();

        // Count selected edges and check degree constraint
        let mut degree = vec![0usize; n];
        let mut selected_count = 0;
        let mut first_selected_vertex = None;

        for (idx, &sel) in config.iter().enumerate() {
            if sel == 1 {
                if let Some(&(u, v)) = edges.get(idx) {
                    degree[u] += 1;
                    degree[v] += 1;
                    selected_count += 1;
                    if first_selected_vertex.is_none() {
                        first_selected_vertex = Some(u);
                    }
                }
            }
        }

        // Must select exactly n edges
        if selected_count != n {
            return false;
        }

        // Every vertex must have degree exactly 2
        if degree.iter().any(|&d| d != 2) {
            return false;
        }

        // Check connectivity: BFS/DFS on selected edges must reach all vertices
        let first = match first_selected_vertex {
            Some(v) => v,
            None => return false,
        };

        // Build adjacency list from selected edges
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for (idx, &sel) in config.iter().enumerate() {
            if sel == 1 {
                if let Some(&(u, v)) = edges.get(idx) {
                    adj[u].push(v);
                    adj[v].push(u);
                }
            }
        }

        // BFS from first vertex
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        visited[first] = true;
        queue.push_back(first);
        let mut visit_count = 1;

        while let Some(node) = queue.pop_front() {
            for &neighbor in &adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    visit_count += 1;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_count == n
    }
}

impl<G, W> Problem for HamiltonianCycle<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    const NAME: &'static str = "HamiltonianCycle";
    type Metric = SolutionSize<W>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![
            ("graph", crate::variant::short_type_name::<G>()),
            ("weight", crate::variant::short_type_name::<W>()),
        ]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W> {
        if !self.is_valid_hamiltonian_cycle(config) {
            return SolutionSize::Invalid;
        }
        let mut total = W::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                if let Some(w) = self.edge_weights.get(idx) {
                    total += w.clone();
                }
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for HamiltonianCycle<G, W>
where
    G: Graph,
    W: Clone
        + Default
        + PartialOrd
        + num_traits::Num
        + num_traits::Zero
        + std::ops::AddAssign
        + 'static,
{
    type Value = W;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}

/// Check if a selection of edges forms a valid Hamiltonian cycle.
///
/// # Arguments
/// * `num_vertices` - Total number of vertices
/// * `edges` - List of edges as (u, v) pairs
/// * `selected` - Boolean slice indicating which edges are selected
pub fn is_hamiltonian_cycle(
    num_vertices: usize,
    edges: &[(usize, usize)],
    selected: &[bool],
) -> bool {
    if selected.len() != edges.len() {
        return false;
    }

    let n = num_vertices;
    let mut degree = vec![0usize; n];
    let mut selected_count = 0;
    let mut first_vertex = None;

    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            if u >= n || v >= n {
                return false;
            }
            degree[u] += 1;
            degree[v] += 1;
            selected_count += 1;
            if first_vertex.is_none() {
                first_vertex = Some(u);
            }
        }
    }

    if selected_count != n {
        return false;
    }

    if degree.iter().any(|&d| d != 2) {
        return false;
    }

    let first = match first_vertex {
        Some(v) => v,
        None => return false,
    };

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, &sel) in selected.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::new();
    visited[first] = true;
    queue.push_back(first);
    let mut visit_count = 1;

    while let Some(node) = queue.pop_front() {
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                visit_count += 1;
                queue.push_back(neighbor);
            }
        }
    }

    visit_count == n
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/hamiltonian_cycle.rs"]
mod tests;
```

**Step 2: Register in mod.rs**

Add to `src/models/graph/mod.rs`:
- Add `mod hamiltonian_cycle;` line
- Add `pub use hamiltonian_cycle::{is_hamiltonian_cycle, HamiltonianCycle};`
- Add to module doc comment: `//! - [`HamiltonianCycle`]: Minimum weight Hamiltonian cycle`

**Step 3: Run tests to verify they pass**

Run: `cargo test hamiltonian_cycle -v`
Expected: All tests pass.

**Step 4: Run full check**

Run: `make test clippy`
Expected: All tests pass, no clippy warnings.

**Step 5: Commit**

```bash
git add src/models/graph/hamiltonian_cycle.rs src/models/graph/mod.rs src/unit_tests/models/graph/hamiltonian_cycle.rs
git commit -m "feat: add HamiltonianCycle model (#47)"
```

---

## Task 3: Add HamiltonianCycle to paper

**Files:**
- Modify: `docs/paper/reductions.typ`

**Step 1: Add display-name entry**

Add to the `display-name` dictionary:
```typst
"HamiltonianCycle": [Hamiltonian Cycle],
```

**Step 2: Add problem-def entry**

Add a `#problem-def` block after the existing graph problems (e.g., after `MaximumMatching`):

```typst
#problem-def("HamiltonianCycle")[
  Given an undirected graph $G=(V,E)$ with edge weights $w: E -> RR$, find a cycle visiting every vertex exactly once that minimizes $sum_(e in C) w(e)$.
]
```

**Step 3: Regenerate schema**

Run: `make export-schemas`

**Step 4: Verify paper builds**

Run: `make paper` (if Typst is available) or just verify no JSON errors.

**Step 5: Commit**

```bash
git add docs/paper/reductions.typ docs/src/reductions/problem_schemas.json
git commit -m "docs: add HamiltonianCycle definition to paper (#47)"
```

---

## Task 4: Final verification

**Step 1: Run full test suite**

Run: `make check`
Expected: fmt, clippy, and all tests pass.

**Step 2: Check coverage (if applicable)**

Run: `make coverage`
Expected: >95% coverage for new code.
