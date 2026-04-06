//! Reduction from Satisfiability to IntegralFlowHomologousArcs.
//!
//! Follows the clause-stage flow construction described by Sahni (1974):
//! one unit of flow per variable chooses either the T or F channel, and each
//! clause stage routes the channels corresponding to literals of the negated
//! clause through a shared bottleneck. Homologous entry/exit pairs force each
//! variable's bottleneck flow to stay on its own channel.

use crate::models::formula::Satisfiability;
use crate::models::graph::IntegralFlowHomologousArcs;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::DirectedGraph;

#[derive(Debug, Clone)]
struct VariablePaths {
    true_path: Vec<usize>,
    false_path: Vec<usize>,
    true_base_arc: usize,
}

#[derive(Debug, Clone, Copy)]
struct NodeIndexer {
    num_vars: usize,
    num_clauses: usize,
}

impl NodeIndexer {
    fn source(self) -> usize {
        0
    }

    fn sink(self) -> usize {
        1
    }

    fn split(self, variable: usize) -> usize {
        2 + variable
    }

    fn boundary_base(self) -> usize {
        2 + self.num_vars
    }

    fn channel(self, boundary: usize, variable: usize, is_true: bool) -> usize {
        self.boundary_base() + (boundary * self.num_vars + variable) * 2 + usize::from(!is_true)
    }

    fn clause_base(self) -> usize {
        self.boundary_base() + (self.num_clauses + 1) * self.num_vars * 2
    }

    fn collector(self, clause: usize) -> usize {
        self.clause_base() + clause * 2
    }

    fn distributor(self, clause: usize) -> usize {
        self.collector(clause) + 1
    }

    fn total_vertices(self) -> usize {
        self.clause_base() + self.num_clauses * 2
    }
}

/// Result of reducing SAT to integral flow with homologous arcs.
#[derive(Debug, Clone)]
pub struct ReductionSATToIntegralFlowHomologousArcs {
    target: IntegralFlowHomologousArcs,
    variable_paths: Vec<VariablePaths>,
}

impl ReductionSATToIntegralFlowHomologousArcs {
    #[cfg(any(test, feature = "example-db"))]
    fn encode_assignment(&self, assignment: &[usize]) -> Vec<usize> {
        assert_eq!(
            assignment.len(),
            self.variable_paths.len(),
            "assignment length must match num_vars",
        );

        let mut flow = vec![0usize; self.target.num_arcs()];
        for (value, paths) in assignment.iter().zip(&self.variable_paths) {
            let path = if *value == 0 {
                &paths.false_path
            } else {
                &paths.true_path
            };
            for &arc_idx in path {
                flow[arc_idx] += 1;
            }
        }
        flow
    }
}

impl ReductionResult for ReductionSATToIntegralFlowHomologousArcs {
    type Source = Satisfiability;
    type Target = IntegralFlowHomologousArcs;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.variable_paths
            .iter()
            .map(|paths| {
                usize::from(
                    target_solution
                        .get(paths.true_base_arc)
                        .copied()
                        .unwrap_or(0)
                        > 0,
                )
            })
            .collect()
    }
}

#[reduction(overhead = {
    num_vertices = "2 * num_vars * num_clauses + 3 * num_vars + 2 * num_clauses + 2",
    num_arcs = "2 * num_vars * num_clauses + 5 * num_vars + num_clauses + num_literals",
})]
impl ReduceTo<IntegralFlowHomologousArcs> for Satisfiability {
    type Result = ReductionSATToIntegralFlowHomologousArcs;

