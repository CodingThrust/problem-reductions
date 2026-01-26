//! Reductions between Satisfiability and K-Satisfiability problems.
//!
//! SAT -> K-SAT: Convert general CNF to K-literal clauses using:
//! - Padding with ancilla variables for clauses with < K literals
//! - Splitting with ancilla variables for clauses with > K literals
//!
//! K-SAT -> SAT: Trivial embedding (K-SAT is a special case of SAT)

use crate::models::satisfiability::{CNFClause, KSatisfiability, Satisfiability};
use crate::rules::traits::{ReduceTo, ReductionResult};
use crate::traits::Problem;
use crate::types::ProblemSize;
use num_traits::{Num, Zero};
use std::ops::AddAssign;

/// Result of reducing general SAT to K-SAT.
///
/// This reduction transforms a SAT formula into an equisatisfiable K-SAT formula
/// by introducing ancilla (auxiliary) variables.
#[derive(Debug, Clone)]
pub struct ReductionSATToKSAT<const K: usize, W> {
    /// Number of original variables in the source problem.
    source_num_vars: usize,
    /// The target K-SAT problem.
    target: KSatisfiability<K, W>,
    /// Size of the source problem.
    source_size: ProblemSize,
}

impl<const K: usize, W> ReductionResult for ReductionSATToKSAT<K, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Source = Satisfiability<W>;
    type Target = KSatisfiability<K, W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Only return the original variables, discarding ancillas
        target_solution[..self.source_num_vars].to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

/// Add a clause to the K-SAT formula, splitting or padding as necessary.
///
/// # Algorithm
/// - If clause has exactly K literals: add as-is
/// - If clause has < K literals: pad with ancilla variables (both positive and negative)
/// - If clause has > K literals: split recursively using ancilla variables
///
/// # Arguments
/// * `k` - Target number of literals per clause
/// * `clause` - The clause to add
/// * `result_clauses` - Output vector to append clauses to
/// * `next_var` - Next available variable number (1-indexed)
///
/// # Returns
/// Updated next_var after any ancilla variables are created
fn add_clause_to_ksat(
    k: usize,
    clause: &CNFClause,
    result_clauses: &mut Vec<CNFClause>,
    mut next_var: i32,
) -> i32 {
    let len = clause.len();

    if len == k {
        // Exact size: add as-is
        result_clauses.push(clause.clone());
    } else if len < k {
        // Too few literals: pad with ancilla variables
        // Create both positive and negative versions to maintain satisfiability
        // (a v b) with k=3 becomes (a v b v x) AND (a v b v -x)
        let ancilla = next_var;
        next_var += 1;

        // Add clause with positive ancilla
        let mut lits_pos = clause.literals.clone();
        lits_pos.push(ancilla);
        next_var = add_clause_to_ksat(k, &CNFClause::new(lits_pos), result_clauses, next_var);

        // Add clause with negative ancilla
        let mut lits_neg = clause.literals.clone();
        lits_neg.push(-ancilla);
        next_var = add_clause_to_ksat(k, &CNFClause::new(lits_neg), result_clauses, next_var);
    } else {
        // Too many literals: split using ancilla variable
        // (a v b v c v d) with k=3 becomes (a v b v x) AND (-x v c v d)
        assert!(k >= 3, "K must be at least 3 for splitting");

        let ancilla = next_var;
        next_var += 1;

        // First clause: first k-1 literals + positive ancilla
        let mut first_lits: Vec<i32> = clause.literals[..k - 1].to_vec();
        first_lits.push(ancilla);
        result_clauses.push(CNFClause::new(first_lits));

        // Remaining clause: negative ancilla + remaining literals
        let mut remaining_lits = vec![-ancilla];
        remaining_lits.extend_from_slice(&clause.literals[k - 1..]);
        let remaining_clause = CNFClause::new(remaining_lits);

        // Recursively process the remaining clause
        next_var = add_clause_to_ksat(k, &remaining_clause, result_clauses, next_var);
    }

    next_var
}

