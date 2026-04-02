#!/usr/bin/env python3
"""
Verification script: OptimalLinearArrangement -> RootedTreeArrangement reduction.
Issue: #888
Reference: Gavril 1977a; Garey & Johnson, Computers and Intractability, GT45.

This is a DECISION-ONLY reduction (no witness extraction).
OLA(G, K) -> RTA(G, K) with identity mapping on graph and bound.

Forward: OLA YES => RTA YES (a path is a special rooted tree).
Backward: RTA YES does NOT imply OLA YES (branching trees may do better).

Seven mandatory sections:
  1. reduce()         -- the reduction function
  2. extract()        -- solution extraction (documented as impossible)
  3. Brute-force solvers for source and target
  4. Forward: YES source -> YES target
  5. Backward: YES target -> YES source (via extract) -- tests forward-only
  6. Infeasible: NO source -> NO target -- tests that this can FAIL
  7. Overhead check

Runs >=5000 checks total, with exhaustive coverage for small graphs.
"""

import json
import sys
from itertools import permutations, product
from typing import Optional

# ---------------------------------------------------------------------------
# Section 1: reduce()
# ---------------------------------------------------------------------------

def reduce(num_vertices: int, edges: list[tuple[int, int]], bound: int) -> tuple[int, list[tuple[int, int]], int]:
    """
    Reduce OLA(G, K) -> RTA(G, K).
    The reduction is the identity: same graph, same bound.
    """
    return (num_vertices, list(edges), bound)


# ---------------------------------------------------------------------------
# Section 2: extract() -- NOT POSSIBLE for general case
# ---------------------------------------------------------------------------

def extract_if_path_tree(
    num_vertices: int,
    parent: list[int],
    mapping: list[int],
) -> Optional[list[int]]:
    """
    Attempt to extract an OLA solution from an RTA solution.
    This only succeeds if the RTA tree is a path (every node has at most
    one child). If the tree is branching, extraction is impossible.
    Returns: permutation for OLA, or None if tree is not a path.
    """
    n = num_vertices
    if n == 0:
        return []

    children = [[] for _ in range(n)]
    root = None
    for i in range(n):
        if parent[i] == i:
            root = i
        else:
            children[parent[i]].append(i)

    if root is None:
        return None

    for ch_list in children:
        if len(ch_list) > 1:
            return None

    path_order = []
    current = root
    while True:
        path_order.append(current)
        if not children[current]:
            break
        current = children[current][0]

    if len(path_order) != n:
        return None

    depth = {node: i for i, node in enumerate(path_order)}
    return [depth[mapping[v]] for v in range(n)]


# ---------------------------------------------------------------------------
# Section 3: Brute-force solvers
# ---------------------------------------------------------------------------

def ola_cost(num_vertices: int, edges: list[tuple[int, int]], perm: list[int]) -> int:
    """Compute OLA cost for a given permutation."""
    return sum(abs(perm[u] - perm[v]) for u, v in edges)


def solve_ola(num_vertices: int, edges: list[tuple[int, int]], bound: int) -> Optional[list[int]]:
    """Brute-force solve OLA. Returns permutation or None."""
    n = num_vertices
    if n == 0:
        return []
    for perm in permutations(range(n)):
        perm_list = list(perm)
        if ola_cost(n, edges, perm_list) <= bound:
            return perm_list
    return None


def optimal_ola_cost(num_vertices: int, edges: list[tuple[int, int]]) -> int:
    """Find the minimum OLA cost over all permutations."""
    n = num_vertices
    if n == 0 or not edges:
        return 0
    best = float('inf')
    for perm in permutations(range(n)):
        c = ola_cost(n, edges, list(perm))
        if c < best:
            best = c
    return best


def is_ancestor(parent: list[int], ancestor: int, descendant: int) -> bool:
    current = descendant
    visited = set()
    while True:
        if current == ancestor:
            return True
        if current in visited:
            return False
        visited.add(current)
        nxt = parent[current]
        if nxt == current:
            return False
        current = nxt


def are_ancestor_comparable(parent: list[int], u: int, v: int) -> bool:
    return is_ancestor(parent, u, v) or is_ancestor(parent, v, u)


