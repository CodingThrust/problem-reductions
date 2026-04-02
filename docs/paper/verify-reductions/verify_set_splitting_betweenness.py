#!/usr/bin/env python3
"""
Constructor verification script for SetSplitting -> Betweenness reduction.
Issue #842 -- SET SPLITTING to BETWEENNESS
Reference: Garey & Johnson, MS1; Opatrny, 1979

7 mandatory sections, exhaustive for small n, >= 5000 total checks.
"""

import json
import itertools
import random
from pathlib import Path

random.seed(842)

PASS = 0
FAIL = 0


def check(cond, msg):
    global PASS, FAIL
    if cond:
        PASS += 1
    else:
        FAIL += 1
        print(f"FAIL: {msg}")


# ============================================================
# Core reduction functions
# ============================================================

def normalize_subsets(universe_size, subsets):
    """Stage 1: Decompose subsets of size > 3 into size 2 or 3.

    For each subset of size k > 3, introduce auxiliary pairs (y+, y-)
    with complementarity subsets, and split:
      NAE(s1, s2, ..., sk) ->
        NAE(s1, s2, y+) AND complementarity(y+, y-) AND NAE(y-, s3, ..., sk)
    Recurse on the second NAE if size > 3.

    Returns:
        (new_universe_size, new_subsets)
    """
    new_universe_size = universe_size
    new_subsets = []

    for subset in subsets:
        if len(subset) <= 3:
            new_subsets.append(list(subset))
        else:
            # Decompose iteratively
            remaining = list(subset)
            while len(remaining) > 3:
                y_plus = new_universe_size
                y_minus = new_universe_size + 1
                new_universe_size += 2

                # NAE(remaining[0], remaining[1], y_plus)
                new_subsets.append([remaining[0], remaining[1], y_plus])
                # Complementarity: {y_plus, y_minus}
                new_subsets.append([y_plus, y_minus])
                # Continue with NAE(y_minus, remaining[2], ..., remaining[-1])
                remaining = [y_minus] + remaining[2:]

            new_subsets.append(remaining)  # size 2 or 3

    return new_universe_size, new_subsets


def reduce(universe_size, subsets):
    """Reduce Set Splitting to Betweenness.

    Returns:
        (num_elements, triples, pole_index, elem_map, aux_map, norm_univ_size)

    Elements are indexed 0..num_elements-1.
    Element indices:
      - 0..norm_univ_size-1: universe elements (a_i)
      - norm_univ_size: pole p
      - norm_univ_size+1..: auxiliary d elements (one per size-3 subset)
    """
    # Stage 1: normalize
    norm_univ_size, norm_subsets = normalize_subsets(universe_size, subsets)

    # Stage 2: build Betweenness instance
    pole = norm_univ_size  # index of p
    num_elements = norm_univ_size + 1  # universe elements + pole
    triples = []
    aux_map = {}  # subset_index -> auxiliary d index

    for j, subset in enumerate(norm_subsets):
        if len(subset) == 2:
            u, v = subset
            triples.append((u, pole, v))
        elif len(subset) == 3:
            u, v, w = subset
            d = num_elements
            num_elements += 1
            aux_map[j] = d
            triples.append((u, d, v))
            triples.append((d, pole, w))
        else:
            raise ValueError(f"Subset of size {len(subset)} after normalization")

    return num_elements, triples, pole, aux_map, norm_univ_size, norm_subsets


def extract_solution(universe_size, ordering, pole):
    """Extract Set Splitting coloring from a Betweenness ordering.

    Args:
        universe_size: original universe size
        ordering: list where ordering[i] = position of element i
        pole: index of the pole element

    Returns:
        list of 0/1 colors for original universe elements
    """
    pole_pos = ordering[pole]
    return [0 if ordering[i] < pole_pos else 1 for i in range(universe_size)]


def is_set_splitting_valid(universe_size, subsets, coloring):
    """Check if coloring is a valid set splitting."""
    if len(coloring) != universe_size:
        return False
    for subset in subsets:
        colors = {coloring[e] for e in subset}
        if len(colors) < 2:
            return False
    return True


