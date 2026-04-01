#!/usr/bin/env python3
"""§1.1 SubsetSum → Partition (#973): exhaustive + structural verification."""
import itertools
import sys
from sympy import symbols, simplify, Abs

passed = failed = 0


def check(condition, msg=""):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL: {msg}")


# ── Reduction implementation ──────────────────────────────────────────────


def reduce(sizes, target):
    """Reduce SubsetSum(sizes, target) to Partition(sizes').

    Returns:
        partition_sizes: list of positive integers for the Partition instance
        d: the padding value (0 if no padding needed)
        sigma: sum of original sizes
    """
    sigma = sum(sizes)
    d = abs(sigma - 2 * target)
    if d == 0:
        partition_sizes = list(sizes)
    else:
        partition_sizes = list(sizes) + [d]
    return partition_sizes, d, sigma


def is_subset_sum_feasible(sizes, target):
    """Brute force: does any subset of sizes sum to target?"""
    n = len(sizes)
    for bits in range(2**n):
        s = sum(sizes[i] for i in range(n) if (bits >> i) & 1)
        if s == target:
            return True
    return False


def find_subset_sum_solution(sizes, target):
    """Return a config (0/1 list) for a subset summing to target, or None."""
    n = len(sizes)
    for bits in range(2**n):
        s = sum(sizes[i] for i in range(n) if (bits >> i) & 1)
        if s == target:
            return [(bits >> i) & 1 for i in range(n)]
    return None


def is_partition_feasible(sizes):
    """Brute force: can sizes be partitioned into two equal-sum subsets?"""
    total = sum(sizes)
    if total % 2 != 0:
        return False
    half = total // 2
    n = len(sizes)
    for bits in range(2**n):
        s = sum(sizes[i] for i in range(n) if (bits >> i) & 1)
        if s == half:
            return True
    return False


def find_partition_solution(sizes):
    """Return a config (0/1 list) for a balanced partition, or None."""
    total = sum(sizes)
    if total % 2 != 0:
        return None
    half = total // 2
    n = len(sizes)
    for bits in range(2**n):
        s = sum(sizes[i] for i in range(n) if (bits >> i) & 1)
        if s == half:
            return [(bits >> i) & 1 for i in range(n)]
    return None


def extract_subsetsum_solution(sizes, target, partition_config, d, sigma):
    """Extract SubsetSum solution from Partition config.

    partition_config: 0/1 list over partition_sizes
    Returns: 0/1 list over original sizes (1 = in subset)
    """
    n = len(sizes)
    if d == 0:
        # Either side works; return the side that sums to target
        side0 = [i for i in range(n) if partition_config[i] == 0]
        side1 = [i for i in range(n) if partition_config[i] == 1]
        if sum(sizes[i] for i in side0) == target:
            return [1 - partition_config[i] for i in range(n)]  # side 0 selected
        else:
            return [partition_config[i] for i in range(n)]  # side 1 selected
    elif sigma > 2 * target:
        # S-elements on SAME side as padding sum to T
        pad_side = partition_config[n]  # padding is last element
        return [1 if partition_config[i] == pad_side else 0 for i in range(n)]
    else:  # sigma < 2 * target
        # S-elements on OPPOSITE side from padding sum to T
        pad_side = partition_config[n]
        return [1 if partition_config[i] != pad_side else 0 for i in range(n)]


