#!/usr/bin/env python3
"""
Verification script: HamiltonianPath -> DegreeConstrainedSpanningTree reduction.
Issue: #911
Reference: Garey & Johnson, Computers and Intractability, ND1, p.206.

Seven mandatory sections:
  1. reduce()         -- the reduction function
  2. extract()        -- solution extraction (back-map)
  3. Brute-force solvers for source and target
  4. Forward: YES source -> YES target
  5. Backward: YES target -> YES source (via extract)
  6. Infeasible: NO source -> NO target
  7. Overhead check

Runs >=5000 checks total, with exhaustive coverage for small graphs.
"""

import json
import sys
import random
from itertools import permutations, product
from typing import Optional


# ---------------------------------------------------------------------
# Section 1: reduce()
# ---------------------------------------------------------------------

def reduce(n: int, edges: list[tuple[int, int]]) -> tuple[int, list[tuple[int, int]], int]:
    """
    Reduce HamiltonianPath(G) -> DegreeConstrainedSpanningTree(G, K=2).

    The graph is passed through unchanged; the degree bound is set to 2.

    Returns: (num_vertices, edges, max_degree)
    """
    return (n, list(edges), 2)


# ---------------------------------------------------------------------
# Section 2: extract()
# ---------------------------------------------------------------------

def extract(
    n: int,
    edges: list[tuple[int, int]],
    target_config: list[int],
) -> list[int]:
    """
    Extract a HamiltonianPath solution from a DegreeConstrainedSpanningTree solution.

    target_config: binary list, length = len(edges), where 1 = edge selected.
    Returns: permutation of 0..n-1 representing the Hamiltonian path.
    """
    if n == 0:
        return []
    if n == 1:
        return [0]

    # Collect selected edges
    selected = [edges[i] for i in range(len(edges)) if target_config[i] == 1]

    # Build adjacency list from selected edges
    adj = [[] for _ in range(n)]
    for u, v in selected:
        adj[u].append(v)
        adj[v].append(u)

    # Find an endpoint (degree 1 vertex)
    start = None
    for v in range(n):
        if len(adj[v]) == 1:
            start = v
            break

    if start is None:
        # Degenerate: single vertex with no edges (n=1 handled above)
        # or something is wrong
        return list(range(n))

    # Walk the path
    path = [start]
    prev = -1
    cur = start
    while len(path) < n:
        for nxt in adj[cur]:
            if nxt != prev:
                path.append(nxt)
                prev = cur
                cur = nxt
                break
        else:
            break

    return path


# ---------------------------------------------------------------------
# Section 3: Brute-force solvers
# ---------------------------------------------------------------------

def has_edge(edges_set: set, u: int, v: int) -> bool:
    """Check if edge (u,v) exists in the edge set."""
    return (u, v) in edges_set or (v, u) in edges_set


def solve_hamiltonian_path(n: int, edges: list[tuple[int, int]]) -> Optional[list[int]]:
    """Brute-force solve HamiltonianPath. Returns vertex permutation or None."""
    if n == 0:
        return []
    if n == 1:
        return [0]

    edges_set = set()
    for u, v in edges:
        edges_set.add((u, v))
        edges_set.add((v, u))

    for perm in permutations(range(n)):
        valid = True
        for i in range(n - 1):
            if not has_edge(edges_set, perm[i], perm[i + 1]):
                valid = False
                break
        if valid:
            return list(perm)
    return None


def solve_dcst(
    n: int, edges: list[tuple[int, int]], max_degree: int
) -> Optional[list[int]]:
    """
    Brute-force solve DegreeConstrainedSpanningTree.
    Returns binary config (edge selection) or None.
    """
    if n == 0:
        return []
    if n == 1:
        return [0] * len(edges)

    m = len(edges)
    # Enumerate all subsets of edges of size n-1
    for config_tuple in product(range(2), repeat=m):
        config = list(config_tuple)
        selected = [edges[i] for i in range(m) if config[i] == 1]

        # Must have exactly n-1 edges
        if len(selected) != n - 1:
            continue

        # Check degree constraint
        degree = [0] * n
        for u, v in selected:
            degree[u] += 1
            degree[v] += 1
        if any(d > max_degree for d in degree):
            continue

        # Check connectivity via BFS
        adj = [[] for _ in range(n)]
        for u, v in selected:
            adj[u].append(v)
            adj[v].append(u)

        visited = [False] * n
        stack = [0]
        visited[0] = True
        count = 1
        while stack:
            cur = stack.pop()
            for nxt in adj[cur]:
                if not visited[nxt]:
                    visited[nxt] = True
                    count += 1
                    stack.append(nxt)

        if count == n:
            return config

    return None


def is_hp_feasible(n: int, edges: list[tuple[int, int]]) -> bool:
    return solve_hamiltonian_path(n, edges) is not None


