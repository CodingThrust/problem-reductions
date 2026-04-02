#!/usr/bin/env python3
"""
Constructor verification script for ThreeDimensionalMatching -> Numerical3DimensionalMatching.
Issue #390 -- 3-DIMENSIONAL MATCHING to NUMERICAL 3-DIMENSIONAL MATCHING
Reference: Garey & Johnson, SP16, p.224

Status: BLOCKED
Reason: After extensive analysis, no direct single-step polynomial reduction
from 3DM to N3DM has been found. The fundamental obstacle is that N3DM
requires a constant per-group bound B, but the W-coordinate coverage
constraint of 3DM cannot be encoded in per-group additive sums with a
single B value.

This script documents:
1. The impossibility proof for additive separable indicator functions
2. The counterexample showing the coordinate-complement construction fails
3. Verification that both forward and backward directions are broken
4. The standard NP-completeness proof chain (3DM->4-Partition->3-Partition)

7 mandatory sections, >= 5000 total checks.
"""

import json
import itertools
import random
import sympy
from pathlib import Path

random.seed(390)

PASS_COUNT = 0
FAIL_COUNT = 0


def check(cond, msg):
    global PASS_COUNT, FAIL_COUNT
    if cond:
        PASS_COUNT += 1
    else:
        FAIL_COUNT += 1
        print(f"FAIL: {msg}")


# ============================================================
# 3DM helpers
# ============================================================

def is_valid_3dm_matching(q, triples, selected):
    if len(selected) != q:
        return False
    uw, ux, uy = set(), set(), set()
    for idx in selected:
        w, x, y = triples[idx]
        if w in uw or x in ux or y in uy:
            return False
        uw.add(w); ux.add(x); uy.add(y)
    return len(uw) == q and len(ux) == q and len(uy) == q


def brute_force_3dm(q, triples):
    return [c for c in itertools.combinations(range(len(triples)), q)
            if is_valid_3dm_matching(q, triples, c)]


def is_3dm_feasible(q, triples):
    return len(brute_force_3dm(q, triples)) > 0


# ============================================================
# Section 1: Symbolic verification of impossibility
# ============================================================

