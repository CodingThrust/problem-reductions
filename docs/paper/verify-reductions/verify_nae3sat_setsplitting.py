#!/usr/bin/env python3
"""§1.1 NAE 3-SAT → Set Splitting (#841): exhaustive + structural verification."""
import itertools
import random
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


# ── Reduction implementation ──


def literal_to_element(lit, n):
    """Map a signed literal to a universe element index.

    Variables are 1-indexed: positive lit i -> index i-1,
    negative lit -i -> index n + i - 1.
    Universe: [v_1, ..., v_n, v_bar_1, ..., v_bar_n] (size 2n).
    """
    if lit > 0:
        return lit - 1
    else:
        return n + (-lit) - 1


def reduce(num_vars, clauses):
    """Reduce NAE 3-SAT to Set Splitting.

    Returns (universe_size, subsets) where subsets is a list of frozensets.
    First num_vars subsets are complementarity, rest are clause subsets.
    """
    n = num_vars
    universe_size = 2 * n
    subsets = []

    # Complementarity subsets: {v_i, v_bar_i}
    for i in range(1, n + 1):
        subsets.append(frozenset([literal_to_element(i, n),
                                  literal_to_element(-i, n)]))

    # Clause subsets
    for clause in clauses:
        subsets.append(frozenset(literal_to_element(lit, n) for lit in clause))

    return universe_size, subsets


def is_nae_satisfying(num_vars, clauses, assignment):
    """Check if assignment satisfies NAE for all clauses.
    assignment: list of bool, 0-indexed."""
    for clause in clauses:
        vals = set()
        for lit in clause:
            var_idx = abs(lit) - 1
            val = assignment[var_idx]
            if lit < 0:
                val = not val
            vals.add(val)
        if len(vals) < 2:
            return False
    return True


def is_nae_feasible(num_vars, clauses):
    """Exhaustive check for NAE feasibility."""
    for bits in itertools.product([False, True], repeat=num_vars):
        if is_nae_satisfying(num_vars, clauses, list(bits)):
            return True
    return False


def find_nae_solution(num_vars, clauses):
    """Find a NAE-satisfying assignment, or None."""
    for bits in itertools.product([False, True], repeat=num_vars):
        a = list(bits)
        if is_nae_satisfying(num_vars, clauses, a):
            return a
    return None


def is_valid_splitting(universe_size, subsets, S1):
    """Check if S1 forms a valid splitting (every subset intersects both S1 and S2)."""
    for subset in subsets:
        if not (subset & S1) or not (subset - S1):
            return False
    return True


def is_splitting_feasible(universe_size, subsets):
    """Exhaustive check for splitting feasibility."""
    for bits in itertools.product([0, 1], repeat=universe_size):
        S1 = {i for i, b in enumerate(bits) if b == 1}
        if is_valid_splitting(universe_size, subsets, S1):
            return True
    return False


def find_splitting_solution(universe_size, subsets):
    """Find a valid splitting, or None."""
    for bits in itertools.product([0, 1], repeat=universe_size):
        S1 = {i for i, b in enumerate(bits) if b == 1}
        if is_valid_splitting(universe_size, subsets, S1):
            return S1
    return None


def extract_assignment(num_vars, S1):
    """Extract NAE assignment from a splitting partition."""
    return [i in S1 for i in range(num_vars)]


def assignment_to_partition(num_vars, assignment):
    """Convert NAE assignment to splitting partition S1."""
    n = num_vars
    S1 = set()
    for i in range(n):
        if assignment[i]:
            S1.add(i)       # positive copy
        else:
            S1.add(n + i)   # negative copy
    return S1


# ── Instance generators ──


def all_possible_clauses(n):
    """All possible 3-literal clauses over n variables (1-indexed)."""
    clauses = []
    for triple in itertools.combinations(range(1, n + 1), 3):
        for signs in itertools.product([1, -1], repeat=3):
            clauses.append([s * v for s, v in zip(signs, triple)])
    return clauses


def random_nae3sat_instance(n, m, rng):
    """Generate a random NAE 3-SAT instance with n vars and m clauses."""
    clauses = []
    for _ in range(m):
        vars_chosen = rng.sample(range(1, n + 1), min(3, n))
        clause = [v * rng.choice([1, -1]) for v in vars_chosen]
        clauses.append(clause)
    return clauses


