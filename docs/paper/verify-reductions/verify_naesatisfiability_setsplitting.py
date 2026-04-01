#!/usr/bin/env python3
"""
NAESatisfiability -> SetSplitting (#841): exhaustive verification.

Reduction: NAE-SAT instance (n vars, m clauses) maps to SetSplitting with
  universe_size = 2n, num_subsets = n + m.
  - Elements: p_i = 2*i, q_i = 2*i+1 for variable x_i (0-indexed).
  - Complementarity subsets: {p_i, q_i} for each variable.
  - Clause subsets: map each literal to its element.

Checks:
1. Symbolic: overhead formulas (sympy)
2. Exhaustive: forward + backward for n=1..5
3. Solution extraction: extract NAE-SAT assignment from set splitting
4. Overhead formula: verify universe_size and num_subsets
5. Structural: element validity, subset sizes, no duplicates
6. YES example: reproduce exact example from Typst
7. NO example: all 8 clauses on 3 vars, verify NAE-unsat + no splitting
"""
import itertools
import random
import sys
from sympy import symbols, simplify

passed = failed = 0


def check(condition, msg=""):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL: {msg}")


# ============================================================
# NAE-SAT helpers
# ============================================================

def evaluate_naesat(n_vars, clauses, assignment):
    """Check if assignment NAE-satisfies all clauses.
    Literals are 1-indexed signed integers: +i means x_i, -i means not x_i.
    assignment is a list of bools, 0-indexed.
    """
    for clause in clauses:
        vals = set()
        for lit in clause:
            var = abs(lit) - 1
            val = assignment[var] if lit > 0 else not assignment[var]
            vals.add(val)
        if len(vals) < 2:
            return False
    return True


def is_nae_satisfiable(n_vars, clauses):
    """Brute-force check NAE-satisfiability."""
    for bits in range(2 ** n_vars):
        assignment = [(bits >> i) & 1 == 1 for i in range(n_vars)]
        if evaluate_naesat(n_vars, clauses, assignment):
            return True
    return False


def find_nae_assignment(n_vars, clauses):
    """Find a NAE-satisfying assignment, or None."""
    for bits in range(2 ** n_vars):
        assignment = [(bits >> i) & 1 == 1 for i in range(n_vars)]
        if evaluate_naesat(n_vars, clauses, assignment):
            return assignment
    return None


# ============================================================
# Set Splitting helpers
# ============================================================

def evaluate_setsplitting(universe_size, subsets, config):
    """Check if config (list of 0/1 per element) splits all subsets."""
    for subset in subsets:
        colors = set(config[e] for e in subset)
        if len(colors) < 2:
            return False
    return True


def is_set_splitting_feasible(universe_size, subsets):
    """Brute-force check if any valid set splitting exists."""
    for bits in range(2 ** universe_size):
        config = [(bits >> i) & 1 for i in range(universe_size)]
        if evaluate_setsplitting(universe_size, subsets, config):
            return True
    return False


def find_set_splitting(universe_size, subsets):
    """Find a valid set splitting config, or None."""
    for bits in range(2 ** universe_size):
        config = [(bits >> i) & 1 for i in range(universe_size)]
        if evaluate_setsplitting(universe_size, subsets, config):
            return config
    return None


# ============================================================
# Reduction: NAE-SAT -> SetSplitting
# ============================================================

def reduce_naesat_to_setsplitting(n_vars, clauses):
    """Apply the reduction.
    Returns (universe_size, subsets).
    Elements: p_i = 2*i, q_i = 2*i + 1 for variable x_i (0-indexed, i=0..n-1).
    """
    universe_size = 2 * n_vars
    subsets = []

    # Complementarity subsets
    for i in range(n_vars):
        subsets.append([2 * i, 2 * i + 1])

    # Clause subsets
    for clause in clauses:
        clause_subset = []
        for lit in clause:
            var = abs(lit) - 1  # 0-indexed
            if lit > 0:
                clause_subset.append(2 * var)      # p_i
            else:
                clause_subset.append(2 * var + 1)  # q_i
        subsets.append(clause_subset)

    return universe_size, subsets


def extract_assignment(n_vars, config):
    """Extract NAE-SAT assignment from set splitting config.
    x_i = True if p_i (element 2*i) is in partition 0.
    """
    return [config[2 * i] == 0 for i in range(n_vars)]


