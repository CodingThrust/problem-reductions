//! Integration tests for problem reductions.
//!
//! These tests verify that reduction chains work correctly and
//! solutions can be properly extracted through the reduction pipeline.

use problemreductions::prelude::*;
use problemreductions::topology::SimpleGraph;

/// Tests for MaximumIndependentSet <-> MinimumVertexCover reductions.
mod is_vc_reductions {
    use super::*;

    #[test]
    fn test_is_to_vc_basic() {
        // Triangle graph
        let is_problem =
            MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

        // Reduce IS to VC
        let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
        let vc_problem = result.target_problem();

        // Same graph structure
        assert_eq!(vc_problem.num_vertices(), 3);
        assert_eq!(vc_problem.num_edges(), 3);

        // Solve the target VC problem
        let solver = BruteForce::new();
        let vc_solutions = solver.find_all_best(vc_problem);

        // Extract back to IS solution
        let is_solution = result.extract_solution(&vc_solutions[0]);

        // Solution should be valid for original problem
        assert!(is_problem.evaluate(&is_solution).is_valid());
    }

    #[test]
    fn test_vc_to_is_basic() {
        // Path graph
        let vc_problem =
            MinimumVertexCover::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

        // Reduce VC to IS
        let result = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&vc_problem);
        let is_problem = result.target_problem();

        // Same graph structure
        assert_eq!(is_problem.num_vertices(), 4);
        assert_eq!(is_problem.num_edges(), 3);

        // Solve the target IS problem
        let solver = BruteForce::new();
        let is_solutions = solver.find_all_best(is_problem);

        // Extract back to VC solution
        let vc_solution = result.extract_solution(&is_solutions[0]);

        // Solution should be valid for original problem
        assert!(vc_problem.evaluate(&vc_solution).is_valid());
    }

    #[test]
    fn test_is_vc_roundtrip() {
        let original =
            MaximumIndependentSet::<SimpleGraph, i32>::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4)]);

        // IS -> VC
        let to_vc = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&original);
        let vc_problem = to_vc.target_problem();

        // VC -> IS
        let back_to_is = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(vc_problem);
        let final_is = back_to_is.target_problem();

        // Should have same structure
        assert_eq!(final_is.num_vertices(), original.num_vertices());
        assert_eq!(final_is.num_edges(), original.num_edges());

        // Solve the final problem
        let solver = BruteForce::new();
        let solutions = solver.find_all_best(final_is);

        // Extract through the chain
        let intermediate_sol = back_to_is.extract_solution(&solutions[0]);
        let original_sol = to_vc.extract_solution(&intermediate_sol);

        // Should be valid
        assert!(original.evaluate(&original_sol).is_valid());
    }

    #[test]
    fn test_is_vc_weighted() {
        let is_problem = MaximumIndependentSet::with_weights(3, vec![(0, 1)], vec![10, 1, 5]);

        let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is_problem);
        let vc_problem = result.target_problem();

        // Weights should be preserved
        assert_eq!(vc_problem.weights(), &[10, 1, 5]);
    }

    #[test]
    fn test_is_vc_optimal_complement() {
        // For any graph: |max IS| + |min VC| = n
        let edges = vec![(0, 1), (1, 2), (2, 3), (0, 3)];
        let n = 4;

        let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(n, edges.clone());
        let vc_problem = MinimumVertexCover::<SimpleGraph, i32>::new(n, edges);

        let solver = BruteForce::new();

        // Solve IS, reduce to VC solution
        let is_solutions = solver.find_all_best(&is_problem);
        let max_is = is_solutions[0].iter().sum::<usize>();

        let vc_solutions = solver.find_all_best(&vc_problem);
        let min_vc = vc_solutions[0].iter().sum::<usize>();

        assert_eq!(max_is + min_vc, n);
    }
}

/// Tests for MaximumIndependentSet <-> MaximumSetPacking reductions.
mod is_sp_reductions {
    use super::*;

    #[test]
    fn test_is_to_sp_basic() {
        // Triangle graph - each vertex's incident edges become a set
        let is_problem =
            MaximumIndependentSet::<SimpleGraph, i32>::new(3, vec![(0, 1), (1, 2), (0, 2)]);

        let result = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is_problem);
        let sp_problem = result.target_problem();

        // 3 sets (one per vertex)
        assert_eq!(sp_problem.num_sets(), 3);

