#!/usr/bin/env python3
"""
Adversary verification script: MinimumVertexCover -> PartialFeedbackEdgeSet reduction.
Issue: #894

Independent re-implementation of the reduction and extraction logic,
plus property-based testing with hypothesis. >= 5000 independent checks.

This script does NOT import from verify_*.py -- it re-derives everything
from scratch as an independent cross-check.
"""

import json
import sys
from itertools import product, combinations
from typing import Optional

try:
    from hypothesis import given, settings, assume, HealthCheck
    from hypothesis import strategies as st
    HAS_HYPOTHESIS = True
except ImportError:
    HAS_HYPOTHESIS = False
    print("WARNING: hypothesis not installed; falling back to pure-random adversary tests")


# ─────────────────────────────────────────────────────────────────────
# Independent re-implementation of reduction
# ─────────────────────────────────────────────────────────────────────

def adv_reduce(n, edges, k, L):
    """Independent reduction: VC(G,k) -> PFES(G', k, L) for even L >= 6."""
    assert L >= 6 and L % 2 == 0
    m = len(edges)
    half = (L - 4) // 2  # p = q = half

    # Hub vertices: 2v, 2v+1 for each v
    n_prime = 2 * n + m * (L - 4)
    hub1 = {v: 2*v for v in range(n)}
    hub2 = {v: 2*v+1 for v in range(n)}

    new_edges = []
    hub_idx = {}
    cycles_info = []

    # Hub edges
    for v in range(n):
        hub_idx[v] = len(new_edges)
        new_edges.append((hub1[v], hub2[v]))

    # Gadgets
    ibase = 2 * n
    for idx, (u, v) in enumerate(edges):
        ce = [hub_idx[u], hub_idx[v]]
        gb = ibase + idx * (L - 4)
        fwd = list(range(gb, gb + half))
        ret = list(range(gb + half, gb + 2*half))

        # Forward: hub2[u] -> fwd -> hub1[v]
        ei = len(new_edges); new_edges.append((hub2[u], fwd[0])); ce.append(ei)
        for i in range(half - 1):
            ei = len(new_edges); new_edges.append((fwd[i], fwd[i+1])); ce.append(ei)
        ei = len(new_edges); new_edges.append((fwd[-1], hub1[v])); ce.append(ei)

        # Return: hub2[v] -> ret -> hub1[u]
        ei = len(new_edges); new_edges.append((hub2[v], ret[0])); ce.append(ei)
        for i in range(half - 1):
            ei = len(new_edges); new_edges.append((ret[i], ret[i+1])); ce.append(ei)
        ei = len(new_edges); new_edges.append((ret[-1], hub1[u])); ce.append(ei)

        cycles_info.append(((u, v), ce))

    return n_prime, new_edges, k, L, hub_idx, cycles_info


def adv_is_vc(n, edges, config):
    """Check vertex cover."""
    for u, v in edges:
        if config[u] == 0 and config[v] == 0:
            return False
    return True


def adv_find_short_cycles(n, edges, max_len):
    """Find simple cycles of length <= max_len."""
    if not edges or max_len < 3:
        return []
    adj = [[] for _ in range(n)]
    for idx, (u, v) in enumerate(edges):
        adj[u].append((v, idx))
        adj[v].append((u, idx))
    cycles = set()
    vis = [False] * n

    def dfs(s, c, pe, pl):
        for nb, ei in adj[c]:
            if nb == s and pl+1 >= 3 and pl+1 <= max_len:
                cycles.add(frozenset(pe + [ei]))
                continue
            if nb == s or vis[nb] or nb < s or pl+1 >= max_len:
                continue
            vis[nb] = True
            dfs(s, nb, pe + [ei], pl + 1)
            vis[nb] = False

    for s in range(n):
        vis[s] = True
        for nb, ei in adj[s]:
            if nb <= s: continue
            vis[nb] = True
            dfs(s, nb, [ei], 1)
            vis[nb] = False
        vis[s] = False
    return list(cycles)


