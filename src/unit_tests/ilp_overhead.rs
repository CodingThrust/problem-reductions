//! Numeric parameter contracts and composed ILP/QUBO workflows.
use crate::models::algebraic::{Bounded, ILP, QUBO};
use crate::models::graph::*;
use crate::models::misc::*;
use crate::parameters::ParameterRelation;
use crate::rules::{ReduceTo, ReductionGraph, ReductionPath, ReductionResult, ReductionStep};
use crate::solvers::{BruteForce, BruteForceProblem, ILPSolver};
use crate::topology::{DirectedGraph, SimpleGraph};
use crate::{
    types::{Min, One},
    Problem,
};

type BoundedILP = ILP<i64, i64, Bounded>;

// Expected counts below come from enumerated rows/segments, independently of
// the registered expressions. Check predictions as well as the real targets.
fn check_counts<S: Problem + ReduceTo<T>, T: Problem>(source: &S, counts: &[(&str, u64, u64)]) {
    check_contract::<S, T>(source);
    let reduction = source.reduce_to().unwrap();
    let actual = reduction.target_problem().parameters();
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|e| {
            e.source_name == S::NAME
                && e.target_name == T::NAME
                && e.source_variant() == S::variant()
                && e.target_variant() == T::variant()
        })
        .unwrap();
    let predicted = entry
        .parameter_contract()
        .unwrap()
        .transform()
        .unwrap()
        .evaluate(&source.parameters())
        .unwrap();
    for &(field, measured, bound) in counts {
        assert_eq!(
            actual.get(field),
            Some(measured),
            "{}: actual {field}",
            S::NAME
        );
        assert_eq!(
            predicted.get(field),
            Some(bound),
            "{}: predicted {field}",
            S::NAME
        );
    }
}

#[test]
fn partition_gadgets_count_sparse_rows_on_the_accepted_domain() {
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|e| e.source_name == "PartitionIntoTriangles" && e.target_name == "ILP")
        .unwrap();
    assert_eq!(
        entry
            .parameter_contract()
            .unwrap()
            .transform()
            .unwrap()
            .relation("num_vars"),
        Some(ParameterRelation::Exact)
    );
    check_counts::<_, ILP<bool>>(
        &PartitionIntoTriangles::new(SimpleGraph::complete(3)),
        &[
            ("num_vars", 3, 3),
            ("num_constraints", 4, 7),
            ("num_nonzeros", 6, 12),
        ],
    );
    check_counts::<_, ILP<bool>>(
        &PartitionIntoTriangles::new(SimpleGraph::empty(6)),
        &[
            ("num_vars", 12, 12),
            ("num_constraints", 38, 38),
            ("num_nonzeros", 84, 84),
        ],
    );
    check_counts::<_, ILP<bool>>(
        &PartitionIntoPathsOfLength2::new(SimpleGraph::path(3)),
        &[
            ("num_vars", 5, 5),
            ("num_constraints", 11, 11),
            ("num_nonzeros", 22, 22),
        ],
    );
    check_counts::<_, ILP<bool>>(
        &PartitionIntoCliques::new(SimpleGraph::empty(3), 2),
        &[("num_constraints", 9, 12), ("num_nonzeros", 18, 27)],
    );
    for n in [0, 3] {
        check_contract::<_, ILP<bool>>(&PartitionIntoTriangles::new(SimpleGraph::empty(n)));
        check_contract::<_, ILP<bool>>(&PartitionIntoPathsOfLength2::new(SimpleGraph::empty(n)));
    }
    // Loops never become product variables; parallel orientations merge.
    check_contract::<_, ILP<bool>>(&PartitionIntoPathsOfLength2::new(SimpleGraph::new(
        3,
        vec![(0, 0), (0, 1), (1, 0), (1, 2)],
    )));
}

#[test]
fn macro_segments_count_endpoint_and_matching_terms() {
    // Three literals and three non-overlapping one-character references.
    check_counts::<_, ILP<bool>>(
        &MinimumInternalMacroDataCompression::new(1, vec![0, 0, 0], 1),
        &[("num_vars", 6, 6), ("num_nonzeros", 12, 12)],
    );
    for (string, vars, terms) in [(vec![], 0, 0), (vec![0, 1], 2, 4), (vec![0, 0], 3, 6)] {
        check_counts::<_, ILP<bool>>(
            &MinimumInternalMacroDataCompression::new(2, string, 2),
            &[
                ("num_vars", vars, if vars == 0 { 0 } else { 3 }),
                ("num_nonzeros", terms, if vars == 0 { 0 } else { 6 }),
            ],
        );
    }
    // n=2: five pointers, six pointer-character matches, four dictionary bits.
    check_counts::<_, ILP<bool>>(
        &MinimumExternalMacroDataCompression::new(2, vec![0, 1], 2),
        &[
            ("num_vars", 13, 13),
            ("num_constraints", 16, 16),
            ("num_nonzeros", 40, 42),
        ],
    );
    check_counts::<_, ILP<bool>>(
        &MinimumExternalMacroDataCompression::new(0, vec![], 2),
        &[
            ("num_vars", 0, 0),
            ("num_constraints", 0, 0),
            ("num_nonzeros", 0, 0),
        ],
    );
}

#[test]
fn triangle_strengthening_counts_cliques_without_dense_rows() {
    // K5: ten triangle pairs and five degree rows, no opposite-edge equalities.
    check_counts::<_, ILP<bool>>(
        &MonochromaticTriangle::new(SimpleGraph::complete(5)),
        &[
            ("num_vars", 10, 10),
            ("num_constraints", 25, 35),
            ("num_nonzeros", 80, 100),
        ],
    );
    // K6: 20 triangles, six K5s, and three opposite edges per base triangle.
    check_counts::<_, ILP<bool>>(
        &MonochromaticTriangle::new(SimpleGraph::complete(6)),
        &[("num_constraints", 110, 130), ("num_nonzeros", 320, 360)],
    );
    for n in 0..5 {
        check_contract::<_, ILP<bool>>(&MonochromaticTriangle::new(SimpleGraph::empty(n)));
    }
}

#[test]
fn rectangle_bounds_cover_empty_shapes_and_normalized_budgets() {
    for bound in [i64::MIN, 0, 1, i64::MAX] {
        // One maximal 2x2 rectangle: four cell rows and one budget row.
        check_counts::<_, ILP<bool>>(
            &RectilinearPictureCompression::new(vec![vec![true; 2]; 2], bound),
            &[
                ("num_vars", 1, 1),
                ("num_nonzeros", 5, 5),
                ("max_constraint_magnitude_bits", 1, 4),
            ],
        );
    }
    for matrix in [vec![vec![false]], vec![vec![false; 2]; 2]] {
        check_counts::<_, ILP<bool>>(
            &RectilinearPictureCompression::new(matrix, i64::MIN),
            &[("num_vars", 0, 0), ("num_nonzeros", 0, 0)],
        );
    }
    // The two maximal dominoes overlap at the top-left cell: four incidences,
    // rather than three distinct true cells, and two budget coefficients.
    check_counts::<_, ILP<bool>>(
        &RectilinearPictureCompression::new(vec![vec![true, true], vec![true, false]], 2),
        &[("num_vars", 2, 2), ("num_nonzeros", 6, 6)],
    );
    check_counts::<_, ILP<bool>>(
        &RectilinearPictureCompression::new(vec![vec![true, false, true]], 2),
        &[("num_vars", 2, 2), ("num_nonzeros", 4, 4)],
    );
}

