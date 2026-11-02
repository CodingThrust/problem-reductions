//! Pairwise one-hot CNF encoding of runtime graph coloring.

use crate::models::formula::{CNFClause, Satisfiability};
use crate::models::graph::KColoring;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::{Graph, SimpleGraph};
use crate::variant::KN;

/// A SAT formula whose satisfying assignments encode proper colorings.
#[derive(Debug, Clone)]
pub struct ReductionKColoringToSatisfiability {
    target: Satisfiability,
    num_vertices: usize,
    num_colors: usize,
}

impl ReductionResult for ReductionKColoringToSatisfiability {
    type Source = KColoring<KN, SimpleGraph>;
    type Target = Satisfiability;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            solution,
            |value| value.0,
            "target assignment does not certify a proper coloring",
        )?;
        // Validation establishes exactly one true entry in every vertex block.
        (0..self.num_vertices)
            .map(|v| {
                (0..self.num_colors)
                    .find(|&c| solution[v * self.num_colors + c])
                    .ok_or_else(|| crate::rules::ExtractionError::invalid("vertex has no color"))
            })
            .collect()
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionKColoringToSatisfiability {}

fn coloring_sat_size(
    n: usize,
    m: usize,
    k: usize,
) -> Result<(usize, usize, usize), crate::rules::ReductionError> {
    let overflow = || {
        crate::rules::ReductionError::integer_overflow::<KColoring<KN, SimpleGraph>, Satisfiability>(
            "computing coloring CNF sizes",
        )
    };
    if n == 0 {
        return Ok((0, 0, 0));
    }
    let variables = n.checked_mul(k).ok_or_else(overflow)?;
    <KColoring<KN, SimpleGraph> as ReduceTo<Satisfiability>>::exact_i64(
        variables,
        "checking the SAT literal limit",
    )?;
    // Divide the even factor first, avoiding an overflowing intermediate product.
    let pairs = if k < 2 {
        0
    } else if k.is_multiple_of(2) {
        (k / 2).checked_mul(k - 1).ok_or_else(overflow)?
    } else {
        k.checked_mul((k - 1) / 2).ok_or_else(overflow)?
    };
    let exclusions = n.checked_mul(pairs).ok_or_else(overflow)?;
    let conflicts = m.checked_mul(k).ok_or_else(overflow)?;
    let binary = exclusions.checked_add(conflicts).ok_or_else(overflow)?;
    let clauses = n.checked_add(binary).ok_or_else(overflow)?;
    let literals = binary
        .checked_mul(2)
        .and_then(|count| variables.checked_add(count))
        .ok_or_else(overflow)?;
    Ok((variables, clauses, literals))
}

fn color_literal(v: usize, k: usize, c: usize) -> Result<i64, crate::rules::ReductionError> {
    let index = v
        .checked_mul(k)
        .and_then(|i| i.checked_add(c))
        .and_then(|i| i.checked_add(1))
        .ok_or_else(|| {
            crate::rules::ReductionError::integer_overflow::<
                KColoring<KN, SimpleGraph>,
                Satisfiability,
            >("indexing a coloring literal")
        })?;
    <KColoring<KN, SimpleGraph> as ReduceTo<Satisfiability>>::exact_i64(
        index,
        "converting a coloring literal to i64",
    )
}

#[crate::reduction(transform = exact {
    num_vars = "num_vertices * num_colors",
    num_clauses = "num_vertices + num_vertices * num_colors * (num_colors - 1) / 2 + num_edges * num_colors",
    num_literals = "num_vertices * num_colors + num_vertices * num_colors * (num_colors - 1) + 2 * num_edges * num_colors",
})]
impl ReduceTo<Satisfiability> for KColoring<KN, SimpleGraph> {
    type Result = ReductionKColoringToSatisfiability;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let n = self.num_vertices();
        let k = self.num_colors();
        let (num_vars, num_clauses, _) = coloring_sat_size(n, self.num_edges(), k)?;
        let allocation_error = |error| {
            crate::rules::ReductionError::invalid_target::<Self, Satisfiability>(format!(
                "cannot allocate coloring CNF: {error}"
            ))
        };
        let mut clauses = Vec::new();
        clauses
            .try_reserve_exact(num_clauses)
            .map_err(allocation_error)?;
        for v in 0..n {
            let mut literals = Vec::new();
            literals.try_reserve_exact(k).map_err(allocation_error)?;
            for c in 0..k {
                literals.push(color_literal(v, k, c)?);
            }
            clauses.push(CNFClause::new(literals));
        }
        for v in 0..n {
            for a in 0..k {
                for b in (a + 1)..k {
                    clauses.push(CNFClause::new(vec![
                        -color_literal(v, k, a)?,
                        -color_literal(v, k, b)?,
                    ]));
                }
            }
        }
        for (u, v) in self.graph().edges() {
            for c in 0..k {
                clauses.push(CNFClause::new(vec![
                    -color_literal(u, k, c)?,
                    -color_literal(v, k, c)?,
                ]));
            }
        }
        Ok(ReductionKColoringToSatisfiability {
            target: Satisfiability::try_new(num_vars, clauses)
                .map_err(<Self as ReduceTo<Satisfiability>>::target_construction)?,
            num_vertices: n,
            num_colors: k,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "kcoloring_to_satisfiability",
        build: || {
            let source = KColoring::<KN, _>::with_k(
                SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]),
                3,
            );
            crate::example_db::specs::rule_example_with_witness::<_, Satisfiability>(
                source,
                crate::export::SolutionPair {
                    source_config: serde_json::json!([0, 1, 0, 1, 2]),
                    target_config: serde_json::json!([
                        true, false, false, false, true, false, true, false, false, false, true,
                        false, false, false, true,
                    ]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/kcoloring_satisfiability.rs"]
mod tests;
