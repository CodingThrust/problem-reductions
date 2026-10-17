//! Deterministic bounded Yen search shared by the registry and browser.
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn shortest_path(
    source: usize,
    target: usize,
    adjacency: &[Vec<usize>],
    excluded_nodes: &HashSet<usize>,
    excluded_edges: &HashSet<(usize, usize)>,
    max_nodes: usize,
) -> Option<Vec<usize>> {
    if excluded_nodes.contains(&source) || max_nodes == 0 {
        return None;
    }
    if source == target {
        return Some(vec![source]);
    }

    let mut queue = VecDeque::from([(source, 1usize)]);
    let mut parents = HashMap::new();
    let mut visited = HashSet::from([source]);

    while let Some((current, path_nodes)) = queue.pop_front() {
        if path_nodes == max_nodes {
            continue;
        }
        for &next in &adjacency[current] {
            if excluded_nodes.contains(&next)
                || excluded_edges.contains(&(current, next))
                || !visited.insert(next)
            {
                continue;
            }
            parents.insert(next, current);
            if next == target {
                let mut path = vec![target];
                let mut node = target;
                while node != source {
                    node = parents[&node];
                    path.push(node);
                }
                path.reverse();
                return Some(path);
            }
            queue.push_back((next, path_nodes + 1));
        }
    }
    None
}

pub(crate) fn find_paths(
    adjacency: &[Vec<usize>],
    order: &[usize],
    source: usize,
    target: usize,
    limit: usize,
    max_nodes: usize,
) -> Vec<Vec<usize>> {
    if source == target || limit == 0 {
        return Vec::new();
    }
    let Some(first) = shortest_path(
        source,
        target,
        adjacency,
        &HashSet::new(),
        &HashSet::new(),
        max_nodes,
    ) else {
        return Vec::new();
    };

    let mut accepted = vec![first];
    let mut candidates = BTreeSet::new();

    while accepted.len() < limit {
        let previous = accepted.last().expect("accepted path exists");
        for spur_index in 0..previous.len().saturating_sub(1) {
            let root = &previous[..=spur_index];
            let excluded_edges = accepted
                .iter()
                .filter(|path| path.len() > spur_index + 1 && path[..=spur_index] == *root)
                .map(|path| (path[spur_index], path[spur_index + 1]))
                .collect::<HashSet<_>>();
            let excluded_nodes = root[..spur_index].iter().copied().collect();
            let max_spur_nodes = max_nodes.saturating_sub(spur_index);
            let Some(spur) = shortest_path(
                previous[spur_index],
                target,
                adjacency,
                &excluded_nodes,
                &excluded_edges,
                max_spur_nodes,
            ) else {
                continue;
            };
            let mut candidate = root[..spur_index].to_vec();
            candidate.extend(spur);
            if !accepted.contains(&candidate) {
                let key = (
                    candidate.len(),
                    candidate
                        .iter()
                        .map(|&node| order[node])
                        .collect::<Vec<_>>(),
                );
                candidates.insert((key, candidate));
            }
        }

        let Some((_, next)) = candidates.pop_first() else {
            break;
        };
        accepted.push(next);
    }

    accepted
}
