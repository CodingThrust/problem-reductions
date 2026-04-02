#!/usr/bin/env python3
"""
Adversary verification script for ExactCoverBy3Sets -> SubsetProduct.
Issue #388.

Independent implementation based ONLY on the Typst proof.
Does NOT import from the constructor script.

Requirements:
- Own reduce() function
- Own extract_solution() function
- Own is_feasible_source() and is_feasible_target() validators
- Exhaustive forward + backward for small instances
- hypothesis PBT (>= 2 strategies)
- Reproduce both Typst examples (YES and NO)
- >= 5,000 total checks
"""

import itertools
import json
import os
import random
import sys

# ---------------------------------------------------------------------------
# Independent prime generation
# ---------------------------------------------------------------------------

def sieve_primes(n: int) -> list[int]:
    """Return the first n primes using trial division (independent impl)."""
    if n == 0:
        return []
    result = []
    c = 2
    while len(result) < n:
        is_prime = True
        for p in result:
            if p * p > c:
                break
            if c % p == 0:
                is_prime = False
                break
        if is_prime:
            result.append(c)
        c += 1
    return result


# ---------------------------------------------------------------------------
# Independent reduction implementation (from Typst proof only)
# ---------------------------------------------------------------------------

def reduce(universe_size: int, subsets: list[list[int]]) -> tuple[list[int], int]:
    """
    From the Typst proof:
    - Assign prime p_i to element i (p_0=2, p_1=3, p_2=5, ...)
    - For each subset {a,b,c}: size = p_a * p_b * p_c
    - Target B = product of all primes p_0 ... p_{3q-1}
    """
    primes = sieve_primes(universe_size)
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


def is_feasible_source(universe_size: int, subsets: list[list[int]], config: list[int]) -> bool:
    """Check if config selects a valid exact cover."""
    if len(config) != len(subsets):
        return False
    q = universe_size // 3
    if sum(config) != q:
        return False
    covered = set()
    for idx in range(len(config)):
        if config[idx] == 1:
            for elem in subsets[idx]:
                if elem in covered:
                    return False
                covered.add(elem)
    return covered == set(range(universe_size))


def is_feasible_target(sizes: list[int], target: int, config: list[int]) -> bool:
    """Check if config selects a subset whose product equals target."""
    if len(config) != len(sizes):
        return False
    prod = 1
    for i, sel in enumerate(config):
        if sel == 1:
            prod *= sizes[i]
            if prod > target:
                return False
    return prod == target


def extract_solution(config: list[int]) -> list[int]:
    """Extract X3C config from SubsetProduct config. Identity per Typst proof."""
    return list(config)


# ---------------------------------------------------------------------------
# Brute force solvers
# ---------------------------------------------------------------------------

def all_x3c_solutions(universe_size: int, subsets: list[list[int]]) -> list[list[int]]:
    """Find all exact covers."""
    n = len(subsets)
    sols = []
    for bits in itertools.product([0, 1], repeat=n):
        if is_feasible_source(universe_size, subsets, list(bits)):
            sols.append(list(bits))
    return sols


def all_sp_solutions(sizes: list[int], target: int) -> list[list[int]]:
    """Find all SubsetProduct solutions."""
    n = len(sizes)
    sols = []
    for bits in itertools.product([0, 1], repeat=n):
        if is_feasible_target(sizes, target, list(bits)):
            sols.append(list(bits))
    return sols


# ---------------------------------------------------------------------------
# Random instance generators
# ---------------------------------------------------------------------------

def random_x3c(rng, universe_size: int, num_subsets: int):
    """Generate random X3C instance."""
    elems = list(range(universe_size))
    subsets = []
    seen = set()
    attempts = 0
    while len(subsets) < num_subsets and attempts < num_subsets * 10:
        s = tuple(sorted(rng.sample(elems, 3)))
        if s not in seen:
            seen.add(s)
            subsets.append(list(s))
        attempts += 1
    return universe_size, subsets


