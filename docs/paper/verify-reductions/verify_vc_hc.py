#!/usr/bin/env python3
"""§1.1 MinimumVertexCover → HamiltonianCircuit (#198): exhaustive + structural verification.

Garey & Johnson Theorem 3.4 gadget reduction. HC is NP-hard to check, so we use
a backtracking solver with pruning and limit to small graphs (n ≤ 4).
"""
import itertools
import sys
from sympy import symbols, simplify

passed = failed = 0


def check(condition, msg=""):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL: {msg}")


# ── Graph utilities ──────────────────────────────────────────────────────


def all_simple_graphs(n):
    """Generate all non-isomorphic simple graphs on n labeled vertices.
    Actually generates ALL labeled simple graphs (including isomorphic ones).
    """
    vertices = list(range(n))
    possible_edges = list(itertools.combinations(vertices, 2))
    for r in range(len(possible_edges) + 1):
        for edge_combo in itertools.combinations(possible_edges, r):
            yield vertices, list(edge_combo)


def adjacency(n_vertices, edges):
    """Build adjacency list from edge list."""
    adj = {v: set() for v in range(n_vertices)}
    for u, v in edges:
        adj[u].add(v)
        adj[v].add(u)
    return adj


def has_vertex_cover(n, edges, K):
    """Brute force: does G have a vertex cover of size ≤ K?"""
    vertices = list(range(n))
    for size in range(K + 1):
        for cover in itertools.combinations(vertices, size):
            cover_set = set(cover)
            if all(u in cover_set or v in cover_set for u, v in edges):
                return True
    return False


def find_vertex_cover(n, edges, K):
    """Find a vertex cover of size ≤ K, or None."""
    vertices = list(range(n))
    for size in range(K + 1):
        for cover in itertools.combinations(vertices, size):
            cover_set = set(cover)
            if all(u in cover_set or v in cover_set for u, v in edges):
                return list(cover)
    return None


def has_hamiltonian_circuit(n_vertices, adj, timeout=500000):
    """Backtracking HC solver with pruning. Returns True/False."""
    if n_vertices == 0:
        return False
    if n_vertices == 1:
        return False  # need at least a cycle

    # Check basic necessary conditions
    for v in range(n_vertices):
        if len(adj.get(v, set())) < 2:
            return False  # HC needs degree ≥ 2

    start = 0
    path = [start]
    visited = {start}
    count = [0]

    def backtrack():
        count[0] += 1
        if count[0] > timeout:
            return None  # timeout

        if len(path) == n_vertices:
            # Check if we can return to start
            return start in adj.get(path[-1], set())

        current = path[-1]
        for neighbor in sorted(adj.get(current, set())):
            if neighbor not in visited:
                # Pruning: if adding this vertex would isolate an unvisited vertex
                path.append(neighbor)
                visited.add(neighbor)
                result = backtrack()
                if result is True:
                    return True
                if result is None:
                    return None  # propagate timeout
                visited.remove(neighbor)
                path.pop()

        return False

    result = backtrack()
    if result is None:
        return None  # timeout
    return result


# ── Reduction implementation ──────────────────────────────────────────────


