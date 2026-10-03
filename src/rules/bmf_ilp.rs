//! Exact Boolean factorization with coverage indicators only for true entries.

use crate::models::algebraic::{LinearConstraint, ObjectiveSense, BMF, ILP};
use crate::reduction;
use crate::rules::traits::{ReduceTo, ReductionResult};

#[derive(Debug, Clone)]
pub struct ReductionBMFToILP {
    target: ILP<bool>,
    m: usize,
    n: usize,
    k: usize,
}

impl ReductionResult for ReductionBMFToILP {
    type Source = BMF;
    type Target = ILP<bool>;

    fn target_problem(&self) -> &ILP<bool> {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_witness(
            self.target_problem(),
            target_solution,
            |v| v.value.is_some(),
            "target ILP assignment does not reconstruct the Boolean matrix",
        )?;

        let b = (0..self.m)
            .map(|i| {
                (0..self.k)
                    .map(|r| target_solution[i * self.k + r] == 1)
                    .collect()
            })
            .collect();
        let c_offset = self.m * self.k;
        let c = (0..self.k)
            .map(|r| {
                (0..self.n)
                    .map(|j| target_solution[c_offset + r * self.n + j] == 1)
                    .collect()
            })
            .collect();
        Ok((b, c))
    }
}

#[reduction(transform = {
    exact { max_constraint_magnitude_bits = "1", },
    upper_bound {
        num_vars = "rows * rank + rank * cols + rows * rank * cols",
        num_constraints = "(2 * rank + 1) * rows * cols + rank * (rows + cols)",
        num_nonzeros = "5 * rows * rank * cols + rank * (rows + cols)",
    },
})]
impl ReduceTo<ILP<bool>> for BMF {
    type Result = ReductionBMFToILP;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let m = self.rows();
        let n = self.cols();
        let k = self.rank();
        let overflow = || {
            crate::rules::ReductionError::integer_overflow::<Self, ILP<bool>>(
                "counting Boolean factorization variables",
            )
        };
        let c_offset = m.checked_mul(k).ok_or_else(overflow)?;
        let factor_count = k
            .checked_mul(n)
            .and_then(|v| v.checked_add(c_offset))
            .ok_or_else(overflow)?;
        <Self as ReduceTo<ILP<bool>>>::exact_i64(factor_count, "bounding Boolean factor size")?;
        // Pairwise incompatible edges must use distinct factors. Name those
        // factors first: permuting B columns and C rows preserves every cover
        // and its objective. Anchor endpoints then exclude non-neighbors.
        let matrix = self.matrix();
        let row_degrees: Vec<_> = matrix
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .collect();
        let col_degrees: Vec<_> = (0..n)
            .map(|j| matrix.iter().filter(|row| row[j]).count())
            .collect();
        let mut edges: Vec<_> = matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &v)| v.then_some((i, j)))
            })
            .collect();
        edges.sort_by_key(|&(i, j)| (row_degrees[i], col_degrees[j]));
        let mut anchors: Vec<(usize, usize)> = Vec::new();
        for (i, j) in edges {
            if anchors.len() == k {
                break;
            }
            if anchors.iter().all(|&(u, v)| !matrix[i][v] || !matrix[u][j]) {
                anchors.push((i, j));
            }
        }
        let mut fixed = vec![None; factor_count];
        for (r, &(u, v)) in anchors.iter().enumerate() {
            for i in 0..m {
                if !matrix[i][v] {
                    fixed[i * k + r] = Some(0);
                }
            }
            for j in 0..n {
                if !matrix[u][j] {
                    fixed[c_offset + r * n + j] = Some(0);
                }
            }
            fixed[u * k + r] = Some(1);
            fixed[c_offset + r * n + v] = Some(1);
        }
        let mut next = factor_count;
        let mut constraints: Vec<_> = fixed
            .iter()
            .enumerate()
            .filter_map(|(index, &value)| {
                value.map(|value| LinearConstraint::eq(vec![(index, 1)], value))
            })
            .collect();
        for (i, row) in self.matrix().iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value {
                    if (0..k).any(|r| {
                        fixed[i * k + r] == Some(1) && fixed[c_offset + r * n + j] == Some(1)
                    }) {
                        continue;
                    }
                    let mut coverage = Vec::new();
                    for r in 0..k {
                        if fixed[i * k + r] == Some(0) || fixed[c_offset + r * n + j] == Some(0) {
                            continue;
                        }
                        let bit = next;
                        next = next.checked_add(1).ok_or_else(overflow)?;
                        constraints.push(LinearConstraint::le(vec![(bit, 1), (i * k + r, -1)], 0));
                        constraints.push(LinearConstraint::le(
                            vec![(bit, 1), (c_offset + r * n + j, -1)],
                            0,
                        ));
                        coverage.push((bit, 1));
                    }
                    constraints.push(LinearConstraint::ge(coverage, 1));
                } else {
                    for r in 0..k {
                        if fixed[i * k + r] == Some(0) || fixed[c_offset + r * n + j] == Some(0) {
                            continue;
                        }
                        constraints.push(LinearConstraint::le(
                            vec![(i * k + r, 1), (c_offset + r * n + j, 1)],
                            1,
                        ));
                    }
                }
            }
        }
        let objective = (0..factor_count).map(|i| (i, 1)).collect();
        let target = ILP::new(next, constraints, objective, ObjectiveSense::Minimize)
            .map_err(<Self as ReduceTo<ILP<bool>>>::target_construction)?;
        Ok(ReductionBMFToILP { target, m, n, k })
    }
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    vec![crate::example_db::specs::RuleExampleSpec {
        id: "bmf_to_ilp",
        build: || {
            // 2x2 identity matrix, rank 2
            let source = BMF::new(vec![vec![true, false], vec![false, true]], 2);
            crate::example_db::specs::rule_example_via_ilp::<_, bool>(source)
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/bmf_ilp.rs"]
mod tests;
