#!/usr/bin/env python3
"""
Adversary verification script: OptimalLinearArrangement → RootedTreeArrangement.
Issue: #888

Independent re-implementation of the reduction, solvers, and property checks.
This script does NOT import from the verify script — it re-derives everything
from scratch as an independent cross-check.

This is a DECISION-ONLY reduction. The key property is:
  OLA(G, K) YES => RTA(G, K) YES
The converse does NOT hold in general.

Uses hypothesis for property-based testing. ≥5000 independent checks.
"""

import json
import sys
from itertools import permutations, product
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

def adv_reduce(n: int, edges: list[tuple[int, int]], bound: int) -> tuple[int, list[tuple[int, int]], int]:
    """Independent reduction: OLA(G,K) -> RTA(G,K) identity."""
    return (n, edges[:], bound)


# ─────────────────────────────────────────────────────────────────────
# Independent OLA solver
# ─────────────────────────────────────────────────────────────────────

def adv_ola_cost(n: int, edges: list[tuple[int, int]], perm: list[int]) -> int:
    """Compute OLA cost."""
    total = 0
    for u, v in edges:
        total += abs(perm[u] - perm[v])
    return total


def adv_solve_ola(n: int, edges: list[tuple[int, int]], bound: int) -> Optional[list[int]]:
    """Brute-force OLA solver."""
    if n == 0:
        return []
    for p in permutations(range(n)):
        if adv_ola_cost(n, edges, list(p)) <= bound:
            return list(p)
    return None


def adv_optimal_ola(n: int, edges: list[tuple[int, int]]) -> int:
    """Find minimum OLA cost."""
    if n == 0 or not edges:
        return 0
    best = float('inf')
    for p in permutations(range(n)):
        c = adv_ola_cost(n, edges, list(p))
        if c < best:
            best = c
    return best


# ─────────────────────────────────────────────────────────────────────
# Independent RTA solver
# ─────────────────────────────────────────────────────────────────────

def adv_compute_depth(parent: list[int]) -> Optional[list[int]]:
    """Compute depths from parent array. None if invalid tree."""
    n = len(parent)
    if n == 0:
        return []
    roots = [i for i in range(n) if parent[i] == i]
    if len(roots) != 1:
        return None

    root = roots[0]
    depth = [-1] * n
    depth[root] = 0

    changed = True
    iterations = 0
    while changed and iterations < n:
        changed = False
        iterations += 1
        for i in range(n):
            if depth[i] >= 0:
                continue
            p = parent[i]
            if p == i:
                return None  # extra root
            if depth[p] >= 0:
                depth[i] = depth[p] + 1
                changed = True

    if any(d < 0 for d in depth):
        return None  # disconnected or cycle
    return depth


def adv_is_ancestor(parent: list[int], anc: int, desc: int) -> bool:
    """Check ancestry relation."""
    cur = desc
    seen = set()
    while cur != anc:
        if cur in seen:
            return False
        seen.add(cur)
        p = parent[cur]
        if p == cur:
            return False
        cur = p
    return True


def adv_are_comparable(parent: list[int], u: int, v: int) -> bool:
    return adv_is_ancestor(parent, u, v) or adv_is_ancestor(parent, v, u)


def adv_rta_cost(n: int, edges: list[tuple[int, int]], parent: list[int], mapping: list[int]) -> Optional[int]:
    """Compute RTA stretch. None if invalid."""
    depth = adv_compute_depth(parent)
    if depth is None:
        return None
    if sorted(mapping) != list(range(n)):
        return None
    total = 0
    for u, v in edges:
        tu, tv = mapping[u], mapping[v]
        if not adv_are_comparable(parent, tu, tv):
            return None
        total += abs(depth[tu] - depth[tv])
    return total


def adv_solve_rta(n: int, edges: list[tuple[int, int]], bound: int) -> Optional[tuple[list[int], list[int]]]:
    """Brute-force RTA solver for small instances."""
    if n == 0:
        return ([], [])

    for root in range(n):
        for parent_choices in product(range(n), repeat=n):
            parent = list(parent_choices)
            if parent[root] != root:
                continue
            ok = True
            for i in range(n):
                if i != root and parent[i] == i:
                    ok = False
                    break
            if not ok:
                continue
            depth = adv_compute_depth(parent)
            if depth is None:
                continue
            for perm in permutations(range(n)):
                mapping = list(perm)
                cost = adv_rta_cost(n, edges, parent, mapping)
                if cost is not None and cost <= bound:
                    return (parent, mapping)
    return None