/// Implementation of SAT -> K-SAT reduction.
///
/// Note: We implement this for specific K values rather than generic K
/// because Rust's type system requires concrete implementations for
/// the `ReduceTo` trait pattern used in this crate.
macro_rules! impl_sat_to_ksat {
    ($k:expr) => {
        impl<W> ReduceTo<KSatisfiability<$k, W>> for Satisfiability<W>
        where
            W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
        {
            type Result = ReductionSATToKSAT<$k, W>;

            fn reduce_to(&self) -> Self::Result {
                let source_num_vars = self.num_vars();
                let mut result_clauses = Vec::new();
                let mut next_var = (source_num_vars + 1) as i32; // 1-indexed

                for clause in self.clauses() {
                    next_var =
                        add_clause_to_ksat($k, clause, &mut result_clauses, next_var);
                }

                // Calculate total number of variables (original + ancillas)
                let total_vars = (next_var - 1) as usize;

                let target = KSatisfiability::<$k, W>::new(total_vars, result_clauses);

                ReductionSATToKSAT {
                    source_num_vars,
                    target,
                    source_size: self.problem_size(),
                }
            }
        }
    };
}

// Implement for common K values
impl_sat_to_ksat!(3);
impl_sat_to_ksat!(4);
impl_sat_to_ksat!(5);

/// Result of reducing K-SAT to general SAT.
///
/// This is a trivial embedding since K-SAT is a special case of SAT.
#[derive(Debug, Clone)]
pub struct ReductionKSATToSAT<const K: usize, W> {
    /// The target SAT problem.
    target: Satisfiability<W>,
    /// Size of the source problem.
    source_size: ProblemSize,
}

impl<const K: usize, W> ReductionResult for ReductionKSATToSAT<K, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Source = KSatisfiability<K, W>;
    type Target = Satisfiability<W>;

    fn target_problem(&self) -> &Self::Target {
        &self.target
    }

    fn extract_solution(&self, target_solution: &[usize]) -> Vec<usize> {
        // Direct mapping - no transformation needed
        target_solution.to_vec()
    }

    fn source_size(&self) -> ProblemSize {
        self.source_size.clone()
    }

    fn target_size(&self) -> ProblemSize {
        self.target.problem_size()
    }
}