    fn reduce_to(&self) -> Self::Result {
        let indexer = NodeIndexer {
            num_vars: self.num_vars(),
            num_clauses: self.num_clauses(),
        };

        let mut arcs = Vec::<(usize, usize)>::new();
        let mut capacities = Vec::<u64>::new();
        let mut homologous_pairs = Vec::<(usize, usize)>::new();
        let mut variable_paths = Vec::<VariablePaths>::with_capacity(self.num_vars());

        let mut add_arc = |u: usize, v: usize, capacity: u64| -> usize {
            arcs.push((u, v));
            capacities.push(capacity);
            arcs.len() - 1
        };

        for variable in 0..self.num_vars() {
            let source_arc = add_arc(indexer.source(), indexer.split(variable), 1);
            let true_base_arc = add_arc(
                indexer.split(variable),
                indexer.channel(0, variable, true),
                1,
            );
            let false_base_arc = add_arc(
                indexer.split(variable),
                indexer.channel(0, variable, false),
                1,
            );

            variable_paths.push(VariablePaths {
                true_path: vec![source_arc, true_base_arc],
                false_path: vec![source_arc, false_base_arc],
                true_base_arc,
            });
        }

        for (clause_idx, clause) in self.clauses().iter().enumerate() {
            let collector = indexer.collector(clause_idx);
            let distributor = indexer.distributor(clause_idx);
            let bottleneck = add_arc(
                collector,
                distributor,
                clause.literals.len().saturating_sub(1) as u64,
            );

            let mut has_positive = vec![false; self.num_vars()];
            let mut has_negative = vec![false; self.num_vars()];
            for &literal in &clause.literals {
                let variable = literal.unsigned_abs() as usize - 1;
                if literal > 0 {
                    has_positive[variable] = true;
                } else {
                    has_negative[variable] = true;
                }
            }

            for variable in 0..self.num_vars() {
                let prev_true = indexer.channel(clause_idx, variable, true);
                let prev_false = indexer.channel(clause_idx, variable, false);
                let next_true = indexer.channel(clause_idx + 1, variable, true);
                let next_false = indexer.channel(clause_idx + 1, variable, false);

                if has_negative[variable] {
                    let entry = add_arc(prev_true, collector, 1);
                    let exit = add_arc(distributor, next_true, 1);
                    homologous_pairs.push((entry, exit));
                    variable_paths[variable]
                        .true_path
                        .extend([entry, bottleneck, exit]);
                } else {
                    let bypass = add_arc(prev_true, next_true, 1);
                    variable_paths[variable].true_path.push(bypass);
                }

                if has_positive[variable] {
                    let entry = add_arc(prev_false, collector, 1);
                    let exit = add_arc(distributor, next_false, 1);
                    homologous_pairs.push((entry, exit));
                    variable_paths[variable]
                        .false_path
                        .extend([entry, bottleneck, exit]);
                } else {
                    let bypass = add_arc(prev_false, next_false, 1);
                    variable_paths[variable].false_path.push(bypass);
                }
            }
        }

        for (variable, paths) in variable_paths.iter_mut().enumerate() {
            let true_sink = add_arc(
                indexer.channel(self.num_clauses(), variable, true),
                indexer.sink(),
                1,
            );
            let false_sink = add_arc(
                indexer.channel(self.num_clauses(), variable, false),
                indexer.sink(),
                1,
            );
            paths.true_path.push(true_sink);
            paths.false_path.push(false_sink);
        }

        let mut requirement = self.num_vars() as u64;
        if self.clauses().iter().any(|clause| clause.is_empty()) {
            requirement += 1;
        }

        ReductionSATToIntegralFlowHomologousArcs {
            target: IntegralFlowHomologousArcs::new(
                DirectedGraph::new(indexer.total_vertices(), arcs),
                capacities,
                indexer.source(),
                indexer.sink(),
                requirement,
                homologous_pairs,
            ),
            variable_paths,
        }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    fn issue_example() -> Satisfiability {
        Satisfiability::new(
            3,
            vec![
                CNFClause::new(vec![1, 2]),
                CNFClause::new(vec![-1, 3]),
                CNFClause::new(vec![-2, -3]),
                CNFClause::new(vec![1, 3]),
            ],
        )
    }

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "satisfiability_to_integralflowhomologousarcs",
        build: || {
            let source = issue_example();
            let source_config = vec![1, 0, 1];
            let target_config = ReduceTo::<IntegralFlowHomologousArcs>::reduce_to(&source)
                .encode_assignment(&source_config);
            crate::example_db::specs::rule_example_with_witness::<_, IntegralFlowHomologousArcs>(
                source,
                SolutionPair {
                    source_config,
                    target_config,
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/satisfiability_integralflowhomologousarcs.rs"]
mod tests;
