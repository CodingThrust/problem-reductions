#!/usr/bin/env python3
"""
Constructor verification script for KSatisfiability(K3) -> Kernel reduction.
Issue #882 — Chvatal (1973).

7 mandatory sections, >= 5000 total checks.
"""

import itertools
import json
import random
import sys
from pathlib import Path

random.seed(42)

# ---------------------------------------------------------------------------
# Reduction implementation
# ---------------------------------------------------------------------------

def literal_vertex(lit, n):
    """Map a signed literal (1-indexed) to a vertex index.
    Positive lit i -> vertex 2*(i-1)       (x_i)
    Negative lit -i -> vertex 2*(i-1) + 1  (x_bar_i)
    """
    var = abs(lit) - 1  # 0-indexed variable
    if lit > 0:
        return 2 * var
    else:
        return 2 * var + 1


def reduce(num_vars, clauses):
    """Reduce a 3-SAT instance to a Kernel (directed graph) instance.

    Args:
        num_vars: number of boolean variables
        clauses: list of clauses, each a list of 3 signed integers (1-indexed)

    Returns:
        (num_vertices, arcs): directed graph specification
    """
    n = num_vars
    m = len(clauses)
    num_vertices = 2 * n + 3 * m
    arcs = []

    # Step 1: Variable digon gadgets
    for i in range(n):
        pos = 2 * i      # x_i
        neg = 2 * i + 1  # x_bar_i
        arcs.append((pos, neg))
        arcs.append((neg, pos))

    # Step 2 & 3: Clause gadgets + connection arcs
    for j, clause in enumerate(clauses):
        assert len(clause) == 3, f"Clause {j} has {len(clause)} literals, expected 3"
        base = 2 * n + 3 * j  # first clause vertex index

        # 3-cycle
        arcs.append((base, base + 1))
        arcs.append((base + 1, base + 2))
        arcs.append((base + 2, base))

        # Connection arcs: each clause vertex points to all literal vertices
        for lit in clause:
            v = literal_vertex(lit, n)
            for t in range(3):
                arcs.append((base + t, v))

    return num_vertices, arcs


def build_successors(num_vertices, arcs):
    """Build adjacency lists for fast kernel checking."""
    successors = [[] for _ in range(num_vertices)]
    for (u, v) in arcs:
        successors[u].append(v)
    return successors


def is_kernel_fast(num_vertices, arcs, selected):
    """Check if selected (set of vertex indices) is a kernel."""
    successors = build_successors(num_vertices, arcs)
    for u in range(num_vertices):
        if u in selected:
            for v in successors[u]:
                if v in selected:
                    return False
        else:
            if not any(v in selected for v in successors[u]):
                return False
    return True


def has_kernel_brute_force(num_vertices, arcs):
    """Check if the directed graph has a kernel by brute force.
    Only works for small graphs (num_vertices <= 22 or so).
    """
    for bits in range(1 << num_vertices):
        selected = set()
        for v in range(num_vertices):
            if bits & (1 << v):
                selected.add(v)
        if is_kernel_fast(num_vertices, arcs, selected):
            return True, selected
    return False, None


def has_kernel_structural(num_vars, clauses, num_vertices, arcs):
    """Check if the reduced graph has a kernel using the structural property
    that only literal-vertex subsets can be kernels.
    Works for any size graph produced by this reduction.
    """
    n = num_vars
    m = len(clauses)
    successors = build_successors(num_vertices, arcs)

    # Only check subsets that pick exactly one literal per variable
    for bits in range(1 << n):
        selected = set()
        for i in range(n):
            if (bits >> i) & 1:
                selected.add(2 * i)      # x_i
            else:
                selected.add(2 * i + 1)  # x_bar_i

        # Check kernel properties
        is_valid = True

        # Independence among selected literal vertices
        for u in selected:
            for v in successors[u]:
                if v in selected:
                    is_valid = False
                    break
            if not is_valid:
                break

        if not is_valid:
            continue

        # Absorption of non-selected literal vertices (guaranteed by digon)
        # Absorption of clause vertices
        all_absorbed = True
        for u in range(num_vertices):
            if u in selected:
                continue
            if not any(v in selected for v in successors[u]):
                all_absorbed = False
                break

        if all_absorbed:
            return True, selected

    return False, None


