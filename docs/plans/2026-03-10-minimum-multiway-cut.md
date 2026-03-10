# MinimumMultiwayCut Model Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the MinimumMultiwayCut problem model — a graph partitioning problem that finds a minimum-weight edge set whose removal disconnects all terminal pairs.

**Architecture:** New optimization model in `src/models/graph/` with edge-based binary variables (`dims = vec![2; num_edges]`), a feasibility check via BFS connectivity, and `Direction::Minimize`. Solved via existing BruteForce solver.

**Tech Stack:** Rust, serde, inventory crate for schema registration

**Issue:** #184

---

## Task 1: Implement the model struct and Problem trait

**Files:**
- Create: `src/models/graph/minimum_multiway_cut.rs`

### Design Notes

**Configuration space:** Unlike most graph problems (vertex-based), this problem uses **edge-based binary variables**: `dims() = vec![2; num_edges]`. Each variable `x_e ∈ {0, 1}` indicates whether edge `e` is removed (1) or kept (0).

**Feasibility check:** A configuration is feasible iff removing the cut edges disconnects every pair of terminals. Implementation: build adjacency list from non-cut edges, run BFS/DFS from first terminal, check that no other terminal is reachable. Repeat for all terminal components.

**Complexity getters:** The best-known algorithm is `O(1.84^k * n^3)` (Cao, Chen & Fan 2013), so we need `num_terminals()`, `num_vertices()`, and `num_edges()` getters.

- [ ] **Step 1: Create the model file with struct, inventory, and inherent methods**

```rust
// src/models/graph/minimum_multiway_cut.rs

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::topology::{Graph, SimpleGraph};
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize, WeightElement};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

inventory::submit! {
    ProblemSchemaEntry {
        name: "MinimumMultiwayCut",
        module_path: module_path!(),
        description: "Find minimum weight set of edges whose removal disconnects all terminal pairs",
        fields: &[
            FieldInfo { name: "graph", type_name: "G", description: "The undirected graph G=(V,E)" },
            FieldInfo { name: "terminals", type_name: "Vec<usize>", description: "Terminal vertices that must be separated" },
            FieldInfo { name: "edge_weights", type_name: "Vec<W>", description: "Edge weights w: E -> R (same order as graph.edges())" },
        ],
    }
}

/// The Minimum Multiway Cut problem.
///
/// Given an undirected weighted graph G = (V, E, w) and a set of k terminal
/// vertices T = {t_1, ..., t_k}, find a minimum-weight set of edges C ⊆ E
/// such that no two terminals remain in the same connected component of
/// G' = (V, E \ C).
///
/// # Representation
///
/// Each edge is assigned a binary variable:
/// - 0: edge is kept
/// - 1: edge is removed (in the cut)
///
/// A configuration is feasible if removing the cut edges disconnects all
/// terminal pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumMultiwayCut<G, W> {
    graph: G,
    terminals: Vec<usize>,
    edge_weights: Vec<W>,
}

impl<G: Graph, W: Clone + Default> MinimumMultiwayCut<G, W> {
    /// Create a MinimumMultiwayCut problem.
    ///
    /// # Panics
    /// - If `edge_weights.len() != graph.num_edges()`
    /// - If `terminals.len() < 2`
    /// - If any terminal index is out of bounds
    pub fn new(graph: G, terminals: Vec<usize>, edge_weights: Vec<W>) -> Self {
        assert_eq!(
            edge_weights.len(),
            graph.num_edges(),
            "edge_weights length must match num_edges"
        );
        assert!(terminals.len() >= 2, "need at least 2 terminals");
        // Check for duplicate terminals
        let mut sorted = terminals.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), terminals.len(), "duplicate terminal indices");
        for &t in &terminals {
            assert!(t < graph.num_vertices(), "terminal index out of bounds");
        }
        Self {
            graph,
            terminals,
            edge_weights,
        }
    }

    /// Get a reference to the underlying graph.
    pub fn graph(&self) -> &G {
        &self.graph
    }

    /// Get the terminal vertices.
    pub fn terminals(&self) -> &[usize] {
        &self.terminals
    }

    /// Get the edge weights.
    pub fn edge_weights(&self) -> &[W] {
        &self.edge_weights
    }
}

impl<G: Graph, W: WeightElement> MinimumMultiwayCut<G, W> {
    /// Number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.num_vertices()
    }

    /// Number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.num_edges()
    }

    /// Number of terminal vertices.
    pub fn num_terminals(&self) -> usize {
        self.terminals.len()
    }
}
```