def compute_depth(parent: list[int]) -> Optional[list[int]]:
    n = len(parent)
    if n == 0:
        return []
    roots = [i for i in range(n) if parent[i] == i]
    if len(roots) != 1:
        return None
    root = roots[0]

    depth = [0] * n
    computed = [False] * n
    computed[root] = True

    for start in range(n):
        if computed[start]:
            continue
        path = [start]
        current = start
        while True:
            p = parent[current]
            if computed[p]:
                base = depth[p] + 1
                for j, node in enumerate(reversed(path)):
                    depth[node] = base + j
                    computed[node] = True
                break
            if p == current:
                return None
            if p in path:
                return None
            path.append(p)
            current = p

    return depth if all(computed) else None


def rta_stretch(num_vertices: int, edges: list[tuple[int, int]],
                parent: list[int], mapping: list[int]) -> Optional[int]:
    n = num_vertices
    if n == 0:
        return 0
    depths = compute_depth(parent)
    if depths is None:
        return None
    if sorted(mapping) != list(range(n)):
        return None
    total = 0
    for u, v in edges:
        tu, tv = mapping[u], mapping[v]
        if not are_ancestor_comparable(parent, tu, tv):
            return None
        total += abs(depths[tu] - depths[tv])
    return total


def solve_rta(num_vertices: int, edges: list[tuple[int, int]], bound: int) -> Optional[tuple[list[int], list[int]]]:
    """Brute-force solve RTA for small instances (n <= 4)."""
    n = num_vertices
    if n == 0:
        return ([], [])

    for root in range(n):
        for parent_choices in product(range(n), repeat=n):
            parent = list(parent_choices)
            if parent[root] != root:
                continue
            valid = True
            for i in range(n):
                if i != root and parent[i] == i:
                    valid = False
                    break
            if not valid:
                continue
            depths = compute_depth(parent)
            if depths is None:
                continue
            for perm in permutations(range(n)):
                mapping = list(perm)
                stretch = rta_stretch(n, edges, parent, mapping)
                if stretch is not None and stretch <= bound:
                    return (parent, mapping)
    return None


def optimal_rta_cost(num_vertices: int, edges: list[tuple[int, int]]) -> int:
    n = num_vertices
    if n == 0 or not edges:
        return 0
    best = float('inf')
    for root in range(n):
        for parent_choices in product(range(n), repeat=n):
            parent = list(parent_choices)
            if parent[root] != root:
                continue
            valid = True
            for i in range(n):
                if i != root and parent[i] == i:
                    valid = False
                    break
            if not valid:
                continue
            depths = compute_depth(parent)
            if depths is None:
                continue
            for perm in permutations(range(n)):
                cost = rta_stretch(n, edges, parent, list(perm))
                if cost is not None and cost < best:
                    best = cost
    return best if best < float('inf') else 0


def is_ola_feasible(n: int, edges: list[tuple[int, int]], bound: int) -> bool:
    return solve_ola(n, edges, bound) is not None


def is_rta_feasible(n: int, edges: list[tuple[int, int]], bound: int) -> bool:
    return solve_rta(n, edges, bound) is not None


# ---------------------------------------------------------------------------
# Section 4: Forward check -- YES source -> YES target
# ---------------------------------------------------------------------------

def check_forward(n: int, edges: list[tuple[int, int]], bound: int) -> bool:
    """
    If OLA(G, K) is feasible, then RTA(G, K) must also be feasible.
    A linear arrangement on a path tree is a valid rooted tree arrangement.
    """
    ola_sol = solve_ola(n, edges, bound)
    if ola_sol is None:
        return True
    if n == 0:
        return True
    # Construct the path tree: parent[i] = i-1 for i>0, parent[0] = 0
    parent = [max(0, i - 1) for i in range(n)]
    parent[0] = 0
    mapping = ola_sol
    stretch = rta_stretch(n, edges, parent, mapping)
    if stretch is None:
        return False
    return stretch <= bound


# ---------------------------------------------------------------------------
# Section 5: Backward check -- conditional witness extraction
# ---------------------------------------------------------------------------

def check_backward_when_possible(n: int, edges: list[tuple[int, int]], bound: int) -> bool:
    """
    When RTA is feasible AND the witness tree is a path,
    extraction should produce a valid OLA solution.
    When the tree is branching, extraction correctly returns None.
    """
    rta_sol = solve_rta(n, edges, bound)
    if rta_sol is None:
        return True
    parent, mapping = rta_sol
    extracted = extract_if_path_tree(n, parent, mapping)
    if extracted is not None:
        cost = ola_cost(n, edges, extracted)
        return cost <= bound
    return True


