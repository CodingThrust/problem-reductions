// # Minesweeper Consistency Check
//
// ## This Example
// - Instance: 3x3 grid with a classic pattern
// - Revealed cells: (0,0)=1, (1,0)=1, (1,1)=2, (2,0)=0, (2,1)=1
// - Unrevealed cells: (0,1), (0,2), (1,2), (2,2)
// - Solution: mines at (0,1) and (1,2)

use problemreductions::models::misc::Minesweeper;
use problemreductions::solvers::{BruteForce, Solver};
use problemreductions::traits::Problem;

pub fn run() {
    println!("=== Minesweeper Consistency ===\n");

    // Grid layout:
    //   1 ? ?
    //   1 2 ?
    //   0 1 ?
    let problem = Minesweeper::new(
        3,
        3,
        vec![(0, 0, 1), (1, 0, 1), (1, 1, 2), (2, 0, 0), (2, 1, 1)],
        vec![(0, 1), (0, 2), (1, 2), (2, 2)],
    );

    println!("Grid: {}x{}", problem.rows(), problem.cols());
    println!("Revealed cells: {:?}", problem.revealed());
    println!("Unrevealed cells: {:?}", problem.unrevealed());
    println!("Variables: {}\n", problem.num_variables());

    let solver = BruteForce::new();
    match solver.find_satisfying(&problem) {
        Some(solution) => {
            println!("Satisfying assignment found!");
            println!("Config: {:?}", solution);
            println!("Verified: {}", problem.evaluate(&solution));

            // Show mine placement
            println!("\nMine placement:");
            for (i, &(r, c)) in problem.unrevealed().iter().enumerate() {
                if solution[i] == 1 {
                    println!("  Mine at ({}, {})", r, c);
                }
            }
        }
        None => {
            println!("No satisfying assignment exists (inconsistent grid).");
        }
    }
}

fn main() {
    run()
}
