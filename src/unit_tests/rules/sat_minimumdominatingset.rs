use super::*;
use crate::models::satisfiability::CNFClause;
use crate::solvers::{BruteForce, Solver};

#[test]
fn test_simple_sat_to_ds() {
    // Simple SAT: (x1) - one variable, one clause
    let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // Should have 3 vertices (variable gadget) + 1 clause vertex = 4 vertices
    assert_eq!(ds_problem.num_vertices(), 4);

    // Edges: 3 for triangle + 1 from positive literal to clause = 4
    // Triangle edges: (0,1), (0,2), (1,2)
    // Clause edge: (0, 3) since x1 positive connects to clause vertex
    assert_eq!(ds_problem.num_edges(), 4);
}

#[test]
fn test_two_variable_sat_to_ds() {
    // SAT: (x1 OR x2)
    let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, 2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 2 variables * 3 = 6 gadget vertices + 1 clause vertex = 7
    assert_eq!(ds_problem.num_vertices(), 7);

    // Edges:
    // - 3 edges for first triangle: (0,1), (0,2), (1,2)
    // - 3 edges for second triangle: (3,4), (3,5), (4,5)
    // - 2 edges from literals to clause: (0,6), (3,6)
    assert_eq!(ds_problem.num_edges(), 8);
}

#[test]
fn test_satisfiable_formula() {
    // SAT: (x1 OR x2) AND (NOT x1 OR x2)
    // Satisfiable with x2 = true
    let sat = Satisfiability::<i32>::new(
        2,
        vec![
            CNFClause::new(vec![1, 2]),  // x1 OR x2
            CNFClause::new(vec![-1, 2]), // NOT x1 OR x2
        ],
    );
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // Solve the dominating set problem
    let solver = BruteForce::new();
    let solutions = solver.find_best(ds_problem);

    // Minimum dominating set should be of size 2 (one per variable)
    let min_size = solutions[0].iter().sum::<usize>();
    assert_eq!(min_size, 2, "Minimum dominating set should have 2 vertices");

    // Extract and verify at least one solution satisfies SAT
    let mut found_satisfying = false;
    for sol in &solutions {
        let sat_sol = reduction.extract_solution(sol);
        let assignment: Vec<bool> = sat_sol.iter().map(|&v| v == 1).collect();
        if sat.is_satisfying(&assignment) {
            found_satisfying = true;
            break;
        }
    }
    assert!(found_satisfying, "Should find a satisfying assignment");
}

#[test]
fn test_unsatisfiable_formula() {
    // SAT: (x1) AND (NOT x1) - unsatisfiable
    let sat =
        Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // Vertices: 3 (gadget) + 2 (clauses) = 5
    assert_eq!(ds_problem.num_vertices(), 5);

    let solver = BruteForce::new();
    let solutions = solver.find_best(ds_problem);

    // For unsatisfiable formula, the minimum dominating set will need
    // more than num_variables vertices OR won't produce a valid assignment
    // Actually, in this case we can still dominate with just selecting
    // one literal vertex (it dominates its gadget AND one clause),
    // but then the other clause isn't dominated.
    // So we need at least 2 vertices: one for each clause's requirement.

    // The key insight is that both clauses share the same variable gadget
    // but require opposite literals. To dominate both clause vertices,
    // we need to select BOTH literal vertices (0 and 1) or the dummy +
    // something else.

    // Verify no extracted solution satisfies the formula
    for sol in &solutions {
        let sat_sol = reduction.extract_solution(sol);
        let assignment: Vec<bool> = sat_sol.iter().map(|&v| v == 1).collect();
        // This unsatisfiable formula should not have a satisfying assignment
        assert!(
            !sat.is_satisfying(&assignment),
            "Unsatisfiable formula should not be satisfied"
        );
    }
}

#[test]
fn test_three_sat_example() {
    // 3-SAT: (x1 OR x2 OR x3) AND (NOT x1 OR NOT x2 OR x3) AND (x1 OR NOT x2 OR NOT x3)
    let sat = Satisfiability::<i32>::new(
        3,
        vec![
            CNFClause::new(vec![1, 2, 3]),   // x1 OR x2 OR x3
            CNFClause::new(vec![-1, -2, 3]), // NOT x1 OR NOT x2 OR x3
            CNFClause::new(vec![1, -2, -3]), // x1 OR NOT x2 OR NOT x3
        ],
    );

    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 3 variables * 3 = 9 gadget vertices + 3 clauses = 12
    assert_eq!(ds_problem.num_vertices(), 12);

    let solver = BruteForce::new();
    let solutions = solver.find_best(ds_problem);

    // Minimum should be 3 (one per variable)
    let min_size = solutions[0].iter().sum::<usize>();
    assert_eq!(min_size, 3, "Minimum dominating set should have 3 vertices");

    // Verify extracted solutions
    let mut found_satisfying = false;
    for sol in &solutions {
        let sat_sol = reduction.extract_solution(sol);
        let assignment: Vec<bool> = sat_sol.iter().map(|&v| v == 1).collect();
        if sat.is_satisfying(&assignment) {
            found_satisfying = true;
            break;
        }
    }
    assert!(
        found_satisfying,
        "Should find a satisfying assignment for 3-SAT"
    );
}

