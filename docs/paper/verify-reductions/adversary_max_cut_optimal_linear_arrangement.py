#!/usr/bin/env python3
"""
Adversary verification script: MaxCut → OptimalLinearArrangement reduction.
Issue: #890

Independent re-implementation of the reduction and extraction logic,
plus property-based testing with hypothesis. ≥5000 independent checks.

This script does NOT import from verify_max_cut_optimal_linear_arrangement.py —
it re-derives everything from scratch as an independent cross-check.
"""

import json
import sys
from itertools import permutations, product, combinations
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

def adv_reduce(n: int, edges: list[tuple[int, int]]) -> tuple[int, list[tuple[int, int]]]:
    """Independent reduction: MaxCut → OLA. Same graph passed through."""
    return (n, edges[:])


def adv_positional_cuts(n: int, edges: list[tuple[int, int]], arrangement: list[int]) -> list[int]:
    """
    Compute positional cuts for an arrangement.
    Returns list of n-1 cut sizes: c_i = edges crossing position i.
    """
    cuts = []
    for cut_pos in range(n - 1):
        c = 0
        for u, v in edges:
            fu, fv = arrangement[u], arrangement[v]
            if (fu <= cut_pos) != (fv <= cut_pos):
                c += 1
        cuts.append(c)
    return cuts


def adv_arrangement_cost(n: int, edges: list[tuple[int, int]], arrangement: list[int]) -> int:
    """Compute total arrangement cost."""
    return sum(abs(arrangement[u] - arrangement[v]) for u, v in edges)


def adv_cut_value(n: int, edges: list[tuple[int, int]], partition: list[int]) -> int:
    """Compute the cut value for a binary partition."""
    return sum(1 for u, v in edges if partition[u] != partition[v])


def adv_extract(n: int, edges: list[tuple[int, int]], arrangement: list[int]) -> list[int]:
    """
    Independent extraction: OLA arrangement → MaxCut partition.
    Pick the positional cut with maximum crossing edges.
    """
    if n <= 1:
        return [0] * n

    best_pos = 0
    best_val = -1
    for cut_pos in range(n - 1):
        c = 0
        for u, v in edges:
            fu, fv = arrangement[u], arrangement[v]
            if (fu <= cut_pos) != (fv <= cut_pos):
                c += 1
        if c > best_val:
            best_val = c
            best_pos = cut_pos

    return [0 if arrangement[v] <= best_pos else 1 for v in range(n)]


def adv_solve_max_cut(n: int, edges: list[tuple[int, int]]) -> tuple[int, Optional[list[int]]]:
    """Brute-force MaxCut solver."""
    if n == 0:
        return (0, [])
    best_val = -1
    best_cfg = None
    for cfg in product(range(2), repeat=n):
        cfg = list(cfg)
        val = adv_cut_value(n, edges, cfg)
        if val > best_val:
            best_val = val
            best_cfg = cfg
    return (best_val, best_cfg)


def adv_solve_ola(n: int, edges: list[tuple[int, int]]) -> tuple[int, Optional[list[int]]]:
    """Brute-force OLA solver."""
    if n == 0:
        return (0, [])
    best_val = float('inf')
    best_arr = None
    for perm in permutations(range(n)):
        arr = list(perm)
        val = adv_arrangement_cost(n, edges, arr)
        if val < best_val:
            best_val = val
            best_arr = arr
    return (best_val, best_arr)


# ─────────────────────────────────────────────────────────────────────
# Property checks
# ─────────────────────────────────────────────────────────────────────

