#!/usr/bin/env python3
"""Cross-compare constructor and adversary for VC → HC."""
import itertools
import sys
sys.path.insert(0, "docs/paper/verify-reductions")

from verify_vc_hc import (
    reduce as c_reduce,
    has_vertex_cover as c_has_vc,
    has_hamiltonian_circuit as c_has_hc,
    count_edges as c_count_edges,
)
from adversary_vc_hc import (
    reduce as a_reduce,
    has_vertex_cover as a_has_vc,
    has_hamiltonian_circuit as a_has_hc,
)

agree = disagree = 0
feasibility_mismatch = 0


def normalize_graph(num_v, adj):
    """Canonical form: (num_vertices, frozenset of sorted edge tuples)."""
    edges = set()
    for v in range(num_v) if isinstance(adj, dict) else range(len(adj)):
        for u in (adj[v] if isinstance(adj, dict) else adj.get(v, [])):
            if v < u:
                edges.add((v, u))
    return (num_v, frozenset(edges))


for n in range(2, 5):
    possible_edges = list(itertools.combinations(range(n), 2))
    instances_tested = 0

    for r in range(1, len(possible_edges) + 1):
        for edge_combo in itertools.combinations(possible_edges, r):
            edges = list(edge_combo)
            non_isolated = sum(1 for v in range(n) if any(u == v or w == v for u, w in edges))

            for K in range(1, non_isolated + 1):
                # Constructor
                c_nv, c_adj, c_vmap, c_vnames, c_inc = c_reduce(n, edges, K)

                # Adversary
                a_result = a_reduce(list(range(n)), edges, K)
                a_all_vertices, a_adj = a_result[0], a_result[1]
                a_nv = len(a_all_vertices)

                # Compare vertex counts
                if c_nv == a_nv:
                    agree += 1
                else:
                    disagree += 1
                    print(f"  VERTEX COUNT DISAGREE: n={n}, edges={edges}, K={K}: "
                          f"constructor={c_nv}, adversary={a_nv}")

                # Compare edge counts
                c_ne = c_count_edges(c_adj)
                a_ne = sum(len(v) for v in a_adj.values()) // 2
                if c_ne != a_ne:
                    disagree += 1
                    print(f"  EDGE COUNT DISAGREE: n={n}, edges={edges}, K={K}: "
                          f"constructor={c_ne}, adversary={a_ne}")

                # Compare VC feasibility
                c_vc = c_has_vc(n, edges, K)
                a_vc = a_has_vc(list(range(n)), edges, K)
                if c_vc != a_vc:
                    feasibility_mismatch += 1
                    print(f"  VC MISMATCH: n={n}, edges={edges}, K={K}")

                instances_tested += 1

    print(f"n={n}: tested {instances_tested} instances")

print(f"\nCross-comparison: {agree} agree, {disagree} disagree, "
      f"{feasibility_mismatch} feasibility mismatches")
if disagree > 0 or feasibility_mismatch > 0:
    print("ACTION REQUIRED: investigate discrepancies before proceeding")
    sys.exit(1)
else:
    print("All instances agree between constructor and adversary.")
    sys.exit(0)
