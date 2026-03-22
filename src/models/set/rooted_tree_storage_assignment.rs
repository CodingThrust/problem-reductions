//! Rooted Tree Storage Assignment problem implementation.

use crate::registry::{FieldInfo, ProblemSchemaEntry};
use crate::traits::{Problem, SatisfactionProblem};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

inventory::submit! {
    ProblemSchemaEntry {
        name: "RootedTreeStorageAssignment",
        display_name: "Rooted Tree Storage Assignment",
        aliases: &[],
        dimensions: &[],
        module_path: module_path!(),
        description: "Does there exist a rooted tree whose subset path extensions cost at most K?",
        fields: &[
            FieldInfo { name: "universe_size", type_name: "usize", description: "Size of the ground set X" },
            FieldInfo { name: "subsets", type_name: "Vec<Vec<usize>>", description: "Collection of subsets of X" },
            FieldInfo { name: "bound", type_name: "usize", description: "Upper bound K on the total extension cost" },
        ],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "RootedTreeStorageAssignmentDef")]
pub struct RootedTreeStorageAssignment {
    universe_size: usize,
    subsets: Vec<Vec<usize>>,
    bound: usize,
}

#[derive(Debug, Deserialize)]
struct RootedTreeStorageAssignmentDef {
    universe_size: usize,
    subsets: Vec<Vec<usize>>,
    bound: usize,
}

impl RootedTreeStorageAssignment {
    pub fn new(universe_size: usize, subsets: Vec<Vec<usize>>, bound: usize) -> Self {
        Self::try_new(universe_size, subsets, bound).unwrap_or_else(|err| panic!("{err}"))
    }

    pub fn try_new(
        universe_size: usize,
        subsets: Vec<Vec<usize>>,
        bound: usize,
    ) -> Result<Self, String> {
        let subsets = subsets
            .into_iter()
            .enumerate()
            .map(|(subset_index, mut subset)| {
                let mut seen = HashSet::with_capacity(subset.len());
                for &element in &subset {
                    if element >= universe_size {
                        return Err(format!(
                            "subset {subset_index} contains element {element} outside universe of size {universe_size}"
                        ));
                    }
                    if !seen.insert(element) {
                        return Err(format!(
                            "subset {subset_index} contains duplicate element {element}"
                        ));
                    }
                }
                subset.sort_unstable();
                Ok(subset)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            universe_size,
            subsets,
            bound,
        })
    }

    pub fn universe_size(&self) -> usize {
        self.universe_size
    }

    pub fn num_subsets(&self) -> usize {
        self.subsets.len()
    }

    pub fn subsets(&self) -> &[Vec<usize>] {
        &self.subsets
    }

    pub fn bound(&self) -> usize {
        self.bound
    }

    fn analyze_tree(config: &[usize]) -> Option<Vec<usize>> {
        let roots = config
            .iter()
            .enumerate()
            .filter(|(vertex, parent)| *vertex == **parent)
            .count();
        if roots != 1 {
            return None;
        }

        let n = config.len();
        let mut state = vec![0u8; n];
        let mut depth = vec![0usize; n];

        fn visit(vertex: usize, config: &[usize], state: &mut [u8], depth: &mut [usize]) -> bool {
            match state[vertex] {
                2 => return true,
                1 => return false,
                _ => {}
            }

            state[vertex] = 1;
            let parent = config[vertex];
            if parent == vertex {
                depth[vertex] = 0;
            } else {
                if !visit(parent, config, state, depth) {
                    return false;
                }
                depth[vertex] = depth[parent] + 1;
            }
            state[vertex] = 2;
            true
        }

        for vertex in 0..n {
            if !visit(vertex, config, &mut state, &mut depth) {
                return None;
            }
        }

        Some(depth)
    }

    fn is_ancestor(ancestor: usize, mut vertex: usize, config: &[usize], depth: &[usize]) -> bool {
        if depth[ancestor] > depth[vertex] {
            return false;
        }

        while depth[vertex] > depth[ancestor] {
            vertex = config[vertex];
        }

        ancestor == vertex
    }

    fn subset_extension_cost(
        &self,
        subset: &[usize],
        config: &[usize],
        depth: &[usize],
    ) -> Option<usize> {
        if subset.len() <= 1 {
            return Some(0);
        }

        let mut ordered = subset.to_vec();
        ordered.sort_by_key(|&vertex| depth[vertex]);

        for pair in ordered.windows(2) {
            if !Self::is_ancestor(pair[0], pair[1], config, depth) {
                return None;
            }
        }

        let top = ordered[0];
        let bottom = *ordered.last().unwrap();
        Some(depth[bottom] - depth[top] + 1 - ordered.len())
    }
}

impl Problem for RootedTreeStorageAssignment {
    const NAME: &'static str = "RootedTreeStorageAssignment";
    type Metric = bool;

    fn dims(&self) -> Vec<usize> {
        vec![self.universe_size; self.universe_size]
    }

    fn evaluate(&self, config: &[usize]) -> bool {
        if config.len() != self.universe_size {
            return false;
        }
        if config.iter().any(|&parent| parent >= self.universe_size) {
            return false;
        }
        if self.universe_size == 0 {
            return self.subsets.is_empty();
        }

        let Some(depth) = Self::analyze_tree(config) else {
            return false;
        };

        let mut total_cost = 0usize;
        for subset in &self.subsets {
            let Some(cost) = self.subset_extension_cost(subset, config, &depth) else {
                return false;
            };
            total_cost += cost;
            if total_cost > self.bound {
                return false;
            }
        }

        true
    }

    fn variant() -> Vec<(&'static str, &'static str)> {
        crate::variant_params![]
    }
}

impl SatisfactionProblem for RootedTreeStorageAssignment {}

crate::declare_variants! {
    default sat RootedTreeStorageAssignment => "universe_size^universe_size",
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_model_example_specs() -> Vec<crate::example_db::specs::ModelExampleSpec> {
    vec![crate::example_db::specs::ModelExampleSpec {
        id: "rooted_tree_storage_assignment",
        instance: Box::new(RootedTreeStorageAssignment::new(
            5,
            vec![vec![0, 2], vec![1, 3], vec![0, 4], vec![2, 4]],
            1,
        )),
        optimal_config: vec![0, 0, 0, 1, 2],
        optimal_value: serde_json::json!(true),
    }]
}

impl TryFrom<RootedTreeStorageAssignmentDef> for RootedTreeStorageAssignment {
    type Error = String;

    fn try_from(value: RootedTreeStorageAssignmentDef) -> Result<Self, Self::Error> {
        Self::try_new(value.universe_size, value.subsets, value.bound)
    }
}

#[cfg(test)]
#[path = "../../unit_tests/models/set/rooted_tree_storage_assignment.rs"]
mod tests;
