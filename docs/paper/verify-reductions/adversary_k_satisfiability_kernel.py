#!/usr/bin/env python3
"""
Adversary verification script for KSatisfiability(K3) -> Kernel reduction.
Issue #882 — Chvatal (1973).

Independent implementation based solely on the Typst proof document.
Does NOT import from the constructor script.

Requirements: >= 5000 checks, hypothesis PBT with >= 2 strategies.
"""

import itertools
import json
import random
from pathlib import Path

# ---------------------------------------------------------------------------
# Independent reduction implementation (from Typst proof only)
# ---------------------------------------------------------------------------

def reduce(n, clauses):
    """
    Reduce a 3-SAT instance to a Kernel directed graph.

    From the Typst proof:
    - Step 1: For each variable u_i (i=1..n), create vertices x_i (index 2*(i-1))
      and x_bar_i (index 2*(i-1)+1). Add digon arcs (x_i, x_bar_i) and (x_bar_i, x_i).
    - Step 2: For each clause C_j (j=1..m), create vertices c_{j,1}, c_{j,2}, c_{j,3}
      at indices 2n + 3*(j-1), 2n+3*(j-1)+1, 2n+3*(j-1)+2.
      Add triangle arcs: c_{j,1}->c_{j,2}->c_{j,3}->c_{j,1}.
    - Step 3: For each clause C_j and each literal l_k in C_j (k=1,2,3),
      add arcs from ALL THREE clause vertices to the literal vertex.
    """
    m = len(clauses)
    num_vertices = 2 * n + 3 * m
    arcs = []

    # Step 1: Variable digons
    for i in range(n):
        xi = 2 * i
        xi_bar = 2 * i + 1
        arcs.append((xi, xi_bar))
        arcs.append((xi_bar, xi))

    # Step 2 + 3: Clause gadgets + connections
    for j in range(m):
        base = 2 * n + 3 * j
        # Triangle
        arcs.append((base, base + 1))
        arcs.append((base + 1, base + 2))
        arcs.append((base + 2, base))

        # Connection arcs
        for lit in clauses[j]:
            var_idx = abs(lit) - 1
            if lit > 0:
                target = 2 * var_idx
            else:
                target = 2 * var_idx + 1
            for t in range(3):
                arcs.append((base + t, target))

    return num_vertices, arcs


def is_feasible_source(n, clauses):
    """Check if a 3-SAT formula is satisfiable (brute force)."""
    for bits in range(1 << n):
        a = [(bits >> i) & 1 == 1 for i in range(n)]
        ok = True
        for clause in clauses:
            clause_sat = False
            for lit in clause:
                var = abs(lit) - 1
                if (lit > 0 and a[var]) or (lit < 0 and not a[var]):
                    clause_sat = True
                    break
            if not clause_sat:
                ok = False
                break
        if ok:
            return True, a
    return False, None


def is_feasible_target(nv, arcs, selected):
    """Check if `selected` is a kernel of the directed graph."""
    # Build adjacency
    succ = [[] for _ in range(nv)]
    for (u, v) in arcs:
        succ[u].append(v)

    for u in range(nv):
        if u in selected:
            # Independence
            for v in succ[u]:
                if v in selected:
                    return False
        else:
            # Absorption
            if not any(v in selected for v in succ[u]):
                return False
    return True


def find_kernel_brute_force(nv, arcs):
    """Find any kernel by brute force (for small graphs)."""
    for bits in range(1 << nv):
        sel = {v for v in range(nv) if (bits >> v) & 1}
        if is_feasible_target(nv, arcs, sel):
            return True, sel
    return False, None


def find_kernel_structural(n, clauses, nv, arcs):
    """Find kernel by only checking literal-vertex subsets (from proof)."""
    succ = [[] for _ in range(nv)]
    for (u, v) in arcs:
        succ[u].append(v)

    for bits in range(1 << n):
        sel = set()
        for i in range(n):
            if (bits >> i) & 1:
                sel.add(2 * i)
            else:
                sel.add(2 * i + 1)

        # Check kernel properties
        valid = True
        for u in range(nv):
            if u in sel:
                for v in succ[u]:
                    if v in sel:
                        valid = False
                        break
                if not valid:
                    break
            else:
                if not any(v in sel for v in succ[u]):
                    valid = False
                    break
        if valid:
            return True, sel

    return False, None


def extract_solution(n, kernel_set):
    """Extract boolean assignment from kernel."""
    assignment = []
    for i in range(n):
        if 2 * i in kernel_set:
            assignment.append(True)
        elif 2 * i + 1 in kernel_set:
            assignment.append(False)
        else:
            raise ValueError(f"Neither x_{i} nor x_bar_{i} in kernel")
    return assignment


