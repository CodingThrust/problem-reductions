#!/usr/bin/env python3
"""
Verification script: ExactCoverBy3Sets -> SubsetProduct reduction.
Issue: #388
Reference: Garey & Johnson, Computers and Intractability, SP14, p.224.

Seven mandatory sections:
  1. reduce()         -- the reduction function
  2. extract()        -- solution extraction (back-map)
  3. Brute-force solvers for source and target
  4. Forward: YES source -> YES target
  5. Backward: YES target -> YES source (via extract)
  6. Infeasible: NO source -> NO target
  7. Overhead check

Runs >=5000 checks total, with exhaustive coverage for small instances.
"""

import json
import math
import sys
from itertools import combinations, product
from typing import Optional

# -----------------------------------------------------------------------
# Helper: prime generation
# -----------------------------------------------------------------------

def nth_primes(n: int) -> list[int]:
    """Return the first n primes."""
    if n == 0:
        return []
    primes = []
    candidate = 2
    while len(primes) < n:
        if all(candidate % p != 0 for p in primes):
            primes.append(candidate)
        candidate += 1
    return primes


# -----------------------------------------------------------------------
# Section 1: reduce()
# -----------------------------------------------------------------------

def reduce(universe_size: int, subsets: list[list[int]]) -> tuple[list[int], int]:
    """
    Reduce X3C(universe_size, subsets) -> SubsetProduct(sizes, target).

    Construction (Garey & Johnson SP14):
    - Assign the i-th prime p_i to each element i in the universe.
    - For each subset {a, b, c}, define size = p_a * p_b * p_c.
    - Target B = product of the first universe_size primes.

    Returns (sizes, target).
    """
    primes = nth_primes(universe_size)
    sizes = []
    for subset in subsets:
        s = 1
        for elem in subset:
            s *= primes[elem]
        sizes.append(s)
    target = 1
    for p in primes:
        target *= p
    return sizes, target


# -----------------------------------------------------------------------
# Section 2: extract()
# -----------------------------------------------------------------------

def extract(
    universe_size: int,
    subsets: list[list[int]],
    sp_config: list[int],
) -> list[int]:
    """
    Extract an X3C solution from a SubsetProduct solution.
    The mapping is identity: the same binary selection vector applies
    because there is a 1-to-1 correspondence between X3C subsets
    and SubsetProduct elements.
    """
    return list(sp_config[:len(subsets)])


# -----------------------------------------------------------------------
# Section 3: Brute-force solvers
# -----------------------------------------------------------------------

def solve_x3c(universe_size: int, subsets: list[list[int]]) -> Optional[list[int]]:
    """Brute-force solve X3C. Returns config or None."""
    n = len(subsets)
    q = universe_size // 3
    for config in product(range(2), repeat=n):
        if sum(config) != q:
            continue
        covered = set()
        ok = True
        for i, sel in enumerate(config):
            if sel == 1:
                for elem in subsets[i]:
                    if elem in covered:
                        ok = False
                        break
                    covered.add(elem)
                if not ok:
                    break
        if ok and len(covered) == universe_size:
            return list(config)
    return None


def is_x3c_feasible(universe_size: int, subsets: list[list[int]]) -> bool:
    return solve_x3c(universe_size, subsets) is not None


def solve_subset_product(sizes: list[int], target: int) -> Optional[list[int]]:
    """Brute-force solve SubsetProduct. Returns config or None."""
    n = len(sizes)
    for config in product(range(2), repeat=n):
        prod = 1
        for i, sel in enumerate(config):
            if sel == 1:
                prod *= sizes[i]
                if prod > target:
                    break
        if prod == target:
            return list(config)
    return None


def is_sp_feasible(sizes: list[int], target: int) -> bool:
    return solve_subset_product(sizes, target) is not None


# -----------------------------------------------------------------------
# Section 4: Forward check -- YES source -> YES target
# -----------------------------------------------------------------------

def check_forward(universe_size: int, subsets: list[list[int]]) -> bool:
    """
    If X3C(universe_size, subsets) is feasible,
    then SubsetProduct(reduce(...)) must also be feasible.
    """
    if not is_x3c_feasible(universe_size, subsets):
        return True  # vacuously true
    sizes, target = reduce(universe_size, subsets)
    return is_sp_feasible(sizes, target)


# -----------------------------------------------------------------------
# Section 5: Backward check -- YES target -> YES source (via extract)
# -----------------------------------------------------------------------