def adv_is_pfes(n, edges, budget, L, config):
    """Check PFES feasibility."""
    if sum(config) > budget:
        return False
    kept = [(u, v) for (u, v), c in zip(edges, config) if c == 0]
    return len(adv_find_short_cycles(n, kept, L)) == 0


def adv_solve_vc(n, edges):
    """Brute-force VC."""
    for k in range(n + 1):
        for bits in combinations(range(n), k):
            cfg = [0] * n
            for b in bits:
                cfg[b] = 1
            if adv_is_vc(n, edges, cfg):
                return k, cfg
    return n + 1, None


def adv_solve_pfes(n, edges, budget, L):
    """Brute-force PFES."""
    m = len(edges)
    for k in range(budget + 1):
        for bits in combinations(range(m), k):
            cfg = [0] * m
            for b in bits:
                cfg[b] = 1
            if adv_is_pfes(n, edges, budget, L, cfg):
                return cfg
    return None


def adv_extract(n, orig_edges, k, L, hub_idx, cycles_info, pfes_config):
    """Extract VC from PFES solution."""
    cover = [0] * n
    for v, ei in hub_idx.items():
        if pfes_config[ei] == 1:
            cover[v] = 1
    for (u, v), _ in cycles_info:
        if cover[u] == 0 and cover[v] == 0:
            cover[u] = 1
    return cover


# ─────────────────────────────────────────────────────────────────────
# Property checks
# ─────────────────────────────────────────────────────────────────────

def adv_check_all(n, edges, k, L=6):
    """Run all adversary checks on a single instance. Returns check count."""
    checks = 0

    # 1. Overhead
    n_p, ne, K_p, L_o, hub, cycs = adv_reduce(n, edges, k, L)
    m = len(edges)
    assert n_p == 2*n + m*(L-4), f"nv mismatch"
    assert len(ne) == n + m*(L-2), f"ne mismatch"
    assert K_p == k, f"K' mismatch"
    checks += 3

    # 2. Forward + backward feasibility
    min_vc, vc_wit = adv_solve_vc(n, edges)
    vc_feas = min_vc <= k

    if len(ne) <= 35:
        pfes_sol = adv_solve_pfes(n_p, ne, K_p, L_o)
        pfes_feas = pfes_sol is not None
        assert vc_feas == pfes_feas, \
            f"Feasibility mismatch: vc={vc_feas}, pfes={pfes_feas}, n={n}, m={m}, k={k}, L={L}"
        checks += 1

        # 3. Extraction
        if pfes_sol is not None:
            ext = adv_extract(n, edges, k, L, hub, cycs, pfes_sol)
            assert adv_is_vc(n, edges, ext), f"Extracted VC invalid"
            assert sum(ext) <= k, f"Extracted VC too large: {sum(ext)} > {k}"
            checks += 2

    # 4. Gadget structure
    for (u, v), ce in cycs:
        assert len(ce) == L, f"Gadget len {len(ce)} != {L}"
        assert hub[u] in ce, f"Missing hub[{u}]"
        assert hub[v] in ce, f"Missing hub[{v}]"
        checks += 3

    # 5. No spurious cycles (if small enough)
    if n_p <= 20 and len(ne) <= 40:
        all_cycs = adv_find_short_cycles(n_p, ne, L)
        gsets = {frozenset(ce) for _, ce in cycs}
        for c in all_cycs:
            assert c in gsets, f"Spurious cycle found"
            checks += 1

    return checks


# ─────────────────────────────────────────────────────────────────────
# Test drivers
# ─────────────────────────────────────────────────────────────────────

def adversary_exhaustive(max_n=5, max_val=None):
    """Exhaustive adversary tests for small graphs."""
    checks = 0
    for n in range(1, max_n + 1):
        all_possible = [(i, j) for i in range(n) for j in range(i+1, n)]
        max_e = len(all_possible)
        for mask in range(1 << max_e):
            edges = [all_possible[i] for i in range(max_e) if mask & (1 << i)]
            min_vc, _ = adv_solve_vc(n, edges)
            for k in set([min_vc, max(0, min_vc - 1)]):
                if 0 <= k <= n:
                    for L in [6, 8]:
                        checks += adv_check_all(n, edges, k, L)
    return checks


