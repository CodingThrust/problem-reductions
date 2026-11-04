//! Reduction from SubsetProduct to CircuitSAT by exact binary multiplication.

use crate::models::formula::{Assignment, BooleanExpr, Circuit};
use crate::models::{CircuitSAT, SubsetProduct};
use crate::reduction;
use crate::rules::{ExtractionError, ReduceTo, ReductionError, ReductionResult};

/// Circuit and named selector coordinates for subset-product extraction.
#[derive(Debug, Clone)]
pub struct ReductionSubsetProductToCircuitSAT {
    target: CircuitSAT,
    source_var_indices: Vec<usize>,
}

impl ReductionResult for ReductionSubsetProductToCircuitSAT {
    type Source = SubsetProduct;
    type Target = CircuitSAT;

    fn target_problem(&self) -> &CircuitSAT {
        &self.target
    }

    fn extract_solution(&self, solution: &Vec<bool>) -> crate::rules::ExtractionResult<Vec<bool>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.0,
            "target circuit is not satisfied",
        )?;
        self.source_var_indices
            .iter()
            .map(|&index| {
                solution
                    .get(index)
                    .copied()
                    .ok_or_else(|| ExtractionError::invalid("missing selector coordinate"))
            })
            .collect()
    }
}

#[crate::aggregate_reduction(identity)]
impl crate::rules::AggregateReductionResult for ReductionSubsetProductToCircuitSAT {}

fn checked<T>(value: Option<T>) -> Result<T, ReductionError> {
    value.ok_or_else(|| {
        ReductionError::integer_overflow::<SubsetProduct, CircuitSAT>(
            "sizing the multiplication circuit",
        )
    })
}

fn sum_sizes(values: impl IntoIterator<Item = usize>) -> Result<usize, ReductionError> {
    values
        .into_iter()
        .try_fold(0usize, |sum, value| checked(sum.checked_add(value)))
}

fn index(value: u64) -> Result<usize, ReductionError> {
    checked(usize::try_from(value).ok())
}

// Width, variables, assignments (also outputs), and expression nodes.
fn dimensions(
    n: usize,
    d: usize,
    h: usize,
    target_bits: usize,
) -> Result<(usize, usize, usize, usize), ReductionError> {
    let w = checked(d.checked_add(1))?.max(target_bits);
    let gates = checked(h.checked_mul(5))?;
    let variables = checked(checked(w.checked_mul(sum_sizes([1, n, gates])?))?.checked_add(n))?;
    let assignments = checked(w.checked_mul(sum_sizes([2, n, gates])?))?;
    let nodes = checked(w.checked_mul(sum_sizes([
        2,
        checked(n.checked_mul(8))?,
        checked(h.checked_mul(15))?,
    ])?))?;
    checked(assignments.checked_mul(std::mem::size_of::<Assignment>()))?;
    checked(nodes.checked_mul(std::mem::size_of::<BooleanExpr>()))?;
    checked(variables.checked_mul(std::mem::size_of::<String>()))?;
    Ok((w, variables, assignments, nodes))
}

fn reserve_assignments(count: usize) -> Result<Vec<Assignment>, ReductionError> {
    let mut assignments = Vec::new();
    assignments.try_reserve_exact(count).map_err(|error| {
        <SubsetProduct as ReduceTo<CircuitSAT>>::target_construction(
            crate::registry::ConstructionError::Conversion(format!(
                "cannot allocate circuit assignments: {error}"
            )),
        )
    })?;
    Ok(assignments)
}

#[reduction(transform = unavailable {
    num_variables = "Integer bit lengths and popcounts are not bounded by num_elements",
    num_assignments = "Integer bit lengths and popcounts are not bounded by num_elements",
    num_assignment_outputs = "Integer bit lengths and popcounts are not bounded by num_elements",
    num_expression_nodes = "Integer bit lengths and popcounts are not bounded by num_elements",
})]
impl ReduceTo<CircuitSAT> for SubsetProduct {
    type Result = ReductionSubsetProductToCircuitSAT;

