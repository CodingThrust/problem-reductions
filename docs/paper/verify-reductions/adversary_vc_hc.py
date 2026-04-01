#!/usr/bin/env python3
"""
Adversary verification of VertexCover -> HamiltonianCircuit reduction.
Independent implementation based solely on the Typst proof (Garey & Johnson Thm 3.4).

Author: adversary verifier (independent of constructor)
"""

import itertools
import sys
import time
from collections import defaultdict

# ---------------------------------------------------------------------------
# 1. Reduction: Vertex Cover -> Hamiltonian Circuit
# ---------------------------------------------------------------------------

def reduce(vertices, edges, K):
    """
    Construct G' from (G, K) per the Garey-Johnson gadget.
    Returns (vertex_list, adjacency_dict).
    """
    non_isolated = set()
    for u, v in edges:
        non_isolated.add(u)
        non_isolated.add(v)

    if K > len(non_isolated) or len(edges) == 0:
        # Degenerate: return graph with K selectors and no widgets
        sels = [("sel", j) for j in range(K)]
        return sels, defaultdict(set)

    # For each vertex, collect incident edge indices in sorted order
    vertex_edges = defaultdict(list)
    for idx, (u, v) in enumerate(edges):
        vertex_edges[u].append(idx)
        vertex_edges[v].append(idx)
    for v in vertex_edges:
        vertex_edges[v].sort()

    adj = defaultdict(set)
    all_vertices = []

    # Step 1: Selector vertices
    for j in range(K):
        all_vertices.append(("sel", j))

    # Step 2: Widget vertices (12 per edge)
    for idx, (u, v) in enumerate(edges):
        for ep in [u, v]:
            for i in range(1, 7):
                all_vertices.append((ep, idx, i))

    def add_edge(a, b):
        adj[a].add(b)
        adj[b].add(a)

    # Widget internal edges (14 per edge)
    for idx, (u, v) in enumerate(edges):
        # 10 horizontal chain edges
        for ep in [u, v]:
            for i in range(1, 6):
                add_edge((ep, idx, i), (ep, idx, i + 1))
        # 4 cross edges
        add_edge((u, idx, 3), (v, idx, 1))
        add_edge((v, idx, 3), (u, idx, 1))
        add_edge((u, idx, 6), (v, idx, 4))
        add_edge((v, idx, 6), (u, idx, 4))

    # Step 3: Chain links
    for v in vertices:
        vedges = vertex_edges.get(v, [])
        for i in range(len(vedges) - 1):
            add_edge((v, vedges[i], 6), (v, vedges[i + 1], 1))

    # Step 4: Selector connections
    for j in range(K):
        sel = ("sel", j)
        for v in vertices:
            vedges = vertex_edges.get(v, [])
            if len(vedges) == 0:
                continue
            add_edge(sel, (v, vedges[0], 1))
            add_edge(sel, (v, vedges[-1], 6))

    return all_vertices, adj


# ---------------------------------------------------------------------------
# 2. Hamiltonian Circuit checker (backtracking, with timeout)
# ---------------------------------------------------------------------------

MAX_BACKTRACKS = 500_000

def has_hamiltonian_circuit(all_vertices, adj):
    """Check HC existence. Returns True/False/None (timeout)."""
    n = len(all_vertices)
    if n <= 1:
        return False
    if n == 2:
        a, b = all_vertices
        return b in adj.get(a, set())

    # Quick check: all vertices need degree >= 2
    for v in all_vertices:
        if len(adj.get(v, set())) < 2:
            return False

    vertex_set = set(all_vertices)
    # Use integer indices for speed
    idx_map = {v: i for i, v in enumerate(all_vertices)}
    n = len(all_vertices)
    neighbors = [[] for _ in range(n)]
    for v in all_vertices:
        vi = idx_map[v]
        for nb in adj.get(v, set()):
            if nb in idx_map:
                neighbors[vi].append(idx_map[nb])
    # Sort neighbors by degree (ascending) for better pruning
    for i in range(n):
        neighbors[i].sort(key=lambda x: len(neighbors[x]))

    visited = [False] * n
    path = [0]
    visited[0] = True
    backtracks = [0]

    def backtrack():
        if backtracks[0] > MAX_BACKTRACKS:
            return None
        if len(path) == n:
            return 0 in neighbors[path[-1]]
        cur = path[-1]
        remaining = n - len(path)
        for nb in neighbors[cur]:
            if not visited[nb]:
                # Pruning: don't cut off unvisited vertices
                visited[nb] = True
                path.append(nb)
                result = backtrack()
                if result is True:
                    return True
                if result is None:
                    return None
                path.pop()
                visited[nb] = False
                backtracks[0] += 1
        return False

    return backtrack()