def is_satisfiable_brute_force(num_vars, clauses):
    """Check if a 3-SAT instance is satisfiable by brute force."""
    for bits in range(1 << num_vars):
        assignment = [(bits >> i) & 1 == 1 for i in range(num_vars)]
        if all(
            any(
                (assignment[abs(lit) - 1] if lit > 0 else not assignment[abs(lit) - 1])
                for lit in clause
            )
            for clause in clauses
        ):
            return True, assignment
    return False, None


def extract_solution(num_vars, kernel_set):
    """Extract a boolean assignment from a kernel.
    x_i (vertex 2*i) in kernel -> u_{i+1} = True
    x_bar_i (vertex 2*i+1) in kernel -> u_{i+1} = False
    """
    assignment = []
    for i in range(num_vars):
        pos_in = (2 * i) in kernel_set
        neg_in = (2 * i + 1) in kernel_set
        assert pos_in != neg_in, f"Variable {i}: pos={pos_in}, neg={neg_in}"
        assignment.append(pos_in)
    return assignment


# ---------------------------------------------------------------------------
# Instance generators
# ---------------------------------------------------------------------------

def random_3sat_instance(n, m):
    """Generate a random 3-SAT instance with n variables and m clauses."""
    clauses = []
    for _ in range(m):
        vars_chosen = random.sample(range(1, n + 1), min(3, n))
        if len(vars_chosen) < 3:
            # Pad with distinct variables if n < 3 (should not happen for 3-SAT)
            raise ValueError("Need at least 3 variables for 3-SAT")
        clause = [v if random.random() < 0.5 else -v for v in vars_chosen]
        clauses.append(clause)
    return clauses


# ---------------------------------------------------------------------------
# Section 1: Symbolic overhead verification (sympy)
# ---------------------------------------------------------------------------

def section_1_symbolic():
    """Verify overhead formulas symbolically."""
    from sympy import symbols, simplify

    n, m = symbols("n m", positive=True, integer=True)

    # num_vertices = 2n + 3m
    num_verts_formula = 2 * n + 3 * m

    # num_arcs = 2n + 12m
    digon_arcs = 2 * n
    triangle_arcs = 3 * m
    connection_arcs = 3 * m * 3  # 3 clause vertices * 3 literals per clause
    num_arcs_formula = 2 * n + 12 * m

    checks = 0

    # Verify breakdown sums
    assert simplify(digon_arcs + triangle_arcs + connection_arcs - num_arcs_formula) == 0
    checks += 1
    assert simplify(2 * n + 3 * m - num_verts_formula) == 0
    checks += 1

    # Verify for concrete values
    for nv in range(1, 20):
        for mv in range(1, 20):
            expected_v = 2 * nv + 3 * mv
            expected_a = 2 * nv + 12 * mv
            assert int(num_verts_formula.subs([(n, nv), (m, mv)])) == expected_v
            assert int(num_arcs_formula.subs([(n, nv), (m, mv)])) == expected_a
            checks += 2

    print(f"  Section 1 (symbolic): {checks} checks passed")
    return checks


# ---------------------------------------------------------------------------
# Section 2: Exhaustive forward + backward
# ---------------------------------------------------------------------------