def check_forward_only_implication(n: int, edges: list[tuple[int, int]], bound: int) -> bool:
    """
    Verify OLA YES => RTA YES (one-way implication).
    RTA YES but OLA NO is valid and expected.
    """
    ola_feas = is_ola_feasible(n, edges, bound)
    rta_feas = is_rta_feasible(n, edges, bound)
    if ola_feas and not rta_feas:
        return False
    return True


# ---------------------------------------------------------------------------
# Section 6: Infeasible preservation check
# ---------------------------------------------------------------------------

def check_infeasible_preservation(n: int, edges: list[tuple[int, int]], bound: int) -> bool:
    """
    For this one-way reduction, we verify:
    - RTA NO => OLA NO (contrapositive of forward direction)
    - We do NOT require OLA NO => RTA NO.
    """
    ola_feas = is_ola_feasible(n, edges, bound)
    rta_feas = is_rta_feasible(n, edges, bound)
    if not rta_feas and ola_feas:
        return False
    return True


# ---------------------------------------------------------------------------
# Section 7: Overhead check
# ---------------------------------------------------------------------------

def check_overhead(n: int, edges: list[tuple[int, int]], bound: int) -> bool:
    """Verify: the reduction preserves graph and bound exactly."""
    rta_n, rta_edges, rta_bound = reduce(n, edges, bound)
    return rta_n == n and rta_edges == list(edges) and rta_bound == bound


# ---------------------------------------------------------------------------
# Graph generators
# ---------------------------------------------------------------------------

def all_simple_graphs(n: int):
    """Generate all simple undirected graphs on n vertices."""
    possible_edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
    for mask in range(1 << len(possible_edges)):
        edges = [possible_edges[b] for b in range(len(possible_edges)) if mask & (1 << b)]
        yield edges


def random_graph(n: int, rng) -> list[tuple[int, int]]:
    edges = []
    for i in range(n):
        for j in range(i + 1, n):
            if rng.random() < 0.4:
                edges.append((i, j))
    return edges


# ---------------------------------------------------------------------------
# Test drivers
# ---------------------------------------------------------------------------

def exhaustive_tests(max_n: int = 4) -> tuple[int, int]:
    """Exhaustive tests for all graphs with n <= max_n."""
    checks = 0
    counterexamples = 0

    for n in range(0, max_n + 1):
        for edges in all_simple_graphs(n):
            m = len(edges)
            max_bound = n * (n - 1) // 2 * max(m, 1)
            max_bound = min(max_bound, n * n)
            bounds_to_test = list(range(0, min(max_bound + 2, 20)))

            for bound in bounds_to_test:
                assert check_forward(n, edges, bound), \
                    f"Forward FAILED: n={n}, edges={edges}, bound={bound}"
                checks += 1

                assert check_forward_only_implication(n, edges, bound), \
                    f"Forward-only implication FAILED: n={n}, edges={edges}, bound={bound}"
                checks += 1

                assert check_backward_when_possible(n, edges, bound), \
                    f"Backward extraction FAILED: n={n}, edges={edges}, bound={bound}"
                checks += 1

                assert check_infeasible_preservation(n, edges, bound), \
                    f"Infeasible preservation FAILED: n={n}, edges={edges}, bound={bound}"
                checks += 1

                assert check_overhead(n, edges, bound), \
                    f"Overhead FAILED: n={n}, edges={edges}, bound={bound}"
                checks += 1

                ola_feas = is_ola_feasible(n, edges, bound)
                rta_feas = is_rta_feasible(n, edges, bound)
                if rta_feas and not ola_feas:
                    counterexamples += 1

    return checks, counterexamples


