//! Exact minimum-cost circulation solver using negative-cycle cancellation.

use crate::models::graph::MinimumCostCirculation;

struct ResidualArc {
    from: usize,
    to: usize,
    cost: i64,
    capacity: i64,
    original: usize,
    forward: bool,
}

pub(crate) fn solve(problem: &MinimumCostCirculation) -> Option<Vec<usize>> {
    let mut flow = vec![0_i64; problem.num_arcs()];

    loop {
        let mut residual = Vec::new();
        for (arc, &(from, to)) in problem.graph().arcs().iter().enumerate() {
            if flow[arc] < problem.capacities()[arc] {
                residual.push(ResidualArc {
                    from,
                    to,
                    cost: problem.costs()[arc],
                    capacity: problem.capacities()[arc] - flow[arc],
                    original: arc,
                    forward: true,
                });
            }
            if flow[arc] > 0 {
                residual.push(ResidualArc {
                    from: to,
                    to: from,
                    cost: -problem.costs()[arc],
                    capacity: flow[arc],
                    original: arc,
                    forward: false,
                });
            }
        }

        let Some(cycle) = negative_cycle(problem.num_vertices(), &residual) else {
            break;
        };
        let amount = cycle
            .iter()
            .map(|&edge| residual[edge].capacity)
            .min()
            .unwrap();
        for edge in cycle {
            let edge = &residual[edge];
            if edge.forward {
                flow[edge.original] += amount;
            } else {
                flow[edge.original] -= amount;
            }
        }
    }

    Some(flow.into_iter().map(|value| value as usize).collect())
}

fn negative_cycle(num_vertices: usize, edges: &[ResidualArc]) -> Option<Vec<usize>> {
    let mut distance = vec![0_i64; num_vertices];
    let mut predecessor = vec![None; num_vertices];
    let mut changed = None;

    for _ in 0..num_vertices {
        changed = None;
        for (index, edge) in edges.iter().enumerate() {
            if distance[edge.to] > distance[edge.from] + edge.cost {
                distance[edge.to] = distance[edge.from] + edge.cost;
                predecessor[edge.to] = Some(index);
                changed = Some(edge.to);
            }
        }
    }

    let mut vertex = changed?;
    for _ in 0..num_vertices {
        vertex = edges[predecessor[vertex].unwrap()].from;
    }
    let start = vertex;
    let mut cycle = Vec::new();
    loop {
        let edge = predecessor[vertex].unwrap();
        cycle.push(edge);
        vertex = edges[edge].from;
        if vertex == start {
            return Some(cycle);
        }
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/minimum_cost_circulation.rs"]
mod tests;
