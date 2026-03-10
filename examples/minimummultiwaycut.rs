// MinimumMultiwayCut example: find minimum weight edge cut separating terminals.

use problemreductions::models::graph::MinimumMultiwayCut;
use problemreductions::topology::SimpleGraph;
use problemreductions::{BruteForce, Problem, Solver};

pub fn run() {
    // 5 vertices, terminals {0, 2, 4}
    // Edges with weights: (0,1)=2, (1,2)=3, (2,3)=1, (3,4)=2, (0,4)=4, (1,3)=5
    let graph = SimpleGraph::new(5, vec![(0, 1), (1, 2), (2, 3), (3, 4), (0, 4), (1, 3)]);
    let problem = MinimumMultiwayCut::new(graph, vec![0, 2, 4], vec![2, 3, 1, 2, 4, 5]);

    let solver = BruteForce::new();
    let best = solver.find_best(&problem).expect("should find a solution");
    let value = problem.evaluate(&best);

    println!("Optimal multiway cut: {:?}", best);
    println!("Cut weight: {:?}", value);

    // Export as JSON
    let json = serde_json::json!({
        "problem": problem,
        "solution": best,
        "objective": 8,
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

fn main() {
    run();
}