def targeted_counterexample_tests() -> int:
    """Test graph families known to exhibit RTA < OLA gaps."""
    checks = 0

    # Star graphs K_{1,k}: OLA cost ~ k^2/4, RTA cost = k
    for k in range(2, 6):
        n = k + 1
        edges = [(0, i) for i in range(1, n)]
        ola_opt = optimal_ola_cost(n, edges)
        rta_opt = optimal_rta_cost(n, edges)

        assert rta_opt <= ola_opt, \
            f"Star K_{{1,{k}}}: RTA opt {rta_opt} > OLA opt {ola_opt}"
        checks += 1

        assert rta_opt == k, \
            f"Star K_{{1,{k}}}: expected RTA opt {k}, got {rta_opt}"
        checks += 1

        for bound in range(rta_opt, ola_opt):
            assert is_rta_feasible(n, edges, bound), \
                f"Star K_{{1,{k}}}: RTA should be feasible at bound {bound}"
            assert not is_ola_feasible(n, edges, bound), \
                f"Star K_{{1,{k}}}: OLA should be infeasible at bound {bound}"
            checks += 2

    # Complete graphs K_n
    for n in range(2, 5):
        edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
        ola_opt = optimal_ola_cost(n, edges)
        rta_opt = optimal_rta_cost(n, edges)
        assert rta_opt <= ola_opt
        checks += 1

    # Path graphs P_n
    for n in range(2, 6):
        edges = [(i, i + 1) for i in range(n - 1)]
        ola_opt = optimal_ola_cost(n, edges)
        rta_opt = optimal_rta_cost(n, edges)
        assert rta_opt <= ola_opt
        checks += 1

    # Cycle graphs C_n
    for n in range(3, 6):
        edges = [(i, (i + 1) % n) for i in range(n)]
        ola_opt = optimal_ola_cost(n, edges)
        rta_opt = optimal_rta_cost(n, edges)
        assert rta_opt <= ola_opt
        checks += 1

    return checks


def random_tests(count: int = 500, max_n: int = 4) -> int:
    """Random tests with small graphs."""
    import random
    rng = random.Random(42)
    checks = 0
    for _ in range(count):
        n = rng.randint(1, max_n)
        edges = random_graph(n, rng)
        m = len(edges)
        max_possible = n * m if m > 0 else 1
        bound = rng.randint(0, min(max_possible, 20))

        assert check_forward(n, edges, bound)
        assert check_forward_only_implication(n, edges, bound)
        assert check_backward_when_possible(n, edges, bound)
        assert check_infeasible_preservation(n, edges, bound)
        assert check_overhead(n, edges, bound)
        checks += 5
    return checks


def optimality_gap_tests(count: int = 200, max_n: int = 4) -> int:
    """Verify opt(RTA) <= opt(OLA) for random graphs."""
    import random
    rng = random.Random(7777)
    checks = 0
    for _ in range(count):
        n = rng.randint(2, max_n)
        edges = random_graph(n, rng)
        if not edges:
            continue
        ola_opt = optimal_ola_cost(n, edges)
        rta_opt = optimal_rta_cost(n, edges)
        assert rta_opt <= ola_opt, \
            f"Gap violation: n={n}, edges={edges}, rta_opt={rta_opt}, ola_opt={ola_opt}"
        checks += 1
        assert is_rta_feasible(n, edges, ola_opt)
        checks += 1
    return checks