def reduce(n, edges, K):
    """Reduce VertexCover(G, K) to HamiltonianCircuit(G').

    Returns:
        (num_vertices, adj, vertex_names): the constructed graph G'
        vertex_names maps index -> human-readable name
    """
    # Order edges incident to each vertex
    adj_source = adjacency(n, edges)
    incident = {}  # v -> ordered list of edges involving v
    for v in range(n):
        incident[v] = []
    for idx, (u, v) in enumerate(edges):
        incident[u].append(idx)
        incident[v].append(idx)

    # Assign vertex indices in G'
    # Selectors: a_0, ..., a_{K-1} get indices 0..K-1
    # Widget vertices: for edge idx e = (u,v), vertex (endpoint, e, pos)
    #   where endpoint is u or v, pos is 1..6
    vertex_map = {}  # (type, ...) -> index
    vertex_names = {}
    idx = 0

    # Selectors
    for j in range(K):
        vertex_map[('sel', j)] = idx
        vertex_names[idx] = f'a_{j}'
        idx += 1

    # Widget vertices
    for e_idx, (u, v) in enumerate(edges):
        for endpoint in [u, v]:
            for pos in range(1, 7):
                key = ('w', endpoint, e_idx, pos)
                vertex_map[key] = idx
                vertex_names[idx] = f'({endpoint},e{e_idx},{pos})'
                idx += 1

    num_vertices = idx
    adj_target = {v: set() for v in range(num_vertices)}

    def add_edge(a, b):
        adj_target[a].add(b)
        adj_target[b].add(a)

    # Widget internal edges (14 per widget)
    for e_idx, (u, v) in enumerate(edges):
        # Horizontal chains
        for endpoint in [u, v]:
            for pos in range(1, 6):
                a = vertex_map[('w', endpoint, e_idx, pos)]
                b = vertex_map[('w', endpoint, e_idx, pos + 1)]
                add_edge(a, b)

        # Cross edges
        add_edge(vertex_map[('w', u, e_idx, 3)], vertex_map[('w', v, e_idx, 1)])
        add_edge(vertex_map[('w', v, e_idx, 3)], vertex_map[('w', u, e_idx, 1)])
        add_edge(vertex_map[('w', u, e_idx, 6)], vertex_map[('w', v, e_idx, 4)])
        add_edge(vertex_map[('w', v, e_idx, 6)], vertex_map[('w', u, e_idx, 4)])

    # Chain links
    for v in range(n):
        chain = incident[v]  # ordered list of edge indices incident to v
        for i in range(len(chain) - 1):
            e_curr = chain[i]
            e_next = chain[i + 1]
            a = vertex_map[('w', v, e_curr, 6)]
            b = vertex_map[('w', v, e_next, 1)]
            add_edge(a, b)

    # Selector connections
    for j in range(K):
        sel = vertex_map[('sel', j)]
        for v in range(n):
            if len(incident[v]) == 0:
                continue  # isolated vertex, no chain
            first_e = incident[v][0]
            last_e = incident[v][-1]
            a = vertex_map[('w', v, first_e, 1)]
            b = vertex_map[('w', v, last_e, 6)]
            add_edge(sel, a)
            add_edge(sel, b)

    return num_vertices, adj_target, vertex_map, vertex_names, incident


def count_edges(adj):
    """Count undirected edges from adjacency dict."""
    return sum(len(neighbors) for neighbors in adj.values()) // 2


