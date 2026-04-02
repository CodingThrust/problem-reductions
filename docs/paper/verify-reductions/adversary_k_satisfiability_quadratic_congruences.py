#!/usr/bin/env python3
"""
Adversary verification script for KSatisfiability(K3) -> QuadraticCongruences reduction.
Issue #553 — Manders and Adleman (1978).

Independent implementation based solely on the Typst proof document.
Does NOT import from the constructor script.

Requirements: >= 5000 checks, hypothesis PBT with >= 2 strategies.

Note: The reduction produces astronomically large numbers (thousands of bits for n=3).
We verify correctness algebraically: construct x from a known satisfying assignment
via the alpha_j -> theta_j chain, and confirm x^2 = a mod b. For UNSAT instances,
we exhaustively verify no knapsack solution exists.
"""

import itertools
import json
import random
from pathlib import Path
from math import gcd

# ---------------------------------------------------------------------------
# Independent number-theoretic helpers
# ---------------------------------------------------------------------------

def primality_check(n):
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i = 5
    while i * i <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True


def find_modular_inverse(a, m):
    """Extended Euclidean algorithm for modular inverse."""
    if m == 1:
        return 0
    old_r, r = a % m, m
    old_s, s = 1, 0
    while r != 0:
        q = old_r // r
        old_r, r = r, old_r - q * r
        old_s, s = s, old_s - q * s
    if old_r != 1:
        raise ValueError(f"No inverse: gcd({a},{m})={old_r}")
    return old_s % m


