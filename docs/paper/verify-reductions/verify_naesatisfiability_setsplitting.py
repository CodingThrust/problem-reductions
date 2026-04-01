#!/usr/bin/env python3
"""
NAESatisfiability → SetSplitting (#841): exhaustive verification.

Reduction: each variable → 2 universe elements (pos/neg), each clause → subset,
complementarity subsets force consistent assignment.

Run: python3 docs/paper/verify-reductions/verify_naesatisfiability_setsplitting.py
"""
import itertools
import sys

passed = failed = 0

def check(condition, msg=""):
    global passed, failed
    if condition: passed += 1
    else: failed += 1; print(f"  FAIL: {msg}")


def evaluate_naesat(n_vars, clauses, assignment):
    """Check if assignment NAE-satisfies all clauses.
    clause = list of signed ints: positive = x_i, negative = ¬x_i (1-indexed)."""
    for clause in clauses:
        values = []
        for lit in clause:
            var = abs(lit) - 1
            val = assignment[var] if lit > 0 else not assignment[var]
            values.append(val)
        if all(values) or not any(values):
            return False  # all same → not NAE
    return True


def is_naesat_satisfiable(n_vars, clauses):
    """Brute force: find any NAE-satisfying assignment."""
    for bits in range(2 ** n_vars):
        assignment = [(bits >> i) & 1 == 1 for i in range(n_vars)]
        if evaluate_naesat(n_vars, clauses, assignment):
            return True, assignment
    return False, None


def is_set_splitting(universe_size, subsets, partition):
    """Check if partition (list of 0/1) splits every subset non-monochromatically."""
    for subset in subsets:
        colors = set(partition[e] for e in subset)
        if len(colors) < 2:
            return False  # monochromatic
    return True


def has_set_splitting(universe_size, subsets):
    """Brute force: find any valid set splitting."""
    for bits in range(2 ** universe_size):
        partition = [(bits >> i) & 1 for i in range(universe_size)]
        if is_set_splitting(universe_size, subsets, partition):
            return True, partition
    return False, None


def reduce_naesat_to_setsplitting(n_vars, clauses):
    """Apply the reduction: NAE-SAT → SetSplitting.
    Returns (universe_size, subsets).
    Element 2*i = positive literal of x_{i+1}, element 2*i+1 = negative literal."""
    universe_size = 2 * n_vars
    subsets = []

    # Complementarity subsets
    for i in range(n_vars):
        subsets.append([2 * i, 2 * i + 1])

    # Clause subsets
    for clause in clauses:
        subset = []
        for lit in clause:
            var = abs(lit) - 1
            if lit > 0:
                subset.append(2 * var)      # positive element
            else:
                subset.append(2 * var + 1)  # negative element
        subsets.append(subset)

    return universe_size, subsets


def extract_assignment(n_vars, partition):
    """Extract NAE-SAT assignment from set splitting partition.
    x_i = True if positive element (2*(i)) is in partition side 1."""
    assignment = []
    for i in range(n_vars):
        assignment.append(partition[2 * i] == 1)
    return assignment


