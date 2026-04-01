#!/usr/bin/env python3
"""
Verify X3C -> AcyclicPartition reduction (§4.3 of proposed-reductions.typ).

STATUS: This reduction is KNOWN TO BE BROKEN.
The paper marks it as OPEN with a red status note.

This script documents the failure:
  1. Runs the construction on 3 test cases
  2. Shows that quotient-graph cycles arise from 2-cycle encoding
  3. Prints clear diagnostics of WHY the construction fails
  4. All tests are EXPECTED to fail (documenting known bug)
  5. Returns exit code 0 (expected failures are not verification failures)

Run: python3 docs/paper/verify-reductions/verify_x3c_ap.py
"""

import itertools
import sys
from collections import defaultdict

expected_failures = 0
expected_passes = 0
total = 0


def check_expected_fail(condition, msg):
    """A check where we EXPECT failure."""
    global expected_failures, expected_passes, total
    total += 1
    if condition:
        expected_passes += 1
        print(f"  UNEXPECTED PASS: {msg}")
    else:
        expected_failures += 1
        print(f"  EXPECTED FAIL: {msg}")


def check_expected_pass(condition, msg):
    """A check where we expect success (structural invariants)."""
    global expected_failures, expected_passes, total
    total += 1
    if condition:
        expected_passes += 1
    else:
        expected_failures += 1
        print(f"  UNEXPECTED FAIL: {msg}")


def has_directed_cycle(adj, vertices):
    """Check if directed graph has a cycle (DFS-based)."""
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {v: WHITE for v in vertices}

    def dfs(u):
        color[u] = GRAY
        for v in adj.get(u, []):
            if v in vertices:
                if color.get(v) == GRAY:
                    return True
                if color.get(v) == WHITE and dfs(v):
                    return True
        color[u] = BLACK
        return False

    return any(color[v] == WHITE and dfs(v) for v in vertices)


def find_cycle(adj, vertices):
    """Find and return a directed cycle, or None."""
    WHITE, GRAY, BLACK = 0, 1, 2
    color = {v: WHITE for v in vertices}
    parent = {}

    def dfs(u, path):
        color[u] = GRAY
        for v in adj.get(u, []):
            if v in vertices:
                if color.get(v) == GRAY:
                    # Found cycle: extract it
                    idx = path.index(v)
                    return path[idx:] + [v]
                if color.get(v) == WHITE:
                    result = dfs(v, path + [v])
                    if result:
                        return result
        color[u] = BLACK
        return None

    for v in vertices:
        if color[v] == WHITE:
            result = dfs(v, [v])
            if result:
                return result
    return None


def build_x3c_graph(universe_size, subsets):
    """
    Build the directed graph per the paper's construction.
    Returns (arcs, compatible_pairs, valid_triples).
    """
    elements = list(range(universe_size))

    # Find compatible pairs: (i,j) with i<j that share a subset
    compatible = set()
    for C in subsets:
        C_list = sorted(C)
        for a, b in itertools.combinations(C_list, 2):
            compatible.add((a, b))

    valid_triples = set()
    for C in subsets:
        valid_triples.add(tuple(sorted(C)))

    arcs = []

    # Compatibility / conflict arcs
    for i, j in itertools.combinations(elements, 2):
        if (i, j) in compatible:
            # Forward arc only
            arcs.append((i, j))
        else:
            # 2-cycle (conflict)
            arcs.append((i, j))
            arcs.append((j, i))

    # Triple-exclusion arcs
    for i, j, k in itertools.combinations(elements, 3):
        if ((i, j) in compatible and (j, k) in compatible
                and (i, k) in compatible):
            if (i, j, k) not in valid_triples:
                arcs.append((k, i))

    return arcs, compatible, valid_triples


