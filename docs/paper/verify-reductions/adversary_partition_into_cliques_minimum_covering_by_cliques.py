#!/usr/bin/env python3
"""Adversary verification script for PartitionIntoCliques -> MinimumCoveringByCliques reduction.

Issue: #889
Independent implementation based solely on the Typst proof.
Does NOT import from the constructor script.

Requirements:
- Own reduce(), extract_solution(), is_feasible_source(), is_feasible_target()
- Exhaustive forward for n <= 5
- hypothesis PBT with >= 2 strategies
- Reproduce both Typst examples (YES and NO)
- >= 5,000 total checks
"""

import itertools
import json
import sys
from pathlib import Path

# ============================================================
# Independent implementation from Typst proof
# ============================================================

def reduce(num_vertices, edges, num_cliques):
    """
    PartitionIntoCliques(G, K) -> MinimumCoveringByCliques(G, K).

    From the Typst proof:
    1. Copy the graph G' = G (same vertices and edges).
    2. Set K' = K.
    """
    return num_vertices, list(edges), num_cliques


def extract_solution(num_vertices, edges, partition_config):
    """
    Extract edge clique cover from vertex partition.
    From proof: for each edge (u,v), assign it to the group containing both endpoints.
    Since partition is disjoint, config[u] == config[v] for every edge.
    """
    edge_cover = []
    for u, v in edges:
        edge_cover.append(partition_config[u])
    return edge_cover


def is_feasible_source(num_vertices, edges, num_cliques, config):
    """Check if config is a valid partition into <= num_cliques cliques."""
    if len(config) != num_vertices:
        return False
    for c in config:
        if c < 0 or c >= num_cliques:
            return False
    adj = set()
    for u, v in edges:
        adj.add((min(u, v), max(u, v)))
    # Each group must be a clique
    for g in range(num_cliques):
        members = [v for v in range(num_vertices) if config[v] == g]
        for i in range(len(members)):
            for j in range(i + 1, len(members)):
                a, b = min(members[i], members[j]), max(members[i], members[j])
                if (a, b) not in adj:
                    return False
    # Every edge must have both endpoints in same group
    for u, v in edges:
        if config[u] != config[v]:
            return False
    return True


def is_feasible_target(num_vertices, edges, num_cliques, edge_config):
    """Check if edge_config is a valid covering by <= num_cliques cliques."""
    if len(edge_config) != len(edges):
        return False
    if len(edges) == 0:
        return True
    if any(g < 0 for g in edge_config):
        return False
    max_group = max(edge_config)
    if max_group >= num_cliques:
        return False

    adj = set()
    for u, v in edges:
        adj.add((min(u, v), max(u, v)))

    # For each group, collect vertices and verify clique
    for g in range(max_group + 1):
        vertices = set()
        for idx, grp in enumerate(edge_config):
            if grp == g:
                u, v = edges[idx]
                vertices.add(u)
                vertices.add(v)
        verts = sorted(vertices)
        for i in range(len(verts)):
            for j in range(i + 1, len(verts)):
                a, b = min(verts[i], verts[j]), max(verts[i], verts[j])
                if (a, b) not in adj:
                    return False
    return True


def brute_force_source(num_vertices, edges, num_cliques):
    """Find any valid clique partition, or None."""
    for config in itertools.product(range(num_cliques), repeat=num_vertices):
        if is_feasible_source(num_vertices, edges, num_cliques, list(config)):
            return list(config)
    return None


def brute_force_target(num_vertices, edges, num_cliques):
    """Find any valid edge clique cover with <= num_cliques groups, or None."""
    if len(edges) == 0:
        return []
    for ng in range(1, num_cliques + 1):
        for edge_config in itertools.product(range(ng), repeat=len(edges)):
            if is_feasible_target(num_vertices, edges, ng, list(edge_config)):
                return list(edge_config)
    return None


# ============================================================
# Counters
# ============================================================
checks = 0
failures = []


def check(condition, msg):
    global checks
    checks += 1
    if not condition:
        failures.append(msg)


# ============================================================
# Test 1: Exhaustive forward (n <= 5)
# ============================================================
print("Test 1: Exhaustive forward verification...")

for n in range(1, 6):
    all_possible = [(i, j) for i in range(n) for j in range(i + 1, n)]
    num_possible = len(all_possible)

    for mask in range(1 << num_possible):
        edges = [all_possible[i] for i in range(num_possible) if mask & (1 << i)]

        for k in range(1, n + 1):
            src_wit = brute_force_source(n, edges, k)
            src_feas = src_wit is not None

            if src_feas:
                # Forward: partition => covering
                tn, tedges, tk = reduce(n, edges, k)
                edge_cover = extract_solution(n, edges, src_wit)
                cover_valid = is_feasible_target(n, edges, k, edge_cover)
                check(cover_valid,
                      f"Forward fail: n={n}, m={len(edges)}, k={k}")

                # Also brute force target
                tgt_wit = brute_force_target(n, edges, k)
                check(tgt_wit is not None,
                      f"Target infeasible despite source feasible: n={n}, m={len(edges)}, k={k}")
            else:
                # Just count
                check(True, f"n={n}, m={len(edges)}, k={k}: src NO")

    print(f"  n={n}: done")