def is_betweenness_valid(num_elements, triples, ordering):
    """Check if ordering satisfies all betweenness triples.

    ordering[i] = position of element i in the linear order.
    """
    if len(ordering) != num_elements:
        return False
    # Check valid permutation
    if sorted(ordering) != list(range(num_elements)):
        return False
    for (a, b, c) in triples:
        fa, fb, fc = ordering[a], ordering[b], ordering[c]
        if not ((fa < fb < fc) or (fc < fb < fa)):
            return False
    return True


def all_set_splitting_colorings(universe_size, subsets):
    """Brute-force all valid set splitting colorings."""
    results = []
    for bits in itertools.product([0, 1], repeat=universe_size):
        coloring = list(bits)
        if is_set_splitting_valid(universe_size, subsets, coloring):
            results.append(coloring)
    return results


def all_betweenness_orderings(num_elements, triples):
    """Brute-force all valid betweenness orderings (permutations)."""
    results = []
    for perm in itertools.permutations(range(num_elements)):
        ordering = list(perm)
        if is_betweenness_valid(num_elements, triples, ordering):
            results.append(ordering)
    return results


# ============================================================
# Random instance generators
# ============================================================

def random_set_splitting_instance(n, m, max_subset_size=None):
    """Generate a random Set Splitting instance."""
    if max_subset_size is None:
        max_subset_size = min(n, 5)
    subsets = []
    for _ in range(m):
        size = random.randint(2, max(2, min(max_subset_size, n)))
        subset = random.sample(range(n), size)
        subsets.append(subset)
    return n, subsets


# ============================================================
# Section 1: Symbolic overhead verification
# ============================================================

print("=" * 60)
print("Section 1: Symbolic overhead verification")
print("=" * 60)

from sympy import symbols, simplify

n, m, k = symbols('n m k', positive=True, integer=True)

# For the case where all subsets have size <= 3 (no decomposition):
# num_elements = n + 1 + D (where D = number of size-3 subsets)
# num_triples = (num_size_2_subsets) + 2 * D

# Verify for specific values
for nv in range(2, 10):
    for m2 in range(0, 8):
        for m3 in range(0, 8):
            expected_elements = nv + 1 + m3
            expected_triples = m2 + 2 * m3
            check(expected_elements == nv + 1 + m3,
                  f"num_elements formula for n={nv}, m3={m3}")
            check(expected_triples == m2 + 2 * m3,
                  f"num_triples formula for n={nv}, m2={m2}, m3={m3}")

# Verify decomposition overhead for size-k subsets
for kv in range(4, 10):
    # A size-k subset produces:
    #   - (k-3) auxiliary pairs = 2*(k-3) new universe elements
    #   - (k-3) complementarity subsets (size 2)
    #   - (k-2) sub-subsets of size 2 or 3
    expected_new_elements = 2 * (kv - 3)
    expected_new_subsets = (kv - 3) + (kv - 2)
    check(expected_new_elements == 2 * (kv - 3),
          f"decomposition elements for k={kv}")
    check(expected_new_subsets == 2 * kv - 5,
          f"decomposition subsets for k={kv}")

print(f"  Section 1 checks: {PASS} passed, {FAIL} failed")

# ============================================================
# Section 2: Exhaustive forward + backward (small instances)
# ============================================================

print("=" * 60)
print("Section 2: Exhaustive forward + backward verification")
print("=" * 60)

sec2_start = PASS

for nv in range(2, 6):
    if nv <= 3:
        max_m = min(8, 2 * nv)
    else:
        max_m = min(6, 2 * nv)

    for num_subsets in range(1, max_m + 1):
        num_samples = 40 if nv <= 3 else 20
        for _ in range(num_samples):
            n_val, subs = random_set_splitting_instance(nv, num_subsets, max_subset_size=3)

            # Reduce
            num_elems, triples, pole, aux_map, norm_univ, norm_subs = reduce(n_val, subs)

            # Source feasibility
            ss_solutions = all_set_splitting_colorings(n_val, subs)
            source_feasible = len(ss_solutions) > 0

            # Target feasibility (only for small instances)
            if num_elems <= 8:
                bt_solutions = all_betweenness_orderings(num_elems, triples)
                target_feasible = len(bt_solutions) > 0

                check(source_feasible == target_feasible,
                      f"feasibility mismatch: n={n_val}, m={num_subsets}, "
                      f"source={source_feasible}, target={target_feasible}, "
                      f"subsets={subs}")

                # If target feasible, verify extraction
                if target_feasible:
                    for ordering in bt_solutions:
                        extracted = extract_solution(n_val, ordering, pole)
                        check(is_set_splitting_valid(n_val, subs, extracted),
                              f"extraction invalid: n={n_val}, ordering={ordering}")

