#!/usr/bin/env python3
"""
Verification script: Partition -> KthLargestMTuple reduction.
Issue: #395
Reference: Garey & Johnson, Computers and Intractability, SP21, p.225
           Johnson and Mizoguchi (1978)

Seven mandatory sections:
  1. reduce()         — the reduction function
  2. extract()        — solution extraction (N/A for Turing reduction; stub)
  3. Brute-force solvers for source and target
  4. Forward: YES source -> YES target
  5. Backward: YES target -> YES source
  6. Infeasible: NO source -> NO target
  7. Overhead check

Runs >=5000 checks total, with exhaustive coverage for small n.
"""

import json
import math
import sys
from itertools import product
from typing import Optional

# ---------------------------------------------------------------------------
# Section 1: reduce()
# ---------------------------------------------------------------------------

def reduce(sizes: list[int]) -> dict:
    """
    Reduce Partition(sizes) -> KthLargestMTuple.

    Returns dict with keys: sets, bound, k.

    Given A = {a_1, ..., a_n} with sizes s(a_i) and S = sum(sizes):
      - m = n, each X_i = {0*, s(a_i)} (using 0 as placeholder for "exclude")
      - B = ceil(S / 2)
      - C = count of tuples with sum > S/2 (subsets with sum > S/2)
      - K = C + 1

    NOTE: This is a Turing reduction because computing C requires counting
    subsets, which is #P-hard in general. For verification we compute C
    by brute force on small instances.
    """
    n = len(sizes)
    s_total = sum(sizes)

    # Build sets: each X_i = [0, s(a_i)] (index 0 = exclude, index 1 = include)
    # For the KthLargestMTuple model, sizes must be positive, so we represent
    # with actual size values. Use a sentinel approach: X_i = {1, s(a_i) + 1}
    # with bound adjusted. But the actual model in the codebase uses raw positive
    # integers.
    #
    # Actually, looking at the model: sets contain positive integers, and evaluate
    # checks if tuple sum >= bound. The issue description uses X_i = {0, s(a_i)}
    # but the model requires all sizes > 0. We work in the mathematical formulation
    # where 0 is allowed (the bijection still holds).
    #
    # For the Python verification, we work with the mathematical formulation directly.
    sets = [[0, s] for s in sizes]

    # Bound
    bound = math.ceil(s_total / 2)

    # Count C: subsets with sum strictly > S/2
    # Each m-tuple corresponds to a subset (include a_i iff x_i = s(a_i))
    c = 0
    half = s_total / 2  # Use float for exact comparison
    for bits in range(1 << n):
        subset_sum = sum(sizes[i] for i in range(n) if (bits >> i) & 1)
        if subset_sum > half:
            c += 1

    k = c + 1

    return {"sets": sets, "bound": bound, "k": k, "c": c}


# ---------------------------------------------------------------------------
# Section 2: extract() — N/A for Turing reduction
# ---------------------------------------------------------------------------

def extract(sizes: list[int], target_answer: bool) -> Optional[list[int]]:
    """
    Solution extraction is not applicable for this Turing reduction.
    The KthLargestMTuple answer is a YES/NO count comparison.
    We return None; correctness is verified via feasibility agreement.
    """
    return None


# ---------------------------------------------------------------------------
# Section 3: Brute-force solvers
# ---------------------------------------------------------------------------

def solve_partition(sizes: list[int]) -> Optional[list[int]]:
    """Brute-force Partition solver. Returns config or None."""
    total = sum(sizes)
    if total % 2 != 0:
        return None
    half = total // 2
    n = len(sizes)
    for bits in range(1 << n):
        subset_sum = sum(sizes[i] for i in range(n) if (bits >> i) & 1)
        if subset_sum == half:
            config = [(bits >> i) & 1 for i in range(n)]
            return config
    return None


def is_partition_feasible(sizes: list[int]) -> bool:
    """Check if Partition instance is feasible."""
    return solve_partition(sizes) is not None


def count_qualifying_tuples(sets: list[list[int]], bound: int) -> int:
    """
    Count m-tuples in X_1 x ... x X_m with sum >= bound.
    Each set has exactly 2 elements [0, s_i].
    """
    n = len(sets)
    count = 0
    for bits in range(1 << n):
        # bit i = 0 -> pick sets[i][0], bit i = 1 -> pick sets[i][1]
        total = sum(sets[i][(bits >> i) & 1] for i in range(n))
        if total >= bound:
            count += 1
    return count


def is_kth_largest_feasible(sets: list[list[int]], k: int, bound: int) -> bool:
    """Check if KthLargestMTuple instance is feasible (count >= k)."""
    return count_qualifying_tuples(sets, bound) >= k


# ---------------------------------------------------------------------------
# Section 4: Forward check -- YES source -> YES target
# ---------------------------------------------------------------------------