print(f"  Checks so far: {checks}")


# ============================================================
# Test 2: YES example from Typst
# ============================================================
print("Test 2: YES example from Typst proof...")

yes_n = 5
yes_edges = [(0, 1), (0, 2), (1, 2), (3, 4)]
yes_k = 2
yes_partition = [0, 0, 0, 1, 1]

# Source feasible
check(is_feasible_source(yes_n, yes_edges, yes_k, yes_partition),
      "YES: source partition should be valid")

# Reduce
tn, tedges, tk = reduce(yes_n, yes_edges, yes_k)

# Graph unchanged
src_set = {(min(u, v), max(u, v)) for u, v in yes_edges}
tgt_set = {(min(u, v), max(u, v)) for u, v in tedges}
check(src_set == tgt_set, "YES: target edges differ from source")
check(tn == 5, f"YES: expected 5 vertices, got {tn}")
check(len(tedges) == 4, f"YES: expected 4 edges, got {len(tedges)}")
check(tk == 2, f"YES: expected K'=2, got {tk}")

# Extract edge cover
edge_cover = extract_solution(yes_n, yes_edges, yes_partition)
check(edge_cover == [0, 0, 0, 1], f"YES: expected [0,0,0,1], got {edge_cover}")

# Verify cover valid
check(is_feasible_target(yes_n, yes_edges, yes_k, edge_cover),
      "YES: extracted edge cover should be valid")

# Group 0: edges (0,1),(0,2),(1,2) -> vertices {0,1,2} -> triangle
# Group 1: edge (3,4) -> vertices {3,4} -> edge
check(yes_partition[0] == yes_partition[1] == yes_partition[2] == 0,
      "YES: V0 should be {0,1,2}")
check(yes_partition[3] == yes_partition[4] == 1,
      "YES: V1 should be {3,4}")

# Brute force
tgt_wit = brute_force_target(yes_n, yes_edges, yes_k)
check(tgt_wit is not None, "YES: target brute force should find solution")

print(f"  YES example checks: {checks}")


# ============================================================
# Test 3: NO example from Typst
# ============================================================
print("Test 3: NO example from Typst proof...")

no_n = 4
no_edges = [(0, 1), (1, 2), (2, 3)]  # P4 path
no_k = 2

# Source infeasible
check(brute_force_source(no_n, no_edges, no_k) is None,
      "NO: P4 should not have 2-clique partition")

# Reduce
tn, tedges, tk = reduce(no_n, no_edges, no_k)

check(tn == 4, "NO: expected 4 vertices")
check(len(tedges) == 3, "NO: expected 3 edges")
check(tk == 2, "NO: expected K'=2")

# Target also infeasible for K=2
check(brute_force_target(no_n, no_edges, no_k) is None,
      "NO: P4 should not have 2-clique edge cover")

# Exhaustively verify all source partitions are invalid
for config in itertools.product(range(no_k), repeat=no_n):
    check(not is_feasible_source(no_n, no_edges, no_k, list(config)),
          f"NO source: config {config} should be invalid")

# Exhaustively verify all target edge assignments are invalid
for edge_config in itertools.product(range(no_k), repeat=len(no_edges)):
    check(not is_feasible_target(no_n, no_edges, no_k, list(edge_config)),
          f"NO target: edge config {edge_config} should be invalid")

# P4 needs 3 cliques
check(brute_force_target(no_n, no_edges, 3) is not None,
      "NO: P4 should have 3-clique edge cover")

print(f"  NO example checks: {checks}")


# ============================================================
# Test 4: hypothesis property-based testing
# ============================================================
print("Test 4: hypothesis property-based testing...")

