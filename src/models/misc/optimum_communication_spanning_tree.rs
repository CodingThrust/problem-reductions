//! Optimum Communication Spanning Tree problem implementation.
//!
//! Given a complete graph K_n with edge weights w(e) and communication
//! requirements r(u,v) for each vertex pair, find a spanning tree T that
//! minimizes the total communication cost: sum_{u<v} r(u,v) * W_T(u,v),
//! where W_T(u,v) is the sum of edge weights on the unique path from u to v in T.

use crate::registry::{CreateSpec, ProblemSchemaEntry};
use crate::traits::Problem;
use crate::types::Min;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

inventory::submit! {
    ProblemSchemaEntry {
        name: "OptimumCommunicationSpanningTree",
        display_name: "Optimum Communication Spanning Tree",
        aliases: &["OCST"],
        dimensions: &[],
        category: crate::registry::ProblemCategory::Misc,
        module_path: module_path!(),
        description: "Find spanning tree minimizing total weighted communication cost",
        fields: OptimumCommunicationSpanningTreeCreateSpec::FIELDS,
    }
}

/// The Optimum Communication Spanning Tree problem.
///
/// Given a complete graph K_n with edge weights w(e) >= 0 and communication
/// requirements r(u,v) >= 0 for each vertex pair, find a spanning tree T
/// minimizing the total communication cost:
///
///   sum_{u < v} r(u,v) * W_T(u,v)
///
/// where W_T(u,v) is the weight of the unique path between u and v in T.
///
/// # Representation
///
/// Each edge of K_n is assigned a binary variable (0 = not in tree, 1 = in tree).
/// Edges are ordered lexicographically: (0,1), (0,2), ..., (0,n-1), (1,2), ..., (n-2,n-1).
/// A valid spanning tree has exactly n-1 selected edges forming a connected subgraph.
///
/// # Example
///
/// ```
/// use problemreductions::models::misc::OptimumCommunicationSpanningTree;
/// use problemreductions::{Problem, BruteForce};
///
/// let problem = OptimumCommunicationSpanningTree::new(
///     vec![
///         vec![0, 1, 2],
///         vec![1, 0, 3],
///         vec![2, 3, 0],
///     ],
///     vec![
///         vec![0, 1, 1],
///         vec![1, 0, 1],
///         vec![1, 1, 0],
///     ],
/// );
/// let solver = BruteForce::new();
/// let solution = solver.solve(&problem).unwrap();
/// assert!(solution.is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimumCommunicationSpanningTree {
    num_vertices: usize,
    edge_weights: Vec<Vec<i64>>,
    requirements: Vec<Vec<i64>>,
}

#[derive(Debug, Deserialize, crate::CreateSpec)]
struct OptimumCommunicationSpanningTreeCreateSpec {
    /// Number of vertices.
    num_vertices: usize,
    /// Symmetric weight matrix; defaults to unit off-diagonal weights.
    edge_weights: Option<Vec<Vec<i64>>>,
    /// Symmetric communication requirement matrix.
    requirements: Vec<Vec<i64>>,
}
impl TryFrom<OptimumCommunicationSpanningTreeCreateSpec> for OptimumCommunicationSpanningTree {
    type Error = crate::registry::ConstructionError;
    fn try_from(spec: OptimumCommunicationSpanningTreeCreateSpec) -> Result<Self, Self::Error> {
        let n = spec.num_vertices;
        if n < 2 {
            return Err("must have at least two vertices".to_string().into());
        }
        let edge_weights = spec.edge_weights.unwrap_or_else(|| {
            (0..n)
                .map(|i| (0..n).map(|j| i64::from(i != j)).collect())
                .collect()
        });
        for (name, matrix) in [
            ("edge_weights", &edge_weights),
            ("requirements", &spec.requirements),
        ] {
            if matrix.len() != n || matrix.iter().any(|row| row.len() != n) {
                return Err(format!("{name} must be a {n} x {n} matrix").into());
            }
            for (i, row) in matrix.iter().enumerate() {
                if row[i] != 0 {
                    return Err(format!("{name} diagonal must be zero").into());
                }
                for (j, &value) in row.iter().enumerate().skip(i + 1) {
                    if value != matrix[j][i] || value < 0 {
                        return Err(format!("{name} must be symmetric and nonnegative").into());
                    }
                }
            }
        }
        Ok(Self::new(edge_weights, spec.requirements))
    }
}

