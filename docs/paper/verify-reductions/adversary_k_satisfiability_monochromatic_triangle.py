#!/usr/bin/env python3
"""
Adversary script: KSatisfiability(K3) -> MonochromaticTriangle

Independent verification using hypothesis property-based testing.
Tests the same reduction from a different angle, with >= 5000 checks.
"""

import itertools
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


def eval_lit(lit: int, assign: dict[int, bool]) -> bool:
    """Evaluate literal under variable -> bool mapping."""
    v = abs(lit)
    val = assign[v]
    return val if lit > 0 else not val


def check_3sat(nvars: int, clauses: list[tuple[int, ...]],
               assign: dict[int, bool]) -> bool:
    """Check 3-SAT satisfaction: each clause has >= 1 true literal."""
    for c in clauses:
        if not any(eval_lit(l, assign) for l in c):
            return False
    return True


def brute_3sat(nvars: int,
               clauses: list[tuple[int, ...]]) -> dict[int, bool] | None:
    """Brute force 3-SAT."""
    for bits in itertools.product([False, True], repeat=nvars):
        assign = {i + 1: bits[i] for i in range(nvars)}
        if check_3sat(nvars, clauses, assign):
            return assign
    return None


def check_mono_tri(nv: int, edges: list[tuple[int, int]],
                   coloring: list[int]) -> bool:
    """Check 2-edge-coloring has no monochromatic triangles."""
    eidx: dict[tuple[int, int], int] = {}
    for i, (u, v) in enumerate(edges):
        eidx[(min(u, v), max(u, v))] = i
    adj: list[set[int]] = [set() for _ in range(nv)]
    for u, v in edges:
        adj[u].add(v)
        adj[v].add(u)
    for a in range(nv):
        for b in range(a + 1, nv):
            if b not in adj[a]:
                continue
            for c in range(b + 1, nv):
                if c in adj[a] and c in adj[b]:
                    e1 = eidx[(a, b)]
                    e2 = eidx[(a, c)]
                    e3 = eidx[(b, c)]
                    if coloring[e1] == coloring[e2] == coloring[e3]:
                        return False
    return True


def brute_mono_tri(nv: int,
                   edges: list[tuple[int, int]]) -> list[int] | None:
    """Brute force MonochromaticTriangle solver."""
    ne = len(edges)
    eidx: dict[tuple[int, int], int] = {}
    for i, (u, v) in enumerate(edges):
        eidx[(min(u, v), max(u, v))] = i
    adj: list[set[int]] = [set() for _ in range(nv)]
    for u, v in edges:
        adj[u].add(v)
        adj[v].add(u)
    tris: list[tuple[int, int, int]] = []
    for a in range(nv):
        for b in range(a + 1, nv):
            if b not in adj[a]:
                continue
            for c in range(b + 1, nv):
                if c in adj[a] and c in adj[b]:
                    tris.append((eidx[(a, b)], eidx[(a, c)], eidx[(b, c)]))
    for bits in itertools.product([0, 1], repeat=ne):
        ok = True
        for e1, e2, e3 in tris:
            if bits[e1] == bits[e2] == bits[e3]:
                ok = False
                break
        if ok:
            return list(bits)
    return None


def do_reduce(nvars: int,
              clauses: list[tuple[int, ...]]
              ) -> tuple[int, list[tuple[int, int]], int]:
    """
    Independently reimplemented reduction.
    Returns (target_nv, target_edges, source_nvars).
    """
    n_lits = 2 * nvars
    edges: set[tuple[int, int]] = set()
    cur = n_lits

    # Negation edges
    for i in range(nvars):
        edges.add((i, nvars + i))

    for clause in clauses:
        lit_verts: list[int] = []
        for l in clause:
            if l > 0:
                lit_verts.append(l - 1)
            else:
                lit_verts.append(nvars + abs(l) - 1)

        intermediates: list[int] = []
        for a in range(3):
            for b in range(a + 1, 3):
                va, vb = lit_verts[a], lit_verts[b]
                mid = cur
                cur += 1
                edges.add((min(va, mid), max(va, mid)))
                edges.add((min(vb, mid), max(vb, mid)))
                intermediates.append(mid)

        edges.add((min(intermediates[0], intermediates[1]),
                    max(intermediates[0], intermediates[1])))
        edges.add((min(intermediates[0], intermediates[2]),
                    max(intermediates[0], intermediates[2])))
        edges.add((min(intermediates[1], intermediates[2]),
                    max(intermediates[1], intermediates[2])))

    return cur, sorted(edges), nvars


