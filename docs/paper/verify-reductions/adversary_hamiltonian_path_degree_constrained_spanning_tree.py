#!/usr/bin/env python3
"""
Adversary verification script: HamiltonianPath -> DegreeConstrainedSpanningTree reduction.
Issue: #911

Independent re-implementation of the reduction and extraction logic,
plus property-based testing with hypothesis. >=5000 independent checks.

This script does NOT import from verify_hamiltonian_path_degree_constrained_spanning_tree.py --
it re-derives everything from scratch as an independent cross-check.
"""

import json
import sys
import random
from itertools import permutations, product
from typing import Optional

try:
    from hypothesis import given, settings, assume, HealthCheck
    from hypothesis import strategies as st
    HAS_HYPOTHESIS = True
except ImportError:
    HAS_HYPOTHESIS = False
    print("WARNING: hypothesis not installed; falling back to pure-random adversary tests")


# ---------------------------------------------------------------------
# Independent re-implementation of reduction
# ---------------------------------------------------------------------

def adv_reduce(n: int, edges: list[tuple[int, int]]) -> tuple[int, list[tuple[int, int]], int]:
    """Independent reduction: HamiltonianPath -> DegreeConstrainedSpanningTree."""
    # Identity on graph, set degree bound to 2
    return (n, edges[:], 2)


def adv_extract(n: int, edges: list[tuple[int, int]], config: list[int]) -> list[int]:
    """Independent extraction: DCST solution -> HamiltonianPath solution."""
    if n <= 1:
        return list(range(n))

    # Build selected edge list
    sel_edges = [edges[i] for i in range(len(edges)) if config[i] == 1]

    # Build adjacency
    adj = [[] for _ in range(n)]
    for u, v in sel_edges:
        adj[u].append(v)
        adj[v].append(u)

    # Find endpoint (degree 1)
    start = -1
    for v in range(n):
        if len(adj[v]) == 1:
            start = v
            break

    if start == -1:
        return list(range(n))  # should not happen for valid solution

    # Trace path
    path = [start]
    prev = -1
    cur = start
    for _ in range(n - 1):
        for nxt in adj[cur]:
            if nxt != prev:
                path.append(nxt)
                prev = cur
                cur = nxt
                break

    return path


def adv_is_hamiltonian_path(n: int, edges: list[tuple[int, int]], perm: list[int]) -> bool:
    """Check if perm is a valid Hamiltonian path."""
    if len(perm) != n:
        return False
    if sorted(perm) != list(range(n)):
        return False
    if n <= 1:
        return True

    edge_set = set()
    for u, v in edges:
        edge_set.add((u, v))
        edge_set.add((v, u))

    for i in range(n - 1):
        if (perm[i], perm[i + 1]) not in edge_set:
            return False
    return True


def adv_is_valid_dcst(n: int, edges: list[tuple[int, int]], config: list[int], max_deg: int) -> bool:
    """Check if config is a valid DCST solution."""
    if n == 0:
        return sum(config) == 0
    if len(config) != len(edges):
        return False

    selected = [edges[i] for i in range(len(edges)) if config[i] == 1]

    if len(selected) != n - 1:
        return False

    deg = [0] * n
    adj = [[] for _ in range(n)]
    for u, v in selected:
        deg[u] += 1
        deg[v] += 1
        adj[u].append(v)
        adj[v].append(u)

    if any(d > max_deg for d in deg):
        return False

    # BFS connectivity
    visited = [False] * n
    stack = [0]
    visited[0] = True
    cnt = 1
    while stack:
        cur = stack.pop()
        for nxt in adj[cur]:
            if not visited[nxt]:
                visited[nxt] = True
                cnt += 1
                stack.append(nxt)
    return cnt == n


def adv_solve_hp(n: int, edges: list[tuple[int, int]]) -> Optional[list[int]]:
    """Brute-force Hamiltonian Path solver."""
    if n == 0:
        return []
    if n == 1:
        return [0]

    edge_set = set()
    for u, v in edges:
        edge_set.add((u, v))
        edge_set.add((v, u))

    for perm in permutations(range(n)):
        ok = True
        for i in range(n - 1):
            if (perm[i], perm[i + 1]) not in edge_set:
                ok = False
                break
        if ok:
            return list(perm)
    return None


def adv_solve_dcst(n: int, edges: list[tuple[int, int]], max_deg: int) -> Optional[list[int]]:
    """Brute-force DCST solver."""
    if n == 0:
        return []
    if n == 1:
        return [0] * len(edges)

    m = len(edges)
    for bits in product(range(2), repeat=m):
        config = list(bits)
        if adv_is_valid_dcst(n, edges, config, max_deg):
            return config
    return None


# ---------------------------------------------------------------------
# Property checks
# ---------------------------------------------------------------------