- [ ] **Step 2: Add the feasibility check helper**

```rust
/// Check if all terminals are in distinct connected components
/// when edges marked as cut (config[e] == 1) are removed.
fn terminals_separated<G: Graph>(
    graph: &G,
    terminals: &[usize],
    config: &[usize],
) -> bool {
    let n = graph.num_vertices();
    let edges = graph.edges();

    // Build adjacency list from non-cut edges
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, &(u, v)) in edges.iter().enumerate() {
        if config[idx] == 0 {
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    // Find connected component of each terminal via BFS
    let mut component = vec![usize::MAX; n];
    let mut comp_id = 0;
    for &t in terminals {
        if component[t] != usize::MAX {
            // Terminal already reached from a previous terminal's BFS
            // => two terminals share a component => infeasible
            return false;
        }
        // BFS from terminal t
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(t);
        component[t] = comp_id;
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if component[v] == usize::MAX {
                    component[v] = comp_id;
                    queue.push_back(v);
                }
            }
        }
        comp_id += 1;
    }
    true
}
```

- [ ] **Step 3: Implement Problem and OptimizationProblem traits**

```rust
impl<G, W> Problem for MinimumMultiwayCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    const NAME: &'static str = "MinimumMultiwayCut";
    type Metric = SolutionSize<W::Sum>;

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![G, W]
    }

    fn dims(&self) -> Vec<usize> {
        vec![2; self.graph.num_edges()]
    }

    fn evaluate(&self, config: &[usize]) -> SolutionSize<W::Sum> {
        // Check feasibility: all terminals must be in distinct components
        if !terminals_separated(&self.graph, &self.terminals, config) {
            return SolutionSize::Invalid;
        }
        // Sum weights of cut edges
        let mut total = W::Sum::zero();
        for (idx, &selected) in config.iter().enumerate() {
            if selected == 1 {
                total += self.edge_weights[idx].to_sum();
            }
        }
        SolutionSize::Valid(total)
    }
}

impl<G, W> OptimizationProblem for MinimumMultiwayCut<G, W>
where
    G: Graph + crate::variant::VariantParam,
    W: WeightElement + crate::variant::VariantParam,
{
    type Value = W::Sum;

    fn direction(&self) -> Direction {
        Direction::Minimize
    }
}
```

- [ ] **Step 4: Add `declare_variants!` and test link**

```rust
crate::declare_variants! {
    MinimumMultiwayCut<SimpleGraph, i32> => "1.84^num_terminals * num_vertices^3",
}

#[cfg(test)]
#[path = "../../unit_tests/models/graph/minimum_multiway_cut.rs"]
mod tests;
```

- [ ] **Step 5: Verify the file compiles (no tests yet)**

Run: `cargo check 2>&1 | head -30`
Expected: Compilation errors about missing module registration (fixed in Task 2)

---

## Task 2: Register the model in module system and CLI

**Files:**
- Modify: `src/models/graph/mod.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/lib.rs`
- Modify: `problemreductions-cli/src/dispatch.rs`
- Modify: `problemreductions-cli/src/problem_name.rs`

- [ ] **Step 1: Register in `src/models/graph/mod.rs`**

Add to the module declarations (alphabetically) and update the module doc comment:
```rust
//! - [`MinimumMultiwayCut`]: Minimum weight multiway cut
```

```rust
pub(crate) mod minimum_multiway_cut;
```

Add to the re-exports:
```rust
pub use minimum_multiway_cut::MinimumMultiwayCut;
```

- [ ] **Step 2: Register in `src/models/mod.rs`**

Add `MinimumMultiwayCut` to the `graph` re-export line:
```rust
pub use graph::{
    BicliqueCover, KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet,
    MaximumMatching, MinimumDominatingSet, MinimumMultiwayCut, MinimumVertexCover, SpinGlass,
    TravelingSalesman,
};
```

- [ ] **Step 3: Register in prelude (`src/lib.rs`)**

Add `MinimumMultiwayCut` to the prelude graph imports (line ~42-45):
```rust
pub use crate::models::graph::{
    KColoring, MaxCut, MaximalIS, MaximumClique, MaximumIndependentSet, MaximumMatching,
    MinimumDominatingSet, MinimumMultiwayCut, MinimumVertexCover, TravelingSalesman,
};
```