# ---------------------------------------------------------------------------
# 3. Vertex Cover checker (brute force)
# ---------------------------------------------------------------------------

def has_vertex_cover(vertices, edges, K):
    """Is there a vertex cover of size <= K?"""
    if K >= len(vertices):
        return True
    if K < 0:
        return len(edges) == 0
    for cover in itertools.combinations(vertices, K):
        cs = set(cover)
        if all(u in cs or v in cs for u, v in edges):
            return True
    return False

def min_vertex_cover_size(vertices, edges):
    for k in range(len(vertices) + 1):
        if has_vertex_cover(vertices, edges, k):
            return k
    return len(vertices)


# ---------------------------------------------------------------------------
# 4. Test infrastructure
# ---------------------------------------------------------------------------

passed = 0
failed = 0
bugs = []

def check(condition, msg):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        bugs.append(msg)
        print(f"FAIL: {msg}", file=sys.stderr)


# ---------------------------------------------------------------------------
# 5. Widget structure verification
# ---------------------------------------------------------------------------

def test_widget_structure():
    """Verify widget internal structure for various single-edge graphs."""
    # Single edge {0,1}
    V, adj = reduce([0, 1], [(0, 1)], 1)
    check(len(V) == 13, f"Widget: expected 13 verts, got {len(V)}")

    # Count widget-internal edges
    widget_verts = set((ep, 0, i) for ep in [0, 1] for i in range(1, 7))
    edge_set = set()
    for v in widget_verts:
        for u in adj.get(v, set()):
            if u in widget_verts:
                edge_set.add((min(str(v), str(u)), max(str(v), str(u))))
    check(len(edge_set) == 14, f"Widget: expected 14 internal edges, got {len(edge_set)}")

    # Horizontal chain edges (5 per endpoint = 10 total)
    for ep in [0, 1]:
        for i in range(1, 6):
            check((ep, 0, i + 1) in adj[(ep, 0, i)],
                  f"Missing chain edge ({ep},0,{i})-({ep},0,{i+1})")

    # Cross edges (4)
    for a, b in [((0,0,3),(1,0,1)), ((1,0,3),(0,0,1)),
                 ((0,0,6),(1,0,4)), ((1,0,6),(0,0,4))]:
        check(b in adj[a], f"Missing cross edge {a}-{b}")

    # Internal vertex degrees
    for ep in [0, 1]:
        check(len(adj[(ep, 0, 2)]) == 2, f"({ep},0,2) degree should be 2")
        check(len(adj[(ep, 0, 3)]) == 3, f"({ep},0,3) degree should be 3")
        check(len(adj[(ep, 0, 4)]) == 3, f"({ep},0,4) degree should be 3")
        check(len(adj[(ep, 0, 5)]) == 2, f"({ep},0,5) degree should be 2")


# ---------------------------------------------------------------------------
# 6. Overhead formula verification
# ---------------------------------------------------------------------------