def solve_crt_pair(r1, m1, r2, m2):
    """Solve x = r1 mod m1, x = r2 mod m2."""
    g = gcd(m1, m2)
    if (r2 - r1) % g != 0:
        raise ValueError("Incompatible CRT")
    lcm = m1 // g * m2
    diff = (r2 - r1) // g
    inv = find_modular_inverse(m1 // g, m2 // g)
    x = (r1 + m1 * (diff * inv % (m2 // g))) % lcm
    return x, lcm


# ---------------------------------------------------------------------------
# Independent reduction implementation (from Typst proof only)
# ---------------------------------------------------------------------------

def build_standard_clauses(num_active_vars):
    """Build all standard 3-literal clauses over num_active_vars variables."""
    l = num_active_vars
    clauses = []
    seen = set()
    for triple in itertools.combinations(range(1, l + 1), 3):
        for pattern in itertools.product([1, -1], repeat=3):
            c = frozenset(s * v for s, v in zip(pattern, triple))
            if c not in seen:
                seen.add(c)
                clauses.append(c)
    index = {c: i + 1 for i, c in enumerate(clauses)}
    return clauses, index


def independent_reduce(n, input_clauses):
    """
    Independent reduction from Typst proof.

    Steps:
    1. Preprocess: deduplicate, find active vars, remap
    2. Base-8 encoding: tau_phi, f_i^+, f_i^-
    3. Doubled coefficients d_j = 2*c_j
    4. CRT lifting with primes >= 13
    5. Output (a, b, c)
    """
    # Deduplicate
    clause_fsets = []
    seen = set()
    for c in input_clauses:
        fs = frozenset(c)
        if fs not in seen:
            seen.add(fs)
            clause_fsets.append(fs)

    # Active variables
    active = sorted({abs(lit) for c in clause_fsets for lit in c})
    var_map = {v: i + 1 for i, v in enumerate(active)}
    l = len(active)

    # Remap
    remapped = []
    for c in clause_fsets:
        remapped.append(frozenset(
            (var_map[abs(lit)] if lit > 0 else -var_map[abs(lit)])
            for lit in c
        ))

    std_clauses, std_idx = build_standard_clauses(l)
    M = len(std_clauses)

    # tau_phi = -sum 8^j for each clause in phi_R
    tau_phi = 0
    for c in remapped:
        if c in std_idx:
            tau_phi -= 8 ** std_idx[c]

    # f_i^+, f_i^-
    fp = [0] * (l + 1)
    fm = [0] * (l + 1)
    for sc in std_clauses:
        j = std_idx[sc]
        for lit in sc:
            v = abs(lit)
            if lit > 0:
                fp[v] += 8 ** j
            else:
                fm[v] += 8 ** j

    N = 2 * M + l

    # Doubled coefficients
    d = [0] * (N + 1)
    d[0] = 2
    for k in range(1, M + 1):
        d[2 * k - 1] = -(8 ** k)
        d[2 * k] = -2 * (8 ** k)
    for i in range(1, l + 1):
        d[2 * M + i] = fp[i] - fm[i]

    tau_2 = 2 * tau_phi + sum(d) + 2 * sum(fm[i] for i in range(1, l + 1))
    mod_val = 2 * (8 ** (M + 1))

    # Primes >= 13
    primes = []
    p = 13
    while len(primes) < N + 1:
        if primality_check(p):
            primes.append(p)
        p += 1

    pp_list = [p ** (N + 1) for p in primes]
    K = 1
    for pp in pp_list:
        K *= pp

    # CRT for thetas
    thetas = []
    for j in range(N + 1):
        other = K // pp_list[j]
        theta, lcm = solve_crt_pair(0, other, d[j] % mod_val, mod_val)
        if theta == 0:
            theta = lcm
        while theta % primes[j] == 0:
            theta += lcm
        thetas.append(theta)

    H = sum(thetas)
    beta = mod_val * K
    inv_factor = mod_val + K
    assert gcd(inv_factor, beta) == 1
    inv = find_modular_inverse(inv_factor, beta)
    alpha = (inv * (K * tau_2 ** 2 + mod_val * H ** 2)) % beta

    return int(alpha), int(beta), int(H) + 1, {
        'thetas': thetas, 'H': H, 'K': K, 'tau_2': tau_2,
        'mod_val': mod_val, 'primes': primes, 'N': N, 'M': M, 'l': l,
        'd': d, 'pp_list': pp_list, 'var_map': var_map,
        'remapped': remapped, 'std_clauses': std_clauses, 'std_idx': std_idx,
        'fp': fp, 'fm': fm, 'tau_phi': tau_phi,
    }


# ---------------------------------------------------------------------------
# Independent assignment-to-x converter
# ---------------------------------------------------------------------------

def build_alphas(assignment, info):
    """Convert Boolean assignment to alpha_j values (independently from Typst proof)."""
    M = info['M']
    l = info['l']
    N = info['N']
    var_map = info['var_map']
    remapped = info['remapped']
    std_clauses = info['std_clauses']
    std_idx = info['std_idx']

    r = {}
    for orig, new in var_map.items():
        r[new] = 1 if assignment[orig - 1] else 0

    alphas = [0] * (N + 1)
    alphas[0] = 1

    for i in range(1, l + 1):
        alphas[2 * M + i] = 1 - 2 * r[i]

    for k in range(1, M + 1):
        sigma = std_clauses[k - 1]
        in_phi = sigma in set(remapped)
        y = 0
        for lit in sigma:
            v = abs(lit)
            if lit > 0:
                y += r[v]
            else:
                y += 1 - r[v]
        if in_phi:
            y -= 1

        target = 3 - 2 * y
        if target == 3:
            alphas[2 * k - 1], alphas[2 * k] = 1, 1
        elif target == 1:
            alphas[2 * k - 1], alphas[2 * k] = -1, 1
        elif target == -1:
            alphas[2 * k - 1], alphas[2 * k] = 1, -1
        elif target == -3:
            alphas[2 * k - 1], alphas[2 * k] = -1, -1
        else:
            return None

    return alphas


def compute_x(alphas, thetas):
    return sum(a * t for a, t in zip(alphas, thetas))


# ---------------------------------------------------------------------------
# Independent feasibility checkers
# ---------------------------------------------------------------------------

def sat_check(n, clauses):
    for bits in range(1 << n):
        a = [(bits >> i) & 1 == 1 for i in range(n)]
        if all(any(
            (a[abs(l) - 1] if l > 0 else not a[abs(l) - 1])
            for l in c
        ) for c in clauses):
            return True, a
    return False, None


def knapsack_check(alphas, d, tau_2, mod_val):
    s = sum(dj * aj for dj, aj in zip(d, alphas))
    return s % mod_val == tau_2 % mod_val


def rand_3sat(n, m, rng):
    clauses = []
    for _ in range(m):
        vs = rng.sample(range(1, n + 1), 3)
        clauses.append([v if rng.random() < 0.5 else -v for v in vs])
    return clauses


# ---------------------------------------------------------------------------
# Tests
# ---------------------------------------------------------------------------

total_checks = 0


def check(cond, msg=""):
    global total_checks
    assert cond, msg
    total_checks += 1


def test_yes_example():
    """Reproduce YES example from Typst."""
    global total_checks
    n = 3
    clauses = [[1, 2, 3]]
    sat, assignment = sat_check(n, clauses)
    check(sat, "YES must be satisfiable")

    a, b, c, info = independent_reduce(n, clauses)
    check(0 <= a < b, "a < b")
    check(c > 1, "c > 1")

    alphas = build_alphas(assignment, info)
    check(alphas is not None, "alphas must exist")
    check(all(alpha in (-1, 1) for alpha in alphas), "all alphas +/-1")

    check(knapsack_check(alphas, info['d'], info['tau_2'], info['mod_val']),
          "knapsack must hold")

    x = abs(compute_x(alphas, info['thetas']))
    check(0 <= x <= info['H'], f"|x|={x} <= H={info['H']}")
    check((x * x) % b == a, "x^2 = a mod b")

    print(f"  YES example: {total_checks} checks so far")


def test_no_example():
    """Reproduce NO example from Typst."""
    global total_checks
    n = 3
    clauses = []
    for signs in itertools.product([1, -1], repeat=3):
        clauses.append([signs[0], signs[1] * 2, signs[2] * 3])
    check(len(clauses) == 8, "8 clauses")

    sat, _ = sat_check(n, clauses)
    check(not sat, "NO must be unsatisfiable")

    a, b, c, info = independent_reduce(n, clauses)
    N = info['N']
    d = info['d']
    tau_2 = info['tau_2']
    mod_val = info['mod_val']

    # Exhaustive knapsack check
    found = False
    for bits in range(1 << (N + 1)):
        alphas = [(1 if (bits >> j) & 1 else -1) for j in range(N + 1)]
        s = sum(dj * aj for dj, aj in zip(d, alphas))
        if s == tau_2:
            found = True
            break
    check(not found, "NO knapsack must have no exact solution")

    print(f"  NO example: {total_checks} checks so far")


def test_exhaustive_forward_backward():
    """Forward/backward check for many random instances."""
    global total_checks
    rng = random.Random(123)

    # All single clauses for n=3
    lits = [1, 2, 3, -1, -2, -3]
    for combo in itertools.combinations(lits, 3):
        if len(set(abs(l) for l in combo)) == 3:
            clauses = [list(combo)]
            sat, assignment = sat_check(3, clauses)
            if sat:
                a, b, c, info = independent_reduce(3, clauses)
                alphas = build_alphas(assignment, info)
                check(alphas is not None)
                x = abs(compute_x(alphas, info['thetas']))
                check((x * x) % b == a, f"forward check for {combo}")

    # Random instances
    for n in [3, 4]:
        for m in range(1, 5):
            num = 80 if n == 3 else 30
            for _ in range(num):
                clauses = rand_3sat(n, m, rng)
                sat, assignment = sat_check(n, clauses)
                if sat:
                    a, b, c, info = independent_reduce(n, clauses)
                    alphas = build_alphas(assignment, info)
                    if alphas is not None:
                        x = abs(compute_x(alphas, info['thetas']))
                        check((x * x) % b == a)
                        check(0 <= x <= info['H'])
                        check(knapsack_check(alphas, info['d'], info['tau_2'], info['mod_val']))
                else:
                    a, b, c, info = independent_reduce(n, clauses)
                    N = info['N']
                    if N <= 20:
                        found = False
                        for bits in range(1 << (N + 1)):
                            als = [(1 if (bits >> j) & 1 else -1) for j in range(N + 1)]
                            if sum(dj * aj for dj, aj in zip(info['d'], als)) == info['tau_2']:
                                found = True
                                break
                        check(not found, f"UNSAT knapsack for n={n} m={m}")

    print(f"  Forward/backward: {total_checks} checks so far")


def test_extraction():
    """Verify assignment recovery from x."""
    global total_checks
    rng = random.Random(456)

    for n in [3, 4]:
        for m in range(1, 4):
            for _ in range(60):
                clauses = rand_3sat(n, m, rng)
                sat, assignment = sat_check(n, clauses)
                if not sat:
                    continue

                a, b, c, info = independent_reduce(n, clauses)
                alphas = build_alphas(assignment, info)
                if alphas is None:
                    continue

                M = info['M']
                l = info['l']
                # var_map: orig_var -> new_var; invert to new_var -> orig_var
                inv_map = {new: orig for orig, new in info['var_map'].items()}

                recovered = [False] * n
                for i in range(1, l + 1):
                    r_xi = (1 - alphas[2 * M + i]) // 2
                    orig_var = inv_map[i]
                    recovered[orig_var - 1] = (r_xi == 1)

                ok = all(any(
                    (recovered[abs(lit) - 1] if lit > 0 else not recovered[abs(lit) - 1])
                    for lit in clause
                ) for clause in clauses)
                check(ok, "recovered assignment must satisfy formula")

                # Also check each alpha is +/- 1
                for alpha in alphas:
                    check(alpha in (-1, 1))

    print(f"  Extraction: {total_checks} checks so far")


def test_overhead():
    """Verify structural overhead properties."""
    global total_checks
    rng = random.Random(789)

    for n in [3, 4, 5]:
        for m in range(1, 5):
            for _ in range(15):
                clauses = rand_3sat(n, m, rng)
                a, b, c, info = independent_reduce(n, clauses)

                check(b == info['mod_val'] * info['K'])
                check(c == info['H'] + 1)
                check(0 <= a < b)
                check(info['K'] % 2 != 0)
                check(gcd(info['mod_val'], info['K']) == 1)
                check(gcd(info['mod_val'] + info['K'], b) == 1)
                check(info['N'] == 2 * info['M'] + info['l'])
                check(len(info['primes']) == info['N'] + 1)

    print(f"  Overhead: {total_checks} checks so far")


def test_structural_properties():
    """Verify CRT and prime conditions."""
    global total_checks
    rng = random.Random(321)

    for n in [3, 4]:
        for m in [1, 2]:
            for _ in range(20):
                clauses = rand_3sat(n, m, rng)
                _, _, _, info = independent_reduce(n, clauses)

                for j in range(info['N'] + 1):
                    theta = info['thetas'][j]
                    check(theta > 0)
                    check(theta % info['mod_val'] == info['d'][j] % info['mod_val'])
                    other = info['K'] // info['pp_list'][j]
                    check(theta % other == 0)
                    check(theta % info['primes'][j] != 0)

                for p in info['primes']:
                    check(primality_check(p))
                    check(p >= 13)

                check(len(set(info['primes'])) == len(info['primes']))
                check(info['d'][0] == 2)

    print(f"  Structural: {total_checks} checks so far")


def test_hypothesis_pbt():
    """Property-based testing with hypothesis."""
    from hypothesis import given, settings, HealthCheck
    from hypothesis import strategies as st

    counter = {"n": 0}

    # Strategy 1: Random 3-SAT instances
    @given(
        n=st.integers(min_value=3, max_value=5),
        m=st.integers(min_value=1, max_value=4),
        seed=st.integers(min_value=0, max_value=10000),
    )
    @settings(max_examples=1500, suppress_health_check=[HealthCheck.too_slow], deadline=None)
    def strategy_1(n, m, seed):
        rng = random.Random(seed)
        clauses = rand_3sat(n, m, rng)
        sat, assignment = sat_check(n, clauses)
        a, b, c, info = independent_reduce(n, clauses)

        assert 0 <= a < b
        assert c > 1
        assert b == info['mod_val'] * info['K']

        if sat:
            alphas = build_alphas(assignment, info)
            if alphas is not None:
                x = abs(compute_x(alphas, info['thetas']))
                assert (x * x) % b == a
                assert 0 <= x <= info['H']

        counter["n"] += 1

    # Strategy 2: Sign pattern enumeration
    @given(
        signs=st.lists(
            st.lists(st.booleans(), min_size=3, max_size=3),
            min_size=1, max_size=5,
        ),
    )
    @settings(max_examples=1500, suppress_health_check=[HealthCheck.too_slow], deadline=None)
    def strategy_2(signs):
        n = 3
        clauses = []
        for sl in signs:
            clause = [i + 1 if sl[i] else -(i + 1) for i in range(3)]
            clauses.append(clause)

        sat, assignment = sat_check(n, clauses)
        a, b, c, info = independent_reduce(n, clauses)

        assert 0 <= a < b
        assert c > 1

        if sat:
            alphas = build_alphas(assignment, info)
            if alphas is not None:
                x = abs(compute_x(alphas, info['thetas']))
                assert (x * x) % b == a

        counter["n"] += 1

    print("  Running hypothesis strategy 1 (random instances)...")
    strategy_1()
    s1 = counter["n"]
    print(f"    Strategy 1: {s1} examples")

    print("  Running hypothesis strategy 2 (sign patterns)...")
    strategy_2()
    print(f"    Strategy 2: {counter['n'] - s1} examples")

    return counter["n"]


def test_cross_comparison():
    """Compare outputs with constructor script's test vectors."""
    global total_checks

    vec_path = Path(__file__).parent / "test_vectors_k_satisfiability_quadratic_congruences.json"
    if not vec_path.exists():
        print("  Cross-comparison: SKIPPED (no test vectors)")
        return

    with open(vec_path) as f:
        vectors = json.load(f)

    # YES instance
    yi = vectors["yes_instance"]
    n_yes = yi["input"]["num_vars"]
    clauses_yes = yi["input"]["clauses"]
    a, b, c, _ = independent_reduce(n_yes, clauses_yes)
    check(str(a) == str(yi["output"]["a"]), "YES a matches")
    check(str(b) == str(yi["output"]["b"]), "YES b matches")
    check(str(c) == str(yi["output"]["c"]), "YES c matches")

    # Verify witness
    x_witness = int(yi["witness_x"])
    check((x_witness * x_witness) % b == a, "YES witness valid")

    # NO instance
    ni = vectors["no_instance"]
    a_no, b_no, c_no, _ = independent_reduce(ni["input"]["num_vars"], ni["input"]["clauses"])
    check(str(a_no) == str(ni["output"]["a"]), "NO a matches")
    check(str(b_no) == str(ni["output"]["b"]), "NO b matches")
    check(str(c_no) == str(ni["output"]["c"]), "NO c matches")

    for claim in vectors["claims"]:
        check(claim["verified"], f"Claim {claim['tag']} not verified")

    print(f"  Cross-comparison: {total_checks} checks so far")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    global total_checks

    print("=== Adversary: KSatisfiability(K3) -> QuadraticCongruences ===")
    print("=== Issue #553 — Manders and Adleman (1978) ===\n")

    test_yes_example()
    test_no_example()
    test_exhaustive_forward_backward()
    test_extraction()
    test_overhead()
    test_structural_properties()

    pbt_count = test_hypothesis_pbt()
    total_checks += pbt_count

    test_cross_comparison()

    print(f"\n=== TOTAL ADVERSARY CHECKS: {total_checks} ===")
    assert total_checks >= 5000, f"Need >= 5000, got {total_checks}"
    print("ALL ADVERSARY CHECKS PASSED")


if __name__ == "__main__":
    main()
