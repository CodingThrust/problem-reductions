#!/usr/bin/env python3
"""
Adversary verification script for SubsetSum -> Partition reduction (#973).

Independent implementation based solely on the Typst proof specification.
Does NOT import or reference the constructor's verify_subsetsum_partition.py.
"""

import itertools
import sys
from typing import List, Optional, Tuple

from hypothesis import given, settings, HealthCheck
from hypothesis import strategies as st

# ---------------------------------------------------------------------------
# Core types
# ---------------------------------------------------------------------------

class SubsetSumInstance:
    """Subset Sum: given multiset S of positive ints and target T, find A subset S with sum(A) = T."""
    def __init__(self, elements: List[int], target: int):
        self.elements = list(elements)
        self.target = target
        self.n = len(self.elements)
        self.sigma = sum(self.elements)


class PartitionInstance:
    """Partition: given multiset S', find a partition into two parts with equal sum."""
    def __init__(self, elements: List[int]):
        self.elements = list(elements)
        self.sigma_prime = sum(self.elements)
        self.half = self.sigma_prime / 2  # may not be integer


# ---------------------------------------------------------------------------
# Reduction: SubsetSum -> Partition (from Typst proof)
# ---------------------------------------------------------------------------

def reduce(ss: SubsetSumInstance) -> PartitionInstance:
    """
    Construction from the Typst proof:
    1. Compute d = |Sigma - 2T|
    2. If d == 0: S' = S
    3. If d > 0: S' = S union {d}
    """
    sigma = ss.sigma
    T = ss.target
    d = abs(sigma - 2 * T)
    if d == 0:
        return PartitionInstance(list(ss.elements))
    else:
        return PartitionInstance(list(ss.elements) + [d])


def extract_solution(ss: SubsetSumInstance, partition_config: List[int]) -> List[int]:
    """
    Given a balanced partition config (list of 0/1 for each element in S'),
    extract the SubsetSum solution (indicator over original n elements).

    From the Typst proof:
    - If d == 0: either side works; return indicator of P_1 restricted to original elements.
    - If Sigma > 2T: S-elements on SAME side as d sum to T.
    - If Sigma < 2T: S-elements on OPPOSITE side from d sum to T.
    """
    sigma = ss.sigma
    T = ss.target
    d = abs(sigma - 2 * T)
    n = ss.n

    if d == 0:
        # Either side; return the side-0 indicator restricted to original elements
        return list(partition_config[:n])
    else:
        # d is the last element (index n)
        pad_side = partition_config[n]
        if sigma > 2 * T:
            # Same side as d
            target_side = pad_side
        else:
            # Opposite side from d
            target_side = 1 - pad_side
        return [1 if partition_config[i] == target_side else 0 for i in range(n)]


# ---------------------------------------------------------------------------
# Feasibility checkers
# ---------------------------------------------------------------------------

def is_feasible_source(ss: SubsetSumInstance) -> bool:
    """Check if any subset of S sums to T (brute force)."""
    for mask in range(1 << ss.n):
        s = 0
        for i in range(ss.n):
            if mask & (1 << i):
                s += ss.elements[i]
        if s == ss.target:
            return True
    return False


def find_source_solution(ss: SubsetSumInstance) -> Optional[List[int]]:
    """Find a subset summing to T, return indicator or None."""
    for mask in range(1 << ss.n):
        s = 0
        for i in range(ss.n):
            if mask & (1 << i):
                s += ss.elements[i]
        if s == ss.target:
            return [1 if (mask & (1 << i)) else 0 for i in range(ss.n)]
    return None


def is_feasible_target(part: PartitionInstance) -> bool:
    """Check if any subset of S' sums to Sigma'/2 (brute force)."""
    if part.sigma_prime % 2 != 0:
        return False
    half = part.sigma_prime // 2
    m = len(part.elements)
    for mask in range(1 << m):
        s = 0
        for i in range(m):
            if mask & (1 << i):
                s += part.elements[i]
        if s == half:
            return True
    return False


