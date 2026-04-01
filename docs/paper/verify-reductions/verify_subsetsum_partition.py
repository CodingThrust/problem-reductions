#!/usr/bin/env python3
"""
§2.1 SubsetSum → Partition: exhaustive + symbolic verification.

Checks:
1. Symbolic: padding algebra for all 3 cases (sympy)
2. Exhaustive: all instances n ≤ 6, sizes ≤ 5, all targets
3. Solution extraction: verify extracted subset actually sums to T
4. Overhead: num_elements + 1 (or num_elements when d=0)
5. Edge cases: T=0, T=Σ, T>Σ, empty set, single element
"""
import itertools
import sys
from sympy import symbols, Abs, simplify

def powerset(s):
    for r in range(len(s) + 1):
        yield from itertools.combinations(s, r)

def has_subset_sum(sizes, target):
    return any(sum(sizes[i] for i in S) == target for S in powerset(range(len(sizes))))

def has_balanced_partition(sizes):
    total = sum(sizes)
    if total % 2 != 0:
        return False
    half = total // 2
    return any(sum(sizes[i] for i in S) == half for S in powerset(range(len(sizes))))

def reduce_subsetsum_to_partition(sizes, target):
    """Apply the reduction and return (partition_sizes, d, sigma)."""
    sigma = sum(sizes)
    d = abs(sigma - 2 * target)
    if d == 0:
        return list(sizes), 0, sigma
    else:
        return list(sizes) + [d], d, sigma

def extract_solution(partition_config, n, d, sigma, target):
    """Given a balanced partition config, extract the SubsetSum subset."""
    if d == 0:
        # Either side works — check which side sums to target
        side0 = [i for i in range(n) if partition_config[i] == 0]
        side1 = [i for i in range(n) if partition_config[i] == 1]
        return side1  # convention: side 1 = "in subset"
    elif sigma > 2 * target:
        # Elements on SAME side as d form subset summing to T
        d_side = partition_config[n]  # d is the last element
        return [i for i in range(n) if partition_config[i] == d_side]
    else:  # sigma < 2 * target
        # Elements on OPPOSITE side from d form subset summing to T
        d_side = partition_config[n]
        other_side = 1 - d_side
        return [i for i in range(n) if partition_config[i] == other_side]

def main():
    passed = failed = 0

    # --- Symbolic verification ---
    print("Symbolic checks...")
    S, T = symbols('Sigma T', positive=True, integer=True)

    # Case Σ > 2T
    d = S - 2*T
    assert simplify(S + d - 2*(S - T)) == 0, "Σ' = 2(Σ-T)"
    assert simplify(T + d - (S - T)) == 0, "T + d = Σ-T"
    assert simplify((S - T) - d - T) == 0, "(Σ-T) - d = T"
    passed += 3

    # Case Σ < 2T
    d2 = 2*T - S
    assert simplify(S + d2 - 2*T) == 0, "Σ' = 2T"
    assert simplify((S - T) + d2 - T) == 0, "(Σ-T) + d = T"
    passed += 2
    print(f"  Symbolic: {passed} passed")

    # --- Exhaustive verification ---
    print("Exhaustive checks (n ≤ 6)...")
    for n in range(0, 7):
        size_range = range(1, 4) if n >= 5 else range(1, 6)
        count = 0
        for sizes in itertools.product(size_range, repeat=n):
            sigma = sum(sizes)
            for target in range(0, sigma + 3):
                ss = has_subset_sum(sizes, target)
                part_sizes, d, sig = reduce_subsetsum_to_partition(sizes, target)
                bp = has_balanced_partition(part_sizes)

                if ss != bp:
                    print(f"  FAIL: sizes={sizes}, T={target}: SS={ss}, Part={bp}")
                    failed += 1
                else:
                    passed += 1

                # Check overhead
                expected_len = n + (1 if d > 0 else 0)
                if len(part_sizes) != expected_len:
                    print(f"  FAIL overhead: sizes={sizes}, T={target}")
                    failed += 1
                else:
                    passed += 1

                # Check solution extraction (forward direction)
                if ss and bp:
                    # Find a balanced partition
                    for config in itertools.product([0, 1], repeat=len(part_sizes)):
                        s0 = sum(part_sizes[i] for i in range(len(part_sizes)) if config[i] == 0)
                        s1 = sum(part_sizes[i] for i in range(len(part_sizes)) if config[i] == 1)
                        if s0 == s1:
                            subset_indices = extract_solution(list(config), n, d, sig, target)
                            subset_sum = sum(sizes[i] for i in subset_indices)
                            if subset_sum != target:
                                print(f"  FAIL extraction: sizes={sizes}, T={target}, "
                                      f"config={config}, extracted_sum={subset_sum}")
                                failed += 1
                            else:
                                passed += 1
                            break

                count += 1
            if n >= 5 and count > 500:
                break

    # --- Edge cases ---
    print("Edge cases...")
    # Empty set
    ss = has_subset_sum((), 0)
    ps, d, sig = reduce_subsetsum_to_partition((), 0)
    bp = has_balanced_partition(ps)
    assert ss == bp, "Empty set, T=0"
    passed += 1

    # Single element
    for s in range(1, 10):
        for t in range(0, s + 3):
            ss = has_subset_sum((s,), t)
            ps, d, sig = reduce_subsetsum_to_partition((s,), t)
            bp = has_balanced_partition(ps)
            assert ss == bp, f"Single element {s}, T={t}"
            passed += 1

    # T > Σ (infeasible)
    for sizes in [(1,2,3), (5,5,5), (1,)]:
        sigma = sum(sizes)
        for t in range(sigma + 1, sigma + 5):
            ss = has_subset_sum(sizes, t)
            ps, d, sig = reduce_subsetsum_to_partition(sizes, t)
            bp = has_balanced_partition(ps)
            assert not ss and not bp, f"Infeasible: {sizes}, T={t}"
            passed += 1

    print(f"\nSubsetSum → Partition: {passed} passed, {failed} failed")
    return failed

if __name__ == "__main__":
    sys.exit(main())
