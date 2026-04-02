#!/usr/bin/env python3
"""
Adversarial property-based testing: MinimumVertexCover -> MinimumMaximalMatching
Issue: #893 (CodingThrust/problem-reductions)

Uses hypothesis to generate random graph instances and verify all reduction
properties. Targets >= 5000 checks.

Properties tested:
  P1: Forward map produces a valid maximal matching.
  P2: Forward matching size <= |vertex cover|.
  P3: Reverse endpoint extraction produces a valid vertex cover.
  P4: Reverse VC size <= 2 * |matching|.
  P5: Bounds inequality: mmm(G) <= vc(G) <= 2*mmm(G).
  P6: Every VC witness maps to a valid maximal matching via forward map.
  P7: Every MMM witness maps to a valid VC via reverse map.

Usage:
    pip install hypothesis
    python adversary_minimum_vertex_cover_minimum_maximal_matching.py
"""

from __future__ import annotations

import itertools
import random
import sys
from collections import Counter

try:
    from hypothesis import given, settings, assume, HealthCheck
    from hypothesis import strategies as st
except ImportError:
    print("ERROR: hypothesis not installed. Run: pip install hypothesis")
    sys.exit(1)


# ─────────────────────────── helpers ──────────────────────────────────

def is_vertex_cover(n: int, edges: list[tuple[int, int]], cover: set[int]) -> bool:
    return all(u in cover or v in cover for u, v in edges)


def is_matching(edges: list[tuple[int, int]], sel: set[int]) -> bool:
    used: set[int] = set()
    for i in sel:
        u, v = edges[i]
        if u in used or v in used:
            return False
        used.add(u)
        used.add(v)
    return True


def is_maximal_matching(n: int, edges: list[tuple[int, int]], sel: set[int]) -> bool:
    if not is_matching(edges, sel):
        return False
    used: set[int] = set()
    for i in sel:
        u, v = edges[i]
        used.add(u)
        used.add(v)
    for j in range(len(edges)):
        if j not in sel:
            u, v = edges[j]
            if u not in used and v not in used:
                return False
    return True


def brute_min_vc(n: int, edges: list[tuple[int, int]]) -> tuple[int, list[int]]:
    for size in range(n + 1):
        for cover in itertools.combinations(range(n), size):
            if is_vertex_cover(n, edges, set(cover)):
                return size, list(cover)
    return n, list(range(n))


def brute_min_mmm(n: int, edges: list[tuple[int, int]]) -> tuple[int, set[int]]:
    for size in range(len(edges) + 1):
        for sel in itertools.combinations(range(len(edges)), size):
            if is_maximal_matching(n, edges, set(sel)):
                return size, set(sel)
    return len(edges), set(range(len(edges)))


def vc_to_maximal_matching(n: int, edges: list[tuple[int, int]], cover: list[int]) -> set[int]:
    """Greedy forward map: vertex cover -> maximal matching of size <= |cover|."""
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for idx, (u, v) in enumerate(edges):
        adj[u].append((v, idx))
        adj[v].append((u, idx))
    matched_verts: set[int] = set()
    matching: set[int] = set()
    for v in cover:
        if v in matched_verts:
            continue
        for u, idx in adj[v]:
            if u not in matched_verts:
                matching.add(idx)
                matched_verts.add(v)
                matched_verts.add(u)
                break
    return matching


def mmm_to_vc_endpoints(edges: list[tuple[int, int]], matching: set[int]) -> set[int]:
    """Reverse map: maximal matching -> vertex cover via all endpoints."""
    cover: set[int] = set()
    for i in matching:
        u, v = edges[i]
        cover.add(u)
        cover.add(v)
    return cover


# ──────────────────── hypothesis strategies ───────────────────────────

@st.composite
def graph_strategy(draw, min_n: int = 2, max_n: int = 9) -> tuple[int, list[tuple[int, int]]]:
    """Generate a random graph with no isolated vertices."""
    n = draw(st.integers(min_value=min_n, max_value=max_n))
    all_edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
    if not all_edges:
        assume(False)
    subset = draw(st.lists(
        st.sampled_from(all_edges),
        min_size=1,
        max_size=len(all_edges),
        unique=True,
    ))
    edges = sorted(set(subset))
    # Check no isolated vertices
    deg = [0] * n
    for u, v in edges:
        deg[u] += 1
        deg[v] += 1
    assume(all(deg[v] > 0 for v in range(n)))
    return n, edges


@st.composite
def connected_graph_strategy(draw, min_n: int = 3, max_n: int = 9) -> tuple[int, list[tuple[int, int]]]:
    """Generate a random connected graph."""
    n = draw(st.integers(min_value=min_n, max_value=max_n))
    # Random spanning tree
    perm = draw(st.permutations(list(range(n))))
    edges_set: set[tuple[int, int]] = set()
    for i in range(1, n):
        parent_idx = draw(st.integers(min_value=0, max_value=i - 1))
        u, v = perm[i], perm[parent_idx]
        edges_set.add((min(u, v), max(u, v)))
    # Extra edges
    all_non_tree = [(i, j) for i in range(n) for j in range(i + 1, n) if (i, j) not in edges_set]
    if all_non_tree:
        extra = draw(st.lists(
            st.sampled_from(all_non_tree),
            min_size=0,
            max_size=min(len(all_non_tree), n),
            unique=True,
        ))
        edges_set.update(extra)
    return n, sorted(edges_set)


# ─────────────────── property-based tests ─────────────────────────────

CHECKS = Counter()