        // Solve
        let solver = BruteForce::new();
        let sp_solutions = solver.find_all_best(sp_problem);

        // Extract to IS solution
        let is_solution = result.extract_solution(&sp_solutions[0]);

        assert!(is_problem.evaluate(&is_solution).is_valid());
    }

    #[test]
    fn test_sp_to_is_basic() {
        // Disjoint sets pack perfectly
        let sets = vec![vec![0, 1], vec![2, 3], vec![4]];
        let sp_problem = MaximumSetPacking::<i32>::new(sets);

        let result = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp_problem);
        let is_problem = result.target_problem();

        // Should have an edge for each pair of overlapping sets (none here)
        assert_eq!(is_problem.num_edges(), 0);

        // Solve
        let solver = BruteForce::new();
        let is_solutions = solver.find_all_best(is_problem);

        // Extract to SP solution
        let sp_solution = result.extract_solution(&is_solutions[0]);

        // All sets can be packed (disjoint)
        assert_eq!(sp_solution.iter().sum::<usize>(), 3);
        assert!(sp_problem.evaluate(&sp_solution).is_valid());
    }

    #[test]
    fn test_is_sp_roundtrip() {
        let original =
            MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

        // IS -> SP
        let to_sp = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&original);
        let sp_problem = to_sp.target_problem();

        // Solve SP
        let solver = BruteForce::new();
        let sp_solutions = solver.find_all_best(sp_problem);

        // Extract to IS solution
        let is_solution = to_sp.extract_solution(&sp_solutions[0]);

        // Valid for original
        assert!(original.evaluate(&is_solution).is_valid());

        // Should match directly solving IS
        let direct_solutions = solver.find_all_best(&original);
        let direct_max = direct_solutions[0].iter().sum::<usize>();
        let reduced_max = is_solution.iter().sum::<usize>();

        assert_eq!(direct_max, reduced_max);
    }
}

/// Tests for SpinGlass <-> QUBO reductions.
mod sg_qubo_reductions {
    use super::*;

    #[test]
    fn test_sg_to_qubo_basic() {
        // Simple 2-spin system
        let sg = SpinGlass::<SimpleGraph, _>::new(2, vec![((0, 1), -1.0)], vec![0.5, -0.5]);

        let result = ReduceTo::<QUBO>::reduce_to(&sg);
        let qubo = result.target_problem();

        assert_eq!(qubo.num_variables(), 2);

        // Solve QUBO
        let solver = BruteForce::new();
        let qubo_solutions = solver.find_all_best(qubo);

        // Extract to SG solution
        let sg_solution = result.extract_solution(&qubo_solutions[0]);
        assert_eq!(sg_solution.len(), 2);
    }

    #[test]
    fn test_qubo_to_sg_basic() {
        // QUBO::new takes linear terms and quadratic terms separately
        let qubo = QUBO::new(vec![1.0, -1.0], vec![((0, 1), 0.5)]);

        let result = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&qubo);
        let sg = result.target_problem();

        assert_eq!(sg.num_spins(), 2);

        // Solve SG
        let solver = BruteForce::new();
        let sg_solutions = solver.find_all_best(sg);

        // Extract to QUBO solution
        let qubo_solution = result.extract_solution(&sg_solutions[0]);
        assert_eq!(qubo_solution.len(), 2);
    }

    #[test]
    fn test_sg_qubo_energy_preservation() {
        // The reduction should preserve optimal energy (up to constant)
        let sg = SpinGlass::<SimpleGraph, _>::new(
            3,
            vec![((0, 1), -1.0), ((1, 2), 1.0)],
            vec![0.0, 0.0, 0.0],
        );

        let result = ReduceTo::<QUBO>::reduce_to(&sg);
        let qubo = result.target_problem();

        // Check that ground states correspond
        let solver = BruteForce::new();

        let sg_solutions = solver.find_all_best(&sg);
        let qubo_solutions = solver.find_all_best(qubo);

        // Extract QUBO solution back to SG
        let extracted = result.extract_solution(&qubo_solutions[0]);

        // Convert solutions to spins for energy computation
        // SpinGlass::config_to_spins converts 0/1 configs to -1/+1 spins
        let sg_spins = SpinGlass::<SimpleGraph, f64>::config_to_spins(&sg_solutions[0]);
        let extracted_spins = SpinGlass::<SimpleGraph, f64>::config_to_spins(&extracted);

        // Should be among optimal SG solutions (or equivalent)
        let sg_energy = sg.compute_energy(&sg_spins);
        let extracted_energy = sg.compute_energy(&extracted_spins);

        // Energies should match for optimal solutions
        assert!((sg_energy - extracted_energy).abs() < 1e-10);
    }
}