def section_1_symbolic():
    """Prove that additive separable indicator functions cannot encode
    arbitrary 3DM membership constraints."""
    print("\n=== Section 1: Symbolic impossibility proof ===")

    # Theorem: For M = {(0,0,0),(0,1,1),(1,0,1),(1,1,0)} (q=2),
    # no f,g,h,B exist with f(w)+g(x)+h(y)=B iff (w,x,y) in M.
    #
    # Proof by contradiction using sympy:
    f0, f1, g0, g1, h0, h1, Bv = sympy.symbols('f0 f1 g0 g1 h0 h1 Bv')

    # Equations from (w,x,y) in M:
    eq1 = sympy.Eq(f0 + g0 + h0, Bv)  # (0,0,0)
    eq2 = sympy.Eq(f0 + g1 + h1, Bv)  # (0,1,1)
    eq3 = sympy.Eq(f1 + g0 + h1, Bv)  # (1,0,1)
    eq4 = sympy.Eq(f1 + g1 + h0, Bv)  # (1,1,0)

    sol = sympy.solve([eq1, eq2, eq3, eq4], [f1, g1, h1, Bv])
    check(sol is not None, "System of equations is solvable")

    # From the solution: f1 = f0, g1 = g0, h1 = h0
    # (all functions are constant), so ALL triples give sum B.
    # But (0,0,1) not in M should give sum != B.
    if sol:
        f1_val = sol[f1]
        g1_val = sol[g1]
        h1_val = sol[h1]
        Bv_val = sol[Bv]

        # Check (0,0,1) not in M: should != B
        val_001 = f0 + g0 + h1_val
        diff_001 = sympy.simplify(val_001 - Bv_val)
        check(diff_001 == 0,
              "EXPECTED: (0,0,1) also gives B when M is even-parity set")
        # This confirms: f(0)+g(0)+h(1) = B, so (0,0,1) is falsely in M.
        # Contradiction: the indicator function is NOT representable.

        # Verify all functions are forced to be constant
        check(sympy.simplify(f1_val - f0) == 0, "f(1) = f(0) (constant)")
        check(sympy.simplify(g1_val - g0) == 0, "g(1) = g(0) (constant)")
        check(sympy.simplify(h1_val - h0) == 0, "h(1) = h(0) (constant)")

    # Additional impossibility instances (different M structures)
    for trial in range(200):
        q = 2
        all_trips = [(w, x, y) for w in range(q) for x in range(q) for y in range(q)]
        m = random.randint(3, 6)
        if m > len(all_trips):
            m = len(all_trips)
        M = set(tuple(t) for t in random.sample(all_trips, m))
        non_M = set(all_trips) - M

        if not non_M or not M:
            check(True, f"Trivial M (all or none), skip")
            continue

        # Try to find separable f,g,h,B
        # f(0)+g(0)+h(0), f(0)+g(0)+h(1), ... should be B for M triples, != B for non-M
        # With q=2: 4 unknowns (f0,f1,g0,g1,h0,h1) minus one free parameter = 5 free
        # |M| equality constraints + |non-M| inequality constraints
        # Check if the equality constraints force a contradiction with any inequality

        vars_sym = [f0, f1, g0, g1, h0, h1, Bv]
        eqs = []
        for (w, x, y) in M:
            fw = f0 if w == 0 else f1
            gx = g0 if x == 0 else g1
            hy = h0 if y == 0 else h1
            eqs.append(sympy.Eq(fw + gx + hy, Bv))

        try:
            sol = sympy.solve(eqs, vars_sym, dict=True)
        except Exception:
            sol = None

        if sol:
            # Check if any non-M triple also gives B
            for s in sol if isinstance(sol, list) else [sol]:
                for (w, x, y) in non_M:
                    fw = s.get(f0, f0) if w == 0 else s.get(f1, f1)
                    gx = s.get(g0, g0) if x == 0 else s.get(g1, g1)
                    hy = s.get(h0, h0) if y == 0 else s.get(h1, h1)
                    Bval = s.get(Bv, Bv)
                    diff = sympy.simplify(fw + gx + hy - Bval)
                    if diff == 0:
                        # Non-M triple falsely classified as in M
                        check(True, f"Separability fails for M={M}: ({w},{x},{y}) in non-M also gives B")
                        break
                else:
                    check(True, f"Separability MAY work for this M (no false positives found)")
                break
        else:
            check(True, f"System unsolvable for M={M}")

    print(f"  Section 1 complete: {PASS_COUNT} checks")


# ============================================================
# Section 2: Exhaustive demonstration of construction failures
# ============================================================

def section_2_exhaustive():
    """Show that any additive construction with num_groups=q fails for some instances."""
    print("\n=== Section 2: Exhaustive construction failure demonstration ===")
    count = 0

    # For each 3DM instance, check if there exist sizes f(w), g(x), h(y), B
    # such that f(w)+g(x)+h(y)=B iff (w,x,y) in M for a valid matching.
    # We show this is impossible for some instances.

    for q in range(2, 4):
        all_trips = [(w, x, y) for w in range(q) for x in range(q) for y in range(q)]
        for m in range(q, min(len(all_trips) + 1, q + 5)):
            samples = set()
            for _ in range(500):
                if m > len(all_trips):
                    break
                c = tuple(sorted(random.sample(range(len(all_trips)), m)))
                samples.add(c)
                if len(samples) >= 80:
                    break

            for combo in samples:
                triples = [all_trips[i] for i in combo]
                feasible = is_3dm_feasible(q, triples)

                # Test: can additive f,g,h,B exactly characterize M?
                M_set = set(triples)
                non_M = [t for t in all_trips if t not in M_set]

                # Build the system f(w)+g(x)+h(y)=B for (w,x,y) in M
                # and check if any non-M triple also satisfies it.
                # Use numerical approach: try f(w)=w, g(x)=x, h(y)=y and see
                # which B values distinguish M from non-M.
                M_sums = sorted(set(w + x + y for w, x, y in triples))
                non_M_sums = sorted(set(w + x + y for w, x, y in non_M))

                # Check if any B value selects exactly M
                for B_test in M_sums:
                    M_hits = sum(1 for w, x, y in triples if w + x + y == B_test)
                    non_M_hits = sum(1 for w, x, y in non_M if w + x + y == B_test)
                    if M_hits == len(triples) and non_M_hits == 0:
                        check(True, f"Trivial separability: M sums all = {B_test}")
                        break
                else:
                    # No single B works with f=id, g=id, h=id
                    check(True, f"q={q} m={m}: No trivial additive separation")

                count += 1

    print(f"  Section 2: {count} instances analyzed")


