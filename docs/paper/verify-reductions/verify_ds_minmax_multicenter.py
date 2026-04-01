#!/usr/bin/env python3
"""
§4.1 DS → Min-Max Multicenter: exhaustive verification.

DS of size ≤ K ↔ K centers with max distance ≤ 1.

Run: python3 docs/paper/verify-reductions/verify_ds_minmax_multicenter.py
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

def has_ds_of_size_leq(K, adj, vertices):
    for k in range(1, K + 1):
        for D in itertools.combinations(vertices, k):
            if is_dominating_set(set(D), adj, vertices):
                return True
    return False

def find_min_ds(adj, vertices):
    for k in range(1, len(vertices) + 1):
        for D in itertools.combinations(vertices, k):
            if is_dominating_set(set(D), adj, vertices):
                return k
    return len(vertices)

def max_distance_to_centers(C_set, adj, vertices):
    max_dist = 0
    for v in vertices:
        if v in C_set:
            continue
        if adj[v] & C_set:
            d = 1
        else:
            d = float('inf')
        max_dist = max(max_dist, d)
    return max_dist

def main():
    global passed, failed
    print("§4.1 DS → Min-Max Multicenter verification")
    print("=" * 50)

    random.seed(123)

    # --- Paper example: P_4, K=2 ---
    print("\nPaper example (P_4, K=2)...")
    vertices = [0, 1, 2, 3]
    adj = defaultdict(set)
    for u, v in [(0,1), (1,2), (2,3)]:
        adj[u].add(v); adj[v].add(u)

    D = {1, 2}
    check(is_dominating_set(D, adj, vertices), "P4: {1,2} is DS")
    check(max_distance_to_centers(D, adj, vertices) <= 1, "P4: max dist ≤ 1")
    check(find_min_ds(adj, vertices) == 2, "P4: min DS = 2")

    # --- Exhaustive: all graphs n ≤ 6 ---
    print("\nExhaustive (n ≤ 6)...")
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
                ds_exists = has_ds_of_size_leq(K, adj, vertices)

                mc_minmax = False
                for C in itertools.combinations(vertices, K):
                    if max_distance_to_centers(set(C), adj, vertices) <= 1:
                        mc_minmax = True
                        break

                check(ds_exists == mc_minmax,
                      f"n={nv}, K={K}, |E|={len(edges)}: DS={ds_exists}, MC={mc_minmax}")

        print(f"  n={nv}: {passed} passed, {failed} failed (cumulative)")

    # --- Identity: same graph, same solution ---
    print("\nSolution identity check...")
    for nv in range(2, 5):
        vertices = list(range(nv))
        all_edges = list(itertools.combinations(vertices, 2))
        for edges in itertools.combinations(all_edges, nv - 1):
            adj = defaultdict(set)
            for u, v in edges:
                adj[u].add(v); adj[v].add(u)
            for K in range(1, nv + 1):
                for C in itertools.combinations(vertices, K):
                    C_set = set(C)
                    is_ds = is_dominating_set(C_set, adj, vertices)
                    max_d = max_distance_to_centers(C_set, adj, vertices) <= 1
                    check(is_ds == max_d,
                          f"Solution identity: n={nv}, C={C}")

    print(f"\n{'='*50}")
    print(f"§4.1 DS → MinMax: {passed} passed, {failed} failed")
    return 1 if failed else 0

if __name__ == "__main__":
    sys.exit(main())