/// Tests for SpinGlass <-> MaxCut reductions.
mod sg_maxcut_reductions {
    use super::*;

    #[test]
    fn test_sg_to_maxcut_basic() {
        // Antiferromagnetic on triangle (frustrated)
        let sg = SpinGlass::<SimpleGraph, _>::new(
            3,
            vec![((0, 1), 1), ((1, 2), 1), ((0, 2), 1)],
            vec![0, 0, 0],
        );

        let result = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
        let maxcut = result.target_problem();

        // Same number of vertices
        assert_eq!(maxcut.num_vertices(), 3);

        // Solve MaxCut
        let solver = BruteForce::new();
        let maxcut_solutions = solver.find_all_best(maxcut);

        // Extract to SG solution
        let sg_solution = result.extract_solution(&maxcut_solutions[0]);
        assert_eq!(sg_solution.len(), 3);
    }

    #[test]
    fn test_maxcut_to_sg_basic() {
        let maxcut = MaxCut::new(3, vec![(0, 1, 2), (1, 2, 1), (0, 2, 3)]);

        let result = ReduceTo::<SpinGlass<SimpleGraph, i32>>::reduce_to(&maxcut);
        let sg = result.target_problem();

        // Same number of spins
        assert_eq!(sg.num_spins(), 3);

        // Solve SG
        let solver = BruteForce::new();
        let sg_solutions = solver.find_all_best(sg);

        // Extract to MaxCut solution
        let maxcut_solution = result.extract_solution(&sg_solutions[0]);
        assert_eq!(maxcut_solution.len(), 3);
    }

    #[test]
    fn test_sg_maxcut_optimal_correspondence() {
        // For pure antiferromagnetic SG (J > 0), optimal <-> max cut
        let sg = SpinGlass::<SimpleGraph, _>::new(
            4,
            vec![((0, 1), 1), ((1, 2), 1), ((2, 3), 1), ((0, 3), 1)],
            vec![0, 0, 0, 0],
        );

        let result = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
        let maxcut = result.target_problem();

        let solver = BruteForce::new();

        // Solve both
        let sg_solutions = solver.find_all_best(&sg);
        let maxcut_solutions = solver.find_all_best(maxcut);

        // Extract MaxCut solution back to SG
        let extracted = result.extract_solution(&maxcut_solutions[0]);

        // Convert solutions to spins for energy computation
        // SpinGlass::config_to_spins converts 0/1 configs to -1/+1 spins
        let direct_spins = SpinGlass::<SimpleGraph, i32>::config_to_spins(&sg_solutions[0]);
        let extracted_spins = SpinGlass::<SimpleGraph, i32>::config_to_spins(&extracted);

        // Should have same energy as directly solved SG
        let direct_energy = sg.compute_energy(&direct_spins);
        let extracted_energy = sg.compute_energy(&extracted_spins);

        assert_eq!(direct_energy, extracted_energy);
    }
}

/// Tests for topology types integration.
mod topology_tests {
    use super::*;
    use problemreductions::topology::{HyperGraph, UnitDiskGraph};

    #[test]
    fn test_hypergraph_to_setpacking() {
        // HyperGraph can be seen as a MaximumSetPacking problem
        let hg = HyperGraph::new(5, vec![vec![0, 1, 2], vec![2, 3], vec![3, 4]]);

        // Convert hyperedges to sets for MaximumSetPacking
        let sets: Vec<Vec<usize>> = hg.edges().to_vec();
        let sp = MaximumSetPacking::<i32>::new(sets);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&sp);

        assert!(sp.evaluate(&solutions[0]).is_valid());
    }

    #[test]
    fn test_unit_disk_graph_to_independent_set() {
        // UDG with some overlapping points
        let positions = vec![
            (0.0, 0.0),
            (0.5, 0.0), // Close to 0
            (2.0, 0.0), // Far from 0 and 1
            (2.5, 0.0), // Close to 2
        ];
        let udg = UnitDiskGraph::new(positions, 1.0);

        // Extract edges
        let edges = udg.edges().to_vec();
        let is_problem = MaximumIndependentSet::<SimpleGraph, i32>::new(4, edges);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(&is_problem);

        // Vertices 0-1 are connected, 2-3 are connected
        // Max IS: {0, 2} or {0, 3} or {1, 2} or {1, 3} = size 2
        assert_eq!(solutions[0].iter().sum::<usize>(), 2);
    }
}

