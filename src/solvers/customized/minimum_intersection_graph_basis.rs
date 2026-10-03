//! Exact intersection-basis solver via maximal cliques and edge-cover branch and bound.

use crate::models::graph::{MinimumCoveringByCliques, MinimumIntersectionGraphBasis};
use crate::topology::{Graph, SimpleGraph};

pub(crate) fn solve(
    problem: &MinimumIntersectionGraphBasis<SimpleGraph>,
) -> Option<Vec<Vec<bool>>> {
    let cliques = minimum_covering_cliques(problem.graph());
    let mut solution = vec![vec![false; problem.num_edges()]; problem.num_vertices()];
    for (element, clique) in cliques.into_iter().enumerate() {
        for vertex in clique {
            solution[vertex][element] = true;
        }
    }
    Some(solution)
}

pub(crate) fn solve_cover(problem: &MinimumCoveringByCliques<SimpleGraph>) -> Vec<usize> {
    let cliques = minimum_covering_cliques(problem.graph());
    problem
        .graph()
        .edges()
        .iter()
        .map(|&(u, v)| {
            cliques
                .iter()
                .position(|clique| clique.contains(&u) && clique.contains(&v))
                .expect("the chosen cliques cover every edge")
        })
        .collect()
}

fn minimum_covering_cliques(graph: &SimpleGraph) -> Vec<Vec<usize>> {
    let edges = graph.edges();
    if edges.is_empty() {
        return Vec::new();
    }
    let mut cliques = Vec::new();
    maximal_cliques(
        graph,
        Vec::new(),
        (0..graph.num_vertices()).collect(),
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
    // Every edge, including a loop, lies in a nonempty maximal clique.
    let mut chosen = (0..edges.len())
        .map(|edge| {
            covers
                .iter()
                .position(|cover| cover[edge])
                .expect("every edge belongs to a maximal clique")
        })
        .collect::<Vec<_>>();
    chosen.sort_unstable();
    chosen.dedup();
    minimum_cover(
        &covers,
        &vec![false; edges.len()],
        &mut Vec::new(),
        &mut chosen,
    );
    chosen.into_iter().map(|i| cliques[i].clone()).collect()
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
        if !clique.is_empty() {
            output.push(clique);
        }
        return;
    }

    let pivot = candidates
        .iter()
        .chain(&excluded)
        .copied()
        .max_by_key(|&v| {
            candidates
                .iter()
                .filter(|&&u| u != v && graph.has_edge(u, v))
                .count()
        })
        .expect("a nonterminal clique has a candidate or excluded vertex");
    let branches: Vec<_> = candidates
        .iter()
        .copied()
        .filter(|&v| v == pivot || !graph.has_edge(v, pivot))
        .collect();
    for vertex in branches {
        candidates.retain(|&v| v != vertex);
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