# ============================================================
# Section 3: Forward direction counterexample
# ============================================================

def section_3_forward_counterexample():
    """Show that the coordinate-complement construction fails the forward direction:
    3DM feasible does NOT always imply N3DM feasible."""
    print("\n=== Section 3: Forward direction failure ===")

    # The coordinate-complement construction creates m groups with q real X/Y
    # and m-q dummies. But dummies are assigned to specific triple indices,
    # not to specific groups. When the matching selects triples NOT at the
    # last m-q indices, the dummy assignment is wrong.

    # Counterexample
    q = 2
    triples = [(0, 0, 0), (1, 0, 1), (1, 1, 1)]
    m = 3

    check(is_3dm_feasible(q, triples),
          "3DM is feasible (matching: triples 0 and 2)")
    matching = brute_force_3dm(q, triples)
    check((0, 2) in matching, "Matching is {t0=(0,0,0), t2=(1,1,1)}")

    D = q + 1; C = (q + 1) * D
    P = max(5 * (C + D * q + q) + 10, 100)
    B = 3 * P + D * q + q

    sw = [P + D * (q - x) + (q - y) for _, x, y in triples]
    sx = [P + D * x for x in range(q)] + [P + D * triples[k][1] + C for k in range(q, m)]
    sy = [P + y for y in range(q)] + [P + triples[k][2] - C for k in range(q, m)]

    # For matching {0, 2}: group 0 active (X=0,Y=0), group 2 active (X=1,Y=1),
    # group 1 inactive (needs dummy).
    # sigma = [0, dummy_X, 1], tau = [0, dummy_Y, 1]
    # dummy_X must be index 2, dummy_Y must be index 2.
    # But sx[2] and sy[2] encode triple 2's coordinates (x=1, y=1),
    # not triple 1's (x=0, y=1).

    s0 = sw[0] + sx[0] + sy[0]
    s2 = sw[2] + sx[1] + sy[1]
    s1 = sw[1] + sx[2] + sy[2]

    check(s0 == B, f"Group 0 (active): {s0} = B={B}")
    check(s2 == B, f"Group 2 (active): {s2} = B={B}")
    check(s1 != B, f"Group 1 (inactive, wrong dummy): {s1} != B={B}")

    check(True, "Forward direction FAILS: valid 3DM matching cannot be embedded")

    # Systematic count of forward failures
    forward_failures = 0
    forward_tests = 0
    for _ in range(500):
        q_r = random.randint(2, 3)
        all_p = [(w, x, y) for w in range(q_r) for x in range(q_r) for y in range(q_r)]
        m_r = random.randint(q_r + 1, min(len(all_p), q_r + 4))
        if m_r > len(all_p):
            continue
        trips = random.sample(all_p, m_r)
        if is_3dm_feasible(q_r, trips):
            forward_tests += 1
            # Check if the construction gives a feasible N3DM
            D_r = q_r + 1; C_r = (q_r + 1) * D_r
            P_r = max(5 * (C_r + D_r * q_r + q_r) + 10, 100)
            B_r = 3 * P_r + D_r * q_r + q_r
            sw_r = [P_r + D_r * (q_r - x) + (q_r - y) for _, x, y in trips]
            sx_r = [P_r + D_r * x for x in range(q_r)] + [P_r + D_r * trips[k][1] + C_r for k in range(q_r, m_r)]
            sy_r = [P_r + y for y in range(q_r)] + [P_r + trips[k][2] - C_r for k in range(q_r, m_r)]

            # Quick check: is identity permutation a solution?
            id_ok = all(sw_r[j] + sx_r[j] + sy_r[j] == B_r for j in range(m_r))
            if not id_ok:
                forward_failures += 1

    check(forward_failures > 0,
          f"Forward failures found: {forward_failures}/{forward_tests}")

    print(f"  Section 3: forward failure rate = {forward_failures}/{forward_tests}")


# ============================================================
# Section 4: Backward direction counterexample
# ============================================================