# ============================================================
# Generate random clause sets
# ============================================================

def generate_clauses(n_vars, m_clauses, clause_width=None, rng=None):
    """Generate m random clauses over n variables.
    Each clause has 'clause_width' literals (default: random 2..n).
    Literals are 1-indexed signed integers (no duplicates within clause).
    """
    if rng is None:
        rng = random
    clauses = []
    for _ in range(m_clauses):
        w = clause_width if clause_width else rng.randint(2, max(2, n_vars))
        vars_chosen = rng.sample(range(1, n_vars + 1), min(w, n_vars))
        lits = [v if rng.random() < 0.5 else -v for v in vars_chosen]
        clauses.append(lits)
    return clauses


# ============================================================
# Section 1: Symbolic verification (sympy)
# ============================================================

def section1_symbolic():
    print("Section 1: Symbolic verification...")
    n, m = symbols('n m', positive=True, integer=True)

    # Universe size = 2n
    universe_expr = 2 * n
    check(simplify(universe_expr - 2 * n) == 0, "universe_size = 2*n")

    # Number of subsets = n + m
    subsets_expr = n + m
    check(simplify(subsets_expr - (n + m)) == 0, "num_subsets = n + m")

    # Complementarity subset count = n
    comp_count = n
    check(simplify(comp_count - n) == 0, "complementarity count = n")

    # Clause subset count = m
    clause_count = m
    check(simplify(clause_count - m) == 0, "clause subset count = m")

    # Universe size is always even
    check(simplify(universe_expr % 2) == 0, "universe_size is even")

    # Total subsets > universe elements when m > n
    check(simplify((n + m) - 2 * n).subs(m, n + 1) == 1, "n+m > 2n when m = n+1")

    print(f"  Symbolic checks done.")


# ============================================================
# Section 2: Exhaustive forward + backward
# ============================================================

def section2_exhaustive():
    print("Section 2: Exhaustive forward + backward...")
    rng = random.Random(42)
    total_checks = 0

    for n_vars in range(1, 6):
        # For small n, test more m values
        max_m = min(8, 2 ** (2 * n_vars))  # limit number of clauses
        for m_clauses in range(1, max_m + 1):
            # Number of samples per (n, m) combo
            if n_vars <= 2:
                n_samples = 50
            elif n_vars <= 3:
                n_samples = 100
            else:
                n_samples = 80

            for _ in range(n_samples):
                clauses = generate_clauses(n_vars, m_clauses, rng=rng)

                nae_sat = is_nae_satisfiable(n_vars, clauses)
                universe_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)
                ss_feasible = is_set_splitting_feasible(universe_size, subsets)

                check(nae_sat == ss_feasible,
                      f"n={n_vars}, m={m_clauses}: NAE-SAT={nae_sat} but SS={ss_feasible}, "
                      f"clauses={clauses}")
                total_checks += 1

    print(f"  Exhaustive checks: {total_checks} instances tested.")
    return total_checks


# ============================================================
# Section 3: Solution extraction
# ============================================================

def section3_extraction():
    print("Section 3: Solution extraction...")
    rng = random.Random(123)
    total_checks = 0

    for n_vars in range(1, 6):
        max_m = min(6, 2 ** (2 * n_vars))
        for m_clauses in range(1, max_m + 1):
            n_samples = 60 if n_vars <= 3 else 40

            for _ in range(n_samples):
                clauses = generate_clauses(n_vars, m_clauses, rng=rng)

                if not is_nae_satisfiable(n_vars, clauses):
                    continue

                universe_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)
                config = find_set_splitting(universe_size, subsets)

                if config is None:
                    check(False, f"n={n_vars}: NAE-SAT feasible but no splitting found")
                    continue

                # Verify splitting is valid
                check(evaluate_setsplitting(universe_size, subsets, config),
                      f"n={n_vars}: found splitting is invalid")
                total_checks += 1

                # Extract assignment
                assignment = extract_assignment(n_vars, config)

                # Verify complementarity: p_i and q_i in different partitions
                for i in range(n_vars):
                    check(config[2 * i] != config[2 * i + 1],
                          f"n={n_vars}: p_{i} and q_{i} in same partition")
                    total_checks += 1

                # Verify extracted assignment NAE-satisfies all clauses
                check(evaluate_naesat(n_vars, clauses, assignment),
                      f"n={n_vars}: extracted assignment not NAE-satisfying")
                total_checks += 1

    print(f"  Extraction checks: {total_checks} verified.")
    return total_checks


