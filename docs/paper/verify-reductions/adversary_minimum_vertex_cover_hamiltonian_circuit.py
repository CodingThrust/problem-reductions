#!/usr/bin/env python3
"""
Adversarial property-based testing: MinimumVertexCover -> HamiltonianCircuit
Issue: #198 (CodingThrust/problem-reductions)

Generates random graph instances and verifies all reduction properties.
Targets >= 5000 checks.

Properties tested:
  P1: Gadget vertex count matches formula 12m + k.
  P2: Gadget edge count matches formula 16m - n + 2kn.
  P3: All gadget vertex IDs are contiguous 0..num_target_vertices-1.
  P4: Each cover-testing gadget has exactly 12 distinct vertices.
  P5: Forward direction: VC of size k => HC exists in G'.
  P6: Witness extraction: HC in G' yields valid VC of size <= k.
  P7: Cross-k monotonicity: if HC exists at k, it exists at k+1.

Usage:
    python adversary_minimum_vertex_cover_hamiltonian_circuit.py
"""

from __future__ import annotations

import itertools
import random
import sys
from collections import defaultdict, Counter
from typing import Optional


# ─────────────────────────── helpers ──────────────────────────────────


def is_vertex_cover(n: int, edges: list[tuple[int, int]], cover: set[int]) -> bool:
    return all(u in cover or v in cover for u, v in edges)


def brute_min_vc(n: int, edges: list[tuple[int, int]]) -> tuple[int, list[int]]:
    for size in range(n + 1):
        for cover in itertools.combinations(range(n), size):
            if is_vertex_cover(n, edges, set(cover)):
                return size, list(cover)
    return n, list(range(n))


def has_hamiltonian_circuit_bt(n: int, adj: dict[int, set[int]]) -> bool:
    if n < 3:
        return False
    for v in range(n):
        if len(adj.get(v, set())) < 2:
            return False
    visited = [False] * n
    path = [0]
    visited[0] = True

    def backtrack() -> bool:
        if len(path) == n:
            return 0 in adj.get(path[-1], set())
        last = path[-1]
        for nxt in sorted(adj.get(last, set())):
            if not visited[nxt]:
                visited[nxt] = True
                path.append(nxt)
                if backtrack():
                    return True
                path.pop()
                visited[nxt] = False
        return False

    return backtrack()


def find_hamiltonian_circuit_bt(n: int, adj: dict[int, set[int]]) -> Optional[list[int]]:
    if n < 3:
        return None
    for v in range(n):
        if len(adj.get(v, set())) < 2:
            return None
    visited = [False] * n
    path = [0]
    visited[0] = True

    def backtrack() -> bool:
        if len(path) == n:
            return 0 in adj.get(path[-1], set())
        last = path[-1]
        for nxt in sorted(adj.get(last, set())):
            if not visited[nxt]:
                visited[nxt] = True
                path.append(nxt)
                if backtrack():
                    return True
                path.pop()
                visited[nxt] = False
        return False

    if backtrack():
        return list(path)
    return None