def find_target_solution(part: PartitionInstance) -> Optional[List[int]]:
    """Find a balanced partition, return config (0/1 per element) or None."""
    if part.sigma_prime % 2 != 0:
        return None
    half = part.sigma_prime // 2
    m = len(part.elements)
    for mask in range(1 << m):
        s = 0
        for i in range(m):
            if mask & (1 << i):
                s += part.elements[i]
        if s == half:
            return [1 if (mask & (1 << i)) else 0 for i in range(m)]
    return None


# ---------------------------------------------------------------------------
# Test tracking
# ---------------------------------------------------------------------------

passed = 0
failed = 0
bugs = []


def check(condition: bool, msg: str):
    global passed, failed, bugs
    if condition:
        passed += 1
    else:
        failed += 1
        bugs.append(msg)
        print(f"FAIL: {msg}", file=sys.stderr)


# ---------------------------------------------------------------------------
# 1. Exhaustive tests for n <= 5
# ---------------------------------------------------------------------------

def test_exhaustive():
    """Exhaustive forward + backward + extraction for all instances with n <= 5."""
    global passed, failed
    count = 0
    for n in range(0, 6):
        # Generate elements from {1, ..., 10} to keep it tractable
        max_val = min(10, 20)
        if n == 0:
            element_lists = [[]]
        elif n <= 3:
            element_lists = list(itertools.product(range(1, max_val + 1), repeat=n))
        elif n == 4:
            element_lists = list(itertools.product(range(1, 7), repeat=n))
        else:
            # n=5: use smaller range
            element_lists = list(itertools.product(range(1, 5), repeat=n))

        for elems in element_lists:
            elems = list(elems)
            sigma = sum(elems)
            # Test various targets including edge cases
            targets = set(range(0, sigma + 2))  # 0 to sigma+1
            # Also add some larger targets
            targets.add(sigma + 5)
            for T in targets:
                ss = SubsetSumInstance(elems, T)
                part = reduce(ss)

                src_feasible = is_feasible_source(ss)
                tgt_feasible = is_feasible_target(part)

                # Forward: source feasible => target feasible
                if src_feasible:
                    check(tgt_feasible,
                          f"Forward failed: S={elems}, T={T}, sigma={sigma}")

                # Backward: target feasible => source feasible
                if tgt_feasible:
                    check(src_feasible,
                          f"Backward failed: S={elems}, T={T}, sigma={sigma}")

                # Equivalence
                check(src_feasible == tgt_feasible,
                      f"Equivalence failed: S={elems}, T={T}, src={src_feasible}, tgt={tgt_feasible}")

                # Extraction test: if target has a solution, extract and verify
                if tgt_feasible:
                    tgt_sol = find_target_solution(part)
                    if tgt_sol is not None:
                        extracted = extract_solution(ss, tgt_sol)
                        ext_sum = sum(e * x for e, x in zip(ss.elements, extracted))
                        check(ext_sum == T,
                              f"Extraction failed: S={elems}, T={T}, extracted_sum={ext_sum}")

                # Overhead: partition has at most n+1 elements
                check(len(part.elements) <= n + 1,
                      f"Overhead failed: S={elems}, T={T}, |S'|={len(part.elements)}, n+1={n+1}")

                # When sigma == 2T, should have exactly n elements
                if sigma == 2 * T:
                    check(len(part.elements) == n,
                          f"Overhead d=0 case: S={elems}, T={T}, |S'|={len(part.elements)} != {n}")

                count += 1
                if count % 5000 == 0:
                    print(f"  Exhaustive progress: {count} instances checked...")

    print(f"  Exhaustive tests completed: {count} instances")


# ---------------------------------------------------------------------------
# 2. YES example from Typst
# ---------------------------------------------------------------------------

