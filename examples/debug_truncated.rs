//! Debug truncatedtetrahedron weighted test failure
use problemreductions::models::optimization::{ILP, LinearConstraint, ObjectiveSense};
use problemreductions::rules::unitdiskmapping::{map_graph_triangular, map_weights, trace_centers};
use problemreductions::solvers::ILPSolver;
use problemreductions::topology::{smallgraph, Graph};

fn is_independent_set(edges: &[(usize, usize)], config: &[usize]) -> bool {
    for &(u, v) in edges {
        if config.get(u).copied().unwrap_or(0) > 0 && config.get(v).copied().unwrap_or(0) > 0 {
            return false;
        }
    }
    true
}

fn main() {
    let (n, edges) = smallgraph("truncatedtetrahedron").unwrap();
    println!("Graph: {} vertices, {} edges", n, edges.len());
    println!("Edges: {:?}", edges);

    let result = map_graph_triangular(n, &edges);
    println!("\nGrid: {} vertices", result.grid_graph.num_vertices());

    // Use uniform weights
    let source_weights = vec![0.5; n];
    let mapped_weights = map_weights(&result, &source_weights);

    // Solve weighted MIS
    let grid_edges = result.grid_graph.edges().to_vec();
    let num_grid = result.grid_graph.num_vertices();

    let constraints: Vec<LinearConstraint> = grid_edges
        .iter()
        .map(|&(i, j)| LinearConstraint::le(vec![(i, 1.0), (j, 1.0)], 1.0))
        .collect();

    let objective: Vec<(usize, f64)> = mapped_weights
        .iter()
        .enumerate()
        .map(|(i, &w)| (i, w))
        .collect();

    let ilp = ILP::binary(num_grid, constraints, objective, ObjectiveSense::Maximize);
    let solver = ILPSolver::new();
    let grid_config: Vec<usize> = solver
        .solve(&ilp)
        .map(|sol| sol.iter().map(|&x| if x > 0 { 1 } else { 0 }).collect())
        .unwrap_or_else(|| vec![0; num_grid]);

    // Get centers
    let centers = trace_centers(&result);
    println!("\nCenters ({}):", centers.len());
    for (i, &(row, col)) in centers.iter().enumerate() {
        println!("  Vertex {}: ({}, {})", i, row, col);
    }

    // Extract config at centers
    let center_config: Vec<usize> = centers
        .iter()
        .map(|&(row, col)| {
            for (i, node) in result.grid_graph.nodes().iter().enumerate() {
                if node.row == row as i32 && node.col == col as i32 {
                    return grid_config[i];
                }
            }
            0
        })
        .collect();

    println!("\nCenter config: {:?}", center_config);
    println!("Selected vertices: {:?}", center_config.iter().enumerate()
        .filter(|(_, &v)| v > 0).map(|(i, _)| i).collect::<Vec<_>>());

    // Check which edges are violated
    let valid = is_independent_set(&edges, &center_config);
    println!("\nIs valid IS: {}", valid);

    if !valid {
        println!("Violated edges:");
        for &(u, v) in &edges {
            if center_config.get(u).copied().unwrap_or(0) > 0
                && center_config.get(v).copied().unwrap_or(0) > 0 {
                println!("  ({}, {})", u, v);
            }
        }
    }
}