@given(graph=graph_strategy())
@settings(max_examples=700, suppress_health_check=[HealthCheck.too_slow])
def test_p1_forward_valid(graph: tuple[int, list[tuple[int, int]]]) -> None:
    """P1: Forward map produces a valid maximal matching."""
    n, edges = graph
    vc_size, vc_verts = brute_min_vc(n, edges)
    matching = vc_to_maximal_matching(n, edges, vc_verts)
    assert is_maximal_matching(n, edges, matching)
    CHECKS["P1"] += 1


@given(graph=graph_strategy())
@settings(max_examples=700, suppress_health_check=[HealthCheck.too_slow])
def test_p2_forward_size(graph: tuple[int, list[tuple[int, int]]]) -> None:
    """P2: Forward matching size <= |vertex cover|."""
    n, edges = graph
    vc_size, vc_verts = brute_min_vc(n, edges)
    matching = vc_to_maximal_matching(n, edges, vc_verts)
    assert len(matching) <= vc_size
    CHECKS["P2"] += 1


@given(graph=graph_strategy())
@settings(max_examples=700, suppress_health_check=[HealthCheck.too_slow])
def test_p3_reverse_valid(graph: tuple[int, list[tuple[int, int]]]) -> None:
    """P3: Reverse endpoint extraction produces a valid vertex cover."""
    n, edges = graph
    mmm_size, mmm_sel = brute_min_mmm(n, edges)
    vc = mmm_to_vc_endpoints(edges, mmm_sel)
    assert is_vertex_cover(n, edges, vc)
    CHECKS["P3"] += 1


@given(graph=graph_strategy())
@settings(max_examples=700, suppress_health_check=[HealthCheck.too_slow])
def test_p4_reverse_size(graph: tuple[int, list[tuple[int, int]]]) -> None:
    """P4: Reverse VC size <= 2 * |matching|."""
    n, edges = graph
    mmm_size, mmm_sel = brute_min_mmm(n, edges)
    vc = mmm_to_vc_endpoints(edges, mmm_sel)
    assert len(vc) <= 2 * mmm_size
    CHECKS["P4"] += 1


@given(graph=graph_strategy())
@settings(max_examples=700, suppress_health_check=[HealthCheck.too_slow])
def test_p5_bounds(graph: tuple[int, list[tuple[int, int]]]) -> None:
    """P5: mmm(G) <= vc(G) <= 2*mmm(G)."""
    n, edges = graph
    vc_size, _ = brute_min_vc(n, edges)
    mmm_size, _ = brute_min_mmm(n, edges)
    assert mmm_size <= vc_size
    assert vc_size <= 2 * mmm_size
    CHECKS["P5"] += 1


@given(graph=connected_graph_strategy(min_n=3, max_n=7))
@settings(max_examples=700, suppress_health_check=[HealthCheck.too_slow])
def test_p6_all_vc_witnesses(graph: tuple[int, list[tuple[int, int]]]) -> None:
    """P6: Every VC witness maps to a valid maximal matching."""
    n, edges = graph
    vc_size, _ = brute_min_vc(n, edges)
    count = 0
    for cover in itertools.combinations(range(n), vc_size):
        if is_vertex_cover(n, edges, set(cover)):
            matching = vc_to_maximal_matching(n, edges, list(cover))
            assert is_maximal_matching(n, edges, matching)
            assert len(matching) <= vc_size
            count += 1
            if count >= 10:
                break
    CHECKS["P6"] += count


@given(graph=connected_graph_strategy(min_n=3, max_n=7))
@settings(max_examples=700, suppress_health_check=[HealthCheck.too_slow])
def test_p7_all_mmm_witnesses(graph: tuple[int, list[tuple[int, int]]]) -> None:
    """P7: Every MMM witness maps to a valid VC via reverse map."""
    n, edges = graph
    mmm_size, _ = brute_min_mmm(n, edges)
    count = 0
    for sel in itertools.combinations(range(len(edges)), mmm_size):
        if is_maximal_matching(n, edges, set(sel)):
            vc = mmm_to_vc_endpoints(edges, set(sel))
            assert is_vertex_cover(n, edges, vc)
            assert len(vc) <= 2 * mmm_size
            count += 1
            if count >= 10:
                break
    CHECKS["P7"] += count


# ────────────────────────── main ──────────────────────────────────────

def main() -> None:
    print("Adversarial PBT: MinimumVertexCover -> MinimumMaximalMatching")
    print("=" * 60)

    tests = [
        ("P1: forward valid", test_p1_forward_valid),
        ("P2: forward size", test_p2_forward_size),
        ("P3: reverse valid", test_p3_reverse_valid),
        ("P4: reverse size", test_p4_reverse_size),
        ("P5: bounds inequality", test_p5_bounds),
        ("P6: all VC witnesses", test_p6_all_vc_witnesses),
        ("P7: all MMM witnesses", test_p7_all_mmm_witnesses),
    ]

    for name, test_fn in tests:
        try:
            test_fn()
            print(f"  {name}: PASSED")
        except Exception as e:
            print(f"  {name}: FAILED -- {e}")
            sys.exit(1)

    total = sum(CHECKS.values())
    print("=" * 60)
    print("Check counts per property:")
    for key in sorted(CHECKS):
        print(f"  {key}: {CHECKS[key]}")
    print(f"TOTAL: {total} checks")
    assert total >= 5000, f"Expected >= 5000 checks, got {total}"
    print("ALL ADVERSARIAL CHECKS PASSED >= 5000")


if __name__ == "__main__":
    main()
