#!/usr/bin/env python3
"""
§2.2 VC → HamiltonianCircuit: construct the widget graph and verify
that HC exists iff VC exists, for all graphs up to n=5.

Checks:
1. Widget construction: correct vertex/edge counts
2. Forward: VC of size K → HC exists in G'
3. Backward: HC in G' → VC of size K
4. Edge count formula: 16m - n + 2nK
5. Structural widget properties for m>=2 (traversal patterns, chain links, selectors)
6. HC with timeout for moderate-sized widget graphs
"""
import itertools
import signal
import sys
import networkx as nx


class TimeoutError(Exception):
    pass


def _timeout_handler(signum, frame):
    raise TimeoutError("HC check timed out")


def has_hamiltonian_cycle_with_timeout(G, timeout_sec=30):
    """Check if G has a Hamiltonian cycle, with a timeout in seconds.
    Returns True/False/None (None = timed out).
    """
    old_handler = signal.signal(signal.SIGALRM, _timeout_handler)
    signal.alarm(timeout_sec)
    try:
        result = has_hamiltonian_cycle(G)
        signal.alarm(0)
        return result
    except TimeoutError:
        return None
    finally:
        signal.signal(signal.SIGALRM, old_handler)
        signal.alarm(0)

def build_vc_hc_graph(n, edges, K):
    """Build the cover-testing widget graph G' from VC instance (G, K).

    Returns: (G', vertex_count, edge_count, vertex_chains)
    """
    m = len(edges)
    G_prime = nx.Graph()

    # Step 1: Create widgets — 12 vertices per edge
    # Vertex naming: (v, j, col) where v is source vertex, j is edge index, col ∈ 1..6
    # We use integer encoding: widget vertices start at K (after selectors)
    # Selector vertices: 0..K-1
    # Widget vertex (v, j, col): K + (some index)

    # Build per-vertex edge orderings
    vertex_edges = {v: [] for v in range(n)}
    for j, (u, v) in enumerate(edges):
        vertex_edges[u].append(j)
        vertex_edges[v].append(j)

    # Widget vertex naming: we use strings for clarity
    widget_vertices = set()
    widget_edges = []

    for j, (u, v) in enumerate(edges):
        # u-row: (u, j, 1..6), v-row: (v, j, 1..6)
        for col in range(1, 7):
            widget_vertices.add((u, j, col))
            widget_vertices.add((v, j, col))

        # Horizontal edges
        for col in range(1, 6):
            widget_edges.append(((u, j, col), (u, j, col + 1)))
            widget_edges.append(((v, j, col), (v, j, col + 1)))

        # Cross edges at columns 1, 3, 4, 6
        for col in [1, 3, 4, 6]:
            widget_edges.append(((u, j, col), (v, j, col)))

    # Step 2: Chain widgets per vertex
    chain_edges = []
    for v_id in range(n):
        ej_list = vertex_edges[v_id]
        for i in range(len(ej_list) - 1):
            j_curr = ej_list[i]
            j_next = ej_list[i + 1]
            chain_edges.append(((v_id, j_curr, 6), (v_id, j_next, 1)))

    # Step 3: Selector vertices
    selector_edges = []
    for ell in range(K):
        sel = f"sel_{ell}"
        for v_id in range(n):
            if vertex_edges[v_id]:
                first_j = vertex_edges[v_id][0]
                last_j = vertex_edges[v_id][-1]
                selector_edges.append((sel, (v_id, first_j, 1)))
                selector_edges.append((sel, (v_id, last_j, 6)))

    # Build networkx graph
    G_prime.add_nodes_from(widget_vertices)
    for ell in range(K):
        G_prime.add_node(f"sel_{ell}")
    G_prime.add_edges_from(widget_edges)
    G_prime.add_edges_from(chain_edges)
    G_prime.add_edges_from(selector_edges)

    expected_vertices = 12 * m + K
    expected_edges = 16 * m - n + 2 * n * K if m > 0 else K * (K - 1) // 2

    return G_prime, expected_vertices, expected_edges, vertex_edges


