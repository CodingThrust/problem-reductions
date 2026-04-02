#!/usr/bin/env python3
"""
Adversary verification script: Partition -> KthLargestMTuple reduction.
Issue: #395

Independent re-implementation of the reduction logic,
plus property-based testing with hypothesis. >=5000 independent checks.

This script does NOT import from verify_partition_kth_largest_m_tuple.py --
it re-derives everything from scratch as an independent cross-check.
"""

import json
import math
import sys
from itertools import product
from typing import Optional

try:
    from hypothesis import given, settings, assume, HealthCheck
    from hypothesis import strategies as st
    HAS_HYPOTHESIS = True
except ImportError:
    HAS_HYPOTHESIS = False
    print("WARNING: hypothesis not installed; falling back to pure-random adversary tests")


# ---------------------------------------------------------------------------
# Independent re-implementation of reduction
# ---------------------------------------------------------------------------

def adv_reduce(sizes: list[int]) -> dict:
    """
    Independent reduction: Partition -> KthLargestMTuple.

    For each a_i, create X_i = {0, s(a_i)}.
    B = ceil(S/2).
    C = number of subsets with sum > S/2.
    K = C + 1.
    """
    n = len(sizes)
    total = sum(sizes)
    half_float = total / 2

    # Build sets
    target_sets = []
    for s in sizes:
        target_sets.append([0, s])

    # Bound
    b = -(-total // 2)  # ceil division without importing math

    # Count C by enumeration
    c = 0
    for mask in range(1 << n):
        s = 0
        for i in range(n):
            if (mask >> i) & 1:
                s += sizes[i]
        if s > half_float:
            c += 1

    return {"sets": target_sets, "bound": b, "k": c + 1, "c": c}


def adv_solve_partition(sizes: list[int]) -> Optional[list[int]]:
    """Independent brute-force Partition solver."""
    total = sum(sizes)
    if total & 1:
        return None
    half = total >> 1
    n = len(sizes)
    for mask in range(1 << n):
        s = 0
        for i in range(n):
            if (mask >> i) & 1:
                s += sizes[i]
        if s == half:
            return [(mask >> i) & 1 for i in range(n)]
    return None


def adv_count_tuples(sets: list[list[int]], bound: int) -> int:
    """Independent count of m-tuples with sum >= bound."""
    n = len(sets)
    count = 0
    for mask in range(1 << n):
        s = 0
        for i in range(n):
            s += sets[i][(mask >> i) & 1]
        if s >= bound:
            count += 1
    return count


# ---------------------------------------------------------------------------
# Property checks
# ---------------------------------------------------------------------------

def adv_check_all(sizes: list[int]) -> int:
    """Run all adversary checks on a single Partition instance. Returns check count."""
    checks = 0
    n = len(sizes)
    total = sum(sizes)

    r = adv_reduce(sizes)

    # 1. Overhead: m = n, each set has 2 elements
    assert len(r["sets"]) == n, f"num_sets mismatch: {len(r['sets'])} != {n}"
    assert all(len(s) == 2 for s in r["sets"]), "Set size mismatch"
    checks += 1

    # 2. Set values: X_i = {0, s(a_i)}
    for i in range(n):
        assert r["sets"][i][0] == 0, f"Set {i} first element not 0"
        assert r["sets"][i][1] == sizes[i], f"Set {i} second element mismatch"
    checks += 1

    # 3. Bound check
    expected_bound = -(-total // 2)  # ceil(total/2)
    assert r["bound"] == expected_bound, f"Bound mismatch: {r['bound']} != {expected_bound}"
    checks += 1

    # 4. Feasibility agreement
    src_feas = adv_solve_partition(sizes) is not None
    qualifying = adv_count_tuples(r["sets"], r["bound"])
    tgt_feas = qualifying >= r["k"]

    assert src_feas == tgt_feas, (
        f"Feasibility mismatch: sizes={sizes}, src={src_feas}, tgt={tgt_feas}, "
        f"qualifying={qualifying}, k={r['k']}, c={r['c']}"
    )
    checks += 1

    # 5. Forward: feasible source -> feasible target
    if src_feas:
        assert tgt_feas, f"Forward violation: sizes={sizes}"
        checks += 1

    # 6. Infeasible: NO source -> NO target
    if not src_feas:
        assert not tgt_feas, f"Infeasible violation: sizes={sizes}"
        checks += 1

    # 7. Count decomposition check
    # qualifying = C + (number of subsets summing to exactly S/2)
    # When S is odd, no subset sums to S/2, so qualifying should equal C
    if total % 2 == 1:
        assert qualifying == r["c"], (
            f"Odd sum count mismatch: qualifying={qualifying}, c={r['c']}, sizes={sizes}"
        )
        checks += 1
    else:
        half = total // 2
        exact_count = 0
        for mask in range(1 << n):
            s = 0
            for i in range(n):
                if (mask >> i) & 1:
                    s += sizes[i]
            if s == half:
                exact_count += 1
        assert qualifying == r["c"] + exact_count, (
            f"Even sum count mismatch: qualifying={qualifying}, "
            f"c={r['c']}, exact={exact_count}, sizes={sizes}"
        )
        checks += 1

    # 8. Symmetry check: subsets with sum > S/2 and subsets with sum < S/2
    # come in complementary pairs (subset A' has complement with sum S - sum(A'))
    above = 0
    below = 0
    exact = 0
    for mask in range(1 << n):
        s = 0
        for i in range(n):
            if (mask >> i) & 1:
                s += sizes[i]
        if s * 2 > total:
            above += 1
        elif s * 2 < total:
            below += 1
        else:
            exact += 1
    assert above == below, (
        f"Symmetry violation: above={above}, below={below}, sizes={sizes}"
    )
    assert above + below + exact == (1 << n), "Total count mismatch"
    assert above == r["c"], f"C mismatch with above count: {above} != {r['c']}"
    checks += 1

    return checks


# ---------------------------------------------------------------------------
# Test drivers
# ---------------------------------------------------------------------------

def adversary_exhaustive(max_n: int = 5, max_val: int = 8) -> int:
    """Exhaustive adversary tests."""
    checks = 0
    for n in range(1, max_n + 1):
        if n <= 3:
            vr = range(1, max_val + 1)
        elif n == 4:
            vr = range(1, min(max_val, 5) + 1)
        else:
            vr = range(1, min(max_val, 3) + 1)

        for sizes_tuple in product(vr, repeat=n):
            sizes = list(sizes_tuple)
            checks += adv_check_all(sizes)
    return checks


def adversary_random(count: int = 1500, max_n: int = 15, max_val: int = 80) -> int:
    """Random adversary tests with independent RNG seed."""
    import random
    rng = random.Random(9999)  # Different seed from verify script
    checks = 0
    for _ in range(count):
        n = rng.randint(1, max_n)
        sizes = [rng.randint(1, max_val) for _ in range(n)]
        checks += adv_check_all(sizes)
    return checks


def adversary_hypothesis() -> int:
    """Property-based testing with hypothesis."""
    if not HAS_HYPOTHESIS:
        return 0

    checks_counter = [0]

    @given(
        sizes=st.lists(st.integers(min_value=1, max_value=50), min_size=1, max_size=12),
    )
    @settings(
        max_examples=1000,
        suppress_health_check=[HealthCheck.too_slow],
        deadline=None,
    )
    def prop_reduction_correct(sizes):
        checks_counter[0] += adv_check_all(sizes)

    prop_reduction_correct()
    return checks_counter[0]


def adversary_edge_cases() -> int:
    """Targeted edge cases."""
    checks = 0
    edge_cases = [
        [1],                        # Single element, odd sum
        [2],                        # Single element, even sum (no partition: only 1 element)
        [1, 1],                     # Two ones, balanced
        [1, 2],                     # Unbalanced
        [1, 1, 1, 1],              # Uniform even count
        [1, 1, 1],                  # Uniform odd sum
        [5, 5, 5, 5],              # Larger uniform
        [3, 1, 1, 2, 2, 1],       # GJ example
        [5, 3, 3],                  # Odd sum, no partition
        [10, 10],                   # Two equal large
        [1, 2, 3],                  # Sum=6, partition {3} vs {1,2}
        [7, 3, 3, 1],              # Sum=14, partition {7} vs {3,3,1}
        [100, 1],                   # Very unbalanced
        [1, 1, 1, 1, 1, 1, 1, 1], # 8 ones
        [2, 3, 5, 7, 11],          # Primes, sum=28
        [1, 2, 4, 8],              # Powers of 2, sum=15 (odd)
        [1, 2, 4, 8, 16],          # Powers of 2, sum=31 (odd)
        [3, 3, 3, 3],              # Uniform, sum=12
        [50, 50, 50, 50],          # Large uniform
    ]
    for sizes in edge_cases:
        checks += adv_check_all(sizes)
    return checks


if __name__ == "__main__":
    print("=" * 60)
    print("Adversary verification: Partition -> KthLargestMTuple")
    print("=" * 60)

    print("\n[1/4] Edge cases...")
    n_edge = adversary_edge_cases()
    print(f"  Edge case checks: {n_edge}")

    print("\n[2/4] Exhaustive adversary (n <= 5)...")
    n_exh = adversary_exhaustive()
    print(f"  Exhaustive checks: {n_exh}")

    print("\n[3/4] Random adversary (different seed)...")
    n_rand = adversary_random()
    print(f"  Random checks: {n_rand}")

    print("\n[4/4] Hypothesis PBT...")
    n_hyp = adversary_hypothesis()
    print(f"  Hypothesis checks: {n_hyp}")

    total = n_edge + n_exh + n_rand + n_hyp
    print(f"\n  TOTAL adversary checks: {total}")
    assert total >= 5000, f"Need >=5000 checks, got {total}"
    print(f"\nAll {total} adversary checks PASSED.")