def test_yes_example():
    """Reproduce the YES example from the Typst proof."""
    S = [3, 5, 7, 1, 4]
    T = 8
    ss = SubsetSumInstance(S, T)

    check(ss.sigma == 20, f"YES: sigma should be 20, got {ss.sigma}")
    check(ss.sigma > 2 * T, f"YES: should have sigma > 2T")

    d = abs(ss.sigma - 2 * T)
    check(d == 4, f"YES: d should be 4, got {d}")

    part = reduce(ss)
    check(part.elements == [3, 5, 7, 1, 4, 4], f"YES: S' should be [3,5,7,1,4,4], got {part.elements}")
    check(part.sigma_prime == 24, f"YES: Sigma' should be 24, got {part.sigma_prime}")
    check(part.half == 12, f"YES: H should be 12, got {part.half}")

    # The proof says A = {3, 5} sums to 8 = T
    check(3 + 5 == T, "YES: 3+5 should equal T=8")

    # Check that the subset sum instance is feasible
    check(is_feasible_source(ss), "YES: source should be feasible")

    # Check partition feasibility
    check(is_feasible_target(part), "YES: target should be feasible")

    # Verify the specific partition from the proof:
    # config = [0, 0, 1, 1, 1, 0] (0 = side with A union {d})
    config = [0, 0, 1, 1, 1, 0]
    side0 = sum(part.elements[i] for i in range(6) if config[i] == 0)
    side1 = sum(part.elements[i] for i in range(6) if config[i] == 1)
    check(side0 == 12, f"YES: side 0 sum should be 12, got {side0}")
    check(side1 == 12, f"YES: side 1 sum should be 12, got {side1}")

    # Extraction from this config
    extracted = extract_solution(ss, config)
    ext_sum = sum(e * x for e, x in zip(ss.elements, extracted))
    check(ext_sum == T, f"YES: extracted sum should be 8, got {ext_sum}")

    # The extracted subset should be {3, 5} (indices 0, 1)
    check(extracted == [1, 1, 0, 0, 0], f"YES: extracted should be [1,1,0,0,0], got {extracted}")


# ---------------------------------------------------------------------------
# 3. NO example from Typst
# ---------------------------------------------------------------------------

def test_no_example():
    """Reproduce the NO example from the Typst proof."""
    S = [3, 7, 11]
    T = 5
    ss = SubsetSumInstance(S, T)

    check(ss.sigma == 21, f"NO: sigma should be 21, got {ss.sigma}")
    check(ss.sigma > 2 * T, f"NO: should have sigma > 2T")

    d = abs(ss.sigma - 2 * T)
    check(d == 11, f"NO: d should be 11, got {d}")

    part = reduce(ss)
    check(part.elements == [3, 7, 11, 11], f"NO: S' should be [3,7,11,11], got {part.elements}")
    check(part.sigma_prime == 32, f"NO: Sigma' should be 32, got {part.sigma_prime}")
    check(part.half == 16, f"NO: H should be 16, got {part.half}")

    # Source should be infeasible
    check(not is_feasible_source(ss), "NO: source should be infeasible")

    # Target should be infeasible
    check(not is_feasible_target(part), "NO: target should be infeasible")

    # Verify by listing all possible subset sums of {3, 7, 11}
    possible_sums = set()
    for mask in range(1 << 3):
        s = sum(S[i] for i in range(3) if mask & (1 << i))
        possible_sums.add(s)
    check(5 not in possible_sums, f"NO: 5 should not be achievable, sums = {possible_sums}")
    expected_sums = {0, 3, 7, 10, 11, 14, 18, 21}
    check(possible_sums == expected_sums, f"NO: possible sums should be {expected_sums}, got {possible_sums}")


# ---------------------------------------------------------------------------
# 4. Overhead formula verification
# ---------------------------------------------------------------------------

def test_overhead():
    """Verify overhead formula: |S'| = n+1 when d > 0, |S'| = n when d == 0."""
    count = 0
    for n in range(1, 6):
        for _ in range(200):
            import random
            elems = [random.randint(1, 20) for _ in range(n)]
            sigma = sum(elems)
            T = random.randint(0, sigma + 5)
            ss = SubsetSumInstance(elems, T)
            part = reduce(ss)
            d = abs(sigma - 2 * T)
            if d == 0:
                check(len(part.elements) == n,
                      f"Overhead: d=0, expected {n} elements, got {len(part.elements)}")
            else:
                check(len(part.elements) == n + 1,
                      f"Overhead: d>0, expected {n+1} elements, got {len(part.elements)}")
            count += 1
    print(f"  Overhead tests: {count} instances")


# ---------------------------------------------------------------------------
# 5. Case analysis tests
# ---------------------------------------------------------------------------

