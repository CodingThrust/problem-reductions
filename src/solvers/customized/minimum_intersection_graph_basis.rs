//! Exact intersection-basis solver via maximal cliques and edge-cover branch and bound.

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
                .map(|&(u, v)| clique.contains(&u) && clique.contains(&v))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // One maximal clique per edge is a feasible initial cover with at most |E| cliques.
    let mut chosen = (0..edges.len())
        .map(|edge| covers.iter().position(|cover| cover[edge]).unwrap())
        .collect::<Vec<_>>();
    chosen.sort_unstable();
    chosen.dedup();
    minimum_cover(
        &covers,
        &vec![false; edges.len()],
        &mut Vec::new(),
        &mut chosen,
    );
    let mut solution = vec![vec![false; edges.len()]; n];
    for (element, clique) in chosen.into_iter().enumerate() {
        for &vertex in &cliques[clique] {
            solution[vertex][element] = true;
        }
    }
    Some(solution)
}

fn minimum_cover(
    covers: &[Vec<bool>],
    covered: &[bool],
    selected: &mut Vec<usize>,
    best: &mut Vec<usize>,
) {
    let Some(edge) = covered.iter().position(|&covered| !covered) else {
        if selected.len() < best.len() {
            best.clone_from(selected);
        }
        return;
    };
    if selected.len() + 1 >= best.len() {
        return;
    }
    for (clique, cover) in covers.iter().enumerate().filter(|(_, cover)| cover[edge]) {
        let next = covered
            .iter()
            .zip(cover)
            .map(|(&left, &right)| left || right)
            .collect::<Vec<_>>();
        selected.push(clique);
        minimum_cover(covers, &next, selected, best);
        selected.pop();
    }
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
