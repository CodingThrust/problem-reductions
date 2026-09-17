use super::SolveOutcome;
use crate::traits::{EvaluationError, EvaluationValue, Problem};
use crate::types::{Extremum, Max, Min, Or, ProblemParameters};
use std::cell::Cell;

#[test]
fn reported_evaluations_compare_numeric_values_without_losing_integer_precision() {
    use super::check_reported_evaluation;
    use serde_json::json;

    for reported in ["Min(2)", "Min(2.0)", "Min(2e0)", "Min(2.000000001)"] {
        assert!(check_reported_evaluation(&json!(reported), &Min(Some(2.0f64))).is_ok());
    }
    for reported in [
        "Min(2.001)",
        "Max(2)",
        "Min(NaN)",
        "Min(inf)",
        "Min(abc)",
        "Min(2",
        "Min2)",
    ] {
        assert!(check_reported_evaluation(&json!(reported), &Min(Some(2.0f64))).is_err());
    }
    assert!(check_reported_evaluation(&json!("Min(0.0000000005)"), &Min(Some(0.0f64))).is_ok());
    assert!(check_reported_evaluation(&json!("Max(2e0)"), &Max(Some(2.0f64))).is_ok());
    assert!(
        check_reported_evaluation(&json!("Min(2e0)"), &Extremum::minimize(Some(2.0f64))).is_ok()
    );
    assert!(
        check_reported_evaluation(&json!("Max(2e0)"), &Extremum::maximize(Some(2.0f64))).is_ok()
    );
    assert!(check_reported_evaluation(&json!("Min(+002)"), &Min(Some(2i64))).is_ok());
    assert!(check_reported_evaluation(&json!("Min(3)"), &Min(Some(2i64))).is_err());
    assert!(check_reported_evaluation(
        &json!("Min(9007199254740992)"),
        &Min(Some(9007199254740993i64))
    )
    .is_err());
    assert!(check_reported_evaluation(&json!("Or(true)"), &Or(true)).is_ok());
    assert!(check_reported_evaluation(&json!("Or(false)"), &Or(true)).is_err());
}

#[derive(Clone)]
struct Evaluated<V> {
    value: Result<V, EvaluationError>,
    evaluations: Cell<usize>,
}

impl<V: EvaluationValue> Problem for Evaluated<V> {
    const NAME: &'static str = "Evaluated";
    type Solution = ();
    type Value = V;

    fn parameter_names() -> &'static [&'static str] {
        &[]
    }
    fn parameters(&self) -> ProblemParameters {
        ProblemParameters::default()
    }
    fn variant() -> Vec<(&'static str, &'static str)> {
        vec![]
    }
    fn evaluate(&self, _: &()) -> Result<V, EvaluationError> {
        self.evaluations.set(self.evaluations.get() + 1);
        self.value.clone()
    }
}

fn check_candidate<V: EvaluationValue + std::fmt::Debug + PartialEq>(value: V, valid: bool) {
    let problem = Evaluated {
        value: Ok(value.clone()),
        evaluations: Cell::new(0),
    };
    let optimal = SolveOutcome::optimal(&problem, ());
    assert_eq!(problem.evaluations.get(), 1);
    let feasible = SolveOutcome::feasible(&problem, ());
    assert_eq!(problem.evaluations.get(), 2);
    if valid {
        assert_eq!(
            optimal,
            Ok(SolveOutcome::Optimal {
                solution: (),
                evaluation: value.clone()
            })
        );
        assert_eq!(
            feasible,
            Ok(SolveOutcome::Feasible {
                solution: (),
                evaluation: value
            })
        );
    } else {
        assert_eq!(optimal, Err(EvaluationError::ConstraintViolation));
        assert_eq!(feasible, Err(EvaluationError::ConstraintViolation));
    }
}

#[test]
fn constructors_validate_candidate_values_with_one_evaluation() {
    check_candidate(Min(Some(0)), true);
    check_candidate(Min::<i64>(None), false);
    check_candidate(Max(Some(-1)), true);
    check_candidate(Max::<i64>(None), false);
    check_candidate(Or(true), true);
    check_candidate(Or(false), false);
    check_candidate(Extremum::minimize(Some(0)), true);
    check_candidate(Extremum::<i64>::minimize(None), false);
    check_candidate(Extremum::maximize(Some(-1)), true);
    check_candidate(Extremum::<i64>::maximize(None), false);
}

#[test]
fn constructors_preserve_evaluation_failures() {
    for error in [
        EvaluationError::InvalidConfiguration("wrong solution length".into()),
        EvaluationError::IntegerOverflow("summing weights".into()),
    ] {
        let problem = Evaluated::<Min<i64>> {
            value: Err(error.clone()),
            evaluations: Cell::new(0),
        };
        assert_eq!(SolveOutcome::optimal(&problem, ()), Err(error.clone()));
        assert_eq!(SolveOutcome::feasible(&problem, ()), Err(error));
        assert_eq!(problem.evaluations.get(), 2);
    }
}

#[test]
fn invalid_candidate_does_not_establish_problem_infeasibility() {
    use crate::models::graph::MinimumVertexCover;
    use crate::topology::SimpleGraph;

    let problem = MinimumVertexCover::new(SimpleGraph::new(2, vec![(0, 1)]), vec![1i64; 2]);
    let invalid = vec![false, false];
    assert_eq!(problem.evaluate(&invalid).unwrap(), Min(None));
    assert_eq!(
        SolveOutcome::feasible(&problem, invalid.clone()),
        Err(EvaluationError::ConstraintViolation),
    );
    assert_eq!(
        SolveOutcome::optimal(&problem, invalid),
        Err(EvaluationError::ConstraintViolation),
    );
    assert_eq!(
        SolveOutcome::feasible(&problem, vec![true, false]).unwrap(),
        SolveOutcome::Feasible {
            solution: vec![true, false],
            evaluation: Min(Some(1)),
        },
    );
}

#[test]
fn result_serialization_rejects_invalid_stored_evaluations() {
    for outcome in [
        SolveOutcome::Optimal {
            solution: (),
            evaluation: Or(false),
        },
        SolveOutcome::Feasible {
            solution: (),
            evaluation: Or(false),
        },
    ] {
        assert!(matches!(
            super::outcome_to_json(&outcome),
            Err(crate::rules::ExtractionError::Evaluation(
                EvaluationError::ConstraintViolation
            )),
        ));
    }
}