# ---------------------------------------------------------------------------
# Random instance generators
# ---------------------------------------------------------------------------

def random_3sat(n, m, rng=None):
    """Generate random 3-SAT instance."""
    if rng is None:
        rng = random
    clauses = []
    for _ in range(m):
        vars_chosen = rng.sample(range(1, n + 1), 3)
        clause = [v if rng.random() < 0.5 else -v for v in vars_chosen]
        clauses.append(clause)
    return clauses


# ---------------------------------------------------------------------------
# Tests
# ---------------------------------------------------------------------------

total_checks = 0


def check(condition, msg=""):
    global total_checks
    assert condition, msg
    total_checks += 1


def test_yes_example():
    """Reproduce the YES example from the Typst proof."""
    global total_checks
    n = 3
    clauses = [[1, 2, 3], [-1, -2, 3]]

    nv, arcs = reduce(n, clauses)
    check(nv == 12, f"YES: expected 12 vertices, got {nv}")
    check(len(arcs) == 30, f"YES: expected 30 arcs, got {len(arcs)}")

    # Kernel from proof: S = {0, 3, 4} = {x1, x_bar_2, x3}
    S = {0, 3, 4}
    check(is_feasible_target(nv, arcs, S), "YES kernel must be valid")

    extracted = extract_solution(n, S)
    check(extracted == [True, False, True], f"YES extraction: got {extracted}")

    sat, _ = is_feasible_source(n, clauses)
    check(sat, "YES instance must be satisfiable")

    has_k, _ = find_kernel_brute_force(nv, arcs)
    check(has_k, "YES graph must have kernel")

    print(f"  YES example: {total_checks} checks so far")


def test_no_example():
    """Reproduce the NO example from the Typst proof."""
    global total_checks
    n = 3
    clauses = [
        [1, 2, 3], [1, 2, -3], [1, -2, 3], [1, -2, -3],
        [-1, 2, 3], [-1, 2, -3], [-1, -2, 3], [-1, -2, -3],
    ]

    sat, _ = is_feasible_source(n, clauses)
    check(not sat, "NO instance must be unsatisfiable")

    nv, arcs = reduce(n, clauses)
    check(nv == 30, f"NO: expected 30 vertices, got {nv}")
    check(len(arcs) == 102, f"NO: expected 102 arcs, got {len(arcs)}")

    has_k, _ = find_kernel_structural(n, clauses, nv, arcs)
    check(not has_k, "NO graph must NOT have kernel")

    # Explicit check: alpha=(T,T,T), S={0,2,4}, clause 8 vertex c_{8,1}=27
    S_ttt = {0, 2, 4}
    check(not is_feasible_target(nv, arcs, S_ttt), "TTT candidate fails")

    # c_{8,1} at index 27 has successors {28, 1, 3, 5}
    succs_27 = {v for (u, v) in arcs if u == 27}
    check(28 in succs_27, "c81 -> c82")
    check(1 in succs_27, "c81 -> x_bar_1")
    check(3 in succs_27, "c81 -> x_bar_2")
    check(5 in succs_27, "c81 -> x_bar_3")
    for v in succs_27:
        check(v not in S_ttt, f"Vertex {v} should not be in TTT candidate")

    print(f"  NO example: {total_checks} checks so far")


def test_exhaustive_forward_backward():
    """Exhaustive forward/backward check for small instances."""
    global total_checks

    rng = random.Random(123)

    # All single-clause instances for n=3
    lits = [1, 2, 3, -1, -2, -3]
    all_clauses_3 = []
    for combo in itertools.combinations(lits, 3):
        if len(set(abs(l) for l in combo)) == 3:
            all_clauses_3.append(list(combo))

    for clause in all_clauses_3:
        sat, _ = is_feasible_source(3, [clause])
        nv, arcs = reduce(3, [clause])
        has_k, _ = find_kernel_brute_force(nv, arcs)
        check(sat == has_k, f"Mismatch for clause {clause}")

    # All pairs of clauses for n=3
    for c1 in all_clauses_3:
        for c2 in all_clauses_3:
            sat, _ = is_feasible_source(3, [c1, c2])
            nv, arcs = reduce(3, [c1, c2])
            has_k, _ = find_kernel_brute_force(nv, arcs)
            check(sat == has_k, f"Mismatch for clauses {[c1, c2]}")

    # Random instances for n=3..5, various m
    for n in range(3, 6):
        for m in range(1, 8):
            num = 100 if n <= 4 else 50
            for _ in range(num):
                clauses = random_3sat(n, m, rng)
                sat, _ = is_feasible_source(n, clauses)
                nv, arcs = reduce(n, clauses)
                if nv <= 20:
                    has_k, _ = find_kernel_brute_force(nv, arcs)
                else:
                    has_k, _ = find_kernel_structural(n, clauses, nv, arcs)
                check(sat == has_k, f"Mismatch n={n} m={m}")

    print(f"  Exhaustive forward/backward: {total_checks} checks so far")