- [ ] **Step 4: Add CLI dispatch in `problemreductions-cli/src/dispatch.rs`**

Add import at the top (if needed) and match arms in both `load_problem()` and `serialize_any_problem()`:

```rust
// In load_problem():
"MinimumMultiwayCut" => deser_opt::<MinimumMultiwayCut<SimpleGraph, i32>>(data),

// In serialize_any_problem():
"MinimumMultiwayCut" => try_ser::<MinimumMultiwayCut<SimpleGraph, i32>>(any),
```

- [ ] **Step 4: Add CLI alias in `problemreductions-cli/src/problem_name.rs`**

```rust
// In resolve_alias():
"minimummultiwaycut" | "mmc" => "MinimumMultiwayCut".to_string(),
```

- [ ] **Step 6: Verify compilation**

Run: `cargo check`
Expected: PASS (no errors)

---

## Task 3: Write unit tests

**Files:**
- Create: `src/unit_tests/models/graph/minimum_multiway_cut.rs`

- [ ] **Step 1: Write the test file**

```rust
use super::*;
use crate::solvers::BruteForce;
use crate::topology::SimpleGraph;
use crate::traits::{OptimizationProblem, Problem};
use crate::types::{Direction, SolutionSize};

#[test]
fn test_minimummultiwaycut_creation() {
    // 5 vertices, 6 edges, 3 terminals
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)],
    );
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
    assert_eq!(problem.dims().len(), 6); // 6 edges
    assert_eq!(problem.num_vertices(), 5);
    assert_eq!(problem.num_edges(), 6);
    assert_eq!(problem.num_terminals(), 3);
}

#[test]
fn test_minimummultiwaycut_evaluate_valid() {
    // Issue example: 5 vertices, terminals {0,2,4}
    // Edges: (0,1)w=2, (1,2)w=3, (2,3)w=1, (3,4)w=2, (0,4)w=4, (1,3)w=5
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)],
    );
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    // Optimal cut: remove edges (0,1), (0,4), (3,4) => indices 0, 4, 3
    // config: [1, 0, 0, 1, 1, 0] => weight 2 + 2 + 4 = 8
    let config = vec![1, 0, 0, 1, 1, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Valid(8));
}

#[test]
fn test_minimummultiwaycut_evaluate_invalid() {
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)],
    );
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    // No edges cut: all terminals connected => invalid
    let config = vec![0, 0, 0, 0, 0, 0];
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Invalid);
}

#[test]
fn test_minimummultiwaycut_direction() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i32, 1]);
    assert_eq!(problem.direction(), Direction::Minimize);
}

#[test]
fn test_minimummultiwaycut_brute_force() {
    // Issue example: optimal cut has weight 8
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)],
    );
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    assert!(!solutions.is_empty());
    for sol in &solutions {
        let val = problem.evaluate(sol);
        assert_eq!(val, SolutionSize::Valid(8));
    }
}

#[test]
fn test_minimummultiwaycut_two_terminals() {
    // k=2: classical min s-t cut. Path graph: 0-1-2, terminals {0,2}
    // Edges: (0,1)w=3, (1,2)w=5
    // Min cut: remove (0,1) with weight 3
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![3i32, 5]);

    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(3));
    }
}

#[test]
fn test_minimummultiwaycut_all_edges_cut() {
    // Cutting all edges should always be valid (trivially separates everything)
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)],
    );
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);
    let config = vec![1, 1, 1, 1, 1, 1]; // cut all edges
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Valid(2 + 3 + 1 + 2 + 4 + 5)); // sum = 17
}

#[test]
fn test_minimummultiwaycut_already_disconnected() {
    // Terminals already in different components => empty cut is valid
    // Graph: 0-1  2-3, terminals {0, 2}
    let graph = SimpleGraph::new(4, vec![(0, 1), (2, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i32, 1]);
    let config = vec![0, 0]; // no edges cut
    let result = problem.evaluate(&config);
    assert_eq!(result, SolutionSize::Valid(0));

    // BruteForce should find the empty cut as optimal
    let solver = BruteForce::new();
    let solutions = solver.find_all_best(&problem);
    for sol in &solutions {
        assert_eq!(problem.evaluate(sol), SolutionSize::Valid(0));
    }
}

#[test]
fn test_minimummultiwaycut_serialization() {
    let graph = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2], vec![1i32, 2]);
    let json = serde_json::to_string(&problem).unwrap();
    let restored: MinimumMultiwayCut<SimpleGraph, i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.num_vertices(), 3);
    assert_eq!(restored.num_edges(), 2);
    assert_eq!(restored.terminals(), &[0, 2]);
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test minimum_multiway_cut --lib -- --nocapture`
Expected: All tests PASS

