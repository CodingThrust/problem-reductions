#!/usr/bin/env python3
"""
Constructor verification script for ExactCoverBy3Sets -> MinimumWeightSolutionToLinearEquations.
Issue #860.

7 mandatory sections, >= 5000 total checks.

Reduction: Given X3C instance (universe X of size 3q, collection C of 3-element subsets),
build incidence matrix A (3q x n) where A[i][j] = 1 iff u_i in C_j, rhs b = all-ones,
bound K = q. The MWSLE instance asks: is there a rational vector y with Ay=b and
at most K nonzero entries?
"""

import itertools
import json
import os
import random
import sys
from collections import defaultdict
from fractions import Fraction

# ---------------------------------------------------------------------------
# Reduction implementation
# ---------------------------------------------------------------------------

def reduce(universe_size, subsets):
    """
    Reduce X3C to MinimumWeightSolutionToLinearEquations.

    Returns:
        (matrix, rhs, bound) where matrix is list of rows (each row is a list
        of ints), rhs is list of ints, bound is int K.
    """
    n = len(subsets)
    q = universe_size // 3

    # Build incidence matrix A (3q x n)
    matrix = []
    for i in range(universe_size):
        row = []
        for j in range(n):
            row.append(1 if i in subsets[j] else 0)
        matrix.append(row)

    rhs = [1] * universe_size
    bound = q

    return matrix, rhs, bound


def gaussian_elimination_consistent(matrix, rhs, columns):
    """
    Check if the system restricted to given columns is consistent over Q.
    Uses fraction-exact Gaussian elimination.
    """
    n_rows = len(matrix)
    k = len(columns)
    if k == 0:
        return all(b == 0 for b in rhs)

    # Build augmented matrix [A'|b] with Fractions
    aug = []
    for i in range(n_rows):
        row = [Fraction(matrix[i][c]) for c in columns] + [Fraction(rhs[i])]
        aug.append(row)

    pivot_row = 0
    for col in range(k):
        # Find pivot
        found = None
        for r in range(pivot_row, n_rows):
            if aug[r][col] != 0:
                found = r
                break
        if found is None:
            continue

        aug[pivot_row], aug[found] = aug[found], aug[pivot_row]
        pivot_val = aug[pivot_row][col]

        # Eliminate
        for r in range(n_rows):
            if r == pivot_row:
                continue
            factor = aug[r][col] / pivot_val
            for c2 in range(k + 1):
                aug[r][c2] -= factor * aug[pivot_row][c2]

        pivot_row += 1

    # Check consistency: zero-coefficient rows must have zero rhs
    for r in range(pivot_row, n_rows):
        if aug[r][k] != 0:
            return False
    return True


def evaluate_mwsle(matrix, rhs, config):
    """
    Evaluate MWSLE: given binary config (which columns to select),
    check if the restricted system is consistent over Q.
    Returns number of selected columns if consistent, else None.
    """
    columns = [j for j, v in enumerate(config) if v == 1]
    if gaussian_elimination_consistent(matrix, rhs, columns):
        return len(columns)
    return None


def is_exact_cover(universe_size, subsets, config):
    """Check if config selects an exact cover."""
    if len(config) != len(subsets):
        return False
    q = universe_size // 3
    selected = [i for i, v in enumerate(config) if v == 1]
    if len(selected) != q:
        return False
    covered = set()
    for idx in selected:
        for elem in subsets[idx]:
            if elem in covered:
                return False
            covered.add(elem)
    return len(covered) == universe_size


def extract_solution(matrix, rhs, config):
    """
    Extract X3C solution from MWSLE solution.
    The config IS the X3C config (identity mapping: select subset j iff column j selected).
    """
    return list(config)


def brute_force_x3c(universe_size, subsets):
    """Find all exact covers by brute force."""
    n = len(subsets)
    solutions = []
    for bits in itertools.product([0, 1], repeat=n):
        config = list(bits)
        if is_exact_cover(universe_size, subsets, config):
            solutions.append(config)
    return solutions


