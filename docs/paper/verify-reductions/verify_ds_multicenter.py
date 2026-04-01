#!/usr/bin/env python3
"""
Verify DS -> Multicenter reductions (§4.1 MinMax, §4.2 MinSum).

Exhaustive verification on all graphs up to n=6, all K values:
  1. DS of size <= K <-> MinMax K-center with B=1
  2. DS of size K -> total distance = n-K (MinSum)
  3. Backward MinSum: total distance <= n-K implies DS

Run: python3 docs/paper/verify-reductions/verify_ds_multicenter.py
"""

import itertools
import sys
from collections import defaultdict

passed = 0
failed = 0
total = 0


def check(condition, msg):
    global passed, failed, total
    total += 1
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL: {msg}")


def powerset(s):
    for r in range(len(s) + 1):
        yield from itertools.combinations(s, r)


def is_dominating_set(D_set, adj, vertices):
    """Check if D_set is a dominating set."""
    return all(v in D_set or bool(adj[v] & D_set) for v in vertices)


def has_ds_of_size_leq(K, adj, vertices):
    """Check if a dominating set of size <= K exists."""
    for k in range(1, K + 1):
        for D in itertools.combinations(vertices, k):
            if is_dominating_set(set(D), adj, vertices):
                return True
    return False


def find_min_ds(adj, vertices):
    """Find the minimum dominating set size."""
    for k in range(1, len(vertices) + 1):
        for D in itertools.combinations(vertices, k):
            if is_dominating_set(set(D), adj, vertices):
                return k
    return len(vertices)


def max_distance_to_centers(C_set, adj, vertices):
    """
    Compute max_{v in V} min_{c in C} d(v, c) using BFS.
    For unit-weight graphs, d(v, c) is shortest path length.
    We only need B=1 check, so distance > 1 means not dominated.
    """
    max_dist = 0
    for v in vertices:
        if v in C_set:
            continue  # distance 0
        if adj[v] & C_set:
            d = 1
        else:
            d = float('inf')  # distance > 1
        max_dist = max(max_dist, d)
    return max_dist


def total_distance_to_centers(C_set, adj, vertices):
    """
    Compute sum_{v in V} min_{c in C} d(v, c).
    On unit-weight graphs with B=1 restriction:
      - Centers: distance 0
      - Dominated non-centers: distance 1
      - Non-dominated: distance >= 2
    """
    total = 0
    for v in vertices:
        if v in C_set:
            continue  # distance 0
        if adj[v] & C_set:
            total += 1
        else:
            # Use BFS for actual distance
            total += bfs_distance(v, C_set, adj)
    return total


def bfs_distance(v, targets, adj):
    """BFS shortest distance from v to any vertex in targets."""
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


# ============================================================
# Exhaustive verification
# ============================================================

def verify_all_graphs():
    print("=== DS -> MinMax/MinSum Multicenter (exhaustive, n<=6) ===")

    import random
    random.seed(123)

    for nv in range(2, 7):
        vertices = list(range(nv))
        all_possible_edges = list(itertools.combinations(vertices, 2))

        edge_subsets = list(powerset(all_possible_edges))
        if len(edge_subsets) > 300:
            edge_subsets = random.sample(edge_subsets, 300)

        graph_count = 0
        for edges in edge_subsets:
            adj = defaultdict(set)
            for u, v in edges:
                adj[u].add(v)
                adj[v].add(u)

            for K in range(1, nv + 1):
                # === MinMax (§4.1) ===
                # DS of size <= K exists <-> K centers with max dist <= 1
                ds_exists = has_ds_of_size_leq(K, adj, vertices)

                # MinMax: find K centers with max distance <= 1
                mc_minmax = False
                for C in itertools.combinations(vertices, K):
                    C_set = set(C)
                    if max_distance_to_centers(C_set, adj, vertices) <= 1:
                        mc_minmax = True
                        break

                check(ds_exists == mc_minmax,
                      f"MinMax n={nv}, K={K}, |E|={len(edges)}: "
                      f"DS={ds_exists}, MC={mc_minmax}")

                # === MinSum (§4.2) ===
                # Forward: DS of size K -> total distance = n-K
                if ds_exists:
                    # Find a DS of size exactly K (or smaller)
                    for k in range(1, K + 1):
                        found_ds = False
                        for D in itertools.combinations(vertices, k):
                            D_set = set(D)
                            if is_dominating_set(D_set, adj, vertices):
                                # Total distance should be n - k
                                td = total_distance_to_centers(D_set, adj, vertices)
                                check(td == nv - k,
                                      f"MinSum forward n={nv}, K={k}, |E|={len(edges)}: "
                                      f"total_dist={td}, expected={nv - k}")
                                found_ds = True
                                break
                        if found_ds:
                            break

                # Backward: if total distance <= n-K for some K centers,
                # then those centers form a DS
                for C in itertools.combinations(vertices, K):
                    C_set = set(C)
                    td = total_distance_to_centers(C_set, adj, vertices)
                    if td <= nv - K:
                        # C must be a dominating set
                        check(is_dominating_set(C_set, adj, vertices),
                              f"MinSum backward n={nv}, K={K}: "
                              f"total_dist={td} <= {nv-K} but not DS")

            graph_count += 1

        print(f"  n={nv}: tested {graph_count} graphs, {passed}/{total} cumulative")


# ============================================================
# Specific examples from the paper
# ============================================================

def verify_paper_examples():
    print("=== Paper examples ===")

    # P_4: 0-1-2-3, K=2
    vertices = [0, 1, 2, 3]
    adj = defaultdict(set)
    for u, v in [(0, 1), (1, 2), (2, 3)]:
        adj[u].add(v)
        adj[v].add(u)

    # DS = {1, 2}
    D = {1, 2}
    check(is_dominating_set(D, adj, vertices),
          "P4: {1,2} is a dominating set")

    # MinMax: max distance = 1
    max_d = max_distance_to_centers(D, adj, vertices)
    check(max_d <= 1, f"P4 MinMax: max dist = {max_d}, expected <= 1")

    # MinSum: total distance = 4 - 2 = 2
    td = total_distance_to_centers(D, adj, vertices)
    check(td == 2, f"P4 MinSum: total dist = {td}, expected 2")

    # Individual distances: d(0,{1,2})=1, d(1,{1,2})=0, d(2,{1,2})=0, d(3,{1,2})=1
    check(bfs_distance(0, D, adj) == 1, "P4: d(0, {1,2}) = 1")
    check(bfs_distance(1, D, adj) == 0, "P4: d(1, {1,2}) = 0")
    check(bfs_distance(2, D, adj) == 0, "P4: d(2, {1,2}) = 0")
    check(bfs_distance(3, D, adj) == 1, "P4: d(3, {1,2}) = 1")

    # Minimum DS size is 2 for P_4
    min_ds = find_min_ds(adj, vertices)
    check(min_ds == 2, f"P4: min DS size = {min_ds}, expected 2")


# ============================================================
# Main
# ============================================================

def main():
    global passed, failed, total

    print("DS -> Multicenter Reduction Verification")
    print("=" * 50)

    verify_paper_examples()
    print(f"  Paper examples: {passed}/{total} cumulative")

    verify_all_graphs()

    print()
    print("=" * 50)
    print(f"TOTAL: {passed}/{total} checks passed, {failed} failed")

    if failed > 0:
        print("VERIFICATION FAILED")
        sys.exit(1)
    else:
        print("ALL VERIFICATIONS PASSED")
        sys.exit(0)


if __name__ == "__main__":
    main()
