#!/usr/bin/env python3
"""
Adversary verification script for NAESatisfiability -> SetSplitting reduction.

Independent implementation based solely on the Typst proof specification.
Does NOT import or reference verify_naesat_setsplitting.py.
"""

import itertools
import random
import sys

# ---------------------------------------------------------------------------
# 1. Data structures
# ---------------------------------------------------------------------------

# NAE-SAT instance: n variables (1..n), clauses are lists of literals.
# A literal is (var_index, is_positive) where var_index in 1..n.
# E.g., v_1 -> (1, True), ~v_2 -> (2, False)

# Set Splitting instance: universe_size (int), subsets (list of frozensets of ints)


def reduce(n_vars, clauses):
    """
    Reduce NAE-SAT to Set Splitting per the Typst proof.

    Universe S = {0, 1, ..., 2n-1} where element 2(i-1) = v_i, 2(i-1)+1 = ~v_i.
    Subsets:
      - Complementarity: P_i = {2(i-1), 2(i-1)+1} for i=1..n
      - Clause: Q_j = {elem for each literal in clause_j}
    """
    universe_size = 2 * n_vars
    subsets = []

    # Complementarity subsets
    for i in range(1, n_vars + 1):
        pos_elem = 2 * (i - 1)
        neg_elem = 2 * (i - 1) + 1
        subsets.append(frozenset([pos_elem, neg_elem]))

    # Clause subsets
    for clause in clauses:
        elems = set()
        for var_idx, is_positive in clause:
            if is_positive:
                elems.add(2 * (var_idx - 1))
            else:
                elems.add(2 * (var_idx - 1) + 1)
        subsets.append(frozenset(elems))

    return universe_size, subsets


def literal_to_elem(var_idx, is_positive):
    """Convert a literal to its universe element index."""
    if is_positive:
        return 2 * (var_idx - 1)
    else:
        return 2 * (var_idx - 1) + 1


def assignment_to_partition(n_vars, assignment):
    """
    Convert a boolean assignment (list of n bools, index 0 = var 1) to a partition.
    Returns S1 as a frozenset. S2 = universe - S1.

    S1 = {v_i : alpha(v_i)=True} union {~v_i : alpha(v_i)=False}
    """
    s1 = set()
    for i in range(n_vars):
        if assignment[i]:  # var (i+1) is True
            s1.add(2 * i)      # v_{i+1} in S1
        else:               # var (i+1) is False
            s1.add(2 * i + 1)  # ~v_{i+1} in S1
    return frozenset(s1)


def partition_to_assignment(n_vars, s1):
    """
    Extract NAE-SAT assignment from a set splitting partition.
    alpha(v_i) = True iff element 2(i-1) in S1.
    """
    assignment = []
    for i in range(n_vars):
        assignment.append(2 * i in s1)
    return assignment


def is_nae_satisfying(n_vars, clauses, assignment):
    """Check if assignment NAE-satisfies all clauses."""
    for clause in clauses:
        values = set()
        for var_idx, is_positive in clause:
            val = assignment[var_idx - 1]
            if not is_positive:
                val = not val
            values.add(val)
        if len(values) < 2:  # all equal
            return False
    return True


def is_valid_splitting(universe_size, subsets, s1):
    """Check if partition (S1, S2) splits every subset."""
    s2 = set(range(universe_size)) - set(s1)
    for subset in subsets:
        if not (subset & s1) or not (subset & s2):
            return False
    return True


def is_consistent_partition(n_vars, s1):
    """
    Check that the partition is consistent with complementarity constraints:
    for each variable i, exactly one of {2(i-1), 2(i-1)+1} is in S1.
    """
    for i in range(n_vars):
        pos = 2 * i
        neg = 2 * i + 1
        in_s1_pos = pos in s1
        in_s1_neg = neg in s1
        if in_s1_pos == in_s1_neg:  # both in or both out
            return False
    return True


# ---------------------------------------------------------------------------
# 2. Test functions
# ---------------------------------------------------------------------------

passed = 0
failed = 0
bugs = []


def check(condition, msg):
    global passed, failed, bugs
    if condition:
        passed += 1
    else:
        failed += 1
        if msg not in bugs:
            bugs.append(msg)


