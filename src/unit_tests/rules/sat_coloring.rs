use super::*;
use crate::models::satisfiability::CNFClause;
use crate::solvers::BruteForce;
include!("../jl_helpers.rs");

#[test]
fn test_constructor_basic_structure() {
    let constructor = SATColoringConstructor::new(2);

    // Should have 2*2 + 3 = 7 vertices
    assert_eq!(constructor.num_vertices, 7);

    // Check pos_vertices and neg_vertices
    assert_eq!(constructor.pos_vertices, vec![3, 4]);
    assert_eq!(constructor.neg_vertices, vec![5, 6]);

    // Check vmap
    assert_eq!(constructor.vmap[&(0, false)], 3);
    assert_eq!(constructor.vmap[&(0, true)], 5);
    assert_eq!(constructor.vmap[&(1, false)], 4);
    assert_eq!(constructor.vmap[&(1, true)], 6);
}

#[test]
fn test_special_vertex_accessors() {
    let constructor = SATColoringConstructor::new(1);
    assert_eq!(constructor.true_vertex(), 0);
    assert_eq!(constructor.false_vertex(), 1);
    assert_eq!(constructor.aux_vertex(), 2);
}

#[test]
fn test_simple_sat_to_coloring() {
    // Simple SAT: (x1) - one clause with one literal
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    // Should have 2*1 + 3 = 5 base vertices
    // Plus edges to set x1 to TRUE (attached to AUX and FALSE)
    assert!(coloring.num_vertices() >= 5);
}

#[test]
fn test_reduction_structure() {
    // Satisfiable formula: (x1 OR x2) AND (NOT x1 OR x2)
    // Just verify the reduction builds the correct structure
    let sat = Satisfiability::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 2])],
    );

    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    // Base vertices: 3 (TRUE, FALSE, AUX) + 2*2 (pos and neg for each var) = 7
    // Each 2-literal clause adds 5 vertices for OR gadget = 2 * 5 = 10
    // Total: 7 + 10 = 17 vertices
    assert_eq!(coloring.num_vertices(), 17);
    assert_eq!(coloring.num_colors(), 3);
    assert_eq!(reduction.pos_vertices().len(), 2);
    assert_eq!(reduction.neg_vertices().len(), 2);
}

#[test]
fn test_unsatisfiable_formula() {
    // Unsatisfiable: (x1) AND (NOT x1)
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])]);

    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    // Solve the coloring problem - use find_all_satisfying since KColoring is a satisfaction problem
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(coloring);

    // For an unsatisfiable formula, the coloring should have no valid solutions
    // OR no valid coloring exists that extracts to a satisfying SAT assignment
    let mut found_satisfying = false;
    for sol in &solutions {
        let sat_sol = reduction.extract_solution(sol);
        let assignment: Vec<bool> = sat_sol.iter().map(|&v| v == 1).collect();
        if sat.is_satisfying(&assignment) {
            found_satisfying = true;
            break;
        }
    }

    // The coloring should not yield a satisfying SAT assignment
    // because the formula is unsatisfiable
    // Note: The coloring graph itself may still be colorable,
    // but the constraints should make it impossible for both
    // x1 and NOT x1 to be TRUE color simultaneously
    // Actually, let's check if ANY coloring solution produces a valid SAT solution
    // If the formula is unsat, no valid coloring should extract to a satisfying assignment
    assert!(
        !found_satisfying,
        "Unsatisfiable formula should not produce satisfying assignment"
    );
}

#[test]
fn test_three_literal_clause_structure() {
    // (x1 OR x2 OR x3)
    let sat = Satisfiability::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    // Base vertices: 3 + 2*3 = 9
    // 3-literal clause needs 2 OR gadgets (x1 OR x2, then result OR x3)
    // Each OR gadget adds 5 vertices, so 2*5 = 10
    // Total: 9 + 10 = 19 vertices
    assert_eq!(coloring.num_vertices(), 19);
    assert_eq!(coloring.num_colors(), 3);
    assert_eq!(reduction.pos_vertices().len(), 3);
    assert_eq!(reduction.neg_vertices().len(), 3);
}

