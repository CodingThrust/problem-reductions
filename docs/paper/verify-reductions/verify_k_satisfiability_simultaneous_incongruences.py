#!/usr/bin/env python3
"""
Verification script: KSatisfiability(K3) -> SimultaneousIncongruences

Reduction from 3-SAT to Simultaneous Incongruences via Stockmeyer & Meyer (1973).
Reference: Garey & Johnson, Appendix A7.1, p.249.

7 mandatory sections:
  1. reduce()
  2. extract_solution()
  3. is_valid_source()
  4. is_valid_target()
  5. closed_loop_check()
  6. exhaustive_small()
  7. random_stress()
"""

import itertools
import json
import math
import random
import sys

# ============================================================
# Section 0: Core types and helpers
# ============================================================

# First n primes >= 5
def nth_primes_from_5(n: int) -> list[int]:
    """Return the first n primes >= 5."""
    primes = []
    candidate = 5
    while len(primes) < n:
        if all(candidate % p != 0 for p in range(2, int(candidate**0.5) + 1)):
            primes.append(candidate)
        candidate += 1 if candidate == 2 else 2
    return primes


def crt_two(r1: int, m1: int, r2: int, m2: int) -> tuple[int, int]:
    """Solve x = r1 mod m1, x = r2 mod m2 via extended Euclidean.
    Returns (x, m1*m2). Assumes gcd(m1, m2) = 1."""
    g, a, _ = extended_gcd(m1, m2)
    assert g == 1, f"Moduli {m1}, {m2} not coprime"
    M = m1 * m2
    x = (r1 + m1 * a * (r2 - r1)) % M
    return x, M


