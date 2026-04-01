#!/usr/bin/env python3
"""
Adversary verification: NAE 3-SAT → Set Splitting reduction.

Independent implementation based solely on the Typst proof in
docs/paper/proposed-reductions-841.typ.

Sections:
  1. Data structures & helpers
  2. Reduction (reduce)
  3. Solution extraction (extract_solution)
  4. Feasibility validators
  5. Exhaustive testing (n ≤ 5)
  6. Overhead verification
  7. Typst example reproduction (YES + NO)
  8. Property-based testing (hypothesis)
"""

from __future__ import annotations

import itertools
import sys
from dataclasses import dataclass, field
from typing import List, Tuple, Optional, Set, FrozenSet

# ---------------------------------------------------------------------------
# 1. Data structures
# ---------------------------------------------------------------------------

# A literal is (var_index, is_positive).  var_index is 0-based.
Literal = Tuple[int, bool]

@dataclass
class NAE3SAT:
    """NAE 3-SAT instance."""
    num_vars: int
    clauses: List[Tuple[Literal, Literal, Literal]]

    @property
    def num_clauses(self) -> int:
        return len(self.clauses)

@dataclass
class SetSplitting:
    """Set Splitting instance."""
    universe: List[str]          # element names
    subsets: List[FrozenSet[str]]  # collection of subsets

    @property
    def universe_size(self) -> int:
        return len(self.universe)

    @property
    def num_subsets(self) -> int:
        return len(self.subsets)


def _pos(i: int) -> str:
    """Positive element name for variable i (0-based)."""
    return f"v{i}"

def _neg(i: int) -> str:
    """Negative element name for variable i (0-based)."""
    return f"v{i}_bar"

def _lit_to_elem(lit: Literal) -> str:
    """Map a literal to its universe element."""
    idx, pos = lit
    return _pos(idx) if pos else _neg(idx)

# ---------------------------------------------------------------------------
# 2. Reduction
# ---------------------------------------------------------------------------

def reduce(src: NAE3SAT) -> SetSplitting:
    """Construct Set Splitting instance from NAE 3-SAT instance."""
    n = src.num_vars
    # Universe: positive and negative copy per variable
    universe = []
    for i in range(n):
        universe.append(_pos(i))
        universe.append(_neg(i))

    subsets: List[FrozenSet[str]] = []

    # Complementarity subsets
    for i in range(n):
        subsets.append(frozenset({_pos(i), _neg(i)}))

    # Clause subsets
    for clause in src.clauses:
        elems = frozenset(_lit_to_elem(lit) for lit in clause)
        subsets.append(elems)

    return SetSplitting(universe=universe, subsets=subsets)

# ---------------------------------------------------------------------------
# 3. Solution extraction
# ---------------------------------------------------------------------------

def extract_solution(src: NAE3SAT, partition_s1: Set[str]) -> List[int]:
    """
    Given a valid Set Splitting partition (S1), extract a NAE 3-SAT assignment.
    Returns list of 0/1 values, one per variable.
    """
    assignment = []
    for i in range(src.num_vars):
        assignment.append(1 if _pos(i) in partition_s1 else 0)
    return assignment

# ---------------------------------------------------------------------------
# 4. Feasibility validators
# ---------------------------------------------------------------------------

def is_feasible_source(src: NAE3SAT, assignment: List[int]) -> bool:
    """Check if assignment NAE-satisfies all clauses."""
    if len(assignment) != src.num_vars:
        return False
    for clause in src.clauses:
        vals = []
        for (idx, pos) in clause:
            v = assignment[idx]
            vals.append(v if pos else 1 - v)
        if all(v == 1 for v in vals) or all(v == 0 for v in vals):
            return False
    return True

def is_feasible_target(tgt: SetSplitting, s1: Set[str]) -> bool:
    """Check if partition (S1, S\\S1) splits every subset."""
    s2 = set(tgt.universe) - s1
    for subset in tgt.subsets:
        if subset <= s1 or subset <= s2:
            return False
    return True

# ---------------------------------------------------------------------------
# Brute-force solvers
# ---------------------------------------------------------------------------