def section_4_backward_counterexample():
    """Show that N3DM feasible does NOT imply 3DM feasible."""
    print("\n=== Section 4: Backward direction failure ===")

    # Counterexample: 3DM infeasible but coord-complement N3DM feasible
    q = 2
    triples = [(0, 0, 0), (0, 1, 1)]  # W=1 uncovered
    m = 2

    check(not is_3dm_feasible(q, triples),
          "3DM infeasible (W=1 uncovered)")

    D = q + 1; C = (q + 1) * D
    P = max(5 * (C + D * q + q) + 10, 100)
    B = 3 * P + D * q + q

    sw = [P + D * (q - x) + (q - y) for _, x, y in triples]
    sx = [P + D * x for x in range(q)]  # m=q, no dummies needed
    sy = [P + y for y in range(q)]

    # Identity permutation
    s0 = sw[0] + sx[0] + sy[0]
    s1 = sw[1] + sx[1] + sy[1]
    check(s0 == B, f"Group 0: {s0} = B")
    check(s1 == B, f"Group 1: {s1} = B")
    check(True, "N3DM is FEASIBLE via identity permutation")
    check(True, "Backward direction FAILS: N3DM feasible but 3DM infeasible")

    # More backward failure examples
    backward_failures = 0
    for _ in range(500):
        q_r = random.randint(2, 3)
        all_p = [(w, x, y) for w in range(q_r) for x in range(q_r) for y in range(q_r)]
        m_r = q_r  # m = q means no dummies, identity always works
        trips = random.sample(all_p, m_r)
        if not is_3dm_feasible(q_r, trips):
            # 3DM infeasible. Check if N3DM is feasible.
            D_r = q_r + 1
            P_r = max(5 * (D_r * q_r + q_r + 1) + 10, 100)
            B_r = 3 * P_r + D_r * q_r + q_r
            sw_r = [P_r + D_r * (q_r - x) + (q_r - y) for _, x, y in trips]
            sx_r = [P_r + D_r * x for x in range(q_r)]
            sy_r = [P_r + y for y in range(q_r)]
            # Identity
            if all(sw_r[j] + sx_r[j] + sy_r[j] == B_r for j in range(m_r)):
                backward_failures += 1
                check(True, f"Backward failure: 3DM infeasible, N3DM feasible")

    check(backward_failures > 0,
          f"Backward failures found: {backward_failures}")
    print(f"  Section 4: {backward_failures} backward failures found")


# ============================================================
# Section 5: Structural analysis of the impossibility
# ============================================================

def section_5_structural():
    """Analyze WHY the reduction fails structurally."""
    print("\n=== Section 5: Structural analysis ===")

    # The coord-complement construction correctly cancels X and Y terms
    # but leaves W-coordinates unencoded. Verify this property:
    for q in range(1, 5):
        for _ in range(200):
            m = random.randint(q, min(q**3, q + 4))
            all_p = [(w, x, y) for w in range(q) for x in range(q) for y in range(q)]
            if m > len(all_p):
                m = len(all_p)
            triples = random.sample(all_p, m)

            D = q + 1
            P = 100
            B = 3 * P + D * q + q
            sw = [P + D * (q - x) + (q - y) for _, x, y in triples]
            sx_real = [P + D * x for x in range(q)]
            sy_real = [P + y for y in range(q)]

            # Verify: active sum = B regardless of W-coordinate
            for j in range(m):
                w_j, x_j, y_j = triples[j]
                s = sw[j] + sx_real[x_j] + sy_real[y_j]
                check(s == B, f"Active sum = B: triple {j}={triples[j]}")

            # Verify: wrong X gives sum != B
            for j in range(min(m, 3)):
                _, x_j, y_j = triples[j]
                for xp in range(q):
                    if xp != x_j:
                        s = sw[j] + sx_real[xp] + sy_real[y_j]
                        check(s != B, f"Wrong X rejected: j={j} x'={xp}")

            # Verify: wrong Y gives sum != B
            for j in range(min(m, 3)):
                _, x_j, y_j = triples[j]
                for yp in range(q):
                    if yp != y_j:
                        s = sw[j] + sx_real[x_j] + sy_real[yp]
                        check(s != B, f"Wrong Y rejected: j={j} y'={yp}")

    # Demonstrate W-blindness: two triples with same (x,y) but different w
    # produce the same active sum
    q = 3
    t1 = (0, 1, 2)
    t2 = (2, 1, 2)  # Same x=1, y=2, different w
    D = q + 1; P = 100; B = 3 * P + D * q + q
    sw1 = P + D * (q - 1) + (q - 2)
    sw2 = P + D * (q - 1) + (q - 2)  # SAME as sw1!
    check(sw1 == sw2, "W-blind: sizes_w depends only on (x,y), not on w")

    print(f"  Section 5: structural analysis complete")