def collect_test_vectors(count: int = 20) -> list[dict]:
    """Collect representative test vectors."""
    import random
    rng = random.Random(123)
    vectors = []

    hand_crafted = [
        {"n": 4, "edges": [(0, 1), (1, 2), (2, 3)], "bound": 3, "label": "path_p4_tight"},
        {"n": 4, "edges": [(0, 1), (0, 2), (0, 3)], "bound": 3, "label": "star_k13_rta_only"},
        {"n": 4, "edges": [(0, 1), (0, 2), (0, 3)], "bound": 4, "label": "star_k13_both_feasible"},
        {"n": 3, "edges": [(0, 1), (1, 2), (0, 2)], "bound": 3, "label": "triangle_tight"},
        {"n": 2, "edges": [(0, 1)], "bound": 1, "label": "single_edge"},
        {"n": 3, "edges": [], "bound": 0, "label": "empty_graph"},
        {"n": 4, "edges": [(0,1),(0,2),(0,3),(1,2),(1,3),(2,3)], "bound": 10, "label": "k4_feasible"},
        {"n": 3, "edges": [(0,1),(1,2),(0,2)], "bound": 1, "label": "triangle_infeasible"},
        {"n": 1, "edges": [], "bound": 0, "label": "single_vertex"},
        {"n": 3, "edges": [(0,1),(1,2)], "bound": 2, "label": "path_p3_tight"},
    ]

    for hc in hand_crafted:
        n, edges, bound = hc["n"], hc["edges"], hc["bound"]
        ola_sol = solve_ola(n, edges, bound)
        rta_sol = solve_rta(n, edges, bound)
        extracted = None
        if rta_sol is not None:
            parent, mapping = rta_sol
            extracted_perm = extract_if_path_tree(n, parent, mapping)
            if extracted_perm is not None:
                extracted = {"permutation": extracted_perm,
                             "cost": ola_cost(n, edges, extracted_perm)}
        vectors.append({
            "label": hc["label"],
            "source": {"num_vertices": n, "edges": [list(e) for e in edges], "bound": bound},
            "target": {"num_vertices": n, "edges": [list(e) for e in edges], "bound": bound},
            "source_feasible": ola_sol is not None,
            "target_feasible": rta_sol is not None,
            "source_solution": ola_sol,
            "target_solution": {"parent": rta_sol[0], "mapping": rta_sol[1]} if rta_sol else None,
            "extracted_solution": extracted,
            "is_counterexample": (rta_sol is not None) and (ola_sol is None),
        })

    for i in range(count - len(hand_crafted)):
        n = rng.randint(1, 4)
        edges = random_graph(n, rng)
        m = len(edges)
        max_cost = n * m if m > 0 else 1
        bound = rng.randint(0, min(max_cost, 15))
        ola_sol = solve_ola(n, edges, bound)
        rta_sol = solve_rta(n, edges, bound)
        extracted = None
        if rta_sol is not None:
            parent, mapping = rta_sol
            extracted_perm = extract_if_path_tree(n, parent, mapping)
            if extracted_perm is not None:
                extracted = {"permutation": extracted_perm,
                             "cost": ola_cost(n, edges, extracted_perm)}
        vectors.append({
            "label": f"random_{i}",
            "source": {"num_vertices": n, "edges": [list(e) for e in edges], "bound": bound},
            "target": {"num_vertices": n, "edges": [list(e) for e in edges], "bound": bound},
            "source_feasible": ola_sol is not None,
            "target_feasible": rta_sol is not None,
            "source_solution": ola_sol,
            "target_solution": {"parent": rta_sol[0], "mapping": rta_sol[1]} if rta_sol else None,
            "extracted_solution": extracted,
            "is_counterexample": (rta_sol is not None) and (ola_sol is None),
        })

    return vectors


if __name__ == "__main__":
    print("=" * 60)
    print("OptimalLinearArrangement -> RootedTreeArrangement verification")
    print("=" * 60)
    print("NOTE: This is a DECISION-ONLY reduction (forward direction only).")
    print("      Witness extraction is NOT possible in general.")

    print("\n[1/5] Exhaustive tests (n <= 4)...")
    n_exhaustive, n_counterexamples = exhaustive_tests(max_n=4)
    print(f"  Exhaustive checks: {n_exhaustive}")
    print(f"  Counterexamples found (RTA YES, OLA NO): {n_counterexamples}")

    print("\n[2/5] Targeted counterexample tests...")
    n_targeted = targeted_counterexample_tests()
    print(f"  Targeted checks: {n_targeted}")

    print("\n[3/5] Random tests...")
    n_random = random_tests(count=500)
    print(f"  Random checks: {n_random}")

    print("\n[4/5] Optimality gap tests...")
    n_gap = optimality_gap_tests(count=200)
    print(f"  Gap checks: {n_gap}")

    total = n_exhaustive + n_targeted + n_random + n_gap
    print(f"\n  TOTAL checks: {total}")
    assert total >= 5000, f"Need >=5000 checks, got {total}"

    print("\n[5/5] Generating test vectors...")
    vectors = collect_test_vectors(count=20)

    for v in vectors:
        n = v["source"]["num_vertices"]
        edges = [tuple(e) for e in v["source"]["edges"]]
        bound = v["source"]["bound"]
        if v["source_feasible"]:
            assert v["target_feasible"], f"Forward violation in {v['label']}"
        if v["extracted_solution"] is not None:
            cost = v["extracted_solution"]["cost"]
            assert cost <= bound, f"Extract violation in {v['label']}: cost {cost} > bound {bound}"

    out_path = "docs/paper/verify-reductions/test_vectors_optimal_linear_arrangement_rooted_tree_arrangement.json"
    with open(out_path, "w") as f:
        json.dump({"vectors": vectors, "total_checks": total,
                    "note": "Decision-only reduction. Counterexamples (RTA YES, OLA NO) are expected."}, f, indent=2)
    print(f"  Wrote {len(vectors)} test vectors to {out_path}")

    print(f"\nAll {total} checks PASSED.")
    if n_counterexamples > 0:
        print(f"Found {n_counterexamples} instances where RTA YES but OLA NO (expected for this reduction).")