def is_dcst_feasible(n: int, edges: list[tuple[int, int]], max_degree: int) -> bool:
    return solve_dcst(n, edges, max_degree) is not None


# ---------------------------------------------------------------------
# Section 4: Forward check -- YES source -> YES target
# ---------------------------------------------------------------------

def check_forward(n: int, edges: list[tuple[int, int]]) -> bool:
    """
    If HamiltonianPath(G) is feasible,
    then DegreeConstrainedSpanningTree(G, 2) must also be feasible.
    """
    if not is_hp_feasible(n, edges):
        return True  # vacuously true
    t_n, t_edges, t_k = reduce(n, edges)
    return is_dcst_feasible(t_n, t_edges, t_k)


# ---------------------------------------------------------------------
# Section 5: Backward check -- YES target -> YES source (via extract)
# ---------------------------------------------------------------------

def check_backward(n: int, edges: list[tuple[int, int]]) -> bool:
    """
    If DegreeConstrainedSpanningTree(G, 2) is feasible,
    solve it, extract a HamiltonianPath config, and verify it.
    """
    t_n, t_edges, t_k = reduce(n, edges)
    dcst_sol = solve_dcst(t_n, t_edges, t_k)
    if dcst_sol is None:
        return True  # vacuously true

    path = extract(n, edges, dcst_sol)

    # Verify: path must be a permutation of 0..n-1
    if sorted(path) != list(range(n)):
        return False

    # Verify: consecutive vertices must be adjacent
    edges_set = set()
    for u, v in edges:
        edges_set.add((u, v))
        edges_set.add((v, u))

    for i in range(n - 1):
        if not has_edge(edges_set, path[i], path[i + 1]):
            return False

    return True


# ---------------------------------------------------------------------
# Section 6: Infeasible check -- NO source -> NO target
# ---------------------------------------------------------------------

def check_infeasible(n: int, edges: list[tuple[int, int]]) -> bool:
    """
    If HamiltonianPath(G) is infeasible,
    then DegreeConstrainedSpanningTree(G, 2) must also be infeasible.
    """
    if is_hp_feasible(n, edges):
        return True  # not an infeasible instance; skip
    t_n, t_edges, t_k = reduce(n, edges)
    return not is_dcst_feasible(t_n, t_edges, t_k)


# ---------------------------------------------------------------------
# Section 7: Overhead check
# ---------------------------------------------------------------------

def check_overhead(n: int, edges: list[tuple[int, int]]) -> bool:
    """
    Verify: target num_vertices = source num_vertices,
            target num_edges = source num_edges.
    """
    t_n, t_edges, t_k = reduce(n, edges)
    return t_n == n and len(t_edges) == len(edges) and t_k == 2


# ---------------------------------------------------------------------
# Graph generators
# ---------------------------------------------------------------------

def all_simple_graphs(n: int):
    """Generate all simple undirected graphs on n vertices."""
    possible_edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
    m = len(possible_edges)
    for mask in range(1 << m):
        edges = []
        for k in range(m):
            if mask & (1 << k):
                edges.append(possible_edges[k])
        yield edges


def random_graph(n: int, p: float, rng: random.Random) -> list[tuple[int, int]]:
    """Generate a random Erdos-Renyi graph G(n, p)."""
    edges = []
    for i in range(n):
        for j in range(i + 1, n):
            if rng.random() < p:
                edges.append((i, j))
    return edges


def path_graph(n: int) -> list[tuple[int, int]]:
    """Path graph 0-1-2-..-(n-1)."""
    return [(i, i + 1) for i in range(n - 1)]


def cycle_graph(n: int) -> list[tuple[int, int]]:
    """Cycle graph."""
    if n < 3:
        return path_graph(n)
    return [(i, (i + 1) % n) for i in range(n)]


def complete_graph(n: int) -> list[tuple[int, int]]:
    """Complete graph K_n."""
    return [(i, j) for i in range(n) for j in range(i + 1, n)]


def star_graph(n: int) -> list[tuple[int, int]]:
    """Star graph with center 0."""
    return [(0, i) for i in range(1, n)]


def petersen_graph() -> tuple[int, list[tuple[int, int]]]:
    """The Petersen graph (10 vertices, 15 edges, no Hamiltonian path)."""
    outer = [(i, (i + 1) % 5) for i in range(5)]
    inner = [(5 + i, 5 + (i + 2) % 5) for i in range(5)]
    spokes = [(i, i + 5) for i in range(5)]
    return 10, outer + inner + spokes


# ---------------------------------------------------------------------
# Exhaustive + random test driver
# ---------------------------------------------------------------------

