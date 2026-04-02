#!/usr/bin/env python3
"""
Adversary script: KSatisfiability(K3) -> SimultaneousIncongruences

Independent verification using hypothesis property-based testing.
Tests the same reduction from a different angle, with >= 5000 checks.
"""

import itertools
import math
import random
import sys

# Try hypothesis; fall back to manual PBT if not available
try:
    from hypothesis import given, settings, assume, HealthCheck
    from hypothesis import strategies as st
    HAS_HYPOTHESIS = True
except ImportError:
    HAS_HYPOTHESIS = False
    print("WARNING: hypothesis not installed, using manual PBT")


# ============================================================
# Independent reimplementation of core functions
# (intentionally different code from verify script)
# ============================================================


def get_primes(n: int) -> list[int]:
    """Get first n primes >= 5 using sieve."""
    if n == 0:
        return []
    # Upper bound for nth prime >= 5
    limit = max(100, n * 20)
    sieve = [True] * limit
    sieve[0] = sieve[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if sieve[i]:
            for j in range(i * i, limit, i):
                sieve[j] = False
    result = [p for p in range(5, limit) if sieve[p]]
    return result[:n]


def egcd(a: int, b: int) -> tuple[int, int, int]:
    if a == 0:
        return b, 0, 1
    g, x, y = egcd(b % a, a)
    return g, y - (b // a) * x, x


def chinese_remainder(rems: list[int], mods: list[int]) -> int:
    """CRT for pairwise coprime moduli."""
    M = 1
    for m in mods:
        M *= m
    result = 0
    for r, m in zip(rems, mods):
        Mi = M // m
        _, inv, _ = egcd(Mi, m)
        result += r * Mi * inv
    return result % M


def eval_lit(lit: int, assign: dict[int, bool]) -> bool:
    v = abs(lit)
    val = assign[v]
    return val if lit > 0 else not val


def check_3sat(nvars: int, clauses: list[tuple[int, ...]], assign: dict[int, bool]) -> bool:
    for c in clauses:
        if not any(eval_lit(l, assign) for l in c):
            return False
    return True


def brute_3sat(nvars: int, clauses: list[tuple[int, ...]]) -> dict[int, bool] | None:
    for bits in itertools.product([False, True], repeat=nvars):
        assign = {i + 1: bits[i] for i in range(nvars)}
        if check_3sat(nvars, clauses, assign):
            return assign
    return None


def check_si(x: int, pairs: list[tuple[int, int]]) -> bool:
    """Check if x satisfies all incongruences."""
    return all(x % b != a % b for a, b in pairs)


def brute_si(pairs: list[tuple[int, int]], limit: int) -> int | None:
    for x in range(limit):
        if check_si(x, pairs):
            return x
    return None


def do_reduce(nvars: int, clauses: list[tuple[int, ...]]) -> tuple[list[tuple[int, int]], list[int]]:
    """Independent reimplementation of the reduction.
    Returns (pairs, primes)."""
    primes = get_primes(nvars)
    pairs: list[tuple[int, int]] = []

    # Variable encoding: forbid invalid residues
    for i in range(nvars):
        p = primes[i]
        # Forbid 0: use (p, p)
        pairs.append((p, p))
        # Forbid 3..p-1
        for r in range(3, p):
            pairs.append((r, p))

    # Clause encoding
    for clause in clauses:
        var_idxs = [abs(l) - 1 for l in clause]
        # Falsifying residues: positive lit -> 2, negative lit -> 1
        false_res = [2 if l > 0 else 1 for l in clause]
        clause_primes = [primes[vi] for vi in var_idxs]
        M = clause_primes[0] * clause_primes[1] * clause_primes[2]
        R = chinese_remainder(false_res, clause_primes)
        if R == 0:
            pairs.append((M, M))
        else:
            pairs.append((R, M))

    return pairs, primes


def verify_instance(nvars: int, clauses: list[tuple[int, ...]]) -> None:
    """Verify a single 3-SAT instance end-to-end."""
    assert nvars >= 3
    for c in clauses:
        assert len(c) == 3
        assert len(set(abs(l) for l in c)) == 3
        for l in c:
            assert 1 <= abs(l) <= nvars

    pairs, primes = do_reduce(nvars, clauses)

    # Validate target
    for a, b in pairs:
        assert b > 0, f"Invalid modulus: {b}"
        assert 1 <= a <= b, f"Invalid pair: ({a}, {b})"

    # Expected number of pairs
    expected_var_pairs = sum(p - 2 for p in primes)
    expected_total = expected_var_pairs + len(clauses)
    assert len(pairs) == expected_total, \
        f"Expected {expected_total} pairs, got {len(pairs)}"

    # Compute search limit
    all_mods = set(b for _, b in pairs)
    lcm_val = 1
    for m in all_mods:
        lcm_val = lcm_val * m // math.gcd(lcm_val, m)
    search_limit = min(lcm_val, 500_000)

    src_sol = brute_3sat(nvars, clauses)
    tgt_sol = brute_si(pairs, search_limit)

    src_sat = src_sol is not None
    tgt_sat = tgt_sol is not None
    assert src_sat == tgt_sat, \
        f"Sat mismatch: src={src_sat} tgt={tgt_sat}, n={nvars}, clauses={clauses}"

    if tgt_sat:
        x = tgt_sol
        # Extract assignment
        extracted = {}
        for i in range(nvars):
            r = x % primes[i]
            assert r in (1, 2), f"Var {i}: residue {r} not in {{1,2}}"
            extracted[i + 1] = (r == 1)
        assert check_3sat(nvars, clauses, extracted), \
            f"Extraction failed: n={nvars}, clauses={clauses}, x={x}"


# ============================================================
# Hypothesis-based property tests
# ============================================================

if HAS_HYPOTHESIS:
    HC_SUPPRESS = [HealthCheck.too_slow, HealthCheck.filter_too_much]

    @given(
        nvars=st.integers(min_value=3, max_value=5),
        clause_data=st.lists(
            st.tuples(
                st.tuples(
                    st.integers(min_value=1, max_value=5),
                    st.integers(min_value=1, max_value=5),
                    st.integers(min_value=1, max_value=5),
                ),
                st.tuples(
                    st.sampled_from([-1, 1]),
                    st.sampled_from([-1, 1]),
                    st.sampled_from([-1, 1]),
                ),
            ),
            min_size=1, max_size=3,
        ),
    )
    @settings(max_examples=3000, deadline=None, suppress_health_check=HC_SUPPRESS)
    def test_reduction_property(nvars, clause_data):
        global counter
        clauses = []
        for (v1, v2, v3), (s1, s2, s3) in clause_data:
            assume(v1 <= nvars and v2 <= nvars and v3 <= nvars)
            assume(len({v1, v2, v3}) == 3)
            clauses.append((s1 * v1, s2 * v2, s3 * v3))
        if not clauses:
            return
        verify_instance(nvars, clauses)
        counter += 1

    @given(
        nvars=st.integers(min_value=3, max_value=6),
        seed=st.integers(min_value=0, max_value=10000),
    )
    @settings(max_examples=2500, deadline=None, suppress_health_check=HC_SUPPRESS)
    def test_reduction_seeded(nvars, seed):
        global counter
        rng = random.Random(seed)
        m = rng.randint(1, 3)
        clauses = []
        for _ in range(m):
            if nvars < 3:
                return
            vs = rng.sample(range(1, nvars + 1), 3)
            lits = tuple(v if rng.random() < 0.5 else -v for v in vs)
            clauses.append(lits)
        verify_instance(nvars, clauses)
        counter += 1

else:
    def test_reduction_property():
        global counter
        rng = random.Random(99999)
        for _ in range(3000):
            nvars = rng.randint(3, 5)
            m = rng.randint(1, 3)
            clauses = []
            for _ in range(m):
                vs = rng.sample(range(1, nvars + 1), 3)
                lits = tuple(v if rng.random() < 0.5 else -v for v in vs)
                clauses.append(lits)
            verify_instance(nvars, clauses)
            counter += 1

    def test_reduction_seeded():
        global counter
        for seed in range(2500):
            rng = random.Random(seed)
            nvars = rng.randint(3, 6)
            m = rng.randint(1, 3)
            clauses = []
            for _ in range(m):
                if nvars < 3:
                    continue
                vs = rng.sample(range(1, nvars + 1), 3)
                lits = tuple(v if rng.random() < 0.5 else -v for v in vs)
                clauses.append(lits)
            if not clauses:
                continue
            verify_instance(nvars, clauses)
            counter += 1


# ============================================================
# Additional adversarial tests
# ============================================================


def test_boundary_cases():
    """Test specific boundary/adversarial cases."""
    global counter

    # All positive literals
    verify_instance(3, [(1, 2, 3)])
    counter += 1

    # All negative literals
    verify_instance(3, [(-1, -2, -3)])
    counter += 1

    # Mixed
    verify_instance(3, [(1, -2, 3)])
    counter += 1

    # Multiple clauses with shared variables
    verify_instance(4, [(1, 2, 3), (-1, -2, 4)])
    counter += 1

    # Same clause repeated
    verify_instance(3, [(1, 2, 3), (1, 2, 3)])
    counter += 1

    # Contradictory pair
    verify_instance(4, [(1, 2, 3), (-1, -2, -3)])
    counter += 1

    # All sign combos for single clause on 3 vars
    for s1, s2, s3 in itertools.product([-1, 1], repeat=3):
        verify_instance(3, [(s1, s2 * 2, s3 * 3)])
        counter += 1

    # All single clauses on 4 vars
    for v_combo in itertools.combinations(range(1, 5), 3):
        for s1, s2, s3 in itertools.product([-1, 1], repeat=3):
            c = tuple(s * v for s, v in zip((s1, s2, s3), v_combo))
            verify_instance(4, [c])
            counter += 1

    # All single clauses on 5 vars
    for v_combo in itertools.combinations(range(1, 6), 3):
        for s1, s2, s3 in itertools.product([-1, 1], repeat=3):
            c = tuple(s * v for s, v in zip((s1, s2, s3), v_combo))
            verify_instance(5, [c])
            counter += 1

    # Test unsatisfiable: all 8 clauses on 3 vars
    all_8 = [
        (1, 2, 3), (-1, -2, -3), (1, -2, 3), (-1, 2, -3),
        (1, 2, -3), (-1, -2, 3), (-1, 2, 3), (1, -2, -3),
    ]
    verify_instance(3, all_8)
    counter += 1

    print(f"  boundary cases: {counter} total so far")


# ============================================================
# Main
# ============================================================

counter = 0

if __name__ == "__main__":
    print("=" * 60)
    print("Adversary: KSatisfiability(K3) -> SimultaneousIncongruences")
    print("=" * 60)

    print("\n--- Boundary cases ---")
    test_boundary_cases()

    print("\n--- Property-based test 1 ---")
    test_reduction_property()
    print(f"  after PBT1: {counter} total")

    print("\n--- Property-based test 2 ---")
    test_reduction_seeded()
    print(f"  after PBT2: {counter} total")

    print(f"\n{'=' * 60}")
    print(f"ADVERSARY TOTAL CHECKS: {counter}")
    assert counter >= 5000, f"Only {counter} checks, need >= 5000"
    print("ADVERSARY PASSED")