def test_extraction():
    """Verify solution extraction for all feasible instances."""
    global total_checks
    rng = random.Random(456)

    for n in range(3, 6):
        for m in range(1, 7):
            for _ in range(80):
                clauses = random_3sat(n, m, rng)
                sat, _ = is_feasible_source(n, clauses)
                if not sat:
                    continue

                nv, arcs = reduce(n, clauses)
                if nv <= 20:
                    has_k, kernel = find_kernel_brute_force(nv, arcs)
                else:
                    has_k, kernel = find_kernel_structural(n, clauses, nv, arcs)
                check(has_k)

                assignment = extract_solution(n, kernel)
                # Verify assignment satisfies formula
                for clause in clauses:
                    clause_sat = any(
                        (assignment[abs(l) - 1] if l > 0 else not assignment[abs(l) - 1])
                        for l in clause
                    )
                    check(clause_sat, f"Clause {clause} not satisfied by {assignment}")

                # Verify kernel has exactly one literal per variable
                for i in range(n):
                    check((2 * i in kernel) != (2 * i + 1 in kernel))

    print(f"  Extraction: {total_checks} checks so far")


def test_overhead():
    """Verify overhead formulas."""
    global total_checks
    rng = random.Random(789)

    for n in range(3, 10):
        for m in range(1, 12):
            for _ in range(15):
                clauses = random_3sat(n, m, rng)
                nv, arcs = reduce(n, clauses)
                check(nv == 2 * n + 3 * m, f"Vertex overhead: {nv} != {2*n+3*m}")
                check(len(arcs) == 2 * n + 12 * m, f"Arc overhead: {len(arcs)} != {2*n+12*m}")

    print(f"  Overhead: {total_checks} checks so far")


def test_structural_properties():
    """Verify gadget structure invariants."""
    global total_checks
    rng = random.Random(321)

    for n in range(3, 6):
        for m in range(1, 6):
            for _ in range(30):
                clauses = random_3sat(n, m, rng)
                nv, arcs = reduce(n, clauses)
                arc_set = set(arcs)

                # Digons
                for i in range(n):
                    check((2 * i, 2 * i + 1) in arc_set, f"Missing digon {i}")
                    check((2 * i + 1, 2 * i) in arc_set, f"Missing digon {i} reverse")

                # Triangles
                for j in range(m):
                    b = 2 * n + 3 * j
                    check((b, b + 1) in arc_set)
                    check((b + 1, b + 2) in arc_set)
                    check((b + 2, b) in arc_set)

                # Connections
                for j, clause in enumerate(clauses):
                    b = 2 * n + 3 * j
                    for lit in clause:
                        v = 2 * (abs(lit) - 1) + (0 if lit > 0 else 1)
                        for t in range(3):
                            check((b + t, v) in arc_set)

                # No self-loops
                for (u, v) in arcs:
                    check(u != v, f"Self-loop at {u}")

    print(f"  Structural: {total_checks} checks so far")