def exhaustive_tests() -> int:
    """
    Exhaustive tests for all graphs with n <= 6.
    Returns number of checks performed.
    """
    checks = 0

    # n=0: trivial
    for n in range(0, 7):
        if n <= 5:
            # All graphs on n vertices
            for edges in all_simple_graphs(n):
                assert check_forward(n, edges), (
                    f"Forward FAILED: n={n}, edges={edges}"
                )
                assert check_backward(n, edges), (
                    f"Backward FAILED: n={n}, edges={edges}"
                )
                assert check_infeasible(n, edges), (
                    f"Infeasible FAILED: n={n}, edges={edges}"
                )
                assert check_overhead(n, edges), (
                    f"Overhead FAILED: n={n}, edges={edges}"
                )
                checks += 4
        else:
            # n=6: sample graphs (all graphs too many: 2^15 = 32768)
            # Use structured families + random sample
            for edges in [
                path_graph(n),
                cycle_graph(n),
                complete_graph(n),
                star_graph(n),
                [],  # empty graph
            ]:
                assert check_forward(n, edges), (
                    f"Forward FAILED: n={n}, edges={edges}"
                )
                assert check_backward(n, edges), (
                    f"Backward FAILED: n={n}, edges={edges}"
                )
                assert check_infeasible(n, edges), (
                    f"Infeasible FAILED: n={n}, edges={edges}"
                )
                assert check_overhead(n, edges), (
                    f"Overhead FAILED: n={n}, edges={edges}"
                )
                checks += 4

    return checks


def structured_tests() -> int:
    """Tests on well-known graph families."""
    checks = 0

    test_cases = []

    # Path graphs (always have HP)
    for n in range(1, 9):
        test_cases.append((n, path_graph(n), f"path_{n}"))

    # Cycle graphs (always have HP for n >= 3; for n=1,2 path_graph fallback)
    for n in range(3, 9):
        test_cases.append((n, cycle_graph(n), f"cycle_{n}"))

    # Complete graphs (always have HP for n >= 1)
    for n in range(1, 8):
        test_cases.append((n, complete_graph(n), f"complete_{n}"))

    # Star graphs (HP exists only for n <= 2)
    for n in range(2, 8):
        test_cases.append((n, star_graph(n), f"star_{n}"))

    # Petersen graph (no Hamiltonian path)
    pn, pe = petersen_graph()
    test_cases.append((pn, pe, "petersen"))

    # K_{1,4} + edge {1,2} from the issue (no HP)
    test_cases.append((5, [(0, 1), (0, 2), (0, 3), (0, 4), (1, 2)], "star_plus_edge"))

    # Disconnected graphs (no HP)
    test_cases.append((4, [(0, 1), (2, 3)], "two_components"))
    test_cases.append((5, [(0, 1), (1, 2)], "partial_path"))

    # Empty graphs (no HP for n >= 2)
    for n in range(2, 6):
        test_cases.append((n, [], f"empty_{n}"))

    for n, edges, label in test_cases:
        assert check_forward(n, edges), f"Forward FAILED: {label}"
        assert check_backward(n, edges), f"Backward FAILED: {label}"
        assert check_infeasible(n, edges), f"Infeasible FAILED: {label}"
        assert check_overhead(n, edges), f"Overhead FAILED: {label}"
        checks += 4

    return checks


def random_tests(count: int = 500, max_n: int = 8) -> int:
    """Random tests with larger instances. Returns number of checks."""
    rng = random.Random(42)
    checks = 0
    for _ in range(count):
        n = rng.randint(1, max_n)
        p = rng.choice([0.2, 0.3, 0.5, 0.7, 0.9])
        edges = random_graph(n, p, rng)

        assert check_forward(n, edges), (
            f"Forward FAILED: n={n}, edges={edges}"
        )
        assert check_backward(n, edges), (
            f"Backward FAILED: n={n}, edges={edges}"
        )
        assert check_infeasible(n, edges), (
            f"Infeasible FAILED: n={n}, edges={edges}"
        )
        assert check_overhead(n, edges), (
            f"Overhead FAILED: n={n}, edges={edges}"
        )
        checks += 4
    return checks


