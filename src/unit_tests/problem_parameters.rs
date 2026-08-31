//! Tests for canonical parameters measured by concrete problem instances.

use crate::models::algebraic::*;
use crate::models::formula::*;
use crate::models::graph::*;
use crate::models::misc::*;
use crate::models::set::*;
use crate::topology::{BipartiteGraph, SimpleGraph};
use crate::traits::Problem;

#[test]
fn test_problem_parameters_mis() {
    let g = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mis = MaximumIndependentSet::new(g, vec![1i64; 4]);
    let size = mis.parameters();
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_parameters_max_clique() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let mc = MaximumClique::new(g, vec![1i64; 3]);
    let size = mc.parameters();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_parameters_min_vc() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mvc = MinimumVertexCover::new(g, vec![1i64; 3]);
    let size = mvc.parameters();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(2));
}

#[test]
fn test_problem_parameters_min_ds() {
    let g = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let mds = MinimumDominatingSet::new(g, vec![1i64; 4]);
    let size = mds.parameters();
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_parameters_max_cut() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let mc = MaxCut::new(g, vec![1i64; 3]);
    let size = mc.parameters();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_parameters_maximum_matching() {
    let g = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mm = MaximumMatching::new(g, vec![1i64; 3]);
    let size = mm.parameters();
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_parameters_maximal_is() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mis = MaximalIS::new(g, vec![1i64; 3]);
    let size = mis.parameters();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(2));
}

#[test]
fn test_problem_parameters_knapsack_capacity() {
    let knapsack = Knapsack::new(vec![2, 3], vec![5, 7], 4);
    let parameters = knapsack.parameters();

    assert_eq!(parameters.get("capacity"), Some(4));
    assert_eq!(parameters.get("num_items"), Some(2));
}

#[test]
fn test_problem_parameters_kcoloring() {
    use crate::variant::KN;
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let kc = KColoring::<KN, _>::with_k(g, 3);
    let size = kc.parameters();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
    assert_eq!(size.get("num_colors"), Some(3));
}

#[test]
fn test_problem_parameters_tsp() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tsp = TravelingSalesman::new(g, vec![1i64; 3]);
    let size = tsp.parameters();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_parameters_sat() {
    use crate::models::formula::CNFClause;
    let sat = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2]), CNFClause::new(vec![2, 3])],
    );
    let size = sat.parameters();
    assert_eq!(size.get("num_vars"), Some(3));
    assert_eq!(size.get("num_clauses"), Some(2));
    assert_eq!(size.get("num_literals"), Some(4));
}

#[test]
fn test_problem_parameters_ksat() {
    use crate::models::formula::CNFClause;
    use crate::variant::K3;
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    );
    let size = ksat.parameters();
    assert_eq!(size.get("num_vars"), Some(3));
    assert_eq!(size.get("num_clauses"), Some(2));
    assert_eq!(size.get("num_literals"), Some(6));
}

#[test]
fn test_problem_parameters_qubo() {
    let qubo = QUBO::<f64>::new(vec![1.0, 2.0, 3.0], vec![]).unwrap();
    let size = qubo.parameters();
    assert_eq!(size.get("num_vars"), Some(3));
}

#[test]
fn test_problem_parameters_spinglass() {
    let sg = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), 1.0), ((1, 2), -1.0)],
        vec![0.0, 0.5, -0.5],
    )
    .unwrap();
    let size = sg.parameters();
    assert_eq!(size.get("num_spins"), Some(3));
    assert_eq!(size.get("num_interactions"), Some(2));
}

#[test]
fn test_problem_parameters_ilp() {
    use crate::models::algebraic::{LinearConstraint, ObjectiveSense};
    let ilp = ILP::<bool>::new(
        2,
        vec![LinearConstraint::le(vec![(0, 1), (1, 1)], 3)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    )
    .unwrap();
    let size = ilp.parameters();
    assert_eq!(size.get("num_vars"), Some(2));
    assert_eq!(size.get("num_constraints"), Some(1));
}

#[test]
fn test_problem_parameters_factoring() {
    let f = Factoring::with_factor_bits(6, 2, 3);
    let size = f.parameters();
    assert_eq!(size.get("num_bits_first"), Some(2));
    assert_eq!(size.get("num_bits_second"), Some(3));
}

#[test]
fn test_problem_parameters_circuitsat() {
    use crate::models::formula::{Assignment, BooleanExpr, Circuit};
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let size = problem.parameters();
    assert_eq!(size.get("num_variables"), Some(3));
    assert_eq!(size.get("num_assignments"), Some(1));
}

#[test]
fn test_problem_parameters_paintshop() {
    let ps = PaintShop::new(vec!["a", "b", "a", "c", "c", "b"]);
    let size = ps.parameters();
    assert_eq!(size.get("num_cars"), Some(3));
    assert_eq!(size.get("num_sequence"), Some(6));
}

#[test]
fn test_problem_parameters_biclique_cover() {
    let bc = BicliqueCover::new(BipartiteGraph::new(2, 3, vec![(0, 0), (0, 1), (1, 2)]), 2);
    let size = bc.parameters();
    assert_eq!(size.get("left_size"), Some(2));
    assert_eq!(size.get("right_size"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
    assert_eq!(size.get("rank"), Some(2));
}

#[test]
fn test_problem_parameters_bmf() {
    let bmf = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let size = bmf.parameters();
    assert_eq!(size.get("rows"), Some(2));
    assert_eq!(size.get("cols"), Some(2));
    assert_eq!(size.get("rank"), Some(2));
}

#[test]
fn test_problem_parameters_set_packing() {
    let sp = MaximumSetPacking::<i64>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let size = sp.parameters();
    assert_eq!(size.get("num_sets"), Some(3));
    assert_eq!(size.get("universe_size"), Some(4));
}

#[test]
fn test_problem_parameters_set_covering() {
    let sc = MinimumSetCovering::<i64>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let size = sc.parameters();
    assert_eq!(size.get("num_sets"), Some(3));
    assert_eq!(size.get("universe_size"), Some(4));
}