def test_yes_example():
    """Reproduce the YES example from the Typst proof."""
    global passed, failed
    n = 4
    # c1=(v1,v2,v3), c2=(~v1,v3,v4), c3=(v2,~v3,~v4), c4=(v1,~v2,v4)
    clauses = [
        [(1, True), (2, True), (3, True)],
        [(1, False), (3, True), (4, True)],
        [(2, True), (3, False), (4, False)],
        [(1, True), (2, False), (4, True)],
    ]
    universe_size, subsets = reduce(n, clauses)

    # Check overhead
    check(universe_size == 8, "YES: universe_size should be 8")
    check(len(subsets) == 8, "YES: num_subsets should be 8")

    # Check specific subsets (0-indexed from proof)
    expected_subsets = [
        frozenset([0, 1]), frozenset([2, 3]), frozenset([4, 5]), frozenset([6, 7]),
        frozenset([0, 2, 4]), frozenset([1, 4, 6]), frozenset([2, 5, 7]), frozenset([0, 3, 6]),
    ]
    for i, (got, exp) in enumerate(zip(subsets, expected_subsets)):
        check(got == exp, f"YES: subset {i} mismatch: got {got}, expected {exp}")

    # Assignment: v1=T, v2=F, v3=T, v4=F
    assignment = [True, False, True, False]
    check(is_nae_satisfying(n, clauses, assignment), "YES: assignment should NAE-satisfy")

    s1 = assignment_to_partition(n, assignment)
    expected_s1 = frozenset([0, 3, 4, 7])
    check(s1 == expected_s1, f"YES: S1 should be {{0,3,4,7}}, got {s1}")

    check(is_valid_splitting(universe_size, subsets, s1), "YES: partition should be valid splitting")

    # Extract back
    recovered = partition_to_assignment(n, s1)
    check(recovered == assignment, "YES: extracted assignment should match original")


def test_no_example():
    """Reproduce the NO example from the Typst proof."""
    n = 3
    clauses = [
        [(1, True), (2, True), (3, True)],
        [(1, True), (2, True), (3, False)],
        [(1, True), (2, False), (3, True)],
        [(1, True), (2, False), (3, False)],
    ]
    universe_size, subsets = reduce(n, clauses)

    check(universe_size == 6, "NO: universe_size should be 6")
    check(len(subsets) == 7, "NO: num_subsets should be 7")

    expected_subsets = [
        frozenset([0, 1]), frozenset([2, 3]), frozenset([4, 5]),
        frozenset([0, 2, 4]), frozenset([0, 2, 5]),
        frozenset([0, 3, 4]), frozenset([0, 3, 5]),
    ]
    for i, (got, exp) in enumerate(zip(subsets, expected_subsets)):
        check(got == exp, f"NO: subset {i} mismatch: got {got}, expected {exp}")

    # Verify no assignment works
    for bits in itertools.product([False, True], repeat=n):
        assignment = list(bits)
        check(not is_nae_satisfying(n, clauses, assignment),
              f"NO: assignment {assignment} should NOT NAE-satisfy")

    # Verify no consistent partition works
    for bits in itertools.product([0, 1], repeat=2 * n):
        s1 = frozenset(i for i, b in enumerate(bits) if b == 0)
        if is_consistent_partition(n, s1):
            check(not is_valid_splitting(universe_size, subsets, s1),
                  f"NO: consistent partition {s1} should not be valid splitting")


def test_exhaustive_small():
    """Exhaustive testing for all NAE-SAT instances with n <= 5 variables."""
    # For small n, generate many clause patterns and verify equivalence
    for n in range(1, 6):
        vars_list = list(range(1, n + 1))
        # Generate all possible literals
        all_literals = []
        for v in vars_list:
            all_literals.append((v, True))
            all_literals.append((v, False))

        # For n<=3, test all possible single-clause instances with 2-3 literals
        if n <= 3:
            clause_sizes = [2, 3] if n >= 2 else [2]
            for size in clause_sizes:
                for combo in itertools.combinations(all_literals, size):
                    # Skip clauses with both v_i and ~v_i (tautological for NAE purposes)
                    clause = list(combo)
                    clauses = [clause]
                    verify_reduction_equivalence(n, clauses)

        # For all n, test random instances
        rng = random.Random(42 + n)
        num_random = 200 if n <= 3 else 100
        for _ in range(num_random):
            m = rng.randint(1, min(n * 2, 8))
            clauses = []
            for _ in range(m):
                clause_size = rng.randint(2, min(len(all_literals), 4))
                clause = rng.sample(all_literals, clause_size)
                clauses.append(clause)
            verify_reduction_equivalence(n, clauses)