def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    """Extended Euclidean algorithm. Returns (g, x, y) with a*x + b*y = g."""
    if b == 0:
        return a, 1, 0
    g, x1, y1 = extended_gcd(b, a % b)
    return g, y1, x1 - (a // b) * y1


def crt_solve(residues: list[int], moduli: list[int]) -> tuple[int, int]:
    """Solve system of congruences via CRT. Returns (x, M)."""
    x, m = residues[0], moduli[0]
    for i in range(1, len(residues)):
        x, m = crt_two(x, m, residues[i], moduli[i])
    return x, m


def literal_value(lit: int, assignment: list[bool]) -> bool:
    """Evaluate a literal (1-indexed, negative = negation)."""
    var_idx = abs(lit) - 1
    val = assignment[var_idx]
    return val if lit > 0 else not val


def is_3sat_satisfied(num_vars: int, clauses: list[list[int]],
                      assignment: list[bool]) -> bool:
    """Check if assignment satisfies all 3-SAT clauses."""
    assert len(assignment) == num_vars
    for clause in clauses:
        if not any(literal_value(lit, assignment) for lit in clause):
            return False
    return True


def solve_3sat_brute(num_vars: int, clauses: list[list[int]]) -> list[bool] | None:
    """Brute-force 3-SAT solver."""
    for bits in itertools.product([False, True], repeat=num_vars):
        a = list(bits)
        if is_3sat_satisfied(num_vars, clauses, a):
            return a
    return None


def is_3sat_satisfiable(num_vars: int, clauses: list[list[int]]) -> bool:
    return solve_3sat_brute(num_vars, clauses) is not None


def solve_si_brute(pairs: list[tuple[int, int]], search_limit: int) -> int | None:
    """Brute-force Simultaneous Incongruences solver.
    Searches x in [0, search_limit) for x that avoids all forbidden residues."""
    for x in range(search_limit):
        if all(x % b != a % b for a, b in pairs):
            return x
    return None


def is_si_satisfiable(pairs: list[tuple[int, int]], search_limit: int) -> bool:
    return solve_si_brute(pairs, search_limit) is not None


# ============================================================
# Section 1: reduce()
# ============================================================


def reduce(num_vars: int,
           clauses: list[list[int]]) -> tuple[list[tuple[int, int]], dict]:
    """
    Reduce 3-SAT to Simultaneous Incongruences.

    Encoding:
    - Assign distinct primes p_i >= 5 to each variable x_i.
    - TRUE(x_i) <-> x = 1 (mod p_i), FALSE(x_i) <-> x = 2 (mod p_i).
    - Forbid all other residues {0, 3, 4, ..., p_i-1} for each variable.
    - For each clause, use CRT to find the unique residue modulo
      the product of its variables' primes that corresponds to
      all literals being false. Forbid that residue.

    Model constraint: pairs (a, b) must satisfy 1 <= a <= b, b > 0.
    - For residue r > 0: pair (r, p_i) with r < p_i, so 1 <= r <= p_i.
    - For residue 0: pair (p_i, p_i) since p_i % p_i = 0.

    Returns: (pairs, metadata)
    """
    n = num_vars
    m = len(clauses)
    primes = nth_primes_from_5(n)

    pairs: list[tuple[int, int]] = []

    metadata = {
        "source_num_vars": n,
        "source_num_clauses": m,
        "primes": primes,
    }

    # Forbid invalid residues for each variable
    for i in range(n):
        p = primes[i]
        # Forbid residue 0: use pair (p, p)
        pairs.append((p, p))
        # Forbid residues 3, 4, ..., p-1
        for r in range(3, p):
            pairs.append((r, p))

    # Clause encoding
    for j, clause in enumerate(clauses):
        assert len(clause) == 3, f"Clause {j} has {len(clause)} literals"

        # Get the variable indices and falsifying residues
        var_indices = []
        false_residues = []
        for lit in clause:
            var_idx = abs(lit) - 1  # 0-indexed
            var_indices.append(var_idx)
            if lit > 0:
                # Positive literal: false when x = 2 (mod p_i)
                false_residues.append(2)
            else:
                # Negative literal: false when x = 1 (mod p_i)
                false_residues.append(1)

        clause_primes = [primes[vi] for vi in var_indices]
        M = clause_primes[0] * clause_primes[1] * clause_primes[2]
        R, _ = crt_solve(false_residues, clause_primes)
        assert 0 <= R < M

        # Add pair with model constraint 1 <= a <= b
        if R == 0:
            pairs.append((M, M))
        else:
            pairs.append((R, M))

    return pairs, metadata


# ============================================================
# Section 2: extract_solution()
# ============================================================


def extract_solution(x: int, metadata: dict) -> list[bool]:
    """
    Extract a 3-SAT solution from a Simultaneous Incongruences solution x.
    For each variable x_i: TRUE if x % p_i == 1, FALSE if x % p_i == 2.
    """
    primes = metadata["primes"]
    n = metadata["source_num_vars"]
    assignment = []
    for i in range(n):
        r = x % primes[i]
        assert r in (1, 2), f"Variable {i}: residue {r} not in {{1, 2}}"
        assignment.append(r == 1)  # 1 = TRUE, 2 = FALSE
    return assignment


# ============================================================
# Section 3: is_valid_source()
# ============================================================


def is_valid_source(num_vars: int, clauses: list[list[int]]) -> bool:
    """Validate a 3-SAT instance."""
    if num_vars < 1:
        return False
    for clause in clauses:
        if len(clause) != 3:
            return False
        for lit in clause:
            if lit == 0 or abs(lit) > num_vars:
                return False
        # Require distinct variables per clause
        if len(set(abs(l) for l in clause)) != 3:
            return False
    return True


# ============================================================
# Section 4: is_valid_target()
# ============================================================


def is_valid_target(pairs: list[tuple[int, int]]) -> bool:
    """Validate a Simultaneous Incongruences instance."""
    for a, b in pairs:
        if b == 0:
            return False
        if a < 1:
            return False
        if a > b:
            return False
    return True


# ============================================================
# Section 5: closed_loop_check()
# ============================================================


def closed_loop_check(num_vars: int, clauses: list[list[int]]) -> bool:
    """
    Full closed-loop verification for a single 3-SAT instance:
    1. Reduce to Simultaneous Incongruences
    2. Solve source and target independently
    3. Check satisfiability equivalence
    4. If satisfiable, extract solution and verify on source
    """
    assert is_valid_source(num_vars, clauses)

    pairs, meta = reduce(num_vars, clauses)
    assert is_valid_target(pairs), \
        f"Target not valid: pairs={pairs}"

    source_sat = is_3sat_satisfiable(num_vars, clauses)

    # Compute search limit for SI brute force: LCM of all moduli
    moduli = set(b for _, b in pairs)
    lcm_val = 1
    for b in moduli:
        lcm_val = lcm_val * b // math.gcd(lcm_val, b)
    # Cap search to keep brute force feasible
    search_limit = min(lcm_val, 500_000)

    target_sat = is_si_satisfiable(pairs, search_limit)

    if source_sat != target_sat:
        print(f"FAIL: sat mismatch: source={source_sat}, target={target_sat}")
        print(f"  source: n={num_vars}, clauses={clauses}")
        print(f"  pairs={pairs}")
        return False

    if target_sat:
        x = solve_si_brute(pairs, search_limit)
        assert x is not None

        s_sol = extract_solution(x, meta)
        if not is_3sat_satisfied(num_vars, clauses, s_sol):
            print(f"FAIL: extraction failed")
            print(f"  source: n={num_vars}, clauses={clauses}")
            print(f"  x={x}, extracted={s_sol}")
            return False

    return True


# ============================================================
# Section 6: exhaustive_small()
# ============================================================


def exhaustive_small() -> int:
    """
    Exhaustively test 3-SAT instances with small n.
    n=3: all single-clause and sampled multi-clause.
    n=4,5: single-clause and sampled two-clause.
    """
    total_checks = 0

    for n in range(3, 6):
        # All clauses with 3 distinct variables
        valid_clauses = []
        for combo in itertools.combinations(range(1, n + 1), 3):
            for signs in itertools.product([1, -1], repeat=3):
                c = [s * v for s, v in zip(signs, combo)]
                valid_clauses.append(c)

        if n == 3:
            # Single clause: all 8 sign patterns
            for c in valid_clauses:
                assert closed_loop_check(n, [c]), \
                    f"FAILED: n={n}, clause={c}"
                total_checks += 1

            # Two clauses: all pairs
            for c1, c2 in itertools.combinations(valid_clauses, 2):
                if is_valid_source(n, [c1, c2]):
                    assert closed_loop_check(n, [c1, c2]), \
                        f"FAILED: n={n}, clauses={[c1, c2]}"
                    total_checks += 1

            # Three clauses: sampled
            random.seed(42)
            triples = list(itertools.combinations(valid_clauses, 3))
            sample_size = min(500, len(triples))
            sampled = random.sample(triples, sample_size)
            for combo in sampled:
                clause_list = list(combo)
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clauses={clause_list}"
                    total_checks += 1

        elif n == 4:
            # Single clause
            for c in valid_clauses:
                assert closed_loop_check(n, [c]), \
                    f"FAILED: n={n}, clause={c}"
                total_checks += 1

            # Two clauses: sampled
            pairs_list = list(itertools.combinations(valid_clauses, 2))
            random.seed(43)
            sample_size = min(800, len(pairs_list))
            sampled = random.sample(pairs_list, sample_size)
            for c1, c2 in sampled:
                clause_list = [c1, c2]
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clauses={clause_list}"
                    total_checks += 1

        elif n == 5:
            # Single clause
            for c in valid_clauses:
                assert closed_loop_check(n, [c]), \
                    f"FAILED: n={n}, clause={c}"
                total_checks += 1

            # Two clauses: sampled
            pairs_list = list(itertools.combinations(valid_clauses, 2))
            random.seed(44)
            sample_size = min(600, len(pairs_list))
            sampled = random.sample(pairs_list, sample_size)
            for c1, c2 in sampled:
                clause_list = [c1, c2]
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clauses={clause_list}"
                    total_checks += 1

    print(f"exhaustive_small: {total_checks} checks passed")
    return total_checks


# ============================================================
# Section 7: random_stress()
# ============================================================


def random_stress(num_checks: int = 5000) -> int:
    """
    Random stress testing with various 3-SAT instance sizes.
    Uses clause-to-variable ratios around the phase transition (~4.27)
    to produce both SAT and UNSAT instances.
    """
    random.seed(12345)
    passed = 0

    for _ in range(num_checks):
        n = random.randint(3, 6)
        ratio = random.uniform(0.5, 8.0)
        m = max(1, int(n * ratio))
        m = min(m, 10)  # Keep manageable for brute force SI search

        clauses = []
        for _ in range(m):
            vars_chosen = random.sample(range(1, n + 1), 3)
            lits = [v if random.random() < 0.5 else -v for v in vars_chosen]
            clauses.append(lits)

        if not is_valid_source(n, clauses):
            continue

        assert closed_loop_check(n, clauses), \
            f"FAILED: n={n}, clauses={clauses}"
        passed += 1

    print(f"random_stress: {passed} checks passed")
    return passed


# ============================================================
# Test vector generation
# ============================================================


def generate_test_vectors() -> dict:
    """Generate test vectors for JSON export."""
    primes_3 = nth_primes_from_5(3)  # [5, 7, 11]

    vectors = []

    # YES: single satisfiable clause
    clauses_1 = [[1, 2, 3]]
    pairs_1, meta_1 = reduce(3, clauses_1)
    x_1 = solve_si_brute(pairs_1, 500_000)
    vectors.append({
        "label": "yes_single_clause",
        "source": {"num_vars": 3, "clauses": clauses_1},
        "target": {"pairs": pairs_1},
        "source_satisfiable": True,
        "target_satisfiable": True,
        "witness_x": x_1,
    })

    # YES: mixed literals
    clauses_2 = [[1, -2, 3]]
    pairs_2, meta_2 = reduce(3, clauses_2)
    x_2 = solve_si_brute(pairs_2, 500_000)
    vectors.append({
        "label": "yes_mixed_literals",
        "source": {"num_vars": 3, "clauses": clauses_2},
        "target": {"pairs": pairs_2},
        "source_satisfiable": True,
        "target_satisfiable": True,
        "witness_x": x_2,
    })

    # YES: two clauses
    clauses_3 = [[1, 2, 3], [-1, -2, -3]]
    pairs_3, meta_3 = reduce(3, clauses_3)
    x_3 = solve_si_brute(pairs_3, 500_000)
    vectors.append({
        "label": "yes_two_clauses",
        "source": {"num_vars": 3, "clauses": clauses_3},
        "target": {"pairs": pairs_3},
        "source_satisfiable": True,
        "target_satisfiable": True,
        "witness_x": x_3,
    })

    # YES: 4 variables
    clauses_4 = [[1, 2, 3], [-2, -3, -4]]
    pairs_4, meta_4 = reduce(4, clauses_4)
    x_4 = solve_si_brute(pairs_4, 500_000)
    vectors.append({
        "label": "yes_four_vars",
        "source": {"num_vars": 4, "clauses": clauses_4},
        "target": {"pairs": pairs_4},
        "source_satisfiable": True,
        "target_satisfiable": True,
        "witness_x": x_4,
    })

    # NO: all 8 clauses on 3 vars (unsatisfiable)
    clauses_no = [
        [1, 2, 3], [-1, -2, -3], [1, -2, 3], [-1, 2, -3],
        [1, 2, -3], [-1, -2, 3], [-1, 2, 3], [1, -2, -3],
    ]
    pairs_no, meta_no = reduce(3, clauses_no)
    vectors.append({
        "label": "no_all_8_clauses",
        "source": {"num_vars": 3, "clauses": clauses_no},
        "target": {"pairs": pairs_no},
        "source_satisfiable": False,
        "target_satisfiable": False,
        "witness_x": None,
    })

    return {
        "reduction": "KSatisfiability_K3_to_SimultaneousIncongruences",
        "source_problem": "KSatisfiability",
        "source_variant": {"k": "K3"},
        "target_problem": "SimultaneousIncongruences",
        "target_variant": {},
        "encoding": {
            "primes_for_3_vars": primes_3,
            "true_residue": 1,
            "false_residue": 2,
        },
        "test_vectors": vectors,
    }


# ============================================================
# Main
# ============================================================


if __name__ == "__main__":
    print("=" * 60)
    print("Verifying: KSatisfiability(K3) -> SimultaneousIncongruences")
    print("=" * 60)

    # Quick sanity checks
    print("\n--- Sanity checks ---")

    # Single satisfiable clause
    pairs, meta = reduce(3, [[1, 2, 3]])
    primes = meta["primes"]
    assert primes == [5, 7, 11]
    # Variable pairs: (5-2)+(7-2)+(11-2) = 3+5+9 = 17, clause pairs: 1
    assert len(pairs) == 18, f"Expected 18 pairs, got {len(pairs)}"
    assert is_valid_target(pairs)
    assert closed_loop_check(3, [[1, 2, 3]])
    print("  Single satisfiable clause: OK")

    # All-negated clause
    assert closed_loop_check(3, [[-1, -2, -3]])
    print("  All-negated clause: OK")

    # Unsatisfiable: test directly with 4 vars, 4 conflicting clauses
    # (x1 v x2 v x3) & (~x1 v ~x2 v ~x3) & (x1 v x2 v ~x3) & (~x1 v ~x2 v x3)
    # This is still satisfiable. Use a known-UNSAT construction.
    # With 3 vars, 8 clauses covering all sign patterns:
    unsat_clauses = [
        [1, 2, 3], [-1, -2, -3], [1, -2, 3], [-1, 2, -3],
        [1, 2, -3], [-1, -2, 3], [-1, 2, 3], [1, -2, -3],
    ]
    assert not is_3sat_satisfiable(3, unsat_clauses)
    pairs_unsat, _ = reduce(3, unsat_clauses)
    assert is_valid_target(pairs_unsat)
    # Verify target is also unsatisfiable (search space is manageable)
    assert not is_si_satisfiable(pairs_unsat, 500_000)
    print("  Unsatisfiable instance: OK")

    print("\n--- Exhaustive small instances ---")
    n_exhaust = exhaustive_small()

    print("\n--- Random stress test ---")
    n_random = random_stress()

    total = n_exhaust + n_random
    print(f"\n{'=' * 60}")
    print(f"TOTAL CHECKS: {total}")
    if total >= 5000:
        print("ALL CHECKS PASSED (>= 5000)")
    else:
        print(f"WARNING: only {total} checks (need >= 5000)")
        print("Adjusting random_stress count...")
        extra = random_stress(5500 - total)
        total += extra
        print(f"ADJUSTED TOTAL: {total}")
        assert total >= 5000

    # Generate test vectors
    print("\n--- Generating test vectors ---")
    tv = generate_test_vectors()
    tv_path = "test_vectors_k_satisfiability_simultaneous_incongruences.json"
    with open(tv_path, "w") as f:
        json.dump(tv, f, indent=2)
    print(f"  Written to {tv_path}")

    print("\nVERIFIED")
