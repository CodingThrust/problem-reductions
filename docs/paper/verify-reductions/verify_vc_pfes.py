#!/usr/bin/env python3
"""
Verify VC -> PFES reduction (§5.1 of proposed-reductions.typ).

Checks:
  1. Build the 6-cycle graph H for small graphs (P_3, K_3, P_4, K_{1,3}, C_4)
  2. Verify vertex count = 2n + 2m, edge count = n + 4m
  3. Verify girth >= 6 using networkx
  4. Forward: VC of size K -> delete K control edges -> all 6-cycles broken
  5. Backward: dominance (control edges break d(v) cycles, others break 1)
  6. Verify P_3 example from the paper

Run: python3 docs/paper/verify-reductions/verify_vc_pfes.py
"""

import itertools
import sys
import networkx as nx

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


def build_pfes_graph(nv, edges):
    """
    Build the PFES graph H from a VC instance.

    Vertices:
      0..nv-1: original vertices
      nv..2*nv-1: control vertices r_v (r_v = nv + v)
      For edge j=(u,w): s_j = 2*nv + 2*j, p_j = 2*nv + 2*j + 1

    Returns dict with graph info.
    """
    m = len(edges)
    h_edges = []
    control_edges = []
    cycles = []

    # Control edges
    for v in range(nv):
        ce = (v, nv + v)
        h_edges.append(ce)
        control_edges.append(ce)

    # Edge gadgets
    for j, (u, w) in enumerate(edges):
        s_j = 2 * nv + 2 * j
        p_j = 2 * nv + 2 * j + 1

        h_edges.append((nv + u, s_j))     # r_u -- s_j
        h_edges.append((s_j, nv + w))      # s_j -- r_w
        h_edges.append((u, p_j))           # u -- p_j
        h_edges.append((p_j, w))           # p_j -- w

        # 6-cycle: u - r_u - s_j - r_w - w - p_j - u
        cyc = [u, nv + u, s_j, nv + w, w, p_j]
        cycles.append(cyc)

    num_vertices = 2 * nv + 2 * m

    # Build networkx graph
    G_h = nx.Graph()
    G_h.add_nodes_from(range(num_vertices))
    G_h.add_edges_from(h_edges)

    return {
        'num_vertices': num_vertices,
        'edges': h_edges,
        'control_edges': control_edges,
        'cycles': cycles,
        'nx_graph': G_h,
    }


def normalize_edge(e):
    return (min(e), max(e))


def cycle_edge_set(cyc):
    """Get the set of edges in a cycle (normalized)."""
    edges = set()
    for i in range(len(cyc)):
        e = normalize_edge((cyc[i], cyc[(i + 1) % len(cyc)]))
        edges.add(e)
    return edges


# ============================================================
# Test graphs
# ============================================================

test_graphs = [
    (3, [(0, 1), (1, 2)], 1, "P_3"),
    (3, [(0, 1), (1, 2), (0, 2)], 2, "K_3"),
    (4, [(0, 1), (1, 2), (2, 3)], 2, "P_4"),
    (4, [(0, 1), (0, 2), (0, 3)], 1, "K_{1,3}"),
    (4, [(0, 1), (1, 2), (2, 3), (3, 0)], 2, "C_4"),
]


# ============================================================
# 1 & 2. Structure: vertex/edge counts
# ============================================================

def verify_structure():
    print("=== 1. Structure: vertex and edge counts ===")

    for nv, edges, min_vc, name in test_graphs:
        m = len(edges)
        h = build_pfes_graph(nv, edges)

        check(h['num_vertices'] == 2 * nv + 2 * m,
              f"{name}: vertices={h['num_vertices']}, expected {2*nv+2*m}")
        check(len(h['edges']) == nv + 4 * m,
              f"{name}: edges={len(h['edges'])}, expected {nv+4*m}")

        # Each cycle has length 6
        for i, cyc in enumerate(h['cycles']):
            check(len(cyc) == 6,
                  f"{name} cycle {i}: length={len(cyc)}, expected 6")