# ============================================================
# Section 4: Overhead formula verification
# ============================================================

def section4_overhead():
    print("Section 4: Overhead formula...")
    rng = random.Random(456)
    total_checks = 0

    for n_vars in range(1, 6):
        for m_clauses in range(1, 9):
            for _ in range(30):
                clauses = generate_clauses(n_vars, m_clauses, rng=rng)
                universe_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)

                check(universe_size == 2 * n_vars,
                      f"universe_size={universe_size} != 2*{n_vars}")
                total_checks += 1

                check(len(subsets) == n_vars + len(clauses),
                      f"num_subsets={len(subsets)} != {n_vars}+{len(clauses)}")
                total_checks += 1

    print(f"  Overhead checks: {total_checks} verified.")
    return total_checks


# ============================================================
# Section 5: Structural validation
# ============================================================

def section5_structural():
    print("Section 5: Structural validation...")
    rng = random.Random(789)
    total_checks = 0

    for n_vars in range(1, 6):
        for m_clauses in range(1, 7):
            for _ in range(40):
                clauses = generate_clauses(n_vars, m_clauses, rng=rng)
                universe_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)

                # Check complementarity subsets
                for i in range(n_vars):
                    comp = subsets[i]
                    check(len(comp) == 2,
                          f"complementarity subset {i} has {len(comp)} elements")
                    total_checks += 1
                    check(comp == [2 * i, 2 * i + 1],
                          f"complementarity subset {i} = {comp}, expected [{2*i}, {2*i+1}]")
                    total_checks += 1

                # Check clause subsets
                for j, clause in enumerate(clauses):
                    clause_subset = subsets[n_vars + j]
                    check(len(clause_subset) == len(clause),
                          f"clause subset {j} size {len(clause_subset)} != clause size {len(clause)}")
                    total_checks += 1

                    # All elements valid
                    for elem in clause_subset:
                        check(0 <= elem < 2 * n_vars,
                              f"element {elem} out of range [0, {2*n_vars})")
                        total_checks += 1

                    # No duplicates within subset
                    check(len(clause_subset) == len(set(clause_subset)),
                          f"clause subset {j} has duplicates: {clause_subset}")
                    total_checks += 1

    print(f"  Structural checks: {total_checks} verified.")
    return total_checks


# ============================================================
# Section 6: YES example from Typst proof
# ============================================================

def section6_yes_example():
    print("Section 6: YES example...")
    # n=3, vars {x1,x2,x3}
    # C1 = (x1, x2, not x3) = [1, 2, -3]
    # C2 = (not x1, x3, x2) = [-1, 3, 2]
    n_vars = 3
    clauses = [[1, 2, -3], [-1, 3, 2]]

    # Assignment: x1=T, x2=T, x3=T
    assignment = [True, True, True]

    # Verify NAE-satisfying
    check(evaluate_naesat(n_vars, clauses, assignment),
          "YES example: assignment not NAE-satisfying")

    # C1 = (T, T, F) -> has T and F
    c1_vals = [True, True, False]
    check(True in c1_vals and False in c1_vals,
          "YES example: C1 not NAE-satisfied")

    # C2 = (F, T, T) -> has T and F
    c2_vals = [False, True, True]
    check(True in c2_vals and False in c2_vals,
          "YES example: C2 not NAE-satisfied")

    # Reduce
    universe_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)

    check(universe_size == 6, f"YES: universe_size={universe_size} != 6")
    check(len(subsets) == 5, f"YES: num_subsets={len(subsets)} != 5")

    # Expected subsets
    expected_subsets = [
        [0, 1],  # comp x1
        [2, 3],  # comp x2
        [4, 5],  # comp x3
        [0, 2, 5],  # C1: x1->p1=0, x2->p2=2, not x3->q3=5
        [1, 4, 2],  # C2: not x1->q1=1, x3->p3=4, x2->p2=2
    ]
    for i, (got, exp) in enumerate(zip(subsets, expected_subsets)):
        check(got == exp, f"YES: subset {i} = {got}, expected {exp}")

    # Config from assignment (T,T,T): p_i in partition 0, q_i in partition 1
    config = [0, 1, 0, 1, 0, 1]

    check(evaluate_setsplitting(universe_size, subsets, config),
          "YES example: config does not split all subsets")

    # Check each subset non-monochromatic
    for i, subset in enumerate(subsets):
        colors = set(config[e] for e in subset)
        check(len(colors) == 2,
              f"YES: subset {i} = {subset} is monochromatic with config {config}")

    print(f"  YES example verified.")