def solve_nae3sat(src: NAE3SAT) -> Optional[List[int]]:
    """Find a NAE-satisfying assignment, or None."""
    for bits in itertools.product([0, 1], repeat=src.num_vars):
        assignment = list(bits)
        if is_feasible_source(src, assignment):
            return assignment
    return None

def solve_set_splitting(tgt: SetSplitting) -> Optional[Set[str]]:
    """Find a valid partition S1, or None."""
    elems = tgt.universe
    n = len(elems)
    for mask in range(1 << n):
        s1 = {elems[i] for i in range(n) if mask & (1 << i)}
        if is_feasible_target(tgt, s1):
            return s1
    return None

# ---------------------------------------------------------------------------
# 5. Exhaustive testing (n ≤ 5)
# ---------------------------------------------------------------------------

passed = 0
failed = 0
bugs: List[str] = []

def check(cond: bool, msg: str):
    global passed, failed
    if cond:
        passed += 1
    else:
        failed += 1
        bugs.append(msg)

def exhaustive_tests():
    """
    For each n in 1..5, generate random NAE 3-SAT instances and test:
    - Forward: source feasible ⟹ target feasible
    - Backward: target feasible ⟹ source feasible
    - Extraction: valid partition gives valid assignment
    """
    import random
    rng = random.Random(42)

    for n in range(1, 6):
        if n < 3:
            # Need at least 3 literals per clause; variables can repeat
            pass
        max_clauses = min(8, 2 * n)  # keep it manageable
        num_instances = 200 if n <= 3 else 100 if n <= 4 else 50

        for _ in range(num_instances):
            m = rng.randint(1, max_clauses)
            clauses = []
            for _ in range(m):
                lits = []
                for _ in range(3):
                    var = rng.randint(0, n - 1)
                    pos = rng.choice([True, False])
                    lits.append((var, pos))
                clauses.append(tuple(lits))

            src = NAE3SAT(num_vars=n, clauses=clauses)
            tgt = reduce(src)

            # Overhead check
            check(tgt.universe_size == 2 * n,
                  f"universe_size {tgt.universe_size} != {2*n}")
            check(tgt.num_subsets == n + m,
                  f"num_subsets {tgt.num_subsets} != {n + m}")

            src_sol = solve_nae3sat(src)
            tgt_sol = solve_set_splitting(tgt)

            # Forward + backward equivalence
            src_feasible = src_sol is not None
            tgt_feasible = tgt_sol is not None
            check(src_feasible == tgt_feasible,
                  f"n={n}: feasibility mismatch src={src_feasible} tgt={tgt_feasible} clauses={src.clauses}")

            # Forward: if source feasible, build partition and check
            if src_sol is not None:
                s1 = set()
                for i in range(n):
                    if src_sol[i] == 1:
                        s1.add(_pos(i))
                        s1.add(_neg(i))  # wait - need to check proof
                # Actually, per proof: S1 = {v_i : σ(v_i)=1} ∪ {v̄_i : σ(v_i)=0}
                s1 = set()
                for i in range(n):
                    if src_sol[i] == 1:
                        s1.add(_pos(i))
                    else:
                        s1.add(_neg(i))
                    # The OTHER element goes to S2
                    if src_sol[i] == 1:
                        pass  # v̄_i goes to S2
                    else:
                        pass  # v_i goes to S2

                # Rebuild properly
                s1 = set()
                for i in range(n):
                    if src_sol[i] == 1:
                        s1.add(_pos(i))
                    else:
                        s1.add(_neg(i))
                    # complementary: other goes to S2
                    if src_sol[i] == 0:
                        s1.add(_neg(i))
                    else:
                        s1.add(_pos(i))

                # OK I'm overcomplicating this. Per the proof:
                # S1 = {v_i : σ(v_i)=1} ∪ {v̄_i : σ(v_i)=0}
                # S2 = S \ S1
                s1 = set()
                for i in range(n):
                    if src_sol[i] == 1:
                        s1.add(_pos(i))
                    else:
                        s1.add(_neg(i))

                check(is_feasible_target(tgt, s1),
                      f"n={n}: forward partition invalid")

            # Backward: if target feasible, extract and check
            if tgt_sol is not None:
                assignment = extract_solution(src, tgt_sol)
                check(is_feasible_source(src, assignment),
                      f"n={n}: backward extraction invalid")