def check_backward(universe_size: int, subsets: list[list[int]]) -> bool:
    """
    If SubsetProduct(reduce(...)) is feasible,
    solve it, extract an X3C config, and verify it.
    """
    sizes, target = reduce(universe_size, subsets)
    sp_sol = solve_subset_product(sizes, target)
    if sp_sol is None:
        return True  # vacuously true
    source_config = extract(universe_size, subsets, sp_sol)
    # Verify the extracted solution is a valid exact cover
    q = universe_size // 3
    if sum(source_config) != q:
        return False
    covered = set()
    for i, sel in enumerate(source_config):
        if sel == 1:
            for elem in subsets[i]:
                if elem in covered:
                    return False
                covered.add(elem)
    return len(covered) == universe_size


# -----------------------------------------------------------------------
# Section 6: Infeasible check -- NO source -> NO target
# -----------------------------------------------------------------------

def check_infeasible(universe_size: int, subsets: list[list[int]]) -> bool:
    """
    If X3C(universe_size, subsets) is infeasible,
    then SubsetProduct(reduce(...)) must also be infeasible.
    """
    if is_x3c_feasible(universe_size, subsets):
        return True  # not an infeasible instance; skip
    sizes, target = reduce(universe_size, subsets)
    return not is_sp_feasible(sizes, target)


# -----------------------------------------------------------------------
# Section 7: Overhead check
# -----------------------------------------------------------------------

def check_overhead(universe_size: int, subsets: list[list[int]]) -> bool:
    """
    Verify: len(sizes) == len(subsets) and target == product of first
    universe_size primes.
    """
    sizes, target = reduce(universe_size, subsets)
    if len(sizes) != len(subsets):
        return False
    primes = nth_primes(universe_size)
    expected_target = 1
    for p in primes:
        expected_target *= p
    if target != expected_target:
        return False
    # Each size must be a product of exactly 3 primes from the list
    prime_set = set(primes)
    for i, s in enumerate(sizes):
        expected_s = 1
        for elem in subsets[i]:
            expected_s *= primes[elem]
        if s != expected_s:
            return False
    return True


# -----------------------------------------------------------------------
# Exhaustive + random test driver
# -----------------------------------------------------------------------

def exhaustive_tests() -> int:
    """
    Exhaustive tests for small X3C instances.
    universe_size=3: all possible subset collections (up to 4 subsets).
    universe_size=6: all possible collections up to 5 subsets.
    Returns number of checks performed.
    """
    checks = 0

    # universe_size=3: only possible triple is [0,1,2]
    all_triples_3 = [[0, 1, 2]]
    for num_sub in range(1, 3):
        for chosen in combinations(all_triples_3 * 2, num_sub):
            subsets = [list(t) for t in chosen]
            # deduplicate
            seen = set()
            unique = []
            for s in subsets:
                key = tuple(s)
                if key not in seen:
                    seen.add(key)
                    unique.append(s)
            subsets = unique

            assert check_forward(3, subsets), f"Forward FAIL: u=3, {subsets}"
            assert check_backward(3, subsets), f"Backward FAIL: u=3, {subsets}"
            assert check_infeasible(3, subsets), f"Infeasible FAIL: u=3, {subsets}"
            assert check_overhead(3, subsets), f"Overhead FAIL: u=3, {subsets}"
            checks += 4

    # universe_size=6: all triples from {0..5}
    all_triples_6 = [list(t) for t in combinations(range(6), 3)]
    for num_sub in range(1, 7):
        for chosen in combinations(all_triples_6, num_sub):
            subsets = [list(t) for t in chosen]
            assert check_forward(6, subsets), f"Forward FAIL: u=6, {subsets}"
            assert check_backward(6, subsets), f"Backward FAIL: u=6, {subsets}"
            assert check_infeasible(6, subsets), f"Infeasible FAIL: u=6, {subsets}"
            assert check_overhead(6, subsets), f"Overhead FAIL: u=6, {subsets}"
            checks += 4

    return checks


def random_tests(count: int = 2000, max_u_mult: int = 4) -> int:
    """Random tests with larger instances. Returns number of checks."""
    import random
    rng = random.Random(42)
    checks = 0
    for _ in range(count):
        q = rng.randint(1, max_u_mult)
        universe_size = 3 * q
        elems = list(range(universe_size))
        num_sub = rng.randint(1, min(8, len(list(combinations(elems, 3)))))
        subsets = [sorted(rng.sample(elems, 3)) for _ in range(num_sub)]
        # Deduplicate
        seen = set()
        unique = []
        for s in subsets:
            key = tuple(s)
            if key not in seen:
                seen.add(key)
                unique.append(s)
        subsets = unique

        assert check_forward(universe_size, subsets), (
            f"Forward FAIL: u={universe_size}, {subsets}"
        )
        assert check_backward(universe_size, subsets), (
            f"Backward FAIL: u={universe_size}, {subsets}"
        )
        assert check_infeasible(universe_size, subsets), (
            f"Infeasible FAIL: u={universe_size}, {subsets}"
        )
        assert check_overhead(universe_size, subsets), (
            f"Overhead FAIL: u={universe_size}, {subsets}"
        )
        checks += 4
    return checks