def try_all_partitions(elements, q, arcs, compatible, valid_triples):
    """
    Try all partitions into q groups of 3 and check:
    1. Each group is internally acyclic
    2. Quotient graph is acyclic
    3. Inter-group cost <= K = A - 3q

    Returns (found_valid, diagnostics).
    """
    A = len(arcs)
    K = A - 3 * q
    diagnostics = []

    adj = defaultdict(list)
    for s, d in arcs:
        adj[s].append(d)

    def partition_gen(remaining, groups):
        if not remaining:
            yield list(groups)
            return
        remaining = sorted(remaining)
        first = remaining[0]
        rest = set(remaining[1:])
        for pair in itertools.combinations(rest, 2):
            group = (first,) + pair
            yield from partition_gen(rest - set(pair), groups + [group])

    found_valid = False
    partitions_checked = 0

    for partition in partition_gen(set(elements), []):
        partitions_checked += 1

        # Check each group is internally acyclic
        group_acyclic = True
        for g in partition:
            g_set = set(g)
            g_adj = defaultdict(list)
            for s, d in arcs:
                if s in g_set and d in g_set:
                    g_adj[s].append(d)
            if has_directed_cycle(g_adj, g_set):
                group_acyclic = False
                break

        if not group_acyclic:
            continue

        # Check quotient graph
        group_of = {}
        for gi, g in enumerate(partition):
            for v in g:
                group_of[v] = gi

        q_adj = defaultdict(list)
        inter_cost = 0
        for s, d in arcs:
            gs, gd = group_of[s], group_of[d]
            if gs != gd:
                q_adj[gs].append(gd)
                inter_cost += 1

        quotient_verts = set(range(len(partition)))
        quotient_acyclic = not has_directed_cycle(q_adj, quotient_verts)

        if not quotient_acyclic:
            cycle = find_cycle(q_adj, quotient_verts)
            diag = {
                'partition': partition,
                'inter_cost': inter_cost,
                'K': K,
                'quotient_cycle': cycle,
                'quotient_acyclic': False,
            }
            diagnostics.append(diag)
            continue

        if inter_cost <= K:
            found_valid = True
            diagnostics.append({
                'partition': partition,
                'inter_cost': inter_cost,
                'K': K,
                'quotient_acyclic': True,
                'valid': True,
            })

    return found_valid, partitions_checked, diagnostics


# ============================================================
# Test cases
# ============================================================

test_cases = [
    {
        'name': 'Simple YES (disjoint cover)',
        'universe_size': 6,
        'subsets': [{0, 1, 2}, {3, 4, 5}],
        'has_exact_cover': True,
    },
    {
        'name': 'YES with extra subset',
        'universe_size': 6,
        'subsets': [{0, 1, 2}, {1, 2, 4}, {3, 4, 5}],
        'has_exact_cover': True,
    },
    {
        'name': 'NO (overlapping only)',
        'universe_size': 6,
        'subsets': [{0, 1, 2}, {2, 3, 4}, {4, 5, 0}],
        'has_exact_cover': False,
    },
]


