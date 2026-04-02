#!/usr/bin/env python3
"""
Verification script: MinimumVertexCover -> MinimumMaximalMatching
Issue: #893 (CodingThrust/problem-reductions)

Seven sections, >=5000 total checks.
Reduction: same graph, same bound K.
Forward: VC of size K => maximal matching of size <= K.
Reverse: maximal matching of size K' => VC of size <= 2K'.

Usage:
    python verify_minimum_vertex_cover_minimum_maximal_matching.py
"""

from __future__ import annotations

import itertools
import json
import random
import sys
from collections import defaultdict
from pathlib import Path
from typing import Optional

# ─────────────────────────── helpers ──────────────────────────────────

def edges_list(n: int, edge_tuples: list[tuple[int, int]]) -> list[tuple[int, int]]:
    """Normalise edge list (sorted endpoints, deduplicated)."""
    seen = set()
    out = []
    for u, v in edge_tuples:
        a, b = min(u, v), max(u, v)
        if (a, b) not in seen:
            seen.add((a, b))
            out.append((a, b))
    return out


def adjacency(n: int, edges: list[tuple[int, int]]) -> list[list[tuple[int, int]]]:
    """Build adjacency list: adj[v] = list of (neighbour, edge_index)."""
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for idx, (u, v) in enumerate(edges):
        adj[u].append((v, idx))
        adj[v].append((u, idx))
    return adj


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


def is_maximal_matching(
    n: int, edges: list[tuple[int, int]], sel: set[int]
) -> bool:
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


def vc_to_maximal_matching(
    n: int, edges: list[tuple[int, int]], cover: list[int]
) -> set[int]:
    """Greedy forward map: vertex cover -> maximal matching of size <= |cover|."""
    adj = adjacency(n, edges)
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


def mmm_to_vc_endpoints(
    edges: list[tuple[int, int]], matching: set[int]
) -> set[int]:
    """Reverse map: maximal matching -> vertex cover via all endpoints."""
    cover: set[int] = set()
    for i in matching:
        u, v = edges[i]
        cover.add(u)
        cover.add(v)
    return cover


# ─────────────────── named graph generators ───────────────────────────

def path_graph(n: int) -> tuple[int, list[tuple[int, int]]]:
    return n, [(i, i + 1) for i in range(n - 1)]


def cycle_graph(n: int) -> tuple[int, list[tuple[int, int]]]:
    return n, [(i, (i + 1) % n) for i in range(n)]


def complete_graph(n: int) -> tuple[int, list[tuple[int, int]]]:
    return n, [(i, j) for i in range(n) for j in range(i + 1, n)]


def star_graph(k: int) -> tuple[int, list[tuple[int, int]]]:
    """Star K_{1,k}: center 0, leaves 1..k."""
    return k + 1, [(0, i) for i in range(1, k + 1)]


def petersen_graph() -> tuple[int, list[tuple[int, int]]]:
    return 10, [
        (0, 1), (0, 4), (0, 5), (1, 2), (1, 6), (2, 3), (2, 7),
        (3, 4), (3, 8), (4, 9), (5, 7), (5, 8), (6, 8), (6, 9), (7, 9),
    ]


def prism_graph() -> tuple[int, list[tuple[int, int]]]:
    """Triangular prism C3 x K2."""
    return 6, [(0, 1), (1, 2), (0, 2), (3, 4), (4, 5), (3, 5), (0, 3), (1, 4), (2, 5)]


def bipartite_complete(a: int, b: int) -> tuple[int, list[tuple[int, int]]]:
    return a + b, [(i, a + j) for i in range(a) for j in range(b)]


def random_graph(n: int, p: float, rng: random.Random) -> tuple[int, list[tuple[int, int]]]:
    edges = []
    for i in range(n):
        for j in range(i + 1, n):
            if rng.random() < p:
                edges.append((i, j))
    return n, edges


