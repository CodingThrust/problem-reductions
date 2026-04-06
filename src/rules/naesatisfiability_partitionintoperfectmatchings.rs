//! Reduction from NAE-Satisfiability to Partition Into Perfect Matchings.
//!
//! This implements the Schaefer-style reduction for the `K = 2` case.
//! Clauses with two literals are normalized to three literals by duplicating
//! the first literal, and clauses with more than three literals are rejected.

use crate::models::formula::NAESatisfiability;
use crate::models::graph::PartitionIntoPerfectMatchings;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

#[derive(Debug, Clone, Copy)]
struct VariableVertices {
    t: usize,
    t_prime: usize,
    f: usize,
    f_prime: usize,
}

#[derive(Debug, Clone, Copy)]
struct SignalVertices {
    s: usize,
    s_prime: usize,
}

#[derive(Debug, Clone)]
struct ClauseLayout {
    literals: [i32; 3],
    signals: [SignalVertices; 3],
    clause_vertices: [usize; 4],
}

#[derive(Debug, Clone, Copy)]
struct ChainPairVertices {
    mu: usize,
    mu_prime: usize,
}

#[derive(Debug, Clone)]
struct ReductionLayout {
    variables: Vec<VariableVertices>,
    #[cfg(any(test, feature = "example-db"))]
    clauses: Vec<ClauseLayout>,
    #[cfg(any(test, feature = "example-db"))]
    positive_chains: Vec<Vec<ChainPairVertices>>,
    #[cfg(any(test, feature = "example-db"))]
    negative_chains: Vec<Vec<ChainPairVertices>>,
    num_vertices: usize,
    edges: Vec<(usize, usize)>,
}

/// Result of reducing NAE-Satisfiability to PartitionIntoPerfectMatchings.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToPartitionIntoPerfectMatchings {
    target: PartitionIntoPerfectMatchings<SimpleGraph>,
    layout: ReductionLayout,
}

impl ReductionResult for ReductionNAESATToPartitionIntoPerfectMatchings {
    type Source = NAESatisfiability;
    type Target = PartitionIntoPerfectMatchings<SimpleGraph>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        self.layout
            .variables
            .iter()
            .map(|variable| usize::from(target_solution[variable.t] == 0))
            .collect()
    }
}

impl ReductionNAESATToPartitionIntoPerfectMatchings {
    #[cfg(any(test, feature = "example-db"))]
    fn construct_target_solution(&self, source_solution: &[usize]) -> Vec<usize> {
        assert_eq!(
            source_solution.len(),
            self.layout.variables.len(),
            "source solution has {} variables but reduction expects {}",
            source_solution.len(),
            self.layout.variables.len()
        );

        let mut target_solution = vec![usize::MAX; self.layout.num_vertices];
        let mut true_groups = Vec::with_capacity(self.layout.variables.len());
        let mut false_groups = Vec::with_capacity(self.layout.variables.len());

        for (index, variable) in self.layout.variables.iter().enumerate() {
            let true_group = if source_solution[index] == 1 { 0 } else { 1 };
            let false_group = 1 - true_group;
            true_groups.push(true_group);
            false_groups.push(false_group);

            target_solution[variable.t] = true_group;
            target_solution[variable.t_prime] = true_group;
            target_solution[variable.f] = false_group;
            target_solution[variable.f_prime] = false_group;
        }

        for clause in &self.layout.clauses {
            for (signal, &literal) in clause.signals.iter().zip(clause.literals.iter()) {
                let variable_index = literal.unsigned_abs() as usize - 1;
                let signal_group = if literal > 0 {
                    true_groups[variable_index]
                } else {
                    false_groups[variable_index]
                };
                target_solution[signal.s] = signal_group;
                target_solution[signal.s_prime] = signal_group;
            }
        }

        for (variable_index, chain_pairs) in self.layout.positive_chains.iter().enumerate() {
            for pair in chain_pairs {
                target_solution[pair.mu] = false_groups[variable_index];
                target_solution[pair.mu_prime] = false_groups[variable_index];
            }
        }

        for (variable_index, chain_pairs) in self.layout.negative_chains.iter().enumerate() {
            for pair in chain_pairs {
                target_solution[pair.mu] = true_groups[variable_index];
                target_solution[pair.mu_prime] = true_groups[variable_index];
            }
        }

        for clause in &self.layout.clauses {
            let clause_groups = clause.signals.map(|signal| 1 - target_solution[signal.s]);
            let zero_count = clause_groups.iter().filter(|&&group| group == 0).count();
            let w3_group = match zero_count {
                1 => 0,
                2 => 1,
                _ => panic!("source assignment is not NAE-satisfying for normalized clauses"),
            };

            for (vertex, &group) in clause
                .clause_vertices
                .iter()
                .take(3)
                .zip(clause_groups.iter())
            {
                target_solution[*vertex] = group;
            }
            target_solution[clause.clause_vertices[3]] = w3_group;
        }

        assert!(
            target_solution.iter().all(|&group| group <= 1),
            "constructed target solution left some vertices unassigned"
        );

        target_solution
    }
}

fn normalize_clauses(problem: &NAESatisfiability) -> Vec<[i32; 3]> {
    problem
        .clauses()
        .iter()
        .map(|clause| match clause.literals.as_slice() {
            [a, b] => [*a, *a, *b],
            [a, b, c] => [*a, *b, *c],
            literals => panic!(
                "NAESatisfiability -> PartitionIntoPerfectMatchings expects clauses of size 2 or 3, got {}",
                literals.len()
            ),
        })
        .collect()
}

