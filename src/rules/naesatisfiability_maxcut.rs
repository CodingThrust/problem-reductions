//! Reduction from NAESatisfiability to MaxCut.
//!
//! Long NAE clauses are first expanded into a chain of ternary clauses with
//! fresh variables. Binary clauses contribute one unit edge, ternary clauses
//! contribute a unit triangle. A positive-weight edge separates each variable
//! from its negation. A cut certifies a source witness exactly when it attains
//! the sum of these componentwise upper bounds.
//!
//! The NAE clause-chain construction is described by Jackson,
//! "Flexible constraint satisfiability and a problem in semigroup theory",
//! Section 4, arXiv:1512.03127. The triangle construction is the classical
//! NAE-3SAT to MaxCut reduction (Garey and Johnson, ND16).

use crate::models::formula::NAESatisfiability;
use crate::models::graph::MaxCut;
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::topology::SimpleGraph;

/// Result of reducing NAESatisfiability to MaxCut.
#[derive(Debug, Clone)]
pub struct ReductionNAESATToMaxCut {
    target: MaxCut<SimpleGraph, i64>,
    source_num_vars: usize,
    feasible_cut: i64,
}

impl ReductionResult for ReductionNAESATToMaxCut {
    type Source = NAESatisfiability;
    type Target = MaxCut<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    /// Extract a NAE-SAT assignment from a MaxCut partition.
    ///
    /// Variable x_i is assigned based on vertex 2*i: if it is in set 0
    /// (config[2*i] == 0), set x_i = false (config value 0); if in set 1,
    /// set x_i = true (config value 1).
    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        let value =
            crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;
        if !crate::rules::AggregateReductionResult::extract_value(self, value).0 {
            return Err(crate::rules::ExtractionError::invalid(
                "target cut does not certify a satisfying NAE assignment",
            ));
        }

        Ok({
            (0..self.source_num_vars)
                .map(|i| target_solution[2 * i])
                .collect()
        })
    }
}

impl crate::rules::AggregateReductionResult for ReductionNAESATToMaxCut {
    type Source = NAESatisfiability;
    type Target = MaxCut<SimpleGraph, i64>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_value(&self, value: crate::types::Max<i64>) -> crate::types::Or {
        crate::types::Or(value.0 == Some(self.feasible_cut))
    }
}

/// Dimensions, variable-edge weight, and certificate for legal clause lengths.
fn nae_maxcut_parameters(
    n: usize,
    lengths: impl ExactSizeIterator<Item = usize>,
) -> Result<(usize, usize, i64, i64), crate::rules::ReductionError> {
    let overflow = |operation| {
        crate::rules::ReductionError::integer_overflow::<NAESatisfiability, MaxCut<SimpleGraph, i64>>(
            operation,
        )
    };
    let weight = i64::try_from(lengths.len())
        .ok()
        .and_then(|m| m.checked_add(1))
        .ok_or_else(|| overflow("computing the variable-edge weight"))?;
    let mut variables = n;
    let mut clause_edges = 0usize;
    let mut clause_cap = 0usize;
    for length in lengths {
        // Source construction guarantees length >= 2.
        let (auxiliary, edges, cap) = if length == 2 {
            (0, 1, 1)
        } else {
            let triangles = length - 2;
            (
                length - 3,
                triangles
                    .checked_mul(3)
                    .ok_or_else(|| overflow("counting clause edges"))?,
                triangles
                    .checked_mul(2)
                    .ok_or_else(|| overflow("counting clause cut capacity"))?,
            )
        };
        variables = variables
            .checked_add(auxiliary)
            .ok_or_else(|| overflow("counting auxiliary variables"))?;
        clause_edges = clause_edges
            .checked_add(edges)
            .ok_or_else(|| overflow("counting all clause edges"))?;
        clause_cap = clause_cap
            .checked_add(cap)
            .ok_or_else(|| overflow("counting all clause cut capacities"))?;
    }
    let vertices = variables
        .checked_mul(2)
        .ok_or_else(|| overflow("counting literal vertices"))?;
    let edges = variables
        .checked_add(clause_edges)
        .ok_or_else(|| overflow("counting target edges"))?;
    let variable_weight = i64::try_from(variables)
        .ok()
        .and_then(|q| q.checked_mul(weight))
        .ok_or_else(|| overflow("summing variable-edge weights"))?;
    // All weights are nonnegative: bounding their total bounds every cut sum.
    variable_weight
        .checked_add(
            i64::try_from(clause_edges).map_err(|_| overflow("converting clause-edge count"))?,
        )
        .ok_or_else(|| overflow("summing all target edge weights"))?;
    let feasible_cut = variable_weight
        .checked_add(
            i64::try_from(clause_cap).map_err(|_| overflow("converting clause cut capacity"))?,
        )
        .ok_or_else(|| overflow("computing the NAE cut certificate"))?;
    // SimpleGraph uses petgraph's default index domain for nodes and edges.
    petgraph::graph::DefaultIx::try_from(vertices)
        .map_err(|_| overflow("representing target vertex indices"))?;
    petgraph::graph::DefaultIx::try_from(edges)
        .map_err(|_| overflow("representing target edge indices"))?;
    Ok((vertices, edges, weight, feasible_cut))
}