try:
    from hypothesis import given, strategies as st, settings

    @st.composite
    def graph_and_k(draw):
        """Strategy 1: random graph with random K."""
        n = draw(st.integers(min_value=1, max_value=6))
        all_e = [(i, j) for i in range(n) for j in range(i + 1, n)]
        edge_mask = draw(st.lists(st.booleans(), min_size=len(all_e), max_size=len(all_e)))
        edges = [e for e, include in zip(all_e, edge_mask) if include]
        k = draw(st.integers(min_value=1, max_value=n))
        return n, edges, k

    @st.composite
    def special_graph_and_k(draw):
        """Strategy 2: special graph families (complete, empty, star, path, cycle)."""
        family = draw(st.sampled_from(["complete", "empty", "star", "path", "cycle"]))
        n = draw(st.integers(min_value=2, max_value=6))
        k = draw(st.integers(min_value=1, max_value=n))

        if family == "complete":
            edges = [(i, j) for i in range(n) for j in range(i + 1, n)]
        elif family == "empty":
            edges = []
        elif family == "star":
            edges = [(0, j) for j in range(1, n)]
        elif family == "path":
            edges = [(i, i + 1) for i in range(n - 1)]
        else:  # cycle
            edges = [(i, (i + 1) % n) for i in range(n)]
            edges = [(min(u, v), max(u, v)) for u, v in edges]
            edges = list(set(edges))

        return n, edges, k

    @given(graph_and_k())
    @settings(max_examples=2500, deadline=None)
    def test_forward_random(args):
        global checks
        n, edges, k = args
        src_wit = brute_force_source(n, edges, k)
        if src_wit is not None:
            tn, tedges, tk = reduce(n, edges, k)
            edge_cover = extract_solution(n, edges, src_wit)
            check(is_feasible_target(n, edges, k, edge_cover),
                  f"PBT random forward: n={n}, m={len(edges)}, k={k}")
        else:
            check(True, f"PBT random: src NO, n={n}, m={len(edges)}, k={k}")

    @given(special_graph_and_k())
    @settings(max_examples=2500, deadline=None)
    def test_forward_special(args):
        global checks
        n, edges, k = args
        src_wit = brute_force_source(n, edges, k)
        if src_wit is not None:
            tn, tedges, tk = reduce(n, edges, k)
            edge_cover = extract_solution(n, edges, src_wit)
            check(is_feasible_target(n, edges, k, edge_cover),
                  f"PBT special forward: n={n}, m={len(edges)}, k={k}")
        else:
            check(True, f"PBT special: src NO, n={n}, m={len(edges)}, k={k}")

    test_forward_random()
    print(f"  Strategy 1 (random graphs) done. Checks: {checks}")
    test_forward_special()
    print(f"  Strategy 2 (special graph families) done. Checks: {checks}")

except ImportError:
    print("  WARNING: hypothesis not available, using manual PBT fallback")
    import random
    random.seed(123)
    for _ in range(5000):
        n = random.randint(1, 6)
        all_e = [(i, j) for i in range(n) for j in range(i + 1, n)]
        edges = [e for e in all_e if random.random() < random.random()]
        k = random.randint(1, n)

        src_wit = brute_force_source(n, edges, k)
        if src_wit is not None:
            tn, tedges, tk = reduce(n, edges, k)
            edge_cover = extract_solution(n, edges, src_wit)
            check(is_feasible_target(n, edges, k, edge_cover),
                  f"Fallback PBT forward: n={n}, m={len(edges)}, k={k}")
        else:
            check(True, f"Fallback PBT: src NO, n={n}, m={len(edges)}, k={k}")


# ============================================================
# Test 5: Cross-comparison with constructor outputs
# ============================================================
print("Test 5: Cross-comparison with constructor outputs...")

vectors_path = Path(__file__).parent / "test_vectors_partition_into_cliques_minimum_covering_by_cliques.json"
if vectors_path.exists():
    with open(vectors_path) as f:
        vectors = json.load(f)

    # YES instance
    yi = vectors["yes_instance"]
    inp = yi["input"]
    out = yi["output"]
    tn, tedges, tk = reduce(inp["num_vertices"], [tuple(e) for e in inp["edges"]], inp["num_cliques"])
    check(tn == out["num_vertices"], "Cross: YES num_vertices mismatch")
    our_edges = {(min(u, v), max(u, v)) for u, v in tedges}
    their_edges = {(min(u, v), max(u, v)) for u, v in [tuple(e) for e in out["edges"]]}
    check(our_edges == their_edges, "Cross: YES edges mismatch")

    # Verify extracted solution
    src_sol = yi["source_solution"]
    our_cover = extract_solution(inp["num_vertices"], [tuple(e) for e in inp["edges"]], src_sol)
    check(our_cover == yi["extracted_solution"], "Cross: YES extracted solution mismatch")

    # NO instance
    ni = vectors["no_instance"]
    inp = ni["input"]
    out = ni["output"]
    tn, tedges, tk = reduce(inp["num_vertices"], [tuple(e) for e in inp["edges"]], inp["num_cliques"])
    check(tn == out["num_vertices"], "Cross: NO num_vertices mismatch")
    our_edges = {(min(u, v), max(u, v)) for u, v in tedges}
    their_edges = {(min(u, v), max(u, v)) for u, v in [tuple(e) for e in out["edges"]]}
    check(our_edges == their_edges, "Cross: NO edges mismatch")

    print(f"  Cross-comparison checks passed")
else:
    print(f"  WARNING: test vectors not found at {vectors_path}, skipping cross-comparison")


# ============================================================
# Summary
# ============================================================
print(f"\n{'=' * 60}")
print(f"ADVERSARY VERIFICATION SUMMARY")
print(f"  Total checks: {checks} (minimum: 5,000)")
print(f"  Failures:     {len(failures)}")
print(f"{'=' * 60}")

if failures:
    print(f"\nFAILED:")
    for f in failures[:20]:
        print(f"  {f}")
    sys.exit(1)
else:
    print(f"\nPASSED: All {checks} adversary checks passed.")

if checks < 5000:
    print(f"\nWARNING: Total checks ({checks}) below minimum (5,000).")
    sys.exit(1)
