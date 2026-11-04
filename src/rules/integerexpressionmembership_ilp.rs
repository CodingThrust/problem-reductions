//! Binary activation and ripple-carry encoding of integer expression membership.

use crate::models::algebraic::{ILP, LinearConstraint, ObjectiveSense};
use crate::models::misc::{IntExpr, IntegerExpressionMembership};
use crate::rules::ReductionError;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
enum Node {
    Atom(i64),
    Sum(usize, usize),
    Union(usize, usize),
}

fn flatten(expression: &IntExpr, nodes: &mut Vec<Node>) -> usize {
    let index = nodes.len();
    nodes.push(Node::Atom(0));
    nodes[index] = match expression {
        IntExpr::Atom(value) => Node::Atom(*value),
        IntExpr::Sum(left, right) => Node::Sum(flatten(left, nodes), flatten(right, nodes)),
        IntExpr::Union(left, right) => Node::Union(flatten(left, nodes), flatten(right, nodes)),
    };
    index
}

fn overflow(operation: &str) -> ReductionError {
    ReductionError::integer_overflow::<IntegerExpressionMembership, ILP<bool>>(operation)
}

fn bit_width(value: i64) -> Result<usize, ReductionError> {
    if value <= 0 {
        return Err(ReductionError::invalid_target::<
            IntegerExpressionMembership,
            ILP<bool>,
        >("binary encoding requires a positive integer"));
    }
    let width = i64::BITS
        .checked_sub(value.leading_zeros())
        .ok_or_else(|| overflow("computing the bit width"))?;
    usize::try_from(width).map_err(|_| overflow("converting the bit width"))
}

fn bit_at(value: i64, bit: usize) -> Result<i64, ReductionError> {
    let shift = u32::try_from(bit).map_err(|_| overflow("converting a bit index"))?;
    value
        .checked_shr(shift)
        .map(|value| value & 1)
        .ok_or_else(|| overflow("reading an integer bit"))
}

fn checked_layout(
    atoms: usize,
    sums: usize,
    unions: usize,
    width: usize,
) -> Result<(usize, usize, usize), ReductionError> {
    if atoms == 0 || !(1..=63).contains(&width) {
        return Err(ReductionError::invalid_target::<
            IntegerExpressionMembership,
            ILP<bool>,
        >(
            "the expression must have atoms and a bit width from 1 to 63",
        ));
    }
    let nodes = atoms
        .checked_add(atoms - 1)
        .ok_or_else(|| overflow("counting expression nodes"))?;
    let prefixes = atoms
        .checked_mul(width)
        .ok_or_else(|| overflow("counting prefix bits"))?;
    let carries = atoms
        .checked_mul(width - 1)
        .ok_or_else(|| overflow("counting carry bits"))?;
    let variables = nodes
        .checked_add(prefixes)
        .and_then(|n| n.checked_add(carries))
        .ok_or_else(|| overflow("counting binary ILP variables"))?;
    let rows = sums
        .checked_mul(2)
        .and_then(|n| n.checked_add(1))
        .and_then(|n| n.checked_add(unions))
        .and_then(|n| n.checked_add(prefixes))
        .and_then(|n| n.checked_add(width))
        .ok_or_else(|| overflow("counting binary ILP constraints"))?;
    Ok((nodes, variables, rows))
}

#[derive(Debug, Clone)]
pub struct ReductionIntegerExpressionMembershipToILP {
    target: ILP<bool>,
    nodes: Vec<Node>,
    unions: usize,
}

impl ReductionResult for ReductionIntegerExpressionMembershipToILP {
    type Source = IntegerExpressionMembership;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        fn visit(nodes: &[Node], index: usize, solution: &[i64], choices: &mut Vec<bool>) {
            match nodes[index] {
                Node::Atom(_) => {}
                Node::Sum(left, right) => {
                    visit(nodes, left, solution, choices);
                    visit(nodes, right, solution, choices);
                }
                Node::Union(left, right) => {
                    let choose_right = solution[right] == 1;
                    choices.push(choose_right);
                    visit(
                        nodes,
                        if choose_right { right } else { left },
                        solution,
                        choices,
                    );
                }
            }
        }
        let mut choices = Vec::with_capacity(self.unions);
        visit(&self.nodes, 0, solution, &mut choices);
        // Unvisited unions consume no bits; the model ignores this trailing sentinel.
        choices.resize(self.unions, false);
        Ok(choices)
    }
}