def test_hypothesis_pbt():
    """Property-based testing using hypothesis."""
    from hypothesis import given, settings, HealthCheck
    from hypothesis import strategies as st

    counter = {"n": 0}

    # Strategy 1: Random 3-SAT instances with n=3..5, m=1..6
    @given(
        n=st.integers(min_value=3, max_value=5),
        m=st.integers(min_value=1, max_value=6),
        seed=st.integers(min_value=0, max_value=10000),
    )
    @settings(max_examples=2000, suppress_health_check=[HealthCheck.too_slow])
    def strategy_1_random(n, m, seed):
        rng = random.Random(seed)
        clauses = random_3sat(n, m, rng)
        sat, _ = is_feasible_source(n, clauses)
        nv, arcs = reduce(n, clauses)

        # Check overhead
        assert nv == 2 * n + 3 * m
        assert len(arcs) == 2 * n + 12 * m

        # Check equivalence
        if nv <= 20:
            has_k, kernel = find_kernel_brute_force(nv, arcs)
        else:
            has_k, kernel = find_kernel_structural(n, clauses, nv, arcs)
        assert sat == has_k, f"sat={sat} kernel={has_k} n={n} m={m}"

        # If feasible, check extraction
        if sat and kernel:
            assignment = extract_solution(n, kernel)
            for clause in clauses:
                assert any(
                    (assignment[abs(l) - 1] if l > 0 else not assignment[abs(l) - 1])
                    for l in clause
                )

        counter["n"] += 1

    # Strategy 2: Specific clause patterns (edge cases)
    @given(
        signs=st.lists(
            st.lists(st.booleans(), min_size=3, max_size=3),
            min_size=1,
            max_size=5,
        ),
    )
    @settings(max_examples=2000, suppress_health_check=[HealthCheck.too_slow])
    def strategy_2_patterns(signs):
        n = 3
        clauses = []
        for sign_list in signs:
            clause = []
            for i, positive in enumerate(sign_list):
                clause.append(i + 1 if positive else -(i + 1))
            clauses.append(clause)

        sat, _ = is_feasible_source(n, clauses)
        nv, arcs = reduce(n, clauses)
        m = len(clauses)

        assert nv == 2 * n + 3 * m
        assert len(arcs) == 2 * n + 12 * m

        if nv <= 20:
            has_k, _ = find_kernel_brute_force(nv, arcs)
        else:
            has_k, _ = find_kernel_structural(n, clauses, nv, arcs)
        assert sat == has_k

        counter["n"] += 1

    print("  Running hypothesis strategy 1 (random instances)...")
    strategy_1_random()
    print(f"    Strategy 1: {counter['n']} examples tested")

    s1_count = counter["n"]
    print("  Running hypothesis strategy 2 (sign patterns)...")
    strategy_2_patterns()
    print(f"    Strategy 2: {counter['n'] - s1_count} examples tested")

    return counter["n"]


def test_cross_comparison():
    """Compare outputs with constructor script's test vectors."""
    global total_checks

    vec_path = Path(__file__).parent / "test_vectors_k_satisfiability_kernel.json"
    if not vec_path.exists():
        print("  Cross-comparison: SKIPPED (no test vectors file)")
        return

    with open(vec_path) as f:
        vectors = json.load(f)

    # YES instance
    yi = vectors["yes_instance"]
    n_yes = yi["input"]["num_vars"]
    clauses_yes = yi["input"]["clauses"]
    nv, arcs = reduce(n_yes, clauses_yes)
    check(nv == yi["output"]["num_vertices"], "YES vertices match")
    check(sorted(arcs) == sorted(tuple(a) for a in yi["output"]["arcs"]), "YES arcs match")

    sat, _ = is_feasible_source(n_yes, clauses_yes)
    check(sat == yi["source_feasible"], "YES source feasibility matches")

    has_k, kernel = find_kernel_brute_force(nv, arcs)
    check(has_k == yi["target_feasible"], "YES target feasibility matches")

    # NO instance
    ni = vectors["no_instance"]
    n_no = ni["input"]["num_vars"]
    clauses_no = ni["input"]["clauses"]
    nv_no, arcs_no = reduce(n_no, clauses_no)
    check(nv_no == ni["output"]["num_vertices"], "NO vertices match")
    check(sorted(arcs_no) == sorted(tuple(a) for a in ni["output"]["arcs"]), "NO arcs match")

    sat_no, _ = is_feasible_source(n_no, clauses_no)
    check(not sat_no == (not ni["source_feasible"]), "NO source feasibility matches")

    has_k_no, _ = find_kernel_structural(n_no, clauses_no, nv_no, arcs_no)
    check(has_k_no == ni["target_feasible"], "NO target feasibility matches")

    # Verify all claims
    for claim in vectors["claims"]:
        check(claim["verified"], f"Claim {claim['tag']} not verified")

    print(f"  Cross-comparison: {total_checks} checks so far")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    global total_checks

    print("=== Adversary: KSatisfiability(K3) -> Kernel ===")
    print("=== Issue #882 — Chvatal (1973) ===\n")

    test_yes_example()
    test_no_example()
    test_exhaustive_forward_backward()
    test_extraction()
    test_overhead()
    test_structural_properties()

    # Hypothesis PBT
    pbt_count = test_hypothesis_pbt()
    total_checks += pbt_count

    test_cross_comparison()

    print(f"\n=== TOTAL ADVERSARY CHECKS: {total_checks} ===")
    assert total_checks >= 5000, f"Need >= 5000, got {total_checks}"
    print("ALL ADVERSARY CHECKS PASSED")


if __name__ == "__main__":
    main()