def collect_test_vectors(count: int = 20) -> list[dict]:
    """Collect representative test vectors for downstream consumption."""
    import random
    rng = random.Random(123)
    vectors = []

    # Hand-crafted vectors
    hand_crafted = [
        {
            "universe_size": 9,
            "subsets": [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6]],
            "label": "yes_disjoint_cover",
        },
        {
            "universe_size": 9,
            "subsets": [[0, 1, 2], [0, 3, 4], [0, 5, 6], [3, 7, 8]],
            "label": "no_overlapping_element_0",
        },
        {
            "universe_size": 3,
            "subsets": [[0, 1, 2]],
            "label": "yes_minimal_trivial",
        },
        {
            "universe_size": 6,
            "subsets": [[0, 1, 2], [3, 4, 5]],
            "label": "yes_two_disjoint",
        },
        {
            "universe_size": 6,
            "subsets": [[0, 1, 2], [0, 3, 4], [1, 3, 5]],
            "label": "no_all_overlap",
        },
        {
            "universe_size": 6,
            "subsets": [[0, 1, 2], [2, 3, 4], [4, 5, 0]],
            "label": "no_cyclic_overlap",
        },
        {
            "universe_size": 6,
            "subsets": [[0, 1, 2], [3, 4, 5], [0, 3, 4], [1, 2, 5]],
            "label": "yes_multiple_covers",
        },
        {
            "universe_size": 9,
            "subsets": [[0, 1, 2], [3, 4, 5], [6, 7, 8]],
            "label": "yes_exact_3_subsets",
        },
    ]

    for hc in hand_crafted:
        u = hc["universe_size"]
        subs = hc["subsets"]
        sizes, target = reduce(u, subs)
        src_sol = solve_x3c(u, subs)
        sp_sol = solve_subset_product(sizes, target)
        extracted = None
        if sp_sol is not None:
            extracted = extract(u, subs, sp_sol)
        vectors.append({
            "label": hc["label"],
            "source": {"universe_size": u, "subsets": subs},
            "target": {"sizes": sizes, "target": target},
            "source_feasible": src_sol is not None,
            "target_feasible": sp_sol is not None,
            "source_solution": src_sol,
            "target_solution": sp_sol,
            "extracted_solution": extracted,
        })

    # Random vectors
    for i in range(count - len(hand_crafted)):
        q = rng.randint(1, 3)
        u = 3 * q
        elems = list(range(u))
        ns = rng.randint(1, min(6, len(list(combinations(elems, 3)))))
        subs = [sorted(rng.sample(elems, 3)) for _ in range(ns)]
        # Deduplicate
        seen = set()
        unique = []
        for s in subs:
            key = tuple(s)
            if key not in seen:
                seen.add(key)
                unique.append(s)
        subs = unique

        sizes, target = reduce(u, subs)
        src_sol = solve_x3c(u, subs)
        sp_sol = solve_subset_product(sizes, target)
        extracted = None
        if sp_sol is not None:
            extracted = extract(u, subs, sp_sol)
        vectors.append({
            "label": f"random_{i}",
            "source": {"universe_size": u, "subsets": subs},
            "target": {"sizes": sizes, "target": target},
            "source_feasible": src_sol is not None,
            "target_feasible": sp_sol is not None,
            "source_solution": src_sol,
            "target_solution": sp_sol,
            "extracted_solution": extracted,
        })

    return vectors


if __name__ == "__main__":
    print("=" * 60)
    print("ExactCoverBy3Sets -> SubsetProduct verification")
    print("=" * 60)

    print("\n[1/3] Exhaustive tests...")
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
        u = v["source"]["universe_size"]
        subs = v["source"]["subsets"]
        if v["source_feasible"]:
            assert v["target_feasible"], f"Forward violation in {v['label']}"
            if v["extracted_solution"] is not None:
                # Verify extraction
                q = u // 3
                assert sum(v["extracted_solution"]) == q, (
                    f"Wrong number of selected subsets in {v['label']}"
                )
        if not v["source_feasible"]:
            assert not v["target_feasible"], f"Infeasible violation in {v['label']}"

    # Write test vectors
    out_path = "docs/paper/verify-reductions/test_vectors_exact_cover_by_3_sets_subset_product.json"
    with open(out_path, "w") as f:
        json.dump({"vectors": vectors, "total_checks": total}, f, indent=2)
    print(f"  Wrote {len(vectors)} test vectors to {out_path}")

    print(f"\nAll {total} checks PASSED.")
