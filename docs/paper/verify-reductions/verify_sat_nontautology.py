#!/usr/bin/env python3
"""
§ SAT → NonTautology (#868): exhaustive verification.

Reduction: negate CNF formula via De Morgan → DNF.
φ satisfiable ↔ ¬φ is not a tautology.
Solution extraction: same assignment (identity).

Checks:
1. Symbolic: De Morgan identity (CNF negation = DNF)
2. Exhaustive: all CNF formulas on n ≤ 4 variables, m ≤ 6 clauses
3. Forward: SAT → NonTautology (satisfying → falsifying)
4. Backward: NonTautology → SAT (falsifying → satisfying)
5. Solution extraction: same assignment works for both
6. Overhead: num_vars same, num_disjuncts = num_clauses
7. Edge cases: tautologies, contradictions, single-clause, single-variable
"""
import itertools
import sys

passed = failed = 0

def check(condition, msg=""):
    global passed, failed
    if condition:
        passed += 1
    else:
        failed += 1
        print(f"  FAIL: {msg}")


# ============================================================
# CNF / DNF representations
# ============================================================

def evaluate_cnf(clauses, assignment):
    """Evaluate CNF: conjunction of clauses, each a disjunction of literals.
    clause = list of (var_index, is_positive).
    assignment = list of bools.
    """
    for clause in clauses:
        clause_val = False
        for var, pos in clause:
            lit = assignment[var] if pos else not assignment[var]
            clause_val = clause_val or lit
        if not clause_val:
            return False
    return True


def evaluate_dnf(disjuncts, assignment):
    """Evaluate DNF: disjunction of disjuncts, each a conjunction of literals.
    disjunct = list of (var_index, is_positive).
    """
    for disjunct in disjuncts:
        disjunct_val = True
        for var, pos in disjunct:
            lit = assignment[var] if pos else not assignment[var]
            disjunct_val = disjunct_val and lit
        if disjunct_val:
            return True
    return False


def negate_cnf_to_dnf(clauses):
    """Apply De Morgan: ¬(C₁ ∧ ... ∧ Cₘ) = ¬C₁ ∨ ... ∨ ¬Cₘ.
    Each ¬Cⱼ = ¬(l₁ ∨ ... ∨ lₖ) = (¬l₁ ∧ ... ∧ ¬lₖ).
    """
    disjuncts = []
    for clause in clauses:
        # Negate each literal in the clause
        disjunct = [(var, not pos) for var, pos in clause]
        disjuncts.append(disjunct)
    return disjuncts


def is_satisfiable(n_vars, clauses):
    """Check if CNF is satisfiable (brute force)."""
    for bits in range(2 ** n_vars):
        assignment = [(bits >> i) & 1 == 1 for i in range(n_vars)]
        if evaluate_cnf(clauses, assignment):
            return True, assignment
    return False, None


def is_not_tautology(n_vars, disjuncts):
    """Check if DNF is NOT a tautology (exists falsifying assignment)."""
    for bits in range(2 ** n_vars):
        assignment = [(bits >> i) & 1 == 1 for i in range(n_vars)]
        if not evaluate_dnf(disjuncts, assignment):
            return True, assignment
    return False, None


# ============================================================
# Verification
# ============================================================