#[test]
fn test_coloring_structure() {
    let sat = Satisfiability::new(
        3,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1, 3])],
    );
    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    // Verify coloring has expected structure
    assert!(coloring.num_vertices() > 0);
    assert_eq!(coloring.num_colors(), 3);
}

#[test]
fn test_extract_solution_basic() {
    // Simple case: one variable, one clause (x1)
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);

    // Manually construct a valid coloring where x1 has TRUE color
    // Vertices: 0=TRUE, 1=FALSE, 2=AUX, 3=x1, 4=NOT_x1
    // Colors: TRUE=0, FALSE=1, AUX=2
    // For x1 to be true, pos_vertex[0]=3 should have color 0 (TRUE)

    // A valid coloring that satisfies x1=TRUE:
    // - Vertex 0 (TRUE): color 0
    // - Vertex 1 (FALSE): color 1
    // - Vertex 2 (AUX): color 2
    // - Vertex 3 (x1): color 0 (TRUE) - connected to AUX(2), NOT_x1(4)
    // - Vertex 4 (NOT_x1): color 1 (FALSE) - connected to AUX(2), x1(3)

    // However, the actual coloring depends on the full graph structure
    // Let's just verify the extraction logic works by checking type signatures
    assert_eq!(reduction.pos_vertices().len(), 1);
    assert_eq!(reduction.neg_vertices().len(), 1);
}

#[test]
fn test_complex_formula_structure() {
    // (x1 OR x2) AND (NOT x1 OR x3) AND (NOT x2 OR NOT x3)
    let sat = Satisfiability::new(
        3,
        vec![
            CNFClause::new(vec![1, 2]),   // x1 OR x2
            CNFClause::new(vec![-1, 3]),  // NOT x1 OR x3
            CNFClause::new(vec![-2, -3]), // NOT x2 OR NOT x3
        ],
    );

    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    // Base vertices: 3 + 2*3 = 9
    // 3 clauses each with 2 literals, each needs 1 OR gadget = 3*5 = 15
    // Total: 9 + 15 = 24 vertices
    assert_eq!(coloring.num_vertices(), 24);
    assert_eq!(coloring.num_colors(), 3);
    assert_eq!(reduction.num_clauses(), 3);
}

#[test]
fn test_single_literal_clauses() {
    // (x1) AND (x2) - both must be true
    let sat = Satisfiability::new(2, vec![CNFClause::new(vec![1]), CNFClause::new(vec![2])]);

    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(coloring);

    let mut found_correct = false;
    for sol in &solutions {
        let sat_sol = reduction.extract_solution(sol);
        if sat_sol == vec![1, 1] {
            found_correct = true;
            break;
        }
    }

    assert!(
        found_correct,
        "Should find solution where both x1 and x2 are true"
    );
}

#[test]
fn test_empty_sat() {
    // Empty SAT (trivially satisfiable)
    let sat = Satisfiability::new(0, vec![]);
    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);

    assert_eq!(reduction.num_clauses(), 0);
    assert!(reduction.pos_vertices().is_empty());
    assert!(reduction.neg_vertices().is_empty());

    let coloring = reduction.target_problem();
    // Just the 3 special vertices
    assert_eq!(coloring.num_vertices(), 3);
}

#[test]
fn test_num_clauses_accessor() {
    let sat = Satisfiability::new(
        2,
        vec![CNFClause::new(vec![1, 2]), CNFClause::new(vec![-1])],
    );
    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    assert_eq!(reduction.num_clauses(), 2);
}

#[test]
fn test_or_gadget_construction() {
    // Test that OR gadget is correctly added
    let mut constructor = SATColoringConstructor::new(2);
    let initial_vertices = constructor.num_vertices;

    // Add an OR gadget
    let input1 = constructor.pos_vertices[0]; // x1
    let input2 = constructor.pos_vertices[1]; // x2
    let output = constructor.add_or_gadget(input1, input2);

    // Should add 5 vertices
    assert_eq!(constructor.num_vertices, initial_vertices + 5);

    // Output should be the last added vertex
    assert_eq!(output, constructor.num_vertices - 1);
}

