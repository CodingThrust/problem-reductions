//! Exact intersection-basis solver via maximal cliques and edge-cover dynamic programming.

use crate::models::graph::MinimumIntersectionGraphBasis;
use crate::topology::{Graph, SimpleGraph};

pub(crate) fn solve(
    problem: &MinimumIntersectionGraphBasis<SimpleGraph>,
) -> Option<Vec<Vec<bool>>> {
    let n = problem.num_vertices();
    let edges = problem.graph().edges();
    if edges.is_empty() {
        return Some(vec![Vec::new(); n]);
    }

    let mut cliques = Vec::new();
    maximal_cliques(
        problem.graph(),
        Vec::new(),
        (0..n).collect(),
        Vec::new(),
        &mut cliques,
    );
    let covers = cliques
        .iter()
        .map(|clique| {
            edges
                .iter()
                .enumerate()
                .fold(0usize, |mask, (edge, &(u, v))| {
                    if clique.contains(&u) && clique.contains(&v) {
                        mask | (1 << edge)
                    } else {
                        mask
                    }
                })
        })
        .collect::<Vec<_>>();

    let full = (1usize << edges.len()) - 1;
    let mut count = vec![usize::MAX; full + 1];
    let mut previous = vec![None; full + 1];
    count[0] = 0;
    for mask in 0..=full {
        if count[mask] == usize::MAX {
            continue;
        }
        for (clique, &cover) in covers.iter().enumerate() {
            let next = mask | cover;
            if count[next] > count[mask] + 1 {
                count[next] = count[mask] + 1;
                previous[next] = Some((mask, clique));
            }
        }
    }

    let mut chosen = Vec::new();
    let mut mask = full;
    while mask != 0 {
        let (prior, clique) = previous[mask].unwrap();
        chosen.push(clique);
        mask = prior;
    }
    let mut solution = vec![vec![false; edges.len()]; n];
    for (element, clique) in chosen.into_iter().enumerate() {
        for &vertex in &cliques[clique] {
            solution[vertex][element] = true;
        }
    }
    Some(solution)
}

fn maximal_cliques(
    graph: &SimpleGraph,
    clique: Vec<usize>,
    mut candidates: Vec<usize>,
    mut excluded: Vec<usize>,
    output: &mut Vec<Vec<usize>>,
) {
    if candidates.is_empty() && excluded.is_empty() {
        if clique.len() >= 2 {
            output.push(clique);
        }
        return;
    }

    while let Some(vertex) = candidates.pop() {
        let mut next_clique = clique.clone();
        next_clique.push(vertex);
        maximal_cliques(
            graph,
            next_clique,
            candidates
                .iter()
                .copied()
                .filter(|&other| graph.has_edge(vertex, other))
                .collect(),
            excluded
                .iter()
                .copied()
                .filter(|&other| graph.has_edge(vertex, other))
                .collect(),
            output,
        );
        excluded.push(vertex);
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/minimum_intersection_graph_basis.rs"]
mod tests;