def check_forward(sizes: list[int]) -> bool:
    """
    If Partition(sizes) is feasible,
    then KthLargestMTuple(reduce(sizes)) must also be feasible.
    """
    if not is_partition_feasible(sizes):
        return True  # vacuously true
    r = reduce(sizes)
    return is_kth_largest_feasible(r["sets"], r["k"], r["bound"])


# ---------------------------------------------------------------------------
# Section 5: Backward check -- YES target -> YES source
# ---------------------------------------------------------------------------

def check_backward(sizes: list[int]) -> bool:
    """
    If KthLargestMTuple(reduce(sizes)) is feasible,
    then Partition(sizes) must also be feasible.
    """
    r = reduce(sizes)
    if not is_kth_largest_feasible(r["sets"], r["k"], r["bound"]):
        return True  # vacuously true
    return is_partition_feasible(sizes)


# ---------------------------------------------------------------------------
# Section 6: Infeasible check -- NO source -> NO target
# ---------------------------------------------------------------------------

def check_infeasible(sizes: list[int]) -> bool:
    """
    If Partition(sizes) is infeasible,
    then KthLargestMTuple(reduce(sizes)) must also be infeasible.
    """
    if is_partition_feasible(sizes):
        return True  # not infeasible; skip
    r = reduce(sizes)
    return not is_kth_largest_feasible(r["sets"], r["k"], r["bound"])


# ---------------------------------------------------------------------------
# Section 7: Overhead check
# ---------------------------------------------------------------------------

def check_overhead(sizes: list[int]) -> bool:
    """
    Verify overhead:
      num_sets = num_elements (= n)
      total_set_sizes = 2 * num_elements (= 2n, each set has 2 elements)
    """
    r = reduce(sizes)
    n = len(sizes)
    sets = r["sets"]

    # num_sets = n
    if len(sets) != n:
        return False
    # Each set has exactly 2 elements
    if not all(len(s) == 2 for s in sets):
        return False
    # total_set_sizes = 2n
    total_sizes = sum(len(s) for s in sets)
    if total_sizes != 2 * n:
        return False
    return True


# ---------------------------------------------------------------------------
# Section 7b: Count consistency check
# ---------------------------------------------------------------------------

def check_count_consistency(sizes: list[int]) -> bool:
    """
    Cross-check: the count of qualifying tuples matches our C calculation.
    Specifically:
      - If Partition is feasible (S even, balanced partition exists):
        qualifying = C + P where P = number of balanced subsets >= 1
        so qualifying >= C + 1 = K
      - If Partition is infeasible:
        qualifying = C (when S even but no balanced partition)
        or qualifying = C (when S odd, since ceil(S/2) > S/2 means
                          tuples with sum >= ceil(S/2) are exactly those > S/2)
        so qualifying < K
    """
    r = reduce(sizes)
    s_total = sum(sizes)
    c = r["c"]
    k = r["k"]
    bound = r["bound"]
    qualifying = count_qualifying_tuples(r["sets"], bound)
    n = len(sizes)

    # Count exact-half subsets (if S is even)
    exact_half_count = 0
    if s_total % 2 == 0:
        half = s_total // 2
        for bits in range(1 << n):
            ss = sum(sizes[i] for i in range(n) if (bits >> i) & 1)
            if ss == half:
                exact_half_count += 1

    # When S is even: qualifying = C + exact_half_count
    # When S is odd: qualifying = C (since bound = ceil(S/2) and all sums are integers)
    if s_total % 2 == 0:
        expected = c + exact_half_count
    else:
        expected = c

    if qualifying != expected:
        return False

    # Feasibility cross-check
    partition_feas = is_partition_feasible(sizes)
    if partition_feas:
        assert exact_half_count >= 1
        assert qualifying >= k
    else:
        assert qualifying < k or qualifying == c

    return True


# ---------------------------------------------------------------------------
# Exhaustive + random test driver
# ---------------------------------------------------------------------------

def exhaustive_tests(max_n: int = 5, max_val: int = 8) -> int:
    """
    Exhaustive tests for all Partition instances with n <= max_n,
    element values in [1, max_val].
    Returns number of checks performed.
    """
    checks = 0
    for n in range(1, max_n + 1):
        if n <= 3:
            val_range = range(1, max_val + 1)
        elif n == 4:
            val_range = range(1, min(max_val, 5) + 1)
        else:
            val_range = range(1, min(max_val, 3) + 1)

        for sizes_tuple in product(val_range, repeat=n):
            sizes = list(sizes_tuple)
            assert check_forward(sizes), f"Forward FAILED: sizes={sizes}"
            assert check_backward(sizes), f"Backward FAILED: sizes={sizes}"
            assert check_infeasible(sizes), f"Infeasible FAILED: sizes={sizes}"
            assert check_overhead(sizes), f"Overhead FAILED: sizes={sizes}"
            assert check_count_consistency(sizes), f"Count consistency FAILED: sizes={sizes}"
            checks += 5
    return checks