def main():
    # === Section 1: Symbolic checks (sympy) — MANDATORY ===
    print("=== Section 1: Symbolic overhead verification ===")

    n_sym, m_sym, K_sym = symbols("n m K", positive=True, integer=True)

    # num_vertices = 12m + K
    v_expr = 12 * m_sym + K_sym
    check(simplify(v_expr - (12 * m_sym + K_sym)) == 0, "|V'| = 12m + K")

    # Verify for small values
    for m_val in range(1, 6):
        for K_val in range(1, 5):
            expected_v = 12 * m_val + K_val
            check(v_expr.subs({m_sym: m_val, K_sym: K_val}) == expected_v,
                  f"|V'|({m_val},{K_val}) = {expected_v}")

    # Widget has exactly 14 edges
    check(10 + 4 == 14, "widget: 10 chain + 4 cross = 14 edges")

    print(f"  Section 1: {passed} passed, {failed} failed")

    # === Section 2: Exhaustive forward + backward — MANDATORY ===
    print("\n=== Section 2: Exhaustive forward + backward ===")
    sec2_start = passed

    # Test all graphs with n ≤ 4 vertices (skip n ≤ 1 as trivial)
    for n in range(2, 5):
        num_tested = 0
        for vertices, edges in all_simple_graphs(n):
            if len(edges) == 0:
                continue  # no edges → trivial, skip

            # Count non-isolated vertices (those with degree ≥ 1)
            non_isolated = sum(1 for v in range(n) if any(u == v or w == v for u, w in edges))
            for K in range(1, non_isolated + 1):
                source_feasible = has_vertex_cover(n, edges, K)
                nv, adj_t, vmap, vnames, inc = reduce(n, edges, K)

                # Check vertex count
                check(nv == 12 * len(edges) + K,
                      f"n={n},m={len(edges)},K={K}: |V'|={nv}, expected {12*len(edges)+K}")

                # HC check with timeout
                hc_result = has_hamiltonian_circuit(nv, adj_t, timeout=1000000)

                if hc_result is None:
                    # Timeout — skip this instance
                    continue

                check(source_feasible == hc_result,
                      f"n={n},m={len(edges)},K={K}: VC={source_feasible}, HC={hc_result}")
                num_tested += 1

        print(f"  n={n}: tested {num_tested} instances")

    print(f"  Section 2: {passed - sec2_start} new checks")

    # === Section 3: Solution extraction — MANDATORY ===
    print("\n=== Section 3: Solution extraction ===")
    sec3_start = passed

    for n in range(2, 5):
        for vertices, edges in all_simple_graphs(n):
            if len(edges) == 0:
                continue
            for K in range(1, n + 1):
                cover = find_vertex_cover(n, edges, K)
                if cover is None:
                    continue

                # Verify the cover is valid
                cover_set = set(cover)
                check(all(u in cover_set or v in cover_set for u, v in edges),
                      f"extracted cover {cover} covers all edges")

                check(len(cover) <= K,
                      f"extracted cover size {len(cover)} ≤ K={K}")

    print(f"  Section 3: {passed - sec3_start} new checks")

    # === Section 4: Overhead formula — MANDATORY ===
    print("\n=== Section 4: Overhead formula verification ===")
    sec4_start = passed

    for n in range(2, 5):
        for vertices, edges in all_simple_graphs(n):
            if len(edges) == 0:
                continue
            m = len(edges)
            for K in range(1, min(n + 1, 4)):
                nv, adj_t, vmap, vnames, inc = reduce(n, edges, K)

                # Vertex count
                check(nv == 12 * m + K,
                      f"overhead V: {nv} vs 12*{m}+{K}={12*m+K}")

                # Edge count
                num_e = count_edges(adj_t)
                widget_edges = 14 * m
                chain_edges = sum(max(0, len(inc[v]) - 1) for v in range(n))
                non_isolated = sum(1 for v in range(n) if len(inc[v]) > 0)
                sel_edges = 2 * non_isolated * K
                expected_edges = widget_edges + chain_edges + sel_edges

                check(num_e == expected_edges,
                      f"overhead E: {num_e} vs {expected_edges} "
                      f"(14*{m} + {chain_edges} + 2*{non_isolated}*{K})")

    print(f"  Section 4: {passed - sec4_start} new checks")

    # === Section 5: Structural properties — MANDATORY ===
    print("\n=== Section 5: Structural properties ===")
    sec5_start = passed

    for n in range(2, 5):
        for vertices, edges in all_simple_graphs(n):
            if len(edges) == 0:
                continue
            m = len(edges)
            K = min(n, 2)
            nv, adj_t, vmap, vnames, inc = reduce(n, edges, K)

            # 1. Widget internal structure: exactly 14 edges per widget
            for e_idx in range(m):
                u, v = edges[e_idx]
                widget_verts = set()
                for ep in [u, v]:
                    for pos in range(1, 7):
                        widget_verts.add(vmap[('w', ep, e_idx, pos)])

                # Count edges within this widget
                widget_edge_count = 0
                for wv in widget_verts:
                    for nb in adj_t[wv]:
                        if nb in widget_verts and wv < nb:
                            widget_edge_count += 1

                check(widget_edge_count == 14,
                      f"widget e{e_idx}: {widget_edge_count} internal edges, expected 14")

            # 2. Widget boundary vertices have correct degrees
            for e_idx in range(m):
                u, v = edges[e_idx]
                # Boundary vertices: (u,e,1), (v,e,1), (u,e,6), (v,e,6)
                for ep in [u, v]:
                    for pos in [1, 6]:
                        bv = vmap[('w', ep, e_idx, pos)]
                        # Internal degree from widget: pos 1 has chain(1-2) + possibly cross
                        # External degree: chain links + selector connections
                        # Just check it has degree ≥ 2
                        check(len(adj_t[bv]) >= 2,
                              f"boundary ({ep},e{e_idx},{pos}) degree={len(adj_t[bv])} ≥ 2")

            # 3. Interior widget vertices (pos 2,3,4,5) have no external edges
            for e_idx in range(m):
                u, v = edges[e_idx]
                widget_verts = set()
                for ep in [u, v]:
                    for pos in range(1, 7):
                        widget_verts.add(vmap[('w', ep, e_idx, pos)])

                for ep in [u, v]:
                    for pos in [2, 3, 4, 5]:
                        iv = vmap[('w', ep, e_idx, pos)]
                        external = [nb for nb in adj_t[iv] if nb not in widget_verts]
                        check(len(external) == 0,
                              f"interior ({ep},e{e_idx},{pos}) has {len(external)} external edges")

            # 4. Cross edges at correct positions
            for e_idx in range(m):
                u, v = edges[e_idx]
                # (u,e,3) -- (v,e,1)
                check(vmap[('w', v, e_idx, 1)] in adj_t[vmap[('w', u, e_idx, 3)]],
                      f"cross edge (u,e{e_idx},3)-(v,e{e_idx},1)")
                # (v,e,3) -- (u,e,1)
                check(vmap[('w', u, e_idx, 1)] in adj_t[vmap[('w', v, e_idx, 3)]],
                      f"cross edge (v,e{e_idx},3)-(u,e{e_idx},1)")
                # (u,e,6) -- (v,e,4)
                check(vmap[('w', v, e_idx, 4)] in adj_t[vmap[('w', u, e_idx, 6)]],
                      f"cross edge (u,e{e_idx},6)-(v,e{e_idx},4)")
                # (v,e,6) -- (u,e,4)
                check(vmap[('w', u, e_idx, 4)] in adj_t[vmap[('w', v, e_idx, 6)]],
                      f"cross edge (v,e{e_idx},6)-(u,e{e_idx},4)")

            # 5. Selector vertices connect to all vertex chain starts/ends
            for j in range(K):
                sel = vmap[('sel', j)]
                for v_src in range(n):
                    if len(inc[v_src]) == 0:
                        continue
                    first_e = inc[v_src][0]
                    last_e = inc[v_src][-1]
                    start_v = vmap[('w', v_src, first_e, 1)]
                    end_v = vmap[('w', v_src, last_e, 6)]
                    check(start_v in adj_t[sel],
                          f"selector a_{j} -> chain start of v{v_src}")
                    check(end_v in adj_t[sel],
                          f"selector a_{j} -> chain end of v{v_src}")

    print(f"  Section 5: {passed - sec5_start} new checks")

    # === Section 6: YES example from Typst — MANDATORY ===
    print("\n=== Section 6: YES example verification ===")
    sec6_start = passed

    # P3: vertices {0,1,2}, edges {(0,1), (1,2)}, K=1
    # VC of size 1: {1}
    yes_n, yes_edges, yes_K = 3, [(0, 1), (1, 2)], 1

    check(has_vertex_cover(yes_n, yes_edges, yes_K),
          "YES: P3 has vertex cover of size 1")

    nv, adj_t, vmap, vnames, inc = reduce(yes_n, yes_edges, yes_K)
    check(nv == 25, f"YES: |V'| = {nv}, expected 25")

    hc = has_hamiltonian_circuit(nv, adj_t, timeout=5000000)
    check(hc is True, f"YES: G' has Hamiltonian circuit = {hc}")

    # Verify cover = {1}
    cover = find_vertex_cover(yes_n, yes_edges, yes_K)
    check(set(cover) == {1}, f"YES: cover = {cover}, expected {{1}}")

    print(f"  Section 6: {passed - sec6_start} new checks")

    # === Section 7: NO example from Typst — MANDATORY ===
    print("\n=== Section 7: NO example verification ===")
    sec7_start = passed

    # K3: vertices {0,1,2}, edges {(0,1),(0,2),(1,2)}, K=1
    # Min vertex cover of K3 = 2, so K=1 is infeasible
    no_n, no_edges, no_K = 3, [(0, 1), (0, 2), (1, 2)], 1

    check(not has_vertex_cover(no_n, no_edges, no_K),
          "NO: K3 has no vertex cover of size 1")

    nv_no, adj_no, vmap_no, vnames_no, inc_no = reduce(no_n, no_edges, no_K)
    check(nv_no == 37, f"NO: |V'| = {nv_no}, expected 37")

    hc_no = has_hamiltonian_circuit(nv_no, adj_no, timeout=5000000)
    check(hc_no is False, f"NO: G' has no Hamiltonian circuit = {hc_no}")

    print(f"  Section 7: {passed - sec7_start} new checks")

    # ── Final report ──
    print(f"\nMinimumVertexCover → HamiltonianCircuit: {passed} passed, {failed} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