def verify_reduction_equivalence(n_vars, clauses):
    """
    Core verification: NAE-SAT instance is feasible iff the reduced
    Set Splitting instance is feasible. Also checks solution extraction.
    """
    universe_size, subsets = reduce(n_vars, clauses)

    # Check overhead formula
    check(universe_size == 2 * n_vars,
          f"Overhead: universe_size should be 2*{n_vars}={2*n_vars}, got {universe_size}")
    check(len(subsets) == n_vars + len(clauses),
          f"Overhead: num_subsets should be {n_vars}+{len(clauses)}, got {len(subsets)}")

    # Enumerate all assignments for NAE-SAT
    nae_feasible = False
    nae_witnesses = []
    for bits in itertools.product([False, True], repeat=n_vars):
        assignment = list(bits)
        if is_nae_satisfying(n_vars, clauses, assignment):
            nae_feasible = True
            nae_witnesses.append(assignment)

    # Enumerate all valid splittings (consistent partitions only)
    ss_feasible = False
    ss_witnesses = []
    for bits in itertools.product([0, 1], repeat=n_vars):
        # Build consistent partition: for each var, bit=0 means v_i in S1
        s1 = set()
        for i in range(n_vars):
            if bits[i] == 0:
                s1.add(2 * i)       # v_{i+1} in S1
            else:
                s1.add(2 * i + 1)   # ~v_{i+1} in S1
        # Complete s1: the complement elements go to S2
        for i in range(n_vars):
            if 2 * i not in s1:
                s1.add(2 * i + 1)
            if 2 * i + 1 not in s1:
                pass  # already not in s1
        # Actually, rebuild properly
        s1 = set()
        for i in range(n_vars):
            if bits[i] == 0:
                s1.add(2 * i)
            else:
                s1.add(2 * i + 1)
        s1 = frozenset(s1)

        if is_valid_splitting(universe_size, subsets, s1):
            ss_feasible = True
            ss_witnesses.append(s1)

    # Equivalence
    check(nae_feasible == ss_feasible,
          f"Equivalence failed for n={n_vars}, clauses={clauses}: NAE={nae_feasible}, SS={ss_feasible}")

    # Forward direction: every NAE witness maps to a valid splitting
    for assignment in nae_witnesses:
        s1 = assignment_to_partition(n_vars, assignment)
        check(is_valid_splitting(universe_size, subsets, s1),
              f"Forward: NAE assignment {assignment} -> partition not valid")

    # Backward direction: every valid splitting maps to NAE assignment
    for s1 in ss_witnesses:
        assignment = partition_to_assignment(n_vars, s1)
        check(is_nae_satisfying(n_vars, clauses, assignment),
              f"Backward: partition {s1} -> assignment {assignment} not NAE-satisfying")

    # Solution extraction roundtrip
    for assignment in nae_witnesses:
        s1 = assignment_to_partition(n_vars, assignment)
        recovered = partition_to_assignment(n_vars, s1)
        check(recovered == assignment,
              f"Roundtrip: assignment {assignment} != recovered {recovered}")


def test_overhead_formula():
    """Verify overhead formula on many random instances."""
    rng = random.Random(12345)
    for _ in range(500):
        n = rng.randint(1, 10)
        m = rng.randint(1, 15)
        all_literals = [(v, p) for v in range(1, n + 1) for p in [True, False]]
        clauses = []
        for _ in range(m):
            size = rng.randint(2, min(len(all_literals), 5))
            clause = rng.sample(all_literals, size)
            clauses.append(clause)

        universe_size, subsets = reduce(n, clauses)
        check(universe_size == 2 * n,
              f"Overhead: universe_size={universe_size} != 2*{n}")
        check(len(subsets) == n + m,
              f"Overhead: num_subsets={len(subsets)} != {n}+{m}")