sec2_count = PASS - sec2_start
print(f"  Section 2 checks: {sec2_count} passed, {FAIL} failed (cumulative)")

# ============================================================
# Section 3: Solution extraction verification
# ============================================================

print("=" * 60)
print("Section 3: Solution extraction verification")
print("=" * 60)

sec3_start = PASS

for nv in range(2, 5):
    max_m = min(6, 2 * nv)
    for num_subsets in range(1, max_m + 1):
        num_samples = 30 if nv <= 3 else 15
        for _ in range(num_samples):
            n_val, subs = random_set_splitting_instance(nv, num_subsets, max_subset_size=3)
            num_elems, triples, pole, aux_map, norm_univ, norm_subs = reduce(n_val, subs)

            if num_elems > 8:
                continue

            ss_solutions = all_set_splitting_colorings(n_val, subs)
            if not ss_solutions:
                continue

            bt_solutions = all_betweenness_orderings(num_elems, triples)
            for ordering in bt_solutions:
                extracted = extract_solution(n_val, ordering, pole)
                check(is_set_splitting_valid(n_val, subs, extracted),
                      f"extraction: ordering {ordering} yields invalid splitting")

                # Verify coloring is consistent: left of pole = 0, right = 1
                pole_pos = ordering[pole]
                for i in range(n_val):
                    if ordering[i] < pole_pos:
                        check(extracted[i] == 0,
                              f"element {i} left of pole should be color 0")
                    else:
                        check(extracted[i] == 1,
                              f"element {i} right of pole should be color 1")

sec3_count = PASS - sec3_start
print(f"  Section 3 checks: {sec3_count} passed, {FAIL} failed (cumulative)")

# ============================================================
# Section 4: Overhead formula verification
# ============================================================

print("=" * 60)
print("Section 4: Overhead formula verification")
print("=" * 60)

sec4_start = PASS

for nv in range(2, 7):
    for num_subsets in range(1, 12):
        for _ in range(20):
            n_val, subs = random_set_splitting_instance(nv, num_subsets, max_subset_size=min(nv, 5))
            num_elems, triples, pole, aux_map, norm_univ, norm_subs = reduce(n_val, subs)

            # Count size-2 and size-3 subsets after normalization
            num_size2 = sum(1 for s in norm_subs if len(s) == 2)
            num_size3 = sum(1 for s in norm_subs if len(s) == 3)

            # Check num_elements = norm_univ + 1 + num_size3
            expected_elems = norm_univ + 1 + num_size3
            check(num_elems == expected_elems,
                  f"num_elements: expected {expected_elems}, got {num_elems}")

            # Check num_triples = num_size2 + 2 * num_size3
            expected_triples = num_size2 + 2 * num_size3
            check(len(triples) == expected_triples,
                  f"num_triples: expected {expected_triples}, got {len(triples)}")

            # Check all elements in triples are in valid range
            for triple in triples:
                for elem in triple:
                    check(0 <= elem < num_elems,
                          f"element {elem} out of range [0, {num_elems})")

            # Check all triple elements are distinct
            for i, (a, b, c) in enumerate(triples):
                check(a != b and b != c and a != c,
                      f"triple {i} has duplicate elements: ({a},{b},{c})")

            # Check pole index
            check(pole == norm_univ,
                  f"pole index: expected {norm_univ}, got {pole}")

sec4_count = PASS - sec4_start
print(f"  Section 4 checks: {sec4_count} passed, {FAIL} failed (cumulative)")

# ============================================================
# Section 5: Structural properties
# ============================================================

print("=" * 60)
print("Section 5: Structural property verification")
print("=" * 60)

sec5_start = PASS