#[reduction(
    aggregate = custom,
    transform = upper_bound {
        num_vertices = "2 * (num_vars + num_literals - 2 * num_clauses)",
        num_edges = "num_vars + 4 * num_literals - 7 * num_clauses",
    }
)]
impl ReduceTo<MaxCut<SimpleGraph, i64>> for NAESatisfiability {
    type Result = ReductionNAESATToMaxCut;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let (total_vertices, total_edges, weight, feasible_cut) =
            nae_maxcut_parameters(self.num_vars(), self.clauses().iter().map(|c| c.len()))?;
        let total_variables = total_vertices / 2;
        let mut edges = Vec::with_capacity(total_edges);
        let mut weights = Vec::with_capacity(total_edges);
        for i in 0..total_variables {
            edges.push((2 * i, 2 * i + 1));
            weights.push(weight);
        }

        let mut next_auxiliary = self.num_vars();
        for clause in self.clauses() {
            let literals: Vec<_> = clause
                .literals
                .iter()
                .map(|&literal| {
                    let index = usize::try_from(literal.unsigned_abs()).map_err(|_| {
                        crate::rules::ReductionError::integer_overflow::<
                            NAESatisfiability,
                            MaxCut<SimpleGraph, i64>,
                        >("converting a literal index")
                    })? - 1;
                    // Validated literals are in 1..=n, and 2*total_variables was checked.
                    Ok(2 * index + usize::from(literal < 0))
                })
                .collect::<Result<_, crate::rules::ReductionError>>()?;
            if literals.len() == 2 {
                edges.push((literals[0], literals[1]));
                weights.push(1);
            } else {
                let mut first = literals[0];
                for &middle in &literals[1..literals.len() - 2] {
                    let auxiliary = 2 * next_auxiliary;
                    next_auxiliary += 1;
                    edges.extend([(first, middle), (first, auxiliary), (middle, auxiliary)]);
                    weights.extend([1; 3]);
                    first = auxiliary + 1;
                }
                let second = literals[literals.len() - 2];
                let third = literals[literals.len() - 1];
                edges.extend([(first, second), (first, third), (second, third)]);
                weights.extend([1; 3]);
            }
        }

        Ok(ReductionNAESATToMaxCut {
            target: MaxCut::new(SimpleGraph::new(total_vertices, edges), weights),
            source_num_vars: self.num_vars(),
            feasible_cut,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    use crate::models::formula::CNFClause;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "naesatisfiability_to_maxcut",
        build: || {
            // 3 variables, 2 clauses:
            //   C1 = (x1, x2, ~x3)
            //   C2 = (~x1, x3, x2)
            // NAE-satisfying: x1=T, x2=F, x3=T
            let source = NAESatisfiability::new(
                3,
                vec![
                    CNFClause::new(vec![1, 2, -3]),
                    CNFClause::new(vec![-1, 3, 2]),
                ],
            );
            crate::example_db::specs::rule_example_with_witness::<_, MaxCut<SimpleGraph, i64>>(
                source,
                SolutionPair {
                    // x1=T(1), x2=F(0), x3=T(1)
                    source_config: serde_json::json!(vec![true, false, true]),
                    // Vertices: x1(0)=1, ~x1(1)=0, x2(2)=0, ~x2(3)=1, x3(4)=1, ~x3(5)=0
                    // All variable edges cross (weight M=3 each) -> 3*3=9
                    // C1=(x1,x2,~x3): vertices 0,2,5 -> sides {1},{0,0} -> edges (0,2) crosses, (0,5) crosses, (2,5) doesn't -> +2
                    // C2=(~x1,x3,x2): vertices 1,4,2 -> sides {0},{1,0} -> edges (1,4) crosses, (1,2) doesn't, (4,2) crosses -> +2
                    // Total = 9 + 2 + 2 = 13
                    target_config: serde_json::json!(vec![true, false, false, true, true, false]),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/naesatisfiability_maxcut.rs"]
mod tests;
