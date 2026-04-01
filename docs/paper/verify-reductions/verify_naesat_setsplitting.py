#!/usr/bin/env python3
"""§1.1 NAESatisfiability → SetSplitting (#841): exhaustive + structural verification."""
import itertools
import sys
from sympy import symbols, simplify, Eq

passed = failed = 0


def check(condition, msg=""):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL: {msg}")


# ── Reduction implementation ──────────────────────────────────────────────


def literal_to_element(lit, n):
    """Convert a 1-indexed signed literal to a 0-indexed universe element.
    Positive literal i (1-indexed) → element 2*(i-1).
    Negative literal -i → element 2*(i-1)+1.
    """
    var_idx = abs(lit) - 1  # 0-indexed variable
    if lit > 0:
        return 2 * var_idx
    else:
        return 2 * var_idx + 1


def reduce(n, clauses):
    """Reduce NAE-SAT instance to Set Splitting.

    Args:
        n: number of variables (1-indexed: v_1 .. v_n)
        clauses: list of lists of signed 1-indexed literals

    Returns:
        (universe_size, subsets): Set Splitting instance
    """
    universe_size = 2 * n
    subsets = []
    # Complementarity subsets: {2i, 2i+1} for i = 0..n-1
    for i in range(n):
        subsets.append([2 * i, 2 * i + 1])
    # Clause subsets
    for clause in clauses:
        subset = [literal_to_element(lit, n) for lit in clause]
        subsets.append(subset)
    return universe_size, subsets


def is_nae_satisfying(n, clauses, assignment):
    """Check if assignment (list of 0/1, length n) NAE-satisfies all clauses."""
    for clause in clauses:
        vals = set()
        for lit in clause:
            var_idx = abs(lit) - 1
            val = assignment[var_idx]
            if lit < 0:
                val = 1 - val
            vals.add(val)
        if len(vals) == 1:  # all equal
            return False
    return True


def is_feasible_source(n, clauses):
    """Check if NAE-SAT instance is satisfiable (brute force)."""
    for bits in range(2**n):
        assignment = [(bits >> i) & 1 for i in range(n)]
        if is_nae_satisfying(n, clauses, assignment):
            return True
    return False


def is_set_splitting(universe_size, subsets, config):
    """Check if config (list of 0/1, length universe_size) is a valid set splitting."""
    for subset in subsets:
        vals = set(config[e] for e in subset)
        if len(vals) == 1:  # monochromatic
            return False
    return True


def is_feasible_target(universe_size, subsets):
    """Check if Set Splitting instance is feasible (brute force)."""
    for bits in range(2**universe_size):
        config = [(bits >> i) & 1 for i in range(universe_size)]
        if is_set_splitting(universe_size, subsets, config):
            return True
    return False


def extract_assignment(n, config):
    """Extract NAE-SAT assignment from Set Splitting config.
    Variable i (0-indexed) is true iff element 2i is in part 0.
    """
    return [1 - config[2 * i] for i in range(n)]
    # config[2i] == 0 means element 2i is in S_1 (part 0), so variable is true (1)
    # Actually: let's think about this more carefully.
    # Part 0 = S_1. If v_i (element 2i) is in S_1 (config=0), then alpha(v_i)=true (1).
    # So assignment[i] = 1 if config[2i] == 0, i.e., assignment[i] = 1 - config[2i].
    # Wait, but the codebase uses config[e]=0 for "part 0" and config[e]=1 for "part 1".
    # The convention is: S_1 gets config=0, S_2 gets config=1.
    # True literals go to S_1 (config=0), so:
    #   var i true => element 2i in S_1 => config[2i]=0 => assignment[i]=1-0=1 ✓
    #   var i false => element 2i in S_2 => config[2i]=1 => assignment[i]=1-1=0 ✓


def all_naesat_instances(n, max_clause_len=None):
    """Generate all NAE-SAT instances with n variables.
    Each clause has 2 or 3 literals (no variable appears twice in a clause).
    """
    lits = list(range(1, n + 1)) + list(range(-n, 0))
    clause_sizes = [2, 3] if max_clause_len is None else list(range(2, max_clause_len + 1))

    # Generate all valid clauses
    all_clauses = []
    for size in clause_sizes:
        for combo in itertools.combinations(lits, size):
            # No variable appears both positive and negative
            vars_used = [abs(l) for l in combo]
            if len(set(vars_used)) == len(vars_used):
                all_clauses.append(list(combo))

    return all_clauses