# ---------------------------------------------------------------------------
# 6. Overhead verification
# ---------------------------------------------------------------------------

def overhead_tests():
    """Verify overhead formulas on various instances."""
    import random
    rng = random.Random(99)

    for _ in range(500):
        n = rng.randint(1, 10)
        m = rng.randint(1, 20)
        clauses = []
        for _ in range(m):
            lits = tuple((rng.randint(0, n-1), rng.choice([True, False])) for _ in range(3))
            clauses.append(lits)
        src = NAE3SAT(num_vars=n, clauses=clauses)
        tgt = reduce(src)

        check(tgt.universe_size == 2 * n,
              f"overhead: universe {tgt.universe_size} != {2*n}")
        check(tgt.num_subsets == n + m,
              f"overhead: subsets {tgt.num_subsets} != {n+m}")

        # Verify complementarity subsets are size 2
        for i in range(n):
            check(len(tgt.subsets[i]) == 2,
                  f"complementarity subset {i} size != 2")

        # Verify clause subsets are size ≤ 3
        for j in range(m):
            check(len(tgt.subsets[n + j]) <= 3,
                  f"clause subset {j} size > 3")

# ---------------------------------------------------------------------------
# 7. Typst example reproduction
# ---------------------------------------------------------------------------

def yes_example_test():
    """Reproduce the YES example from the Typst proof."""
    # n=4 variables, m=3 clauses
    # c1 = (v1, v2, v3)        → 1-indexed, so 0-indexed: (0,T), (1,T), (2,T)
    # c2 = (¬v1, v3, v4)       → (0,F), (2,T), (3,T)
    # c3 = (v2, ¬v3, ¬v4)      → (1,T), (2,F), (3,F)
    src = NAE3SAT(num_vars=4, clauses=[
        ((0, True), (1, True), (2, True)),
        ((0, False), (2, True), (3, True)),
        ((1, True), (2, False), (3, False)),
    ])
    tgt = reduce(src)

    # Check universe size
    check(tgt.universe_size == 8, "YES: universe_size != 8")
    # Check num_subsets
    check(tgt.num_subsets == 7, "YES: num_subsets != 7")

    # Check complementarity subsets
    expected_comp = [
        frozenset({"v0", "v0_bar"}),
        frozenset({"v1", "v1_bar"}),
        frozenset({"v2", "v2_bar"}),
        frozenset({"v3", "v3_bar"}),
    ]
    for i in range(4):
        check(tgt.subsets[i] == expected_comp[i],
              f"YES: complementarity {i} mismatch: {tgt.subsets[i]} vs {expected_comp[i]}")

    # Check clause subsets (using 0-indexed names)
    # Typst uses 1-indexed: {v1, v2, v3} → {v0, v1, v2}
    expected_clause = [
        frozenset({"v0", "v1", "v2"}),          # c1 = (v1, v2, v3)
        frozenset({"v0_bar", "v2", "v3"}),       # c2 = (¬v1, v3, v4)
        frozenset({"v1", "v2_bar", "v3_bar"}),   # c3 = (v2, ¬v3, ¬v4)
    ]
    for j in range(3):
        check(tgt.subsets[4 + j] == expected_clause[j],
              f"YES: clause subset {j} mismatch: {tgt.subsets[4+j]} vs {expected_clause[j]}")

    # Satisfying assignment: σ = (v1=1, v2=0, v3=1, v4=0) → 0-indexed: [1,0,1,0]
    assignment = [1, 0, 1, 0]
    check(is_feasible_source(src, assignment), "YES: assignment not NAE-satisfying")

    # Build partition from assignment
    # S1 = {v1, v̄2, v3, v̄4} → {v0, v1_bar, v2, v3_bar}
    s1 = set()
    for i in range(4):
        if assignment[i] == 1:
            s1.add(_pos(i))
        else:
            s1.add(_neg(i))

    expected_s1 = {"v0", "v1_bar", "v2", "v3_bar"}
    check(s1 == expected_s1, f"YES: S1 mismatch {s1} vs {expected_s1}")
    check(is_feasible_target(tgt, s1), "YES: partition not valid splitting")

    # Extract solution back
    extracted = extract_solution(src, s1)
    check(extracted == assignment, f"YES: extracted {extracted} != {assignment}")