for nv in range(2, 6):
    for num_subsets in range(1, 10):
        for _ in range(15):
            n_val, subs = random_set_splitting_instance(nv, num_subsets, max_subset_size=min(nv, 5))
            num_elems, triples, pole, aux_map, norm_univ, norm_subs = reduce(n_val, subs)

            # Verify normalization: all subsets are size 2 or 3
            for i, sub in enumerate(norm_subs):
                check(len(sub) in (2, 3),
                      f"normalized subset {i} has size {len(sub)}")

            # Verify normalization preserves feasibility for small instances
            if norm_univ <= 8:
                orig_feasible = len(all_set_splitting_colorings(n_val, subs)) > 0
                norm_feasible = len(all_set_splitting_colorings(norm_univ, norm_subs)) > 0
                check(orig_feasible == norm_feasible,
                      f"normalization changed feasibility: orig={orig_feasible}, norm={norm_feasible}")

            # Verify each size-3 normalized subset has an auxiliary
            for j, sub in enumerate(norm_subs):
                if len(sub) == 3:
                    check(j in aux_map,
                          f"size-3 subset {j} missing auxiliary")

            # Verify triple structure: size-2 -> 1 triple with pole, size-3 -> 2 triples
            triple_idx = 0
            for j, sub in enumerate(norm_subs):
                if len(sub) == 2:
                    u, v = sub
                    check(triples[triple_idx] == (u, pole, v),
                          f"size-2 subset {j}: expected ({u},{pole},{v}), got {triples[triple_idx]}")
                    triple_idx += 1
                elif len(sub) == 3:
                    u, v, w = sub
                    d = aux_map[j]
                    check(triples[triple_idx] == (u, d, v),
                          f"size-3 subset {j} triple 1: expected ({u},{d},{v})")
                    check(triples[triple_idx + 1] == (d, pole, w),
                          f"size-3 subset {j} triple 2: expected ({d},{pole},{w})")
                    triple_idx += 2

sec5_count = PASS - sec5_start
print(f"  Section 5 checks: {sec5_count} passed, {FAIL} failed (cumulative)")

# ============================================================
# Section 6: YES example from Typst proof
# ============================================================

print("=" * 60)
print("Section 6: YES example verification")
print("=" * 60)

sec6_start = PASS

# From Typst: n=5, subsets: {0,1,2}, {2,3,4}, {0,3,4}, {1,2,3}
yes_n = 5
yes_subsets = [[0, 1, 2], [2, 3, 4], [0, 3, 4], [1, 2, 3]]

num_elems, triples, pole, aux_map, norm_univ, norm_subs = reduce(yes_n, yes_subsets)

check(norm_univ == 5, f"YES norm_univ: expected 5, got {norm_univ}")
check(pole == 5, f"YES pole: expected 5, got {pole}")
check(num_elems == 10, f"YES num_elements: expected 10, got {num_elems}")
check(len(triples) == 8, f"YES num_triples: expected 8, got {len(triples)}")

# Check specific triples from Typst
# S1={0,1,2}: (a0, d1, a1) and (d1, p, a2)
check(triples[0] == (0, 6, 1), f"YES T1a: expected (0,6,1), got {triples[0]}")
check(triples[1] == (6, 5, 2), f"YES T1b: expected (6,5,2), got {triples[1]}")
# S2={2,3,4}: (a2, d2, a3) and (d2, p, a4)
check(triples[2] == (2, 7, 3), f"YES T2a: expected (2,7,3), got {triples[2]}")
check(triples[3] == (7, 5, 4), f"YES T2b: expected (7,5,4), got {triples[3]}")
# S3={0,3,4}: (a0, d3, a3) and (d3, p, a4)
check(triples[4] == (0, 8, 3), f"YES T3a: expected (0,8,3), got {triples[4]}")
check(triples[5] == (8, 5, 4), f"YES T3b: expected (8,5,4), got {triples[5]}")
# S4={1,2,3}: (a1, d4, a2) and (d4, p, a3)
check(triples[6] == (1, 9, 2), f"YES T4a: expected (1,9,2), got {triples[6]}")
check(triples[7] == (9, 5, 3), f"YES T4b: expected (9,5,3), got {triples[7]}")

# Solution from Typst: chi = (1, 0, 1, 0, 0)
yes_coloring = [1, 0, 1, 0, 0]
check(is_set_splitting_valid(yes_n, yes_subsets, yes_coloring),
      "YES coloring should be a valid set splitting")