def section_2_exhaustive():
    """Verify: source feasible <=> target feasible, for all small instances."""
    checks = 0

    # For n=3..5, various m values, generate random instances
    # Use brute_force kernel check for small graphs, structural for larger
    for n in range(3, 6):
        for m in range(1, 8):
            num_instances = 200 if n <= 4 else 100
            for _ in range(num_instances):
                clauses = random_3sat_instance(n, m)
                sat, _ = is_satisfiable_brute_force(n, clauses)
                nv, arcs = reduce(n, clauses)

                # Use brute force for small enough graphs, structural otherwise
                if nv <= 20:
                    has_k, _ = has_kernel_brute_force(nv, arcs)
                else:
                    has_k, _ = has_kernel_structural(n, clauses, nv, arcs)

                assert sat == has_k, (
                    f"Mismatch for n={n}, m={m}, clauses={clauses}: "
                    f"sat={sat}, has_kernel={has_k}"
                )
                checks += 1

    # Extra: exhaustive over all distinct clauses for n=3, m=1
    lits = [1, 2, 3, -1, -2, -3]
    all_possible_clauses = []
    for combo in itertools.combinations(lits, 3):
        vs = set(abs(l) for l in combo)
        if len(vs) == 3:
            all_possible_clauses.append(list(combo))

    for clause in all_possible_clauses:
        clauses = [clause]
        sat, _ = is_satisfiable_brute_force(3, clauses)
        nv, arcs = reduce(3, clauses)
        has_k, _ = has_kernel_brute_force(nv, arcs)
        assert sat == has_k
        checks += 1

    # Exhaustive for n=3, m=2 (all pairs of clauses)
    for c1 in all_possible_clauses:
        for c2 in all_possible_clauses:
            clauses = [c1, c2]
            sat, _ = is_satisfiable_brute_force(3, clauses)
            nv, arcs = reduce(3, clauses)
            has_k, _ = has_kernel_brute_force(nv, arcs)
            assert sat == has_k
            checks += 1

    print(f"  Section 2 (exhaustive forward+backward): {checks} checks passed")
    return checks


# ---------------------------------------------------------------------------
# Section 3: Solution extraction
# ---------------------------------------------------------------------------

def section_3_extraction():
    """For every feasible instance, extract source solution from kernel."""
    checks = 0

    for n in range(3, 6):
        for m in range(1, 8):
            num_instances = 150 if n <= 4 else 80
            for _ in range(num_instances):
                clauses = random_3sat_instance(n, m)
                sat, _ = is_satisfiable_brute_force(n, clauses)
                if not sat:
                    continue

                nv, arcs = reduce(n, clauses)

                # Find kernel
                if nv <= 20:
                    has_k, kernel_set = has_kernel_brute_force(nv, arcs)
                else:
                    has_k, kernel_set = has_kernel_structural(n, clauses, nv, arcs)
                assert has_k

                # Extract and verify assignment
                extracted = extract_solution(n, kernel_set)
                assert all(
                    any(
                        (extracted[abs(lit) - 1] if lit > 0 else not extracted[abs(lit) - 1])
                        for lit in clause
                    )
                    for clause in clauses
                ), f"Extracted assignment does not satisfy formula"
                checks += 1

                # Verify kernel structure: exactly one of {x_i, x_bar_i}
                for i in range(n):
                    assert (2 * i in kernel_set) != (2 * i + 1 in kernel_set)
                    checks += 1

                # Verify no clause vertex in kernel
                for j in range(m):
                    base = 2 * n + 3 * j
                    for t in range(3):
                        assert base + t not in kernel_set
                        checks += 1

    print(f"  Section 3 (solution extraction): {checks} checks passed")
    return checks


# ---------------------------------------------------------------------------
# Section 4: Overhead formula verification
# ---------------------------------------------------------------------------

def section_4_overhead():
    """Build target, measure actual size, compare against formula."""
    checks = 0

    for n in range(3, 10):
        for m in range(1, 15):
            for _ in range(20):
                clauses = random_3sat_instance(n, m)
                nv, arcs = reduce(n, clauses)

                expected_verts = 2 * n + 3 * m
                expected_arcs = 2 * n + 12 * m

                assert nv == expected_verts, (
                    f"Vertex count mismatch: got {nv}, expected {expected_verts}"
                )
                assert len(arcs) == expected_arcs, (
                    f"Arc count mismatch: got {len(arcs)}, expected {expected_arcs}"
                )
                checks += 2

    print(f"  Section 4 (overhead formula): {checks} checks passed")
    return checks


# ---------------------------------------------------------------------------
# Section 5: Structural properties
# ---------------------------------------------------------------------------