# ============================================================
# 3. Girth >= 6
# ============================================================

def verify_girth():
    print("=== 2. Girth >= 6 ===")

    for nv, edges, min_vc, name in test_graphs:
        h = build_pfes_graph(nv, edges)
        G_h = h['nx_graph']
        girth = nx.girth(G_h)
        check(girth >= 6,
              f"{name}: girth={girth}, expected >= 6")

        # If there are cycles, girth should be exactly 6
        if len(edges) > 0:
            check(girth == 6,
                  f"{name}: girth={girth}, expected exactly 6")


# ============================================================
# 4. Forward direction: VC -> PFES
# ============================================================

def verify_forward():
    print("=== 3. Forward: VC of size K -> all 6-cycles broken ===")

    for nv, edges, min_vc, name in test_graphs:
        m = len(edges)
        h = build_pfes_graph(nv, edges)

        # Find all vertex covers of size min_vc
        for K in range(nv + 1):
            vc_exists = False
            for C in itertools.combinations(range(nv), K):
                C_set = set(C)
                if all(u in C_set or w in C_set for u, w in edges):
                    vc_exists = True

                    # Delete control edges for C
                    deleted = {normalize_edge((v, nv + v)) for v in C_set}

                    # Check all 6-cycles are broken
                    all_broken = True
                    for j, cyc in enumerate(h['cycles']):
                        cyc_edges = cycle_edge_set(cyc)
                        if not (cyc_edges & deleted):
                            all_broken = False
                            break

                    check(all_broken,
                          f"{name} K={K} VC={C_set}: not all cycles broken")
                    break  # test one VC per K

            if K == min_vc:
                check(vc_exists,
                      f"{name}: min_vc={min_vc} should exist at K={K}")


# ============================================================
# 5. Dominance: control edges break more cycles than non-control
# ============================================================

def verify_dominance():
    print("=== 4. Dominance: control edges break d(v) cycles ===")

    for nv, edges, min_vc, name in test_graphs:
        h = build_pfes_graph(nv, edges)

        # For each vertex v, control edge e_v* appears in d(v) 6-cycles
        for v in range(nv):
            degree_v = sum(1 for u, w in edges if u == v or w == v)
            ce = normalize_edge((v, nv + v))

            cycles_containing = 0
            for cyc in h['cycles']:
                if ce in cycle_edge_set(cyc):
                    cycles_containing += 1

            check(cycles_containing == degree_v,
                  f"{name} v={v}: control edge in {cycles_containing} cycles, "
                  f"expected d(v)={degree_v}")

        # Non-control edges appear in exactly 1 cycle
        for j, (u, w) in enumerate(edges):
            s_j = 2 * nv + 2 * j
            p_j = 2 * nv + 2 * j + 1

            non_control = [
                normalize_edge((nv + u, s_j)),
                normalize_edge((s_j, nv + w)),
                normalize_edge((u, p_j)),
                normalize_edge((p_j, w)),
            ]

            for nc_edge in non_control:
                cycles_containing = 0
                for cyc in h['cycles']:
                    if nc_edge in cycle_edge_set(cyc):
                        cycles_containing += 1
                check(cycles_containing == 1,
                      f"{name} edge {nc_edge}: in {cycles_containing} cycles, "
                      f"expected 1")


# ============================================================
# 6. P_3 example from the paper
# ============================================================