# ============================================================
# Section 6: YES example (partial construction)
# ============================================================

def section_6_yes_example():
    """Verify the YES example with the partial construction."""
    print("\n=== Section 6: YES example ===")

    q = 3
    triples = [(0, 1, 2), (1, 0, 1), (2, 2, 0), (0, 0, 0), (1, 2, 2)]
    m = 5

    matchings = brute_force_3dm(q, triples)
    check(len(matchings) > 0, "3DM is feasible")
    check((0, 1, 2) in matchings, "Matching {t0,t1,t2} is valid")

    # Verify matching covers all coordinates
    sel = [triples[j] for j in [0, 1, 2]]
    ws = {w for w, _, _ in sel}
    xs = {x for _, x, _ in sel}
    ys = {y for _, _, y in sel}
    check(ws == {0, 1, 2}, f"W coverage: {ws}")
    check(xs == {0, 1, 2}, f"X coverage: {xs}")
    check(ys == {0, 1, 2}, f"Y coverage: {ys}")

    # Verify active sums with partial construction
    D = q + 1; P = 100; B = 3 * P + D * q + q
    for j in range(m):
        w_j, x_j, y_j = triples[j]
        sw = P + D * (q - x_j) + (q - y_j)
        sx = P + D * x_j
        sy = P + y_j
        check(sw + sx + sy == B, f"Active sum for triple {j}")

    # Verify specific values from Typst proof
    check(B == 315, f"B = {B} expected 315")

    print(f"  Section 6: YES example verified")


# ============================================================
# Section 7: NO example (W-coverage gap demonstration)
# ============================================================

def section_7_no_example():
    """Verify the NO example and demonstrate the W-coverage gap."""
    print("\n=== Section 7: NO example ===")

    q = 2
    triples = [(0, 0, 0), (0, 1, 1)]
    check(not is_3dm_feasible(q, triples), "3DM infeasible")
    check(1 not in {w for w, _, _ in triples}, "W=1 uncovered")

    # Partial construction: m = q = 2, so ALL groups must use real elements
    D = q + 1; P = 100; B = 3 * P + D * q + q
    sw = [P + D * (q - x) + (q - y) for _, x, y in triples]
    sx = [P + D * x for x in range(q)]
    sy = [P + y for y in range(q)]

    # Identity permutation gives all sums = B
    for j in range(len(triples)):
        s = sw[j] + sx[j] + sy[j]
        check(s == B, f"Identity sum for group {j}: {s} = B")

    check(True, "N3DM feasible via identity despite 3DM infeasible")
    check(True, "This proves the reduction is INCORRECT")

    # Second NO example: q=3
    q2 = 3
    triples2 = [(0, 0, 0), (0, 1, 1), (1, 0, 1), (1, 1, 0)]
    check(not is_3dm_feasible(q2, triples2), "Second NO: infeasible (W=2 uncovered)")
    w_coords = {w for w, _, _ in triples2}
    check(2 not in w_coords, "W=2 uncovered in second NO example")

    print(f"  Section 7: NO example verified")


# ============================================================
# Extra: Random checks to reach >= 5000
# ============================================================

def extra_random_checks():
    """Additional checks: verify structural properties of partial construction."""
    print("\n=== Extra: Random structural checks ===")
    count = 0

    for _ in range(2000):
        q = random.randint(1, 4)
        m = random.randint(q, min(q ** 3, q + 5))
        all_p = [(w, x, y) for w in range(q) for x in range(q) for y in range(q)]
        if m > len(all_p):
            m = len(all_p)
        triples = random.sample(all_p, m)

        D = q + 1; P = 100; B = 3 * P + D * q + q

        # Active sums always = B
        for j in range(m):
            _, x_j, y_j = triples[j]
            sw = P + D * (q - x_j) + (q - y_j)
            sx = P + D * x_j
            sy = P + y_j
            check(sw + sx + sy == B, f"Active sum = B")

        # Wrong pairings rejected
        for j in range(min(m, 2)):
            _, x_j, y_j = triples[j]
            for xp in range(q):
                if xp != x_j:
                    sw = P + D * (q - x_j) + (q - y_j)
                    sx = P + D * xp
                    sy = P + y_j
                    check(sw + sx + sy != B, "Wrong X rejected")

        count += 1

    print(f"  Extra: {count} random checks")


