//! Validation of edge-selected paths, shared by graph problem models.

use std::collections::VecDeque;

/// Whether selected edges form a simple undirected path between valid terminals.
pub(crate) fn is_simple_st_path(
    num_vertices: usize,
    edges: &[(usize, usize)],
    source_vertex: usize,
    target_vertex: usize,
    config: &[bool],
) -> bool {
    if config.len() != edges.len() {
        return false;
    }

    if source_vertex == target_vertex {
        return config.iter().all(|&selected| !selected);
    }

    let mut degree = vec![0usize; num_vertices];
    let mut adjacency = vec![Vec::new(); num_vertices];
    let mut selected_edge_count = 0usize;

    for (idx, &selected) in config.iter().enumerate() {
        if !selected {
            continue;
        }
        let (u, v) = edges[idx];
        degree[u] += 1;
        degree[v] += 1;
        if degree[u] > 2 || degree[v] > 2 {
            return false;
        }
        adjacency[u].push(v);
        adjacency[v].push(u);
        selected_edge_count += 1;
    }

    if selected_edge_count == 0 {
        return false;
    }
    if degree[source_vertex] != 1 || degree[target_vertex] != 1 {
        return false;
    }

    let mut selected_vertex_count = 0usize;
    for (vertex, &vertex_degree) in degree.iter().enumerate() {
        if vertex_degree == 0 {
            continue;
        }
        selected_vertex_count += 1;
        if vertex != source_vertex && vertex != target_vertex && vertex_degree != 2 {
            return false;
        }
    }

    if selected_edge_count != selected_vertex_count - 1 {
        return false;
    }

    let mut visited = vec![false; num_vertices];
    let mut queue = VecDeque::new();
    visited[source_vertex] = true;
    queue.push_back(source_vertex);

    while let Some(vertex) = queue.pop_front() {
        for &neighbor in &adjacency[vertex] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    visited[target_vertex]
        && degree
            .iter()
            .enumerate()
            .all(|(vertex, &vertex_degree)| vertex_degree == 0 || visited[vertex])
}