def random_connected_graph(n: int, extra: int, rng: random.Random) -> tuple[int, list[tuple[int, int]]]:
    """Random tree + extra random edges."""
    edges_set: set[tuple[int, int]] = set()
    # Random spanning tree
    verts = list(range(n))
    rng.shuffle(verts)
    for i in range(1, n):
        u = verts[i]
        v = verts[rng.randint(0, i - 1)]
        a, b = min(u, v), max(u, v)
        edges_set.add((a, b))
    # Extra edges
    all_possible = [(i, j) for i in range(n) for j in range(i + 1, n) if (i, j) not in edges_set]
    extras = min(extra, len(all_possible))
    for e in rng.sample(all_possible, extras):
        edges_set.add(e)
    return n, sorted(edges_set)


def cubic_random(n: int, rng: random.Random) -> Optional[tuple[int, list[tuple[int, int]]]]:
    """Try to generate a random cubic (3-regular) graph on n vertices (n even)."""
    if n % 2 != 0 or n < 4:
        return None
    for _attempt in range(100):
        stubs = []
        for v in range(n):
            stubs.extend([v, v, v])
        rng.shuffle(stubs)
        edges_set: set[tuple[int, int]] = set()
        ok = True
        for i in range(0, len(stubs), 2):
            u, v = stubs[i], stubs[i + 1]
            if u == v:
                ok = False
                break
            a, b = min(u, v), max(u, v)
            if (a, b) in edges_set:
                ok = False
                break
            edges_set.add((a, b))
        if ok and all(sum(1 for a, b in edges_set if a == v or b == v) == 3 for v in range(n)):
            return n, sorted(edges_set)
    return None


# ────────────────────────── Section 1 ─────────────────────────────────

def section1_named_graphs() -> int:
    """Section 1: Verify on named graphs (paths, cycles, stars, Petersen, etc.)."""
    checks = 0
    named = [
        ("P2", *path_graph(2)),
        ("P3", *path_graph(3)),
        ("P4", *path_graph(4)),
        ("P5", *path_graph(5)),
        ("P6", *path_graph(6)),
        ("P7", *path_graph(7)),
        ("C3", *cycle_graph(3)),
        ("C4", *cycle_graph(4)),
        ("C5", *cycle_graph(5)),
        ("C6", *cycle_graph(6)),
        ("C7", *cycle_graph(7)),
        ("K3", *complete_graph(3)),
        ("K4", *complete_graph(4)),
        ("K5", *complete_graph(5)),
        ("S3", *star_graph(3)),
        ("S4", *star_graph(4)),
        ("S5", *star_graph(5)),
        ("K2,2", *bipartite_complete(2, 2)),
        ("K2,3", *bipartite_complete(2, 3)),
        ("K3,3", *bipartite_complete(3, 3)),
        ("Petersen", *petersen_graph()),
        ("Prism", *prism_graph()),
    ]
    for name, n, edges in named:
        if not edges:
            continue
        vc_size, vc_verts = brute_min_vc(n, edges)
        mmm_size, mmm_sel = brute_min_mmm(n, edges)

        # Check 1: mmm <= vc
        assert mmm_size <= vc_size, f"{name}: mmm={mmm_size} > vc={vc_size}"
        checks += 1

        # Check 2: vc <= 2*mmm
        assert vc_size <= 2 * mmm_size, f"{name}: vc={vc_size} > 2*mmm={2*mmm_size}"
        checks += 1

        # Check 3: forward construction produces valid maximal matching
        matching = vc_to_maximal_matching(n, edges, vc_verts)
        assert is_maximal_matching(n, edges, matching), f"{name}: forward matching not maximal"
        checks += 1

        # Check 4: forward matching size <= vc
        assert len(matching) <= vc_size, f"{name}: forward matching size {len(matching)} > vc {vc_size}"
        checks += 1

        # Check 5: reverse extraction from brute mmm produces valid vc
        vc_extracted = mmm_to_vc_endpoints(edges, mmm_sel)
        assert is_vertex_cover(n, edges, vc_extracted), f"{name}: reverse vc invalid"
        checks += 1

        # Check 6: reverse vc size <= 2*mmm
        assert len(vc_extracted) <= 2 * mmm_size, f"{name}: reverse vc size {len(vc_extracted)} > 2*mmm"
        checks += 1

    print(f"  Section 1 (named graphs): {checks} checks PASSED")
    return checks