#[test]
fn nae_support_ignores_repeated_literals_and_bounds_cancellation() {
    use crate::models::formula::{CNFClause, NAESatisfiability, Satisfiability};
    for (literals, variables, terms) in [
        (vec![1, 2], 2, 4),
        (vec![1, 1, 1, 1], 1, 2),
        (vec![1, -1, 2, 2], 2, 2),
        (vec![1, -1], 1, 0),
    ] {
        let source = NAESatisfiability::new(4, vec![CNFClause::new(literals)]);
        assert_eq!(
            source.parameters().get("num_clause_variables"),
            Some(variables)
        );
        check_counts::<_, ILP<bool>>(&source, &[("num_nonzeros", terms, 2 * variables)]);
    }
    check_counts::<_, ILP<bool>>(
        &NAESatisfiability::new(0, vec![]),
        &[("num_nonzeros", 0, 0)],
    );
    for clauses in [
        vec![CNFClause::new(vec![])],
        vec![CNFClause::new(vec![1, 1])],
    ] {
        check_contract::<_, NAESatisfiability>(&Satisfiability::new(1, clauses));
    }
}

#[test]
fn timetable_counts_core_edges_and_their_available_colors() {
    use crate::models::formula::{CNFClause, KSatisfiability};
    use crate::variant::K3;
    let source = KSatisfiability::<K3>::new_allow_less(
        1,
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
    );
    // Six two-list variable edges each supply three edges with two colors;
    // the two singleton clause edges each supply one available assignment.
    check_counts::<_, TimetableDesign>(
        &source,
        &[
            ("num_nonzero_requirements", 20, 48),
            ("num_available_assignments", 38, 96),
            ("period_count_bits", 3, 5),
            ("num_craftsmen", 10, 23),
            ("num_tasks", 10, 24),
        ],
    );
    check_contract::<_, TimetableDesign>(&KSatisfiability::<K3>::new_allow_less(0, vec![]));
    check_contract::<_, TimetableDesign>(&KSatisfiability::<K3>::new_allow_less(
        0,
        vec![CNFClause::new(vec![])],
    ));
}

#[test]
fn lattice_bounds_count_bits_and_off_diagonal_terms() {
    use crate::models::algebraic::ClosestVectorProblem;
    // Determinant four, largest complementary squared norm five, and rounded
    // residual squared at most four: coefficient width <=sqrt(5)<3, or two bits.
    check_counts::<_, QUBO<i64>>(
        &ClosestVectorProblem::new(vec![vec![2, 0], vec![1, 2]], vec![3, 1]).unwrap(),
        &[("num_vars", 1, 4), ("num_quadratic_terms", 0, 8)],
    );
    check_counts::<_, QUBO<i64>>(
        &ClosestVectorProblem::new(vec![], vec![]).unwrap(),
        &[("num_vars", 0, 0), ("num_quadratic_terms", 0, 0)],
    );
}

#[test]
fn lattice_geometry_bounds_preserve_ambient_residuals_and_cancellation() {
    use crate::models::algebraic::ClosestVectorProblem;
    // An unspanned residual of length two permits coefficients -2..=2.
    // Three bits encode that range; the two orthogonal blocks each have three
    // interactions, and every interaction between the blocks cancels.
    let source =
        ClosestVectorProblem::new(vec![vec![1, 0, 0], vec![0, 1, 0]], vec![0, 0, 2]).unwrap();
    assert_eq!(source.parameters().get("coefficient_bound_bits"), Some(3));
    check_counts::<_, QUBO<i64>>(
        &source,
        &[("num_vars", 6, 6), ("num_quadratic_terms", 6, 18)],
    );
    // A one-bit interval exercises the final ceiling of the off-diagonal cap.
    check_counts::<_, QUBO<i64>>(
        &ClosestVectorProblem::new(vec![vec![2]], vec![1]).unwrap(),
        &[("num_vars", 1, 1), ("num_quadratic_terms", 0, 1)],
    );
    check_counts::<_, QUBO<i64>>(
        &ClosestVectorProblem::new(vec![vec![2]], vec![0]).unwrap(),
        &[("num_vars", 0, 0), ("num_quadratic_terms", 0, 0)],
    );
}

#[test]
fn kings_mapping_bounds_count_wires_and_gadget_edges() {
    use crate::topology::KingsSubgraph;
    check_counts::<_, MaximumIndependentSet<KingsSubgraph, One>>(
        &MaximumIndependentSet::new(SimpleGraph::empty(1), vec![One]),
        &[("num_vertices", 1, 5), ("num_edges", 0, 11)],
    );
    // Two adjacent source vertices simplify to a single target edge.
    check_counts::<_, MaximumIndependentSet<KingsSubgraph, One>>(
        &MaximumIndependentSet::new(SimpleGraph::new(2, vec![(0, 1)]), vec![One; 2]),
        &[("num_vertices", 2, 20), ("num_edges", 1, 44)],
    );
}

#[test]
fn rooted_tree_bounds_count_each_sparse_gadget() {
    use crate::models::set::RootedTreeStorageAssignment;
    // Two vertices contribute 82 terms; the two-element subset contributes
    // 112 gadget terms and one budget term. Dropping -n gives excess two.
    check_counts::<_, BoundedILP>(
        &RootedTreeStorageAssignment::new(2, vec![vec![0, 1]], 1),
        &[
            ("num_nonzeros", 195, 197),
            ("max_constraint_magnitude_bits", 2, 4),
        ],
    );
    check_contract::<_, BoundedILP>(&RootedTreeStorageAssignment::new(0, vec![], i64::MIN));
    check_contract::<_, BoundedILP>(&RootedTreeStorageAssignment::new(
        1,
        vec![vec![], vec![0]],
        i64::MAX,
    ));
}

#[test]
fn sparse_matrix_and_schedule_rows_count_conflicting_pairs() {
    use crate::models::algebraic::SparseMatrixCompression;
    check_counts::<_, ILP<bool>>(
        &SparseMatrixCompression::new(vec![vec![true], vec![true]], 2),
        &[
            ("num_vars", 4, 4),
            ("num_constraints", 4, 4),
            ("num_nonzeros", 8, 8),
        ],
    );
    check_counts::<_, ILP<bool>>(
        &SequencingWithinIntervals::new(vec![0, 0], vec![2, 2], vec![1, 1]).unwrap(),
        &[
            ("num_vars", 4, 4),
            ("num_constraints", 4, 8),
            ("num_nonzeros", 8, 16),
        ],
    );
}