/// Tests for TruthTable integration with reductions.
mod truth_table_tests {
    use problemreductions::truth_table::TruthTable;

    #[test]
    fn test_truth_table_to_sat() {
        // Create a simple truth table (AND gate)
        let and_gate = TruthTable::and(2);

        // Find satisfying assignments
        let satisfying = and_gate.satisfying_assignments();

        // AND gate: only [T, T] satisfies
        assert_eq!(satisfying.len(), 1);
        assert_eq!(satisfying[0], vec![true, true]);
    }

    #[test]
    fn test_truth_table_xor_to_sat() {
        // XOR has exactly 2^(n-1) satisfying assignments for n inputs
        let xor3 = TruthTable::xor(3);
        let satisfying = xor3.satisfying_assignments();

        // 3-XOR: exactly 4 satisfying assignments
        assert_eq!(satisfying.len(), 4);

        // Each should have odd number of true inputs
        for assignment in &satisfying {
            let true_count = assignment.iter().filter(|&&b| b).count();
            assert_eq!(true_count % 2, 1);
        }
    }

    #[test]
    fn test_truth_table_combined() {
        // Test combining truth tables (useful for circuit construction)
        let a = TruthTable::and(2);
        let b = TruthTable::or(2);

        // a AND b (element-wise AND of two truth tables)
        let combined = a.and_with(&b);

        // AND result: [F,F,F,T], OR result: [F,T,T,T]
        // Combined: [F,F,F,T]
        assert_eq!(combined.outputs_vec(), vec![false, false, false, true]);
    }
}

/// Tests for QUBO reductions against ground truth JSON.
mod qubo_reductions {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct QuboOptimal {
        value: f64,
        configs: Vec<Vec<usize>>,
    }

    #[derive(Deserialize)]
    struct ISToQuboData {
        source: ISSource,
        qubo_num_vars: usize,
        qubo_optimal: QuboOptimal,
    }

    #[derive(Deserialize)]
    struct ISSource {
        num_vertices: usize,
        edges: Vec<(usize, usize)>,
    }

    #[test]
    fn test_is_to_qubo_ground_truth() {
        let json =
            std::fs::read_to_string("tests/data/qubo/maximumindependentset_to_qubo.json").unwrap();
        let data: ISToQuboData = serde_json::from_str(&json).unwrap();

        let is = MaximumIndependentSet::<SimpleGraph, i32>::new(
            data.source.num_vertices,
            data.source.edges,
        );
        let reduction = ReduceTo::<QUBO>::reduce_to(&is);
        let qubo = reduction.target_problem();

        assert_eq!(qubo.num_variables(), data.qubo_num_vars);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(qubo);

        // All QUBO optimal solutions should extract to valid IS solutions
        for sol in &solutions {
            let extracted = reduction.extract_solution(sol);
            assert!(is.evaluate(&extracted).is_valid());
        }

        // Optimal IS size should match ground truth
        let gt_is_size: usize = data.qubo_optimal.configs[0].iter().sum();
        let our_is_size: usize = reduction.extract_solution(&solutions[0]).iter().sum();
        assert_eq!(our_is_size, gt_is_size);
    }

    #[derive(Deserialize)]
    struct VCToQuboData {
        source: VCSource,
        qubo_num_vars: usize,
        qubo_optimal: QuboOptimal,
    }

    #[derive(Deserialize)]
    struct VCSource {
        num_vertices: usize,
        edges: Vec<(usize, usize)>,
    }

    #[test]
    fn test_vc_to_qubo_ground_truth() {
        let json =
            std::fs::read_to_string("tests/data/qubo/minimumvertexcover_to_qubo.json").unwrap();
        let data: VCToQuboData = serde_json::from_str(&json).unwrap();

        let vc = MinimumVertexCover::<SimpleGraph, i32>::new(
            data.source.num_vertices,
            data.source.edges,
        );
        let reduction = ReduceTo::<QUBO>::reduce_to(&vc);
        let qubo = reduction.target_problem();

        assert_eq!(qubo.num_variables(), data.qubo_num_vars);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(qubo);

        for sol in &solutions {
            let extracted = reduction.extract_solution(sol);
            assert!(vc.evaluate(&extracted).is_valid());
        }

        // Optimal VC size should match ground truth
        let gt_vc_size: usize = data.qubo_optimal.configs[0].iter().sum();
        let our_vc_size: usize = reduction.extract_solution(&solutions[0]).iter().sum();
        assert_eq!(our_vc_size, gt_vc_size);
    }