#[test]
fn test_extract_solution_positive_literal() {
    // (x1) - select positive literal
    let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Solution: select vertex 0 (positive literal x1)
    // This dominates vertices 1, 2 (gadget) and vertex 3 (clause)
    let ds_sol = vec![1, 0, 0, 0];
    let sat_sol = reduction.extract_solution(&ds_sol);
    assert_eq!(sat_sol, vec![1]); // x1 = true
}

#[test]
fn test_extract_solution_negative_literal() {
    // (NOT x1) - select negative literal
    let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![-1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Solution: select vertex 1 (negative literal NOT x1)
    // This dominates vertices 0, 2 (gadget) and vertex 3 (clause)
    let ds_sol = vec![0, 1, 0, 0];
    let sat_sol = reduction.extract_solution(&ds_sol);
    assert_eq!(sat_sol, vec![0]); // x1 = false
}

#[test]
fn test_extract_solution_dummy() {
    // (x1 OR x2) where only x1 matters
    let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Select: vertex 0 (x1 positive) and vertex 5 (x2 dummy)
    // Vertex 0 dominates: itself, 1, 2, and clause 6
    // Vertex 5 dominates: 3, 4, and itself
    let ds_sol = vec![1, 0, 0, 0, 0, 1, 0];
    let sat_sol = reduction.extract_solution(&ds_sol);
    assert_eq!(sat_sol, vec![1, 0]); // x1 = true, x2 = false (from dummy)
}

#[test]
fn test_source_and_target_size() {
    let sat = Satisfiability::<i32>::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    let source_size = reduction.source_size();
    let target_size = reduction.target_size();

    assert_eq!(source_size.get("num_vars"), Some(3));
    assert_eq!(source_size.get("num_clauses"), Some(2));
    // 3 vars * 3 = 9 gadget vertices + 2 clause vertices = 11
    assert_eq!(target_size.get("num_vertices"), Some(11));
}

#[test]
fn test_empty_sat() {
    // Empty SAT (trivially satisfiable)
    let sat = Satisfiability::<i32>::new(0, vec![]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    assert_eq!(ds_problem.num_vertices(), 0);
    assert_eq!(ds_problem.num_edges(), 0);
    assert_eq!(reduction.num_clauses(), 0);
    assert_eq!(reduction.num_literals(), 0);
}

#[test]
fn test_multiple_literals_same_variable() {
    // Clause with repeated variable: (x1 OR NOT x1) - tautology
    let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1, -1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 3 gadget vertices + 1 clause vertex = 4
    assert_eq!(ds_problem.num_vertices(), 4);

    // Edges:
    // - 3 for triangle
    // - 2 from literals to clause (both positive and negative literals connect)
    assert_eq!(ds_problem.num_edges(), 5);
}

#[test]
fn test_sat_ds_solution_correspondence() {
    // Comprehensive test: verify that solutions extracted from DS satisfy SAT
    let sat = Satisfiability::<i32>::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, -2])],
    );

    // Solve SAT directly
    let sat_solver = BruteForce::new();
    let direct_sat_solutions = sat_solver.find_best(&sat);

    // Solve via reduction
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();
    let ds_solutions = sat_solver.find_best(ds_problem);

    // Direct SAT solutions should all be valid
    for sol in &direct_sat_solutions {
        let assignment: Vec<bool> = sol.iter().map(|&v| v == 1).collect();
        assert!(sat.is_satisfying(&assignment));
    }

    // DS solutions with minimum size should correspond to valid SAT solutions
    let min_size = ds_solutions[0].iter().sum::<usize>();
    if min_size == 2 {
        // Only if min dominating set = num_vars
        let mut found_satisfying = false;
        for sol in &ds_solutions {
            if sol.iter().sum::<usize>() == 2 {
                let sat_sol = reduction.extract_solution(sol);
                let assignment: Vec<bool> = sat_sol.iter().map(|&v| v == 1).collect();
                if sat.is_satisfying(&assignment) {
                    found_satisfying = true;
                    break;
                }
            }
        }
        assert!(
            found_satisfying,
            "At least one DS solution should give a SAT solution"
        );
    }
}

#[test]
fn test_accessors() {
    let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, -2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    assert_eq!(reduction.num_literals(), 2);
    assert_eq!(reduction.num_clauses(), 1);
}

#[test]
fn test_extract_solution_too_many_selected() {
    // Test that extract_solution handles invalid (non-minimal) dominating sets
    let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);

    // Select all 4 vertices (more than num_literals=1)
    let ds_sol = vec![1, 1, 1, 1];
    let sat_sol = reduction.extract_solution(&ds_sol);
    // Should return default (all false)
    assert_eq!(sat_sol, vec![0]);
}

#[test]
fn test_negated_variable_connection() {
    // (NOT x1 OR NOT x2) - both negated
    let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![-1, -2])]);
    let reduction = ReduceTo::<MinimumDominatingSet<SimpleGraph, i32>>::reduce_to(&sat);
    let ds_problem = reduction.target_problem();

    // 2 * 3 = 6 gadget vertices + 1 clause = 7
    assert_eq!(ds_problem.num_vertices(), 7);

    // Edges:
    // - 3 for first triangle: (0,1), (0,2), (1,2)
    // - 3 for second triangle: (3,4), (3,5), (4,5)
    // - 2 from negated literals to clause: (1,6), (4,6)
    assert_eq!(ds_problem.num_edges(), 8);
}