def adversary_random(count=500, max_n=8):
    """Random adversary tests."""
    import random
    rng = random.Random(9999)
    checks = 0
    for _ in range(count):
        n = rng.randint(1, max_n)
        p_edge = rng.random()
        edges = [(i, j) for i in range(n) for j in range(i+1, n) if rng.random() < p_edge]
        min_vc, _ = adv_solve_vc(n, edges)
        k = rng.choice([max(0, min_vc - 1), min_vc, min(n, min_vc + 1)])
        L = rng.choice([6, 8, 10])
        checks += adv_check_all(n, edges, k, L)
    return checks


def adversary_hypothesis():
    """Property-based testing with hypothesis."""
    if not HAS_HYPOTHESIS:
        return 0

    checks_counter = [0]

    @given(
        n=st.integers(min_value=1, max_value=6),
        edges=st.lists(st.tuples(
            st.integers(min_value=0, max_value=5),
            st.integers(min_value=0, max_value=5),
        ), min_size=0, max_size=10),
        k=st.integers(min_value=0, max_value=6),
        L=st.sampled_from([6, 8, 10]),
    )
    @settings(
        max_examples=500,
        suppress_health_check=[HealthCheck.too_slow, HealthCheck.filter_too_much],
        deadline=None,
    )
    def prop_reduction_correct(n, edges, k, L):
        # Filter to valid simple graph edges
        filtered = []
        seen = set()
        for u, v in edges:
            if u >= n or v >= n or u == v:
                continue
            key = (min(u, v), max(u, v))
            if key not in seen:
                seen.add(key)
                filtered.append(key)
        assume(0 <= k <= n)
        checks_counter[0] += adv_check_all(n, filtered, k, L)

    prop_reduction_correct()
    return checks_counter[0]


def adversary_edge_cases():
    """Targeted edge cases."""
    checks = 0
    cases = [
        # Empty graph
        (1, [], 0), (1, [], 1), (2, [], 0),
        # Single edge
        (2, [(0, 1)], 0), (2, [(0, 1)], 1),
        # Triangle
        (3, [(0, 1), (1, 2), (0, 2)], 0),
        (3, [(0, 1), (1, 2), (0, 2)], 1),
        (3, [(0, 1), (1, 2), (0, 2)], 2),
        (3, [(0, 1), (1, 2), (0, 2)], 3),
        # Star
        (4, [(0, 1), (0, 2), (0, 3)], 1),
        (4, [(0, 1), (0, 2), (0, 3)], 2),
        # Path
        (4, [(0, 1), (1, 2), (2, 3)], 1),
        (4, [(0, 1), (1, 2), (2, 3)], 2),
        # K4
        (4, [(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)], 2),
        (4, [(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)], 3),
        # Bipartite
        (4, [(0, 2), (0, 3), (1, 2), (1, 3)], 2),
        # Isolated vertices
        (5, [(0, 1)], 1),
        (5, [(0, 1), (2, 3)], 2),
    ]
    for n, edges, k in cases:
        for L in [6, 8]:
            checks += adv_check_all(n, edges, k, L)
    return checks


if __name__ == "__main__":
    print("=" * 60)
    print("Adversary verification: MinimumVertexCover -> PartialFeedbackEdgeSet")
    print("=" * 60)

    print("\n[1/4] Edge cases...")
    n_edge = adversary_edge_cases()
    print(f"  Edge case checks: {n_edge}")

    print("\n[2/4] Exhaustive adversary (n <= 5)...")
    n_exh = adversary_exhaustive()
    print(f"  Exhaustive checks: {n_exh}")

    print("\n[3/4] Random adversary (different seed)...")
    n_rand = adversary_random()
    print(f"  Random checks: {n_rand}")

    print("\n[4/4] Hypothesis PBT...")
    n_hyp = adversary_hypothesis()
    print(f"  Hypothesis checks: {n_hyp}")

    total = n_edge + n_exh + n_rand + n_hyp
    print(f"\n  TOTAL adversary checks: {total}")
    assert total >= 5000, f"Need >= 5000 checks, got {total}"
    print(f"\nAll {total} adversary checks PASSED.")