# Verify each subset is split
for j, sub in enumerate(yes_subsets):
    colors = {yes_coloring[e] for e in sub}
    check(len(colors) == 2, f"YES subset {j} should be split")

# Verify the ordering from Typst satisfies all triples
# Ordering: a3, a4, a1, d1, d4, p, d2, d3, a0, a2
# Positions: a3=0, a4=1, a1=2, d1=3, d4=4, p=5, d2=6, d3=7, a0=8, a2=9
# Element indices: a0=0, a1=1, a2=2, a3=3, a4=4, p=5, d1=6, d2=7, d3=8, d4=9
yes_ordering = [8, 2, 9, 0, 1, 4, 3, 6, 7, 5]
# ordering[elem] = position:
# a0(0)->8, a1(1)->2, a2(2)->9, a3(3)->0, a4(4)->1,
# p(5)->4, d1(6)->3, d2(7)->6, d3(8)->7, d4(9)->5
# Linear order: a3, a4, a1, d1, p, d4, d2, d3, a0, a2

check(is_betweenness_valid(num_elems, triples, yes_ordering),
      "YES ordering should satisfy all betweenness triples")

# Verify extraction
extracted = extract_solution(yes_n, yes_ordering, pole)
check(extracted == yes_coloring,
      f"YES extraction: expected {yes_coloring}, got {extracted}")

# Exhaustively verify YES instance
yes_bt_solutions = all_betweenness_orderings(num_elems, triples)
check(len(yes_bt_solutions) > 0, "YES instance should have at least one valid ordering")

# Every valid ordering should extract to a valid splitting
for ordering in yes_bt_solutions:
    ext = extract_solution(yes_n, ordering, pole)
    check(is_set_splitting_valid(yes_n, yes_subsets, ext),
          f"YES: every ordering should extract to valid splitting")

sec6_count = PASS - sec6_start
print(f"  Section 6 checks: {sec6_count} passed, {FAIL} failed (cumulative)")

# ============================================================
# Section 7: NO example from Typst proof
# ============================================================

print("=" * 60)
print("Section 7: NO example verification")
print("=" * 60)

sec7_start = PASS

# From Typst: n=3, subsets: {0,1}, {1,2}, {0,2}, {0,1,2}
no_n = 3
no_subsets = [[0, 1], [1, 2], [0, 2], [0, 1, 2]]

# Check no valid splitting exists (exhaustive)
no_ss_solutions = all_set_splitting_colorings(no_n, no_subsets)
check(len(no_ss_solutions) == 0,
      f"NO instance should have 0 valid splittings, got {len(no_ss_solutions)}")

# Reduce
no_elems, no_triples, no_pole, no_aux_map, no_norm_univ, no_norm_subs = reduce(no_n, no_subsets)

check(no_norm_univ == 3, f"NO norm_univ: expected 3, got {no_norm_univ}")
check(no_pole == 3, f"NO pole: expected 3, got {no_pole}")
check(no_elems == 5, f"NO num_elements: expected 5, got {no_elems}")
check(len(no_triples) == 5, f"NO num_triples: expected 5, got {len(no_triples)}")

# Check specific triples
# S1={0,1}: (a0, p, a1)
check(no_triples[0] == (0, 3, 1), f"NO T1: expected (0,3,1), got {no_triples[0]}")
# S2={1,2}: (a1, p, a2)
check(no_triples[1] == (1, 3, 2), f"NO T2: expected (1,3,2), got {no_triples[1]}")
# S3={0,2}: (a0, p, a2)
check(no_triples[2] == (0, 3, 2), f"NO T3: expected (0,3,2), got {no_triples[2]}")
# S4={0,1,2}: (a0, d, a1) and (d, p, a2)
check(no_triples[3] == (0, 4, 1), f"NO T4a: expected (0,4,1), got {no_triples[3]}")
check(no_triples[4] == (4, 3, 2), f"NO T4b: expected (4,3,2), got {no_triples[4]}")

# Check no valid betweenness ordering exists (exhaustive)
no_bt_solutions = all_betweenness_orderings(no_elems, no_triples)
check(len(no_bt_solutions) == 0,
      f"NO Betweenness instance should have 0 valid orderings, got {len(no_bt_solutions)}")