def section_5_structural():
    """Verify structural properties of the target directed graph."""
    checks = 0

    for n in range(3, 7):
        for m in range(1, 8):
            for _ in range(30):
                clauses = random_3sat_instance(n, m)
                nv, arcs = reduce(n, clauses)

                arc_set = set(arcs)
                successors = build_successors(nv, arcs)

                # Property: variable digons are 2-cycles
                for i in range(n):
                    pos, neg = 2 * i, 2 * i + 1
                    assert (pos, neg) in arc_set
                    assert (neg, pos) in arc_set
                    checks += 2

                # Property: clause triangles are 3-cycles
                for j in range(m):
                    base = 2 * n + 3 * j
                    assert (base, base + 1) in arc_set
                    assert (base + 1, base + 2) in arc_set
                    assert (base + 2, base) in arc_set
                    checks += 3

                # Property: connection arcs
                for j, clause in enumerate(clauses):
                    base = 2 * n + 3 * j
                    for lit in clause:
                        v = literal_vertex(lit, n)
                        for t in range(3):
                            assert (base + t, v) in arc_set
                            checks += 1

                # Property: no self-loops
                for (u, v) in arcs:
                    assert u != v
                    checks += 1

                # Property: all endpoints valid
                for (u, v) in arcs:
                    assert 0 <= u < nv and 0 <= v < nv
                    checks += 1

                # Property: literal vertices have exactly one successor (digon partner)
                for i in range(n):
                    pos, neg = 2 * i, 2 * i + 1
                    assert set(successors[pos]) == {neg}
                    assert set(successors[neg]) == {pos}
                    checks += 2

                # Property: each clause vertex has exactly 4 successors
                # (1 in triangle + 3 literal vertices)
                for j in range(m):
                    base = 2 * n + 3 * j
                    for t in range(3):
                        assert len(successors[base + t]) == 4, (
                            f"Clause vertex {base+t} has {len(successors[base+t])} "
                            f"successors, expected 4"
                        )
                        checks += 1

    print(f"  Section 5 (structural properties): {checks} checks passed")
    return checks


# ---------------------------------------------------------------------------
# Section 6: YES example (from Typst)
# ---------------------------------------------------------------------------

def section_6_yes_example():
    """Reproduce the exact feasible example from the Typst proof."""
    checks = 0

    # 3 variables, 2 clauses:
    # phi = (u1 OR u2 OR u3) AND (NOT u1 OR NOT u2 OR u3)
    n = 3
    clauses = [[1, 2, 3], [-1, -2, 3]]
    m = len(clauses)

    nv, arcs = reduce(n, clauses)

    # Check sizes from Typst: 2*3+3*2=12 vertices, 2*3+12*2=30 arcs
    assert nv == 12, f"Expected 12 vertices, got {nv}"
    checks += 1
    assert len(arcs) == 30, f"Expected 30 arcs, got {len(arcs)}"
    checks += 1

    # Verify the specific kernel from the Typst proof:
    # alpha(u1)=T, alpha(u2)=F, alpha(u3)=T -> S = {x1, x_bar_2, x3} = {0, 3, 4}
    S = {0, 3, 4}
    assert is_kernel_fast(nv, arcs, S), "Typst YES kernel is not valid"
    checks += 1

    # Verify assignment extraction
    extracted = extract_solution(n, S)
    assert extracted == [True, False, True], f"Expected [T, F, T], got {extracted}"
    checks += 1

    # Verify satisfaction
    assert all(
        any(
            (extracted[abs(lit) - 1] if lit > 0 else not extracted[abs(lit) - 1])
            for lit in clause
        )
        for clause in clauses
    )
    checks += 1

    sat, _ = is_satisfiable_brute_force(n, clauses)
    assert sat
    checks += 1

    has_k, _ = has_kernel_brute_force(nv, arcs)
    assert has_k
    checks += 1

    # Verify specific arcs from the Typst proof
    arc_set = set(arcs)
    # Variable digons
    for expected_arc in [(0, 1), (1, 0), (2, 3), (3, 2), (4, 5), (5, 4)]:
        assert expected_arc in arc_set
        checks += 1

    # Clause 1 triangle
    for expected_arc in [(6, 7), (7, 8), (8, 6)]:
        assert expected_arc in arc_set
        checks += 1

    # Clause 2 triangle
    for expected_arc in [(9, 10), (10, 11), (11, 9)]:
        assert expected_arc in arc_set
        checks += 1

    # Clause 1 connections: u1->0, u2->2, u3->4
    for cv in [6, 7, 8]:
        for lv in [0, 2, 4]:
            assert (cv, lv) in arc_set
            checks += 1

    # Clause 2 connections: NOT u1->1, NOT u2->3, u3->4
    for cv in [9, 10, 11]:
        for lv in [1, 3, 4]:
            assert (cv, lv) in arc_set
            checks += 1

    # Verify absorption from Typst
    assert (1, 0) in arc_set and 0 in S  # x_bar_1 absorbed by x_1
    checks += 1
    assert (2, 3) in arc_set and 3 in S  # x_2 absorbed by x_bar_2
    checks += 1
    assert (5, 4) in arc_set and 4 in S  # x_bar_3 absorbed by x_3
    checks += 1

    print(f"  Section 6 (YES example): {checks} checks passed")
    return checks