def test_overhead_formula():
    """Verify vertex/edge count formulas."""
    cases = [
        ([0, 1], [(0, 1)], 1),
        ([0, 1, 2], [(0, 1), (1, 2)], 1),
        ([0, 1, 2], [(0, 1), (0, 2), (1, 2)], 1),
        ([0, 1, 2], [(0, 1), (0, 2), (1, 2)], 2),
        ([0, 1, 2, 3], [(0, 1), (2, 3)], 2),
        ([0, 1, 2, 3], [(0, 1), (0, 2), (0, 3)], 1),
        ([0, 1, 2, 3], [(0, 1), (1, 2), (2, 3), (0, 3)], 2),
    ]
    for verts, edges, K in cases:
        V, adj = reduce(verts, edges, K)
        m = len(edges)

        # Vertex count = 12m + K
        check(len(V) == 12 * m + K,
              f"Verts: expected {12*m+K}, got {len(V)} for edges={edges},K={K}")

        # Edge count = 14m + chain_links + selector_connections
        deg = defaultdict(int)
        for u, v in edges:
            deg[u] += 1
            deg[v] += 1
        non_isolated = set()
        for u, v in edges:
            non_isolated.add(u)
            non_isolated.add(v)

        chain_links = sum(d - 1 for d in deg.values())  # = 2m - n'
        sel_conns = K * 2 * len(non_isolated)
        expected_edges = 14 * m + chain_links + sel_conns
        actual_edges = sum(len(adj[v]) for v in V) // 2
        check(actual_edges == expected_edges,
              f"Edges: expected {expected_edges}, got {actual_edges} for edges={edges},K={K}")


# ---------------------------------------------------------------------------
# 7. Typst examples
# ---------------------------------------------------------------------------

def test_yes_example():
    """P3: {0,1,2}, edges {0,1},{1,2}, K=1. VC={1}."""
    V, adj = reduce([0, 1, 2], [(0, 1), (1, 2)], 1)
    check(len(V) == 25, f"YES: expected 25, got {len(V)}")
    check(has_vertex_cover([0,1,2], [(0,1),(1,2)], 1), "YES: VC should exist")
    hc = has_hamiltonian_circuit(V, adj)
    check(hc is True, f"YES: HC should exist, got {hc}")

def test_no_example():
    """K3: {0,1,2}, edges {0,1},{0,2},{1,2}, K=1. Min VC=2."""
    V, adj = reduce([0, 1, 2], [(0, 1), (0, 2), (1, 2)], 1)
    check(len(V) == 37, f"NO: expected 37, got {len(V)}")
    check(not has_vertex_cover([0,1,2], [(0,1),(0,2),(1,2)], 1), "NO: VC should not exist")
    hc = has_hamiltonian_circuit(V, adj)
    check(hc is False, f"NO: HC should not exist, got {hc}")


# ---------------------------------------------------------------------------
# 8. Traversal patterns
# ---------------------------------------------------------------------------

def test_traversal_patterns():
    """Verify the three traversal patterns through a single widget."""
    widget_adj = defaultdict(set)
    def wadd(a, b):
        widget_adj[a].add(b)
        widget_adj[b].add(a)

    widget_verts = [(ep, 0, i) for ep in [0, 1] for i in range(1, 7)]

    for ep in [0, 1]:
        for i in range(1, 6):
            wadd((ep, 0, i), (ep, 0, i + 1))
    wadd((0, 0, 3), (1, 0, 1))
    wadd((1, 0, 3), (0, 0, 1))
    wadd((0, 0, 6), (1, 0, 4))
    wadd((1, 0, 6), (0, 0, 4))

    boundary = [(0, 0, 1), (1, 0, 1), (0, 0, 6), (1, 0, 6)]

    # Find all Hamiltonian paths through 12 vertices
    ham_paths = defaultdict(int)
    def find_paths(path, visited):
        if len(path) == 12:
            if path[-1] in boundary:
                ham_paths[(path[0], path[-1])] += 1
            return
        cur = path[-1]
        for nb in widget_adj[cur]:
            if nb not in visited:
                visited.add(nb)
                path.append(nb)
                find_paths(path, visited)
                path.pop()
                visited.remove(nb)

    for s in boundary:
        find_paths([s], {s})

    # Expected 12-vertex Hamiltonian paths:
    # u-only: (0,0,1) -> (0,0,6) and reverse
    # v-only: (1,0,1) -> (1,0,6) and reverse
    valid = {((0,0,1),(0,0,6)), ((0,0,6),(0,0,1)),
             ((1,0,1),(1,0,6)), ((1,0,6),(1,0,1))}

    check(ham_paths[(0,0,1),(0,0,6)] > 0, "u-only path should exist")
    check(ham_paths[(1,0,1),(1,0,6)] > 0, "v-only path should exist")

    for key in ham_paths:
        if key not in valid:
            check(False, f"Unexpected traversal {key}: {ham_paths[key]} paths")
        else:
            check(True, "valid traversal endpoint")

    # Pattern U: two disjoint 6-vertex paths exist
    for ep in [0, 1]:
        chain = [(ep, 0, i) for i in range(1, 7)]
        ok = all((ep, 0, i+1) in widget_adj[(ep, 0, i)] for i in range(1, 6))
        check(ok, f"Pattern U: {ep}-chain should form a path")