    #[derive(Deserialize)]
    struct ColoringToQuboData {
        source: ColoringSource,
        qubo_num_vars: usize,
        qubo_optimal: QuboOptimal,
    }

    #[derive(Deserialize)]
    struct ColoringSource {
        num_vertices: usize,
        edges: Vec<(usize, usize)>,
        num_colors: usize,
    }

    #[test]
    fn test_coloring_to_qubo_ground_truth() {
        let json = std::fs::read_to_string("tests/data/qubo/coloring_to_qubo.json").unwrap();
        let data: ColoringToQuboData = serde_json::from_str(&json).unwrap();

        assert_eq!(data.source.num_colors, 3);

        let kc = KColoring::<3, SimpleGraph>::new(data.source.num_vertices, data.source.edges);
        let reduction = ReduceTo::<QUBO>::reduce_to(&kc);
        let qubo = reduction.target_problem();

        assert_eq!(qubo.num_variables(), data.qubo_num_vars);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(qubo);

        for sol in &solutions {
            let extracted = reduction.extract_solution(sol);
            assert!(kc.evaluate(&extracted));
        }

        // Same number of optimal colorings as ground truth
        assert_eq!(solutions.len(), data.qubo_optimal.configs.len());
    }

    #[derive(Deserialize)]
    struct SPToQuboData {
        source: SPSource,
        qubo_num_vars: usize,
        qubo_optimal: QuboOptimal,
    }

    #[derive(Deserialize)]
    struct SPSource {
        sets: Vec<Vec<usize>>,
        weights: Vec<f64>,
    }

    #[test]
    fn test_setpacking_to_qubo_ground_truth() {
        let json =
            std::fs::read_to_string("tests/data/qubo/maximumsetpacking_to_qubo.json").unwrap();
        let data: SPToQuboData = serde_json::from_str(&json).unwrap();

        let sp = MaximumSetPacking::with_weights(data.source.sets, data.source.weights);
        let reduction = ReduceTo::<QUBO>::reduce_to(&sp);
        let qubo = reduction.target_problem();

        assert_eq!(qubo.num_variables(), data.qubo_num_vars);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(qubo);

        for sol in &solutions {
            let extracted = reduction.extract_solution(sol);
            assert!(sp.evaluate(&extracted).is_valid());
        }

        // Optimal packing should match ground truth
        let gt_selected: usize = data.qubo_optimal.configs[0].iter().sum();
        let our_selected: usize = reduction.extract_solution(&solutions[0]).iter().sum();
        assert_eq!(our_selected, gt_selected);
    }

    #[derive(Deserialize)]
    struct KSatToQuboData {
        source: KSatSource,
        qubo_num_vars: usize,
        qubo_optimal: QuboOptimal,
    }

    #[derive(Deserialize)]
    struct KSatSource {
        num_variables: usize,
        clauses: Vec<Vec<KSatLiteral>>,
    }

    #[derive(Deserialize)]
    struct KSatLiteral {
        variable: usize,
        negated: bool,
    }

    #[test]
    fn test_ksat_to_qubo_ground_truth() {
        let json = std::fs::read_to_string("tests/data/qubo/ksatisfiability_to_qubo.json").unwrap();
        let data: KSatToQuboData = serde_json::from_str(&json).unwrap();

        // Convert JSON clauses to CNFClause (1-indexed signed literals)
        let clauses: Vec<CNFClause> = data
            .source
            .clauses
            .iter()
            .map(|lits| {
                let signed: Vec<i32> = lits
                    .iter()
                    .map(|l| {
                        let var = (l.variable + 1) as i32; // 0-indexed to 1-indexed
                        if l.negated {
                            -var
                        } else {
                            var
                        }
                    })
                    .collect();
                CNFClause::new(signed)
            })
            .collect();

        let ksat = KSatisfiability::<K2>::new(data.source.num_variables, clauses);
        let reduction = ReduceTo::<QUBO>::reduce_to(&ksat);
        let qubo = reduction.target_problem();

        assert_eq!(qubo.num_variables(), data.qubo_num_vars);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(qubo);

        for sol in &solutions {
            let extracted = reduction.extract_solution(sol);
            assert!(ksat.evaluate(&extracted));
        }

        // Verify extracted solution matches ground truth assignment
        let gt_config = &data.qubo_optimal.configs[0];
        let our_config = reduction.extract_solution(&solutions[0]);
        assert_eq!(&our_config, gt_config);
    }