fn build_layout(problem: &NAESatisfiability) -> ReductionLayout {
    let num_vars = problem.num_vars();
    let clauses = normalize_clauses(problem);
    let num_clauses = clauses.len();

    let mut next_vertex = 0usize;
    let mut edges = Vec::with_capacity(3 * num_vars + 21 * num_clauses);
    let mut variables = Vec::with_capacity(num_vars);

    for _ in 0..num_vars {
        let variable = VariableVertices {
            t: next_vertex,
            t_prime: next_vertex + 1,
            f: next_vertex + 2,
            f_prime: next_vertex + 3,
        };
        next_vertex += 4;

        edges.push((variable.t, variable.t_prime));
        edges.push((variable.f, variable.f_prime));
        edges.push((variable.t, variable.f));
        variables.push(variable);
    }

    let mut clause_layouts = Vec::with_capacity(num_clauses);

    for &literals in &clauses {
        let mut signals = [SignalVertices { s: 0, s_prime: 0 }; 3];
        for (literal_index, _) in literals.iter().enumerate() {
            signals[literal_index] = SignalVertices {
                s: next_vertex,
                s_prime: next_vertex + 1,
            };
            next_vertex += 2;
            edges.push((signals[literal_index].s, signals[literal_index].s_prime));
        }

        clause_layouts.push(ClauseLayout {
            literals,
            signals,
            clause_vertices: [0; 4],
        });
    }

    for clause_layout in &mut clause_layouts {
        let clause_vertices = [
            next_vertex,
            next_vertex + 1,
            next_vertex + 2,
            next_vertex + 3,
        ];
        next_vertex += 4;

        for a in 0..4 {
            for b in (a + 1)..4 {
                edges.push((clause_vertices[a], clause_vertices[b]));
            }
        }
        for (literal_index, &clause_vertex) in clause_vertices.iter().enumerate().take(3) {
            edges.push((clause_layout.signals[literal_index].s, clause_vertex));
        }
        clause_layout.clause_vertices = clause_vertices;
    }

    let mut positive_occurrences: Vec<Vec<(usize, usize)>> = vec![Vec::new(); num_vars];
    let mut negative_occurrences: Vec<Vec<(usize, usize)>> = vec![Vec::new(); num_vars];
    for (clause_index, clause_layout) in clause_layouts.iter().enumerate() {
        for (literal_index, &literal) in clause_layout.literals.iter().enumerate() {
            let variable_index = literal.unsigned_abs() as usize - 1;
            if literal > 0 {
                positive_occurrences[variable_index].push((clause_index, literal_index));
            } else {
                negative_occurrences[variable_index].push((clause_index, literal_index));
            }
        }
    }

    let mut positive_chains: Vec<Vec<ChainPairVertices>> = vec![Vec::new(); num_vars];
    let mut negative_chains: Vec<Vec<ChainPairVertices>> = vec![Vec::new(); num_vars];

    for variable_index in 0..num_vars {
        let mut source_vertex = variables[variable_index].t;
        for &(clause_index, literal_index) in &positive_occurrences[variable_index] {
            let pair = ChainPairVertices {
                mu: next_vertex,
                mu_prime: next_vertex + 1,
            };
            next_vertex += 2;

            let signal_vertex = clause_layouts[clause_index].signals[literal_index].s;
            edges.push((pair.mu, pair.mu_prime));
            edges.push((source_vertex, pair.mu));
            edges.push((signal_vertex, pair.mu));
            positive_chains[variable_index].push(pair);
            source_vertex = signal_vertex;
        }

        let mut source_vertex = variables[variable_index].f;
        for &(clause_index, literal_index) in &negative_occurrences[variable_index] {
            let pair = ChainPairVertices {
                mu: next_vertex,
                mu_prime: next_vertex + 1,
            };
            next_vertex += 2;

            let signal_vertex = clause_layouts[clause_index].signals[literal_index].s;
            edges.push((pair.mu, pair.mu_prime));
            edges.push((source_vertex, pair.mu));
            edges.push((signal_vertex, pair.mu));
            negative_chains[variable_index].push(pair);
            source_vertex = signal_vertex;
        }
    }

    ReductionLayout {
        variables,
        #[cfg(any(test, feature = "example-db"))]
        clauses: clause_layouts,
        #[cfg(any(test, feature = "example-db"))]
        positive_chains,
        #[cfg(any(test, feature = "example-db"))]
        negative_chains,
        num_vertices: next_vertex,
        edges,
    }
}

#[reduction(
    overhead = {
        num_vertices = "4 * num_vars + 16 * num_clauses",
        num_edges = "3 * num_vars + 21 * num_clauses",
        num_matchings = "2",
    }
)]
impl ReduceTo<PartitionIntoPerfectMatchings<SimpleGraph>> for NAESatisfiability {
    type Result = ReductionNAESATToPartitionIntoPerfectMatchings;

    fn reduce_to(&self) -> Self::Result {
        let layout = build_layout(self);
        let target = PartitionIntoPerfectMatchings::new(
            SimpleGraph::new(layout.num_vertices, layout.edges.clone()),
            2,
        );

        ReductionNAESATToPartitionIntoPerfectMatchings { target, layout }
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_partitionintoperfectmatchings",
        build: || {
            let source = NAESatisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, 3]),
                    CNFClause::new(vec![-1, 2, -3]),
                ],
            );
            let source_config = vec![1, 1, 0];
            let reduction =
                ReduceTo::<PartitionIntoPerfectMatchings<SimpleGraph>>::reduce_to(&source);
            let target_config = reduction.construct_target_solution(&source_config);

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
#[path = "../unit_tests/rules/naesatisfiability_partitionintoperfectmatchings.rs"]
mod tests;
