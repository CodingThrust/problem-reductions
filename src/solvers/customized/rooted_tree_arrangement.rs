//! Structured backtracking backend for RootedTreeArrangement.
//!
//! Enumerates rooted labeled trees, then for each tree searches for a
//! valid vertex-to-node permutation mapping with bounded total stretch.
//! Prunes using ancestor-comparability checks and partial stretch bounds.

use crate::models::graph::RootedTreeArrangement;
use crate::topology::{Graph, SimpleGraph};

/// Find a witness for RootedTreeArrangement, or None if no solution exists.
pub(crate) fn find_witness(problem: &RootedTreeArrangement<SimpleGraph>) -> Option<Vec<usize>> {
    let graph = problem.graph();
    let n = graph.num_vertices();
    let bound = problem.bound();

    if n == 0 {
        return Some(vec![]);
    }

    let edges = graph.edges();

    // Build graph adjacency list
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in &edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    // Try each possible root
    for root in 0..n {
        let mut parent = vec![0usize; n];
        parent[root] = root;

        // Non-root nodes to assign parents to
        let non_root: Vec<usize> = (0..n).filter(|&i| i != root).collect();

        if let Some(config) =
            search_trees(n, root, &non_root, 0, &mut parent, &edges, &adj, bound)
        {
            return Some(config);
        }
    }

    None
}

/// Recursively build a rooted tree by assigning parents to non-root nodes,
/// then search for a valid permutation mapping.
fn search_trees(
    n: usize,
    root: usize,
    non_root: &[usize],
    depth_idx: usize,
    parent: &mut Vec<usize>,
    edges: &[(usize, usize)],
    adj: &[Vec<usize>],
    bound: usize,
) -> Option<Vec<usize>> {
    if depth_idx == non_root.len() {
        // All parents assigned — validate tree structure and search for mapping
        if let Some(depths) = compute_depths(parent, root) {
            return search_mapping(n, parent, &depths, edges, adj, bound);
        }
        return None;
    }

    let node = non_root[depth_idx];

    // Try each possible parent for this node
    for p in 0..n {
        if p == node {
            continue; // Can't be own parent (only root has that)
        }
        parent[node] = p;
        if let Some(config) =
            search_trees(n, root, non_root, depth_idx + 1, parent, edges, adj, bound)
        {
            return Some(config);
        }
    }

    None
}

/// Compute depths from a parent array. Returns None if not a valid tree.
fn compute_depths(parent: &[usize], root: usize) -> Option<Vec<usize>> {
    let n = parent.len();
    let mut depth = vec![0usize; n];
    let mut computed = vec![false; n];
    computed[root] = true;

    for start in 0..n {
        if computed[start] {
            continue;
        }
        // Walk up to root, collecting path
        let mut path = vec![start];
        let mut current = start;
        loop {
            let p = parent[current];
            if computed[p] {
                // Compute depths for the whole path
                let base_depth = depth[p] + 1;
                let path_len = path.len();
                for (i, &node) in path.iter().rev().enumerate() {
                    depth[node] = base_depth + i;
                    computed[node] = true;
                }
                break;
            }
            if p == current {
                return None; // Second root or self-loop (not root)
            }
            if path.contains(&p) {
                return None; // Cycle detected
            }
            path.push(p);
            current = p;
        }
    }

    if computed.iter().all(|&c| c) {
        Some(depth)
    } else {
        None
    }
}

/// Check if `ancestor` is an ancestor of `descendant` in the tree defined by `parent`.
fn is_ancestor(parent: &[usize], ancestor: usize, descendant: usize) -> bool {
    let mut current = descendant;
    loop {
        if current == ancestor {
            return true;
        }
        let next = parent[current];
        if next == current {
            return false; // Reached root without finding ancestor
        }
        current = next;
    }
}

/// Check if two nodes are ancestor-comparable (one is an ancestor of the other).
fn are_comparable(parent: &[usize], u: usize, v: usize) -> bool {
    is_ancestor(parent, u, v) || is_ancestor(parent, v, u)
}

/// Search for a permutation mapping of graph vertices to tree nodes
/// such that all graph edges map to ancestor-comparable pairs and
/// total stretch <= bound.
fn search_mapping(
    n: usize,
    parent: &[usize],
    depths: &[usize],
    edges: &[(usize, usize)],
    adj: &[Vec<usize>],
    bound: usize,
) -> Option<Vec<usize>> {
    let mut mapping = vec![usize::MAX; n]; // graph vertex -> tree node
    let mut used = vec![false; n]; // which tree nodes are taken

    search_mapping_dfs(
        n,
        parent,
        depths,
        edges,
        adj,
        bound,
        &mut mapping,
        &mut used,
        0,
        0,
    )
}

fn search_mapping_dfs(
    n: usize,
    parent: &[usize],
    depths: &[usize],
    _edges: &[(usize, usize)],
    adj: &[Vec<usize>],
    bound: usize,
    mapping: &mut Vec<usize>,
    used: &mut Vec<bool>,
    vertex: usize,
    partial_stretch: usize,
) -> Option<Vec<usize>> {
    if vertex == n {
        // All vertices assigned
        if partial_stretch <= bound {
            let mut config = parent.to_vec();
            config.extend_from_slice(mapping);
            return Some(config);
        }
        return None;
    }

    for tree_node in 0..n {
        if used[tree_node] {
            continue;
        }

        // Check ancestor-comparability with all already-mapped neighbors
        let mut valid = true;
        let mut added_stretch = 0usize;
        for &neighbor in &adj[vertex] {
            if neighbor < vertex && mapping[neighbor] != usize::MAX {
                let t_neighbor = mapping[neighbor];
                if !are_comparable(parent, tree_node, t_neighbor) {
                    valid = false;
                    break;
                }
                added_stretch += depths[tree_node].abs_diff(depths[t_neighbor]);
            }
        }

        if !valid {
            continue;
        }

        let new_stretch = partial_stretch + added_stretch;
        if new_stretch > bound {
            continue;
        }

        mapping[vertex] = tree_node;
        used[tree_node] = true;

        if let Some(config) = search_mapping_dfs(
            n,
            parent,
            depths,
            _edges,
            adj,
            bound,
            mapping,
            used,
            vertex + 1,
            new_stretch,
        ) {
            return Some(config);
        }

        mapping[vertex] = usize::MAX;
        used[tree_node] = false;
    }

    None
}