def adv_check_all(n: int, edges: list[tuple[int, int]]) -> int:
    """Run all adversary checks on a single graph instance. Returns check count."""
    checks = 0
    m = len(edges)

    # 1. Overhead: same graph
    n2, edges2 = adv_reduce(n, edges)
    assert n2 == n, f"Overhead violation: n changed from {n} to {n2}"
    assert len(edges2) == m, f"Overhead violation: m changed from {m} to {len(edges2)}"
    checks += 1

    if n <= 1:
        return checks

    # 2. Solve both problems
    mc_val, mc_sol = adv_solve_max_cut(n, edges)
    ola_val, ola_arr = adv_solve_ola(n, edges)
    checks += 1

    # 3. Core identity: cost = sum of positional cuts
    if ola_arr is not None:
        cuts = adv_positional_cuts(n, edges, ola_arr)
        assert sum(cuts) == ola_val, (
            f"Positional cut identity failed: sum={sum(cuts)} != ola={ola_val}, "
            f"n={n}, edges={edges}"
        )
        checks += 1

    # 4. Key inequality: max_cut * (n-1) >= OLA
    assert mc_val * (n - 1) >= ola_val, (
        f"Key inequality failed: mc={mc_val}, ola={ola_val}, n={n}, edges={edges}"
    )
    checks += 1

    # 5. Lower bound: OLA >= m
    assert ola_val >= m, (
        f"Lower bound failed: ola={ola_val} < m={m}, n={n}, edges={edges}"
    )
    checks += 1

    # 6. Extraction: from optimal OLA arrangement, extract a valid partition
    if ola_arr is not None:
        extracted = adv_extract(n, edges, ola_arr)
        assert len(extracted) == n and all(x in (0, 1) for x in extracted), (
            f"Extraction produced invalid partition: {extracted}"
        )
        extracted_cut = adv_cut_value(n, edges, extracted)

        # The extracted cut must be >= OLA / (n-1) (pigeonhole)
        assert extracted_cut * (n - 1) >= ola_val, (
            f"Extraction quality: extracted_cut={extracted_cut}, "
            f"ola={ola_val}, n={n}, edges={edges}"
        )
        checks += 1

    # 7. Cross-check: verify on ALL arrangements that cost = sum of positional cuts
    if n <= 5:
        for perm in permutations(range(n)):
            arr = list(perm)
            cost = adv_arrangement_cost(n, edges, arr)
            cuts = adv_positional_cuts(n, edges, arr)
            assert sum(cuts) == cost, (
                f"Identity failed for arr={arr}: sum(cuts)={sum(cuts)}, cost={cost}"
            )
            # max positional cut <= max_cut
            if cuts:
                assert max(cuts) <= mc_val, (
                    f"Max positional cut {max(cuts)} > max_cut {mc_val}"
                )
            checks += 1

    return checks


# ─────────────────────────────────────────────────────────────────────
# Test drivers
# ─────────────────────────────────────────────────────────────────────

def adversary_exhaustive(max_n: int = 5) -> int:
    """Exhaustive adversary tests on all graphs up to max_n vertices."""
    checks = 0
    for n in range(1, max_n + 1):
        all_possible_edges = list(combinations(range(n), 2))
        for r in range(len(all_possible_edges) + 1):
            for edge_subset in combinations(all_possible_edges, r):
                checks += adv_check_all(n, list(edge_subset))
    return checks


def adversary_random(count: int = 1500, max_n: int = 7) -> int:
    """Random adversary tests with independent RNG seed."""
    import random
    rng = random.Random(9999)  # Different seed from verify script
    checks = 0
    for _ in range(count):
        n = rng.randint(2, max_n)
        all_possible = list(combinations(range(n), 2))
        num_edges = rng.randint(0, len(all_possible))
        edges = rng.sample(all_possible, num_edges)
        checks += adv_check_all(n, edges)
    return checks


def adversary_hypothesis() -> int:
    """Property-based testing with hypothesis."""
    if not HAS_HYPOTHESIS:
        return 0

    checks_counter = [0]

    @st.composite
    def graph_strategy(draw):
        """Generate a random simple undirected graph."""
        n = draw(st.integers(min_value=2, max_value=6))
        all_possible = list(combinations(range(n), 2))
        # Pick a random subset of edges
        edge_mask = draw(st.lists(
            st.booleans(), min_size=len(all_possible), max_size=len(all_possible)
        ))
        edges = [e for e, include in zip(all_possible, edge_mask) if include]
        return n, edges

    @given(graph=graph_strategy())
    @settings(
        max_examples=1000,
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
    edge_cases = [
        # Single vertex
        (1, []),
        # Single edge
        (2, [(0, 1)]),
        # Two vertices, no edge
        (2, []),
        # Triangle
        (3, [(0, 1), (1, 2), (0, 2)]),
        # Path of length 3
        (4, [(0, 1), (1, 2), (2, 3)]),
        # Complete K4
        (4, list(combinations(range(4), 2))),
        # Complete K5
        (5, list(combinations(range(5), 2))),
        # Star with 6 leaves
        (7, [(0, i) for i in range(1, 7)]),
        # Two disjoint triangles
        (6, [(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5)]),
        # Complete bipartite K3,3
        (6, [(i, 3+j) for i in range(3) for j in range(3)]),
        # Cycle C6
        (6, [(i, (i+1) % 6) for i in range(6)]),
        # Empty graph on 5 vertices
        (5, []),
        # Petersen graph
        (10, [(i, (i+1) % 5) for i in range(5)] +
              [(5+i, 5+(i+2) % 5) for i in range(5)] +
              [(i, 5+i) for i in range(5)]),
    ]
    for n, edges in edge_cases:
        checks += adv_check_all(n, edges)
    return checks


if __name__ == "__main__":
    print("=" * 60)
    print("Adversary verification: MaxCut → OptimalLinearArrangement")
    print("=" * 60)

    print("\n[1/4] Edge cases...")
    n_edge = adversary_edge_cases()
    print(f"  Edge case checks: {n_edge}")

    print("\n[2/4] Exhaustive adversary (n ≤ 5)...")
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
    assert total >= 5000, f"Need ≥5000 checks, got {total}"
    print(f"\nAll {total} adversary checks PASSED.")
