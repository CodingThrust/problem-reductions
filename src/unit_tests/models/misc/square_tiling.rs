use crate::models::misc::SquareTiling;
use crate::solvers::BruteForce;
use crate::traits::Problem;
use crate::types::Or;

/// Positive example from the issue: 3 colors, 4 tiles, 2x2 grid.
/// Tiles: t0=(0,1,2,0), t1=(0,0,2,1), t2=(2,1,0,0), t3=(2,0,0,1)
fn example_problem() -> SquareTiling {
    SquareTiling::new(
        3,
        vec![(0, 1, 2, 0), (0, 0, 2, 1), (2, 1, 0, 0), (2, 0, 0, 1)],
        2,
    )
}

#[test]
fn test_square_tiling_basic() {
    let problem = example_problem();
    assert_eq!(problem.num_colors(), 3);
    assert_eq!(problem.num_tiles(), 4);
    assert_eq!(problem.grid_size(), 2);
    assert_eq!(problem.tiles().len(), 4);
    assert_eq!(problem.dims(), vec![4; 4]);
    assert_eq!(problem.num_variables(), 4);
    assert_eq!(<SquareTiling as Problem>::NAME, "SquareTiling");
    assert_eq!(<SquareTiling as Problem>::variant(), vec![]);
}

#[test]
fn test_square_tiling_evaluate_valid() {
    let problem = example_problem();
    // Config [0, 1, 2, 3] means:
    //   (0,0)=t0, (0,1)=t1
    //   (1,0)=t2, (1,1)=t3
    // Horizontal: t0.right=1==t1.left=1, t2.right=1==t3.left=1
    // Vertical:   t0.bottom=2==t2.top=2, t1.bottom=2==t3.top=2
    assert_eq!(problem.evaluate(&[0, 1, 2, 3]), Or(true));
}

#[test]
fn test_square_tiling_evaluate_invalid_horizontal() {
    let problem = example_problem();
    // Config [0, 0, 2, 3]:
    //   (0,0)=t0, (0,1)=t0
    //   t0.right=1, t0.left=0 => mismatch
    assert_eq!(problem.evaluate(&[0, 0, 2, 3]), Or(false));
}

#[test]
fn test_square_tiling_evaluate_invalid_vertical() {
    let problem = example_problem();
    // Config [0, 1, 0, 3]:
    //   (0,0)=t0, (0,1)=t1
    //   (1,0)=t0, (1,1)=t3
    //   Vertical (0,0)-(1,0): t0.bottom=2, t0.top=0 => mismatch
    assert_eq!(problem.evaluate(&[0, 1, 0, 3]), Or(false));
}

#[test]
fn test_square_tiling_evaluate_wrong_length() {
    let problem = example_problem();
    assert_eq!(problem.evaluate(&[0, 1, 2]), Or(false));
    assert_eq!(problem.evaluate(&[0, 1, 2, 3, 0]), Or(false));
}

#[test]
fn test_square_tiling_evaluate_tile_index_out_of_range() {
    let problem = example_problem();
    assert_eq!(problem.evaluate(&[0, 1, 2, 4]), Or(false));
}

#[test]
fn test_square_tiling_solver_finds_witness() {
    let problem = example_problem();
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(problem.evaluate(&witness), Or(true));
}

#[test]
fn test_square_tiling_unsatisfiable_instance() {
    // Negative example from issue: only t0=(0,1,2,0) and t2=(2,1,0,0)
    // Both have right=1, left=0, so no horizontal match possible.
    let problem = SquareTiling::new(3, vec![(0, 1, 2, 0), (2, 1, 0, 0)], 2);
    let solver = BruteForce::new();
    assert!(solver.find_witness(&problem).is_none());
}

#[test]
fn test_square_tiling_single_cell() {
    // 1x1 grid: any single tile is a valid tiling
    let problem = SquareTiling::new(2, vec![(0, 1, 0, 1)], 1);
    assert_eq!(problem.evaluate(&[0]), Or(true));
    let solver = BruteForce::new();
    let witness = solver.find_witness(&problem).unwrap();
    assert_eq!(witness, vec![0]);
}

#[test]
fn test_square_tiling_serialization_round_trip() {
    let problem = example_problem();
    let json = serde_json::to_value(&problem).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "num_colors": 3,
            "tiles": [[0,1,2,0], [0,0,2,1], [2,1,0,0], [2,0,0,1]],
            "grid_size": 2,
        })
    );

    let restored: SquareTiling = serde_json::from_value(json).unwrap();
    assert_eq!(restored.num_colors(), problem.num_colors());
    assert_eq!(restored.num_tiles(), problem.num_tiles());
    assert_eq!(restored.grid_size(), problem.grid_size());
    assert_eq!(restored.tiles(), problem.tiles());
}

#[test]
fn test_square_tiling_deserialization_rejects_invalid() {
    let invalid_cases = [
        // Zero colors
        serde_json::json!({
            "num_colors": 0,
            "tiles": [[0, 0, 0, 0]],
            "grid_size": 1,
        }),
        // Empty tiles
        serde_json::json!({
            "num_colors": 2,
            "tiles": [],
            "grid_size": 1,
        }),
        // Zero grid size
        serde_json::json!({
            "num_colors": 2,
            "tiles": [[0, 0, 0, 0]],
            "grid_size": 0,
        }),
        // Color out of range
        serde_json::json!({
            "num_colors": 2,
            "tiles": [[0, 0, 0, 5]],
            "grid_size": 1,
        }),
    ];

    for invalid in invalid_cases {
        assert!(serde_json::from_value::<SquareTiling>(invalid).is_err());
    }
}

#[test]
#[should_panic(expected = "at least one color")]
fn test_square_tiling_zero_colors_panics() {
    SquareTiling::new(0, vec![(0, 0, 0, 0)], 1);
}

#[test]
#[should_panic(expected = "at least one tile")]
fn test_square_tiling_empty_tiles_panics() {
    SquareTiling::new(2, vec![], 1);
}

#[test]
#[should_panic(expected = "grid_size >= 1")]
fn test_square_tiling_zero_grid_size_panics() {
    SquareTiling::new(2, vec![(0, 0, 0, 0)], 0);
}

#[test]
#[should_panic(expected = "out of range")]
fn test_square_tiling_color_out_of_range_panics() {
    SquareTiling::new(2, vec![(0, 0, 0, 5)], 1);
}

#[test]
fn test_square_tiling_count_valid_tilings() {
    // Issue states 16 valid tilings out of 256 for the positive example
    let problem = example_problem();
    let solver = BruteForce::new();
    let witnesses = solver.find_all_witnesses(&problem);
    assert_eq!(witnesses.len(), 16);
}