def brute_force_mwsle(matrix, rhs, bound):
    """
    Find all binary configs with weight <= bound where the restricted system
    is consistent over Q.
    """
    n_cols = len(matrix[0]) if matrix else 0
    solutions = []
    for bits in itertools.product([0, 1], repeat=n_cols):
        config = list(bits)
        weight = sum(config)
        if weight > bound:
            continue
        val = evaluate_mwsle(matrix, rhs, config)
        if val is not None:
            solutions.append(config)
    return solutions


def brute_force_mwsle_optimal(matrix, rhs):
    """Find minimum weight solution (any weight)."""
    n_cols = len(matrix[0]) if matrix else 0
    best_weight = None
    best_solutions = []
    for bits in itertools.product([0, 1], repeat=n_cols):
        config = list(bits)
        val = evaluate_mwsle(matrix, rhs, config)
        if val is not None:
            weight = sum(config)
            if best_weight is None or weight < best_weight:
                best_weight = weight
                best_solutions = [config]
            elif weight == best_weight:
                best_solutions.append(config)
    return best_weight, best_solutions


# ---------------------------------------------------------------------------
# Instance generators
# ---------------------------------------------------------------------------

def generate_all_x3c_small(universe_size, max_num_subsets):
    """Generate all X3C instances for given universe size."""
    elements = list(range(universe_size))
    all_triples = list(itertools.combinations(elements, 3))
    instances = []
    for num_subsets in range(1, min(max_num_subsets + 1, len(all_triples) + 1)):
        for chosen in itertools.combinations(all_triples, num_subsets):
            subsets = [list(t) for t in chosen]
            instances.append((universe_size, subsets))
    return instances


def generate_random_x3c(universe_size, num_subsets, rng):
    """Generate a random X3C instance."""
    elements = list(range(universe_size))
    subsets = []
    for _ in range(num_subsets):
        triple = sorted(rng.sample(elements, 3))
        subsets.append(triple)
    return universe_size, subsets


# ---------------------------------------------------------------------------
# Section 1: Symbolic verification (overhead formulas)
# ---------------------------------------------------------------------------

def section_1_symbolic():
    """Verify overhead formulas symbolically."""
    print("=== Section 1: Symbolic verification ===")
    checks = 0

    for universe_size in [3, 6, 9, 12, 15]:
        for n_subsets in range(1, 12):
            rng = random.Random(universe_size * 100 + n_subsets)
            elems = list(range(universe_size))
            subsets = [sorted(rng.sample(elems, 3)) for _ in range(n_subsets)]

            matrix, rhs, bound = reduce(universe_size, subsets)

            # num_variables (columns) = n
            assert len(matrix[0]) == n_subsets, f"num_variables: {len(matrix[0])} != {n_subsets}"
            checks += 1

            # num_equations (rows) = universe_size = 3q
            assert len(matrix) == universe_size, f"num_equations: {len(matrix)} != {universe_size}"
            checks += 1

            # bound = q = universe_size / 3
            q = universe_size // 3
            assert bound == q, f"bound: {bound} != {q}"
            checks += 1

            # rhs = all-ones of length universe_size
            assert rhs == [1] * universe_size
            checks += 1

            # Each column has exactly 3 ones
            for j in range(n_subsets):
                col_sum = sum(matrix[i][j] for i in range(universe_size))
                assert col_sum == 3, f"Column {j} has {col_sum} ones, expected 3"
                checks += 1

            # Matrix entries are 0 or 1
            for i in range(universe_size):
                for j in range(n_subsets):
                    assert matrix[i][j] in (0, 1)
                    checks += 1

    # Verify incidence structure matches subsets
    for _ in range(300):
        rng_test = random.Random(checks)
        universe_size = rng_test.choice([3, 6, 9])
        n_sub = rng_test.randint(1, 7)
        elems = list(range(universe_size))
        subsets = [sorted(rng_test.sample(elems, 3)) for _ in range(n_sub)]

        matrix, rhs, bound = reduce(universe_size, subsets)

        for i in range(universe_size):
            for j in range(n_sub):
                expected = 1 if i in subsets[j] else 0
                assert matrix[i][j] == expected, (
                    f"matrix[{i}][{j}] = {matrix[i][j]}, expected {expected}"
                )
                checks += 1

    print(f"  Symbolic checks: {checks}")
    return checks