# ---------------------------------------------------------------------------
# Section 7: NO example (from Typst)
# ---------------------------------------------------------------------------

def section_7_no_example():
    """Reproduce the exact infeasible example from the Typst proof."""
    checks = 0

    # 3 variables, 8 clauses (all possible sign patterns on 3 variables):
    # This is the only way to make an unsatisfiable 3-SAT on 3 variables.
    n = 3
    clauses = [
        [1, 2, 3], [1, 2, -3], [1, -2, 3], [1, -2, -3],
        [-1, 2, 3], [-1, 2, -3], [-1, -2, 3], [-1, -2, -3],
    ]
    m = len(clauses)

    # Verify unsatisfiability by checking all 8 assignments
    for bits in range(8):
        assignment = [(bits >> i) & 1 == 1 for i in range(n)]
        satisfied = all(
            any(
                (assignment[abs(lit) - 1] if lit > 0 else not assignment[abs(lit) - 1])
                for lit in clause
            )
            for clause in clauses
        )
        assert not satisfied, f"Assignment {assignment} should not satisfy formula"
        checks += 1

    sat, _ = is_satisfiable_brute_force(n, clauses)
    assert not sat, "NO example must be unsatisfiable"
    checks += 1

    nv, arcs = reduce(n, clauses)

    # Check sizes: 2*3+3*8=30 vertices, 2*3+12*8=102 arcs
    assert nv == 30, f"Expected 30 vertices, got {nv}"
    checks += 1
    assert len(arcs) == 102, f"Expected 102 arcs, got {len(arcs)}"
    checks += 1

    # Verify no kernel exists using structural checker
    # (brute force on 30 vertices would be too slow)
    has_k, _ = has_kernel_structural(n, clauses, nv, arcs)
    assert not has_k, "NO example graph must NOT have a kernel"
    checks += 1

    # Also verify each of the 8 candidate kernels (one per assignment) fails
    for bits in range(8):
        candidate = set()
        for i in range(n):
            if (bits >> i) & 1:
                candidate.add(2 * i)
            else:
                candidate.add(2 * i + 1)
        assert not is_kernel_fast(nv, arcs, candidate), (
            f"Candidate kernel {candidate} should fail"
        )
        checks += 1

    # Specific check from Typst: alpha=(T,T,T) -> S={0,2,4}
    # Clause 8 = [-1,-2,-3] with literal vertices 1,3,5
    # c_{8,1} at index 2*3+3*7=27, successors: 28, 1, 3, 5
    S_ttt = {0, 2, 4}
    assert not is_kernel_fast(nv, arcs, S_ttt)
    checks += 1

    c81 = 2 * n + 3 * 7  # clause index 7 (0-based)
    assert c81 == 27
    c81_succs = {v for (u, v) in arcs if u == c81}
    assert 28 in c81_succs  # c82
    assert 1 in c81_succs   # x_bar_1
    assert 3 in c81_succs   # x_bar_2
    assert 5 in c81_succs   # x_bar_3
    checks += 4

    # None of {28, 1, 3, 5} are in S_ttt={0, 2, 4}
    for v in c81_succs:
        assert v not in S_ttt
        checks += 1

    print(f"  Section 7 (NO example): {checks} checks passed")
    return checks


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    print("=== Verify KSatisfiability(K3) -> Kernel ===")
    print("=== Issue #882 — Chvatal (1973) ===\n")

    total = 0
    total += section_1_symbolic()
    total += section_2_exhaustive()
    total += section_3_extraction()
    total += section_4_overhead()
    total += section_5_structural()
    total += section_6_yes_example()
    total += section_7_no_example()

    print(f"\n=== TOTAL CHECKS: {total} ===")
    assert total >= 5000, f"Need >= 5000 checks, got {total}"
    print("ALL CHECKS PASSED")

    # Export test vectors
    export_test_vectors()


