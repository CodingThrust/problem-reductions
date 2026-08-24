use crate::models::misc::SubsetSum;
use crate::models::misc::{IntExpr, IntegerExpressionMembership};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};
use num_traits::ToPrimitive;

#[derive(Debug, Clone)]
pub struct ReductionSubsetSumToIntegerExpressionMembership {
    target: IntegerExpressionMembership,
}

impl ReductionResult for ReductionSubsetSumToIntegerExpressionMembership {
    type Source = SubsetSum;
    type Target = IntegerExpressionMembership;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &[usize],
    ) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            // Union choice 0 = left = Atom(1) = exclude, choice 1 = right = Atom(s_i+1) = include.
            // This maps directly to SubsetSum's 0/1 include/exclude encoding.
            target_solution.to_vec()
        })
    }
}

/// Build a left-associative chain of `Sum` nodes over the given union nodes.
///
/// For n items with sizes s_0, ..., s_{n-1}, each item becomes
/// `Union(Atom(1), Atom(s_i + 1))`. The chain is built as:
/// `Sum(Sum(...Sum(Union_0, Union_1), Union_2), ..., Union_{n-1})`.
///
/// DFS order visits Union_0 first, then Union_1, etc., so config[i]
/// corresponds to item i.
fn build_expression(sizes: &[i64]) -> Result<IntExpr, &'static str> {
    let make_union = |size: i64| -> Result<IntExpr, &'static str> {
        let included = size
            .checked_add(1)
            .ok_or("an item size cannot be shifted into the target expression domain")?;
        Ok(IntExpr::Union(
            Box::new(IntExpr::Atom(1)),
            Box::new(IntExpr::Atom(included)),
        ))
    };

    let mut sizes = sizes.iter().copied();
    let first = sizes
        .next()
        .ok_or("the target expression requires at least one source item")?;
    let mut expr = make_union(first)?;
    for size in sizes {
        expr = IntExpr::Sum(Box::new(expr), Box::new(make_union(size)?));
    }
    Ok(expr)
}

#[reduction(
    size = exact {
        num_union_nodes = "num_elements",
    })]
impl ReduceTo<IntegerExpressionMembership> for SubsetSum {
    type Result = ReductionSubsetSumToIntegerExpressionMembership;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let sizes: Vec<i64> = self
            .sizes()
            .iter()
            .map(|size| {
                size.to_i64().ok_or_else(|| {
                    crate::rules::ReductionError::invalid_target::<
                        SubsetSum,
                        IntegerExpressionMembership,
                    >("subset size does not fit the target i64 domain")
                })
            })
            .collect::<Result<_, _>>()?;

        let shift =
            i64::try_from(self.num_elements()).map_err(|_| {
                crate::rules::ReductionError::integer_overflow::<
                    SubsetSum,
                    IntegerExpressionMembership,
                >("converting the number of elements to i64")
            })?;
        let source_target = self.target().to_i64().ok_or_else(|| {
            crate::rules::ReductionError::invalid_target::<SubsetSum, IntegerExpressionMembership>(
                "subset target does not fit the target i64 domain",
            )
        })?;
        let target =
            source_target.checked_add(shift).ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<
                    SubsetSum,
                    IntegerExpressionMembership,
                >("computing the shifted target")
            })?;

        let expr = build_expression(&sizes).map_err(|message| {
            crate::rules::ReductionError::invalid_target::<SubsetSum, IntegerExpressionMembership>(
                message,
            )
        })?;

        Ok(ReductionSubsetSumToIntegerExpressionMembership {
            target: IntegerExpressionMembership::new(expr, target),
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetsum_to_integerexpressionmembership",
        build: || {
            crate::example_db::specs::rule_example_with_witness::<_, IntegerExpressionMembership>(
                SubsetSum::new(vec![1u32, 5, 6, 8], 11u32),
                SolutionPair {
                    source_config: vec![0, 1, 1, 0],
                    target_config: vec![0, 1, 1, 0],
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetsum_integerexpressionmembership.rs"]
mod tests;