#[test]
fn flow_and_tour_bounds_include_sparse_global_rows() {
    check_counts::<_, BoundedILP>(
        &BoundedComponentSpanningForest::new(SimpleGraph::path(2), vec![1, 1], 1, 2),
        &[("num_nonzeros", 52, 52)],
    );
    check_contract::<_, BoundedILP>(&BoundedComponentSpanningForest::new(
        SimpleGraph::new(1, vec![(0, 0)]),
        vec![0],
        1,
        1,
    ));
    check_counts::<_, ILP<bool>>(
        &StackerCrane::new(3, vec![(0, 1), (2, 0)], vec![(1, 2)], vec![1, 1], vec![1]),
        &[("num_nonzeros", 64, 72)],
    );
    check_counts::<_, ILP<bool>>(
        &BottleneckTravelingSalesman::new(SimpleGraph::path(2), vec![1]),
        &[("num_nonzeros", 52, 52)],
    );
}

#[test]
fn string_state_rows_count_terms_instead_of_dense_entries() {
    check_counts::<_, ILP<bool>>(
        &ShortestCommonSupersequence::new(2, vec![vec![0, 1], vec![1, 0]]),
        &[
            ("num_vars", 28, 28),
            ("num_nonzeros", 78, 100),
            ("max_constraint_magnitude_bits", 2, 3),
        ],
    );
    check_counts::<_, ILP<bool>>(
        &StringToStringCorrection::new(1, vec![0], vec![0], 1),
        &[("num_nonzeros", 21, 41)],
    );
    check_counts::<_, ILP<bool>>(
        &StringToStringCorrection::new(0, vec![], vec![], 2),
        &[("num_nonzeros", 2, 2)],
    );
    check_contract::<_, ILP<bool>>(&StringToStringCorrection::new(1, vec![], vec![0], 2));
}

#[test]
fn ensemble_terms_include_ordering_and_target_matches() {
    // Two operands, one operation, one target: 1 activity bound, four selector
    // terms + two activity terms, five ordering terms, ten union/disjoint
    // terms, and eight match terms.
    check_counts::<_, ILP<bool>>(
        &EnsembleComputation::new(2, vec![vec![0, 1]], 1),
        &[("num_nonzeros", 30, 30)],
    );
    check_counts::<_, ILP<bool>>(
        &EnsembleComputation::new(0, vec![], 1),
        &[("num_nonzeros", 4, 4)],
    );
}