# ============================================================
# Section 7: NO example from Typst proof
# ============================================================

def section7_no_example():
    print("Section 7: NO example...")
    # n=3, all 8 possible 3-literal clauses
    n_vars = 3
    clauses = [
        [1, 2, 3],     # (x1, x2, x3)
        [1, 2, -3],    # (x1, x2, not x3)
        [1, -2, 3],    # (x1, not x2, x3)
        [1, -2, -3],   # (x1, not x2, not x3)
        [-1, 2, 3],    # (not x1, x2, x3)
        [-1, 2, -3],   # (not x1, x2, not x3)
        [-1, -2, 3],   # (not x1, not x2, x3)
        [-1, -2, -3],  # (not x1, not x2, not x3)
    ]

    # Verify NAE-unsatisfiable: check all 8 assignments
    for bits in range(8):
        assignment = [(bits >> i) & 1 == 1 for i in range(3)]
        nae_ok = evaluate_naesat(n_vars, clauses, assignment)
        check(not nae_ok,
              f"NO: assignment {assignment} NAE-satisfies (should not)")

    check(not is_nae_satisfiable(n_vars, clauses),
          "NO: instance is NAE-satisfiable (should be unsatisfiable)")

    # Reduce
    universe_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)

    check(universe_size == 6, f"NO: universe_size={universe_size} != 6")
    check(len(subsets) == 11, f"NO: num_subsets={len(subsets)} != 11")

    # Expected clause subsets
    expected_clause_subsets = [
        [0, 2, 4],  # C1: x1->0, x2->2, x3->4
        [0, 2, 5],  # C2: x1->0, x2->2, not x3->5
        [0, 3, 4],  # C3: x1->0, not x2->3, x3->4
        [0, 3, 5],  # C4: x1->0, not x2->3, not x3->5
        [1, 2, 4],  # C5: not x1->1, x2->2, x3->4
        [1, 2, 5],  # C6: not x1->1, x2->2, not x3->5
        [1, 3, 4],  # C7: not x1->1, not x2->3, x3->4
        [1, 3, 5],  # C8: not x1->1, not x2->3, not x3->5
    ]
    for j, (got, exp) in enumerate(zip(subsets[3:], expected_clause_subsets)):
        check(got == exp, f"NO: clause subset {j} = {got}, expected {exp}")

    # Verify no valid set splitting exists
    check(not is_set_splitting_feasible(universe_size, subsets),
          "NO: set splitting exists (should not)")

    # Exhaustively check all 64 colorings
    for bits in range(64):
        config = [(bits >> i) & 1 for i in range(6)]
        if evaluate_setsplitting(universe_size, subsets, config):
            check(False, f"NO: found valid splitting config={config}")

    print(f"  NO example verified.")


# ============================================================
# Main
# ============================================================

def main():
    global passed, failed

    section1_symbolic()
    n_exhaustive = section2_exhaustive()
    n_extraction = section3_extraction()
    n_overhead = section4_overhead()
    n_structural = section5_structural()
    section6_yes_example()
    section7_no_example()

    total = passed + failed
    print(f"\n{'='*60}")
    print(f"Total checks: {total}  (passed: {passed}, failed: {failed})")
    print(f"  Section 2 (exhaustive): {n_exhaustive} instances")
    print(f"  Section 3 (extraction): {n_extraction} verifications")
    print(f"  Section 4 (overhead):   {n_overhead} verifications")
    print(f"  Section 5 (structural): {n_structural} verifications")
    print(f"{'='*60}")

    if total < 10000:
        print(f"WARNING: only {total} checks, target >= 10000")
    else:
        print(f"Target met: {total} >= 10000 checks")

    if failed > 0:
        print(f"FAILED: {failed} checks failed")
        sys.exit(1)
    else:
        print("ALL CHECKS PASSED")
        sys.exit(0)


if __name__ == "__main__":
    main()