# ---------------------------------------------------------------------------
# Section 2: Exhaustive forward + backward
# ---------------------------------------------------------------------------

def section_2_exhaustive():
    """Exhaustive forward+backward: source feasible <=> target feasible."""
    print("=== Section 2: Exhaustive forward + backward ===")
    checks = 0

    # universe_size=3: all possible subset collections up to 4 subsets
    instances_3 = generate_all_x3c_small(3, 4)
    print(f"  universe_size=3: {len(instances_3)} instances")
    for universe_size, subsets in instances_3:
        source_feasible = len(brute_force_x3c(universe_size, subsets)) > 0
        matrix, rhs, bound = reduce(universe_size, subsets)
        target_feasible = len(brute_force_mwsle(matrix, rhs, bound)) > 0
        assert source_feasible == target_feasible, (
            f"Mismatch u={universe_size}, subsets={subsets}: "
            f"source={source_feasible}, target={target_feasible}"
        )
        checks += 1

    # universe_size=6: up to 5 subsets
    instances_6 = generate_all_x3c_small(6, 5)
    print(f"  universe_size=6: {len(instances_6)} instances")
    for universe_size, subsets in instances_6:
        n = len(subsets)
        if n > 8:
            continue
        source_feasible = len(brute_force_x3c(universe_size, subsets)) > 0
        matrix, rhs, bound = reduce(universe_size, subsets)
        target_feasible = len(brute_force_mwsle(matrix, rhs, bound)) > 0
        assert source_feasible == target_feasible, (
            f"Mismatch u={universe_size}, subsets={subsets}: "
            f"source={source_feasible}, target={target_feasible}"
        )
        checks += 1

    # Random instances
    rng = random.Random(42)
    for _ in range(1500):
        universe_size = rng.choice([3, 6, 9])
        max_sub = {3: 5, 6: 6, 9: 5}[universe_size]
        n_subsets = rng.randint(1, max_sub)
        u, subsets = generate_random_x3c(universe_size, n_subsets, rng)

        source_feasible = len(brute_force_x3c(u, subsets)) > 0
        matrix, rhs, bound = reduce(u, subsets)
        target_feasible = len(brute_force_mwsle(matrix, rhs, bound)) > 0
        assert source_feasible == target_feasible, (
            f"Random mismatch u={u}, subsets={subsets}"
        )
        checks += 1

    print(f"  Exhaustive checks: {checks}")
    return checks


# ---------------------------------------------------------------------------
# Section 3: Solution extraction
# ---------------------------------------------------------------------------