def random_x3c_with_cover(rng, q: int, extra: int = 0):
    """Generate X3C instance guaranteed to have at least one cover."""
    universe_size = 3 * q
    elems = list(range(universe_size))
    shuffled = list(elems)
    rng.shuffle(shuffled)
    subsets = [sorted(shuffled[i:i+3]) for i in range(0, universe_size, 3)]
    # Add extra random subsets
    seen = set(tuple(s) for s in subsets)
    for _ in range(extra):
        s = tuple(sorted(rng.sample(elems, 3)))
        if s not in seen:
            seen.add(s)
            subsets.append(list(s))
    return universe_size, subsets


# ---------------------------------------------------------------------------
# Tests
# ---------------------------------------------------------------------------

def test_yes_example():
    """Reproduce Typst YES example."""
    print("  Testing YES example...")
    checks = 0

    universe_size = 9
    subsets = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6]]

    sizes, target = reduce(universe_size, subsets)

    # Verify primes
    primes = sieve_primes(9)
    assert primes == [2, 3, 5, 7, 11, 13, 17, 19, 23]
    checks += 1

    # Verify sizes from Typst
    assert sizes[0] == 2 * 3 * 5  # 30
    checks += 1
    assert sizes[0] == 30
    checks += 1
    assert sizes[1] == 7 * 11 * 13  # 1001
    checks += 1
    assert sizes[1] == 1001
    checks += 1
    assert sizes[2] == 17 * 19 * 23  # 7429
    checks += 1
    assert sizes[2] == 7429
    checks += 1
    assert sizes[3] == 2 * 7 * 17  # 238
    checks += 1
    assert sizes[3] == 238
    checks += 1

    # Verify target from Typst
    assert target == 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23
    checks += 1
    assert target == 223092870
    checks += 1

    # (1,1,1,0) should satisfy
    sol = [1, 1, 1, 0]
    assert is_feasible_target(sizes, target, sol)
    checks += 1
    assert is_feasible_source(universe_size, subsets, sol)
    checks += 1

    # Product check from Typst
    assert 30 * 1001 * 7429 == 223092870
    checks += 1

    # Verify extraction
    extracted = extract_solution(sol)
    assert is_feasible_source(universe_size, subsets, extracted)
    checks += 1

    # Verify uniqueness: only (1,1,1,0) satisfies both
    all_sp = all_sp_solutions(sizes, target)
    assert len(all_sp) == 1
    assert all_sp[0] == [1, 1, 1, 0]
    checks += 1

    all_x3c = all_x3c_solutions(universe_size, subsets)
    assert len(all_x3c) == 1
    assert all_x3c[0] == [1, 1, 1, 0]
    checks += 1

    return checks


def test_no_example():
    """Reproduce Typst NO example."""
    print("  Testing NO example...")
    checks = 0

    universe_size = 9
    subsets = [[0, 1, 2], [0, 3, 4], [0, 5, 6], [3, 7, 8]]

    # No X3C solution
    x3c_sols = all_x3c_solutions(universe_size, subsets)
    assert len(x3c_sols) == 0
    checks += 1

    sizes, target = reduce(universe_size, subsets)

    # Verify sizes from Typst
    assert sizes[0] == 2 * 3 * 5  # 30
    checks += 1
    assert sizes[1] == 2 * 7 * 11  # 154
    checks += 1
    assert sizes[2] == 2 * 13 * 17  # 442
    checks += 1
    assert sizes[3] == 7 * 19 * 23  # 3059
    checks += 1

    assert target == 223092870
    checks += 1

    # No SP solution
    sp_sols = all_sp_solutions(sizes, target)
    assert len(sp_sols) == 0
    checks += 1

    # All 16 assignments fail
    for bits in itertools.product([0, 1], repeat=4):
        assert not is_feasible_target(sizes, target, list(bits))
        checks += 1

    return checks


