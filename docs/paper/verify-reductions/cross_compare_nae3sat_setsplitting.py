#!/usr/bin/env python3
"""Cross-compare constructor and adversary implementations for NAE 3-SAT → Set Splitting."""
import itertools
import random
import sys

# Import constructor
sys.path.insert(0, "docs/paper/verify-reductions")
from verify_nae3sat_setsplitting import (
    reduce as c_reduce,
    is_nae_feasible as c_is_nae_feasible,
    is_splitting_feasible as c_is_splitting_feasible,
    find_nae_solution as c_find_nae_solution,
    find_splitting_solution as c_find_splitting_solution,
    extract_assignment as c_extract_assignment,
    is_nae_satisfying as c_is_nae_satisfying,
    is_valid_splitting as c_is_valid_splitting,
)

# Import adversary
from adversary_nae3sat_setsplitting import (
    reduce as a_reduce,
    NAE3SAT,
    is_feasible_source as a_is_feasible_source,
    is_feasible_target as a_is_feasible_target,
    solve_nae3sat as a_solve_nae3sat,
    solve_set_splitting as a_solve_set_splitting,
    extract_solution as a_extract_solution,
)


def signed_to_literal(lit):
    """Convert signed int (1-indexed) to adversary's (var_index, is_positive) tuple."""
    if lit > 0:
        return (lit - 1, True)
    else:
        return (-lit - 1, False)


def make_adversary_instance(num_vars, clauses):
    """Convert constructor-style (num_vars, list-of-signed-lists) to adversary NAE3SAT."""
    adv_clauses = [tuple(signed_to_literal(lit) for lit in c) for c in clauses]
    return NAE3SAT(num_vars=num_vars, clauses=adv_clauses)


def normalize_constructor(num_vars, universe_size, subsets):
    """Normalize constructor output to a canonical form: sorted list of sorted tuples."""
    result = []
    for s in subsets:
        result.append(tuple(sorted(s)))
    return tuple(sorted(result))


def normalize_adversary(tgt):
    """Normalize adversary output. Map element names to indices for comparison.

    Adversary uses "v0", "v0_bar", etc. Constructor uses 0, n+0, etc.
    """
    n = tgt.universe_size // 2
    name_to_idx = {}
    for i in range(n):
        name_to_idx[f"v{i}"] = i
        name_to_idx[f"v{i}_bar"] = n + i

    result = []
    for s in tgt.subsets:
        indices = tuple(sorted(name_to_idx[elem] for elem in s))
        result.append(indices)
    return tuple(sorted(result))


def main():
    rng = random.Random(12345)
    agree = disagree = 0
    feasibility_mismatch = 0

    # Test across n=3..5 with random instances
    for n in range(3, 6):
        for _ in range(300):
            m = rng.randint(1, 8)
            # Generate random clauses
            clauses = []
            for _ in range(m):
                vars_chosen = rng.sample(range(1, n + 1), min(3, n))
                clause = [v * rng.choice([1, -1]) for v in vars_chosen]
                clauses.append(clause)

            # Constructor reduction
            c_univ, c_subs = c_reduce(n, clauses)

            # Adversary reduction
            adv_inst = make_adversary_instance(n, clauses)
            a_tgt = a_reduce(adv_inst)

            # Compare structural equivalence
            c_norm = normalize_constructor(n, c_univ, c_subs)
            a_norm = normalize_adversary(a_tgt)

            if c_norm == a_norm:
                agree += 1
            else:
                disagree += 1
                print(f"  DISAGREE on n={n}, clauses={clauses}")
                print(f"    Constructor: {c_norm}")
                print(f"    Adversary:   {a_norm}")

            # Compare feasibility verdicts
            c_feas = c_is_nae_feasible(n, clauses)

            adv_sol = a_solve_nae3sat(adv_inst)
            a_feas = adv_sol is not None

            if c_feas != a_feas:
                feasibility_mismatch += 1
                print(f"  FEASIBILITY MISMATCH on n={n}, clauses={clauses}: "
                      f"constructor={c_feas}, adversary={a_feas}")

    print(f"\nCross-comparison: {agree} agree, {disagree} disagree, "
          f"{feasibility_mismatch} feasibility mismatches")
    if disagree > 0 or feasibility_mismatch > 0:
        print("ACTION REQUIRED: investigate discrepancies before proceeding")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