def test_complementarity_always_split():
    """Every consistent partition always splits complementarity subsets."""
    rng = random.Random(99999)
    for _ in range(500):
        n = rng.randint(1, 8)
        # Any assignment -> partition must split all complementarity subsets
        assignment = [rng.choice([True, False]) for _ in range(n)]
        s1 = assignment_to_partition(n, assignment)
        for i in range(n):
            p_i = frozenset([2 * i, 2 * i + 1])
            check(bool(p_i & s1) and bool(p_i - s1),
                  f"Complementarity: P_{i+1} not split by assignment {assignment}")


def test_nae_symmetry():
    """
    NAE-SAT has the property that if alpha is NAE-satisfying, so is ~alpha
    (flipping all variables). Verify this is preserved through reduction.
    """
    rng = random.Random(77777)
    for _ in range(500):
        n = rng.randint(2, 6)
        m = rng.randint(1, 8)
        all_literals = [(v, p) for v in range(1, n + 1) for p in [True, False]]
        clauses = []
        for _ in range(m):
            size = rng.randint(2, min(len(all_literals), 4))
            clause = rng.sample(all_literals, size)
            clauses.append(clause)

        universe_size, subsets = reduce(n, clauses)

        assignment = [rng.choice([True, False]) for _ in range(n)]
        flipped = [not a for a in assignment]

        nae_orig = is_nae_satisfying(n, clauses, assignment)
        nae_flip = is_nae_satisfying(n, clauses, flipped)
        check(nae_orig == nae_flip,
              f"NAE symmetry: original={nae_orig}, flipped={nae_flip}")

        # Also check that the corresponding partitions are complements
        s1_orig = assignment_to_partition(n, assignment)
        s1_flip = assignment_to_partition(n, flipped)
        universe = frozenset(range(universe_size))
        check(s1_orig | s1_flip == universe,
              f"Partition symmetry: union != universe")
        check(not (s1_orig & s1_flip),
              f"Partition symmetry: non-empty intersection")

        # Both partitions should give same splitting result
        ss_orig = is_valid_splitting(universe_size, subsets, s1_orig)
        ss_flip = is_valid_splitting(universe_size, subsets, s1_flip)
        check(ss_orig == ss_flip,
              f"Splitting symmetry: orig={ss_orig}, flip={ss_flip}")


def test_single_clause_edge_cases():
    """Test edge cases: single clause with all positive, all negative, mixed."""
    for n in range(2, 6):
        # All positive: (v1, v2, ..., vn) — NAE-sat iff not all same
        clause_all_pos = [(i, True) for i in range(1, n + 1)]
        verify_reduction_equivalence(n, [clause_all_pos])

        # All negative: (~v1, ~v2, ..., ~vn)
        clause_all_neg = [(i, False) for i in range(1, n + 1)]
        verify_reduction_equivalence(n, [clause_all_neg])

        # Mixed: (v1, ~v2, v3, ~v4, ...)
        clause_mixed = [(i, i % 2 == 1) for i in range(1, n + 1)]
        verify_reduction_equivalence(n, [clause_mixed])


def test_two_literal_clauses():
    """2-literal clauses are the minimum for NAE-SAT. Test systematically."""
    for n in range(2, 5):
        all_literals = [(v, p) for v in range(1, n + 1) for p in [True, False]]
        for l1, l2 in itertools.combinations(all_literals, 2):
            verify_reduction_equivalence(n, [[l1, l2]])


# ---------------------------------------------------------------------------
# 3. Hypothesis property-based tests
# ---------------------------------------------------------------------------

try:
    from hypothesis import given, settings, assume
    from hypothesis import strategies as st
    HAS_HYPOTHESIS = True
except ImportError:
    HAS_HYPOTHESIS = False