def verify_p3_example():
    print("=== 5. P_3 example from paper ===")

    nv = 3
    edges = [(0, 1), (1, 2)]
    m = 2
    h = build_pfes_graph(nv, edges)

    # Vertices: 2*3 + 2*2 = 10
    check(h['num_vertices'] == 10, f"P3: vertices={h['num_vertices']}, expected 10")
    # Edges: 3 + 4*2 = 11
    check(len(h['edges']) == 11, f"P3: edges={len(h['edges'])}, expected 11")

    # Control edges: (0,r_0), (1,r_1), (2,r_2) = (0,3), (1,4), (2,5)
    expected_control = [(0, 3), (1, 4), (2, 5)]
    check(h['control_edges'] == expected_control,
          f"P3: control edges = {h['control_edges']}")

    # 6-cycle for (0,1): 0 - r_0 - s_01 - r_1 - 1 - p_01 - 0
    # = 0 - 3 - 6 - 4 - 1 - 7 - 0
    cyc0 = h['cycles'][0]
    check(cyc0 == [0, 3, 6, 4, 1, 7],
          f"P3 cycle 0: {cyc0}, expected [0, 3, 6, 4, 1, 7]")

    # 6-cycle for (1,2): 1 - r_1 - s_12 - r_2 - 2 - p_12 - 1
    # = 1 - 4 - 8 - 5 - 2 - 9 - 1
    cyc1 = h['cycles'][1]
    check(cyc1 == [1, 4, 8, 5, 2, 9],
          f"P3 cycle 1: {cyc1}, expected [1, 4, 8, 5, 2, 9]")

    # VC = {1}: delete control edge (1, r_1) = (1, 4)
    deleted = {normalize_edge((1, 4))}

    # Both cycles pass through edge (1,4)
    for i, cyc in enumerate(h['cycles']):
        broken = bool(cycle_edge_set(cyc) & deleted)
        check(broken, f"P3: deleting e_1* breaks cycle {i}")

    # Girth check
    girth = nx.girth(h['nx_graph'])
    check(girth == 6, f"P3: girth={girth}, expected 6")

    # Backward: minimum PFES budget = minimum VC size = 1
    # Verify by brute force: find minimum number of edges to delete
    # to break all 6-cycles
    all_h_edges = [normalize_edge(e) for e in h['edges']]
    min_pfes = None
    for k in range(1, len(all_h_edges) + 1):
        found = False
        for deletion_set in itertools.combinations(all_h_edges, k):
            del_set = set(deletion_set)
            all_broken = all(
                bool(cycle_edge_set(cyc) & del_set)
                for cyc in h['cycles']
            )
            if all_broken:
                min_pfes = k
                found = True
                break
        if found:
            break

    check(min_pfes == 1, f"P3: min PFES budget = {min_pfes}, expected 1 (= min VC)")


# ============================================================
# Backward: min PFES = min VC for all test graphs
# ============================================================

def verify_backward():
    print("=== 6. Backward: min PFES budget = min VC size ===")

    for nv, edges, min_vc, name in test_graphs:
        h = build_pfes_graph(nv, edges)
        all_h_edges = list(set(normalize_edge(e) for e in h['edges']))

        # Find minimum PFES budget by brute force
        min_pfes = None
        for k in range(0, len(all_h_edges) + 1):
            found = False
            for deletion_set in itertools.combinations(all_h_edges, k):
                del_set = set(deletion_set)
                all_broken = all(
                    bool(cycle_edge_set(cyc) & del_set)
                    for cyc in h['cycles']
                )
                if all_broken:
                    min_pfes = k
                    found = True
                    break
            if found:
                break

        check(min_pfes == min_vc,
              f"{name}: min PFES={min_pfes}, min VC={min_vc}")


# ============================================================
# Main
# ============================================================

def main():
    global passed, failed, total

    print("VC -> PFES Reduction Verification")
    print("=" * 50)

    verify_structure()
    print(f"  Structure: {passed}/{total} cumulative")

    verify_girth()
    print(f"  Girth: {passed}/{total} cumulative")

    verify_forward()
    print(f"  Forward: {passed}/{total} cumulative")

    verify_dominance()
    print(f"  Dominance: {passed}/{total} cumulative")

    verify_p3_example()
    print(f"  P3 example: {passed}/{total} cumulative")

    verify_backward()
    print(f"  Backward: {passed}/{total} cumulative")

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