# ────────────────────────── Section 2 ─────────────────────────────────

def section2_forward_construction() -> int:
    """Section 2: Forward VC -> MMM on random graphs."""
    checks = 0
    rng = random.Random(42)
    for _ in range(500):
        n = rng.randint(2, 8)
        n_graph, edges = random_graph(n, rng.uniform(0.3, 0.8), rng)
        if not edges:
            continue
        # Remove isolated vertices
        adj = [set() for _ in range(n_graph)]
        for u, v in edges:
            adj[u].add(v)
            adj[v].add(u)
        if any(len(adj[v]) == 0 for v in range(n_graph)):
            continue

        vc_size, vc_verts = brute_min_vc(n_graph, edges)
        matching = vc_to_maximal_matching(n_graph, edges, vc_verts)

        # Check validity
        assert is_maximal_matching(n_graph, edges, matching), f"forward matching not maximal"
        checks += 1

        # Check size
        assert len(matching) <= vc_size, f"forward size {len(matching)} > vc {vc_size}"
        checks += 1

    print(f"  Section 2 (forward construction): {checks} checks PASSED")
    return checks


# ────────────────────────── Section 3 ─────────────────────────────────

def section3_reverse_extraction() -> int:
    """Section 3: Reverse MMM -> VC endpoint extraction on random graphs."""
    checks = 0
    rng = random.Random(123)
    for _ in range(500):
        n = rng.randint(2, 8)
        n_graph, edges = random_graph(n, rng.uniform(0.3, 0.8), rng)
        if not edges:
            continue
        adj = [set() for _ in range(n_graph)]
        for u, v in edges:
            adj[u].add(v)
            adj[v].add(u)
        if any(len(adj[v]) == 0 for v in range(n_graph)):
            continue

        mmm_size, mmm_sel = brute_min_mmm(n_graph, edges)
        vc_extracted = mmm_to_vc_endpoints(edges, mmm_sel)

        # Check: valid vertex cover
        assert is_vertex_cover(n_graph, edges, vc_extracted), "reverse vc invalid"
        checks += 1

        # Check: size <= 2 * mmm
        assert len(vc_extracted) <= 2 * mmm_size, f"reverse size {len(vc_extracted)} > 2*{mmm_size}"
        checks += 1

    print(f"  Section 3 (reverse extraction): {checks} checks PASSED")
    return checks


# ────────────────────────── Section 4 ─────────────────────────────────

def section4_bounds_inequality() -> int:
    """Section 4: Verify mmm(G) <= vc(G) <= 2*mmm(G) on exhaustive small graphs."""
    checks = 0
    for n in range(2, 8):
        all_possible = [(i, j) for i in range(n) for j in range(i + 1, n)]
        # Sample subsets of edges
        rng = random.Random(n * 1000 + 4)
        num_samples = min(200, 2 ** len(all_possible))
        seen: set[frozenset[tuple[int, int]]] = set()
        for _ in range(num_samples * 3):
            if len(seen) >= num_samples:
                break
            m = rng.randint(1, len(all_possible))
            edges = tuple(sorted(rng.sample(all_possible, m)))
            fs = frozenset(edges)
            if fs in seen:
                continue
            seen.add(fs)
            edges_list_local = list(edges)
            # Check no isolated vertices
            adj = [0] * n
            for u, v in edges_list_local:
                adj[u] += 1
                adj[v] += 1
            if any(adj[v] == 0 for v in range(n)):
                continue

            vc_size, _ = brute_min_vc(n, edges_list_local)
            mmm_size, _ = brute_min_mmm(n, edges_list_local)

            assert mmm_size <= vc_size, f"n={n} edges={edges}: mmm={mmm_size} > vc={vc_size}"
            checks += 1
            assert vc_size <= 2 * mmm_size, f"n={n} edges={edges}: vc={vc_size} > 2*mmm={2*mmm_size}"
            checks += 1

    print(f"  Section 4 (bounds inequality): {checks} checks PASSED")
    return checks