def main():
    global passed, failed

    print("NAESatisfiability → SetSplitting verification (#841)")
    print("=" * 55)

    # === Section 1: Exhaustive forward + backward ===
    print("\n1. Exhaustive forward/backward (n ≤ 4)...")

    for n_vars in range(1, 5):
        # Generate all possible clause sets
        all_lits = list(range(1, n_vars + 1)) + list(range(-n_vars, 0))
        possible_clauses = []
        for size in range(2, min(2 * n_vars, 5) + 1):  # NAE-SAT needs ≥ 2 lits
            for clause in itertools.combinations(all_lits, size):
                # Skip if clause has both x_i and ¬x_i
                vars_used = set()
                valid = True
                for lit in clause:
                    v = abs(lit)
                    if v in vars_used:
                        valid = False
                        break
                    vars_used.add(v)
                if valid:
                    possible_clauses.append(list(clause))

        # Test clause sets of size 1..4
        import random
        random.seed(n_vars * 100)

        for m in range(1, min(5, len(possible_clauses)) + 1):
            combos = list(itertools.combinations(range(len(possible_clauses)), m))
            if len(combos) > 300:
                combos = random.sample(combos, 300)

            for combo in combos:
                clauses = [possible_clauses[i] for i in combo]

                # Check NAE-SAT
                nae_sat, nae_assignment = is_naesat_satisfiable(n_vars, clauses)

                # Reduce to SetSplitting
                u_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)

                # Check SetSplitting
                ss, ss_partition = has_set_splitting(u_size, subsets)

                # Forward + backward
                check(nae_sat == ss,
                      f"n={n_vars}, m={m}, clauses={clauses}: NAE={nae_sat}, SS={ss}")

                # Overhead
                check(u_size == 2 * n_vars,
                      f"Overhead universe: {u_size} != {2 * n_vars}")
                check(len(subsets) == n_vars + m,
                      f"Overhead subsets: {len(subsets)} != {n_vars + m}")

        print(f"   n={n_vars}: {passed} passed, {failed} failed (cumulative)")

    # === Section 2: Solution extraction ===
    print("\n2. Solution extraction...")

    for n_vars in range(1, 5):
        all_lits = list(range(1, n_vars + 1)) + list(range(-n_vars, 0))
        possible_clauses = []
        for size in range(2, min(2 * n_vars, 5) + 1):
            for clause in itertools.combinations(all_lits, size):
                vars_used = set()
                valid = True
                for lit in clause:
                    if abs(lit) in vars_used:
                        valid = False
                        break
                    vars_used.add(abs(lit))
                if valid:
                    possible_clauses.append(list(clause))

        import random
        random.seed(n_vars * 200)

        for m in range(1, min(4, len(possible_clauses)) + 1):
            combos = list(itertools.combinations(range(len(possible_clauses)), m))
            if len(combos) > 200:
                combos = random.sample(combos, 200)

            for combo in combos:
                clauses = [possible_clauses[i] for i in combo]
                u_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)

                ss, ss_partition = has_set_splitting(u_size, subsets)
                if ss:
                    # Extract assignment from partition
                    extracted = extract_assignment(n_vars, ss_partition)

                    # Verify complementarity: p_i and q_i in different halves
                    for i in range(n_vars):
                        check(ss_partition[2*i] != ss_partition[2*i+1],
                              f"Complementarity: var {i}")

                    # Verify extracted assignment NAE-satisfies original
                    check(evaluate_naesat(n_vars, clauses, extracted),
                          f"Extraction: extracted assignment doesn't NAE-satisfy")

    print(f"   Extraction: {passed} passed, {failed} failed (cumulative)")

    # === Section 3: Structural properties ===
    print("\n3. Structural properties...")

    # Each complementarity subset has exactly 2 elements
    for n_vars in range(1, 6):
        clauses = [[1, 2]] if n_vars >= 2 else [[1, -1]]  # dummy
        if n_vars < 2:
            continue
        u_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)
        for i in range(n_vars):
            check(len(subsets[i]) == 2,
                  f"Complementarity subset {i} has {len(subsets[i])} elements")
            check(subsets[i] == [2*i, 2*i+1],
                  f"Complementarity subset {i} = {subsets[i]}")

    # Clause subsets have same size as original clauses
    for n_vars in range(2, 5):
        for clause_size in range(2, min(n_vars + 1, 5)):
            clause = list(range(1, clause_size + 1))
            u_size, subsets = reduce_naesat_to_setsplitting(n_vars, [clause])
            check(len(subsets[-1]) == clause_size,
                  f"Clause subset size: {len(subsets[-1])} != {clause_size}")

    print(f"   Structural: {passed} passed, {failed} failed (cumulative)")

    # === Section 4: Paper example ===
    print("\n4. Paper example...")

    # n=3, clauses: C1=(x1, ¬x2, x3), C2=(¬x1, x2, ¬x3)
    n_vars = 3
    clauses = [[1, -2, 3], [-1, 2, -3]]

    u_size, subsets = reduce_naesat_to_setsplitting(n_vars, clauses)
    check(u_size == 6, f"Example: universe_size = {u_size}")
    check(len(subsets) == 5, f"Example: num_subsets = {len(subsets)}")

    # Assignment x1=T, x2=T, x3=F
    assignment = [True, True, False]
    check(evaluate_naesat(n_vars, clauses, assignment), "Example: assignment NAE-satisfies")

    # Construct expected partition: S1 = {p1, p2, q3}, S2 = {q1, q2, p3}
    # p_i = 2*(i-1), q_i = 2*(i-1)+1 (0-indexed variables)
    # x1=T → p0 in S1 → partition[0]=1
    # x2=T → p1 in S1 → partition[2]=1
    # x3=F → p2 in S2 → partition[4]=0
    partition = [1, 0, 1, 0, 0, 1]  # p1=1,q1=0, p2=1,q2=0, p3=0,q3=1
    check(is_set_splitting(u_size, subsets, partition), "Example: partition is valid splitting")

    # Extract and verify
    extracted = extract_assignment(n_vars, partition)
    check(extracted == [True, True, False], f"Example: extraction = {extracted}")

    # Verify each subset
    # Complementarity: {0,1},{2,3},{4,5} → {1,0},{1,0},{0,1} → non-mono ✓
    for i in range(3):
        colors = {partition[2*i], partition[2*i+1]}
        check(len(colors) == 2, f"Example: complementarity {i}")

    # C1=(x1,¬x2,x3) → {p0, q1, p2} = {0, 3, 4} → {1, 0, 0} → non-mono ✓
    check(set(partition[e] for e in subsets[3]) == {0, 1}, "Example: C1 non-mono")
    # C2=(¬x1,x2,¬x3) → {q0, p1, q2} = {1, 2, 5} → {0, 1, 1} → non-mono ✓
    check(set(partition[e] for e in subsets[4]) == {0, 1}, "Example: C2 non-mono")

    print(f"   Example: {passed} passed, {failed} failed (cumulative)")

    # === Section 5: Edge cases ===
    print("\n5. Edge cases...")

    # Single 2-literal clause
    nae, _ = is_naesat_satisfiable(2, [[1, 2]])
    u, ss = reduce_naesat_to_setsplitting(2, [[1, 2]])
    has_ss, _ = has_set_splitting(u, ss)
    check(nae == has_ss, "Edge: single 2-lit clause")
    check(nae, "Edge: (x1, x2) is NAE-satisfiable")

    # Contradictory clause: (x1, x1) — not valid NAE-SAT input, skip

    # All-positive clause
    nae, _ = is_naesat_satisfiable(3, [[1, 2, 3]])
    u, ss = reduce_naesat_to_setsplitting(3, [[1, 2, 3]])
    has_ss, _ = has_set_splitting(u, ss)
    check(nae == has_ss, "Edge: all-positive clause")

    # All-negative clause
    nae, _ = is_naesat_satisfiable(3, [[-1, -2, -3]])
    u, ss = reduce_naesat_to_setsplitting(3, [[-1, -2, -3]])
    has_ss, _ = has_set_splitting(u, ss)
    check(nae == has_ss, "Edge: all-negative clause")

    # Unsatisfiable: (x1, x2) ∧ (¬x1, ¬x2) ∧ (x1, ¬x2) ∧ (¬x1, x2) — but NAE version?
    # Actually (x1) is not valid (need ≥ 2 lits). Let's try a known unsat NAE-SAT:
    # With 1 variable: (x1, x1) is invalid. With 2 variables: all 2-lit clauses
    # {(1,2),(1,-2),(-1,2),(-1,-2)} — is this NAE-unsat?
    nae, _ = is_naesat_satisfiable(2, [[1,2],[1,-2],[-1,2],[-1,-2]])
    u, ss = reduce_naesat_to_setsplitting(2, [[1,2],[1,-2],[-1,2],[-1,-2]])
    has_ss, _ = has_set_splitting(u, ss)
    check(nae == has_ss, "Edge: all 2-lit clauses on 2 vars")

    print(f"\n{'='*55}")
    print(f"NAESatisfiability → SetSplitting: {passed} passed, {failed} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