impl OptimumCommunicationSpanningTree {
    /// Create a new OptimumCommunicationSpanningTree instance.
    ///
    /// # Arguments
    ///
    /// * `edge_weights` - Symmetric n x n matrix with w(i,i) = 0 and w(i,j) >= 0.
    /// * `requirements` - Symmetric n x n matrix with r(i,i) = 0 and r(i,j) >= 0.
    ///
    /// # Panics
    ///
    /// Panics if the matrices are not square, not the same size, have nonzero
    /// diagonals, are not symmetric, or contain negative entries.
    pub fn new(edge_weights: Vec<Vec<i64>>, requirements: Vec<Vec<i64>>) -> Self {
        let n = edge_weights.len();
        assert!(n >= 2, "must have at least 2 vertices");
        assert_eq!(
            requirements.len(),
            n,
            "requirements matrix must have same size as edge_weights"
        );

        for (i, row) in edge_weights.iter().enumerate() {
            assert_eq!(
                row.len(),
                n,
                "edge_weights must be square: row {i} has length {} but expected {n}",
                row.len()
            );
            assert_eq!(
                row[i], 0,
                "diagonal of edge_weights must be zero: edge_weights[{i}][{i}] = {}",
                row[i]
            );
        }

        for (i, row) in requirements.iter().enumerate() {
            assert_eq!(
                row.len(),
                n,
                "requirements must be square: row {i} has length {} but expected {n}",
                row.len()
            );
            assert_eq!(
                row[i], 0,
                "diagonal of requirements must be zero: requirements[{i}][{i}] = {}",
                row[i]
            );
        }

        // Check symmetry and non-negativity
        for i in 0..n {
            for j in (i + 1)..n {
                assert_eq!(
                    edge_weights[i][j], edge_weights[j][i],
                    "edge_weights must be symmetric: w[{i}][{j}]={} != w[{j}][{i}]={}",
                    edge_weights[i][j], edge_weights[j][i]
                );
                assert!(
                    edge_weights[i][j] >= 0,
                    "edge_weights must be non-negative: w[{i}][{j}]={}",
                    edge_weights[i][j]
                );
                assert_eq!(
                    requirements[i][j], requirements[j][i],
                    "requirements must be symmetric: r[{i}][{j}]={} != r[{j}][{i}]={}",
                    requirements[i][j], requirements[j][i]
                );
                assert!(
                    requirements[i][j] >= 0,
                    "requirements must be non-negative: r[{i}][{j}]={}",
                    requirements[i][j]
                );
            }
        }

        Self {
            num_vertices: n,
            edge_weights,
            requirements,
        }
    }

    /// Returns the number of vertices.
    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Returns the number of edges in the complete graph K_n.
    pub fn num_edges(&self) -> usize {
        self.num_vertices * (self.num_vertices - 1) / 2
    }

    /// Returns the edge weight matrix.
    pub fn edge_weights(&self) -> &Vec<Vec<i64>> {
        &self.edge_weights
    }

    /// Returns the requirements matrix.
    pub fn requirements(&self) -> &Vec<Vec<i64>> {
        &self.requirements
    }

    /// Returns the list of edges in lexicographic order: (0,1), (0,2), ..., (n-2,n-1).
    pub fn edges(&self) -> Vec<(usize, usize)> {
        let n = self.num_vertices;
        let mut edges = Vec::with_capacity(self.num_edges());
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j));
            }
        }
        edges
    }

    /// Map a pair (i, j) with i < j to its edge index.
    pub fn edge_index(i: usize, j: usize, n: usize) -> usize {
        debug_assert!(i < j && j < n);
        i * n - i * (i + 1) / 2 + (j - i - 1)
    }
}

/// Check if a configuration forms a valid spanning tree of K_n.
fn is_valid_spanning_tree(n: usize, edges: &[(usize, usize)], config: &[bool]) -> bool {
    if config.len() != edges.len() {
        return false;
    }

    // Count selected edges: must be exactly n-1
    let selected_count = config.iter().filter(|&&selected| selected).count();
    if selected_count != n - 1 {
        return false;
    }

    // Build adjacency and check connectivity via BFS
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (idx, &sel) in config.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            adj[u].push(v);
            adj[v].push(u);
        }
    }

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                queue.push_back(u);
            }
        }
    }

    visited.iter().all(|&v| v)
}