# ────────────────────────── Section 5 ─────────────────────────────────

def section5_cubic_graphs() -> int:
    """Section 5: Verify on cubic (3-regular) graphs specifically."""
    checks = 0
    rng = random.Random(555)

    # Known cubic graphs
    cubic_named = [
        ("K4", *complete_graph(4)),
        ("K3,3", *bipartite_complete(3, 3)),
        ("Petersen", *petersen_graph()),
        ("Prism", *prism_graph()),
    ]

    for name, n, edges in cubic_named:
        vc_size, vc_verts = brute_min_vc(n, edges)
        mmm_size, mmm_sel = brute_min_mmm(n, edges)

        assert mmm_size <= vc_size, f"{name}: mmm > vc"
        checks += 1
        assert vc_size <= 2 * mmm_size, f"{name}: vc > 2*mmm"
        checks += 1

        matching = vc_to_maximal_matching(n, edges, vc_verts)
        assert is_maximal_matching(n, edges, matching), f"{name}: forward not maximal"
        checks += 1
        assert len(matching) <= vc_size, f"{name}: forward too large"
        checks += 1

    # Random cubic graphs
    for n_target in [6, 8, 10]:
        for _ in range(100):
            result = cubic_random(n_target, rng)
            if result is None:
                continue
            n, edges = result

            vc_size, vc_verts = brute_min_vc(n, edges)
            mmm_size, mmm_sel = brute_min_mmm(n, edges)

            assert mmm_size <= vc_size
            checks += 1
            assert vc_size <= 2 * mmm_size
            checks += 1

            matching = vc_to_maximal_matching(n, edges, vc_verts)
            assert is_maximal_matching(n, edges, matching)
            checks += 1
            assert len(matching) <= vc_size
            checks += 1

            vc_back = mmm_to_vc_endpoints(edges, mmm_sel)
            assert is_vertex_cover(n, edges, vc_back)
            checks += 1

    print(f"  Section 5 (cubic graphs): {checks} checks PASSED")
    return checks


# ────────────────────────── Section 6 ─────────────────────────────────

def section6_connected_random() -> int:
    """Section 6: Verify on random connected graphs."""
    checks = 0
    rng = random.Random(6789)
    for _ in range(500):
        n = rng.randint(3, 9)
        extra = rng.randint(0, min(n, 6))
        n_graph, edges = random_connected_graph(n, extra, rng)
        if not edges:
            continue

        vc_size, vc_verts = brute_min_vc(n_graph, edges)
        mmm_size, mmm_sel = brute_min_mmm(n_graph, edges)

        assert mmm_size <= vc_size
        checks += 1
        assert vc_size <= 2 * mmm_size
        checks += 1

        matching = vc_to_maximal_matching(n_graph, edges, vc_verts)
        assert is_maximal_matching(n_graph, edges, matching)
        checks += 1
        assert len(matching) <= vc_size
        checks += 1

        vc_back = mmm_to_vc_endpoints(edges, mmm_sel)
        assert is_vertex_cover(n_graph, edges, vc_back)
        checks += 1

    print(f"  Section 6 (connected random): {checks} checks PASSED")
    return checks


# ────────────────────────── Section 7 ─────────────────────────────────

def section7_all_vc_witnesses() -> int:
    """Section 7: For each optimal VC witness, verify the forward map produces
    a valid maximal matching."""
    checks = 0
    rng = random.Random(777)
    for _ in range(300):
        n = rng.randint(2, 7)
        n_graph, edges = random_graph(n, rng.uniform(0.3, 0.7), rng)
        if not edges:
            continue
        adj = [0] * n_graph
        for u, v in edges:
            adj[u] += 1
            adj[v] += 1
        if any(adj[v] == 0 for v in range(n_graph)):
            continue

        vc_size = brute_min_vc(n_graph, edges)[0]

        # Enumerate all optimal VC witnesses
        vc_count = 0
        for cover in itertools.combinations(range(n_graph), vc_size):
            if is_vertex_cover(n_graph, edges, set(cover)):
                matching = vc_to_maximal_matching(n_graph, edges, list(cover))
                assert is_maximal_matching(n_graph, edges, matching), \
                    f"forward map failed for vc={cover}"
                assert len(matching) <= vc_size
                checks += 1
                vc_count += 1
                if vc_count >= 20:
                    break

    print(f"  Section 7 (all VC witnesses): {checks} checks PASSED")
    return checks