#[crate::aggregate_reduction(ilp_feasibility)]
impl crate::rules::AggregateReductionResult for ReductionIntegerExpressionMembershipToILP {}

// T + A(2H-1) variables, 1+2S+U+AH+H rows and A(4H-2)+B+1+4S+3U nonzeros.
// Coefficients are -2,-1,1, and RHS/bounds are 0 or 1, including H=1 without carries.
#[crate::reduction(transform = {
    upper_bound { max_constraint_magnitude_bits = "2", },
    unavailable {
        num_vars = "Atom count and bit width are not bounded by num_union_nodes",
        num_constraints = "Atom count and bit width are not bounded by num_union_nodes",
        num_nonzeros = "Atom count and bit width are not bounded by num_union_nodes",
    },
})]
impl ReduceTo<ILP<bool>> for IntegerExpressionMembership {
    type Result = ReductionIntegerExpressionMembershipToILP;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let mut nodes = Vec::new();
        flatten(self.expression(), &mut nodes);
        let atoms: Vec<_> = nodes
            .iter()
            .enumerate()
            .filter_map(|(index, node)| match node {
                Node::Atom(value) => Some((index, *value)),
                _ => None,
            })
            .collect();
        let sums = nodes
            .iter()
            .filter(|node| matches!(node, Node::Sum(..)))
            .count();
        let unions = nodes
            .iter()
            .filter(|node| matches!(node, Node::Union(..)))
            .count();
        let width = atoms
            .iter()
            .try_fold(bit_width(self.target())?, |width, (_, value)| {
                Ok::<_, ReductionError>(width.max(bit_width(*value)?))
            })?;
        let (node_count, variables, row_count) = checked_layout(atoms.len(), sums, unions, width)?;
        // The checked total bounds every included prefix/carry offset below.
        let prefix = |j: usize, b: usize| node_count + j * width + b;
        let carry = |j: usize, b: usize| node_count + atoms.len() * width + j * (width - 1) + b - 1;
        let mut rows = Vec::with_capacity(row_count);
        rows.push(LinearConstraint::eq(vec![(0, 1)], 1));
        for (index, node) in nodes.iter().enumerate() {
            match *node {
                Node::Atom(_) => {}
                Node::Sum(left, right) => {
                    rows.push(LinearConstraint::eq(vec![(left, 1), (index, -1)], 0));
                    rows.push(LinearConstraint::eq(vec![(right, 1), (index, -1)], 0));
                }
                Node::Union(left, right) => {
                    rows.push(LinearConstraint::eq(
                        vec![(left, 1), (right, 1), (index, -1)],
                        0,
                    ));
                }
            }
        }
        for (j, &(leaf, value)) in atoms.iter().enumerate() {
            for b in 0..width {
                let mut terms = vec![(prefix(j, b), -1)];
                if j > 0 {
                    terms.push((prefix(j - 1, b), 1));
                }
                if bit_at(value, b)? == 1 {
                    terms.push((leaf, 1));
                }
                if b > 0 {
                    terms.push((carry(j, b), 1));
                }
                if b + 1 < width {
                    terms.push((carry(j, b + 1), -2));
                }
                rows.push(LinearConstraint::eq(terms, 0));
            }
        }
        for b in 0..width {
            rows.push(LinearConstraint::eq(
                vec![(prefix(atoms.len() - 1, b), 1)],
                bit_at(self.target(), b)?,
            ));
        }
        let target = ILP::new(variables, rows, vec![], ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        Ok(Self::Result {
            target,
            nodes,
            unions,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::export::SolutionPair;
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "integerexpressionmembership_to_ilp",
        build: || {
            let union = |left, right| {
                IntExpr::Union(
                    Box::new(IntExpr::Atom(left)),
                    Box::new(IntExpr::Atom(right)),
                )
            };
            let expression = IntExpr::Sum(
                Box::new(IntExpr::Sum(Box::new(union(1, 4)), Box::new(union(3, 6)))),
                Box::new(union(2, 5)),
            );
            let mut witness = vec![1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0];
            witness.extend([
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1,
            ]);
            witness.extend([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0]);
            crate::example_db::specs::rule_example_with_witness::<_, ILP<bool>>(
                IntegerExpressionMembership::new(expression, 12),
                SolutionPair {
                    source_config: serde_json::json!([true, true, false]),
                    target_config: serde_json::json!(witness),
                },
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/integerexpressionmembership_ilp.rs"]
mod tests;