def adv_check_all(n: int, edges: list[tuple[int, int]]) -> int:
    """Run all adversary checks on a single instance. Returns check count."""
    checks = 0

    # 1. Overhead
    t_n, t_edges, t_k = adv_reduce(n, edges)
    assert t_n == n, f"Overhead: vertices changed {n} -> {t_n}"
    assert len(t_edges) == len(edges), f"Overhead: edges changed {len(edges)} -> {len(t_edges)}"
    assert t_k == 2, f"Overhead: degree bound not 2"
    checks += 1

    # 2. Forward + Backward + Infeasible
    hp_sol = adv_solve_hp(n, edges)
    dcst_sol = adv_solve_dcst(t_n, t_edges, t_k)

    # Feasibility must agree
    hp_feas = hp_sol is not None
    dcst_feas = dcst_sol is not None
    assert hp_feas == dcst_feas, (
        f"Feasibility mismatch: hp={hp_feas}, dcst={dcst_feas}, n={n}, edges={edges}"
    )
    checks += 1

    # Forward
    if hp_feas:
        assert dcst_feas, f"Forward violation: n={n}, edges={edges}"
        checks += 1

    # Backward via extract
    if dcst_feas:
        path = adv_extract(n, edges, dcst_sol)
        assert adv_is_hamiltonian_path(n, edges, path), (
            f"Extract violation: n={n}, edges={edges}, path={path}"
        )
        checks += 1

    # Infeasible
    if not hp_feas:
        assert not dcst_feas, f"Infeasible violation: n={n}, edges={edges}"
        checks += 1

    # 3. Cross-check: if we have a DCST solution, verify it is actually valid
    if dcst_sol is not None:
        assert adv_is_valid_dcst(n, edges, dcst_sol, 2), (
            f"DCST solution invalid: n={n}, edges={edges}"
        )
        checks += 1

    return checks


# ---------------------------------------------------------------------
# Test drivers
# ---------------------------------------------------------------------

def all_simple_graphs(n: int):
    """Generate all simple undirected graphs on n vertices."""
    possible = [(i, j) for i in range(n) for j in range(i + 1, n)]
    m = len(possible)
    for mask in range(1 << m):
        edges = [possible[k] for k in range(m) if mask & (1 << k)]
        yield edges


def adversary_exhaustive(max_n: int = 5) -> int:
    """Exhaustive adversary tests for all graphs with n <= max_n."""
    checks = 0
    for n in range(0, max_n + 1):
        for edges in all_simple_graphs(n):
            checks += adv_check_all(n, edges)
    return checks


def adversary_random(count: int = 500, max_n: int = 8) -> int:
    """Random adversary tests with independent RNG seed."""
    rng = random.Random(9999)  # Different seed from verify script
    checks = 0
    for _ in range(count):
        n = rng.randint(1, max_n)
        p = rng.choice([0.2, 0.4, 0.6, 0.8, 1.0])
        edges = []
        for i in range(n):
            for j in range(i + 1, n):
                if rng.random() < p:
                    edges.append((i, j))
        checks += adv_check_all(n, edges)
    return checks


def adversary_hypothesis() -> int:
    """Property-based testing with hypothesis."""
    if not HAS_HYPOTHESIS:
        return 0

    checks_counter = [0]

    @st.composite
    def graph_strategy(draw):
        n = draw(st.integers(min_value=1, max_value=7))
        possible_edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
        if not possible_edges:
            return n, []
        mask = draw(st.integers(min_value=0, max_value=(1 << len(possible_edges)) - 1))
        edges = [possible_edges[k] for k in range(len(possible_edges)) if mask & (1 << k)]
        return n, edges

    @given(graph=graph_strategy())
    @settings(
        max_examples=2000,
        suppress_health_check=[HealthCheck.too_slow],
        deadline=None,
    )
    def prop_reduction_correct(graph):
        n, edges = graph
        checks_counter[0] += adv_check_all(n, edges)

    prop_reduction_correct()
    return checks_counter[0]


def adversary_edge_cases() -> int:
    """Targeted edge cases."""
    checks = 0
    cases = [
        # Single vertex
        (1, []),
        # Two vertices, connected
        (2, [(0, 1)]),
        # Two vertices, disconnected
        (2, []),
        # Triangle
        (3, [(0, 1), (1, 2), (0, 2)]),
        # Path of 3
        (3, [(0, 1), (1, 2)]),
        # Star K_{1,3}
        (4, [(0, 1), (0, 2), (0, 3)]),
        # K_{1,4} + edge from issue
        (5, [(0, 1), (0, 2), (0, 3), (0, 4), (1, 2)]),
        # Petersen graph
        (10, [(i, (i + 1) % 5) for i in range(5)]
             + [(5 + i, 5 + (i + 2) % 5) for i in range(5)]
             + [(i, i + 5) for i in range(5)]),
        # Complete bipartite K_{2,3}
        (5, [(0, 2), (0, 3), (0, 4), (1, 2), (1, 3), (1, 4)]),
        # Disconnected: two triangles
        (6, [(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]),
        # Almost complete minus one edge
        (4, [(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)]),
        # Self-loop-free multigraph edge case: just two edges forming a path
        (3, [(0, 2), (2, 1)]),
    ]
    for n, edges in cases:
        checks += adv_check_all(n, edges)
    return checks


if __name__ == "__main__":
    print("=" * 60)
    print("Adversary verification: HamiltonianPath -> DegreeConstrainedSpanningTree")
    print("=" * 60)

    print("\n[1/4] Edge cases...")
    n_edge = adversary_edge_cases()
    print(f"  Edge case checks: {n_edge}")

    print("\n[2/4] Exhaustive adversary (n <= 5, all graphs)...")
    n_exh = adversary_exhaustive()
    print(f"  Exhaustive checks: {n_exh}")

    print("\n[3/4] Random adversary (different seed)...")
    n_rand = adversary_random(count=500)
    print(f"  Random checks: {n_rand}")

    print("\n[4/4] Hypothesis PBT...")
    n_hyp = adversary_hypothesis()
    print(f"  Hypothesis checks: {n_hyp}")

    total = n_edge + n_exh + n_rand + n_hyp
    print(f"\n  TOTAL adversary checks: {total}")
    assert total >= 5000, f"Need >=5000 checks, got {total}"
    print(f"\nAll {total} adversary checks PASSED.")