def adv_optimal_rta(n: int, edges: list[tuple[int, int]]) -> int:
    """Find minimum RTA cost."""
    if n == 0 or not edges:
        return 0
    best = float('inf')
    for root in range(n):
        for parent_choices in product(range(n), repeat=n):
            parent = list(parent_choices)
            if parent[root] != root:
                continue
            ok = True
            for i in range(n):
                if i != root and parent[i] == i:
                    ok = False
                    break
            if not ok:
                continue
            depth = adv_compute_depth(parent)
            if depth is None:
                continue
            for perm in permutations(range(n)):
                cost = adv_rta_cost(n, edges, parent, list(perm))
                if cost is not None and cost < best:
                    best = cost
    return best if best < float('inf') else 0


# ─────────────────────────────────────────────────────────────────────
# Property checks
# ─────────────────────────────────────────────────────────────────────

def adv_check_all(n: int, edges: list[tuple[int, int]], bound: int) -> int:
    """Run all adversary checks on a single instance. Returns check count."""
    checks = 0

    # 1. Overhead: identity reduction preserves everything
    rn, re, rb = adv_reduce(n, edges, bound)
    assert rn == n and re == edges and rb == bound, \
        f"Overhead: reduction should be identity"
    checks += 1

    # 2. Forward: OLA YES => RTA YES
    ola_sol = adv_solve_ola(n, edges, bound)
    rta_sol = adv_solve_rta(n, edges, bound)

    if ola_sol is not None:
        # Construct path tree and verify it's a valid RTA solution
        if n > 0:
            path_parent = [max(0, i - 1) for i in range(n)]
            path_parent[0] = 0
            cost = adv_rta_cost(n, edges, path_parent, ola_sol)
            assert cost is not None and cost <= bound, \
                f"Forward violation (path construction): n={n}, edges={edges}, bound={bound}"
        assert rta_sol is not None, \
            f"Forward violation: OLA feasible but RTA infeasible: n={n}, edges={edges}, bound={bound}"
        checks += 1

    # 3. Optimality gap: opt(RTA) <= opt(OLA)
    if edges and n >= 2:
        ola_opt = adv_optimal_ola(n, edges)
        rta_opt = adv_optimal_rta(n, edges)
        assert rta_opt <= ola_opt, \
            f"Gap violation: rta_opt={rta_opt} > ola_opt={ola_opt}, n={n}, edges={edges}"
        checks += 1

    # 4. Contrapositive: RTA NO => OLA NO
    if rta_sol is None:
        assert ola_sol is None, \
            f"Contrapositive violation: RTA infeasible but OLA feasible"
        checks += 1

    # 5. Cross-check: OLA solution cost matches claim
    if ola_sol is not None:
        cost = adv_ola_cost(n, edges, ola_sol)
        assert cost <= bound, \
            f"OLA solution invalid: cost {cost} > bound {bound}"
        checks += 1

    return checks


# ─────────────────────────────────────────────────────────────────────
# Graph generation helpers
# ─────────────────────────────────────────────────────────────────────

def adv_all_graphs(n: int):
    """Generate all simple undirected graphs on n vertices."""
    possible = [(i, j) for i in range(n) for j in range(i + 1, n)]
    for mask in range(1 << len(possible)):
        edges = [possible[b] for b in range(len(possible)) if mask & (1 << b)]
        yield edges


def adv_random_graph(n: int, rng) -> list[tuple[int, int]]:
    """Random graph generation with different strategy from verify script."""
    edges = []
    for i in range(n):
        for j in range(i + 1, n):
            if rng.random() < 0.35:
                edges.append((i, j))
    return edges


# ─────────────────────────────────────────────────────────────────────
# Test drivers
# ─────────────────────────────────────────────────────────────────────

def adversary_exhaustive(max_n: int = 4) -> int:
    """Exhaustive adversary checks for all graphs n <= max_n."""
    checks = 0
    for n in range(0, max_n + 1):
        for edges in adv_all_graphs(n):
            m = len(edges)
            max_bound = min(n * n, n * m + 1) if m > 0 else 2
            for bound in range(0, min(max_bound + 1, 18)):
                checks += adv_check_all(n, edges, bound)
    return checks


def adversary_random(count: int = 800, max_n: int = 4) -> int:
    """Random adversary tests with independent RNG seed."""
    import random
    rng = random.Random(9999)  # Different seed from verify script
    checks = 0
    for _ in range(count):
        n = rng.randint(1, max_n)
        edges = adv_random_graph(n, rng)
        m = len(edges)
        max_cost = n * m if m > 0 else 1
        bound = rng.randint(0, min(max_cost + 2, 20))
        checks += adv_check_all(n, edges, bound)
    return checks