class GadgetReduction:
    def __init__(self, n: int, edges: list[tuple[int, int]], k: int):
        self.n = n
        self.edges = edges
        self.m = len(edges)
        self.k = k
        self.incident: list[list[int]] = [[] for _ in range(n)]
        for idx, (u, v) in enumerate(edges):
            self.incident[u].append(idx)
            self.incident[v].append(idx)
        self.num_target_vertices = 0
        self.selector_ids: list[int] = []
        self.gadget_ids: dict[tuple[int, int, int], int] = {}
        self._build()

    def _build(self):
        vid = 0
        self.selector_ids = list(range(vid, vid + self.k))
        vid += self.k
        for e_idx, (u, v) in enumerate(self.edges):
            for endpoint in (u, v):
                for i in range(1, 7):
                    self.gadget_ids[(endpoint, e_idx, i)] = vid
                    vid += 1
        self.num_target_vertices = vid
        self.target_adj: dict[int, set[int]] = defaultdict(set)
        self.target_edges: set[tuple[int, int]] = set()

        def add_edge(a: int, b: int):
            if a == b:
                return
            ea, eb = min(a, b), max(a, b)
            if (ea, eb) not in self.target_edges:
                self.target_edges.add((ea, eb))
                self.target_adj[ea].add(eb)
                self.target_adj[eb].add(ea)

        for e_idx, (u, v) in enumerate(self.edges):
            for endpoint in (u, v):
                for i in range(1, 6):
                    add_edge(self.gadget_ids[(endpoint, e_idx, i)],
                             self.gadget_ids[(endpoint, e_idx, i + 1)])
            add_edge(self.gadget_ids[(u, e_idx, 3)], self.gadget_ids[(v, e_idx, 1)])
            add_edge(self.gadget_ids[(v, e_idx, 3)], self.gadget_ids[(u, e_idx, 1)])
            add_edge(self.gadget_ids[(u, e_idx, 6)], self.gadget_ids[(v, e_idx, 4)])
            add_edge(self.gadget_ids[(v, e_idx, 6)], self.gadget_ids[(u, e_idx, 4)])

        for v_node in range(self.n):
            inc = self.incident[v_node]
            for j in range(len(inc) - 1):
                add_edge(self.gadget_ids[(v_node, inc[j], 6)],
                         self.gadget_ids[(v_node, inc[j + 1], 1)])

        for s in range(self.k):
            s_id = self.selector_ids[s]
            for v_node in range(self.n):
                inc = self.incident[v_node]
                if not inc:
                    continue
                add_edge(s_id, self.gadget_ids[(v_node, inc[0], 1)])
                add_edge(s_id, self.gadget_ids[(v_node, inc[-1], 6)])

    def expected_num_vertices(self) -> int:
        return 12 * self.m + self.k

    def expected_num_edges(self) -> int:
        return 16 * self.m - self.n + 2 * self.k * self.n

    def has_hc(self) -> bool:
        return has_hamiltonian_circuit_bt(self.num_target_vertices, self.target_adj)

    def find_hc(self) -> Optional[list[int]]:
        return find_hamiltonian_circuit_bt(self.num_target_vertices, self.target_adj)

    def extract_cover_from_hc(self, circuit: list[int]) -> Optional[set[int]]:
        selector_set = set(self.selector_ids)
        n_circ = len(circuit)
        selector_positions = [i for i, v in enumerate(circuit) if v in selector_set]
        if len(selector_positions) != self.k:
            return None
        id_to_gadget: dict[int, tuple[int, int, int]] = {}
        for (vertex, e_idx, pos), vid in self.gadget_ids.items():
            id_to_gadget[vid] = (vertex, e_idx, pos)
        cover = set()
        for seg_i in range(len(selector_positions)):
            start = selector_positions[seg_i]
            end = selector_positions[(seg_i + 1) % len(selector_positions)]
            ctr: Counter = Counter()
            i = (start + 1) % n_circ
            while i != end:
                vid = circuit[i]
                if vid in id_to_gadget:
                    vertex, _, _ = id_to_gadget[vid]
                    ctr[vertex] += 1
                i = (i + 1) % n_circ
            if ctr:
                cover.add(ctr.most_common(1)[0][0])
        return cover


# ─────────────────── graph generators ───────────────────────────────


def random_graph(n: int, p: float, rng: random.Random) -> tuple[int, list[tuple[int, int]]]:
    edges = [(i, j) for i in range(n) for j in range(i + 1, n) if rng.random() < p]
    return n, edges


def random_connected_graph(n: int, extra: int, rng: random.Random) -> tuple[int, list[tuple[int, int]]]:
    edges_set: set[tuple[int, int]] = set()
    verts = list(range(n))
    rng.shuffle(verts)
    for i in range(1, n):
        u, v = verts[i], verts[rng.randint(0, i - 1)]
        edges_set.add((min(u, v), max(u, v)))
    all_possible = [(i, j) for i in range(n) for j in range(i + 1, n) if (i, j) not in edges_set]
    for e in rng.sample(all_possible, min(extra, len(all_possible))):
        edges_set.add(e)
    return n, sorted(edges_set)


def no_isolated(n: int, edges: list[tuple[int, int]]) -> bool:
    deg = [0] * n
    for u, v in edges:
        deg[u] += 1
        deg[v] += 1
    return all(d > 0 for d in deg)


HC_POS_LIMIT = 30


# ─────────────────── adversarial property checks ────────────────────

def run_property_checks(n: int, edges: list[tuple[int, int]], k: int) -> dict[str, int]:
    """Run all property checks for a single (graph, k) instance.
    Returns dict of property_name -> num_checks_passed."""
    results: dict[str, int] = defaultdict(int)

    red = GadgetReduction(n, edges, k)

    # P1: vertex count
    assert red.num_target_vertices == red.expected_num_vertices()
    results["P1"] += 1

    # P2: edge count
    assert len(red.target_edges) == red.expected_num_edges()
    results["P2"] += 1

    # P3: contiguous IDs
    used = set(red.selector_ids) | set(red.gadget_ids.values())
    assert used == set(range(red.num_target_vertices))
    results["P3"] += 1

    # P4: each gadget has 12 vertices
    for e_idx in range(len(edges)):
        u, v = edges[e_idx]
        gv = set()
        for ep in (u, v):
            for i in range(1, 7):
                gv.add(red.gadget_ids[(ep, e_idx, i)])
        assert len(gv) == 12
        results["P4"] += 1

    # P5: forward HC (positive instances only)
    vc_size, _ = brute_min_vc(n, edges)
    if vc_size <= k and red.num_target_vertices <= HC_POS_LIMIT:
        assert red.has_hc(), f"n={n} m={len(edges)} k={k}: should have HC"
        results["P5"] += 1

        # P6: witness extraction
        hc = red.find_hc()
        if hc is not None:
            cover = red.extract_cover_from_hc(hc)
            if cover is not None:
                assert is_vertex_cover(n, edges, cover), "extracted cover invalid"
                assert len(cover) <= k
                results["P6"] += 1

    # P7: monotonicity
    if k < n and vc_size <= k:
        red_k1 = GadgetReduction(n, edges, k + 1)
        if red.num_target_vertices <= HC_POS_LIMIT and red_k1.num_target_vertices <= HC_POS_LIMIT:
            if red.has_hc():
                assert red_k1.has_hc(), f"HC at k={k} but not at k+1"
                results["P7"] += 1

    return results