def random_tests(count: int = 2000, max_n: int = 15, max_val: int = 100) -> int:
    """Random tests with larger instances. Returns number of checks."""
    import random
    rng = random.Random(42)
    checks = 0
    for _ in range(count):
        n = rng.randint(1, max_n)
        sizes = [rng.randint(1, max_val) for _ in range(n)]
        assert check_forward(sizes), f"Forward FAILED: sizes={sizes}"
        assert check_backward(sizes), f"Backward FAILED: sizes={sizes}"
        assert check_infeasible(sizes), f"Infeasible FAILED: sizes={sizes}"
        assert check_overhead(sizes), f"Overhead FAILED: sizes={sizes}"
        assert check_count_consistency(sizes), f"Count consistency FAILED: sizes={sizes}"
        checks += 5
    return checks


def collect_test_vectors(count: int = 20) -> list[dict]:
    """Collect representative test vectors for downstream consumption."""
    import random
    rng = random.Random(123)
    vectors = []

    # Hand-crafted vectors
    hand_crafted = [
        {"sizes": [3, 1, 1, 2, 2, 1], "label": "yes_balanced_partition"},
        {"sizes": [5, 3, 3], "label": "no_odd_sum"},
        {"sizes": [1, 1, 1, 1], "label": "yes_uniform_even"},
        {"sizes": [1, 2, 3, 4, 5], "label": "no_odd_sum_15"},
        {"sizes": [1, 2, 3, 4, 5, 5], "label": "yes_sum_20"},
        {"sizes": [10], "label": "no_single_element"},
        {"sizes": [1, 1], "label": "yes_two_ones"},
        {"sizes": [1, 2], "label": "no_unbalanced"},
        {"sizes": [7, 3, 3, 1], "label": "yes_sum_14"},
        {"sizes": [100, 1, 1, 1], "label": "no_huge_element"},
    ]

    for hc in hand_crafted:
        sizes = hc["sizes"]
        r = reduce(sizes)
        source_sol = solve_partition(sizes)
        qualifying = count_qualifying_tuples(r["sets"], r["bound"])
        vectors.append({
            "label": hc["label"],
            "source": {"sizes": sizes},
            "target": {
                "sets": r["sets"],
                "k": r["k"],
                "bound": r["bound"],
            },
            "source_feasible": source_sol is not None,
            "target_feasible": qualifying >= r["k"],
            "source_solution": source_sol,
            "qualifying_count": qualifying,
            "c_strict": r["c"],
        })

    # Random vectors
    for i in range(count - len(hand_crafted)):
        n = rng.randint(1, 8)
        sizes = [rng.randint(1, 20) for _ in range(n)]
        r = reduce(sizes)
        source_sol = solve_partition(sizes)
        qualifying = count_qualifying_tuples(r["sets"], r["bound"])
        vectors.append({
            "label": f"random_{i}",
            "source": {"sizes": sizes},
            "target": {
                "sets": r["sets"],
                "k": r["k"],
                "bound": r["bound"],
            },
            "source_feasible": source_sol is not None,
            "target_feasible": qualifying >= r["k"],
            "source_solution": source_sol,
            "qualifying_count": qualifying,
            "c_strict": r["c"],
        })

    return vectors


if __name__ == "__main__":
    print("=" * 60)
    print("Partition -> KthLargestMTuple verification")
    print("=" * 60)

    print("\n[1/3] Exhaustive tests (n <= 5)...")
    n_exhaustive = exhaustive_tests()
    print(f"  Exhaustive checks: {n_exhaustive}")

    print("\n[2/3] Random tests...")
    n_random = random_tests(count=2000)
    print(f"  Random checks: {n_random}")

    total = n_exhaustive + n_random
    print(f"\n  TOTAL checks: {total}")
    assert total >= 5000, f"Need >=5000 checks, got {total}"

    print("\n[3/3] Generating test vectors...")
    vectors = collect_test_vectors(count=20)

    # Validate all vectors
    for v in vectors:
        src_feas = v["source_feasible"]
        tgt_feas = v["target_feasible"]
        assert src_feas == tgt_feas, (
            f"Feasibility mismatch in {v['label']}: "
            f"source={src_feas}, target={tgt_feas}"
        )

    # Write test vectors
    out_path = "docs/paper/verify-reductions/test_vectors_partition_kth_largest_m_tuple.json"
    with open(out_path, "w") as f:
        json.dump({"vectors": vectors, "total_checks": total}, f, indent=2)
    print(f"  Wrote {len(vectors)} test vectors to {out_path}")

    print(f"\nAll {total} checks PASSED.")
