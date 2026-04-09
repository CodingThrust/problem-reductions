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

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        let Some(x) = self.target.decode_witness(target_solution) else {
            return self.congruence_reduction.extract_solution(&[]);
        };

        let Some(congruence_config) = self
            .congruence_reduction
            .target_problem()
            .encode_witness(&x)
        else {
            return self.congruence_reduction.extract_solution(&[]);
        };

        self.congruence_reduction
            .extract_solution(&congruence_config)
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

#[reduction(overhead = {
    bit_length_a = "1",
    bit_length_b = "(num_vars + num_clauses)^2 * log(num_vars + num_clauses + 1)",
    bit_length_c = "(num_vars + num_clauses)^2 * log(num_vars + num_clauses + 1)",
})]
impl ReduceTo<QuadraticDiophantineEquations> for KSatisfiability<K3> {
    type Result = Reduction3SATToQuadraticDiophantineEquations;

    fn reduce_to(&self) -> Self::Result {
        let congruence_reduction = ReduceTo::<QuadraticCongruences>::reduce_to(self);
        let target = translate_congruence(congruence_reduction.target_problem());

        Reduction3SATToQuadraticDiophantineEquations {
            target,
            congruence_reduction,
        }
    }
}

#[cfg(any(test, feature = "example-db"))]
fn canonical_source() -> KSatisfiability<K3> {
    use crate::models::formula::CNFClause;

    KSatisfiability::<K3>::new(3, vec![CNFClause::new(vec![1, 2, 3])])
}

#[cfg(any(test, feature = "example-db"))]
fn canonical_witness() -> BigUint {
    BigUint::parse_bytes(
        b"1751451122102119958305507786775835374858648979796949071929887579732578264063983923970828608254544727567945005331103265320267846420581308180536461678218456421163010842022583797942541569366464959069523226763069748653830351684499364645098951736761394790343553460544021210289436100818494593367113721596780252083857888675004881955664228675079663569835052161564690932502575257394108174870151908279593037426404556490332761276593006398441245490978500647642893471046425509487910796951416870024826654351366508266859321005453091128123256128675758429165869380881549388896022325625404673271432251145796159394173120179999131480837018022329857587128653018300402",
        10,
    )
    .expect("reference witness must parse")
}

#[cfg(feature = "example-db")]
pub(crate) fn canonical_rule_example_specs() -> Vec<crate::example_db::specs::RuleExampleSpec> {
    use crate::example_db::specs::assemble_rule_example;
    use crate::export::SolutionPair;

    vec![crate::example_db::specs::RuleExampleSpec {
        id: "ksatisfiability_to_quadraticdiophantineequations",
        build: || {
            let source = canonical_source();
            let reduction = ReduceTo::<QuadraticDiophantineEquations>::reduce_to(&source);
            let target_config = reduction
                .target_problem()
                .encode_witness(&canonical_witness())
                .expect("reference witness must fit QDE encoding");

            assemble_rule_example(
                &source,
                reduction.target_problem(),
                vec![SolutionPair {
                    source_config: vec![1, 0, 0],
                    target_config,
                }],
            )
        },
    }]
}

#[cfg(test)]
#[path = "../unit_tests/rules/ksatisfiability_quadraticdiophantineequations.rs"]
mod tests;
