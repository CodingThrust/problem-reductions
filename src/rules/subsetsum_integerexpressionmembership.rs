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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Union choice 0 = left = Atom(1) = exclude, choice 1 = right = Atom(s_i+1) = include.
        // This maps directly to SubsetSum's 0/1 include/exclude encoding.
        target_solution.to_vec()
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
fn build_expression(sizes: &[u64]) -> IntExpr {
    assert!(
        !sizes.is_empty(),
        "SubsetSum must have at least one element"
    );

    let make_union = |s: u64| -> IntExpr {
        IntExpr::Union(Box::new(IntExpr::Atom(1)), Box::new(IntExpr::Atom(s + 1)))
    };

    let mut expr = make_union(sizes[0]);
    for &s in &sizes[1..] {
        expr = IntExpr::Sum(Box::new(expr), Box::new(make_union(s)));
    }
    expr
}

#[reduction(overhead = {
    num_union_nodes = "num_elements",
})]
impl ReduceTo<IntegerExpressionMembership> for SubsetSum {
    type Result = ReductionSubsetSumToIntegerExpressionMembership;

    fn reduce_to(&self) -> Self::Result {
        let sizes: Vec<u64> = self
            .sizes()
            .iter()
            .map(|size| size.to_u64().unwrap())
            .collect();

        let shift = u64::try_from(self.num_elements())
            .expect("SubsetSum -> IntegerExpressionMembership requires num_elements to fit in u64");
        let target = self.target().to_u64().unwrap().checked_add(shift).expect(
            "SubsetSum -> IntegerExpressionMembership requires shifted target to fit in u64",
        );

        let expr = build_expression(&sizes);

        ReductionSubsetSumToIntegerExpressionMembership {
            target: IntegerExpressionMembership::new(expr, target),
        }
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