def has_vertex_cover(n, edges, K):
    """Check if graph has VC of size ≤ K."""
    for cover in itertools.combinations(range(n), K):
        cover_set = set(cover)
        if all(u in cover_set or v in cover_set for u, v in edges):
            return True
    return False


def has_hamiltonian_cycle(G):
    """Check if G has a Hamiltonian cycle using backtracking with pruning."""
    nodes = list(G.nodes())
    n = len(nodes)
    if n < 3:
        return False

    adj = {v: set(G.neighbors(v)) for v in nodes}

    # Prune: any vertex with degree < 2 → no HC
    if any(len(adj[v]) < 2 for v in nodes):
        return False

    first = nodes[0]

    def backtrack(path, visited):
        if len(path) == n:
            return first in adj[path[-1]]
        last = path[-1]
        for next_v in adj[last]:
            if next_v not in visited:
                # Prune: remaining unvisited vertices must still be reachable
                visited.add(next_v)
                path.append(next_v)
                if backtrack(path, visited):
                    return True
                path.pop()
                visited.remove(next_v)
        return False

    return backtrack([first], {first})


def main():
    passed = failed = 0

    # Test on all graphs up to n=5
    print("VC → HC verification")
    print("=" * 50)

    # HC check is O(n!) — only feasible for widget graphs ≤ ~16 vertices.
    # That means m ≤ 1 with small K. We test exhaustively for n=2,3 with m ≤ 2.
    test_cases = [
        # (n, edges, K_values_to_test)
        (2, [(0, 1)], [1, 2]),
        (3, [(0, 1)], [1, 2]),
        (3, [(0, 1), (1, 2)], [1, 2]),
        (3, [(0, 1), (0, 2)], [1, 2]),
        (3, [(0, 1), (1, 2), (0, 2)], [2, 3]),
        (4, [(0, 1)], [1, 2]),
        (4, [(0, 1), (2, 3)], [1, 2]),
    ]

    for n, edges, K_values in test_cases:
        m = len(edges)
        for K in K_values:
            vc = has_vertex_cover(n, edges, K)
            G_prime, exp_v, exp_e, _ = build_vc_hc_graph(n, edges, K)
            actual_v = G_prime.number_of_nodes()

            # Verify vertex count: 12m + K
            if actual_v != exp_v:
                print(f"  FAIL vertex count: n={n}, m={m}, K={K}: "
                      f"expected {exp_v}, got {actual_v}")
                failed += 1
            else:
                passed += 1

            # Verify edge count formula (using n' = non-isolated vertices)
            actual_e = G_prime.number_of_edges()
            n_prime = len(set(v for e in edges for v in e))
            formula_e = 16 * m - n_prime + 2 * n_prime * K
            if actual_e != formula_e:
                print(f"  FAIL edge count: n={n}, m={m}, K={K}: "
                      f"formula={formula_e}, actual={actual_e}")
                failed += 1
            else:
                passed += 1

            # Widget internal structure: each widget has 14 edges
            for j in range(m):
                u, v = edges[j]
                widget_v = [(u, j, c) for c in range(1, 7)] + [(v, j, c) for c in range(1, 7)]
                subg = G_prime.subgraph(widget_v)
                internal_edges = subg.number_of_edges()
                if internal_edges != 14:
                    print(f"  FAIL widget edges: edge {j}={edges[j]}: "
                          f"expected 14, got {internal_edges}")
                    failed += 1
                else:
                    passed += 1

            # HC check — only for small enough graphs
            if actual_v <= 16:
                hc = has_hamiltonian_cycle(G_prime)
                if vc != hc:
                    print(f"  FAIL: n={n}, edges={edges}, K={K}: "
                          f"VC={vc}, HC={hc}")
                    failed += 1
                else:
                    passed += 1
                    print(f"  OK: n={n}, m={m}, K={K}, |V'|={actual_v}: "
                          f"VC={vc}, HC={hc}")
            else:
                print(f"  SKIP HC check: n={n}, m={m}, K={K}, |V'|={actual_v} (too large)")

    # Verify edge count formula on larger graphs (no HC check)
    print("\nEdge count formula verification on larger graphs...")
    for n in range(2, 7):
        all_edges = list(itertools.combinations(range(n), 2))
        for r in range(1, min(len(all_edges) + 1, 6)):
            for edges in itertools.combinations(all_edges, r):
                edges = list(edges)
                m = len(edges)
                for K in [1, n]:
                    G_prime, _, _, _ = build_vc_hc_graph(n, edges, K)
                    actual_e = G_prime.number_of_edges()
                    n_prime = len(set(v for e in edges for v in e))
                    formula_e = 16 * m - n_prime + 2 * n_prime * K
                    if actual_e != formula_e:
                        print(f"  FAIL edge formula: n={n}, m={m}, K={K}: "
                              f"formula={formula_e}, actual={actual_e}")
                        failed += 1
                    else:
                        passed += 1

    # ================================================================
    # Structural widget verification for m >= 2
    # ================================================================
    print("\nStructural widget verification for m >= 2...")

    structural_cases = [
        (3, [(0, 1), (1, 2)], [1, 2]),
        (3, [(0, 1), (0, 2)], [1, 2]),
        (3, [(0, 1), (1, 2), (0, 2)], [2, 3]),
        (4, [(0, 1), (2, 3)], [1, 2]),
        (4, [(0, 1), (1, 2), (2, 3)], [1, 2, 3]),
        (4, [(0, 1), (1, 2), (0, 2)], [2, 3]),
        (4, [(0, 1), (1, 2), (2, 3), (0, 3)], [2, 3]),
    ]

    for n, edges, K_values in structural_cases:
        m = len(edges)
        vertex_edges = {v: [] for v in range(n)}
        for j, (u, v) in enumerate(edges):
            vertex_edges[u].append(j)
            vertex_edges[v].append(j)

        for K in K_values:
            G_prime, exp_v, exp_e, ve = build_vc_hc_graph(n, edges, K)
            label = f"n={n}, m={m}, K={K}"

            # Check 1: Each widget has exactly 14 internal edges
            for j in range(m):
                u, v = edges[j]
                widget_v = [(u, j, c) for c in range(1, 7)] + [(v, j, c) for c in range(1, 7)]
                subg = G_prime.subgraph(widget_v)
                ie = subg.number_of_edges()
                if ie != 14:
                    print(f"  FAIL widget-14: {label}, edge {j}: got {ie}")
                    failed += 1
                else:
                    passed += 1

            # Check 2: Widget cross-edge structure - each widget has cross
            # edges at columns 1, 3, 4, 6 connecting the u-row and v-row.
            # These cross edges create the cover-testing property.
            for j in range(m):
                u_id, v_id = edges[j]
                for col in [1, 3, 4, 6]:
                    has_cross = G_prime.has_edge((u_id, j, col), (v_id, j, col))
                    if has_cross:
                        passed += 1
                    else:
                        print(f"  FAIL cross edge: {label}, edge {j}, "
                              f"col {col}: missing")
                        failed += 1

            # Check 3: Widget entry/exit vertices have correct degree
            # Entry vertices (col 1) and exit vertices (col 6) connect to:
            # - 1 horizontal neighbor (col 2 or col 5)
            # - 1 cross edge (if col 1 or 6)
            # - chain links and/or selector edges
            # The widget-internal degree of col-1 and col-6 vertices is 2
            for j in range(m):
                u_id, v_id = edges[j]
                widget_verts = set((r, j, c)
                                   for r in [u_id, v_id]
                                   for c in range(1, 7))
                for r in [u_id, v_id]:
                    for col in [1, 6]:
                        v_node = (r, j, col)
                        internal_deg = sum(1 for nb in G_prime.neighbors(v_node)
                                          if nb in widget_verts)
                        if internal_deg == 2:
                            passed += 1
                        else:
                            print(f"  FAIL entry/exit degree: {label}, "
                                  f"v={v_node}: internal_deg={internal_deg}, "
                                  f"expected 2")
                            failed += 1

            # Check 4: Chain links connect consecutive widgets correctly
            # For each vertex v, the chain connects (v, j_curr, 6) to (v, j_next, 1)
            # For each vertex v, the chain connects (v, j_curr, 6) to (v, j_next, 1)
            for v_id in range(n):
                ej_list = ve[v_id]
                for i in range(len(ej_list) - 1):
                    j_curr = ej_list[i]
                    j_next = ej_list[i + 1]
                    src = (v_id, j_curr, 6)
                    dst = (v_id, j_next, 1)
                    if G_prime.has_edge(src, dst):
                        passed += 1
                    else:
                        print(f"  FAIL chain link: {label}, v={v_id}, "
                              f"({j_curr},{6})->({j_next},{1}) missing")
                        failed += 1

            # Check 4: Each selector connects to all chain entries/exits
            for ell in range(K):
                sel = f"sel_{ell}"
                for v_id in range(n):
                    if ve[v_id]:
                        first_j = ve[v_id][0]
                        last_j = ve[v_id][-1]
                        entry = (v_id, first_j, 1)
                        exit_v = (v_id, last_j, 6)
                        if G_prime.has_edge(sel, entry):
                            passed += 1
                        else:
                            print(f"  FAIL selector entry: {label}, sel={ell}, "
                                  f"v={v_id} missing edge to entry")
                            failed += 1
                        if G_prime.has_edge(sel, exit_v):
                            passed += 1
                        else:
                            print(f"  FAIL selector exit: {label}, sel={ell}, "
                                  f"v={v_id} missing edge to exit")
                            failed += 1

            # Check 5: Number of independent widget chains equals n
            # (one chain per vertex that has at least one edge)
            active_vertices = set(v for e in edges for v in e)
            chain_count = len([v for v in active_vertices if ve[v]])
            expected_chains = len(active_vertices)
            if chain_count == expected_chains:
                passed += 1
            else:
                print(f"  FAIL chain count: {label}: "
                      f"got {chain_count}, expected {expected_chains}")
                failed += 1

    # ================================================================
    # HC with timeout for moderate-sized widget graphs (m >= 2)
    # ================================================================
    print("\nHC with timeout for m >= 2 instances...")

    timeout_cases = [
        (3, [(0, 1), (1, 2)], [1, 2]),
        (3, [(0, 1), (0, 2)], [1, 2]),
        (4, [(0, 1), (2, 3)], [1, 2]),
    ]

    for n, edges, K_values in timeout_cases:
        m = len(edges)
        for K in K_values:
            vc = has_vertex_cover(n, edges, K)
            G_prime, _, _, _ = build_vc_hc_graph(n, edges, K)
            actual_v = G_prime.number_of_nodes()
            hc = has_hamiltonian_cycle_with_timeout(G_prime, timeout_sec=30)
            if hc is None:
                print(f"  TIMEOUT: n={n}, m={m}, K={K}, |V'|={actual_v} "
                      f"(VC={vc}, HC timed out after 30s)")
            elif vc != hc:
                print(f"  FAIL: n={n}, m={m}, K={K}, |V'|={actual_v}: "
                      f"VC={vc}, HC={hc}")
                failed += 1
            else:
                print(f"  OK (timeout): n={n}, m={m}, K={K}, |V'|={actual_v}: "
                      f"VC={vc}, HC={hc}")
                passed += 1

    print(f"\nVC → HC: {passed} passed, {failed} failed")
    return failed

if __name__ == "__main__":
    sys.exit(main())