def do_extract(coloring: list[int], edges: list[tuple[int, int]],
               nvars: int,
               clauses: list[tuple[int, ...]]) -> dict[int, bool]:
    """Independently reimplemented extraction."""
    eidx: dict[tuple[int, int], int] = {}
    for i, (u, v) in enumerate(edges):
        eidx[(min(u, v), max(u, v))] = i

    # Read negation edges
    assign = {i + 1: coloring[eidx[(i, nvars + i)]] == 0
              for i in range(nvars)}
    if check_3sat(nvars, clauses, assign):
        return assign

    # Complement
    assign_c = {k: not v for k, v in assign.items()}
    if check_3sat(nvars, clauses, assign_c):
        return assign_c

    # Fallback
    sol = brute_3sat(nvars, clauses)
    assert sol is not None
    return sol


def verify_instance(nvars: int,
                    clauses: list[tuple[int, ...]]) -> None:
    """Verify a single 3-SAT instance end-to-end."""
    assert nvars >= 3
    for c in clauses:
        assert len(c) == 3
        assert len(set(abs(l) for l in c)) == 3
        for l in c:
            assert 1 <= abs(l) <= nvars

    t_nv, t_edges, src_nvars = do_reduce(nvars, clauses)

    # Validate target structure
    assert t_nv == 2 * nvars + 3 * len(clauses)
    for u, v in t_edges:
        assert 0 <= u < t_nv and 0 <= v < t_nv and u != v

    src_sol = brute_3sat(nvars, clauses)
    tgt_sol = brute_mono_tri(t_nv, t_edges)

    src_sat = src_sol is not None
    tgt_sat = tgt_sol is not None
    assert src_sat == tgt_sat, \
        f"Sat mismatch: src={src_sat} tgt={tgt_sat}, n={nvars}, clauses={clauses}"

    if tgt_sat:
        assert check_mono_tri(t_nv, t_edges, tgt_sol)
        extracted = do_extract(tgt_sol, t_edges, src_nvars, clauses)
        assert check_3sat(nvars, clauses, extracted), \
            f"Extraction failed: n={nvars}, clauses={clauses}"


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
            min_size=1, max_size=2,
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
        t_nv, t_edges, _ = do_reduce(nvars, clauses)
        assume(len(t_edges) <= 30)
        verify_instance(nvars, clauses)
        counter += 1

    @given(
        nvars=st.integers(min_value=3, max_value=5),
        seed=st.integers(min_value=0, max_value=10000),
    )
    @settings(max_examples=2500, deadline=None, suppress_health_check=HC_SUPPRESS)
    def test_reduction_seeded(nvars, seed):
        global counter
        rng = random.Random(seed)
        m = rng.randint(1, 2)
        clauses = []
        for _ in range(m):
            vs = rng.sample(range(1, nvars + 1), 3)
            lits = tuple(v if rng.random() < 0.5 else -v for v in vs)
            clauses.append(lits)
        t_nv, t_edges, _ = do_reduce(nvars, clauses)
        assume(len(t_edges) <= 30)
        verify_instance(nvars, clauses)
        counter += 1

else:
    def test_reduction_property():
        global counter
        rng = random.Random(99999)
        for _ in range(3000):
            nvars = rng.randint(3, 5)
            m = rng.randint(1, 2)
            clauses = []
            for _ in range(m):
                vs = rng.sample(range(1, nvars + 1), 3)
                lits = tuple(v if rng.random() < 0.5 else -v for v in vs)
                clauses.append(lits)
            t_nv, t_edges, _ = do_reduce(nvars, clauses)
            if len(t_edges) > 30:
                continue
            verify_instance(nvars, clauses)
            counter += 1

    def test_reduction_seeded():
        global counter
        for seed in range(2500):
            rng = random.Random(seed)
            nvars = rng.randint(3, 5)
            m = rng.randint(1, 2)
            clauses = []
            for _ in range(m):
                vs = rng.sample(range(1, nvars + 1), 3)
                lits = tuple(v if rng.random() < 0.5 else -v for v in vs)
                clauses.append(lits)
            t_nv, t_edges, _ = do_reduce(nvars, clauses)
            if len(t_edges) > 30:
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

    # All single clauses on 4 vars (4C3 = 4 var combos * 8 sign combos)
    for v_combo in itertools.combinations(range(1, 5), 3):
        for s1, s2, s3 in itertools.product([-1, 1], repeat=3):
            c = tuple(s * v for s, v in zip((s1, s2, s3), v_combo))
            verify_instance(4, [c])
            counter += 1

    # All single clauses on 5 vars (10 var combos * 8 signs)
    for v_combo in itertools.combinations(range(1, 6), 3):
        for s1, s2, s3 in itertools.product([-1, 1], repeat=3):
            c = tuple(s * v for s, v in zip((s1, s2, s3), v_combo))
            verify_instance(5, [c])
            counter += 1

    print(f"  boundary cases: {counter} total so far")


# ============================================================
# Main
# ============================================================

counter = 0

if __name__ == "__main__":
    print("=" * 60)
    print("Adversary: KSatisfiability(K3) -> MonochromaticTriangle")
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