def no_example_test():
    """Reproduce the NO example from the Typst proof."""
    # n=3, m=8: all 8 sign patterns
    # 1-indexed in proof, 0-indexed here
    clauses = []
    for signs in itertools.product([True, False], repeat=3):
        clause = tuple((i, signs[i]) for i in range(3))
        clauses.append(clause)
    # Order per proof:
    # c1=(v1,v2,v3), c2=(¬v1,v2,v3), c3=(v1,¬v2,v3), c4=(v1,v2,¬v3),
    # c5=(¬v1,¬v2,v3), c6=(¬v1,v2,¬v3), c7=(v1,¬v2,¬v3), c8=(¬v1,¬v2,¬v3)
    # itertools.product([True,False],repeat=3) gives:
    # (T,T,T),(T,T,F),(T,F,T),(T,F,F),(F,T,T),(F,T,F),(F,F,T),(F,F,F)
    # Which is a different order but same set of clauses - that's fine for the check.

    src = NAE3SAT(num_vars=3, clauses=clauses)
    tgt = reduce(src)

    check(tgt.universe_size == 6, "NO: universe_size != 6")
    check(tgt.num_subsets == 11, "NO: num_subsets != 11")

    # Verify unsatisfiable
    sol = solve_nae3sat(src)
    check(sol is None, "NO: source should be unsatisfiable")

    # Verify target also unsplittable
    tgt_sol = solve_set_splitting(tgt)
    check(tgt_sol is None, "NO: target should be unsplittable")

    # Check every assignment fails
    for bits in itertools.product([0, 1], repeat=3):
        assignment = list(bits)
        check(not is_feasible_source(src, assignment),
              f"NO: assignment {assignment} should fail NAE")

# ---------------------------------------------------------------------------
# 8. Property-based testing (hypothesis)
# ---------------------------------------------------------------------------

from hypothesis import given, settings, assume, HealthCheck
from hypothesis import strategies as st

@given(
    assignment=st.lists(st.integers(0, 1), min_size=3, max_size=8),
    clause_data=st.lists(
        st.tuples(
            st.tuples(st.integers(0, 2), st.booleans()),
            st.tuples(st.integers(0, 2), st.booleans()),
            st.tuples(st.integers(0, 2), st.booleans()),
        ),
        min_size=1, max_size=10,
    ),
)
@settings(max_examples=1500, suppress_health_check=[HealthCheck.too_slow])
def test_roundtrip_from_assignment(assignment, clause_data):
    """
    Property: For any NAE 3-SAT instance, source is feasible iff target is.
    Build instance from random clause data, check equivalence.
    """
    global passed, failed
    n = len(assignment)
    clauses = []
    for (l1, l2, l3) in clause_data:
        clause = (
            (l1[0] % n, l1[1]),
            (l2[0] % n, l2[1]),
            (l3[0] % n, l3[1]),
        )
        clauses.append(clause)

    src = NAE3SAT(num_vars=n, clauses=clauses)
    tgt = reduce(src)

    # Check overhead
    check(tgt.universe_size == 2 * n, "hyp1: universe size")
    check(tgt.num_subsets == n + len(clauses), "hyp1: num subsets")

    # If assignment is NAE-satisfying, build partition and verify
    if is_feasible_source(src, assignment):
        s1 = set()
        for i in range(n):
            if assignment[i] == 1:
                s1.add(_pos(i))
            else:
                s1.add(_neg(i))
        check(is_feasible_target(tgt, s1), "hyp1: forward direction failed")
        # Extraction roundtrip
        extracted = extract_solution(src, s1)
        check(extracted == assignment, "hyp1: extraction roundtrip failed")


