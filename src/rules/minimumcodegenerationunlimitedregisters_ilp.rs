//! Rank dependencies and copy indicators for two-address operations.
use crate::models::algebraic::{Bounded, IntegerVariable, LinearConstraint, ObjectiveSense, ILP};
use crate::models::misc::MinimumCodeGenerationUnlimitedRegisters;
use crate::rules::ilp_helpers::ranks_to_positions;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionMinimumCodeGenerationUnlimitedRegistersToILP {
    target: ILP<i64, i64, Bounded>,
    num_operations: usize,
}
impl ReductionResult for ReductionMinimumCodeGenerationUnlimitedRegistersToILP {
    type Source = MinimumCodeGenerationUnlimitedRegisters;
    type Target = ILP<i64, i64, Bounded>;
    fn target_problem(&self) -> &Self::Target {
        &self.target
    }
    fn extract_solution(&self, solution: &Vec<i64>) -> crate::rules::ExtractionResult<Vec<usize>> {
        crate::rules::traits::validate_target_witness(
            &self.target,
            solution,
            |value| value.value.is_some(),
            "target ILP assignment is infeasible",
        )?;
        Ok(ranks_to_positions(&solution[..self.num_operations]))
    }
}
#[crate::reduction(transform = upper_bound {
    num_vars = "2 * num_vertices + 1",
    num_constraints = "num_vertices * num_vertices + num_vertices",
    num_nonzeros = "3 * num_vertices * num_vertices + num_vertices",
    max_constraint_magnitude_bits = "num_vertices + 1",
})]
impl ReduceTo<ILP<i64, i64, Bounded>> for MinimumCodeGenerationUnlimitedRegisters {
    type Result = ReductionMinimumCodeGenerationUnlimitedRegistersToILP;
    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        // Every internal vertex has exactly one left operand.
        let mut operations = self.left_arcs().to_vec();
        operations.sort_unstable();
        let k = operations.len();
        let count = k
            .checked_mul(2)
            .and_then(|v| v.checked_add(1))
            .ok_or_else(|| {
                crate::rules::ReductionError::integer_overflow::<Self, ILP<i64, i64, Bounded>>(
                    "counting code-generation variables",
                )
            })?;
        let bound = Self::exact_i64(k, "bounding operation ranks")?;
        Self::exact_i64(count, "bounding the instruction count")?;
        let mut variables = Vec::with_capacity(count);
        for _ in 0..k {
            variables.push(
                IntegerVariable::new(Some(0), Some(bound - 1))
                    .map_err(Self::target_construction)?,
            );
        }
        variables.extend(std::iter::repeat_n(IntegerVariable::binary(), k));
        variables.push(IntegerVariable::new(Some(1), Some(1)).map_err(Self::target_construction)?);
        let mut index = vec![None; self.num_vertices()];
        let mut users = vec![Vec::new(); self.num_vertices()];
        for (i, &(v, _)) in operations.iter().enumerate() {
            index[v] = Some(i);
        }
        let mut rows = Vec::new();
        for &(v, child) in self.left_arcs().iter().chain(self.right_arcs()) {
            let parent = index[v].expect("every operation has a left operand");
            users[child].push(parent);
            if let Some(child) = index[child] {
                rows.push(LinearConstraint::ge(vec![(parent, 1), (child, -1)], 1));
            }
        }
        for users in &mut users {
            users.sort_unstable();
            users.dedup();
        }
        for (v, &(_, left)) in operations.iter().enumerate() {
            for &u in &users[left] {
                if u != v {
                    rows.push(LinearConstraint::ge(
                        vec![(v, 1), (u, -1), (k + v, bound)],
                        1,
                    ));
                }
            }
        }
        let mut objective: Vec<_> = (k..2 * k).map(|i| (i, 1)).collect();
        objective.push((2 * k, bound));
        let target = ILP::with_variables(variables, rows, objective, ObjectiveSense::Minimize)
            .map_err(Self::target_construction)?;
        crate::rules::ilp_helpers::validate_bounded_constraint_arithmetic::<Self>(&target)?;
        Ok(Self::Result {
            target,
            num_operations: k,
        })
    }
}
#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "minimumcodegenerationunlimitedregisters_to_ilp",
        build: || {
            crate::example_db::specs::rule_example_via_bounded_ilp(
                MinimumCodeGenerationUnlimitedRegisters::new(
                    5,
                    vec![(1, 3), (2, 3), (0, 1)],
                    vec![(1, 4), (2, 4), (0, 2)],
                ),
            )
        },
    }]
}
#[cfg(test)]
#[path = "../unit_tests/rules/minimumcodegenerationunlimitedregisters_ilp.rs"]
mod tests;