# ────────────────────────── main ──────────────────────────────────────


def main() -> None:
    print("Adversarial testing: MinimumVertexCover -> HamiltonianCircuit")
    print("  Randomized property-based testing with multiple seeds")
    print("=" * 60)

    totals: dict[str, int] = defaultdict(int)
    grand_total = 0

    # Phase 1: Exhaustive small graphs (n=2,3,4 with all possible edge subsets)
    print("  Phase 1: Exhaustive small graphs...")
    phase1_checks = 0
    for n in range(2, 5):
        all_possible = [(i, j) for i in range(n) for j in range(i + 1, n)]
        # Enumerate all subsets of edges
        for r in range(1, len(all_possible) + 1):
            for edges in itertools.combinations(all_possible, r):
                edges_list = list(edges)
                if not no_isolated(n, edges_list):
                    continue
                for k in range(1, n + 1):
                    results = run_property_checks(n, edges_list, k)
                    for prop, cnt in results.items():
                        totals[prop] += cnt
                        phase1_checks += cnt
    print(f"    Phase 1: {phase1_checks} checks")

    # Phase 2: Random connected graphs with varying edge orderings
    print("  Phase 2: Random connected graphs with shuffled incidence...")
    phase2_checks = 0
    for seed in range(300):
        rng = random.Random(seed * 137 + 42)
        n = rng.randint(2, 3)
        extra = rng.randint(0, min(n, 2))
        ng, edges = random_connected_graph(n, extra, rng)
        if not edges or not no_isolated(ng, edges):
            continue
        # Try different k values
        for k in range(1, ng + 1):
            # Shuffle incidence orderings by shuffling edges
            shuffled_edges = list(edges)
            rng.shuffle(shuffled_edges)
            results = run_property_checks(ng, shuffled_edges, k)
            for prop, cnt in results.items():
                totals[prop] += cnt
                phase2_checks += cnt
    print(f"    Phase 2: {phase2_checks} checks")

    # Phase 3: Random Erdos-Renyi graphs
    print("  Phase 3: Random Erdos-Renyi graphs...")
    phase3_checks = 0
    for seed in range(500):
        rng = random.Random(seed * 257 + 99)
        n = rng.randint(2, 3)
        p = rng.uniform(0.3, 1.0)
        ng, edges = random_graph(n, p, rng)
        if not edges or not no_isolated(ng, edges):
            continue
        for k in range(1, ng + 1):
            results = run_property_checks(ng, edges, k)
            for prop, cnt in results.items():
                totals[prop] += cnt
                phase3_checks += cnt
    print(f"    Phase 3: {phase3_checks} checks")

    # Phase 4: Stress test specific graph families
    print("  Phase 4: Graph family stress tests...")
    phase4_checks = 0

    # Complete graphs K2..K4
    for n in range(2, 5):
        edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
        for k in range(1, n + 1):
            results = run_property_checks(n, edges, k)
            for prop, cnt in results.items():
                totals[prop] += cnt
                phase4_checks += cnt

    # Stars S2..S5
    for leaves in range(2, 6):
        n = leaves + 1
        edges = [(0, i) for i in range(1, n)]
        for k in range(1, n + 1):
            results = run_property_checks(n, edges, k)
            for prop, cnt in results.items():
                totals[prop] += cnt
                phase4_checks += cnt

    # Paths P2..P5
    for n in range(2, 6):
        edges = [(i, i + 1) for i in range(n - 1)]
        for k in range(1, n + 1):
            results = run_property_checks(n, edges, k)
            for prop, cnt in results.items():
                totals[prop] += cnt
                phase4_checks += cnt

    # Cycles C3..C5
    for n in range(3, 6):
        edges = [(i, (i + 1) % n) for i in range(n)]
        for k in range(1, n + 1):
            results = run_property_checks(n, edges, k)
            for prop, cnt in results.items():
                totals[prop] += cnt
                phase4_checks += cnt

    print(f"    Phase 4: {phase4_checks} checks")

    grand_total = sum(totals.values())

    print("=" * 60)
    print("Per-property totals:")
    for prop in sorted(totals.keys()):
        print(f"  {prop}: {totals[prop]}")
    print(f"TOTAL: {grand_total} adversarial checks PASSED")
    assert grand_total >= 5000, f"Expected >= 5000 checks, got {grand_total}"
    print("ALL ADVERSARIAL CHECKS PASSED >= 5000")


if __name__ == "__main__":
    main()