if HAS_HYPOTHESIS:
    @given(st.lists(st.booleans(), min_size=2, max_size=8))
    @settings(max_examples=500)
    def test_roundtrip_hypothesis(assignment):
        """Forward-backward roundtrip for random assignments."""
        global passed, failed
        n = len(assignment)
        # Create a single clause that this assignment NAE-satisfies
        # (v1, ~v2) if assignment has at least one T and one F
        if all(assignment) or not any(assignment):
            # All same -> can't NAE-satisfy any clause with all same polarity
            # Just test overhead
            clauses = [[(1, True), (2, True)]]
            universe_size, subsets = reduce(n, clauses)
            check(universe_size == 2 * n, "Hypothesis roundtrip: overhead")
            return

        # Build clause from first True and first False literal
        true_idx = assignment.index(True) + 1
        false_idx = assignment.index(False) + 1
        clauses = [[(true_idx, True), (false_idx, True)]]

        universe_size, subsets = reduce(n, clauses)
        s1 = assignment_to_partition(n, assignment)
        check(is_valid_splitting(universe_size, subsets, s1),
              "Hypothesis roundtrip: partition should be valid")
        recovered = partition_to_assignment(n, s1)
        check(recovered == assignment,
              "Hypothesis roundtrip: recovered != original")

    @given(
        st.integers(min_value=2, max_value=6),
        st.lists(
            st.lists(
                st.tuples(st.integers(min_value=1, max_value=6), st.booleans()),
                min_size=2, max_size=5
            ),
            min_size=1, max_size=6
        )
    )
    @settings(max_examples=500)
    def test_equivalence_hypothesis(n, raw_clauses):
        """Equivalence check on random instances from hypothesis."""
        # Filter literals to valid range
        clauses = []
        for raw_clause in raw_clauses:
            clause = [(v, p) for v, p in raw_clause if 1 <= v <= n]
            # Deduplicate
            clause = list(set(clause))
            if len(clause) >= 2:
                clauses.append(clause)
        assume(len(clauses) >= 1)

        verify_reduction_equivalence(n, clauses)

    @given(
        st.integers(min_value=2, max_value=5),
        st.integers(min_value=1, max_value=4),
    )
    @settings(max_examples=500)
    def test_unsatisfiable_pattern_hypothesis(n, k):
        """
        Test patterns likely to be unsatisfiable:
        For a single variable v1, add clauses with all combinations of other vars.
        """
        assume(n >= 2)
        # Create clauses: (v1, ...) with all combinations of signs for vars 2..min(n,k+1)
        other_vars = list(range(2, min(n, k + 1) + 1))
        assume(len(other_vars) >= 1)
        clauses = []
        for signs in itertools.product([True, False], repeat=len(other_vars)):
            clause = [(1, True)] + [(other_vars[i], signs[i]) for i in range(len(other_vars))]
            clauses.append(clause)
        verify_reduction_equivalence(n, clauses)


# ---------------------------------------------------------------------------
# 4. Run all tests
# ---------------------------------------------------------------------------

def main():
    global passed, failed, bugs

    print("Running YES example test...")
    test_yes_example()

    print("Running NO example test...")
    test_no_example()

    print("Running exhaustive small instances test...")
    test_exhaustive_small()

    print("Running overhead formula test...")
    test_overhead_formula()

    print("Running complementarity test...")
    test_complementarity_always_split()

    print("Running NAE symmetry test...")
    test_nae_symmetry()

    print("Running single clause edge cases test...")
    test_single_clause_edge_cases()

    print("Running two literal clauses test...")
    test_two_literal_clauses()

    if HAS_HYPOTHESIS:
        print("Running hypothesis roundtrip test...")
        test_roundtrip_hypothesis()

        print("Running hypothesis equivalence test...")
        test_equivalence_hypothesis()

        print("Running hypothesis unsatisfiable pattern test...")
        test_unsatisfiable_pattern_hypothesis()
    else:
        print("WARNING: hypothesis not installed, skipping property-based tests")

    total = passed + failed
    print(f"\nTotal checks: {total}")
    print(f"ADVERSARY: NAESatisfiability -> SetSplitting: {passed} passed, {failed} failed")
    if bugs:
        print(f"BUGS FOUND: {bugs}")
    else:
        print("BUGS FOUND: none")

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
