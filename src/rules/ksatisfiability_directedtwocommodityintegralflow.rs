//! Reduction from KSatisfiability (3-SAT) to DirectedTwoCommodityIntegralFlow.
//!
//! This uses a padded occurrence-lobe variant of the Even-Itai-Shamir
//! construction: each variable branch begins with one dummy segment, then one
//! segment per literal occurrence of that polarity. Commodity 1 chooses exactly
//! one branch per variable. Commodity 2 must enter a literal-occurrence segment
//! from `s_2`, traverse that segment's internal arc, and then exit to a clause
//! vertex before reaching `t_2`.

use crate::models::formula::KSatisfiability;
use crate::models::graph::DirectedTwoCommodityIntegralFlow;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::DirectedGraph;
use crate::variant::K3;

#[derive(Debug, Clone, Copy)]
struct ClauseOccurrence {
    clause_idx: usize,
    variable: usize,
    requires_true: bool,
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct VariablePaths {
    upper_path: Vec<usize>,
    lower_path: Vec<usize>,
    lower_entry_arc: usize,
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
struct ClauseRoute {
    variable: usize,
    requires_true: bool,
    source_arc: usize,
    branch_arc: usize,
    clause_arc: usize,
}

#[derive(Debug, Clone)]
struct BranchBuild {
    path_arcs: Vec<usize>,
    entry_arc: usize,
}

struct BranchContext<'a> {
    source_2: usize,
    clause_vertices: &'a [usize],
    clause_routes: &'a mut [Vec<ClauseRoute>],
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
#[derive(Debug, Clone)]
pub struct Reduction3SATToDirectedTwoCommodityIntegralFlow {
    target: DirectedTwoCommodityIntegralFlow,
    commodity_1_chain_arcs: Vec<usize>,
    variable_paths: Vec<VariablePaths>,
    clause_routes: Vec<Vec<ClauseRoute>>,
    clause_sink_arcs: Vec<usize>,
}

fn literal_var_index(literal: i32) -> usize {
    literal.unsigned_abs() as usize - 1
}

#[cfg_attr(not(any(test, feature = "example-db")), allow(dead_code))]
fn literal_satisfied(requires_true: bool, assignment: &[usize], variable: usize) -> bool {
    assignment.get(variable).copied().unwrap_or(0) == usize::from(requires_true)
}

fn build_branch<FV, FA>(
    add_vertex: &mut FV,
    add_arc: &mut FA,
    entry: usize,
    exit: usize,
    occurrences: &[ClauseOccurrence],
    branch_context: &mut BranchContext<'_>,
) -> BranchBuild
where
    FV: FnMut() -> usize,
    FA: FnMut(usize, usize) -> usize,
{
    let mut path_arcs = Vec::with_capacity(2 * occurrences.len() + 3);

    let dummy_odd = add_vertex();
    let dummy_even = add_vertex();
    let entry_arc = add_arc(entry, dummy_odd);
    path_arcs.push(entry_arc);
    path_arcs.push(add_arc(dummy_odd, dummy_even));

    let mut previous_even = dummy_even;

    for occurrence in occurrences {
        let odd = add_vertex();
        let even = add_vertex();
        path_arcs.push(add_arc(previous_even, odd));
        let branch_arc = add_arc(odd, even);
        path_arcs.push(branch_arc);

        let source_arc = add_arc(branch_context.source_2, odd);
        let clause_arc = add_arc(even, branch_context.clause_vertices[occurrence.clause_idx]);
        branch_context.clause_routes[occurrence.clause_idx].push(ClauseRoute {
            variable: occurrence.variable,
            requires_true: occurrence.requires_true,
            source_arc,
            branch_arc,
            clause_arc,
        });

        previous_even = even;
    }

    path_arcs.push(add_arc(previous_even, exit));

    BranchBuild {
        path_arcs,
        entry_arc,
    }
}

impl Reduction3SATToDirectedTwoCommodityIntegralFlow {
    #[cfg(any(test, feature = "example-db"))]
    pub(crate) fn encode_assignment(&self, assignment: &[usize]) -> Vec<usize> {
        assert_eq!(
            assignment.len(),
            self.variable_paths.len(),
            "assignment length must match num_vars",
        );

        let num_arcs = self.target.num_arcs();
        let mut flow = vec![0usize; 2 * num_arcs];

        for &arc_idx in &self.commodity_1_chain_arcs {
            flow[arc_idx] = 1;
        }

        for (value, paths) in assignment.iter().zip(&self.variable_paths) {
            let chosen_path = if *value == 1 {
                &paths.lower_path
            } else {
                &paths.upper_path
            };
            for &arc_idx in chosen_path {
                flow[arc_idx] = 1;
            }
        }

        for (clause_idx, routes) in self.clause_routes.iter().enumerate() {
            if let Some(route) = routes
                .iter()
                .find(|route| literal_satisfied(route.requires_true, assignment, route.variable))
            {
                flow[num_arcs + route.source_arc] = 1;
                flow[num_arcs + route.branch_arc] = 1;
                flow[num_arcs + route.clause_arc] = 1;
                flow[num_arcs + self.clause_sink_arcs[clause_idx]] = 1;
            }
        }

        flow
    }
}

impl ReductionResult for Reduction3SATToDirectedTwoCommodityIntegralFlow {
    type Source = KSatisfiability<K3>;
    type Target = DirectedTwoCommodityIntegralFlow;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.variable_paths
            .iter()
            .map(|paths| {
                usize::from(
                    target_solution
                        .get(paths.lower_entry_arc)
                        .copied()
                        .unwrap_or(0)
                        > 0,
                )
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_vertices = "6 * num_vars + 2 * num_literals + num_clauses + 4",
    num_arcs = "7 * num_vars + 4 * num_literals + num_clauses + 1",
})]
impl ReduceTo<DirectedTwoCommodityIntegralFlow> for KSatisfiability<K3> {
    type Result = Reduction3SATToDirectedTwoCommodityIntegralFlow;

    fn reduce_to(&self) -> Self::Result {
        let source_1 = 0usize;
        let sink_1 = 1usize;
        let source_2 = 2usize;
        let sink_2 = 3usize;

        let mut positive_occurrences = vec![Vec::<ClauseOccurrence>::new(); self.num_vars()];
        let mut negative_occurrences = vec![Vec::<ClauseOccurrence>::new(); self.num_vars()];
        for (clause_idx, clause) in self.clauses().iter().enumerate() {
            for &literal in &clause.literals {
                let variable = literal_var_index(literal);
                let occurrence = ClauseOccurrence {
                    clause_idx,
                    variable,
                    requires_true: literal > 0,
                };
                if literal > 0 {
                    positive_occurrences[variable].push(occurrence);
                } else {
                    negative_occurrences[variable].push(occurrence);
                }
            }
        }

        let mut next_vertex = 4 + self.num_clauses();
        let clause_vertices: Vec<usize> = (0..self.num_clauses()).map(|idx| 4 + idx).collect();
        let mut add_vertex = || {
            let id = next_vertex;
            next_vertex += 1;
            id
        };

        let mut arcs = Vec::<(usize, usize)>::new();
        let mut add_arc = |u: usize, v: usize| {
            arcs.push((u, v));
            arcs.len() - 1
        };

        let mut entries = Vec::with_capacity(self.num_vars());
        let mut exits = Vec::with_capacity(self.num_vars());
        let mut variable_paths = Vec::with_capacity(self.num_vars());
        let mut clause_routes = vec![Vec::<ClauseRoute>::new(); self.num_clauses()];
        let mut branch_context = BranchContext {
            source_2,
            clause_vertices: &clause_vertices,
            clause_routes: &mut clause_routes,
        };

        for variable in 0..self.num_vars() {
            let entry = add_vertex();
            let exit = add_vertex();
            entries.push(entry);
            exits.push(exit);

            let upper = build_branch(
                &mut add_vertex,
                &mut add_arc,
                entry,
                exit,
                &positive_occurrences[variable],
                &mut branch_context,
            );
            let lower = build_branch(
                &mut add_vertex,
                &mut add_arc,
                entry,
                exit,
                &negative_occurrences[variable],
                &mut branch_context,
            );

            variable_paths.push(VariablePaths {
                upper_path: upper.path_arcs,
                lower_path: lower.path_arcs,
                lower_entry_arc: lower.entry_arc,
            });
        }

        let mut commodity_1_chain_arcs = Vec::with_capacity(self.num_vars() + 1);
        if self.num_vars() == 0 {
            commodity_1_chain_arcs.push(add_arc(source_1, sink_1));
        } else {
            commodity_1_chain_arcs.push(add_arc(source_1, entries[0]));
            for variable in 0..self.num_vars() - 1 {
                commodity_1_chain_arcs.push(add_arc(exits[variable], entries[variable + 1]));
            }
            commodity_1_chain_arcs.push(add_arc(exits[self.num_vars() - 1], sink_1));
        }

        let clause_sink_arcs: Vec<usize> = clause_vertices
            .iter()
            .map(|&clause_vertex| add_arc(clause_vertex, sink_2))
            .collect();

        let capacities = vec![1u64; arcs.len()];
        let target = DirectedTwoCommodityIntegralFlow::new(
            DirectedGraph::new(next_vertex, arcs),
            capacities,
            source_1,
            sink_1,
            source_2,
            sink_2,
            1,
            self.num_clauses() as u64,
        );

        Reduction3SATToDirectedTwoCommodityIntegralFlow {
            target,
            commodity_1_chain_arcs,
            variable_paths,
            clause_routes,
            clause_sink_arcs,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_directedtwocommodityintegralflow",
        build: || {
            let source = KSatisfiability::<K3>::new(
                3,
                vec![
                    crate::models::formula::CNFClause::new(vec![1, -2, 3]),
                    crate::models::formula::CNFClause::new(vec![-1, 2, -3]),
                ],
            );
            let reduction =
                crate::rules::ReduceTo::<DirectedTwoCommodityIntegralFlow>::reduce_to(&source);
            let source_config = vec![1, 1, 0];
            let target_config = reduction.encode_assignment(&source_config);

            crate::example_db::specs::assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config,
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_directedtwocommodityintegralflow.rs"]
mod tests;