impl<const K: usize, W> ReduceTo<Satisfiability<W>> for KSatisfiability<K, W>
where
    W: Clone + Default + PartialOrd + Num + Zero + AddAssign + From<i32> + 'static,
{
    type Result = ReductionKSATToSAT<K, W>;

    fn reduce_to(&self) -> Self::Result {
        let clauses = self.clauses().to_vec();
        let target = Satisfiability::new(self.num_vars(), clauses);

        ReductionKSATToSAT {
            target,
            source_size: self.problem_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::{BruteForce, Solver};

    #[test]
    fn test_sat_to_3sat_exact_size() {
        // Clause already has 3 literals - should remain unchanged
        let sat = Satisfiability::<i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        assert_eq!(ksat.num_vars(), 3);
        assert_eq!(ksat.num_clauses(), 1);
        assert_eq!(ksat.clauses()[0].literals, vec![1, 2, 3]);
    }

    #[test]
    fn test_sat_to_3sat_padding() {
        // Clause has 2 literals - should be padded to 3
        // (a v b) becomes (a v b v x) AND (a v b v -x)
        let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, 2])]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // Should have 2 clauses (positive and negative ancilla)
        assert_eq!(ksat.num_clauses(), 2);
        // All clauses should have exactly 3 literals
        for clause in ksat.clauses() {
            assert_eq!(clause.len(), 3);
        }
    }

    #[test]
    fn test_sat_to_3sat_splitting() {
        // Clause has 4 literals - should be split
        // (a v b v c v d) becomes (a v b v x) AND (-x v c v d)
        let sat = Satisfiability::<i32>::new(4, vec![CNFClause::new(vec![1, 2, 3, 4])]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // Should have 2 clauses after splitting
        assert_eq!(ksat.num_clauses(), 2);
        // All clauses should have exactly 3 literals
        for clause in ksat.clauses() {
            assert_eq!(clause.len(), 3);
        }

        // Verify structure: first clause has positive ancilla, second has negative
        let c1 = &ksat.clauses()[0];
        let c2 = &ksat.clauses()[1];
        // First clause: [1, 2, 5] (ancilla is var 5)
        assert_eq!(c1.literals[0], 1);
        assert_eq!(c1.literals[1], 2);
        let ancilla = c1.literals[2];
        assert!(ancilla > 0);
        // Second clause: [-5, 3, 4]
        assert_eq!(c2.literals[0], -ancilla);
        assert_eq!(c2.literals[1], 3);
        assert_eq!(c2.literals[2], 4);
    }

    #[test]
    fn test_sat_to_3sat_large_clause() {
        // Clause has 5 literals - requires multiple splits
        // (a v b v c v d v e) -> (a v b v x1) AND (-x1 v c v x2) AND (-x2 v d v e)
        let sat = Satisfiability::<i32>::new(5, vec![CNFClause::new(vec![1, 2, 3, 4, 5])]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // Should have 3 clauses after splitting
        assert_eq!(ksat.num_clauses(), 3);
        // All clauses should have exactly 3 literals
        for clause in ksat.clauses() {
            assert_eq!(clause.len(), 3);
        }
    }

    #[test]
    fn test_sat_to_3sat_single_literal() {
        // Single literal clause - needs padding twice
        // (a) becomes (a v x v y) where we pad twice
        let sat = Satisfiability::<i32>::new(1, vec![CNFClause::new(vec![1])]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // With recursive padding: (a) -> (a v x) AND (a v -x)
        // Then each of those gets padded again
        // (a v x) -> (a v x v y) AND (a v x v -y)
        // (a v -x) -> (a v -x v z) AND (a v -x v -z)
        // Total: 4 clauses
        assert_eq!(ksat.num_clauses(), 4);
        for clause in ksat.clauses() {
            assert_eq!(clause.len(), 3);
        }
    }

    #[test]
    fn test_sat_to_3sat_preserves_satisfiability() {
        // Create a SAT formula and verify the 3-SAT version is equisatisfiable
        let sat = Satisfiability::<i32>::new(
            3,
            vec![
                CNFClause::new(vec![1, 2]),          // Needs padding
                CNFClause::new(vec![-1, 2, 3]),      // Already 3 literals
                CNFClause::new(vec![1, -2, 3, -3]), // Needs splitting (tautology for testing)
            ],
        );

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // Solve both problems
        let solver = BruteForce::new();

        let sat_solutions = solver.find_best(&sat);
        let ksat_solutions = solver.find_best(ksat);

        // If SAT is satisfiable, K-SAT should be too
        let sat_satisfiable = sat_solutions.iter().any(|s| sat.solution_size(s).is_valid);
        let ksat_satisfiable = ksat_solutions.iter().any(|s| ksat.solution_size(s).is_valid);

        assert_eq!(sat_satisfiable, ksat_satisfiable);

        // Extract solutions should map back correctly
        if ksat_satisfiable {
            for ksat_sol in &ksat_solutions {
                if ksat.solution_size(ksat_sol).is_valid {
                    let sat_sol = reduction.extract_solution(ksat_sol);
                    assert_eq!(sat_sol.len(), 3); // Original variable count
                }
            }
        }
    }

    #[test]
    fn test_sat_to_3sat_solution_extraction() {
        let sat = Satisfiability::<i32>::new(2, vec![CNFClause::new(vec![1, 2])]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // Solve K-SAT
        let solver = BruteForce::new();
        let ksat_solutions = solver.find_best(ksat);

        // Extract and verify solutions
        for ksat_sol in &ksat_solutions {
            if ksat.solution_size(ksat_sol).is_valid {
                let sat_sol = reduction.extract_solution(ksat_sol);
                // Should only have original 2 variables
                assert_eq!(sat_sol.len(), 2);
                // Should satisfy original problem
                assert!(sat.solution_size(&sat_sol).is_valid);
            }
        }
    }

    #[test]
    fn test_3sat_to_sat() {
        let ksat = KSatisfiability::<3, i32>::new(
            3,
            vec![
                CNFClause::new(vec![1, 2, 3]),
                CNFClause::new(vec![-1, -2, 3]),
            ],
        );

        let reduction = ReduceTo::<Satisfiability<i32>>::reduce_to(&ksat);
        let sat = reduction.target_problem();

        assert_eq!(sat.num_vars(), 3);
        assert_eq!(sat.num_clauses(), 2);

        // Verify clauses are preserved
        assert_eq!(sat.clauses()[0].literals, vec![1, 2, 3]);
        assert_eq!(sat.clauses()[1].literals, vec![-1, -2, 3]);
    }

    #[test]
    fn test_3sat_to_sat_solution_extraction() {
        let ksat = KSatisfiability::<3, i32>::new(3, vec![CNFClause::new(vec![1, 2, 3])]);

        let reduction = ReduceTo::<Satisfiability<i32>>::reduce_to(&ksat);

        let sol = vec![1, 0, 1];
        let extracted = reduction.extract_solution(&sol);
        assert_eq!(extracted, vec![1, 0, 1]);
    }

    #[test]
    fn test_roundtrip_sat_3sat_sat() {
        // SAT -> 3-SAT -> SAT roundtrip
        let original_sat = Satisfiability::<i32>::new(
            3,
            vec![CNFClause::new(vec![1, -2]), CNFClause::new(vec![2, 3])],
        );

        // SAT -> 3-SAT
        let to_ksat = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&original_sat);
        let ksat = to_ksat.target_problem();

        // 3-SAT -> SAT
        let to_sat = ReduceTo::<Satisfiability<i32>>::reduce_to(ksat);
        let final_sat = to_sat.target_problem();

        // Solve all three
        let solver = BruteForce::new();

        let orig_solutions = solver.find_best(&original_sat);
        let ksat_solutions = solver.find_best(ksat);
        let final_solutions = solver.find_best(final_sat);

        // All should be satisfiable
        assert!(orig_solutions.iter().any(|s| original_sat.solution_size(s).is_valid));
        assert!(ksat_solutions.iter().any(|s| ksat.solution_size(s).is_valid));
        assert!(final_solutions.iter().any(|s| final_sat.solution_size(s).is_valid));
    }

    #[test]
    fn test_sat_to_4sat() {
        let sat = Satisfiability::<i32>::new(
            4,
            vec![
                CNFClause::new(vec![1, 2]),       // Needs padding
                CNFClause::new(vec![1, 2, 3, 4]), // Exact
                CNFClause::new(vec![1, 2, 3, 4, -1]), // Needs splitting
            ],
        );

        let reduction = ReduceTo::<KSatisfiability<4, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // All clauses should have exactly 4 literals
        for clause in ksat.clauses() {
            assert_eq!(clause.len(), 4);
        }
    }

    #[test]
    fn test_problem_sizes() {
        let sat = Satisfiability::<i32>::new(3, vec![CNFClause::new(vec![1, 2, 3, 4])]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);

        let source_size = reduction.source_size();
        let target_size = reduction.target_size();

        assert_eq!(source_size.get("num_vars"), Some(3));
        assert_eq!(target_size.get("k"), Some(3));
    }

    #[test]
    fn test_empty_sat_to_3sat() {
        let sat = Satisfiability::<i32>::new(3, vec![]);

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        assert_eq!(ksat.num_clauses(), 0);
        assert_eq!(ksat.num_vars(), 3);
    }

    #[test]
    fn test_mixed_clause_sizes() {
        let sat = Satisfiability::<i32>::new(
            5,
            vec![
                CNFClause::new(vec![1]),                // 1 literal
                CNFClause::new(vec![2, 3]),             // 2 literals
                CNFClause::new(vec![1, 2, 3]),          // 3 literals
                CNFClause::new(vec![1, 2, 3, 4]),       // 4 literals
                CNFClause::new(vec![1, 2, 3, 4, 5]),    // 5 literals
            ],
        );

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        // All clauses should have exactly 3 literals
        for clause in ksat.clauses() {
            assert_eq!(clause.len(), 3);
        }

        // Verify satisfiability is preserved
        let solver = BruteForce::new();
        let sat_solutions = solver.find_best(&sat);
        let ksat_solutions = solver.find_best(ksat);

        let sat_satisfiable = sat_solutions.iter().any(|s| sat.solution_size(s).is_valid);
        let ksat_satisfiable = ksat_solutions.iter().any(|s| ksat.solution_size(s).is_valid);
        assert_eq!(sat_satisfiable, ksat_satisfiable);
    }

    #[test]
    fn test_unsatisfiable_formula() {
        // (x) AND (-x) is unsatisfiable
        let sat = Satisfiability::<i32>::new(
            1,
            vec![CNFClause::new(vec![1]), CNFClause::new(vec![-1])],
        );

        let reduction = ReduceTo::<KSatisfiability<3, i32>>::reduce_to(&sat);
        let ksat = reduction.target_problem();

        let solver = BruteForce::new();

        // Both should be unsatisfiable
        let sat_solutions = solver.find_best(&sat);
        let ksat_solutions = solver.find_best(ksat);

        let sat_satisfiable = sat_solutions.iter().any(|s| sat.solution_size(s).is_valid);
        let ksat_satisfiable = ksat_solutions.iter().any(|s| ksat.solution_size(s).is_valid);

        assert!(!sat_satisfiable);
        assert!(!ksat_satisfiable);
    }
}