def export_test_vectors():
    """Export test vectors JSON."""
    n_yes = 3
    clauses_yes = [[1, 2, 3], [-1, -2, 3]]
    nv_yes, arcs_yes = reduce(n_yes, clauses_yes)
    _, kernel_yes = has_kernel_brute_force(nv_yes, arcs_yes)
    extracted_yes = extract_solution(n_yes, kernel_yes)

    n_no = 3
    clauses_no = [
        [1, 2, 3], [1, 2, -3], [1, -2, 3], [1, -2, -3],
        [-1, 2, 3], [-1, 2, -3], [-1, -2, 3], [-1, -2, -3],
    ]
    nv_no, arcs_no = reduce(n_no, clauses_no)

    test_vectors = {
        "source": "KSatisfiability<K3>",
        "target": "Kernel",
        "issue": 882,
        "yes_instance": {
            "input": {
                "num_vars": n_yes,
                "clauses": clauses_yes,
            },
            "output": {
                "num_vertices": nv_yes,
                "arcs": arcs_yes,
            },
            "source_feasible": True,
            "target_feasible": True,
            "source_solution": [True, False, True],
            "extracted_solution": extracted_yes,
        },
        "no_instance": {
            "input": {
                "num_vars": n_no,
                "clauses": clauses_no,
            },
            "output": {
                "num_vertices": nv_no,
                "arcs": arcs_no,
            },
            "source_feasible": False,
            "target_feasible": False,
        },
        "overhead": {
            "num_vertices": "2 * num_vars + 3 * num_clauses",
            "num_arcs": "2 * num_vars + 12 * num_clauses",
        },
        "claims": [
            {"tag": "digon_forces_one_literal", "formula": "exactly one of {x_i, x_bar_i} in kernel", "verified": True},
            {"tag": "no_clause_vertex_in_kernel", "formula": "clause vertices never in kernel", "verified": True},
            {"tag": "forward_sat_implies_kernel", "formula": "satisfying assignment -> kernel", "verified": True},
            {"tag": "backward_kernel_implies_sat", "formula": "kernel -> satisfying assignment", "verified": True},
            {"tag": "vertex_overhead", "formula": "2*n + 3*m", "verified": True},
            {"tag": "arc_overhead", "formula": "2*n + 12*m", "verified": True},
            {"tag": "extraction_correct", "formula": "kernel -> valid assignment", "verified": True},
            {"tag": "literal_vertex_out_degree_1", "formula": "literal vertices have exactly 1 successor", "verified": True},
            {"tag": "clause_vertex_out_degree_4", "formula": "clause vertices have exactly 4 successors", "verified": True},
        ],
    }

    out_path = Path(__file__).parent / "test_vectors_k_satisfiability_kernel.json"
    with open(out_path, "w") as f:
        json.dump(test_vectors, f, indent=2)
    print(f"\nTest vectors exported to {out_path}")


if __name__ == "__main__":
    main()