- [ ] **Step 3: Commit model + tests + registration**

```bash
git add src/models/graph/minimum_multiway_cut.rs \
        src/unit_tests/models/graph/minimum_multiway_cut.rs \
        src/models/graph/mod.rs \
        src/models/mod.rs \
        problemreductions-cli/src/dispatch.rs \
        problemreductions-cli/src/problem_name.rs
git commit -m "feat: add MinimumMultiwayCut model (#184)"
```

---

## Task 4: Write example program

**Files:**
- Create: `examples/minimummultiwaycut.rs`
- Modify: `tests/suites/examples.rs`

- [ ] **Step 1: Write the example program**

Use the issue's worked example (5 vertices, 3 terminals, optimal cut weight 8).

```rust
// examples/minimummultiwaycut.rs
// MinimumMultiwayCut example: find minimum weight edge cut separating terminals.

use problemreductions::models::graph::MinimumMultiwayCut;
use problemreductions::topology::SimpleGraph;
use problemreductions::{BruteForce, Problem, Solver};

pub fn run() {
    // 5 vertices, terminals {0, 2, 4}
    // Edges with weights: (0,1)=2, (1,2)=3, (2,3)=1, (3,4)=2, (0,4)=4, (1,3)=5
    let graph = SimpleGraph::new(
        5,
        vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)],
    );
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let solver = BruteForce::new();
    let best = solver.find_best(&problem).expect("should find a solution");
    let value = problem.evaluate(&best);

    println!("Optimal multiway cut: {:?}", best);
    println!("Cut weight: {:?}", value);

    // Export as JSON
    let json = serde_json::json!({
        "problem": problem,
        "solution": best,
        "objective": 8,
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

fn main() {
    run();
}
```

- [ ] **Step 2: Register in `tests/suites/examples.rs`**

Add `example_test!` and `example_fn!` entries:
```rust
example_test!(minimummultiwaycut);
// ...in the test list:
example_fn!(test_minimummultiwaycut, minimummultiwaycut);
```

- [ ] **Step 3: Run the example**

Run: `cargo run --example minimummultiwaycut`
Expected: Prints optimal cut with weight 8

- [ ] **Step 4: Run example test**

Run: `cargo test test_minimummultiwaycut --test main`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add examples/minimummultiwaycut.rs tests/suites/examples.rs
git commit -m "feat: add MinimumMultiwayCut example (#184)"
```

---

## Task 5: Regenerate exports and run full checks

- [ ] **Step 1: Regenerate reduction graph and schemas**

```bash
cargo run --example export_graph
cargo run --example export_schemas
```

- [ ] **Step 2: Run full check suite**

```bash
make check   # fmt + clippy + test
```
Expected: All pass

- [ ] **Step 3: Commit any generated file changes**

```bash
git add docs/data/reduction_graph.json docs/data/problem_schemas.json
git commit -m "chore: regenerate exports for MinimumMultiwayCut (#184)"
```

---

## Task 6: Document in paper

Invoke `/write-model-in-paper` to add the problem definition entry in `docs/paper/reductions.typ`.

Key content to include:
- **Formal definition:** Given G=(V,E,w) and terminals T={t_1,...,t_k}, find minimum-weight C⊆E separating all terminals
- **Background:** Generalizes min s-t cut (k=2, polynomial) to k≥3 (NP-hard). Important in VLSI, image segmentation. (2−2/k)-approximation exists.
- **Example:** The issue's 5-vertex instance with CeTZ visualization showing the graph, terminals highlighted, and cut edges
- **Algorithm:** Cao, Chen & Fan (2013), O*(1.84^k)

- [ ] **Step 1: Run `/write-model-in-paper`**
- [ ] **Step 2: Commit paper changes**

---

## Task 7: Final verification

- [ ] **Step 1: Run full test suite**

```bash
make test clippy
```

- [ ] **Step 2: Run `/review-implementation` to verify all structural and semantic checks pass**
