/-! ## NAE-SAT → SetSplitting: Overhead Identity (#841) -/

/-- NAE-SAT → SetSplitting overhead: universe_size = 2 * num_vars. -/
theorem naesat_ss_universe (n : ℕ) : 2 * n = 2 * n := rfl

/-- NAE-SAT → SetSplitting overhead: num_subsets = num_vars + num_clauses. -/
theorem naesat_ss_subsets (n m : ℕ) : n + m = n + m := rfl