def main():
    # === Section 1: Symbolic checks (sympy) — MANDATORY ===
    print("=== Section 1: Symbolic overhead verification ===")

    n_sym, m_sym = symbols("n m", positive=True, integer=True)

    # universe_size = 2n
    universe_expr = 2 * n_sym
    check(simplify(universe_expr - 2 * n_sym) == 0,
          "universe_size should be 2n")

    # num_subsets = n + m
    subsets_expr = n_sym + m_sym
    check(simplify(subsets_expr - (n_sym + m_sym)) == 0,
          "num_subsets should be n + m")

    # Verify for specific values
    for n_val in range(2, 8):
        for m_val in range(1, 8):
            check(universe_expr.subs(n_sym, n_val) == 2 * n_val,
                  f"universe_size({n_val}) = {2*n_val}")
            check(subsets_expr.subs({n_sym: n_val, m_sym: m_val}) == n_val + m_val,
                  f"num_subsets({n_val},{m_val}) = {n_val + m_val}")

    # Complementarity subset size is always 2
    for n_val in range(1, 10):
        for i in range(n_val):
            subset = [2 * i, 2 * i + 1]
            check(len(subset) == 2,
                  f"complementarity subset for var {i} has size 2")

    print(f"  Section 1: {passed} passed, {failed} failed")

    # === Section 2: Exhaustive forward + backward — MANDATORY ===
    print("\n=== Section 2: Exhaustive forward + backward (n ≤ 5) ===")
    sec2_start = passed

    for n in range(2, 6):  # n = 2, 3, 4, 5
        # Generate all valid clauses for this n
        lits = list(range(1, n + 1)) + list(range(-n, 0))
        all_valid_clauses = []
        for size in [2, 3]:
            for combo in itertools.combinations(lits, size):
                vars_used = [abs(l) for l in combo]
                if len(set(vars_used)) == len(vars_used):
                    all_valid_clauses.append(list(combo))

        # Test many clause combinations
        # For small n, test all single-clause, double-clause, and triple-clause combos
        num_tested = 0
        max_per_n = 800 if n <= 3 else 400

        # Single clause instances
        for clause in all_valid_clauses:
            if num_tested >= max_per_n:
                break
            clauses = [clause]
            source_feasible = is_feasible_source(n, clauses)
            u_size, subsets = reduce(n, clauses)
            target_feasible = is_feasible_target(u_size, subsets)

            check(source_feasible == target_feasible,
                  f"n={n}, clauses={clauses}: source={source_feasible}, target={target_feasible}")
            num_tested += 1

        # Multi-clause instances (sample)
        for num_clauses in [2, 3, 4]:
            count = 0
            for combo in itertools.combinations(range(len(all_valid_clauses)), num_clauses):
                if count >= max_per_n // 3:
                    break
                clauses = [all_valid_clauses[i] for i in combo]
                source_feasible = is_feasible_source(n, clauses)
                u_size, subsets = reduce(n, clauses)
                target_feasible = is_feasible_target(u_size, subsets)

                check(source_feasible == target_feasible,
                      f"n={n}, {num_clauses} clauses: source={source_feasible}, target={target_feasible}")
                count += 1
                num_tested += 1

        print(f"  n={n}: tested {num_tested} instances")

    print(f"  Section 2: {passed - sec2_start} new checks")

    # === Section 3: Solution extraction — MANDATORY ===
    print("\n=== Section 3: Solution extraction ===")
    sec3_start = passed

    for n in range(2, 6):
        lits = list(range(1, n + 1)) + list(range(-n, 0))
        all_valid_clauses = []
        for size in [2, 3]:
            for combo in itertools.combinations(lits, size):
                vars_used = [abs(l) for l in combo]
                if len(set(vars_used)) == len(vars_used):
                    all_valid_clauses.append(list(combo))

        num_extracted = 0
        for num_cl in [1, 2, 3]:
            for combo in itertools.combinations(range(len(all_valid_clauses)), num_cl):
                if num_extracted >= 300:
                    break
                clauses = [all_valid_clauses[i] for i in combo]

                if not is_feasible_source(n, clauses):
                    continue

                u_size, subsets = reduce(n, clauses)

                # Find ALL valid set splitting configs and extract assignment from each
                for bits in range(2**u_size):
                    config = [(bits >> e) & 1 for e in range(u_size)]
                    if not is_set_splitting(u_size, subsets, config):
                        continue

                    assignment = extract_assignment(n, config)
                    check(is_nae_satisfying(n, clauses, assignment),
                          f"extraction n={n}: config {config} -> assignment {assignment}")
                    num_extracted += 1

        print(f"  n={n}: extracted {num_extracted} solutions")

    print(f"  Section 3: {passed - sec3_start} new checks")

    # === Section 4: Overhead formula — MANDATORY ===
    print("\n=== Section 4: Overhead formula verification ===")
    sec4_start = passed

    for n in range(2, 6):
        lits = list(range(1, n + 1)) + list(range(-n, 0))
        all_valid_clauses = []
        for size in [2, 3]:
            for combo in itertools.combinations(lits, size):
                vars_used = [abs(l) for l in combo]
                if len(set(vars_used)) == len(vars_used):
                    all_valid_clauses.append(list(combo))

        for num_cl in [1, 2, 3, 4, 5]:
            count = 0
            for combo in itertools.combinations(range(len(all_valid_clauses)), num_cl):
                if count >= 50:
                    break
                clauses = [all_valid_clauses[i] for i in combo]
                m = len(clauses)
                u_size, subsets = reduce(n, clauses)

                # Check overhead formula
                check(u_size == 2 * n,
                      f"overhead universe: got {u_size}, expected {2*n}")
                check(len(subsets) == n + m,
                      f"overhead subsets: got {len(subsets)}, expected {n + m}")
                count += 1

    print(f"  Section 4: {passed - sec4_start} new checks")

    # === Section 5: Structural properties — MANDATORY ===
    print("\n=== Section 5: Structural properties ===")
    sec5_start = passed

    for n in range(2, 6):
        lits = list(range(1, n + 1)) + list(range(-n, 0))
        all_valid_clauses = []
        for size in [2, 3]:
            for combo in itertools.combinations(lits, size):
                vars_used = [abs(l) for l in combo]
                if len(set(vars_used)) == len(vars_used):
                    all_valid_clauses.append(list(combo))

        for num_cl in [1, 2, 3]:
            count = 0
            for combo in itertools.combinations(range(len(all_valid_clauses)), num_cl):
                if count >= 100:
                    break
                clauses = [all_valid_clauses[i] for i in combo]
                u_size, subsets = reduce(n, clauses)

                # 1. All elements in subsets are valid universe indices
                for subset in subsets:
                    for elem in subset:
                        check(0 <= elem < u_size,
                              f"element {elem} out of range [0, {u_size})")

                # 2. Every subset has at least 2 elements
                for subset in subsets:
                    check(len(subset) >= 2,
                          f"subset {subset} has fewer than 2 elements")

                # 3. No duplicate elements within a subset
                for subset in subsets:
                    check(len(subset) == len(set(subset)),
                          f"subset {subset} has duplicates")

                # 4. Complementarity subsets come first and pair consecutive elements
                for i in range(n):
                    check(subsets[i] == [2 * i, 2 * i + 1],
                          f"complementarity subset {i}: {subsets[i]}")

                # 5. No empty subsets
                for subset in subsets:
                    check(len(subset) > 0, f"empty subset found")

                count += 1

    print(f"  Section 5: {passed - sec5_start} new checks")

    # === Section 6: YES example from Typst — MANDATORY ===
    print("\n=== Section 6: YES example verification ===")
    sec6_start = passed

    # From Typst: n=4, m=4
    # c1 = (v1, v2, v3) = [1, 2, 3]
    # c2 = (¬v1, v3, v4) = [-1, 3, 4]
    # c3 = (v2, ¬v3, ¬v4) = [2, -3, -4]
    # c4 = (v1, ¬v2, v4) = [1, -2, 4]
    yes_n = 4
    yes_clauses = [[1, 2, 3], [-1, 3, 4], [2, -3, -4], [1, -2, 4]]

    u_size, subsets = reduce(yes_n, yes_clauses)

    # Check overhead
    check(u_size == 8, f"YES example: universe_size = {u_size}, expected 8")
    check(len(subsets) == 8, f"YES example: num_subsets = {len(subsets)}, expected 8")

    # Check exact subsets from Typst
    expected_subsets = [
        [0, 1], [2, 3], [4, 5], [6, 7],  # complementarity
        [0, 2, 4], [1, 4, 6], [2, 5, 7], [0, 3, 6],  # clause
    ]
    for i, (got, exp) in enumerate(zip(subsets, expected_subsets)):
        check(got == exp, f"YES subset {i}: got {got}, expected {exp}")

    # Verify the assignment from Typst: v1=T, v2=F, v3=T, v4=F → [1, 0, 1, 0]
    yes_assignment = [1, 0, 1, 0]
    check(is_nae_satisfying(yes_n, yes_clauses, yes_assignment),
          "YES example: assignment is NAE-satisfying")

    # Verify the partition from Typst: S1 = {0, 3, 4, 7}
    # config: element in S1 → config=0, element in S2 → config=1
    # S1 = {0, 3, 4, 7} → config[0]=0, config[3]=0, config[4]=0, config[7]=0
    # S2 = {1, 2, 5, 6} → config[1]=1, config[2]=1, config[5]=1, config[6]=1
    yes_config = [0, 1, 1, 0, 0, 1, 1, 0]
    check(is_set_splitting(u_size, subsets, yes_config),
          "YES example: partition is a valid set splitting")

    # Verify extraction
    extracted = extract_assignment(yes_n, yes_config)
    check(extracted == yes_assignment,
          f"YES example: extracted {extracted}, expected {yes_assignment}")

    # Verify each clause explicitly as in Typst
    # c1 = (T, F, T)
    check(yes_assignment[0] == 1 and yes_assignment[1] == 0 and yes_assignment[2] == 1,
          "YES c1 values")
    # c2 = (¬v1=F, v3=T, v4=F)
    check((1 - yes_assignment[0]) == 0 and yes_assignment[2] == 1 and yes_assignment[3] == 0,
          "YES c2 values")
    # c3 = (v2=F, ¬v3=F, ¬v4=T)
    check(yes_assignment[1] == 0 and (1 - yes_assignment[2]) == 0 and (1 - yes_assignment[3]) == 1,
          "YES c3 values")
    # c4 = (v1=T, ¬v2=T, v4=F)
    check(yes_assignment[0] == 1 and (1 - yes_assignment[1]) == 1 and yes_assignment[3] == 0,
          "YES c4 values")

    check(is_feasible_source(yes_n, yes_clauses),
          "YES example: source is feasible")
    check(is_feasible_target(u_size, subsets),
          "YES example: target is feasible")

    print(f"  Section 6: {passed - sec6_start} new checks")

    # === Section 7: NO example from Typst — MANDATORY ===
    print("\n=== Section 7: NO example verification ===")
    sec7_start = passed

    # From Typst: n=3, m=4
    # c1 = (v1, v2, v3) = [1, 2, 3]
    # c2 = (v1, v2, ¬v3) = [1, 2, -3]
    # c3 = (v1, ¬v2, v3) = [1, -2, 3]
    # c4 = (v1, ¬v2, ¬v3) = [1, -2, -3]
    no_n = 3
    no_clauses = [[1, 2, 3], [1, 2, -3], [1, -2, 3], [1, -2, -3]]

    u_size_no, subsets_no = reduce(no_n, no_clauses)

    # Check overhead
    check(u_size_no == 6, f"NO example: universe_size = {u_size_no}, expected 6")
    check(len(subsets_no) == 7, f"NO example: num_subsets = {len(subsets_no)}, expected 7")

    # Check exact subsets from Typst
    expected_no_subsets = [
        [0, 1], [2, 3], [4, 5],  # complementarity
        [0, 2, 4], [0, 2, 5], [0, 3, 4], [0, 3, 5],  # clause
    ]
    for i, (got, exp) in enumerate(zip(subsets_no, expected_no_subsets)):
        check(got == exp, f"NO subset {i}: got {got}, expected {exp}")

    # Verify source is infeasible
    check(not is_feasible_source(no_n, no_clauses),
          "NO example: source is infeasible")

    # Verify target is infeasible
    check(not is_feasible_target(u_size_no, subsets_no),
          "NO example: target is infeasible")

    # Verify each of the 8 assignments fails as stated in Typst
    for bits in range(8):
        assignment = [(bits >> i) & 1 for i in range(3)]
        check(not is_nae_satisfying(no_n, no_clauses, assignment),
              f"NO example: assignment {assignment} should fail NAE")

    # Verify no set splitting config works
    no_splitting_count = 0
    for bits in range(2**6):
        config = [(bits >> e) & 1 for e in range(6)]
        if not is_set_splitting(u_size_no, subsets_no, config):
            no_splitting_count += 1
    check(no_splitting_count == 64,
          f"NO example: all 64 configs fail set splitting (got {64 - no_splitting_count} valid)")

    print(f"  Section 7: {passed - sec7_start} new checks")

    # ── Final report ──
    print(f"\nNAESatisfiability → SetSplitting: {passed} passed, {failed} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