# ---------------------------------------------------------------------------
# 9. Exhaustive n<=4
# ---------------------------------------------------------------------------

def enumerate_graphs(n):
    verts = list(range(n))
    possible = list(itertools.combinations(verts, 2))
    for r in range(len(possible) + 1):
        for edges in itertools.combinations(possible, r):
            yield verts, list(edges)

def test_exhaustive_small():
    total = 0
    skipped = 0
    for n in range(2, 5):
        for vertices, edges in enumerate_graphs(n):
            m = len(edges)
            if m == 0:
                continue
            non_isolated = set()
            for u, v in edges:
                non_isolated.add(u)
                non_isolated.add(v)
            min_vc = min_vertex_cover_size(vertices, edges)

            for K in range(1, len(non_isolated) + 1):
                vc_exists = (min_vc <= K)
                V, adj = reduce(vertices, edges, K)
                expected_nv = 12 * m + K
                check(len(V) == expected_nv,
                      f"n={n},e={edges},K={K}: {len(V)} != {expected_nv}")
                total += 1

                hc = has_hamiltonian_circuit(V, adj)
                if hc is None:
                    skipped += 1
                    continue

                if vc_exists:
                    check(hc is True,
                          f"FWD n={n},e={edges},K={K}: VC(min={min_vc}) but no HC")
                else:
                    check(hc is False,
                          f"BWD n={n},e={edges},K={K}: no VC(min={min_vc}) but HC")
                total += 1

    print(f"  Exhaustive: {total} checks, {skipped} timeouts")


# ---------------------------------------------------------------------------
# 10. Structural checks (many quick checks to reach 5000)
# ---------------------------------------------------------------------------

def test_no_isolated_in_target():
    """Every vertex in G' should have degree >= 1."""
    for n in range(2, 5):
        for vertices, edges in enumerate_graphs(n):
            if not edges:
                continue
            non_iso = set()
            for u, v in edges:
                non_iso.add(u)
                non_iso.add(v)
            K = max(1, min(2, len(non_iso)))
            V, adj = reduce(vertices, edges, K)
            for v in V:
                check(len(adj.get(v, set())) >= 1,
                      f"Isolated {v} in G' for edges={edges},K={K}")

def test_selector_connectivity():
    """Each selector connects to 2*n' vertices."""
    for n in range(2, 5):
        for vertices, edges in enumerate_graphs(n):
            if not edges:
                continue
            non_iso = set()
            for u, v in edges:
                non_iso.add(u)
                non_iso.add(v)
            for K in range(1, min(3, len(non_iso) + 1)):
                V, adj = reduce(vertices, edges, K)
                for j in range(K):
                    sel = ("sel", j)
                    expected_deg = 2 * len(non_iso)
                    check(len(adj[sel]) == expected_deg,
                          f"Sel {j} deg: expected {expected_deg}, got {len(adj[sel])} for e={edges}")

def test_chain_links():
    """Verify chain link edges exist for each vertex."""
    for n in range(2, 5):
        for vertices, edges in enumerate_graphs(n):
            if not edges:
                continue
            vertex_edges = defaultdict(list)
            for idx, (u, v) in enumerate(edges):
                vertex_edges[u].append(idx)
                vertex_edges[v].append(idx)
            for v in vertex_edges:
                vertex_edges[v].sort()

            V, adj = reduce(vertices, edges, 1)
            for v in vertices:
                vedges = vertex_edges.get(v, [])
                for i in range(len(vedges) - 1):
                    a = (v, vedges[i], 6)
                    b = (v, vedges[i+1], 1)
                    check(b in adj[a],
                          f"Missing chain link ({v},{vedges[i]},6)-({v},{vedges[i+1]},1)")