/// Compute the communication cost of a spanning tree.
///
/// For each pair (u, v) with u < v, compute W_T(u,v) via BFS in the tree,
/// then accumulate r(u,v) * W_T(u,v).
fn communication_cost(
    n: usize,
    edges: &[(usize, usize)],
    config: &[bool],
    edge_weights: &[Vec<i64>],
    requirements: &[Vec<i64>],
) -> Result<i64, crate::traits::EvaluationError> {
    // Build weighted adjacency list for the tree
    let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for (idx, &sel) in config.iter().enumerate() {
        if sel {
            let (u, v) = edges[idx];
            let w = edge_weights[u][v];
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
    }

    let mut total_cost: i64 = 0;

    // For each source vertex, BFS to find path weights to all other vertices
    for src in 0..n {
        let mut dist = vec![-1i64; n];
        dist[src] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(src);
        while let Some(u) = queue.pop_front() {
            for &(v, w) in &adj[u] {
                if dist[v] < 0 {
                    dist[v] = dist[u].checked_add(w).ok_or_else(|| {
                        crate::traits::EvaluationError::IntegerOverflow(
                            "summing communication-tree path weights".to_string(),
                        )
                    })?;
                    queue.push_back(v);
                }
            }
        }

        // Accumulate r(src, dst) * W_T(src, dst) for dst > src
        for (dst, &d) in dist.iter().enumerate().skip(src + 1) {
            let term = requirements[src][dst].checked_mul(d).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "multiplying communication requirement by path weight".to_string(),
                )
            })?;
            total_cost = total_cost.checked_add(term).ok_or_else(|| {
                crate::traits::EvaluationError::IntegerOverflow(
                    "summing communication-tree costs".to_string(),
                )
            })?;
        }
    }

    Ok(total_cost)
}

impl Problem for OptimumCommunicationSpanningTree {
    const NAME: &'static str = "OptimumCommunicationSpanningTree";
    type Solution = Vec<bool>;
    type Value = Min<i64>;

    crate::problem_parameters![("num_edges", num_edges), ("num_vertices", num_vertices),];

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }

    fn evaluate(
        &self,
        config: &Self::Solution,
    ) -> Result<Min<i64>, crate::traits::EvaluationError> {
        if config.len() != self.edges().len() {
            return Err(crate::traits::EvaluationError::InvalidConfiguration(
                "edge-selection length does not match the complete graph".into(),
            ));
        }
        Ok({
            let edges = self.edges();
            if !is_valid_spanning_tree(self.num_vertices, &edges, config) {
                return Ok(Min(None));
            }
            Min(Some(communication_cost(
                self.num_vertices,
                &edges,
                config,
                &self.edge_weights,
                &self.requirements,
            )?))
        })
    }
}

impl crate::solvers::BruteForceProblem for OptimumCommunicationSpanningTree {
    fn dimensions(&self) -> Vec<usize> {
        vec![2; self.num_edges()]
    }
}

crate::declare_variants! {
    default OptimumCommunicationSpanningTree => "2^num_edges" create OptimumCommunicationSpanningTreeCreateSpec,
}

crate::register_brute_force! {
    OptimumCommunicationSpanningTree decode |_, indices: Vec<usize>| crate::config::config_to_bits(&indices),
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    // K4 example from issue #906
    // Edge weights:
    //   w(0,1)=1, w(0,2)=3, w(0,3)=2, w(1,2)=2, w(1,3)=4, w(2,3)=1
    // Requirements:
    //   r(0,1)=2, r(0,2)=1, r(0,3)=3, r(1,2)=1, r(1,3)=1, r(2,3)=2
    // Optimal tree: {(0,1), (0,3), (2,3)} = edges at indices 0, 2, 5
    // Optimal cost: 20
    let edge_weights = vec![
        vec![0, 1, 3, 2],
        vec![1, 0, 2, 4],
        vec![3, 2, 0, 1],
        vec![2, 4, 1, 0],
    ];
    let requirements = vec![
        vec![0, 2, 1, 3],
        vec![2, 0, 1, 1],
        vec![1, 1, 0, 2],
        vec![3, 1, 2, 0],
    ];
    // Edges in lex order: (0,1)=idx0, (0,2)=idx1, (0,3)=idx2, (1,2)=idx3, (1,3)=idx4, (2,3)=idx5
    // Optimal tree: {(0,1), (0,3), (2,3)} -> config = [1, 0, 1, 0, 0, 1]
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "optimum_communication_spanning_tree",
        instance: Box::new(OptimumCommunicationSpanningTree::new(
            edge_weights,
            requirements,
        )),
        optimal_config: serde_json::json!(vec![true, false, true, false, false, true]),
        optimal_value: serde_json::json!(20),
    }]
}

#[cfg(test)]
#[path = "../../unit_tests/models/misc/optimum_communication_spanning_tree.rs"]
mod tests;