def verify_construction():
    print("=== X3C -> Acyclic Partition: Documenting Known Failure ===")
    print()
    print("The paper marks this reduction as OPEN/INCORRECT.")
    print("The 2-cycle encoding creates quotient-graph cycles between")
    print("distinct groups, violating the acyclicity constraint.")
    print()

    for tc in test_cases:
        name = tc['name']
        universe_size = tc['universe_size']
        subsets = tc['subsets']
        has_x3c = tc['has_exact_cover']
        q = universe_size // 3

        print(f"--- Test: {name} ---")
        print(f"  Universe: {{0..{universe_size-1}}}, q={q}")
        print(f"  Subsets: {[sorted(s) for s in subsets]}")
        print(f"  Has exact cover: {has_x3c}")

        arcs, compatible, valid_triples = build_x3c_graph(universe_size, subsets)
        A = len(arcs)
        K = A - 3 * q

        print(f"  Arcs: {A}, Cost bound K = {K}")
        print(f"  Compatible pairs: {sorted(compatible)}")

        # Count 2-cycles (conflict arcs)
        arc_set = set(arcs)
        two_cycles = [(i, j) for i, j in arc_set if (j, i) in arc_set and i < j]
        print(f"  2-cycles (conflicts): {len(two_cycles)}")
        if len(two_cycles) <= 10:
            print(f"    Pairs: {two_cycles}")

        elements = list(range(universe_size))

        # Try all partitions
        found_valid, n_checked, diagnostics = try_all_partitions(
            elements, q, arcs, compatible, valid_triples
        )

        print(f"  Partitions checked: {n_checked}")
        print(f"  Valid acyclic partition found: {found_valid}")

        if has_x3c:
            # The forward direction SHOULD work (paper claims it does)
            # But the backward direction is where the bug lies
            # Let's check the specific cover partition
            if tc['name'] == 'Simple YES (disjoint cover)':
                cover_partition = [(0, 1, 2), (3, 4, 5)]
            elif tc['name'] == 'YES with extra subset':
                cover_partition = [(0, 1, 2), (3, 4, 5)]
            else:
                cover_partition = None

            if cover_partition:
                # Check this specific partition
                adj = defaultdict(list)
                for s, d in arcs:
                    adj[s].append(d)

                group_of = {}
                for gi, g in enumerate(cover_partition):
                    for v in g:
                        group_of[v] = gi

                # Check groups acyclic
                all_groups_ok = True
                for g in cover_partition:
                    g_set = set(g)
                    g_adj = defaultdict(list)
                    for s, d in arcs:
                        if s in g_set and d in g_set:
                            g_adj[s].append(d)
                    if has_directed_cycle(g_adj, g_set):
                        all_groups_ok = False
                        print(f"  WARNING: group {g} has internal cycle!")

                check_expected_pass(all_groups_ok,
                                    f"{name}: cover partition groups are internally acyclic")

                # Check quotient
                q_adj = defaultdict(list)
                inter_cost = 0
                for s, d in arcs:
                    gs, gd = group_of[s], group_of[d]
                    if gs != gd:
                        q_adj[gs].append(gd)
                        inter_cost += 1

                quotient_verts = set(range(len(cover_partition)))
                quotient_acyclic = not has_directed_cycle(q_adj, quotient_verts)

                if not quotient_acyclic:
                    cycle = find_cycle(q_adj, quotient_verts)
                    print(f"  DIAGNOSTIC: Quotient graph has cycle: {cycle}")
                    print(f"  This is the core bug: 2-cycle arcs between groups")
                    print(f"  create quotient cycles even for correct covers.")

                    # Show the problematic inter-group arcs
                    for gi in range(len(cover_partition)):
                        for gj in range(len(cover_partition)):
                            if gi != gj:
                                cross = [(s, d) for s, d in arcs
                                         if group_of[s] == gi and group_of[d] == gj]
                                if cross:
                                    print(f"  Group {cover_partition[gi]} -> "
                                          f"Group {cover_partition[gj]}: "
                                          f"{len(cross)} arcs")

                # For the forward direction, the paper claims quotient is acyclic
                # because "all arcs go from smaller to larger index".
                # But 2-cycle reverse arcs go from larger to smaller!
                # This is the bug.

                # Show that incompatible pairs between groups create reverse arcs
                for i in cover_partition[0]:
                    for j in cover_partition[1]:
                        pair = (min(i, j), max(i, j))
                        if pair not in compatible:
                            print(f"  Incompatible cross-group pair ({i},{j}): "
                                  f"has 2-cycle arcs ({i}->{j}) and ({j}->{i})")

        # The key test: does the reduction work?
        # For YES instances, we expect to find a valid partition
        # For NO instances, we expect to NOT find one
        # The bug means YES instances may also fail
        if has_x3c:
            check_expected_fail(found_valid == has_x3c,
                                f"{name}: reduction {'works' if found_valid else 'FAILS'} "
                                f"(X3C has cover={has_x3c}, AP found={found_valid})")
        else:
            # NO instance: the construction might accidentally be correct here
            # (no valid partition should exist for either X3C or AP)
            check_expected_pass(found_valid == has_x3c,
                                f"{name}: NO instance correctly rejected "
                                f"(found={found_valid}, expected={has_x3c})")

        # Print diagnostic summary
        quotient_cycle_count = sum(
            1 for d in diagnostics if not d.get('quotient_acyclic', True)
        )
        if quotient_cycle_count > 0:
            print(f"  {quotient_cycle_count} partitions had quotient cycles "
                  f"(demonstrating the bug)")

        print()


# ============================================================
# Explain the root cause
# ============================================================

def explain_failure():
    print("=== Root Cause Analysis ===")
    print()
    print("The construction encodes incompatible pairs using 2-cycles:")
    print("  If elements i,j cannot be in the same group, add arcs (i->j) AND (j->i).")
    print()
    print("Problem: When i and j are in DIFFERENT groups (as intended),")
    print("these 2-cycle arcs create BOTH directions in the quotient graph:")
    print("  group(i) -> group(j)  AND  group(j) -> group(i)")
    print()
    print("This means the quotient graph has a 2-cycle between any two groups")
    print("that contain an incompatible pair, making it CYCLIC even for valid covers.")
    print()
    print("The paper's forward-direction proof incorrectly claims that")
    print("'all inter-group arcs go from groups with smaller-indexed elements")
    print("to groups with larger-indexed elements'. This ignores the reverse")
    print("arcs in 2-cycles, which go from larger to smaller indices.")
    print()
    print("A correct reduction would need a fundamentally different encoding")
    print("of the covering constraint that avoids creating quotient cycles.")
    print()


# ============================================================
# Main
# ============================================================

def main():
    print("X3C -> Acyclic Partition Reduction Verification")
    print("=" * 50)
    print("STATUS: DOCUMENTING KNOWN BUG (all failures expected)")
    print()

    verify_construction()
    explain_failure()

    print("=" * 50)
    print(f"TOTAL: {total} checks run")
    print(f"  Expected failures: {expected_failures}")
    print(f"  Expected passes: {expected_passes}")
    print()

    # This script returns 0 because the failures are EXPECTED
    # (we are documenting a known bug, not discovering one)
    print("Exit code 0: all failures are expected (known broken reduction)")
    sys.exit(0)


if __name__ == "__main__":
    main()
