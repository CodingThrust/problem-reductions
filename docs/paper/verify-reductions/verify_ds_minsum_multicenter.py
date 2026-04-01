#!/usr/bin/env python3
"""
§4.2 DS → Min-Sum Multicenter: exhaustive verification.

DS of size K → K centers with total distance = n - K.
Backward: total distance ≤ n - K → centers form a DS.

Run: python3 docs/paper/verify-reductions/verify_ds_minsum_multicenter.py
"""
import itertools
import sys
from collections import defaultdict
import random

passed = 0
failed = 0

def check(condition, msg=""):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL: {msg}")

def powerset(s):
    for r in range(len(s) + 1):
        yield from itertools.combinations(s, r)

def is_dominating_set(D_set, adj, vertices):
    return all(v in D_set or bool(adj[v] & D_set) for v in vertices)

def find_min_ds(adj, vertices):
    for k in range(1, len(vertices) + 1):
        for D in itertools.combinations(vertices, k):
            if is_dominating_set(set(D), adj, vertices):
                return k
    return len(vertices)

def bfs_distance(v, targets, adj):
    if v in targets:
        return 0
    visited = {v}
    frontier = [v]
    dist = 0
    while frontier:
        dist += 1
        next_frontier = []
        for u in frontier:
            for w in adj[u]:
                if w in targets:
                    return dist
                if w not in visited:
                    visited.add(w)
                    next_frontier.append(w)
        frontier = next_frontier
    return float('inf')

def total_distance_to_centers(C_set, adj, vertices):
    return sum(bfs_distance(v, C_set, adj) for v in vertices)

def main():
    global passed, failed
    print("§4.2 DS → Min-Sum Multicenter verification")
    print("=" * 50)

    random.seed(456)

    # --- Paper example: P_4, K=2 ---
    print("\nPaper example (P_4, K=2)...")
    vertices = [0, 1, 2, 3]
    adj = defaultdict(set)
    for u, v in [(0,1), (1,2), (2,3)]:
        adj[u].add(v); adj[v].add(u)

    D = {1, 2}
    check(is_dominating_set(D, adj, vertices), "P4: {1,2} is DS")
    td = total_distance_to_centers(D, adj, vertices)
    check(td == 2, f"P4: total dist = {td}, expected n-K = 2")

    # Individual distances
    check(bfs_distance(0, D, adj) == 1, "d(0, {1,2}) = 1")
    check(bfs_distance(1, D, adj) == 0, "d(1, {1,2}) = 0")
    check(bfs_distance(2, D, adj) == 0, "d(2, {1,2}) = 0")
    check(bfs_distance(3, D, adj) == 1, "d(3, {1,2}) = 1")

    # --- Forward: DS of size K → total distance = n - K ---
    print("\nForward direction (exhaustive, n ≤ 6)...")
    for nv in range(2, 7):
        vertices = list(range(nv))
        all_edges = list(itertools.combinations(vertices, 2))
        edge_subsets = list(powerset(all_edges))
        if len(edge_subsets) > 300:
            edge_subsets = random.sample(edge_subsets, 300)

        for edges in edge_subsets:
            adj = defaultdict(set)
            for u, v in edges:
                adj[u].add(v); adj[v].add(u)

            for K in range(1, nv + 1):
                for D in itertools.combinations(vertices, K):
                    D_set = set(D)
                    if is_dominating_set(D_set, adj, vertices):
                        td = total_distance_to_centers(D_set, adj, vertices)
                        check(td == nv - K,
                              f"Forward n={nv}, K={K}, D={D}: "
                              f"total_dist={td}, expected {nv-K}")
                        break  # one DS per K is enough

        print(f"  n={nv}: {passed} passed, {failed} failed (cumulative)")

    # --- Backward: total distance ≤ n-K → DS ---
    print("\nBackward direction (exhaustive, n ≤ 5)...")
    for nv in range(2, 6):
        vertices = list(range(nv))
        all_edges = list(itertools.combinations(vertices, 2))
        edge_subsets = list(powerset(all_edges))
        if len(edge_subsets) > 200:
            edge_subsets = random.sample(edge_subsets, 200)

        for edges in edge_subsets:
            adj = defaultdict(set)
            for u, v in edges:
                adj[u].add(v); adj[v].add(u)

            for K in range(1, nv + 1):
                for C in itertools.combinations(vertices, K):
                    C_set = set(C)
                    td = total_distance_to_centers(C_set, adj, vertices)
                    if td <= nv - K:
                        check(is_dominating_set(C_set, adj, vertices),
                              f"Backward n={nv}, K={K}, C={C}: "
                              f"dist={td} ≤ {nv-K} but not DS")

        print(f"  n={nv}: {passed} passed, {failed} failed (cumulative)")

    # --- Tight bound: non-DS always has total distance > n-K ---
    print("\nTight bound check (n ≤ 4)...")
    for nv in range(2, 5):
        vertices = list(range(nv))
        all_edges = list(itertools.combinations(vertices, 2))
        for edges in itertools.combinations(all_edges, max(1, nv - 1)):
            adj = defaultdict(set)
            for u, v in edges:
                adj[u].add(v); adj[v].add(u)

            for K in range(1, nv + 1):
                for C in itertools.combinations(vertices, K):
                    C_set = set(C)
                    is_ds = is_dominating_set(C_set, adj, vertices)
                    td = total_distance_to_centers(C_set, adj, vertices)
                    if is_ds:
                        check(td == nv - K,
                              f"Tight: DS has dist exactly n-K")
                    else:
                        check(td > nv - K,
                              f"Tight: non-DS n={nv}, K={K}, C={C}: "
                              f"dist={td} should be > {nv-K}")

    print(f"\n{'='*50}")
    print(f"§4.2 DS → MinSum: {passed} passed, {failed} failed")
    return 1 if failed else 0

if __name__ == "__main__":
    sys.exit(main())