def collect_test_vectors(count: int = 20) -> list[dict]:
    """Collect representative test vectors for downstream consumption."""
    rng = random.Random(123)
    vectors = []

    # Hand-crafted vectors
    hand_crafted = [
        {
            "n": 5,
            "edges": [[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4], [3, 4]],
            "label": "yes_issue_example",
        },
        {
            "n": 5,
            "edges": [[0, 1], [0, 2], [0, 3], [0, 4], [1, 2]],
            "label": "no_star_plus_edge",
        },
        {
            "n": 4,
            "edges": [[0, 1], [1, 2], [2, 3]],
            "label": "yes_path_4",
        },
        {
            "n": 4,
            "edges": [[0, 1], [1, 2], [2, 3], [3, 0]],
            "label": "yes_cycle_4",
        },
        {
            "n": 4,
            "edges": [[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]],
            "label": "yes_complete_4",
        },
        {
            "n": 4,
            "edges": [[0, 1], [0, 2], [0, 3]],
            "label": "no_star_4",
        },
        {
            "n": 4,
            "edges": [[0, 1], [2, 3]],
            "label": "no_disconnected",
        },
        {
            "n": 1,
            "edges": [],
            "label": "yes_single_vertex",
        },
        {
            "n": 2,
            "edges": [[0, 1]],
            "label": "yes_single_edge",
        },
        {
            "n": 3,
            "edges": [],
            "label": "no_empty_3",
        },
    ]

    for hc in hand_crafted:
        n = hc["n"]
        edges = [tuple(e) for e in hc["edges"]]
        t_n, t_edges, t_k = reduce(n, edges)
        hp_sol = solve_hamiltonian_path(n, edges)
        dcst_sol = solve_dcst(t_n, t_edges, t_k)
        extracted = None
        if dcst_sol is not None:
            extracted = extract(n, edges, dcst_sol)
        vectors.append({
            "label": hc["label"],
            "source": {"num_vertices": n, "edges": [list(e) for e in edges]},
            "target": {
                "num_vertices": t_n,
                "edges": [list(e) for e in t_edges],
                "max_degree": t_k,
            },
            "source_feasible": hp_sol is not None,
            "target_feasible": dcst_sol is not None,
            "source_solution": list(hp_sol) if hp_sol is not None else None,
            "target_solution": dcst_sol,
            "extracted_solution": extracted,
        })

    # Random vectors
    for i in range(count - len(hand_crafted)):
        n = rng.randint(2, 6)
        edges = random_graph(n, rng.choice([0.3, 0.5, 0.7]), rng)
        t_n, t_edges, t_k = reduce(n, edges)
        hp_sol = solve_hamiltonian_path(n, edges)
        dcst_sol = solve_dcst(t_n, t_edges, t_k)
        extracted = None
        if dcst_sol is not None:
            extracted = extract(n, edges, dcst_sol)
        vectors.append({
            "label": f"random_{i}",
            "source": {"num_vertices": n, "edges": [list(e) for e in edges]},
            "target": {
                "num_vertices": t_n,
                "edges": [list(e) for e in t_edges],
                "max_degree": t_k,
            },
            "source_feasible": hp_sol is not None,
            "target_feasible": dcst_sol is not None,
            "source_solution": list(hp_sol) if hp_sol is not None else None,
            "target_solution": dcst_sol,
            "extracted_solution": extracted,
        })

    return vectors


if __name__ == "__main__":
    print("=" * 60)
    print("HamiltonianPath -> DegreeConstrainedSpanningTree verification")
    print("=" * 60)

    print("\n[1/4] Exhaustive tests (n <= 5, all graphs)...")
    n_exhaustive = exhaustive_tests()
    print(f"  Exhaustive checks: {n_exhaustive}")

    print("\n[2/4] Structured graph family tests...")
    n_structured = structured_tests()
    print(f"  Structured checks: {n_structured}")

    print("\n[3/4] Random tests...")
    n_random = random_tests(count=500)
    print(f"  Random checks: {n_random}")

    total = n_exhaustive + n_structured + n_random
    print(f"\n  TOTAL checks: {total}")
    assert total >= 5000, f"Need >=5000 checks, got {total}"

    print("\n[4/4] Generating test vectors...")
    vectors = collect_test_vectors(count=20)

    # Validate all vectors
    for v in vectors:
        n = v["source"]["num_vertices"]
        edges = [tuple(e) for e in v["source"]["edges"]]
        if v["source_feasible"]:
            assert v["target_feasible"], f"Forward violation in {v['label']}"
            if v["extracted_solution"] is not None:
                path = v["extracted_solution"]
                assert sorted(path) == list(range(n)), (
                    f"Extract not a permutation in {v['label']}"
                )
                edges_set = set()
                for u, w in edges:
                    edges_set.add((u, w))
                    edges_set.add((w, u))
                for i in range(n - 1):
                    assert (path[i], path[i + 1]) in edges_set, (
                        f"Extract invalid edge in {v['label']}"
                    )
        if not v["source_feasible"]:
            assert not v["target_feasible"], (
                f"Infeasible violation in {v['label']}"
            )

    # Write test vectors
    out_path = "docs/paper/verify-reductions/test_vectors_hamiltonian_path_degree_constrained_spanning_tree.json"
    with open(out_path, "w") as f:
        json.dump({"vectors": vectors, "total_checks": total}, f, indent=2)
    print(f"  Wrote {len(vectors)} test vectors to {out_path}")

    print(f"\nAll {total} checks PASSED.")