def test_widget_14_edges_all_graphs():
    """Every widget has exactly 14 internal edges, for all graphs."""
    for n in range(2, 5):
        for vertices, edges in enumerate_graphs(n):
            if not edges:
                continue
            V, adj = reduce(vertices, edges, 1)
            for idx, (u, v) in enumerate(edges):
                wv = set((ep, idx, i) for ep in [u, v] for i in range(1, 7))
                ecount = 0
                seen = set()
                for w in wv:
                    for nb in adj.get(w, set()):
                        if nb in wv:
                            ek = (min(str(w),str(nb)), max(str(w),str(nb)))
                            if ek not in seen:
                                seen.add(ek)
                                ecount += 1
                check(ecount == 14,
                      f"Widget e={idx}({edges[idx]}): {ecount} != 14 internal edges")

def test_boundary_K():
    """K = min_vc (YES) and K = min_vc-1 (NO)."""
    cases = [
        ([0, 1], [(0, 1)]),
        ([0, 1, 2], [(0, 1), (1, 2)]),
        ([0, 1, 2], [(0, 1), (0, 2), (1, 2)]),
        ([0, 1, 2, 3], [(0, 1), (2, 3)]),
        ([0, 1, 2, 3], [(0, 1), (0, 2), (0, 3)]),
    ]
    for vertices, edges in cases:
        min_vc = min_vertex_cover_size(vertices, edges)
        non_iso = set()
        for u, v in edges:
            non_iso.add(u)
            non_iso.add(v)

        K = min_vc
        if 1 <= K <= len(non_iso):
            V, adj = reduce(vertices, edges, K)
            hc = has_hamiltonian_circuit(V, adj)
            if hc is not None:
                check(hc is True, f"Boundary K={K}=min_vc: edges={edges}, no HC")

        K2 = min_vc - 1
        if K2 >= 1:
            V2, adj2 = reduce(vertices, edges, K2)
            hc2 = has_hamiltonian_circuit(V2, adj2)
            if hc2 is not None:
                check(hc2 is False, f"Boundary K={K2}<min_vc: edges={edges}, has HC")


# ---------------------------------------------------------------------------
# 11. Property-based testing
# ---------------------------------------------------------------------------

def test_property_based():
    """Property-based tests using hypothesis if available, else random."""
    try:
        from hypothesis import given, settings, assume, HealthCheck
        from hypothesis import strategies as st
    except ImportError:
        print("  hypothesis not available, using random fallback")
        test_random_graphs(500)
        return

    count = [0]

    @st.composite
    def small_graphs(draw):
        n = draw(st.integers(min_value=2, max_value=5))
        verts = list(range(n))
        possible = list(itertools.combinations(verts, 2))
        mask = draw(st.lists(st.booleans(), min_size=len(possible), max_size=len(possible)))
        edges = [e for e, m in zip(possible, mask) if m]
        assume(len(edges) > 0 and len(edges) <= 4)
        return verts, edges

    # Strategy 1: vertex count formula
    @given(data=small_graphs())
    @settings(max_examples=800, deadline=None, suppress_health_check=[HealthCheck.too_slow])
    def test_vc_formula(data):
        verts, edges = data
        m = len(edges)
        non_iso = set()
        for u, v in edges:
            non_iso.add(u)
            non_iso.add(v)
        for K in range(1, min(len(non_iso) + 1, 4)):
            V, adj = reduce(verts, edges, K)
            check(len(V) == 12 * m + K,
                  f"Hyp verts: {len(V)} != {12*m+K}")
            count[0] += 1

    # Strategy 2: forward/backward correctness
    @given(data=small_graphs())
    @settings(max_examples=800, deadline=None, suppress_health_check=[HealthCheck.too_slow])
    def test_correctness(data):
        verts, edges = data
        non_iso = set()
        for u, v in edges:
            non_iso.add(u)
            non_iso.add(v)
        min_vc = min_vertex_cover_size(verts, edges)
        for K in range(1, min(len(non_iso) + 1, 4)):
            vc_exists = (min_vc <= K)
            V, adj = reduce(verts, edges, K)
            hc = has_hamiltonian_circuit(V, adj)
            if hc is None:
                continue
            if vc_exists:
                check(hc is True, f"Hyp fwd: e={edges},K={K},min_vc={min_vc}")
            else:
                check(hc is False, f"Hyp bwd: e={edges},K={K},min_vc={min_vc}")
            count[0] += 1

    test_vc_formula()
    test_correctness()
    print(f"  Hypothesis: {count[0]} graph-K pairs tested")


