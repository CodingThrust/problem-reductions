use super::*;
use crate::rules::AggregateReductionResult;
use crate::topology::DirectedGraph;
use crate::traits::Problem;

#[test]
fn test_kernel_to_ilp_preserves_selections_with_loops_and_parallel_arcs() {
    for arc_mask in 0..512 {
        let mut arcs = Vec::new();
        for u in 0..3 {
            for v in 0..3 {
                if arc_mask & (1 << (3 * u + v)) != 0 {
                    arcs.extend([(u, v), (u, v), (u, v)]);
                }
            }
        }
        let source = Kernel::new(DirectedGraph::new(3, arcs));
        let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
        let target = ReductionResult::target_problem(&reduction);
        assert_eq!(target.num_vars(), 3);
        assert_eq!(target.num_constraints(), source.num_arcs() + 3);
        assert!(target.num_nonzeros() <= 3 * source.num_arcs() + 3);
        assert!(target.max_constraint_magnitude_bits() <= 2);
        for mask in 0..8 {
            let bits: Vec<bool> = (0..3).map(|i| mask & (1 << i) != 0).collect();
            let values = bits.iter().copied().map(i64::from).collect();
            let expected = source.evaluate(&bits).unwrap();
            let actual = target.evaluate(&values).unwrap();
            assert_eq!(actual.value.is_some(), expected.0);
            assert_eq!(reduction.extract_value(actual).unwrap(), expected);
            if expected.0 {
                assert_eq!(reduction.extract_solution(&values).unwrap(), bits);
            } else {
                assert!(reduction.extract_solution(&values).is_err());
            }
        }
    }
}

#[test]
fn test_kernel_to_ilp_closed_loop() {
    let source = Kernel::new(DirectedGraph::new(3, vec![(0, 1), (1, 2)]));
    let reduction = ReduceTo::<ILP<bool>>::reduce_to(&source).unwrap();
    crate::rules::test_helpers::assert_bf_vs_ilp(&source, &reduction);
    for malformed in [vec![1, 0], vec![1, 0, 1, 0], vec![2, 0, 1]] {
        assert!(reduction.extract_solution(&malformed).is_err());
    }
}