    #[cfg(feature = "ilp")]
    #[derive(Deserialize)]
    struct ILPToQuboData {
        source: ILPSource,
        qubo_num_vars: usize,
        qubo_optimal: QuboOptimal,
    }

    #[cfg(feature = "ilp")]
    #[derive(Deserialize)]
    struct ILPSource {
        num_variables: usize,
        objective: Vec<f64>,
        constraints_lhs: Vec<Vec<f64>>,
        constraints_rhs: Vec<f64>,
        constraint_signs: Vec<i32>,
    }

    #[cfg(feature = "ilp")]
    #[test]
    fn test_ilp_to_qubo_ground_truth() {
        let json = std::fs::read_to_string("tests/data/qubo/ilp_to_qubo.json").unwrap();
        let data: ILPToQuboData = serde_json::from_str(&json).unwrap();

        // Build constraints from dense matrix
        let constraints: Vec<LinearConstraint> = data
            .source
            .constraints_lhs
            .iter()
            .zip(data.source.constraints_rhs.iter())
            .zip(data.source.constraint_signs.iter())
            .map(|((row, &rhs), &sign)| {
                let terms: Vec<(usize, f64)> = row
                    .iter()
                    .enumerate()
                    .filter(|(_, &c)| c.abs() > 1e-15)
                    .map(|(i, &c)| (i, c))
                    .collect();
                match sign {
                    -1 => LinearConstraint::le(terms, rhs),
                    0 => LinearConstraint::eq(terms, rhs),
                    1 => LinearConstraint::ge(terms, rhs),
                    _ => panic!("Invalid constraint sign: {}", sign),
                }
            })
            .collect();

        // Build objective (dense to sparse)
        let objective: Vec<(usize, f64)> = data
            .source
            .objective
            .iter()
            .enumerate()
            .filter(|(_, &c)| c.abs() > 1e-15)
            .map(|(i, &c)| (i, c))
            .collect();

        // The qubogen formula maximizes, so this is a Maximize ILP
        let ilp = ILP::binary(
            data.source.num_variables,
            constraints,
            objective,
            ObjectiveSense::Maximize,
        );
        let reduction = ReduceTo::<QUBO>::reduce_to(&ilp);
        let qubo = reduction.target_problem();

        // QUBO may have more variables (slack), but original count matches
        assert!(qubo.num_variables() >= data.qubo_num_vars);

        let solver = BruteForce::new();
        let solutions = solver.find_all_best(qubo);

        for sol in &solutions {
            let extracted = reduction.extract_solution(sol);
            assert!(ilp.evaluate(&extracted).is_valid());
        }

        // Optimal assignment should match ground truth
        let gt_config = &data.qubo_optimal.configs[0];
        let our_config = reduction.extract_solution(&solutions[0]);
        assert_eq!(&our_config, gt_config);
    }
}

/// Tests for File I/O with reductions.
mod io_tests {
    use super::*;
    use problemreductions::io::{from_json, to_json};

    #[test]
    fn test_serialize_reduce_deserialize() {
        let original =
            MaximumIndependentSet::<SimpleGraph, i32>::new(4, vec![(0, 1), (1, 2), (2, 3)]);

        // Serialize
        let json = to_json(&original).unwrap();

        // Deserialize
        let restored: MaximumIndependentSet<SimpleGraph, i32> = from_json(&json).unwrap();

        // Should have same structure
        assert_eq!(restored.num_vertices(), original.num_vertices());
        assert_eq!(restored.num_edges(), original.num_edges());

        // Reduce the restored problem
        let result = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&restored);
        let vc = result.target_problem();