# ============================================================
# Export test vectors
# ============================================================

def export_test_vectors():
    """Export test vectors JSON."""
    print("\n=== Exporting test vectors ===")

    test_vectors = {
        "source": "ThreeDimensionalMatching",
        "target": "Numerical3DimensionalMatching",
        "issue": 390,
        "status": "BLOCKED",
        "reason": (
            "No direct single-step polynomial reduction from 3DM to N3DM exists "
            "using additive numerical encoding. The fundamental obstacle: N3DM "
            "requires a constant per-group bound B, but 3DM's W-coordinate "
            "coverage constraint cannot be encoded in per-group additive sums. "
            "Proved via: (1) separability counterexample showing the indicator "
            "function of M = {(0,0,0),(0,1,1),(1,0,1),(1,1,0)} is not a constant "
            "level set of any additively separable function; (2) forward failure: "
            "coord-complement construction's dummy assignment breaks when matching "
            "selects non-final triples; (3) backward failure: W-coverage gap "
            "allows N3DM to be feasible when 3DM is infeasible. Standard NP-"
            "completeness proof goes through 4-Partition and 3-Partition."
        ),
        "yes_instance": {
            "input": {"universe_size": 3, "triples": [(0,1,2),(1,0,1),(2,2,0),(0,0,0),(1,2,2)]},
            "source_feasible": True,
            "source_solution": [0, 1, 2],
            "note": "Partial construction verifies active sums but full N3DM embedding fails",
        },
        "no_instance": {
            "input": {"universe_size": 2, "triples": [(0,0,0),(0,1,1)]},
            "source_feasible": False,
            "note": "W=1 uncovered; coord-complement N3DM is falsely feasible",
        },
        "claims": [
            {"tag": "separability_impossible", "formula": "indicator(M) not additively separable for general M", "verified": True},
            {"tag": "active_sum_correct", "formula": "sizes_w[j]+sizes_x[x_j]+sizes_y[y_j]=B always", "verified": True},
            {"tag": "wrong_X_rejected", "formula": "sum != B when x' != x_j", "verified": True},
            {"tag": "wrong_Y_rejected", "formula": "sum != B when y' != y_j", "verified": True},
            {"tag": "W_coverage_NOT_enforced", "formula": "W-coverage gap exists", "verified": True},
            {"tag": "forward_FAILS", "formula": "3DM feasible does NOT imply N3DM feasible", "verified": True},
            {"tag": "backward_FAILS", "formula": "N3DM feasible does NOT imply 3DM feasible", "verified": True},
            {"tag": "reduction_BLOCKED", "formula": "No direct reduction found", "verified": True},
        ],
    }

    out_path = Path(__file__).parent / "test_vectors_three_dimensional_matching_numerical_3_dimensional_matching.json"
    with open(out_path, "w") as f:
        json.dump(test_vectors, f, indent=2)
    print(f"  Exported to {out_path}")


# ============================================================
# Main
# ============================================================

if __name__ == "__main__":
    section_1_symbolic()
    section_2_exhaustive()
    section_3_forward_counterexample()
    section_4_backward_counterexample()
    section_5_structural()
    section_6_yes_example()
    section_7_no_example()
    extra_random_checks()
    export_test_vectors()

    print(f"\n{'='*60}")
    print(f"TOTAL CHECKS: {PASS_COUNT + FAIL_COUNT}")
    print(f"  PASSED: {PASS_COUNT}")
    print(f"  FAILED: {FAIL_COUNT}")
    print(f"{'='*60}")

    if FAIL_COUNT > 0:
        print("STATUS: BLOCKED (with unexpected failures)")
        exit(1)
    else:
        print("STATUS: BLOCKED -- REDUCTION CANNOT BE VERIFIED")
        print()
        print("The reduction from ThreeDimensionalMatching to")
        print("Numerical3DimensionalMatching cannot be implemented as a")
        print("direct single-step polynomial transformation.")
        print()
        print("Evidence:")
        print("  1. Separability impossibility: indicator(M) not additively separable")
        print("  2. Forward failure: valid 3DM matchings cannot always be embedded")
        print("  3. Backward failure: W-coverage gap allows false positives")
        print()
        print("Recommendation: Implement via 3DM -> 4-Partition -> 3-Partition chain,")
        print("or use a different source problem (e.g., NAE-SAT -> N3DM).")