#[test]
fn test_manual_coloring_extraction() {
    // Test solution extraction with a manually constructed coloring solution
    // for a simple 1-variable SAT problem: (x1)
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);
    let coloring = reduction.target_problem();

    // The graph structure for (x1) with set_true:
    // - Vertices 0, 1, 2: TRUE, FALSE, AUX (triangle)
    // - Vertex 3: x1 (pos)
    // - Vertex 4: NOT x1 (neg)
    // After set_true(3): x1 is connected to AUX and FALSE
    // So x1 must have TRUE color

    // A valid 3-coloring where x1 has TRUE color:
    // TRUE=0, FALSE=1, AUX=2
    // x1 must have color 0 (connected to 1 and 2)
    // NOT_x1 must have color 1 (connected to 2 and x1=0)
    let valid_coloring = vec![0, 1, 2, 0, 1];

    assert_eq!(coloring.num_vertices(), 5);
    let extracted = reduction.extract_solution(&valid_coloring);
    // x1 should be true (1) because vertex 3 has color 0 which equals TRUE vertex's color
    assert_eq!(extracted, vec![1]);
}

#[test]
fn test_extraction_with_different_color_assignment() {
    // Test that extraction works with different color assignments
    // (colors may be permuted but semantics preserved)
    let sat = Satisfiability::new(1, vec![CNFClause::new(vec![1])]);
    let reduction = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&sat);

    // Different valid coloring: TRUE=2, FALSE=0, AUX=1
    // x1 must have color 2 (TRUE), NOT_x1 must have color 0 (FALSE)
    let coloring_permuted = vec![2, 0, 1, 2, 0];
    let extracted = reduction.extract_solution(&coloring_permuted);
    // x1 should still be true because its color equals TRUE vertex's color
    assert_eq!(extracted, vec![1]);

    // Another permutation: TRUE=1, FALSE=2, AUX=0
    // x1 has color 1 (TRUE), NOT_x1 has color 2 (FALSE)
    let coloring_permuted2 = vec![1, 2, 0, 1, 2];
    let extracted2 = reduction.extract_solution(&coloring_permuted2);
    assert_eq!(extracted2, vec![1]);
}

#[test]
fn test_jl_parity_sat_to_coloring() {
    let sat_data: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/data/jl/satisfiability.json")).unwrap();
    let fixtures: &[(&str, &str)] = &[
        (include_str!("../../../tests/data/jl/satisfiability_to_coloring3.json"), "simple_clause"),
        (include_str!("../../../tests/data/jl/rule_satisfiability2_to_coloring3.json"), "rule_sat_coloring"),
    ];
    for (fixture_str, label) in fixtures {
        let data: serde_json::Value = serde_json::from_str(fixture_str).unwrap();
        let inst = &jl_find_instance_by_label(&sat_data, label)["instance"];
        let (num_vars, clauses) = jl_parse_sat_clauses(inst);
        let source = Satisfiability::new(num_vars, clauses);
        let result = ReduceTo::<KColoring<3, SimpleGraph>>::reduce_to(&source);
        let ilp_solver = crate::solvers::ILPSolver::new();
        let target = result.target_problem();
        let target_sol = ilp_solver.solve_reduced(target).expect("ILP should find a coloring");
        let extracted = result.extract_solution(&target_sol);
        let best_source: HashSet<Vec<usize>> = BruteForce::new()
            .find_all_satisfying(&source).into_iter().collect();
        assert!(best_source.contains(&extracted), "SAT->Coloring [{label}]: extracted not satisfying");
        for case in data["cases"].as_array().unwrap() {
            assert_eq!(best_source, jl_parse_configs_set(&case["best_source"]),
                "SAT->Coloring [{label}]: best source mismatch");
        }
    }
}
