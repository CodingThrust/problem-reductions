use problemreductions::models::algebraic::SimultaneousIncongruences;
use problemreductions::solvers::BruteForce;
use problemreductions::traits::Problem;

#[test]
fn test_simultaneous_incongruences_issue_example() {
    // pairs: (a_i, b_i) meaning x != a_i (mod b_i)
    let problem = SimultaneousIncongruences::new(vec![(2, 2), (1, 3), (2, 5), (3, 7)]).unwrap();

    assert_eq!(problem.num_pairs(), 4);
    assert_eq!(problem.pairs(), &[(2, 2), (1, 3), (2, 5), (3, 7)]);
    assert_eq!(problem.lcm_moduli(), 210);
    assert_eq!(problem.dims(), vec![210]);
    // x=5: 5%2=1!=0(=2%2), 5%3=2!=1, 5%5=0!=2, 5%7=5!=3 => valid
    assert!(problem.evaluate(&[5]));
    // x=2: 2%2=0=2%2 => invalid (first incongruence violated)
    assert!(!problem.evaluate(&[2]));
}

#[test]
fn test_simultaneous_incongruences_solver_finds_witness() {
    let problem = SimultaneousIncongruences::new(vec![(2, 2), (1, 3), (2, 5), (3, 7)]).unwrap();
    let solver = BruteForce::new();

    let witness = solver.find_witness(&problem);
    // x=1: 1%2=1!=0, 1%3=1!=1? No, 1%3=1=1 so invalid!
    // x=3: 3%2=1!=0, 3%3=0!=1, 3%5=3!=2, 3%7=3!=3 => valid. First valid.
    assert!(witness.is_some());
    let w = witness.unwrap();
    assert!(problem.evaluate(&w));
}