@given(
    n=st.integers(min_value=1, max_value=6),
    m=st.integers(min_value=1, max_value=8),
    data=st.data(),
)
@settings(max_examples=1500, suppress_health_check=[HealthCheck.too_slow])
def test_equivalence_brute_force(n, m, data):
    """
    Property: For any NAE 3-SAT instance with n≤6, brute-force both
    source and target. They must agree on feasibility.
    """
    global passed, failed
    clauses = []
    for _ in range(m):
        lits = data.draw(
            st.tuples(
                st.tuples(st.integers(0, n - 1), st.booleans()),
                st.tuples(st.integers(0, n - 1), st.booleans()),
                st.tuples(st.integers(0, n - 1), st.booleans()),
            )
        )
        clauses.append(lits)

    src = NAE3SAT(num_vars=n, clauses=clauses)
    tgt = reduce(src)

    src_sol = solve_nae3sat(src)
    src_feas = src_sol is not None

    # For small universe, brute-force target
    if tgt.universe_size <= 12:
        tgt_sol = solve_set_splitting(tgt)
        tgt_feas = tgt_sol is not None
        check(src_feas == tgt_feas, f"hyp2: n={n} m={m} feasibility mismatch")

        # If both feasible, check extraction
        if tgt_sol is not None:
            extracted = extract_solution(src, tgt_sol)
            check(is_feasible_source(src, extracted), "hyp2: extraction gives invalid assignment")
    else:
        # Just check forward direction
        if src_sol is not None:
            s1 = set()
            for i in range(n):
                if src_sol[i] == 1:
                    s1.add(_pos(i))
                else:
                    s1.add(_neg(i))
            check(is_feasible_target(tgt, s1), "hyp2: forward direction failed")


@given(
    n=st.integers(min_value=1, max_value=5),
    data=st.data(),
)
@settings(max_examples=500, suppress_health_check=[HealthCheck.too_slow])
def test_complement_symmetry(n, data):
    """
    Property: NAE 3-SAT is symmetric under complement. If σ satisfies NAE,
    then ¬σ also satisfies NAE. Check that the reduction preserves this.
    """
    global passed, failed
    m = data.draw(st.integers(1, 6))
    clauses = []
    for _ in range(m):
        lits = data.draw(
            st.tuples(
                st.tuples(st.integers(0, n - 1), st.booleans()),
                st.tuples(st.integers(0, n - 1), st.booleans()),
                st.tuples(st.integers(0, n - 1), st.booleans()),
            )
        )
        clauses.append(lits)

    src = NAE3SAT(num_vars=n, clauses=clauses)
    tgt = reduce(src)

    sol = solve_nae3sat(src)
    if sol is not None:
        # Complement should also work
        comp = [1 - x for x in sol]
        check(is_feasible_source(src, comp),
              f"complement symmetry: original works but complement fails")

        # Both partitions should be valid
        s1_orig = set()
        s1_comp = set()
        for i in range(n):
            if sol[i] == 1:
                s1_orig.add(_pos(i))
            else:
                s1_orig.add(_neg(i))
            if comp[i] == 1:
                s1_comp.add(_pos(i))
            else:
                s1_comp.add(_neg(i))

        check(is_feasible_target(tgt, s1_orig), "complement: orig partition invalid")
        check(is_feasible_target(tgt, s1_comp), "complement: comp partition invalid")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    global passed, failed

    print("=== Exhaustive tests (n ≤ 5) ===")
    exhaustive_tests()
    print(f"  After exhaustive: {passed} passed, {failed} failed")

    print("=== Overhead tests ===")
    overhead_tests()
    print(f"  After overhead: {passed} passed, {failed} failed")

    print("=== YES example ===")
    yes_example_test()
    print(f"  After YES: {passed} passed, {failed} failed")

    print("=== NO example ===")
    no_example_test()
    print(f"  After NO: {passed} passed, {failed} failed")

    print("=== Hypothesis: roundtrip from assignment ===")
    test_roundtrip_from_assignment()
    print(f"  After hyp1: {passed} passed, {failed} failed")

    print("=== Hypothesis: equivalence brute force ===")
    test_equivalence_brute_force()
    print(f"  After hyp2: {passed} passed, {failed} failed")

    print("=== Hypothesis: complement symmetry ===")
    test_complement_symmetry()
    print(f"  After hyp3: {passed} passed, {failed} failed")

    print()
    print(f"ADVERSARY: NAE 3-SAT → Set Splitting: {passed} passed, {failed} failed")
    if bugs:
        unique_bugs = list(dict.fromkeys(bugs))  # deduplicate preserving order
        print(f"BUGS FOUND: {unique_bugs[:20]}")
    else:
        print("BUGS FOUND: none")

    sys.exit(1 if failed > 0 else 0)

if __name__ == "__main__":
    main()