def section_3_extraction():
    """Extract source solution from every feasible target witness."""
    print("=== Section 3: Solution extraction ===")
    checks = 0

    for universe_size in [3, 6]:
        max_sub = {3: 4, 6: 5}[universe_size]
        instances = generate_all_x3c_small(universe_size, max_sub)
        for u, subsets in instances:
            n = len(subsets)
            if n > 8:
                continue
            source_solutions = brute_force_x3c(u, subsets)
            if not source_solutions:
                continue

            matrix, rhs, bound = reduce(u, subsets)
            target_solutions = brute_force_mwsle(matrix, rhs, bound)

            # Every target solution must extract to a valid X3C cover
            for t_sol in target_solutions:
                extracted = extract_solution(matrix, rhs, t_sol)
                assert is_exact_cover(u, subsets, extracted), (
                    f"Extracted not valid: u={u}, subsets={subsets}, t_sol={t_sol}"
                )
                checks += 1

            # Bijection: source solutions = target solutions (identity mapping)
            source_set = {tuple(s) for s in source_solutions}
            target_set = {tuple(s) for s in target_solutions}
            assert source_set == target_set, (
                f"Solution sets differ: u={u}, subsets={subsets}"
            )
            checks += 1

    # Random feasible instances
    rng = random.Random(999)
    for _ in range(500):
        universe_size = rng.choice([3, 6, 9])
        n_subsets = rng.randint(1, min(5, 2 * universe_size // 3 + 2))
        u, subsets = generate_random_x3c(universe_size, n_subsets, rng)

        source_solutions = brute_force_x3c(u, subsets)
        if not source_solutions:
            continue

        matrix, rhs, bound = reduce(u, subsets)
        target_solutions = brute_force_mwsle(matrix, rhs, bound)

        for t_sol in target_solutions:
            extracted = extract_solution(matrix, rhs, t_sol)
            assert is_exact_cover(u, subsets, extracted)
            checks += 1

    print(f"  Extraction checks: {checks}")
    return checks


# ---------------------------------------------------------------------------
# Section 4: Overhead formula
# ---------------------------------------------------------------------------

def section_4_overhead():
    """Build target, measure actual size, compare against formula."""
    print("=== Section 4: Overhead formula ===")
    checks = 0

    rng = random.Random(456)
    for _ in range(2000):
        universe_size = rng.choice([3, 6, 9, 12, 15])
        n_subsets = rng.randint(1, min(10, 3 * universe_size))
        elems = list(range(universe_size))
        subsets = [sorted(rng.sample(elems, 3)) for _ in range(n_subsets)]

        matrix, rhs, bound = reduce(universe_size, subsets)

        # num_variables = n_subsets (columns)
        assert len(matrix[0]) == n_subsets
        checks += 1

        # num_equations = universe_size (rows)
        assert len(matrix) == universe_size
        checks += 1

        # bound = universe_size / 3
        assert bound == universe_size // 3
        checks += 1

        # rhs is all-ones
        assert all(b == 1 for b in rhs)
        checks += 1

        # Matrix dimensions
        assert len(rhs) == len(matrix)
        checks += 1

        # Verify A is the incidence matrix
        for j in range(n_subsets):
            col_ones = [i for i in range(universe_size) if matrix[i][j] == 1]
            assert sorted(col_ones) == sorted(subsets[j]), (
                f"Column {j} ones {col_ones} != subset {subsets[j]}"
            )
            checks += 1

    print(f"  Overhead checks: {checks}")
    return checks


# ---------------------------------------------------------------------------
# Section 5: Structural properties
# ---------------------------------------------------------------------------

def section_5_structural():
    """Target well-formed, no degenerate cases."""
    print("=== Section 5: Structural properties ===")
    checks = 0

    rng = random.Random(789)
    for _ in range(800):
        universe_size = rng.choice([3, 6, 9, 12, 15])
        n_subsets = rng.randint(1, min(10, 3 * universe_size))
        elems = list(range(universe_size))
        subsets = [sorted(rng.sample(elems, 3)) for _ in range(n_subsets)]

        matrix, rhs, bound = reduce(universe_size, subsets)

        # Matrix entries are 0 or 1
        for i in range(len(matrix)):
            for j in range(len(matrix[0])):
                assert matrix[i][j] in (0, 1)
                checks += 1

        # Each column has exactly 3 ones (each subset has 3 elements)
        for j in range(n_subsets):
            col_sum = sum(matrix[i][j] for i in range(universe_size))
            assert col_sum == 3, f"Column {j} sum = {col_sum}"
            checks += 1

        # Each row sum equals the number of subsets containing that element
        element_counts = defaultdict(int)
        for s in subsets:
            for elem in s:
                element_counts[elem] += 1
        for i in range(universe_size):
            row_sum = sum(matrix[i])
            assert row_sum == element_counts[i]
            checks += 1

        # RHS positive
        for b in rhs:
            assert b > 0
            checks += 1

        # Bound is positive
        assert bound > 0
        checks += 1

        # Total ones in matrix = 3 * n_subsets
        total_ones = sum(sum(row) for row in matrix)
        assert total_ones == 3 * n_subsets
        checks += 1

    print(f"  Structural checks: {checks}")
    return checks


# ---------------------------------------------------------------------------
# Section 6: YES example
# ---------------------------------------------------------------------------

def section_6_yes_example():
    """Reproduce exact Typst feasible example numbers."""
    print("=== Section 6: YES example ===")
    checks = 0

    # From Typst: X = {0,...,5}, q=2
    # C1={0,1,2}, C2={3,4,5}, C3={0,3,4}
    universe_size = 6
    subsets = [[0, 1, 2], [3, 4, 5], [0, 3, 4]]

    matrix, rhs, bound = reduce(universe_size, subsets)

    # num_variables = 3
    assert len(matrix[0]) == 3
    checks += 1

    # num_equations = 6
    assert len(matrix) == 6
    checks += 1

    # bound = 2
    assert bound == 2
    checks += 1

    # Check matrix entries from Typst
    expected_matrix = [
        [1, 0, 1],  # u0: in C1, C3
        [1, 0, 0],  # u1: in C1
        [1, 0, 0],  # u2: in C1
        [0, 1, 1],  # u3: in C2, C3
        [0, 1, 1],  # u4: in C2, C3
        [0, 1, 0],  # u5: in C2
    ]
    assert matrix == expected_matrix
    checks += 1

    # rhs = all-ones
    assert rhs == [1, 1, 1, 1, 1, 1]
    checks += 1

    # Solution y = (1, 1, 0): select C1, C2
    config = [1, 1, 0]
    val = evaluate_mwsle(matrix, rhs, config)
    assert val == 2
    checks += 1

    assert is_exact_cover(universe_size, subsets, config)
    checks += 1

    # Verify Ay = b manually
    for i in range(6):
        dot = sum(matrix[i][j] * config[j] for j in range(3))
        assert dot == 1, f"Row {i}: dot = {dot}"
        checks += 1

    # Verify y = (0, 0, 1) does NOT work (C3 covers {0,3,4}, only 3 elements)
    val2 = evaluate_mwsle(matrix, rhs, [0, 0, 1])
    assert val2 is None or val2 == 1  # weight 1 but system inconsistent
    # Actually: restricted to column 2, A' is column [1,0,0,1,1,0], rhs [1,1,1,1,1,1]
    # Row 1: 0*y = 1 => inconsistent
    assert evaluate_mwsle(matrix, rhs, [0, 0, 1]) is None
    checks += 1

    # Check all 8 configs
    feasible_configs = []
    for bits in itertools.product([0, 1], repeat=3):
        config = list(bits)
        val = evaluate_mwsle(matrix, rhs, config)
        if val is not None and val <= bound:
            feasible_configs.append(config)
        checks += 1

    # Only (1,1,0) should be feasible with weight <= 2
    assert feasible_configs == [[1, 1, 0]], f"Got {feasible_configs}"
    checks += 1

    print(f"  YES example checks: {checks}")
    return checks


# ---------------------------------------------------------------------------
# Section 7: NO example
# ---------------------------------------------------------------------------

def section_7_no_example():
    """Reproduce exact Typst infeasible example, verify both sides infeasible."""
    print("=== Section 7: NO example ===")
    checks = 0

    # From Typst: X = {0,...,5}, q=2
    # C1={0,1,2}, C2={0,3,4}, C3={0,4,5}
    universe_size = 6
    subsets = [[0, 1, 2], [0, 3, 4], [0, 4, 5]]

    # Verify no exact cover
    source_solutions = brute_force_x3c(universe_size, subsets)
    assert len(source_solutions) == 0
    checks += 1

    matrix, rhs, bound = reduce(universe_size, subsets)

    # Verify matrix
    expected_matrix = [
        [1, 1, 1],  # u0: in C1, C2, C3
        [1, 0, 0],  # u1: in C1
        [1, 0, 0],  # u2: in C1
        [0, 1, 0],  # u3: in C2
        [0, 1, 1],  # u4: in C2, C3
        [0, 0, 1],  # u5: in C3
    ]
    assert matrix == expected_matrix
    checks += 1

    # Verify no MWSLE solution with weight <= K=2
    target_solutions = brute_force_mwsle(matrix, rhs, bound)
    assert len(target_solutions) == 0
    checks += 1

    # Check all 8 configs
    for bits in itertools.product([0, 1], repeat=3):
        config = list(bits)
        val = evaluate_mwsle(matrix, rhs, config)
        weight = sum(config)
        if val is not None and weight <= bound:
            assert False, f"Unexpected feasible config: {config}"
        checks += 1

    # From Typst: row 1 forces y1=1, row 3 forces y2=1.
    # Then row 0: y1+y2+y3 = 1 => 1+1+y3=1 => y3=-1.
    # So 3 nonzero entries needed, but K=2.
    # Check (1,1,1): system consistent (3 columns span all rows)?
    val_all = evaluate_mwsle(matrix, rhs, [1, 1, 1])
    # With all 3 columns, the system [1,1,1;1,0,0;1,0,0;0,1,0;0,1,1;0,0,1]y=[1,1,1,1,1,1]
    # Row 1: y1=1, Row 3: y2=1, Row 5: y3=1, Row 0: 1+1+1=3!=1? No: over rationals.
    # Row 1: y1=1. Row 2: y1=1 (redundant). Row 3: y2=1. Row 5: y3=1.
    # Row 4: y2+y3=1 => 1+1=2!=1. Inconsistent!
    # Actually wait, let me re-check. The system is Ay=b where y can be any rationals.
    # Row 1: 1*y1 + 0*y2 + 0*y3 = 1 => y1=1
    # Row 3: 0*y1 + 1*y2 + 0*y3 = 1 => y2=1
    # Row 5: 0*y1 + 0*y2 + 1*y3 = 1 => y3=1
    # Row 0: 1*1 + 1*1 + 1*1 = 3 != 1 => INCONSISTENT
    assert val_all is None, "Expected inconsistent with all columns"
    checks += 1

    # Verify no config works at all (not just weight<=2)
    _, any_solutions = brute_force_mwsle_optimal(matrix, rhs)
    # Actually the system may be solvable with some config... let me check all
    any_feasible = False
    for bits in itertools.product([0, 1], repeat=3):
        config = list(bits)
        val = evaluate_mwsle(matrix, rhs, config)
        if val is not None:
            any_feasible = True
        checks += 1

    # Actually (1,1,0): restricted to cols 0,1: A'=[[1,1],[1,0],[1,0],[0,1],[0,1],[0,0]]
    # Row 5: 0*y1+0*y2=0!=1 => inconsistent
    # (1,0,1): restricted to cols 0,2: A'=[[1,1],[1,0],[1,0],[0,0],[0,1],[0,1]]
    # Row 3: 0*y1+0*y3=0!=1 => inconsistent
    # (0,1,1): restricted to cols 1,2: A'=[[1,1],[0,0],[0,0],[1,0],[1,1],[0,1]]
    # Row 1: 0*y2+0*y3=0!=1 => inconsistent
    # So truly no solution exists at any weight
    assert not any_feasible, "Expected no feasible config at all"
    checks += 1

    print(f"  NO example checks: {checks}")
    return checks


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    total_checks = 0

    c1 = section_1_symbolic()
    total_checks += c1

    c2 = section_2_exhaustive()
    total_checks += c2

    c3 = section_3_extraction()
    total_checks += c3

    c4 = section_4_overhead()
    total_checks += c4

    c5 = section_5_structural()
    total_checks += c5

    c6 = section_6_yes_example()
    total_checks += c6

    c7 = section_7_no_example()
    total_checks += c7

    print(f"\n{'='*60}")
    print(f"CHECK COUNT AUDIT:")
    print(f"  Total checks:          {total_checks} (minimum: 5,000)")
    print(f"  Section 1 (symbolic):  {c1}")
    print(f"  Section 2 (exhaustive):{c2}")
    print(f"  Section 3 (extraction):{c3}")
    print(f"  Section 4 (overhead):  {c4}")
    print(f"  Section 5 (structural):{c5}")
    print(f"  Section 6 (YES):       {c6}")
    print(f"  Section 7 (NO):        {c7}")
    print(f"{'='*60}")

    if total_checks < 5000:
        print(f"FAIL: Total checks {total_checks} < 5000 minimum!")
        sys.exit(1)

    print("ALL CHECKS PASSED")

    # Export test vectors
    export_test_vectors()


def export_test_vectors():
    """Export test vectors JSON."""
    # YES instance
    yes_universe = 6
    yes_subsets = [[0, 1, 2], [3, 4, 5], [0, 3, 4]]
    yes_matrix, yes_rhs, yes_bound = reduce(yes_universe, yes_subsets)
    yes_config = [1, 1, 0]

    # NO instance
    no_universe = 6
    no_subsets = [[0, 1, 2], [0, 3, 4], [0, 4, 5]]
    no_matrix, no_rhs, no_bound = reduce(no_universe, no_subsets)

    test_vectors = {
        "source": "ExactCoverBy3Sets",
        "target": "MinimumWeightSolutionToLinearEquations",
        "issue": 860,
        "yes_instance": {
            "input": {
                "universe_size": yes_universe,
                "subsets": yes_subsets
            },
            "output": {
                "matrix": yes_matrix,
                "rhs": yes_rhs,
                "bound": yes_bound
            },
            "source_feasible": True,
            "target_feasible": True,
            "source_solution": yes_config,
            "extracted_solution": yes_config
        },
        "no_instance": {
            "input": {
                "universe_size": no_universe,
                "subsets": no_subsets
            },
            "output": {
                "matrix": no_matrix,
                "rhs": no_rhs,
                "bound": no_bound
            },
            "source_feasible": False,
            "target_feasible": False
        },
        "overhead": {
            "num_variables": "num_subsets",
            "num_equations": "universe_size",
            "bound": "universe_size / 3"
        },
        "claims": [
            {"tag": "variables_equal_subsets", "formula": "num_variables = num_subsets", "verified": True},
            {"tag": "equations_equal_universe_size", "formula": "num_equations = universe_size", "verified": True},
            {"tag": "bound_equals_q", "formula": "bound = universe_size / 3", "verified": True},
            {"tag": "incidence_matrix_01", "formula": "A[i][j] = 1 iff u_i in C_j", "verified": True},
            {"tag": "each_column_3_ones", "formula": "each column has exactly 3 ones", "verified": True},
            {"tag": "forward_direction", "formula": "exact cover => MWSLE feasible with weight q", "verified": True},
            {"tag": "backward_direction", "formula": "MWSLE feasible with weight <= q => exact cover", "verified": True},
            {"tag": "solution_extraction", "formula": "target config = source config (identity)", "verified": True}
        ]
    }

    out_path = os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        "test_vectors_exact_cover_by_3_sets_minimum_weight_solution_to_linear_equations.json"
    )
    with open(out_path, "w") as f:
        json.dump(test_vectors, f, indent=2)
    print(f"Test vectors exported to {out_path}")


if __name__ == "__main__":
    main()
