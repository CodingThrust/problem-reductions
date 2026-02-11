use crate::core::{
    Assignment, Evaluation, LegacyProblemAdapter, LegacyReductionAdapter, ObjectiveDirection,
    ProblemInstance, ProblemSpec, Reduction, VariantDimension,
};
use crate::models::graph::MaximumIndependentSet;
use crate::models::optimization::QUBO;
use crate::rules::ReduceTo;
use crate::topology::SimpleGraph;
use crate::traits::Problem;
use crate::types::ProblemSize;

#[derive(Clone)]
struct DummySource {
    n: usize,
}

#[derive(Clone)]
struct DummyTarget {
    n: usize,
}

impl ProblemSpec for DummySource {
    const NAME: &'static str = "DummySource";
    type Value = i32;

    fn variant_dimensions() -> Vec<VariantDimension> {
        vec![VariantDimension::graph("SimpleGraph")]
    }
}

impl ProblemSpec for DummyTarget {
    const NAME: &'static str = "DummyTarget";
    type Value = i32;

    fn variant_dimensions() -> Vec<VariantDimension> {
        vec![VariantDimension::graph("SimpleGraph")]
    }
}

impl ProblemInstance for DummySource {
    fn num_variables(&self) -> usize {
        self.n
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn size_profile(&self) -> ProblemSize {
        ProblemSize::new(vec![("n", self.n)])
    }

    fn objective_direction(&self) -> ObjectiveDirection {
        ObjectiveDirection::Maximize
    }

    fn evaluate_assignment(&self, assignment: &Assignment) -> Evaluation<Self::Value> {
        let sum = assignment.as_slice().iter().copied().sum::<usize>() as i32;
        Evaluation::feasible(sum)
    }
}

impl ProblemInstance for DummyTarget {
    fn num_variables(&self) -> usize {
        self.n
    }

    fn num_flavors(&self) -> usize {
        2
    }

    fn size_profile(&self) -> ProblemSize {
        ProblemSize::new(vec![("n", self.n)])
    }

    fn objective_direction(&self) -> ObjectiveDirection {
        ObjectiveDirection::Minimize
    }

    fn evaluate_assignment(&self, assignment: &Assignment) -> Evaluation<Self::Value> {
        let sum = assignment.as_slice().iter().copied().sum::<usize>() as i32;
        Evaluation::feasible(sum)
    }
}

#[derive(Clone)]
struct DummyReduction {
    target: DummyTarget,
}

impl Reduction<DummySource, DummyTarget> for DummyReduction {
    fn target_instance(&self) -> &DummyTarget {
        &self.target
    }

    fn project_assignment(&self, target_solution: &Assignment) -> Assignment {
        // Identity projection for contract test.
        target_solution.clone()
    }

    fn source_size_profile(&self) -> ProblemSize {
        ProblemSize::new(vec![("n", self.target.n)])
    }

    fn target_size_profile(&self) -> ProblemSize {
        self.target.size_profile()
    }
}

#[test]
fn test_reduction_projection_round_trip_identity() {
    let reduction = DummyReduction {
        target: DummyTarget { n: 4 },
    };
    let assignment = Assignment::from(vec![1, 0, 1, 0]);
    let projected = reduction.project_assignment(&assignment);
    assert_eq!(projected, assignment);
}

#[test]
fn test_legacy_problem_adapter_matches_legacy_evaluation() {
    let legacy = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let adapter = LegacyProblemAdapter::new(legacy.clone());
    let assignment = Assignment::from(vec![1, 0, 1]);

    let legacy_eval = legacy.solution_size(assignment.as_slice());
    let new_eval = adapter.evaluate_assignment(&assignment);

    assert_eq!(new_eval.objective, legacy_eval.size);
    assert_eq!(new_eval.feasible, legacy_eval.is_valid);
    assert_eq!(
        adapter.objective_direction(),
        ObjectiveDirection::from(legacy.energy_mode())
    );
}

#[test]
fn test_legacy_reduction_adapter_projection_and_sizes() {
    let source = MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2)]);
    let legacy_reduction = ReduceTo::<QUBO<f64>>::reduce_to(&source);
    let adapter = LegacyReductionAdapter::new(legacy_reduction);

    let target_assignment = Assignment::from(vec![1, 0, 1]);
    let projected = adapter.project_assignment(&target_assignment);

    assert_eq!(projected, target_assignment);
    assert_eq!(adapter.source_size_profile(), source.problem_size());
    assert_eq!(
        adapter.target_size_profile(),
        adapter.target_instance().size_profile()
    );
}
