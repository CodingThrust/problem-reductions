//! Tests for Problem::problem_size() implementations.

use crate::models::graph::*;
use crate::models::optimization::*;
use crate::models::satisfiability::*;
use crate::models::set::*;
use crate::models::specialized::*;
use crate::topology::SimpleGraph;
use crate::traits::Problem;

#[test]
fn test_problem_size_mis() {
    let g = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mis = MaximumIndependentSet::new(g, vec![1i32; 4]);
    let size = mis.problem_size();
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_size_max_clique() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let mc = MaximumClique::new(g, vec![1i32; 3]);
    let size = mc.problem_size();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_size_min_vc() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mvc = MinimumVertexCover::new(g, vec![1i32; 3]);
    let size = mvc.problem_size();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(2));
}

#[test]
fn test_problem_size_min_ds() {
    let g = SimpleGraph::new(4, vec![(0, 1), (0, 2), (0, 3)]);
    let mds = MinimumDominatingSet::new(g, vec![1i32; 4]);
    let size = mds.problem_size();
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_size_max_cut() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let mc = MaxCut::new(g, vec![1i32; 3]);
    let size = mc.problem_size();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_size_maximum_matching() {
    let g = SimpleGraph::new(4, vec![(0, 1), (1, 2), (2, 3)]);
    let mm = MaximumMatching::new(g, vec![1i32; 3]);
    let size = mm.problem_size();
    assert_eq!(size.get("num_vertices"), Some(4));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_size_maximal_is() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2)]);
    let mis = MaximalIS::new(g, vec![1i32; 3]);
    let size = mis.problem_size();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(2));
}

#[test]
fn test_problem_size_kcoloring() {
    use crate::variant::KN;
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let kc = KColoring::<KN, _>::with_k(g, 3);
    let size = kc.problem_size();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
    assert_eq!(size.get("num_colors"), Some(3));
}

#[test]
fn test_problem_size_tsp() {
    let g = SimpleGraph::new(3, vec![(0, 1), (1, 2), (0, 2)]);
    let tsp = TravelingSalesman::new(g, vec![1i32; 3]);
    let size = tsp.problem_size();
    assert_eq!(size.get("num_vertices"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
}

#[test]
fn test_problem_size_sat() {
    use crate::models::satisfiability::CNFClause;
    let sat = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, -2]), CNFClause::new(vec![2, 3])],
    );
    let size = sat.problem_size();
    assert_eq!(size.get("num_vars"), Some(3));
    assert_eq!(size.get("num_clauses"), Some(2));
    assert_eq!(size.get("num_literals"), Some(4));
}

#[test]
fn test_problem_size_ksat() {
    use crate::models::satisfiability::CNFClause;
    use crate::variant::K3;
    let ksat = KSatisfiability::<K3>::new(
        3,
        vec![
            CNFClause::new(vec![1, -2, 3]),
            CNFClause::new(vec![-1, 2, -3]),
        ],
    );
    let size = ksat.problem_size();
    assert_eq!(size.get("num_vars"), Some(3));
    assert_eq!(size.get("num_clauses"), Some(2));
    assert_eq!(size.get("num_literals"), Some(6));
}

#[test]
fn test_problem_size_qubo() {
    let qubo = QUBO::<f64>::new(vec![1.0, 2.0, 3.0], vec![]);
    let size = qubo.problem_size();
    assert_eq!(size.get("num_vars"), Some(3));
}

#[test]
fn test_problem_size_spinglass() {
    let sg = SpinGlass::<SimpleGraph, f64>::new(
        3,
        vec![((0, 1), 1.0), ((1, 2), -1.0)],
        vec![0.0, 0.5, -0.5],
    );
    let size = sg.problem_size();
    assert_eq!(size.get("num_spins"), Some(3));
    assert_eq!(size.get("num_interactions"), Some(2));
}

#[test]
fn test_problem_size_ilp() {
    use crate::models::optimization::{LinearConstraint, ObjectiveSense};
    let ilp = ILP::binary(
        2,
        vec![LinearConstraint::le(vec![(0, 1.0), (1, 1.0)], 3.0)],
        vec![(0, 1.0), (1, 2.0)],
        ObjectiveSense::Maximize,
    );
    let size = ilp.problem_size();
    assert_eq!(size.get("num_vars"), Some(2));
    assert_eq!(size.get("num_constraints"), Some(1));
}

#[test]
fn test_problem_size_factoring() {
    let f = Factoring::new(2, 3, 6);
    let size = f.problem_size();
    assert_eq!(size.get("num_bits_first"), Some(2));
    assert_eq!(size.get("num_bits_second"), Some(3));
}

#[test]
fn test_problem_size_circuitsat() {
    use crate::models::specialized::{Assignment, BooleanExpr, Circuit};
    let circuit = Circuit::new(vec![Assignment::new(
        vec!["c".to_string()],
        BooleanExpr::and(vec![BooleanExpr::var("x"), BooleanExpr::var("y")]),
    )]);
    let problem = CircuitSAT::new(circuit);
    let size = problem.problem_size();
    assert_eq!(size.get("num_variables"), Some(problem.num_variables()));
    assert_eq!(size.get("num_assignments"), Some(1));
}

#[test]
fn test_problem_size_paintshop() {
    let ps = PaintShop::new(vec!["a", "b", "a", "c", "c", "b"]);
    let size = ps.problem_size();
    assert_eq!(size.get("num_cars"), Some(3));
    assert_eq!(size.get("num_sequence"), Some(6));
}

#[test]
fn test_problem_size_biclique_cover() {
    let bc = BicliqueCover::new(2, 3, vec![(0, 2), (0, 3), (1, 4)], 2);
    let size = bc.problem_size();
    assert_eq!(size.get("left_size"), Some(2));
    assert_eq!(size.get("right_size"), Some(3));
    assert_eq!(size.get("num_edges"), Some(3));
    assert_eq!(size.get("rank"), Some(2));
}

#[test]
fn test_problem_size_bmf() {
    let bmf = BMF::new(vec![vec![true, false], vec![false, true]], 2);
    let size = bmf.problem_size();
    assert_eq!(size.get("m"), Some(2));
    assert_eq!(size.get("n"), Some(2));
    assert_eq!(size.get("rank"), Some(2));
}

#[test]
fn test_problem_size_set_packing() {
    let sp = MaximumSetPacking::<i32>::new(vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let size = sp.problem_size();
    assert_eq!(size.get("num_sets"), Some(3));
    assert_eq!(size.get("universe_size"), Some(4));
}

#[test]
fn test_problem_size_set_covering() {
    let sc = MinimumSetCovering::<i32>::new(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    let size = sc.problem_size();
    assert_eq!(size.get("num_sets"), Some(3));
    assert_eq!(size.get("universe_size"), Some(4));
}
