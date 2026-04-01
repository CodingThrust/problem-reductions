#!/usr/bin/env python3
"""Cross-compare constructor and adversary implementations for NAESatisfiability → SetSplitting."""
import itertools
import sys
sys.path.insert(0, "docs/paper/verify-reductions")

from verify_naesat_setsplitting import (
    reduce as constructor_reduce,
    is_feasible_source as c_is_feasible_source,
    is_feasible_target as c_is_feasible_target,
    is_nae_satisfying as c_is_nae_satisfying,
    extract_assignment as c_extract_assignment,
    is_set_splitting as c_is_set_splitting,
)
from adversary_naesat_setsplitting import (
    reduce as adversary_reduce_raw,
    is_nae_satisfying as a_is_nae_satisfying_raw,
    is_valid_splitting as a_is_valid_splitting_raw,
    partition_to_assignment as a_partition_to_assignment,
)

agree = disagree = 0
feasibility_mismatch = 0


def signed_to_tuple(clause):
    """Convert signed literal list to (var_idx, is_positive) tuple list."""
    return [(abs(lit), lit > 0) for lit in clause]


def adversary_reduce(n, clauses):
    """Adapter: convert signed-literal clauses to adversary format and reduce."""
    a_clauses = [signed_to_tuple(c) for c in clauses]
    u_size, subsets = adversary_reduce_raw(n, a_clauses)
    # Convert frozensets to sorted lists for comparison
    return u_size, [sorted(s) for s in subsets]


def generate_all_valid_clauses(n):
    """Generate all valid 2- and 3-literal NAE-SAT clauses for n variables."""
    lits = list(range(1, n + 1)) + list(range(-n, 0))
    clauses = []
    for size in [2, 3]:
        for combo in itertools.combinations(lits, size):
            vars_used = [abs(l) for l in combo]
            if len(set(vars_used)) == len(vars_used):
                clauses.append(list(combo))
    return clauses


def normalize_subsets(universe_size, subsets):
    """Normalize subsets to a canonical form for comparison."""
    return (universe_size, tuple(tuple(sorted(s)) for s in sorted(subsets, key=lambda x: tuple(sorted(x)))))


for n in range(2, 6):
    all_clauses = generate_all_valid_clauses(n)
    instances_tested = 0
    max_instances = 200

    for num_cl in [1, 2, 3, 4]:
        for combo in itertools.combinations(range(len(all_clauses)), num_cl):
            if instances_tested >= max_instances:
                break
            clauses = [all_clauses[i] for i in combo]

            # Run both reductions
            c_usize, c_subsets = constructor_reduce(n, clauses)
            a_usize, a_subsets = adversary_reduce(n, clauses)

            # Compare structural equivalence
            c_norm = normalize_subsets(c_usize, c_subsets)
            a_norm = normalize_subsets(a_usize, a_subsets)

            if c_norm == a_norm:
                agree += 1
            else:
                disagree += 1
                print(f"  DISAGREE on n={n}, clauses={clauses}")
                print(f"    Constructor: u={c_usize}, subsets={c_subsets}")
                print(f"    Adversary:   u={a_usize}, subsets={a_subsets}")

            # Compare source feasibility
            c_feas = c_is_feasible_source(n, clauses)
            a_clauses_tuples = [signed_to_tuple(c) for c in clauses]
            a_feas = any(
                a_is_nae_satisfying_raw(n, a_clauses_tuples, [(bits >> i) & 1 for i in range(n)])
                for bits in range(2**n)
            )

            if c_feas != a_feas:
                feasibility_mismatch += 1
                print(f"  SOURCE FEASIBILITY MISMATCH on n={n}, clauses={clauses}: "
                      f"constructor={c_feas}, adversary={a_feas}")

            # Compare target feasibility via constructor's format
            c_t_feas = c_is_feasible_target(c_usize, c_subsets)
            # For adversary, check target via constructor's validator on constructor's output
            # (since both should produce the same target, this checks structural agreement)
            if c_norm == a_norm and c_feas != c_t_feas:
                # This would mean the reduction is wrong
                feasibility_mismatch += 1
                print(f"  REDUCTION CONSISTENCY MISMATCH: source={c_feas}, target={c_t_feas}")

            instances_tested += 1

    print(f"n={n}: tested {instances_tested} instances")

print(f"\nCross-comparison: {agree} agree, {disagree} disagree, "
      f"{feasibility_mismatch} feasibility mismatches")
if disagree > 0 or feasibility_mismatch > 0:
    print("ACTION REQUIRED: investigate discrepancies before proceeding")
    sys.exit(1)
else:
    print("All instances agree between constructor and adversary.")
    sys.exit(0)