        assert_eq!(vc.num_vertices(), 4);
        assert_eq!(vc.num_edges(), 3);
    }

    #[test]
    fn test_serialize_qubo_sg_roundtrip() {
        // Use from_matrix for simpler construction
        let qubo = QUBO::from_matrix(vec![vec![1.0, 0.5], vec![0.0, -1.0]]);

        // Serialize
        let json = to_json(&qubo).unwrap();

        // Deserialize
        let restored: QUBO = from_json(&json).unwrap();

        // Reduce to SG
        let result = ReduceTo::<SpinGlass<SimpleGraph, f64>>::reduce_to(&restored);
        let sg = result.target_problem();

        // Serialize the SG
        let sg_json = to_json(sg).unwrap();

        // Deserialize
        let sg_restored: SpinGlass<SimpleGraph, f64> = from_json(&sg_json).unwrap();

        assert_eq!(sg_restored.num_spins(), 2);
    }
}

/// End-to-end tests combining multiple features.
mod end_to_end {
    use super::*;

    #[test]
    fn test_full_pipeline_is_vc_sp() {
        // Start with an MaximumIndependentSet problem
        let is = MaximumIndependentSet::<SimpleGraph, i32>::new(
            5,
            vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4)],
        );

        // Solve directly
        let solver = BruteForce::new();
        let is_solutions = solver.find_all_best(&is);
        let direct_size = is_solutions[0].iter().sum::<usize>();

        // Reduce to VC and solve
        let to_vc = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(&is);
        let vc = to_vc.target_problem();
        let vc_solutions = solver.find_all_best(vc);
        let vc_extracted = to_vc.extract_solution(&vc_solutions[0]);
        let via_vc_size = vc_extracted.iter().sum::<usize>();

        // Reduce to MaximumSetPacking and solve
        let to_sp = ReduceTo::<MaximumSetPacking<i32>>::reduce_to(&is);
        let sp = to_sp.target_problem();
        let sp_solutions = solver.find_all_best(sp);
        let sp_extracted = to_sp.extract_solution(&sp_solutions[0]);
        let via_sp_size = sp_extracted.iter().sum::<usize>();

        // All should give same optimal size
        assert_eq!(direct_size, via_vc_size);
        assert_eq!(direct_size, via_sp_size);
    }

    #[test]
    fn test_full_pipeline_sg_maxcut() {
        // Start with SpinGlass (integer weights for MaxCut compatibility)
        let sg = SpinGlass::<SimpleGraph, _>::new(
            4,
            vec![((0, 1), 1), ((1, 2), -1), ((2, 3), 1), ((0, 3), -1)],
            vec![0, 0, 0, 0],
        );

        // Solve directly
        let solver = BruteForce::new();
        let sg_solutions = solver.find_all_best(&sg);

        // Convert usize solution to i32 spin values for compute_energy
        let direct_spins: Vec<i32> = sg_solutions[0].iter().map(|&x| x as i32).collect();
        let direct_energy = sg.compute_energy(&direct_spins);

        // Reduce to MaxCut and solve
        let to_maxcut = ReduceTo::<MaxCut<SimpleGraph, i32>>::reduce_to(&sg);
        let maxcut = to_maxcut.target_problem();
        let maxcut_solutions = solver.find_all_best(maxcut);
        let maxcut_extracted = to_maxcut.extract_solution(&maxcut_solutions[0]);

        // Convert extracted solution to spins for energy computation
        let extracted_spins: Vec<i32> = maxcut_extracted.iter().map(|&x| x as i32).collect();
        let via_maxcut_energy = sg.compute_energy(&extracted_spins);

        // Should give same optimal energy
        assert_eq!(direct_energy, via_maxcut_energy);
    }

    #[test]
    fn test_chain_reduction_sp_is_vc() {
        // MaximumSetPacking -> MaximumIndependentSet -> MinimumVertexCover
        let sets = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3]];
        let sp = MaximumSetPacking::<i32>::new(sets);

        // SP -> IS
        let sp_to_is = ReduceTo::<MaximumIndependentSet<SimpleGraph, i32>>::reduce_to(&sp);
        let is = sp_to_is.target_problem();

        // IS -> VC
        let is_to_vc = ReduceTo::<MinimumVertexCover<SimpleGraph, i32>>::reduce_to(is);
        let vc = is_to_vc.target_problem();

        // Solve VC
        let solver = BruteForce::new();
        let vc_solutions = solver.find_all_best(vc);

        // Extract back through chain
        let is_sol = is_to_vc.extract_solution(&vc_solutions[0]);
        let sp_sol = sp_to_is.extract_solution(&is_sol);

        // Should be valid MaximumSetPacking
        assert!(sp.evaluate(&sp_sol).is_valid());
    }
}