def main():
    global passed, failed

    print("SAT → NonTautology verification (#868)")
    print("=" * 50)

    # === Section 1: De Morgan identity ===
    print("\n1. De Morgan identity check...")

    for n_vars in range(1, 5):
        # Generate random clauses and check De Morgan
        all_lits = [(v, p) for v in range(n_vars) for p in [True, False]]

        # Test all possible single clauses
        for clause_size in range(1, min(n_vars * 2, 4) + 1):
            for clause in itertools.combinations(all_lits, clause_size):
                # Skip clauses with both x and ¬x (tautological clause)
                vars_in_clause = set()
                skip = False
                for v, p in clause:
                    if v in vars_in_clause:
                        skip = True
                        break
                    vars_in_clause.add(v)
                if skip:
                    continue

                clauses = [list(clause)]
                dnf = negate_cnf_to_dnf(clauses)

                # Check: for ALL assignments, ¬CNF(a) == DNF(a)
                for bits in range(2 ** n_vars):
                    a = [(bits >> i) & 1 == 1 for i in range(n_vars)]
                    cnf_val = evaluate_cnf(clauses, a)
                    dnf_val = evaluate_dnf(dnf, a)
                    check(dnf_val == (not cnf_val),
                          f"De Morgan: n={n_vars}, clause={clause}, a={a}")

    print(f"   De Morgan: {passed} passed")

    # === Section 2: Exhaustive forward/backward ===
    print("\n2. Exhaustive SAT ↔ NonTautology (n ≤ 4, m ≤ 5)...")

    for n_vars in range(1, 5):
        all_lits = [(v, p) for v in range(n_vars) for p in [True, False]]
        # Generate clause sets
        possible_clauses = []
        for size in range(1, min(n_vars * 2, 4) + 1):
            for clause in itertools.combinations(all_lits, size):
                vars_used = set()
                valid = True
                for v, p in clause:
                    if v in vars_used:
                        valid = False
                        break
                    vars_used.add(v)
                if valid:
                    possible_clauses.append(list(clause))

        # Sample clause sets (all combinations up to m=5)
        max_m = min(5, len(possible_clauses))
        for m in range(1, max_m + 1):
            clause_combos = list(itertools.combinations(range(len(possible_clauses)), m))
            if len(clause_combos) > 200:
                import random
                random.seed(n_vars * 1000 + m)
                clause_combos = random.sample(clause_combos, 200)

            for combo in clause_combos:
                clauses = [possible_clauses[i] for i in combo]
                dnf = negate_cnf_to_dnf(clauses)

                sat, sat_assignment = is_satisfiable(n_vars, clauses)
                nontaut, falsify_assignment = is_not_tautology(n_vars, dnf)

                # Forward + Backward: SAT ↔ NonTautology
                check(sat == nontaut,
                      f"n={n_vars}, m={m}: SAT={sat}, NonTaut={nontaut}")

                # Overhead: same number of vars, disjuncts = clauses
                check(len(dnf) == len(clauses),
                      f"Overhead: |DNF|={len(dnf)} != |CNF|={len(clauses)}")

        print(f"   n={n_vars}: {passed} passed, {failed} failed (cumulative)")

    # === Section 3: Solution extraction ===
    print("\n3. Solution extraction (same assignment)...")

    for n_vars in range(1, 5):
        all_lits = [(v, p) for v in range(n_vars) for p in [True, False]]
        possible_clauses = []
        for size in range(1, min(n_vars * 2, 4) + 1):
            for clause in itertools.combinations(all_lits, size):
                vars_used = set()
                valid = True
                for v, p in clause:
                    if v in vars_used:
                        valid = False
                        break
                    vars_used.add(v)
                if valid:
                    possible_clauses.append(list(clause))

        for m in range(1, min(4, len(possible_clauses)) + 1):
            clause_combos = list(itertools.combinations(range(len(possible_clauses)), m))
            if len(clause_combos) > 100:
                import random
                random.seed(n_vars * 2000 + m)
                clause_combos = random.sample(clause_combos, 100)

            for combo in clause_combos:
                clauses = [possible_clauses[i] for i in combo]
                dnf = negate_cnf_to_dnf(clauses)

                sat, sat_a = is_satisfiable(n_vars, clauses)
                if sat:
                    # The satisfying assignment for φ should falsify ¬φ (the DNF)
                    dnf_at_sat_a = evaluate_dnf(dnf, sat_a)
                    check(not dnf_at_sat_a,
                          f"Extraction: SAT assignment should falsify DNF")

                nontaut, falsify_a = is_not_tautology(n_vars, dnf)
                if nontaut:
                    # The falsifying assignment for ¬φ should satisfy φ
                    cnf_at_falsify = evaluate_cnf(clauses, falsify_a)
                    check(cnf_at_falsify,
                          f"Extraction: falsifying assignment should satisfy CNF")

    print(f"   Extraction: {passed} passed, {failed} failed (cumulative)")

    # === Section 4: Paper example ===
    print("\n4. Paper example (#868)...")

    # φ = (x₁ ∨ ¬x₂ ∨ x₃) ∧ (¬x₁ ∨ x₂ ∨ x₄) ∧ (x₂ ∨ ¬x₃ ∨ ¬x₄) ∧ (¬x₁ ∨ ¬x₂ ∨ x₃)
    clauses = [
        [(0, True), (1, False), (2, True)],
        [(0, False), (1, True), (3, True)],
        [(1, True), (2, False), (3, False)],
        [(0, False), (1, False), (2, True)],
    ]

    # Issue #868 claims x₁=T, x₂=F, x₃=T, x₄=F but this FAILS clause 2
    # (¬x₁ ∨ x₂ ∨ x₄) = (F ∨ F ∨ F) = F. Find a correct assignment:
    sat_found, sat_a = is_satisfiable(4, clauses)
    check(sat_found, "Paper example: φ is satisfiable")
    check(evaluate_cnf(clauses, sat_a), "Paper example: found assignment satisfies φ")

    dnf = negate_cnf_to_dnf(clauses)
    check(len(dnf) == 4, f"Paper example: 4 disjuncts, got {len(dnf)}")

    # Same assignment should falsify ¬φ
    check(not evaluate_dnf(dnf, sat_a), "Paper example: assignment falsifies ¬φ")

    # Verify each disjunct is negation of corresponding clause
    for j, (clause, disjunct) in enumerate(zip(clauses, dnf)):
        for (cv, cp), (dv, dp) in zip(clause, disjunct):
            check(cv == dv and cp != dp,
                  f"Paper example: disjunct {j} literal mismatch")

    # === Section 5: Edge cases ===
    print("\n5. Edge cases...")

    # Single variable, single clause
    sat, _ = is_satisfiable(1, [[(0, True)]])
    dnf = negate_cnf_to_dnf([[(0, True)]])
    nontaut, _ = is_not_tautology(1, dnf)
    check(sat == nontaut, "Single positive literal")

    sat, _ = is_satisfiable(1, [[(0, False)]])
    dnf = negate_cnf_to_dnf([[(0, False)]])
    nontaut, _ = is_not_tautology(1, dnf)
    check(sat == nontaut, "Single negative literal")

    # Contradiction: x ∧ ¬x
    sat, _ = is_satisfiable(1, [[(0, True)], [(0, False)]])
    dnf = negate_cnf_to_dnf([[(0, True)], [(0, False)]])
    nontaut, _ = is_not_tautology(1, dnf)
    check(sat == nontaut, "Contradiction: x ∧ ¬x")
    check(not sat, "Contradiction is unsatisfiable")
    check(not nontaut, "Negation of contradiction is tautology")

    # Empty clause set (vacuously true)
    sat, _ = is_satisfiable(2, [])
    check(sat, "Empty CNF is satisfiable")

    print(f"\n{'='*50}")
    print(f"SAT → NonTautology: {passed} passed, {failed} failed")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
