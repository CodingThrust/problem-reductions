use problemreductions::models::graph::HamiltonianPath;
use problemreductions::topology::SimpleGraph;
use problemreductions::{BruteForce, Problem};

pub fn run() {
    // Instance 2 from issue: 6 vertices, 8 edges (non-trivial)
    let graph = SimpleGraph::new(
        6,
        vec![
            (0, 1),
            (0, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (3, 5),
            (4, 2),
            (5, 1),
        ],
    );
    let problem = HamiltonianPath::new(graph);

    println!("HamiltonianPath instance:");
    println!("  Vertices: {}", problem.num_vertices());
    println!("  Edges: {}", problem.num_edges());

    let json = serde_json::to_string_pretty(&problem).unwrap();
    println!("  JSON: {}", json);

    // Find all Hamiltonian paths
    let solver = BruteForce::new();
    let solutions = solver.find_all_satisfying(&problem);
    println!("  Solutions found: {}", solutions.len());

    for (i, sol) in solutions.iter().enumerate() {
        println!("  Path {}: {:?} (valid: {})", i, sol, problem.evaluate(sol));
    }
}

fn main() {
    run();
}