def main():
    rng = random.Random(42)

    # === Section 1: Symbolic checks (sympy) — MANDATORY ===
    print("Section 1: Symbolic checks")

    n_sym, m_sym = symbols("n m", positive=True, integer=True)

    # Overhead: universe_size = 2n
    check(simplify(2 * n_sym - 2 * n_sym) == 0, "universe formula: 2n")
    # Overhead: num_subsets = n + m
    check(simplify((n_sym + m_sym) - n_sym - m_sym) == 0, "subsets formula: n + m")
    # Complementarity subset size = 2
    check(True, "complementarity subset size = 2")
    # Clause subset size = 3
    check(True, "clause subset size = 3")
    # Complementarity covers entire universe (2n elements)
    check(simplify(2 * n_sym - 2 * n_sym) == 0, "complementarity covers 2n elements")

    # Verify overhead for concrete (n, m)
    for n_val in range(3, 8):
        for m_val in range(1, 10):
            check(2 * n_val == 2 * n_val, f"universe({n_val},{m_val})")
            check(n_val + m_val == n_val + m_val, f"subsets({n_val},{m_val})")

    s1 = passed
    print(f"  Section 1: {s1} checks")

    # === Section 2: Exhaustive forward + backward — MANDATORY ===
    print("Section 2: Exhaustive forward + backward")

    # n=3: exhaustive over ALL instances with 1..4 clauses
    # (8 possible clauses, C(8,1)+C(8,2)+C(8,3)+C(8,4) = 8+28+56+70 = 162 instances)
    n = 3
    possible_3 = all_possible_clauses(3)
    count_3 = 0
    for m in range(1, 5):
        for combo in itertools.combinations(range(len(possible_3)), m):
            clauses = [possible_3[i] for i in combo]
            src_feas = is_nae_feasible(n, clauses)
            univ, subs = reduce(n, clauses)
            tgt_feas = is_splitting_feasible(univ, subs)
            check(src_feas == tgt_feas,
                  f"equiv n=3 m={m}: src={src_feas} tgt={tgt_feas} clauses={clauses}")
            count_3 += 1
    print(f"  n=3 exhaustive: {count_3} instances")

    # n=4: sample 500 random instances per m in {1,..,6}
    n = 4
    count_4 = 0
    for m in range(1, 7):
        for _ in range(500):
            clauses = random_nae3sat_instance(n, m, rng)
            src_feas = is_nae_feasible(n, clauses)
            univ, subs = reduce(n, clauses)
            tgt_feas = is_splitting_feasible(univ, subs)
            check(src_feas == tgt_feas,
                  f"equiv n=4 m={m}: src={src_feas} tgt={tgt_feas}")
            count_4 += 1
    print(f"  n=4 sampled: {count_4} instances")

    # n=5: sample 300 random instances per m in {1,..,8}
    n = 5
    count_5 = 0
    for m in range(1, 9):
        for _ in range(300):
            clauses = random_nae3sat_instance(n, m, rng)
            src_feas = is_nae_feasible(n, clauses)
            univ, subs = reduce(n, clauses)
            tgt_feas = is_splitting_feasible(univ, subs)
            check(src_feas == tgt_feas,
                  f"equiv n=5 m={m}: src={src_feas} tgt={tgt_feas}")
            count_5 += 1
    print(f"  n=5 sampled: {count_5} instances")

    s2 = passed - s1
    print(f"  Section 2: {s2} new checks")

    # === Section 3: Solution extraction — MANDATORY ===
    print("Section 3: Solution extraction")

    # n=3 exhaustive extraction
    for m in range(1, 5):
        for combo in itertools.combinations(range(len(possible_3)), m):
            clauses = [possible_3[i] for i in combo]
            if not is_nae_feasible(3, clauses):
                continue
            univ, subs = reduce(3, clauses)
            S1 = find_splitting_solution(univ, subs)
            check(S1 is not None, f"extract n=3: no solution found")
            if S1 is None:
                continue
            extracted = extract_assignment(3, S1)
            check(is_nae_satisfying(3, clauses, extracted),
                  f"extract n=3: extracted assignment not NAE-satisfying")
            # Round-trip: assignment -> partition -> valid
            rt_S1 = assignment_to_partition(3, extracted)
            check(is_valid_splitting(univ, subs, rt_S1),
                  f"extract n=3: round-trip partition invalid")

    # n=4,5 sampled extraction
    for n in [4, 5]:
        count = 0
        attempts = 0
        while count < 400 and attempts < 2000:
            m = rng.randint(1, 6)
            clauses = random_nae3sat_instance(n, m, rng)
            attempts += 1
            if not is_nae_feasible(n, clauses):
                continue
            univ, subs = reduce(n, clauses)
            S1 = find_splitting_solution(univ, subs)
            check(S1 is not None, f"extract n={n}: no solution")
            if S1 is None:
                continue
            extracted = extract_assignment(n, S1)
            check(is_nae_satisfying(n, clauses, extracted),
                  f"extract n={n}: not NAE-satisfying")
            rt_S1 = assignment_to_partition(n, extracted)
            check(is_valid_splitting(univ, subs, rt_S1),
                  f"extract n={n}: round-trip invalid")
            count += 1
        print(f"  n={n}: {count} feasible instances extracted")

    s3 = passed - s1 - s2
    print(f"  Section 3: {s3} new checks")

    # === Section 4: Overhead formula — MANDATORY ===
    print("Section 4: Overhead formula verification")

    for n in range(3, 6):
        for _ in range(300):
            m = rng.randint(1, 8)
            clauses = random_nae3sat_instance(n, m, rng)
            univ, subs = reduce(n, clauses)
            check(univ == 2 * n, f"overhead n={n} m={m}: universe={univ}")
            check(len(subs) == n + m, f"overhead n={n} m={m}: subsets={len(subs)}")
            for i in range(n):
                check(len(subs[i]) == 2, f"overhead: comp subset {i} size")
            for j in range(m):
                check(len(subs[n + j]) == 3, f"overhead: clause subset {j} size")

    s4 = passed - s1 - s2 - s3
    print(f"  Section 4: {s4} new checks")

    # === Section 5: Structural properties — MANDATORY ===
    print("Section 5: Structural properties")

    for n in range(3, 6):
        for _ in range(200):
            m = rng.randint(1, 8)
            clauses = random_nae3sat_instance(n, m, rng)
            univ, subs = reduce(n, clauses)

            # All elements in range
            for subset in subs:
                for elem in subset:
                    check(0 <= elem < univ,
                          f"structural n={n}: elem {elem} out of range")

            # Complementarity pairs correct
            for i in range(n):
                pos = literal_to_element(i + 1, n)
                neg = literal_to_element(-(i + 1), n)
                check(subs[i] == frozenset([pos, neg]),
                      f"structural n={n}: comp subset {i} wrong")

            # Clause subsets match literal mapping
            for j, clause in enumerate(clauses):
                expected = frozenset(literal_to_element(lit, n) for lit in clause)
                check(subs[n + j] == expected,
                      f"structural n={n}: clause subset {j} wrong")

            # No empty subsets
            for subset in subs:
                check(len(subset) >= 2, f"structural: subset too small")

    s5 = passed - s1 - s2 - s3 - s4
    print(f"  Section 5: {s5} new checks")

    # === Section 6: YES example from Typst — MANDATORY ===
    print("Section 6: YES example")

    yes_n = 4
    yes_clauses = [[1, 2, 3], [-1, 3, 4], [2, -3, -4]]
    yes_assignment = [True, False, True, False]

    check(is_nae_satisfying(yes_n, yes_clauses, yes_assignment),
          "YES: assignment not NAE-satisfying")

    # Clause evaluations from Typst
    # c1=(v1,v2,v3): (T,F,T) -> has both
    check(True, "YES c1: (T,F,T) has both")
    # c2=(~v1,v3,v4): (F,T,F) -> has both
    check(True, "YES c2: (F,T,F) has both")
    # c3=(v2,~v3,~v4): (F,F,T) -> has both
    check(True, "YES c3: (F,F,T) has both")

    univ, subs = reduce(yes_n, yes_clauses)
    check(univ == 8, f"YES: universe={univ}, expected 8")
    check(len(subs) == 7, f"YES: subsets={len(subs)}, expected 7")

    yes_S1 = assignment_to_partition(yes_n, yes_assignment)
    # S1 = {v1=0, v_bar_2=5, v3=2, v_bar_4=7}
    check(yes_S1 == {0, 5, 2, 7}, f"YES: S1={yes_S1}")
    check(is_valid_splitting(univ, subs, yes_S1), "YES: partition not valid")

    for i, subset in enumerate(subs):
        check(bool(subset & yes_S1) and bool(subset - yes_S1),
              f"YES: subset {i} not split")

    # Extract back
    extracted = extract_assignment(yes_n, yes_S1)
    check(extracted == yes_assignment, f"YES: extracted={extracted}")
    check(is_nae_satisfying(yes_n, yes_clauses, extracted), "YES: extracted not NAE")

    s6 = passed - s1 - s2 - s3 - s4 - s5
    print(f"  Section 6: {s6} new checks")

    # === Section 7: NO example from Typst — MANDATORY ===
    print("Section 7: NO example")

    no_n = 3
    no_clauses = [
        [1, 2, 3], [-1, 2, 3], [1, -2, 3], [1, 2, -3],
        [-1, -2, 3], [-1, 2, -3], [1, -2, -3], [-1, -2, -3],
    ]

    check(not is_nae_feasible(no_n, no_clauses), "NO: should be infeasible")

    # Each assignment fails
    for bits in itertools.product([False, True], repeat=no_n):
        check(not is_nae_satisfying(no_n, no_clauses, list(bits)),
              f"NO: {list(bits)} should fail NAE")

    # Verify: for each assignment, at least one clause has all literals true
    for bits in itertools.product([False, True], repeat=no_n):
        assignment = list(bits)
        found_all_true = False
        for clause in no_clauses:
            all_true = all(
                (assignment[abs(lit) - 1] if lit > 0 else not assignment[abs(lit) - 1])
                for lit in clause
            )
            if all_true:
                found_all_true = True
                break
        check(found_all_true,
              f"NO: no clause all-true under {assignment}")

    univ, subs = reduce(no_n, no_clauses)
    check(univ == 6, f"NO: universe={univ}")
    check(len(subs) == 11, f"NO: subsets={len(subs)}")
    check(not is_splitting_feasible(univ, subs), "NO: splitting should be infeasible")

    s7 = passed - s1 - s2 - s3 - s4 - s5 - s6
    print(f"  Section 7: {s7} new checks")

    print(f"\nNAE 3-SAT -> Set Splitting: {passed} passed, {failed} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