def test_case_analysis():
    """Test the three cases from the proof explicitly."""
    count = 0

    # Case 1: Sigma = 2T (d = 0)
    for _ in range(100):
        import random
        n = random.randint(2, 6)
        elems = [random.randint(1, 10) for _ in range(n)]
        sigma = sum(elems)
        if sigma % 2 != 0:
            elems[0] += 1
            sigma = sum(elems)
        T = sigma // 2
        ss = SubsetSumInstance(elems, T)
        part = reduce(ss)
        d = abs(sigma - 2 * T)
        check(d == 0, f"Case1: d should be 0")
        check(len(part.elements) == n, f"Case1: |S'| should be {n}")
        check(part.sigma_prime == sigma, f"Case1: Sigma' should be {sigma}")
        check(part.half == T, f"Case1: H should be {T}")
        # Equivalence
        check(is_feasible_source(ss) == is_feasible_target(part), "Case1: equivalence")
        count += 6

    # Case 2: Sigma > 2T (d = Sigma - 2T, H = Sigma - T)
    for _ in range(100):
        import random
        n = random.randint(2, 6)
        elems = [random.randint(1, 10) for _ in range(n)]
        sigma = sum(elems)
        T = random.randint(0, max(0, sigma // 2 - 1))
        ss = SubsetSumInstance(elems, T)
        if sigma <= 2 * T:
            continue
        part = reduce(ss)
        d = sigma - 2 * T
        check(d > 0, "Case2: d > 0")
        check(len(part.elements) == n + 1, f"Case2: |S'| should be {n+1}")
        expected_H = sigma - T
        check(part.sigma_prime == 2 * expected_H, f"Case2: Sigma' = 2H")
        check(part.half == expected_H, f"Case2: H = Sigma - T")
        check(is_feasible_source(ss) == is_feasible_target(part), "Case2: equivalence")
        count += 5

    # Case 3: Sigma < 2T (d = 2T - Sigma, H = T)
    for _ in range(100):
        import random
        n = random.randint(2, 6)
        elems = [random.randint(1, 10) for _ in range(n)]
        sigma = sum(elems)
        T = random.randint(sigma // 2 + 1, sigma)
        ss = SubsetSumInstance(elems, T)
        if sigma >= 2 * T:
            continue
        part = reduce(ss)
        d = 2 * T - sigma
        check(d > 0, "Case3: d > 0")
        check(len(part.elements) == n + 1, f"Case3: |S'| should be {n+1}")
        check(part.half == T, f"Case3: H = T")
        check(is_feasible_source(ss) == is_feasible_target(part), "Case3: equivalence")
        count += 4

    # Infeasible case: T > Sigma
    for _ in range(50):
        import random
        n = random.randint(1, 5)
        elems = [random.randint(1, 10) for _ in range(n)]
        sigma = sum(elems)
        T = sigma + random.randint(1, 10)
        ss = SubsetSumInstance(elems, T)
        part = reduce(ss)
        check(not is_feasible_source(ss), "Infeasible: source not feasible")
        check(not is_feasible_target(part), "Infeasible: target not feasible")
        count += 2

    print(f"  Case analysis tests: {count} checks")


# ---------------------------------------------------------------------------
# 6. Hypothesis property-based tests
# ---------------------------------------------------------------------------

hypothesis_checks = 0


@given(st.lists(st.integers(1, 20), min_size=1, max_size=8),
       st.integers(0, 100))
@settings(max_examples=1000, suppress_health_check=[HealthCheck.too_slow])
def test_hyp_equivalence(elements, target):
    """Property: source feasible iff target feasible."""
    global hypothesis_checks, passed, failed
    ss = SubsetSumInstance(elements, target)
    part = reduce(ss)
    src = is_feasible_source(ss)
    tgt = is_feasible_target(part)
    check(src == tgt,
          f"Hyp equiv: S={elements}, T={target}, src={src}, tgt={tgt}")
    hypothesis_checks += 1


@given(st.lists(st.integers(1, 15), min_size=2, max_size=7))
@settings(max_examples=1000, suppress_health_check=[HealthCheck.too_slow])
def test_hyp_roundtrip(elements):
    """Property: if source is feasible, extraction recovers a valid solution."""
    global hypothesis_checks, passed, failed
    # Pick target = sum of a random subset
    sigma = sum(elements)
    # Use first element as target for determinism
    T = elements[0]
    ss = SubsetSumInstance(elements, T)
    part = reduce(ss)
    if is_feasible_source(ss) and is_feasible_target(part):
        tgt_sol = find_target_solution(part)
        if tgt_sol is not None:
            extracted = extract_solution(ss, tgt_sol)
            ext_sum = sum(e * x for e, x in zip(ss.elements, extracted))
            check(ext_sum == T,
                  f"Hyp roundtrip: S={elements}, T={T}, ext_sum={ext_sum}")
    hypothesis_checks += 1


@given(st.lists(st.integers(1, 30), min_size=1, max_size=6),
       st.integers(1, 50))
@settings(max_examples=1000, suppress_health_check=[HealthCheck.too_slow])
def test_hyp_overhead(elements, target):
    """Property: overhead is at most n+1 elements."""
    global hypothesis_checks, passed, failed
    ss = SubsetSumInstance(elements, target)
    part = reduce(ss)
    n = len(elements)
    check(len(part.elements) <= n + 1,
          f"Hyp overhead: |S'|={len(part.elements)}, n+1={n+1}")
    d = abs(sum(elements) - 2 * target)
    if d == 0:
        check(len(part.elements) == n,
              f"Hyp overhead d=0: |S'|={len(part.elements)}, n={n}")
    else:
        check(len(part.elements) == n + 1,
              f"Hyp overhead d>0: |S'|={len(part.elements)}, n+1={n+1}")
    hypothesis_checks += 1


@given(st.lists(st.integers(1, 10), min_size=2, max_size=6))
@settings(max_examples=500, suppress_health_check=[HealthCheck.too_slow])
def test_hyp_all_valid_targets(elements):
    """For each achievable subset sum T, verify the full pipeline."""
    global hypothesis_checks, passed, failed
    sigma = sum(elements)
    n = len(elements)
    # Find all achievable subset sums
    achievable = set()
    for mask in range(1 << n):
        s = sum(elements[i] for i in range(n) if mask & (1 << i))
        achievable.add(s)
    # Test a few
    for T in list(achievable)[:5]:
        ss = SubsetSumInstance(elements, T)
        part = reduce(ss)
        check(is_feasible_target(part),
              f"Hyp all_valid: S={elements}, T={T} should be feasible")
        tgt_sol = find_target_solution(part)
        if tgt_sol is not None:
            extracted = extract_solution(ss, tgt_sol)
            ext_sum = sum(e * x for e, x in zip(ss.elements, extracted))
            check(ext_sum == T,
                  f"Hyp all_valid extraction: S={elements}, T={T}, ext_sum={ext_sum}")
    hypothesis_checks += 1


# ---------------------------------------------------------------------------
# 7. Edge cases
# ---------------------------------------------------------------------------

def test_edge_cases():
    """Test boundary conditions."""
    count = 0

    # Single element, target equals element
    ss = SubsetSumInstance([5], 5)
    part = reduce(ss)
    check(is_feasible_source(ss), "Edge: single elem, T=elem, feasible")
    check(is_feasible_target(part), "Edge: single elem partition feasible")
    count += 2

    # Single element, target = 0
    ss = SubsetSumInstance([5], 0)
    part = reduce(ss)
    check(is_feasible_source(ss), "Edge: T=0 always feasible (empty subset)")
    check(is_feasible_target(part), "Edge: T=0 partition feasible")
    count += 2

    # All elements equal
    ss = SubsetSumInstance([3, 3, 3, 3], 6)
    part = reduce(ss)
    check(is_feasible_source(ss), "Edge: all equal, T=6 feasible")
    check(is_feasible_target(part), "Edge: all equal partition feasible")
    count += 2

    # Target = sigma (pick all elements)
    ss = SubsetSumInstance([1, 2, 3], 6)
    part = reduce(ss)
    check(is_feasible_source(ss), "Edge: T=sigma feasible")
    check(is_feasible_target(part), "Edge: T=sigma partition feasible")
    count += 2

    # Target > sigma (infeasible)
    ss = SubsetSumInstance([1, 2, 3], 7)
    part = reduce(ss)
    check(not is_feasible_source(ss), "Edge: T>sigma infeasible")
    check(not is_feasible_target(part), "Edge: T>sigma partition infeasible")
    count += 2

    # Large padding (T >> sigma)
    ss = SubsetSumInstance([1, 1], 100)
    part = reduce(ss)
    check(not is_feasible_source(ss), "Edge: T>>sigma infeasible")
    check(not is_feasible_target(part), "Edge: T>>sigma partition infeasible")
    count += 2

    # T = sigma/2 exactly (Case 1)
    ss = SubsetSumInstance([2, 4, 6], 6)
    part = reduce(ss)
    d = abs(ss.sigma - 2 * ss.target)
    check(d == 0, "Edge: T=sigma/2, d=0")
    check(len(part.elements) == 3, "Edge: T=sigma/2, no padding")
    count += 2

    # Empty elements, T=0
    ss = SubsetSumInstance([], 0)
    part = reduce(ss)
    check(is_feasible_source(ss), "Edge: empty S, T=0 feasible")
    check(is_feasible_target(part), "Edge: empty partition feasible")
    count += 2

    # Empty elements, T>0
    ss = SubsetSumInstance([], 5)
    part = reduce(ss)
    check(not is_feasible_source(ss), "Edge: empty S, T=5 infeasible")
    check(not is_feasible_target(part), "Edge: empty S, T=5 partition infeasible")
    count += 2

    print(f"  Edge case tests: {count} checks")


# ---------------------------------------------------------------------------
# 8. Padding element correctness
# ---------------------------------------------------------------------------

def test_padding_values():
    """Verify the padding element d is computed correctly for all three cases."""
    import random
    count = 0
    for _ in range(500):
        n = random.randint(1, 6)
        elems = [random.randint(1, 15) for _ in range(n)]
        sigma = sum(elems)
        T = random.randint(0, sigma + 5)
        ss = SubsetSumInstance(elems, T)
        part = reduce(ss)
        d = abs(sigma - 2 * T)

        if sigma == 2 * T:
            # Case 1: no padding
            check(part.elements == elems, f"Padding Case1: S'=S")
            check(part.sigma_prime == sigma, f"Padding Case1: Sigma'=Sigma")
        elif sigma > 2 * T:
            # Case 2: d = sigma - 2T, H = sigma - T
            check(d == sigma - 2 * T, f"Padding Case2: d formula")
            check(part.elements == elems + [d], f"Padding Case2: S'=S+[d]")
            H = sigma - T
            check(part.sigma_prime == 2 * H, f"Padding Case2: Sigma'=2H")
        else:
            # Case 3: d = 2T - sigma, H = T
            check(d == 2 * T - sigma, f"Padding Case3: d formula")
            check(part.elements == elems + [d], f"Padding Case3: S'=S+[d]")
            check(part.sigma_prime == 2 * T, f"Padding Case3: Sigma'=2T")
        count += 3

    print(f"  Padding value tests: {count} checks")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    global passed, failed, bugs

    print("=" * 60)
    print("ADVERSARY: SubsetSum -> Partition verification")
    print("=" * 60)

    print("\n[1] Exhaustive tests (n <= 5)...")
    test_exhaustive()

    print("\n[2] YES example from Typst...")
    test_yes_example()

    print("\n[3] NO example from Typst...")
    test_no_example()

    print("\n[4] Overhead formula verification...")
    test_overhead()

    print("\n[5] Case analysis tests...")
    test_case_analysis()

    print("\n[6] Hypothesis property-based tests...")
    test_hyp_equivalence()
    test_hyp_roundtrip()
    test_hyp_overhead()
    test_hyp_all_valid_targets()

    print("\n[7] Edge cases...")
    test_edge_cases()

    print("\n[8] Padding value tests...")
    test_padding_values()

    total = passed + failed
    print("\n" + "=" * 60)
    print(f"ADVERSARY: SubsetSum -> Partition: {passed} passed, {failed} failed")
    if bugs:
        print(f"BUGS FOUND: {bugs[:20]}")  # Show first 20
    else:
        print("BUGS FOUND: none")
    print(f"Total checks: {total}")
    if total < 5000:
        print(f"WARNING: Only {total} checks, need >= 5000")

    return failed


if __name__ == "__main__":
    sys.exit(main())