def test_exhaustive_small():
    """Exhaustive forward+backward for small instances."""
    print("  Testing exhaustive small...")
    checks = 0

    # universe_size=3: only triple is [0,1,2]
    all_triples_3 = [[0, 1, 2]]
    for num_sub in range(1, 2):
        for chosen in itertools.combinations(all_triples_3, num_sub):
            subsets = [list(t) for t in chosen]
            src = len(all_x3c_solutions(3, subsets)) > 0
            sizes, target = reduce(3, subsets)
            tgt = len(all_sp_solutions(sizes, target)) > 0
            assert src == tgt
            checks += 1

    # universe_size=6: all combos of triples from {0..5}
    all_triples_6 = [list(t) for t in itertools.combinations(range(6), 3)]
    for num_sub in range(1, 7):
        for chosen in itertools.combinations(all_triples_6, num_sub):
            subsets = [list(t) for t in chosen]
            n = len(subsets)
            if n > 8:
                continue
            src = len(all_x3c_solutions(6, subsets)) > 0
            sizes, target = reduce(6, subsets)
            tgt = len(all_sp_solutions(sizes, target)) > 0
            assert src == tgt
            checks += 1

    # Random instances for universe_size=9
    rng = random.Random(12345)
    for _ in range(500):
        u, subs = random_x3c(rng, 9, rng.randint(1, 5))
        src = len(all_x3c_solutions(u, subs)) > 0
        sizes, target = reduce(u, subs)
        tgt = len(all_sp_solutions(sizes, target)) > 0
        assert src == tgt
        checks += 1

    return checks


def test_extraction_all():
    """Test solution extraction for all feasible instances."""
    print("  Testing extraction...")
    checks = 0

    # universe_size=6, up to 5 subsets
    all_triples_6 = [list(t) for t in itertools.combinations(range(6), 3)]
    for num_sub in range(1, 6):
        for chosen in itertools.combinations(all_triples_6, num_sub):
            subsets = [list(t) for t in chosen]
            n = len(subsets)
            if n > 8:
                continue

            x3c_sols = all_x3c_solutions(6, subsets)
            if not x3c_sols:
                continue

            sizes, target = reduce(6, subsets)
            sp_sols = all_sp_solutions(sizes, target)

            for sp_sol in sp_sols:
                ext = extract_solution(sp_sol)
                assert is_feasible_source(6, subsets, ext)
                checks += 1

            # Bijection check: same solution sets
            assert set(tuple(s) for s in x3c_sols) == set(tuple(s) for s in sp_sols)
            checks += 1

    # Random feasible instances
    rng = random.Random(67890)
    for _ in range(300):
        q = rng.randint(1, 3)
        u, subs = random_x3c_with_cover(rng, q, extra=rng.randint(0, 3))

        x3c_sols = all_x3c_solutions(u, subs)
        if not x3c_sols:
            continue

        sizes, target = reduce(u, subs)
        sp_sols = all_sp_solutions(sizes, target)

        for sp_sol in sp_sols:
            ext = extract_solution(sp_sol)
            assert is_feasible_source(u, subs, ext)
            checks += 1

    return checks


def test_hypothesis_pbt():
    """Property-based testing with hypothesis (2 strategies)."""
    print("  Testing hypothesis PBT...")
    checks = 0

    try:
        from hypothesis import given, settings, assume
        from hypothesis import strategies as st

        # Strategy 1: Random X3C instances
        @given(
            universe_size_mult=st.integers(min_value=1, max_value=3),
            num_subsets=st.integers(min_value=1, max_value=5),
            seed=st.integers(min_value=0, max_value=10000)
        )
        @settings(max_examples=1500, deadline=None)
        def prop_feasibility_preserved(universe_size_mult, num_subsets, seed):
            nonlocal checks
            universe_size = universe_size_mult * 3
            rng = random.Random(seed)
            elems = list(range(universe_size))
            subsets = []
            seen = set()
            for _ in range(num_subsets):
                s = tuple(sorted(rng.sample(elems, 3)))
                if s not in seen:
                    seen.add(s)
                    subsets.append(list(s))

            src = len(all_x3c_solutions(universe_size, subsets)) > 0
            sizes, target = reduce(universe_size, subsets)
            tgt = len(all_sp_solutions(sizes, target)) > 0
            assert src == tgt
            checks += 1

        # Strategy 2: Guaranteed-feasible instances
        @given(
            q=st.integers(min_value=1, max_value=3),
            extra=st.integers(min_value=0, max_value=3),
            seed=st.integers(min_value=0, max_value=10000)
        )
        @settings(max_examples=1500, deadline=None)
        def prop_feasible_extraction(q, extra, seed):
            nonlocal checks
            rng = random.Random(seed)
            universe_size, subsets = random_x3c_with_cover(rng, q, extra)

            assert len(all_x3c_solutions(universe_size, subsets)) > 0

            sizes, target = reduce(universe_size, subsets)
            sp_sols = all_sp_solutions(sizes, target)
            assert len(sp_sols) > 0

            for sol in sp_sols:
                ext = extract_solution(sol)
                assert is_feasible_source(universe_size, subsets, ext)
            checks += 1

        prop_feasibility_preserved()
        prop_feasible_extraction()

    except ImportError:
        print("  hypothesis not available, using manual PBT fallback...")

        # Strategy 1: random instances
        rng = random.Random(11111)
        for _ in range(1500):
            u = rng.choice([3, 6, 9])
            ns = rng.randint(1, 5)
            _, subs = random_x3c(rng, u, ns)
            src = len(all_x3c_solutions(u, subs)) > 0
            sizes, target = reduce(u, subs)
            tgt = len(all_sp_solutions(sizes, target)) > 0
            assert src == tgt
            checks += 1

        # Strategy 2: guaranteed feasible
        rng2 = random.Random(22222)
        for _ in range(1500):
            q = rng2.randint(1, 3)
            u, subs = random_x3c_with_cover(rng2, q, extra=rng2.randint(0, 3))

            assert len(all_x3c_solutions(u, subs)) > 0
            sizes, target = reduce(u, subs)
            sp_sols = all_sp_solutions(sizes, target)
            assert len(sp_sols) > 0
            for sol in sp_sols:
                ext = extract_solution(sol)
                assert is_feasible_source(u, subs, ext)
            checks += 1

    return checks