    fn reduce_to(&self) -> Result<Self::Result, ReductionError> {
        let d = self
            .sizes()
            .iter()
            .try_fold(0usize, |sum, a| checked(sum.checked_add(index(a.bits())?)))?;
        let h = self.sizes().iter().try_fold(0usize, |sum, a| {
            checked(sum.checked_add(index(a.count_ones())?))
        })?;
        let (w, _, count, _) = dimensions(self.sizes().len(), d, h, index(self.target().bits())?)?;
        let mut assignments = reserve_assignments(count)?;
        for b in 0..w {
            assignments.push(Assignment::new(
                vec![format!("p_0_{b}")],
                BooleanExpr::constant(b == 0),
            ));
        }
        for (i, a) in self.sizes().iter().enumerate() {
            // The width bounds every branch, including products discarded by a selector.
            let stage = checked(i.checked_add(1))?;
            let mut accumulator = vec![BooleanExpr::constant(false); w];
            for shift in (0..a.bits()).filter(|&shift| a.bit(shift)) {
                let j = index(shift)?;
                let mut carry = BooleanExpr::constant(false);
                for (b, sum) in accumulator.iter_mut().enumerate() {
                    let u = sum.clone();
                    let v = if b >= j {
                        BooleanExpr::var(&format!("p_{i}_{}", b - j))
                    } else {
                        BooleanExpr::constant(false)
                    };
                    let name = |kind| format!("a_{stage}_{j}_{b}_{kind}");
                    let h = BooleanExpr::var(&name('h'));
                    let f = BooleanExpr::var(&name('f'));
                    let g = BooleanExpr::var(&name('g'));
                    for (kind, expr) in [
                        ('h', BooleanExpr::xor(vec![u.clone(), v.clone()])),
                        ('s', BooleanExpr::xor(vec![h.clone(), carry.clone()])),
                        ('f', BooleanExpr::and(vec![u, v])),
                        ('g', BooleanExpr::and(vec![h, carry])),
                        ('d', BooleanExpr::or(vec![f, g])),
                    ] {
                        assignments.push(Assignment::new(vec![name(kind)], expr));
                    }
                    *sum = BooleanExpr::var(&name('s'));
                    carry = BooleanExpr::var(&name('d'));
                }
            }
            for (b, product) in accumulator.into_iter().enumerate() {
                let selector = BooleanExpr::var(&format!("x_{i}"));
                assignments.push(Assignment::new(
                    vec![format!("p_{stage}_{b}")],
                    BooleanExpr::or(vec![
                        BooleanExpr::and(vec![selector.clone(), product]),
                        BooleanExpr::and(vec![
                            BooleanExpr::not(selector),
                            BooleanExpr::var(&format!("p_{i}_{b}")),
                        ]),
                    ]),
                ));
            }
        }
        for b in 0..w {
            let bit = checked(u64::try_from(b).ok())?;
            assignments.push(Assignment::new(
                vec![format!("p_{}_{b}", self.sizes().len())],
                BooleanExpr::constant(self.target().bit(bit)),
            ));
        }
        let target =
            CircuitSAT::try_new(Circuit::new(assignments)).map_err(Self::target_construction)?;
        let source_var_indices = (0..self.sizes().len())
            .map(|i| {
                target
                    .variable_names()
                    .binary_search(&format!("x_{i}"))
                    .map_err(|_| {
                        ReductionError::invalid_target::<Self, CircuitSAT>(
                            "missing selector variable",
                        )
                    })
            })
            .collect::<Result<_, _>>()?;
        Ok(ReductionSubsetProductToCircuitSAT {
            target,
            source_var_indices,
        })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "subsetproduct_to_circuitsat",
        build: || {
            let source = SubsetProduct::new(vec![2u32, 3, 5], 30u32);
            let reduction = ReduceTo::<CircuitSAT>::reduce_to(&source).expect("example reduction");
            let target = reduction.target_problem();
            let mut values = std::collections::HashMap::from([
                ("x_0".to_owned(), true),
                ("x_1".to_owned(), true),
                ("x_2".to_owned(), true),
            ]);
            for assignment in &target.circuit().assignments {
                let value = assignment.expr.evaluate(&values);
                for name in &assignment.outputs {
                    values.entry(name.clone()).or_insert(value);
                }
            }
            let witness: Vec<bool> = target
                .variable_names()
                .iter()
                .map(|name| values[name])
                .collect();
            assert_eq!(reduction.extract_solution(&witness).unwrap(), vec![true; 3]);
            crate::example_db::specs::assemble_rule_example(
                &source,
                target,
                vec![crate::export::SolutionPair {
                    source_config: serde_json::json!([true, true, true]),
                    target_config: serde_json::json!(witness),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/subsetproduct_circuitsat.rs"]
mod tests;
