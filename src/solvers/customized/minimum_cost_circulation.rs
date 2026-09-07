//! Exact minimum-cost circulation solver using negative-cycle cancellation.

use crate::models::graph::MinimumCostCirculation;
use crate::solvers::SolveError;

struct ResidualArc {
    from: usize,
    to: usize,
    cost: i64,
    capacity: i64,
    original: usize,
    forward: bool,
}

pub(crate) fn solve(problem: &MinimumCostCirculation) -> Result<Vec<usize>, SolveError> {
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
                    cost: problem.costs()[arc].checked_neg().ok_or_else(|| {
                        SolveError::IntegerOverflow("negating a circulation residual cost".into())
                    })?,
                    capacity: flow[arc],
                    original: arc,
                    forward: false,
                });
            }
        }

        let Some(cycle) = negative_cycle(problem.num_vertices(), &residual)? else {
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

    Ok(flow.into_iter().map(|value| value as usize).collect())
}

fn negative_cycle(
    num_vertices: usize,
    edges: &[ResidualArc],
) -> Result<Option<Vec<usize>>, SolveError> {
    let mut distance = vec![0_i64; num_vertices];
    let mut predecessor = vec![None; num_vertices];
    let mut changed = None;

    for _ in 0..num_vertices {
        changed = None;
        for (index, edge) in edges.iter().enumerate() {
            let candidate = distance[edge.from].checked_add(edge.cost).ok_or_else(|| {
                SolveError::IntegerOverflow("relaxing a circulation residual arc".into())
            })?;
            if distance[edge.to] > candidate {
                distance[edge.to] = candidate;
                predecessor[edge.to] = Some(index);
                changed = Some(edge.to);
            }
        }
        if changed.is_none() {
            return Ok(None);
        }
    }

    let Some(mut vertex) = changed else {
        return Ok(None);
    };
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
            return Ok(Some(cycle));
        }
    }
}

#[cfg(test)]
#[path = "../../unit_tests/solvers/customized/minimum_cost_circulation.rs"]
mod tests;
