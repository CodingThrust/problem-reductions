//! Reduction from KSatisfiability (3-SAT) to Quadratic Diophantine Equations.
//!
//! This reuses the existing Manders-Adleman 3-SAT -> QuadraticCongruences
//! construction, then converts the bounded congruence witness into an equation
//! of the form x^2 + by = c.

use crate::models::algebraic::{QuadraticCongruences, QuadraticDiophantineEquations};
use crate::models::formula::KSatisfiability;
use crate::reduction;
use crate::rules::ksatisfiability_quadraticcongruences::Reduction3SATToQuadraticCongruences;
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::variant::K3;
use num_bigint::BigUint;
use num_traits::One;

/// Result of reducing 3-SAT to Quadratic Diophantine Equations.
#[derive(Debug, Clone)]
pub struct Reduction3SATToQuadraticDiophantineEquations {
    target: QuadraticDiophantineEquations,
    congruence_reduction: Reduction3SATToQuadraticCongruences,
}

impl ReductionResult for Reduction3SATToQuadraticDiophantineEquations {
    type Source = KSatisfiability<K3>;
    type Target = QuadraticDiophantineEquations;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(
        &self,
        target_solution: &<Self::Target as crate::traits::Problem>::Solution,
    ) -> crate::rules::ExtractionResult<<Self::Source as crate::traits::Problem>::Solution> {
        crate::rules::traits::validate_target_solution(self.target_problem(), target_solution)?;

        Ok({
            self.congruence_reduction
                .extract_solution(target_solution)?
        })
    }
}

fn no_instance() -> QuadraticDiophantineEquations {
    QuadraticDiophantineEquations::new(1u32, 1u32, 1u32)
}

fn translate_congruence(source: &QuadraticCongruences) -> QuadraticDiophantineEquations {
    if source.c() <= &BigUint::one() {
        return no_instance();
    }

    let h = source.c().clone() - BigUint::one();
    let h_squared = &h * &h;
    if h_squared < *source.a() {
        return no_instance();
    }

    let padding = ((&h_squared - source.a()) / source.b()) + BigUint::one();
    let c = source.a() + (source.b() * &padding);

    QuadraticDiophantineEquations::new(BigUint::one(), source.b().clone(), c)
}

#[reduction(
    transform = upper_bound {
        bit_length_a = "1",
        bit_length_b = "64 * (2 * num_clauses + num_vars + 1)^2 + 3 * num_clauses + 4",
        bit_length_c = "128 * (2 * num_clauses + num_vars + 1)^2 + 6 * num_clauses + 9",
    }
)]
impl ReduceTo<QuadraticDiophantineEquations> for KSatisfiability<K3> {
    type Result = Reduction3SATToQuadraticDiophantineEquations;

    fn reduce_to(&self) -> Result<Self::Result, crate::rules::ReductionError> {
        let congruence_reduction = ReduceTo::<QuadraticCongruences>::reduce_to(self)?;
        let target = translate_congruence(congruence_reduction.target_problem());

        Ok(Reduction3SATToQuadraticDiophantineEquations {
            target,
            congruence_reduction,
        })
    }
}

#[cfg(any(test, feature = "example-db"))]
fn canonical_source() -> KSatisfiability<K3> {
    use crate::models::formula::CNFClause;

    KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])])
}

#[cfg(any(test, feature = "example-db"))]
fn canonical_witness() -> BigUint {
    BigUint::parse_bytes(b"3851422232510508672725868082377332726402809", 10)
        .expect("canonical CRT witness must parse")
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::assemble_rule_example;
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_quadraticdiophantineequations",
        build: || {
            let source = canonical_source();
            let reduction = ReduceTo::<QuadraticDiophantineEquations>::reduce_to(&source)
                .expect("reduction should succeed");
            let target_config = canonical_witness();

            assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: serde_json::json!(vec![true, false, false]),
                    target_config: serde_json::to_value(target_config)
                        .expect("solution serialization must succeed"),
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_quadraticdiophantineequations.rs"]
mod tests;