def adversary_star_family() -> int:
    """Test star graphs which are known to exhibit OLA/RTA gaps."""
    checks = 0
    for k in range(2, 6):
        n = k + 1
        edges = [(0, i) for i in range(1, n)]
        rta_opt = adv_optimal_rta(n, edges)
        ola_opt = adv_optimal_ola(n, edges)

        assert rta_opt == k, f"Star K_{{1,{k}}}: expected rta_opt={k}, got {rta_opt}"
        assert rta_opt <= ola_opt, f"Star K_{{1,{k}}}: gap violation"
        checks += 2

        # Verify gap bounds
        for b in range(rta_opt, ola_opt):
            rta_feas = adv_solve_rta(n, edges, b) is not None
            ola_feas = adv_solve_ola(n, edges, b) is not None
            assert rta_feas and not ola_feas, \
                f"Star K_{{1,{k}}}, bound={b}: expected gap"
            checks += 1

    return checks


def adversary_hypothesis() -> int:
    """Property-based testing with hypothesis."""
    if not HAS_HYPOTHESIS:
        return 0

    checks_counter = [0]

    # Strategy for small graphs
    @st.composite
    def graph_instance(draw):
        n = draw(st.integers(min_value=1, max_value=4))
        possible_edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
        edge_mask = draw(st.integers(min_value=0, max_value=(1 << len(possible_edges)) - 1))
        edges = [possible_edges[b] for b in range(len(possible_edges)) if edge_mask & (1 << b)]
        m = len(edges)
        max_cost = n * m if m > 0 else 1
        bound = draw(st.integers(min_value=0, max_value=min(max_cost + 2, 20)))
        return (n, edges, bound)

    @given(instance=graph_instance())
    @settings(
        max_examples=1500,
        suppress_health_check=[HealthCheck.too_slow],
        deadline=None,
    )
    def prop_forward_direction(instance):
        n, edges, bound = instance
        checks_counter[0] += adv_check_all(n, edges, bound)

    prop_forward_direction()
    return checks_counter[0]


def adversary_edge_cases() -> int:
    """Targeted edge cases."""
    checks = 0
    cases = [
        # Empty graph
        (0, [], 0),
        (1, [], 0),
        (2, [], 0),
        # Single edge
        (2, [(0, 1)], 0),
        (2, [(0, 1)], 1),
        (2, [(0, 1)], 2),
        # Triangle
        (3, [(0, 1), (1, 2), (0, 2)], 2),
        (3, [(0, 1), (1, 2), (0, 2)], 3),
        (3, [(0, 1), (1, 2), (0, 2)], 4),
        # Path P3
        (3, [(0, 1), (1, 2)], 1),
        (3, [(0, 1), (1, 2)], 2),
        (3, [(0, 1), (1, 2)], 3),
        # Star K_{1,2}
        (3, [(0, 1), (0, 2)], 1),
        (3, [(0, 1), (0, 2)], 2),
        (3, [(0, 1), (0, 2)], 3),
        # K4
        (4, [(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)], 5),
        (4, [(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)], 10),
        (4, [(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)], 15),
        # Star K_{1,3}
        (4, [(0,1),(0,2),(0,3)], 2),
        (4, [(0,1),(0,2),(0,3)], 3),
        (4, [(0,1),(0,2),(0,3)], 4),
        (4, [(0,1),(0,2),(0,3)], 5),
    ]
    for n, edges, bound in cases:
        checks += adv_check_all(n, edges, bound)
    return checks


if __name__ == "__main__":
    print("=" * 60)
    print("Adversary verification: OLA → RTA")
    print("=" * 60)

    print("\n[1/5] Edge cases...")
    n_edge = adversary_edge_cases()
    print(f"  Edge case checks: {n_edge}")

    print("\n[2/5] Star family tests...")
    n_star = adversary_star_family()
    print(f"  Star family checks: {n_star}")

    print("\n[3/5] Exhaustive adversary (n ≤ 4)...")
    n_exh = adversary_exhaustive()
    print(f"  Exhaustive checks: {n_exh}")

    print("\n[4/5] Random adversary (different seed)...")
    n_rand = adversary_random()
    print(f"  Random checks: {n_rand}")

    print("\n[5/5] Hypothesis PBT...")
    n_hyp = adversary_hypothesis()
    print(f"  Hypothesis checks: {n_hyp}")

    total = n_edge + n_star + n_exh + n_rand + n_hyp
    print(f"\n  TOTAL adversary checks: {total}")
    assert total >= 5000, f"Need ≥5000 checks, got {total}"
    print(f"\nAll {total} adversary checks PASSED.")