# ────────────────────────── Test vectors ──────────────────────────────

def generate_test_vectors() -> list[dict]:
    """Generate test vectors for JSON export."""
    vectors = []
    rng = random.Random(12345)

    # Named graphs
    named = [
        ("P3", *path_graph(3)),
        ("P4", *path_graph(4)),
        ("C4", *cycle_graph(4)),
        ("C5", *cycle_graph(5)),
        ("K4", *complete_graph(4)),
        ("Petersen", *petersen_graph()),
        ("K2,3", *bipartite_complete(2, 3)),
        ("Prism", *prism_graph()),
        ("S3", *star_graph(3)),
    ]

    for name, n, edges in named:
        if not edges:
            continue
        vc_size, vc_verts = brute_min_vc(n, edges)
        mmm_size, mmm_sel = brute_min_mmm(n, edges)
        matching = vc_to_maximal_matching(n, edges, vc_verts)
        vc_back = mmm_to_vc_endpoints(edges, mmm_sel)

        vectors.append({
            "name": name,
            "n": n,
            "edges": edges,
            "min_vc": vc_size,
            "vc_witness": vc_verts,
            "min_mmm": mmm_size,
            "mmm_witness": sorted(mmm_sel),
            "forward_matching": sorted(matching),
            "forward_matching_size": len(matching),
            "reverse_vc": sorted(vc_back),
            "reverse_vc_size": len(vc_back),
        })

    # Random graphs
    for i in range(20):
        n = rng.randint(3, 8)
        n_graph, edges = random_connected_graph(n, rng.randint(0, 4), rng)
        if not edges:
            continue
        vc_size, vc_verts = brute_min_vc(n_graph, edges)
        mmm_size, mmm_sel = brute_min_mmm(n_graph, edges)
        matching = vc_to_maximal_matching(n_graph, edges, vc_verts)
        vc_back = mmm_to_vc_endpoints(edges, mmm_sel)

        vectors.append({
            "name": f"random_{i}",
            "n": n_graph,
            "edges": edges,
            "min_vc": vc_size,
            "vc_witness": vc_verts,
            "min_mmm": mmm_size,
            "mmm_witness": sorted(mmm_sel),
            "forward_matching": sorted(matching),
            "forward_matching_size": len(matching),
            "reverse_vc": sorted(vc_back),
            "reverse_vc_size": len(vc_back),
        })

    return vectors


# ────────────────────────── main ──────────────────────────────────────

def main() -> None:
    print("Verifying: MinimumVertexCover -> MinimumMaximalMatching")
    print("=" * 60)

    total = 0
    total += section1_named_graphs()
    total += section2_forward_construction()
    total += section3_reverse_extraction()
    total += section4_bounds_inequality()
    total += section5_cubic_graphs()
    total += section6_connected_random()
    total += section7_all_vc_witnesses()

    print("=" * 60)
    print(f"TOTAL: {total} checks PASSED")
    assert total >= 5000, f"Expected >= 5000 checks, got {total}"
    print("ALL CHECKS PASSED >= 5000")

    # Generate test vectors JSON
    vectors = generate_test_vectors()
    out_path = Path(__file__).parent / "test_vectors_minimum_vertex_cover_minimum_maximal_matching.json"
    with open(out_path, "w") as f:
        json.dump(vectors, f, indent=2)
    print(f"\nTest vectors written to {out_path} ({len(vectors)} vectors)")


if __name__ == "__main__":
    main()