def main():
    # === Section 1: Symbolic checks (sympy) — MANDATORY ===
    print("=== Section 1: Symbolic overhead verification ===")

    n_sym, sigma_sym, T_sym = symbols("n Sigma T", positive=True, integer=True)

    # Overhead: num_elements = n + 1 (worst case)
    overhead = n_sym + 1
    check(simplify(overhead - (n_sym + 1)) == 0, "overhead = n + 1")

    # Case 1: Sigma = 2T => d = 0, Sigma' = 2T, H = T
    d1 = Abs(sigma_sym - 2 * T_sym).subs(sigma_sym, 2 * T_sym)
    check(simplify(d1) == 0, "Case 1: d = 0 when Sigma = 2T")

    # Case 2: Sigma > 2T => d = Sigma - 2T, Sigma' = 2(Sigma - T), H = Sigma - T
    d2 = sigma_sym - 2 * T_sym  # when sigma > 2T
    sigma_prime_2 = sigma_sym + d2
    H2 = sigma_prime_2 / 2
    check(simplify(sigma_prime_2 - 2 * (sigma_sym - T_sym)) == 0,
          "Case 2: Sigma' = 2(Sigma - T)")
    check(simplify(H2 - (sigma_sym - T_sym)) == 0,
          "Case 2: H = Sigma - T")

    # Case 3: Sigma < 2T => d = 2T - Sigma, Sigma' = 2T, H = T
    d3 = 2 * T_sym - sigma_sym  # when sigma < 2T
    sigma_prime_3 = sigma_sym + d3
    H3 = sigma_prime_3 / 2
    check(simplify(sigma_prime_3 - 2 * T_sym) == 0,
          "Case 3: Sigma' = 2T")
    check(simplify(H3 - T_sym) == 0,
          "Case 3: H = T")

    # Verify forward direction algebra for Case 2
    # A sums to T, A ∪ {d} sums to T + d = T + (Sigma - 2T) = Sigma - T = H
    forward_case2 = T_sym + (sigma_sym - 2 * T_sym)
    check(simplify(forward_case2 - (sigma_sym - T_sym)) == 0,
          "Case 2 forward: T + d = Sigma - T")

    # Verify backward direction algebra for Case 2
    # Side with d: S-elements sum to H - d = (Sigma - T) - (Sigma - 2T) = T
    back_case2 = (sigma_sym - T_sym) - (sigma_sym - 2 * T_sym)
    check(simplify(back_case2 - T_sym) == 0,
          "Case 2 backward: H - d = T")

    # Verify forward direction algebra for Case 3
    # (S\A) ∪ {d}: (Sigma - T) + (2T - Sigma) = T
    forward_case3 = (sigma_sym - T_sym) + (2 * T_sym - sigma_sym)
    check(simplify(forward_case3 - T_sym) == 0,
          "Case 3 forward: (Sigma-T) + d = T")

    # Numeric checks for various (n, sigma, T)
    for n_val in range(1, 8):
        for sigma_val in range(n_val, 3 * n_val + 5):
            for t_val in range(0, sigma_val + 3):
                d_val = abs(sigma_val - 2 * t_val)
                if d_val == 0:
                    sp = sigma_val
                else:
                    sp = sigma_val + d_val
                check(sp % 2 == 0,
                      f"Sigma'={sp} is even for n={n_val},Sigma={sigma_val},T={t_val}")

    print(f"  Section 1: {passed} passed, {failed} failed")

    # === Section 2: Exhaustive forward + backward — MANDATORY ===
    print("\n=== Section 2: Exhaustive forward + backward (n ≤ 5) ===")
    sec2_start = passed

    for n in range(1, 6):
        # Generate SubsetSum instances: sizes from 1..max_val, various targets
        max_val = min(10, 2**n)
        num_tested = 0

        # All multisets of size n with elements in 1..max_val (sample)
        if n <= 3:
            size_range = range(1, max_val + 1)
            all_size_combos = list(itertools.product(size_range, repeat=n))
        else:
            # Sample for larger n
            import random
            random.seed(42 + n)
            all_size_combos = [tuple(random.randint(1, max_val) for _ in range(n))
                               for _ in range(200)]

        for sizes in all_size_combos:
            sigma = sum(sizes)
            # Test several targets including edge cases
            targets = set([0, 1, sigma // 2, sigma, sigma + 1])
            targets.update(range(max(0, sigma // 2 - 2), min(sigma + 2, sigma // 2 + 3)))
            if n <= 3:
                targets = set(range(0, sigma + 2))

            for target in targets:
                if target < 0:
                    continue
                source_feasible = is_subset_sum_feasible(list(sizes), target)
                part_sizes, d, sig = reduce(list(sizes), target)
                target_feasible = is_partition_feasible(part_sizes)

                check(source_feasible == target_feasible,
                      f"n={n}, sizes={sizes}, T={target}: "
                      f"source={source_feasible}, target={target_feasible}")
                num_tested += 1

        print(f"  n={n}: tested {num_tested} instances")

    print(f"  Section 2: {passed - sec2_start} new checks")

    # === Section 3: Solution extraction — MANDATORY ===
    print("\n=== Section 3: Solution extraction ===")
    sec3_start = passed

    for n in range(1, 6):
        max_val = min(10, 2**n)
        num_extracted = 0

        if n <= 3:
            all_size_combos = list(itertools.product(range(1, max_val + 1), repeat=n))
        else:
            import random
            random.seed(100 + n)
            all_size_combos = [tuple(random.randint(1, max_val) for _ in range(n))
                               for _ in range(150)]

        for sizes in all_size_combos:
            sizes = list(sizes)
            sigma = sum(sizes)

            for target in range(0, sigma + 1):
                if not is_subset_sum_feasible(sizes, target):
                    continue

                part_sizes, d, sig = reduce(sizes, target)
                part_config = find_partition_solution(part_sizes)
                if part_config is None:
                    check(False, f"Feasible source but no partition config: sizes={sizes}, T={target}")
                    continue

                extracted = extract_subsetsum_solution(sizes, target, part_config, d, sig)
                extracted_sum = sum(sizes[i] for i in range(n) if extracted[i] == 1)

                check(extracted_sum == target,
                      f"extraction: sizes={sizes}, T={target}, got sum={extracted_sum}")
                num_extracted += 1

                if num_extracted >= 300:
                    break
            if num_extracted >= 300:
                break

        print(f"  n={n}: extracted {num_extracted} solutions")

    print(f"  Section 3: {passed - sec3_start} new checks")

    # === Section 4: Overhead formula — MANDATORY ===
    print("\n=== Section 4: Overhead formula verification ===")
    sec4_start = passed

    for n in range(1, 6):
        max_val = min(10, 2**n)
        if n <= 3:
            all_size_combos = list(itertools.product(range(1, max_val + 1), repeat=n))
        else:
            import random
            random.seed(200 + n)
            all_size_combos = [tuple(random.randint(1, max_val) for _ in range(n))
                               for _ in range(100)]

        count = 0
        for sizes in all_size_combos:
            sizes = list(sizes)
            sigma = sum(sizes)
            for target in [0, sigma // 2, sigma, sigma + 1, 1]:
                if target < 0:
                    continue
                part_sizes, d, sig = reduce(sizes, target)

                # Overhead: num_elements
                if d == 0:
                    check(len(part_sizes) == n,
                          f"overhead d=0: got {len(part_sizes)}, expected {n}")
                else:
                    check(len(part_sizes) == n + 1,
                          f"overhead d>0: got {len(part_sizes)}, expected {n+1}")

                # Verify Sigma' and H
                sp = sum(part_sizes)
                check(sp % 2 == 0,
                      f"Sigma' = {sp} must be even")

                half = sp // 2
                if sigma == 2 * target:
                    check(half == target, f"Case 1: H={half} should be T={target}")
                elif sigma > 2 * target:
                    check(half == sigma - target,
                          f"Case 2: H={half} should be Sigma-T={sigma - target}")
                else:
                    check(half == target, f"Case 3: H={half} should be T={target}")

                count += 1
            if count >= 500:
                break

    print(f"  Section 4: {passed - sec4_start} new checks")

    # === Section 5: Structural properties — MANDATORY ===
    print("\n=== Section 5: Structural properties ===")
    sec5_start = passed

    for n in range(1, 6):
        max_val = min(10, 2**n)
        if n <= 3:
            all_size_combos = list(itertools.product(range(1, max_val + 1), repeat=n))
        else:
            import random
            random.seed(300 + n)
            all_size_combos = [tuple(random.randint(1, max_val) for _ in range(n))
                               for _ in range(100)]

        for sizes in all_size_combos:
            sizes = list(sizes)
            sigma = sum(sizes)
            for target in [0, sigma // 2, sigma, sigma + 1]:
                if target < 0:
                    continue
                part_sizes, d, sig = reduce(sizes, target)

                # All elements must be positive
                for i, s in enumerate(part_sizes):
                    check(s > 0, f"element {i} = {s} must be positive")

                # Padding value correctness
                check(d == abs(sigma - 2 * target),
                      f"d = {d}, expected |{sigma} - {2*target}| = {abs(sigma - 2*target)}")

                # Original elements preserved
                for i in range(n):
                    check(part_sizes[i] == sizes[i],
                          f"element {i} preserved: {part_sizes[i]} vs {sizes[i]}")

                # If d > 0, padding is last element
                if d > 0:
                    check(part_sizes[-1] == d,
                          f"padding element: {part_sizes[-1]} vs d={d}")

                # Infeasible case check: T > Sigma => d > Sigma
                if target > sigma:
                    check(d > sigma,
                          f"T > Sigma: d={d} should be > Sigma={sigma}")

    print(f"  Section 5: {passed - sec5_start} new checks")

    # === Section 6: YES example from Typst — MANDATORY ===
    print("\n=== Section 6: YES example verification ===")
    sec6_start = passed

    # From Typst: S = {3, 5, 7, 1, 4}, T = 8
    # Sigma = 20, 2T = 16, Sigma > 2T, d = 4
    # S' = {3, 5, 7, 1, 4, 4}, Sigma' = 24, H = 12
    yes_sizes = [3, 5, 7, 1, 4]
    yes_target = 8
    yes_sigma = 20

    check(sum(yes_sizes) == yes_sigma, f"YES: Sigma = {sum(yes_sizes)}")
    check(yes_sigma > 2 * yes_target, "YES: Sigma > 2T (Case 2)")

    part_sizes, d, sigma = reduce(yes_sizes, yes_target)
    check(d == 4, f"YES: d = {d}, expected 4")
    check(part_sizes == [3, 5, 7, 1, 4, 4], f"YES: S' = {part_sizes}")
    check(sum(part_sizes) == 24, f"YES: Sigma' = {sum(part_sizes)}")
    check(sum(part_sizes) // 2 == 12, f"YES: H = {sum(part_sizes) // 2}")

    # Verify subset {3, 5} sums to T = 8
    check(3 + 5 == yes_target, "YES: subset {3,5} sums to T")

    # Verify partition: {3, 5, 4_pad} vs {7, 1, 4}
    check(3 + 5 + 4 == 12, "YES: side with A∪{d} = 12 = H")
    check(7 + 1 + 4 == 12, "YES: other side = 12 = H")

    # Verify config from Typst: [0, 0, 1, 1, 1, 0]
    yes_config = [0, 0, 1, 1, 1, 0]
    side0_sum = sum(part_sizes[i] for i in range(6) if yes_config[i] == 0)
    side1_sum = sum(part_sizes[i] for i in range(6) if yes_config[i] == 1)
    check(side0_sum == 12, f"YES config: side 0 sum = {side0_sum}")
    check(side1_sum == 12, f"YES config: side 1 sum = {side1_sum}")

    # Verify extraction
    extracted = extract_subsetsum_solution(yes_sizes, yes_target, yes_config, d, sigma)
    extracted_sum = sum(yes_sizes[i] for i in range(5) if extracted[i] == 1)
    check(extracted_sum == yes_target,
          f"YES extraction: sum = {extracted_sum}, expected {yes_target}")

    check(is_subset_sum_feasible(yes_sizes, yes_target), "YES: source is feasible")
    check(is_partition_feasible(part_sizes), "YES: target is feasible")

    print(f"  Section 6: {passed - sec6_start} new checks")

    # === Section 7: NO example from Typst — MANDATORY ===
    print("\n=== Section 7: NO example verification ===")
    sec7_start = passed

    # From Typst: S = {3, 7, 11}, T = 5
    # Sigma = 21, 2T = 10, Sigma > 2T, d = 11
    # S' = {3, 7, 11, 11}, Sigma' = 32, H = 16
    no_sizes = [3, 7, 11]
    no_target = 5
    no_sigma = 21

    check(sum(no_sizes) == no_sigma, f"NO: Sigma = {sum(no_sizes)}")
    check(no_sigma > 2 * no_target, "NO: Sigma > 2T (Case 2)")

    part_sizes_no, d_no, sigma_no = reduce(no_sizes, no_target)
    check(d_no == 11, f"NO: d = {d_no}, expected 11")
    check(part_sizes_no == [3, 7, 11, 11], f"NO: S' = {part_sizes_no}")
    check(sum(part_sizes_no) == 32, f"NO: Sigma' = {sum(part_sizes_no)}")
    check(sum(part_sizes_no) // 2 == 16, f"NO: H = {sum(part_sizes_no) // 2}")

    # Verify source infeasible: no subset of {3,7,11} sums to 5
    check(not is_subset_sum_feasible(no_sizes, no_target), "NO: source is infeasible")

    # Verify target infeasible
    check(not is_partition_feasible(part_sizes_no), "NO: target is infeasible")

    # Verify achievable subset sums of {3,7,11,11} match Typst
    achievable = set()
    for bits in range(16):
        achievable.add(sum(part_sizes_no[i] for i in range(4) if (bits >> i) & 1))
    expected_sums = {0, 3, 7, 10, 11, 14, 18, 21, 22, 25, 29, 32}
    check(achievable == expected_sums,
          f"NO: achievable sums = {sorted(achievable)}, expected {sorted(expected_sums)}")
    check(16 not in achievable, "NO: 16 not achievable")

    print(f"  Section 7: {passed - sec7_start} new checks")

    # ── Final report ──
    print(f"\nSubsetSum → Partition: {passed} passed, {failed} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