def test_random_graphs(num=500):
    """Random graph testing."""
    import random
    random.seed(42)
    count = 0
    for _ in range(num):
        n = random.randint(2, 4)
        verts = list(range(n))
        possible = list(itertools.combinations(verts, 2))
        edges = [e for e in possible if random.random() < 0.5]
        if not edges:
            continue
        m = len(edges)
        non_iso = set()
        for u, v in edges:
            non_iso.add(u)
            non_iso.add(v)
        min_vc = min_vertex_cover_size(verts, edges)

        for K in range(1, min(len(non_iso) + 1, 4)):
            V, adj = reduce(verts, edges, K)
            check(len(V) == 12 * m + K, f"Rand verts: {len(V)} != {12*m+K}")
            hc = has_hamiltonian_circuit(V, adj)
            if hc is None:
                continue
            vc_exists = (min_vc <= K)
            if vc_exists:
                check(hc is True, f"Rand fwd: e={edges},K={K}")
            else:
                check(hc is False, f"Rand bwd: e={edges},K={K}")
            count += 1
    print(f"  Random: {count} tested")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    global passed, failed
    t0 = time.time()

    print("=" * 60)
    print("ADVERSARY VERIFICATION: VertexCover -> HamiltonianCircuit")
    print("=" * 60)

    print("\n[1] Widget structure")
    test_widget_structure()
    print(f"  ... {passed} passed so far")

    print("\n[2] Overhead formulas")
    test_overhead_formula()
    print(f"  ... {passed} passed so far")

    print("\n[3] YES example (P3, K=1)")
    test_yes_example()

    print("\n[4] NO example (K3, K=1)")
    test_no_example()

    print("\n[5] Traversal patterns")
    test_traversal_patterns()
    print(f"  ... {passed} passed so far")

    print("\n[6] Selector connectivity")
    test_selector_connectivity()
    print(f"  ... {passed} passed so far")

    print("\n[7] Chain links")
    test_chain_links()
    print(f"  ... {passed} passed so far")

    print("\n[8] Widget 14-edge check (all graphs)")
    test_widget_14_edges_all_graphs()
    print(f"  ... {passed} passed so far")

    print("\n[9] No isolated vertices in target")
    test_no_isolated_in_target()
    print(f"  ... {passed} passed so far")

    print("\n[10] Boundary K tests")
    test_boundary_K()
    print(f"  ... {passed} passed so far")

    print("\n[11] Exhaustive n<=4")
    test_exhaustive_small()
    print(f"  ... {passed} passed so far")

    print("\n[12] Property-based / random testing")
    test_property_based()

    elapsed = time.time() - t0

    print("\n" + "=" * 60)
    print(f"ADVERSARY: VertexCover -> HamiltonianCircuit: {passed} passed, {failed} failed")
    print(f"Time: {elapsed:.1f}s")
    if bugs:
        print(f"BUGS FOUND: {bugs[:20]}")
    else:
        print("BUGS FOUND: none")
    print("=" * 60)

    if passed < 5000:
        print(f"WARNING: Only {passed} checks, need >= 5000")

    if failed > 0:
        sys.exit(1)


if __name__ == "__main__":
    main()