def test_cross_compare():
    """Cross-compare with constructor script outputs via test vectors JSON."""
    print("  Cross-comparing with test vectors...")
    checks = 0

    tv_path = os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        "test_vectors_exact_cover_by_3_sets_subset_product.json"
    )

    if not os.path.exists(tv_path):
        print("  WARNING: test vectors not found, skipping cross-compare")
        return 0

    with open(tv_path) as f:
        tv = json.load(f)

    for v in tv["vectors"]:
        u = v["source"]["universe_size"]
        subs = v["source"]["subsets"]
        sizes_expected = v["target"]["sizes"]
        target_expected = v["target"]["target"]

        # Our independent reduction must match
        sizes, target = reduce(u, subs)
        assert sizes == sizes_expected, f"Sizes differ for {v['label']}"
        checks += 1
        assert target == target_expected, f"Target differs for {v['label']}"
        checks += 1

        # Feasibility must match
        src_ok = len(all_x3c_solutions(u, subs)) > 0
        assert src_ok == v["source_feasible"], f"Source feasibility mismatch for {v['label']}"
        checks += 1

        tgt_ok = len(all_sp_solutions(sizes, target)) > 0
        assert tgt_ok == v["target_feasible"], f"Target feasibility mismatch for {v['label']}"
        checks += 1

        if v["source_feasible"]:
            assert v["target_feasible"]
            checks += 1

        if not v["source_feasible"]:
            assert not v["target_feasible"]
            checks += 1

        if v["extracted_solution"] is not None:
            assert is_feasible_source(u, subs, v["extracted_solution"])
            checks += 1

    return checks


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    total = 0

    print("=== Adversary verification: X3C -> SubsetProduct ===")

    c = test_yes_example()
    print(f"  YES example: {c} checks")
    total += c

    c = test_no_example()
    print(f"  NO example: {c} checks")
    total += c

    c = test_exhaustive_small()
    print(f"  Exhaustive: {c} checks")
    total += c

    c = test_extraction_all()
    print(f"  Extraction: {c} checks")
    total += c

    c = test_hypothesis_pbt()
    print(f"  Hypothesis PBT: {c} checks")
    total += c

    c = test_cross_compare()
    print(f"  Cross-compare: {c} checks")
    total += c

    print(f"\n{'='*60}")
    print(f"ADVERSARY CHECK COUNT: {total} (minimum: 5,000)")
    print(f"{'='*60}")

    if total < 5000:
        print(f"FAIL: {total} < 5000")
        sys.exit(1)

    print("ADVERSARY: ALL CHECKS PASSED")


if __name__ == "__main__":
    main()