#[test]
fn changed_predictions_bound_real_intermediate_targets() {
    use crate::models::formula::{CNFClause, KSatisfiability, NAESatisfiability, Satisfiability};
    use crate::variant::K3;
    let source = KSatisfiability::<K3>::new_allow_less(
        1,
        vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
    );
    for intermediate in [
        step::<TimetableDesign>(),
        step::<MonochromaticTriangle<SimpleGraph>>(),
    ] {
        let first = ReductionPath {
            steps: vec![step::<KSatisfiability<K3>>(), intermediate],
        };
        let mut second = first.clone();
        second.steps.push(step::<ILP<bool>>());
        let graph = ReductionGraph::new();
        for path in [first, second] {
            let chain = graph.reduce_along_path(&path, &source).unwrap().unwrap();
            let predicted = graph
                .compose_path_parameter_transform(&path)
                .unwrap()
                .unwrap()
                .evaluate(&source.parameters())
                .unwrap();
            let target = path.steps.last().unwrap();
            let actual = ReductionGraph::compute_problem_parameters(
                &target.name,
                &target.variant,
                chain.target_problem_any(),
            );
            for (field, value) in actual.iter() {
                assert!(
                    predicted.get(field).expect(field) >= value,
                    "{path:?}: {field}"
                );
            }
        }
    }
    // Includes empty clauses, repeated literals and a fresh sentinel through
    // the new incoming support contract and a real solve/extract workflow.
    for literals in [vec![], vec![1, 1], vec![1, -1]] {
        check_path(
            Satisfiability::new(1, vec![CNFClause::new(literals)]),
            ReductionPath {
                steps: vec![
                    step::<Satisfiability>(),
                    step::<NAESatisfiability>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

#[test]
fn circuit_support_counts_constant_leaves_and_xor_folds() {
    use crate::models::formula::{Assignment, BooleanExpr, Circuit, CircuitSAT};
    let source = CircuitSAT::new(Circuit::new(vec![Assignment::new(
        vec!["out".into()],
        BooleanExpr::xor(vec![BooleanExpr::constant(false); 32]),
    )]));
    // 32 one-term constant rows, 31 four-row XORs of three terms, one output link.
    check_counts::<_, ILP<bool>>(
        &source,
        &[
            ("num_nonzeros", 406, 419),
            ("max_constraint_magnitude_bits", 2, 19),
        ],
    );
    for expression in [
        BooleanExpr::and(vec![]),
        BooleanExpr::or(vec![]),
        BooleanExpr::xor(vec![]),
        BooleanExpr::xor(vec![BooleanExpr::var("out"), BooleanExpr::var("out")]),
    ] {
        check_contract::<_, ILP<bool>>(&CircuitSAT::new(Circuit::new(vec![Assignment::new(
            vec!["out".into()],
            expression,
        )])));
    }
}

#[test]
fn graph_mapping_rows_count_non_edges() {
    check_counts::<_, ILP<bool>>(
        &IsomorphicSpanningTree::new(SimpleGraph::path(3), SimpleGraph::path(3)),
        &[("num_constraints", 10, 18), ("num_nonzeros", 26, 42)],
    );
    use crate::topology::BipartiteGraph;
    check_counts::<_, ILP<bool>>(
        &BalancedCompleteBipartiteSubgraph::new(BipartiteGraph::new(2, 2, vec![]), 2),
        &[
            ("num_constraints", 6, 6),
            ("num_nonzeros", 12, 12),
            ("max_constraint_magnitude_bits", 2, 2),
        ],
    );
}

#[test]
fn homologous_flow_support_counts_repeated_pairs() {
    // Accepted duplicate pair declarations each emit their own equality row;
    // pair count cannot be bounded by a function of arc/vertex counts alone.
    let source = IntegralFlowHomologousArcs::new(
        DirectedGraph::new(2, vec![(0, 1), (0, 1)]),
        vec![1, 1],
        0,
        1,
        1,
        vec![(0, 1); 10],
    );
    check_counts::<_, BoundedILP>(
        &source,
        &[("num_constraints", 13, 14), ("num_nonzeros", 24, 26)],
    );
    assert_eq!(source.parameters().get("num_homologous_pairs"), Some(10));
    // All terms in a (self,self) homologous equality cancel.
    let source = IntegralFlowHomologousArcs::new(
        DirectedGraph::new(1, vec![(0, 0)]),
        vec![i64::MAX],
        0,
        0,
        i64::MAX,
        vec![(0, 0); 10],
    );
    assert_eq!(source.parameters().get("max_capacity_bits"), Some(63));
    check_counts::<_, BoundedILP>(&source, &[("max_constraint_magnitude_bits", 63, 65)]);
    use crate::models::formula::{CNFClause, Satisfiability};
    let source = Satisfiability::new(1, vec![CNFClause::new(vec![1, 1, -1])]);
    check_counts::<_, IntegralFlowHomologousArcs>(
        &source,
        &[("num_homologous_pairs", 2, 3), ("max_capacity_bits", 1, 4)],
    );
    for clauses in [
        vec![],
        vec![CNFClause::new(vec![1])],
        vec![CNFClause::new(vec![])],
    ] {
        check_path(
            Satisfiability::new(1, clauses),
            ReductionPath {
                steps: vec![
                    step::<Satisfiability>(),
                    step::<IntegralFlowHomologousArcs>(),
                    step::<BoundedILP>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

#[test]
fn labelled_graph_products_count_sparse_support() {
    use crate::models::graph::{LabelledArc, LabelledDigraph};
    for (n, arc, terms) in [
        (2, LabelledArc::new(0, 0, 1), 15),
        (1, LabelledArc::new(0, 0, 0), 8),
    ] {
        let graph = LabelledDigraph::new(n, vec![arc]);
        check_counts::<_, ILP<bool>>(
            &MaximumCommonEdgeSubgraph::new(graph.clone(), graph),
            &[("num_nonzeros", terms, 2 * n as u64 * n as u64 + 7)],
        );
    }
    check_contract::<_, ILP<bool>>(&MaximumCommonEdgeSubgraph::new(
        LabelledDigraph::new(0, vec![]),
        LabelledDigraph::new(2, vec![]),
    ));
}

#[test]
fn sentinel_literal_pairs_count_each_clause_once() {
    use crate::models::formula::{CNFClause, NAESatisfiability, Satisfiability};
    check_counts::<_, NAESatisfiability>(
        &Satisfiability::new(
            1,
            vec![CNFClause::new(vec![]), CNFClause::new(vec![1, 1, -1])],
        ),
        &[("num_literal_pairs", 7, 8)],
    );
}

#[test]
fn unit_schedule_bits_and_precedences_count_layers() {
    use crate::models::formula::{CNFClause, KSatisfiability};
    use crate::variant::K3;
    // Capacities [1,3,3,6], seven processors, filler sizes [6,4,4,1].
    // 4 variable links + 21 literal links + 24+16+4 filler links.
    check_counts::<_, PreemptiveScheduling>(
        &KSatisfiability::<K3>::new_allow_less(1, vec![CNFClause::new(vec![1])]),
        &[
            ("num_precedences", 69, 388),
            ("max_schedule_magnitude_bits", 5, 8),
        ],
    );
    for clauses in [vec![], vec![CNFClause::new(vec![])]] {
        check_contract::<_, PreemptiveScheduling>(&KSatisfiability::<K3>::new_allow_less(
            0, clauses,
        ));
    }
}

#[test]
fn numeric_source_sums_make_raw_target_predictions_available() {
    use crate::models::Decision;
    for (sizes, sum, capacity, horizon) in [
        (vec![1], 1u64, 0, 3),
        (vec![1, 2], 3, 1, 12),
        (vec![1, 3], 4, 2, 18),
    ] {
        let source = Partition::new(sizes).unwrap();
        check_counts::<_, Knapsack>(&source, &[("capacity", capacity, sum.div_ceil(2))]);
        check_counts::<_, Decision<OpenShopScheduling>>(
            &source,
            &[("schedule_horizon", horizon, (9 * sum).div_ceil(2))],
        );
        check_counts::<_, IntegralFlowWithMultipliers>(
            &source,
            &[("max_capacity", if sum % 2 == 1 { 1 } else { 3 }, sum)],
        );
        check_counts::<_, ProductionPlanning>(
            &source,
            &[(
                "max_capacity",
                if sum == 1 {
                    1
                } else if sum == 3 {
                    2
                } else {
                    3
                },
                sum,
            )],
        );
        assert_eq!(source.parameters().get("total_sum"), Some(sum));
    }
    let source = ThreePartition::new(vec![1; 3], 3);
    check_counts::<_, SequencingWithReleaseTimesAndDeadlines>(&source, &[("time_horizon", 3, 3)]);
    assert_eq!(source.parameters().get("bound"), Some(3));
    use crate::models::set::ThreeDimensionalMatching;
    for source in [
        ThreeDimensionalMatching::new(0, vec![]),
        ThreeDimensionalMatching::new(1, vec![]),
        ThreeDimensionalMatching::new(1, vec![(0, 0, 0)]),
    ] {
        check_contract::<_, ThreePartition>(&source);
        let path = ReductionPath {
            steps: vec![
                step::<ThreeDimensionalMatching>(),
                step::<ThreePartition>(),
                step::<SequencingWithReleaseTimesAndDeadlines>(),
            ],
        };
        let predicted = ReductionGraph::new()
            .compose_path_parameter_transform(&path)
            .unwrap()
            .unwrap()
            .evaluate(&source.parameters())
            .unwrap();
        let intermediate = ReduceTo::<ThreePartition>::reduce_to(&source).unwrap();
        let target = ReduceTo::<SequencingWithReleaseTimesAndDeadlines>::reduce_to(
            intermediate.target_problem(),
        )
        .unwrap();
        assert!(
            predicted.get("time_horizon").unwrap()
                >= target
                    .target_problem()
                    .parameters()
                    .get("time_horizon")
                    .unwrap()
        );
    }
}

#[test]
fn hamiltonian_edge_bound_includes_normalization_and_fixed_outputs() {
    use crate::models::Decision;
    // Two 14-edge gadgets, one incident chain link, six selector links.
    check_counts::<_, HamiltonianCircuit<SimpleGraph>>(
        &Decision::new(
            MinimumVertexCover::new(SimpleGraph::path(3), vec![One; 3]),
            1,
        ),
        &[("num_edges", 35, 53)],
    );
    for bound in [-1, 0, 3] {
        check_contract::<_, HamiltonianCircuit<SimpleGraph>>(&Decision::new(
            MinimumVertexCover::new(
                SimpleGraph::new(3, vec![(0, 0), (0, 1), (1, 0)]),
                vec![One; 3],
            ),
            bound,
        ));
    }
}

#[test]
fn integer_knapsack_and_open_shop_bit_predictions_reach_qubo() {
    use crate::models::{set::IntegerKnapsack, Decision};
    for capacity in [0, 2, 5] {
        check_qubo::<_, BoundedILP>(
            IntegerKnapsack::new(vec![2, 3], vec![3, 5], capacity).unwrap(),
        );
    }
    let source = OpenShopScheduling::new(1, vec![vec![2]]);
    check_qubo::<_, BoundedILP>(source.clone());
    for bound in [1, 2] {
        check_qubo::<_, BoundedILP>(Decision::new(source.clone(), bound));
    }
}

#[test]
fn partition_open_shop_predictions_preserve_ilp_solution_recovery() {
    use crate::models::Decision;
    let graph = ReductionGraph::new();
    let path = ReductionPath {
        steps: vec![
            step::<Partition>(),
            step::<Decision<OpenShopScheduling>>(),
            step::<BoundedILP>(),
        ],
    };
    for (sizes, feasible) in [(vec![1], false), (vec![1, 1], true), (vec![1, 3], false)] {
        let source = Partition::new(sizes).unwrap();
        let predicted = graph
            .compose_path_parameter_transform(&path)
            .unwrap()
            .unwrap()
            .evaluate(&source.parameters())
            .unwrap();
        let chain = graph.reduce_along_path(&path, &source).unwrap().unwrap();
        let target = chain.target_problem::<BoundedILP>();
        for (field, actual) in target.parameters().iter() {
            assert!(predicted.get(field).expect(field) >= actual);
        }
        match ILPSolver::new().solve(target) {
            Ok(solution) => {
                assert!(feasible);
                let recovered = chain.extract_solution::<Vec<bool>, _>(&solution).unwrap();
                assert!(source.evaluate(&recovered).unwrap().0);
            }
            Err(crate::solvers::ILPSolveError::Infeasible) => assert!(!feasible),
            Err(error) => panic!("{error}"),
        }
    }
}

#[test]
fn partition_knapsack_qubo_predictions_and_solution_recovery() {
    for sizes in [vec![1], vec![1, 1], vec![1, 2], vec![1, 3]] {
        for through_ilp in [false, true] {
            let mut steps = vec![step::<Partition>(), step::<Knapsack>()];
            if through_ilp {
                steps.push(step::<ILP<bool>>());
            }
            steps.push(step::<QUBO<i64>>());
            check_path(
                Partition::new(sizes.clone()).unwrap(),
                ReductionPath { steps },
            );
        }
    }
    let path = ReductionPath {
        steps: vec![step::<Partition>(), step::<Knapsack>(), step::<QUBO<i64>>()],
    };
    let source = Partition::new(vec![1 << 19; 10]).unwrap();
    let predicted = ReductionGraph::new()
        .compose_path_parameter_transform(&path)
        .unwrap()
        .unwrap()
        .evaluate(&source.parameters())
        .unwrap();
    assert!(predicted.get("num_vars").unwrap() <= 40);
    assert!(predicted.get("num_quadratic_terms").unwrap() <= 1600);
}

#[test]
fn subset_sum_lattice_qubo_predictions_and_solution_recovery() {
    use crate::models::{algebraic::ClosestVectorProblem, Decision};
    for (sizes, target) in [(vec![1], 1), (vec![2], 1)] {
        let source = SubsetSum::new(sizes.clone(), target);
        // The carry lattices have unit selected determinants. Squared column
        // norms (3), then (3,5), and squared target norm two bound widths by
        // floor(sqrt(8)) and floor(sqrt(40)), respectively.
        let (actual_bits, predicted_bits) = if sizes[0] == 1 { (2, 4) } else { (3, 7) };
        check_counts::<_, Decision<ClosestVectorProblem>>(
            &source,
            &[("coefficient_bound_bits", actual_bits, predicted_bits)],
        );
        check_path(
            source,
            ReductionPath {
                steps: vec![
                    step::<SubsetSum>(),
                    step::<Decision<ClosestVectorProblem>>(),
                    step::<ClosestVectorProblem>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

#[test]
fn factoring_circuit_sat_qubo_predictions_and_solution_recovery() {
    use crate::models::formula::{CircuitSAT, NAESatisfiability, Satisfiability};
    for target in [1u32, 3] {
        check_path(
            Factoring::with_factor_bits(target, 1, 1),
            ReductionPath {
                steps: vec![
                    step::<Factoring>(),
                    step::<CircuitSAT>(),
                    step::<Satisfiability>(),
                    step::<NAESatisfiability>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

fn check_contract<S: Problem + ReduceTo<T>, T: Problem>(source: &S) {
    let target = source.reduce_to().unwrap();
    let entry = crate::rules::registry::reduction_entries()
        .into_iter()
        .find(|entry| {
            entry.source_name == S::NAME
                && entry.target_name == T::NAME
                && entry.source_variant() == S::variant()
                && entry.target_variant() == T::variant()
        })
        .unwrap();
    let contract = entry.parameter_contract().unwrap();
    assert!(
        contract.unavailable().is_empty(),
        "{} -> {}",
        S::NAME,
        T::NAME
    );
    let transform = contract.transform().unwrap();
    let predicted = transform.evaluate(&source.parameters()).unwrap();
    for (field, actual) in target.target_problem().parameters().iter() {
        let prediction = predicted.get(field).expect(field);
        match transform.relation(field).unwrap() {
            ParameterRelation::Exact => assert_eq!(prediction, actual, "{}: {field}", S::NAME),
            ParameterRelation::UpperBound => assert!(
                prediction >= actual,
                "{}: {field}: {prediction} < {actual}",
                S::NAME
            ),
        }
    }
}

fn step<P: Problem>() -> ReductionStep {
    ReductionStep {
        name: P::NAME.into(),
        variant: ReductionGraph::variant_to_map(&P::variant()),
    }
}

fn check_qubo<S, T>(source: S)
where
    S: BruteForceProblem + ReduceTo<T> + 'static,
    S::Solution: 'static,
    S::Value: PartialEq + std::fmt::Debug + crate::types::SolutionAggregate + 'static,
    T: Problem,
{
    check_contract::<S, T>(&source);
    let mut steps = vec![step::<S>(), step::<T>()];
    if T::variant() != ILP::<bool>::variant() {
        steps.push(step::<ILP<bool>>());
    }
    steps.push(step::<QUBO<i64>>());
    check_path(source, ReductionPath { steps });
}

fn check_path<S>(source: S, path: ReductionPath)
where
    S: BruteForceProblem + 'static,
    S::Solution: 'static,
    S::Value: PartialEq + std::fmt::Debug + crate::types::SolutionAggregate + 'static,
{
    let graph = ReductionGraph::new();
    let predicted = graph
        .compose_path_parameter_transform(&path)
        .unwrap()
        .unwrap()
        .evaluate(&source.parameters())
        .unwrap();
    let chain = graph.reduce_along_path(&path, &source).unwrap().unwrap();
    let target = chain.target_problem::<QUBO<i64>>();
    for (field, actual) in target.parameters().iter() {
        assert!(
            predicted.get(field).expect(field) >= actual,
            "{}: {field}",
            S::NAME
        );
    }
    let expected = BruteForce::new().solve(&source).unwrap();
    let solution = ILPSolver::new().solve(target).unwrap();
    let recovered = chain.extract_solution::<S::Solution, _>(&solution);
    match expected {
        Some(witness) => assert_eq!(
            source.evaluate(&recovered.unwrap()).unwrap(),
            source.evaluate(&witness).unwrap()
        ),
        None => assert!(recovered.is_err()),
    }
}

#[test]
fn capacity_assignment_magnitude_and_qubo() {
    for (delay, budget, bits) in [
        (0, 0, 1),
        (7, 1, 3),
        (8, 1, 4),
        (1, 8, 4),
        (-8, 1, 4),
        (1, -8, 4),
        (i64::MIN, 0, 64),
        (0, i64::MIN, 64),
        (i64::MAX, 0, 63),
    ] {
        let source = CapacityAssignment::new(
            vec![i64::MAX],
            vec![vec![i64::MAX]],
            vec![vec![delay]],
            budget,
        );
        assert_eq!(source.parameters().get("max_delay_bits"), Some(bits));
        check_contract::<_, ILP<bool>>(&source);
    }
    check_contract::<_, ILP<bool>>(&CapacityAssignment::new(vec![1], vec![], vec![], 8));
    for budget in [-3, 0, 8] {
        check_qubo::<_, ILP<bool>>(CapacityAssignment::new(
            vec![1, 2],
            vec![vec![1, 3]],
            vec![vec![2, -2]],
            budget,
        ));
    }
}

#[test]
fn partially_ordered_knapsack_magnitude_and_qubo() {
    for (weight, capacity, bits) in [
        (0, 0, 1),
        (7, 1, 3),
        (8, 1, 4),
        (1, 8, 4),
        (i64::MAX, 0, 63),
        (0, i64::MAX, 63),
    ] {
        let source = PartiallyOrderedKnapsack::new(vec![weight], vec![i64::MAX], vec![], capacity);
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, ILP<bool>>(&source);
    }
    check_contract::<_, ILP<bool>>(&PartiallyOrderedKnapsack::new(vec![], vec![], vec![], 8));
    check_qubo::<_, ILP<bool>>(PartiallyOrderedKnapsack::new(
        vec![2, 3],
        vec![1, 9],
        vec![(0, 1)],
        3,
    ));
}

#[test]
fn constrained_path_magnitude_and_qubo() {
    for (weight, bound, bits) in [
        (1, 1, 1),
        (8, 1, 4),
        (1, 8, 4),
        (i64::MAX, 1, 63),
        (1, i64::MAX, 63),
    ] {
        let source = ShortestWeightConstrainedPath::new(
            SimpleGraph::path(2),
            vec![i64::MAX],
            vec![weight],
            0,
            1,
            bound,
        );
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    check_contract::<_, BoundedILP>(&ShortestWeightConstrainedPath::<_, i64>::new(
        SimpleGraph::empty(8),
        vec![],
        vec![],
        0,
        0,
        1,
    ));
    for bound in [1, 2] {
        check_qubo::<_, BoundedILP>(ShortestWeightConstrainedPath::new(
            SimpleGraph::path(2),
            vec![3],
            vec![2],
            0,
            1,
            bound,
        ));
    }
}

#[test]
fn bounded_forest_magnitude_and_incoming_qubo() {
    for (weights, bound, bits) in [
        (vec![], 8, 4),
        (vec![8], 1, 4),
        (vec![1], 8, 4),
        (vec![i64::MAX], 1, 63),
        (vec![1], i64::MAX, 63),
        (vec![0; 8], 1, 1),
    ] {
        let source = BoundedComponentSpanningForest::new(
            SimpleGraph::empty(weights.len()),
            weights,
            1,
            bound,
        );
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    check_qubo::<_, BoundedILP>(BoundedComponentSpanningForest::new(
        SimpleGraph::path(2),
        vec![1, 1],
        1,
        2,
    ));
    for graph in [SimpleGraph::empty(0), SimpleGraph::empty(3)] {
        let source = PartitionIntoPathsOfLength2::new(graph);
        check_contract::<_, BoundedComponentSpanningForest<SimpleGraph, i64>>(&source);
        check_path(
            source,
            ReductionPath {
                steps: vec![
                    step::<PartitionIntoPathsOfLength2<SimpleGraph>>(),
                    step::<BoundedComponentSpanningForest<SimpleGraph, i64>>(),
                    step::<BoundedILP>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}

#[test]
fn acyclic_partition_magnitude_and_qubo() {
    for (weight, bound, bits) in [
        (0, 1, 1),
        (8, 1, 4),
        (1, -8, 4),
        (i64::MIN, 1, 64),
        (i64::MIN, i64::MIN, 64),
        (i64::MAX, -1, 63),
    ] {
        let source = AcyclicPartition::new(DirectedGraph::empty(1), vec![weight], vec![], bound, 0);
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(bits)
        );
        check_contract::<_, ILP<i64, i64, Bounded>>(&source);
        let reduction = ReduceTo::<BoundedILP>::reduce_to(&source).unwrap();
        let mut feasible = 0;
        // One vertex forces its membership and label; enumerate the auxiliary bit.
        for auxiliary in 0..=1 {
            let target = vec![1, auxiliary, 0];
            if reduction
                .target_problem()
                .evaluate(&target)
                .unwrap()
                .value
                .is_some()
            {
                assert_eq!(reduction.extract_solution(&target).unwrap(), vec![0]);
                feasible += 1;
            }
        }
        assert_eq!(feasible, usize::from(source.evaluate(&vec![0]).unwrap().0));
    }
    check_contract::<_, ILP<i64, i64, Bounded>>(&AcyclicPartition::new(
        DirectedGraph::new(1, vec![(0, 0)]),
        vec![1],
        vec![1],
        1,
        1,
    ));
    for bound in [1, 3] {
        check_qubo::<_, ILP<i64, i64, Bounded>>(AcyclicPartition::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![1, 2],
            vec![1],
            bound,
            0,
        ));
    }
    use crate::models::formula::{CNFClause, KSatisfiability};
    use crate::variant::K3;
    for clauses in [
        vec![],
        vec![
            CNFClause::new(vec![1, 1, 1]),
            CNFClause::new(vec![-1, -1, -1]),
        ],
        vec![CNFClause::new(vec![1, 1, 1])],
    ] {
        check_contract::<_, AcyclicPartition<i64>>(&KSatisfiability::<K3>::new(1, clauses));
    }
}

#[test]
fn branching_magnitude_and_qubo() {
    for (weight, threshold, bits) in [(1, 8, 4), (8, 1, 4), (i64::MIN, 0, 64), (1, i64::MIN, 64)] {
        let source = MultipleChoiceBranching::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![weight],
            vec![vec![0]],
            threshold,
        );
        assert_eq!(source.parameters().get("max_weight_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    for threshold in [1, 3] {
        check_qubo::<_, BoundedILP>(MultipleChoiceBranching::new(
            DirectedGraph::new(2, vec![(0, 1)]),
            vec![2],
            vec![vec![0]],
            threshold,
        ));
    }
}

#[test]
fn capacitated_tree_magnitude_and_qubo() {
    let source = MinimumCapacitatedSpanningTree::new(
        SimpleGraph::path(3),
        vec![i64::MAX; 2],
        0,
        vec![0, 7, 7],
        8,
    );
    assert_eq!(source.parameters().get("max_requirement_bits"), Some(4));
    check_contract::<_, BoundedILP>(&source);
    for capacity in [1, 2] {
        check_qubo::<_, BoundedILP>(MinimumCapacitatedSpanningTree::new(
            SimpleGraph::path(2),
            vec![3],
            0,
            vec![0, 2],
            capacity,
        ));
    }
}

#[test]
fn multicenter_magnitude_products_and_qubo() {
    let source = MinMaxMulticenter::new(SimpleGraph::path(3), vec![8; 3], vec![8; 2], 1);
    assert_eq!(
        source.parameters().get("max_numeric_magnitude_bits"),
        Some(4)
    );
    check_contract::<_, BoundedILP>(&source);
    for graph in [SimpleGraph::path(2), SimpleGraph::empty(2)] {
        let lengths = vec![2; crate::topology::Graph::num_edges(&graph)];
        check_qubo::<_, BoundedILP>(MinMaxMulticenter::new(graph, vec![2, 1], lengths, 1));
    }
}

#[test]
fn multicenter_distance_overflow_is_typed() {
    let source = MinMaxMulticenter::new(SimpleGraph::path(3), vec![1; 3], vec![i64::MAX; 2], 1);
    assert!(matches!(
        source.evaluate(&vec![true, false, false]),
        Err(crate::traits::EvaluationError::IntegerOverflow(_))
    ));
    assert!(matches!(
        ReduceTo::<BoundedILP>::reduce_to(&source),
        Err(crate::rules::ReductionError::IntegerOverflow { .. })
    ));
}

#[test]
fn flow_shop_magnitude_and_qubo() {
    for (time, deadline, bits) in [(1, 8, 4), (8, 1, 4), (0, 0, 1), (7, 7, 3)] {
        let source = FlowShopScheduling::new(1, vec![vec![time]; 2], deadline);
        assert_eq!(source.parameters().get("max_time_bits"), Some(bits));
        check_contract::<_, BoundedILP>(&source);
    }
    check_contract::<_, BoundedILP>(&FlowShopScheduling::new(0, vec![vec![]; 2], 8));
    for deadline in [1, 2] {
        check_qubo::<_, BoundedILP>(FlowShopScheduling::new(1, vec![vec![1]; 2], deadline));
    }
}

#[test]
fn minimum_tardiness_negative_deadlines_preserve_optimum() {
    // Every schedule has exactly one tardy task, even for the smallest deadline.
    for deadline in [-1, i64::MIN] {
        let source = MinimumTardinessSequencing::<One>::new(1, vec![deadline], vec![]);
        assert_eq!(source.evaluate(&vec![0]).unwrap(), Min(Some(1)));
        let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let witness = ILPSolver::new().solve(reduced.target_problem()).unwrap();
        assert_eq!(
            source
                .evaluate(&reduced.extract_solution(&witness).unwrap())
                .unwrap(),
            Min(Some(1))
        );
        let source =
            MinimumTardinessSequencing::<i64>::with_lengths(vec![2], vec![deadline], vec![]);
        let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let witness = ILPSolver::new().solve(reduced.target_problem()).unwrap();
        assert_eq!(
            source
                .evaluate(&reduced.extract_solution(&witness).unwrap())
                .unwrap(),
            Min(Some(1))
        );
    }
}

#[test]
fn minimum_tardiness_magnitude_and_qubo() {
    for deadline in [i64::MIN, 0, i64::MAX] {
        check_qubo::<_, ILP<bool>>(MinimumTardinessSequencing::<One>::new(
            2,
            vec![deadline, 1],
            vec![(0, 1)],
        ));
        let source = MinimumTardinessSequencing::<i64>::with_lengths(
            vec![2, 1],
            vec![deadline, 2],
            vec![(0, 1)],
        );
        assert_eq!(source.parameters().get("max_processing_time_bits"), Some(2));
        check_qubo::<_, ILP<bool>>(source);
    }
}

#[test]
fn completion_time_magnitude_and_qubo() {
    let source = SchedulingToMinimizeWeightedCompletionTime::new(vec![7, 7], vec![i64::MAX; 2], 2);
    assert_eq!(source.parameters().get("max_processing_time_bits"), Some(3));
    check_contract::<_, BoundedILP>(&source);
    check_qubo::<_, BoundedILP>(SchedulingToMinimizeWeightedCompletionTime::new(
        vec![1, 2],
        vec![2, 1],
        2,
    ));
    let source =
        SequencingToMinimizeWeightedCompletionTime::new(vec![7, 7], vec![i64::MAX; 2], vec![]);
    assert_eq!(source.parameters().get("max_processing_time_bits"), Some(3));
    check_contract::<_, BoundedILP>(&source);
    check_qubo::<_, BoundedILP>(SequencingToMinimizeWeightedCompletionTime::new(
        vec![1, 2],
        vec![1, 2],
        vec![(0, 1)],
    ));
    let source = OptimalLinearArrangement::new(SimpleGraph::path(2));
    check_contract::<_, SequencingToMinimizeWeightedCompletionTime>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<OptimalLinearArrangement<SimpleGraph>>(),
                step::<SequencingToMinimizeWeightedCompletionTime>(),
                step::<BoundedILP>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
}

#[test]
fn cumulative_cost_magnitude_and_qubo() {
    let source = SequencingToMinimizeMaximumCumulativeCost::new(vec![-8, 8], vec![]);
    assert_eq!(source.parameters().get("max_cost_bits"), Some(4));
    check_contract::<_, BoundedILP>(&source);
    for costs in [vec![-2, 3], vec![-2, -1], vec![]] {
        check_qubo::<_, BoundedILP>(SequencingToMinimizeMaximumCumulativeCost::new(
            costs,
            vec![],
        ));
    }
}

#[test]
fn tardy_task_weight_magnitude_and_incoming_qubo() {
    for deadlines in [vec![i64::MIN; 2], vec![i64::MAX; 2]] {
        let source = SequencingToMinimizeTardyTaskWeight::new(vec![-1, 2], vec![1, -1], deadlines);
        assert_eq!(source.parameters().get("max_processing_time_bits"), Some(2));
        check_qubo::<_, ILP<bool>>(source);
    }
    use crate::models::Decision;
    let source = Partition::new(vec![1, 1]).unwrap();
    check_contract::<_, Decision<SequencingToMinimizeTardyTaskWeight>>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<Partition>(),
                step::<Decision<SequencingToMinimizeTardyTaskWeight>>(),
                step::<SequencingToMinimizeTardyTaskWeight>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
}

#[test]
fn weighted_tardiness_magnitude_and_qubo() {
    for (length, weight, deadline, bound) in
        [(8, 1, 1, 1), (1, 8, 1, 1), (1, 1, 8, 1), (1, 1, 1, 8)]
    {
        let source = SequencingToMinimizeWeightedTardiness::new(
            vec![length],
            vec![weight],
            vec![deadline],
            bound,
        );
        assert_eq!(
            source.parameters().get("max_numeric_magnitude_bits"),
            Some(4)
        );
        check_contract::<_, BoundedILP>(&source);
    }
    for bound in [0, 2] {
        check_qubo::<_, BoundedILP>(SequencingToMinimizeWeightedTardiness::new(
            vec![2],
            vec![1],
            vec![1],
            bound,
        ));
    }
}

#[test]
fn setup_time_magnitude_and_qubo() {
    for (length, deadline, setup) in [(8, 1, 1), (1, 8, 1), (1, 1, 8)] {
        let source = SequencingWithDeadlinesAndSetUpTimes::new(
            vec![length; 2],
            vec![deadline; 2],
            vec![0, 1],
            vec![setup; 2],
        );
        assert_eq!(source.parameters().get("max_time_bits"), Some(4));
        check_contract::<_, ILP<bool>>(&source);
    }
    for deadline in [2, 3] {
        check_qubo::<_, ILP<bool>>(SequencingWithDeadlinesAndSetUpTimes::new(
            vec![1; 2],
            vec![deadline; 2],
            vec![0, 1],
            vec![1; 2],
        ));
    }
}

#[test]
fn resource_scheduling_packs_slots_and_ignores_excess_processors() {
    let source = ResourceConstrainedScheduling::new(2, vec![2], vec![vec![1]; 2], 1000).unwrap();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(reduction.target_problem().num_vars() <= 4);
    // These instances must not allocate or iterate through the numeric deadline.
    let source =
        ResourceConstrainedScheduling::new(usize::MAX, vec![2], vec![vec![1]; 2], i64::MAX)
            .unwrap();
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    assert!(reduction.target_problem().num_vars() <= 4);
    assert!(
        source
            .evaluate(&vec![0, (i64::MAX - 1) as usize])
            .unwrap()
            .0
    );
    let witness = ILPSolver::new().solve(reduction.target_problem()).unwrap();
    assert!(
        source
            .evaluate(&reduction.extract_solution(&witness).unwrap())
            .unwrap()
            .0
    );
    let empty = ResourceConstrainedScheduling::new(0, vec![0], vec![], i64::MAX).unwrap();
    assert!(empty.evaluate(&vec![]).unwrap().0);
    assert_eq!(
        ReduceTo::<ILP<bool>>::reduce_to(&empty)
            .unwrap()
            .target_problem()
            .num_vars(),
        0
    );
}

#[test]
fn resource_magnitude_and_incoming_predictions() {
    for (requirement, bound, bits) in [
        (0, 0, 1),
        (8, 1, 4),
        (1, 8, 4),
        (i64::MAX, 0, 63),
        (0, i64::MAX, 63),
    ] {
        let source =
            ResourceConstrainedScheduling::new(2, vec![bound], vec![vec![requirement]], 2).unwrap();
        assert_eq!(source.parameters().get("max_resource_bits"), Some(bits));
        check_contract::<_, ILP<bool>>(&source);
    }
    for processors in [0, 1, 2] {
        check_qubo::<_, ILP<bool>>(
            ResourceConstrainedScheduling::new(processors, vec![1], vec![vec![1]; 2], 2).unwrap(),
        );
    }
    let source = ThreePartition::new(vec![1; 3], 3);
    check_contract::<_, ResourceConstrainedScheduling>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<ThreePartition>(),
                step::<ResourceConstrainedScheduling>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
    use crate::models::set::ThreeDimensionalMatching;
    for (size, triples) in [
        (0, vec![]),
        (1, vec![]),
        (1, vec![(0, 0, 0)]),
        (2, vec![(0, 0, 0), (1, 1, 1)]),
    ] {
        let source = ThreeDimensionalMatching::new(size, triples);
        check_contract::<_, ThreePartition>(&source);
        let path = ReductionPath {
            steps: vec![
                step::<ThreeDimensionalMatching>(),
                step::<ThreePartition>(),
                step::<ResourceConstrainedScheduling>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        };
        let predicted = ReductionGraph::new()
            .compose_path_parameter_transform(&path)
            .unwrap()
            .unwrap()
            .evaluate(&source.parameters())
            .unwrap();
        assert!(predicted.get("num_vars").is_some());
        assert!(predicted.get("num_quadratic_terms").is_some());
    }
}

#[test]
fn circuit_threshold_normalization_and_incoming_qubo() {
    use crate::models::Decision;
    for threshold in [i64::MIN, 0, 3, 4, i64::MAX] {
        let source = Decision::new(
            LongestCircuit::new(SimpleGraph::cycle(3), vec![1i64; 3]),
            threshold,
        );
        assert_eq!(source.parameters().get("max_length_bits"), Some(1));
        check_contract::<_, ILP<bool>>(&source);
        let reduced = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        assert!(reduced.target_problem().max_constraint_magnitude_bits() <= 5);
        assert_eq!(
            ILPSolver::new().solve(reduced.target_problem()).is_ok(),
            threshold <= 3
        );
    }
    let source = HamiltonianCircuit::new(SimpleGraph::cycle(3));
    check_contract::<_, Decision<LongestCircuit<SimpleGraph, i64>>>(&source);
    check_path(
        source,
        ReductionPath {
            steps: vec![
                step::<HamiltonianCircuit<SimpleGraph>>(),
                step::<Decision<LongestCircuit<SimpleGraph, i64>>>(),
                step::<ILP<bool>>(),
                step::<QUBO<i64>>(),
            ],
        },
    );
}

#[test]
fn multiprocessor_predictions_compose_and_recover_through_qubo() {
    for sizes in [vec![1, 1], vec![3; 4], vec![1, 2], vec![2]] {
        check_path(
            Partition::new(sizes).unwrap(),
            ReductionPath {
                steps: vec![
                    step::<Partition>(),
                    step::<MultiprocessorScheduling>(),
                    step::<ILP<bool>>(),
                    step::<QUBO<i64>>(),
                ],
            },
        );
    }
}