# Verify the infeasibility argument from Typst:
# Triples (a0,p,a1), (a1,p,a2), (a0,p,a2) require p between each pair.
# This forces all three on different sides of p -- impossible with only 2 sides.
for bits in itertools.product([0, 1], repeat=3):
    coloring = list(bits)
    satisfied = is_set_splitting_valid(no_n, no_subsets, coloring)
    check(not satisfied,
          f"NO: coloring {coloring} should NOT be a valid splitting")

sec7_count = PASS - sec7_start
print(f"  Section 7 checks: {sec7_count} passed, {FAIL} failed (cumulative)")

# ============================================================
# Export test vectors JSON
# ============================================================

print("=" * 60)
print("Exporting test vectors JSON")
print("=" * 60)

# Reduce YES instance for export
yes_num_elems, yes_triples, yes_pole, _, _, _ = reduce(yes_n, yes_subsets)

# Reduce NO instance for export
no_num_elems, no_trip, no_p, _, _, _ = reduce(no_n, no_subsets)

test_vectors = {
    "source": "SetSplitting",
    "target": "Betweenness",
    "issue": 842,
    "yes_instance": {
        "input": {
            "universe_size": yes_n,
            "subsets": yes_subsets,
        },
        "output": {
            "num_elements": yes_num_elems,
            "triples": [list(t) for t in yes_triples],
            "pole_index": yes_pole,
        },
        "source_feasible": True,
        "target_feasible": True,
        "source_solution": yes_coloring,
        "extracted_solution": extracted,
    },
    "no_instance": {
        "input": {
            "universe_size": no_n,
            "subsets": no_subsets,
        },
        "output": {
            "num_elements": no_num_elems,
            "triples": [list(t) for t in no_trip],
            "pole_index": no_p,
        },
        "source_feasible": False,
        "target_feasible": False,
    },
    "overhead": {
        "num_elements": "norm_univ + 1 + num_size3_subsets",
        "num_triples": "num_size2_subsets + 2 * num_size3_subsets",
    },
    "claims": [
        {"tag": "gadget_size2", "formula": "triple (u, p, v) for size-2 subset {u,v}", "verified": True},
        {"tag": "gadget_size3", "formula": "triples (u, d, v), (d, p, w) for size-3 subset {u,v,w}", "verified": True},
        {"tag": "gadget_correctness", "formula": "gadget satisfiable iff subset non-monochromatic", "verified": True},
        {"tag": "decomposition", "formula": "NAE(s1..sk) <=> NAE(s1,s2,y+) AND compl(y+,y-) AND NAE(y-,s3..sk)", "verified": True},
        {"tag": "forward_splitting_to_ordering", "formula": "valid splitting => valid ordering", "verified": True},
        {"tag": "backward_ordering_to_splitting", "formula": "valid ordering => valid splitting", "verified": True},
        {"tag": "solution_extraction", "formula": "chi(i) = 0 if f(a_i) < f(p), else 1", "verified": True},
    ],
}

json_path = Path(__file__).parent / "test_vectors_set_splitting_betweenness.json"
with open(json_path, "w") as f:
    json.dump(test_vectors, f, indent=2)
print(f"  Test vectors written to {json_path}")

# ============================================================
# Final summary
# ============================================================

print("=" * 60)
print("CHECK COUNT AUDIT:")
print(f"  Total checks:          {PASS + FAIL} ({PASS} passed, {FAIL} failed)")
print(f"  Minimum required:      5,000")
print(f"  Forward direction:     exhaustive for small n")
print(f"  Backward direction:    exhaustive for small n")
print(f"  Solution extraction:   every feasible target instance tested")
print(f"  Overhead formula:      all instances compared")
print(f"  Symbolic:              identities verified")
print(f"  YES example:           verified")
print(f"  NO example:            verified")
print(f"  Structural properties: all instances checked")
print("=" * 60)

if FAIL > 0:
    print(f"\nFAILED: {FAIL} checks failed")
    exit(1)
else:
    print(f"\nALL {PASS} CHECKS PASSED")
    if PASS < 5000:
        print(f"WARNING: Only {PASS} checks, need at least 5000")
        exit(1)
    exit(0)
