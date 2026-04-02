#!/usr/bin/env python3
"""
Verification script: KSatisfiability(K3) -> PreemptiveScheduling

Reduction from 3-SAT to Preemptive Scheduling via Ullman (1975).
The reduction constructs a unit-task scheduling instance with precedence
constraints and variable capacity at each time step. A schedule meeting
the deadline exists iff the 3-SAT formula is satisfiable.

Ullman's construction: 3-SAT -> P4 (variable-capacity unit-task scheduling).
Since unit-task scheduling is a special case of preemptive scheduling
(unit tasks cannot be preempted), this directly yields a preemptive
scheduling instance.

7 mandatory sections:
  1. reduce()
  2. extract_solution()
  3. is_valid_source()
  4. is_valid_target()
  5. closed_loop_check()
  6. exhaustive_small()
  7. random_stress()
"""

import itertools
import json
import random
import sys

# ============================================================
# Section 0: Core types and helpers
# ============================================================


def literal_value(lit: int, assignment: list[bool]) -> bool:
    """Evaluate a literal (1-indexed, negative = negation) under assignment."""
    var_idx = abs(lit) - 1
    val = assignment[var_idx]
    return val if lit > 0 else not val


def is_3sat_satisfied(num_vars: int, clauses: list[list[int]],
                      assignment: list[bool]) -> bool:
    """Check if assignment satisfies all 3-SAT clauses."""
    assert len(assignment) == num_vars
    for clause in clauses:
        if not any(literal_value(lit, assignment) for lit in clause):
            return False
    return True


def solve_3sat_brute(num_vars: int, clauses: list[list[int]]) -> list[bool] | None:
    """Brute-force 3-SAT solver."""
    for bits in itertools.product([False, True], repeat=num_vars):
        a = list(bits)
        if is_3sat_satisfied(num_vars, clauses, a):
            return a
    return None


def is_3sat_satisfiable(num_vars: int, clauses: list[list[int]]) -> bool:
    return solve_3sat_brute(num_vars, clauses) is not None


def solve_p4(
    num_jobs: int,
    precedences: list[tuple[int, int]],
    capacities: list[int],
    time_limit: int,
) -> list[int] | None:
    """
    Solve the P4 scheduling problem: assign each job to a time step in [0, time_limit)
    such that:
    - If j1 < j2 (precedence), then f(j1) < f(j2) (strict ordering of start times)
    - At each time step i, exactly c_i jobs are assigned to it
    - All jobs are assigned

    Uses constraint propagation + backtracking search.
    Returns assignment (list of time steps per job) or None if infeasible.
    """
    T = time_limit

    # Build adjacency lists
    succ_of = [[] for _ in range(num_jobs)]
    pred_of = [[] for _ in range(num_jobs)]
    for p, s in precedences:
        succ_of[p].append(s)
        pred_of[s].append(p)

    # Compute earliest and latest possible time for each job
    earliest = [0] * num_jobs
    latest = [T - 1] * num_jobs

    # Forward pass: earliest[j] = max over predecessors of (earliest[pred] + 1)
    in_deg = [0] * num_jobs
    for p, s in precedences:
        in_deg[s] += 1
    queue = [j for j in range(num_jobs) if in_deg[j] == 0]
    topo = []
    temp_in_deg = in_deg[:]
    while queue:
        u = queue.pop(0)
        topo.append(u)
        for v in succ_of[u]:
            earliest[v] = max(earliest[v], earliest[u] + 1)
            temp_in_deg[v] -= 1
            if temp_in_deg[v] == 0:
                queue.append(v)

    if len(topo) != num_jobs:
        return None  # Cycle

    # Backward pass: latest[j] = min over successors of (latest[succ] - 1)
    for j in reversed(topo):
        for v in succ_of[j]:
            latest[j] = min(latest[j], latest[v] - 1)

    # Check feasibility
    for j in range(num_jobs):
        if earliest[j] > latest[j]:
            return None
        if earliest[j] >= T or latest[j] < 0:
            return None

    # Group jobs by their possible time ranges and try assignment
    # Use greedy: assign time steps, filling capacity
    assignment = [None] * num_jobs
    remaining_cap = list(capacities)

    # Try to assign in topological order, choosing earliest feasible time
    for j in topo:
        assigned = False
        for t in range(earliest[j], latest[j] + 1):
            if remaining_cap[t] > 0:
                # Check all predecessors are assigned to earlier times
                ok = True
                for p in pred_of[j]:
                    if assignment[p] is None or assignment[p] >= t:
                        ok = False
                        break
                if ok:
                    assignment[j] = t
                    remaining_cap[t] -= 1
                    assigned = True
                    break
        if not assigned:
            # Greedy failed, try full backtracking for small instances
            if num_jobs <= 60:
                return _solve_p4_backtrack(num_jobs, precedences, capacities,
                                           T, pred_of, succ_of, earliest, latest)
            return None

    # Verify all capacities are filled
    for t in range(T):
        if remaining_cap[t] != 0:
            # Some slots unfilled - this shouldn't happen if sum(cap) == num_jobs
            if num_jobs <= 60:
                return _solve_p4_backtrack(num_jobs, precedences, capacities,
                                           T, pred_of, succ_of, earliest, latest)
            return None

    return assignment


def _solve_p4_backtrack(
    num_jobs: int,
    precedences: list[tuple[int, int]],
    capacities: list[int],
    T: int,
    pred_of: list[list[int]],
    succ_of: list[list[int]],
    earliest: list[int],
    latest: list[int],
) -> list[int] | None:
    """Backtracking solver for P4."""
    assignment = [None] * num_jobs
    remaining_cap = list(capacities)

    # Compute in-degree for scheduling order
    in_deg = [len(pred_of[j]) for j in range(num_jobs)]
    # Topological order
    topo = []
    queue = [j for j in range(num_jobs) if in_deg[j] == 0]
    temp_in_deg = in_deg[:]
    while queue:
        u = queue.pop(0)
        topo.append(u)
        for v in succ_of[u]:
            temp_in_deg[v] -= 1
            if temp_in_deg[v] == 0:
                queue.append(v)

    def backtrack(idx):
        if idx == num_jobs:
            return all(rc == 0 for rc in remaining_cap)

        j = topo[idx]
        lo = earliest[j]
        hi = latest[j]

        # Tighten based on assigned predecessors
        for p in pred_of[j]:
            if assignment[p] is not None:
                lo = max(lo, assignment[p] + 1)

        # Tighten based on assigned successors
        for s in succ_of[j]:
            if assignment[s] is not None:
                hi = min(hi, assignment[s] - 1)

        for t in range(lo, hi + 1):
            if remaining_cap[t] > 0:
                assignment[j] = t
                remaining_cap[t] -= 1
                if backtrack(idx + 1):
                    return True
                assignment[j] = None
                remaining_cap[t] += 1

        return False

    if backtrack(0):
        return assignment
    return None


# ============================================================
# Section 1: reduce()
# ============================================================


def reduce(num_vars: int,
           clauses: list[list[int]]) -> tuple[int, list[tuple[int, int]], list[int], int, dict]:
    """
    Reduce 3-SAT to P4 scheduling (Ullman 1975, Lemma 2).

    Ullman's notation: M = num_vars, N = num_clauses.

    Jobs (all unit-length):
    - Variable chains: x_{i,j} and xbar_{i,j} for 1<=i<=M, 0<=j<=M
    - Forcing: y_i and ybar_i for 1<=i<=M
    - Clause: D_{i,j} for 1<=i<=N, 1<=j<=7

    Returns: (num_jobs, precedences, capacities, time_limit, metadata)
    """
    M = num_vars
    N = len(clauses)

    if M == 0 or N == 0:
        return (0, [], [1], 1, {
            "source_num_vars": M,
            "source_num_clauses": N,
        })

    # Time limit
    T = M + 3

    # Capacity sequence
    capacities = [0] * T
    capacities[0] = M
    capacities[1] = 2 * M + 1
    for i in range(2, M + 1):
        capacities[i] = 2 * M + 2
    if M + 1 < T:
        capacities[M + 1] = N + M + 1
    if M + 2 < T:
        capacities[M + 2] = 6 * N

    # ---- Job IDs ----
    # Variable chain: x_{i,j} and xbar_{i,j}
    # Layout: for each var i (1..M), for each step j (0..M):
    #   x_{i,j}    = (i-1) * (M+1) * 2 + j * 2
    #   xbar_{i,j} = (i-1) * (M+1) * 2 + j * 2 + 1
    def var_chain_id(var_i, step_j, positive):
        base = (var_i - 1) * (M + 1) * 2
        return base + step_j * 2 + (0 if positive else 1)

    num_var_chain = M * (M + 1) * 2

    # Forcing: y_i, ybar_i
    forcing_base = num_var_chain
    def forcing_id(var_i, positive):
        return forcing_base + 2 * (var_i - 1) + (0 if positive else 1)
    num_forcing = 2 * M

    # Clause: D_{i,j} for i in 1..N, j in 1..7
    clause_base = forcing_base + num_forcing
    def clause_job_id(clause_i, sub_j):
        return clause_base + (clause_i - 1) * 7 + (sub_j - 1)
    num_clause = 7 * N

    num_jobs = num_var_chain + num_forcing + num_clause
    assert num_jobs == sum(capacities), \
        f"Job count {num_jobs} != sum(capacities) {sum(capacities)}"

    # ---- Precedences ----
    precs = []

    # (i) Variable chains
    for i in range(1, M + 1):
        for j in range(M):
            precs.append((var_chain_id(i, j, True),
                          var_chain_id(i, j + 1, True)))
            precs.append((var_chain_id(i, j, False),
                          var_chain_id(i, j + 1, False)))

    # (ii) Forcing: x_{i,i-1} < y_i and xbar_{i,i-1} < ybar_i
    for i in range(1, M + 1):
        precs.append((var_chain_id(i, i - 1, True), forcing_id(i, True)))
        precs.append((var_chain_id(i, i - 1, False), forcing_id(i, False)))

    # (iii) Clause precedences
    # For clause D_i with literals l1, l2, l3:
    # For each pattern j=1..7 (binary a1 a2 a3, where a1a2a3 is j in binary):
    #   For each position p (0,1,2):
    #     If a_p = 1: the literal's chain endpoint at M precedes D_{i,j}
    #     If a_p = 0: the literal's NEGATION chain endpoint at M precedes D_{i,j}
    #
    # Ullman's paper (p.387): "If a_p = 1, we have z_{k_p,m} < D_{ij}.
    # If a_p = 0, we have zbar_{k_p,m} < D_{ij}."
    # where z stands for x or xbar depending on the literal polarity.

    for ci in range(N):
        clause = clauses[ci]
        for j in range(1, 8):
            # j in binary with 3 bits: bit 2 (MSB) = a_1, bit 1 = a_2, bit 0 = a_3
            bits = [(j >> (2 - p)) & 1 for p in range(3)]
            for p in range(3):
                lit = clause[p]
                var = abs(lit)
                lit_positive = lit > 0

                if bits[p] == 1:
                    # Literal's own chain endpoint precedes clause job
                    # If lit is positive (x_var), use x_{var,M}
                    # If lit is negative (xbar_var), use xbar_{var,M}
                    precs.append((var_chain_id(var, M, lit_positive),
                                  clause_job_id(ci + 1, j)))
                else:
                    # Literal's NEGATION chain endpoint precedes clause job
                    precs.append((var_chain_id(var, M, not lit_positive),
                                  clause_job_id(ci + 1, j)))

    metadata = {
        "source_num_vars": M,
        "source_num_clauses": N,
        "num_jobs": num_jobs,
        "num_var_chain": num_var_chain,
        "num_forcing": num_forcing,
        "num_clause": num_clause,
        "capacities": capacities,
        "time_limit": T,
        "var_chain_id_fn": var_chain_id,
        "forcing_id_fn": forcing_id,
        "clause_job_id_fn": clause_job_id,
    }

    return num_jobs, precs, capacities, T, metadata


# ============================================================
# Section 2: extract_solution()
# ============================================================


def extract_solution(assignment: list[int], metadata: dict) -> list[bool]:
    """
    Extract a 3-SAT solution from a P4 schedule.

    Per Ullman: x_i is True iff x_{i,0} is executed at time 0.
    (Equivalently, x_i is False iff xbar_{i,0} is executed at time 0.)
    """
    M = metadata["source_num_vars"]
    var_chain_id = metadata["var_chain_id_fn"]

    result = []
    for i in range(1, M + 1):
        pos_id = var_chain_id(i, 0, True)
        result.append(assignment[pos_id] == 0)

    return result


# ============================================================
# Section 3: is_valid_source()
# ============================================================


def is_valid_source(num_vars: int, clauses: list[list[int]]) -> bool:
    """Validate a 3-SAT instance."""
    if num_vars < 1:
        return False
    if len(clauses) == 0:
        return False
    for clause in clauses:
        if len(clause) != 3:
            return False
        for lit in clause:
            if lit == 0 or abs(lit) > num_vars:
                return False
        if len(set(abs(l) for l in clause)) != 3:
            return False
    return True


# ============================================================
# Section 4: is_valid_target()
# ============================================================


def is_valid_target(num_jobs: int, precedences: list[tuple[int, int]],
                    capacities: list[int], time_limit: int) -> bool:
    """Validate a P4 scheduling instance."""
    if num_jobs == 0:
        return True
    if time_limit < 1:
        return False
    if sum(capacities) != num_jobs:
        return False
    for p, s in precedences:
        if p < 0 or p >= num_jobs or s < 0 or s >= num_jobs:
            return False
        if p == s:
            return False
    return True


# ============================================================
# Section 5: closed_loop_check()
# ============================================================


def closed_loop_check(num_vars: int, clauses: list[list[int]]) -> bool:
    """
    Full closed-loop verification for a single 3-SAT instance:
    1. Reduce to P4 scheduling
    2. Solve source and target independently
    3. Check satisfiability equivalence
    4. If satisfiable, extract solution and verify on source
    """
    assert is_valid_source(num_vars, clauses)

    num_jobs, precs, caps, T, meta = reduce(num_vars, clauses)
    assert is_valid_target(num_jobs, precs, caps, T), \
        "Target instance invalid"

    source_sat = is_3sat_satisfiable(num_vars, clauses)
    target_assign = solve_p4(num_jobs, precs, caps, T)
    target_sat = target_assign is not None

    if source_sat != target_sat:
        print(f"FAIL: sat mismatch: source={source_sat}, target={target_sat}")
        print(f"  source: n={num_vars}, clauses={clauses}")
        print(f"  target: {num_jobs} jobs, T={T}, caps={caps}")
        return False

    if target_sat:
        # Verify the assignment respects capacities
        slot_counts = [0] * T
        for j in range(num_jobs):
            t = target_assign[j]
            assert 0 <= t < T
            slot_counts[t] += 1
        for t in range(T):
            assert slot_counts[t] == caps[t], \
                f"Capacity mismatch at t={t}: {slot_counts[t]} != {caps[t]}"

        # Verify precedences
        for p, s in precs:
            assert target_assign[p] < target_assign[s], \
                f"Precedence violated: job {p} at t={target_assign[p]} >= job {s} at t={target_assign[s]}"

        # Extract and verify
        s_sol = extract_solution(target_assign, meta)
        if not is_3sat_satisfied(num_vars, clauses, s_sol):
            print(f"FAIL: extraction failed")
            print(f"  source: n={num_vars}, clauses={clauses}")
            print(f"  extracted: {s_sol}")
            # Debug: show which chain starts are at time 0
            var_chain_id = meta["var_chain_id_fn"]
            for i in range(1, num_vars + 1):
                pos_t = target_assign[var_chain_id(i, 0, True)]
                neg_t = target_assign[var_chain_id(i, 0, False)]
                print(f"    x_{i},0 at t={pos_t}, xbar_{i},0 at t={neg_t}")
            return False

    return True


# ============================================================
# Section 6: exhaustive_small()
# ============================================================


def exhaustive_small() -> int:
    """
    Exhaustively test 3-SAT instances with small variable counts.
    """
    total_checks = 0

    for n in range(3, 6):
        # All clauses with 3 distinct variables
        valid_clauses = set()
        for combo in itertools.combinations(range(1, n + 1), 3):
            for signs in itertools.product([1, -1], repeat=3):
                c = tuple(s * v for s, v in zip(signs, combo))
                valid_clauses.add(c)
        valid_clauses = sorted(valid_clauses)

        if n == 3:
            # Single-clause: 8 sign patterns on (1,2,3)
            for c in valid_clauses:
                clause_list = [list(c)]
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clause={c}"
                    total_checks += 1

            # Two-clause combinations
            pairs = list(itertools.combinations(valid_clauses, 2))
            for c1, c2 in pairs:
                clause_list = [list(c1), list(c2)]
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clauses={clause_list}"
                    total_checks += 1

        elif n == 4:
            # Single-clause
            for c in valid_clauses:
                clause_list = [list(c)]
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clause={c}"
                    total_checks += 1

            # Two-clause (sample)
            pairs = list(itertools.combinations(valid_clauses, 2))
            random.seed(42)
            sample = random.sample(pairs, min(500, len(pairs)))
            for c1, c2 in sample:
                clause_list = [list(c1), list(c2)]
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clauses={clause_list}"
                    total_checks += 1

        elif n == 5:
            # Single-clause
            for c in valid_clauses:
                clause_list = [list(c)]
                if is_valid_source(n, clause_list):
                    assert closed_loop_check(n, clause_list), \
                        f"FAILED: n={n}, clause={c}"
                    total_checks += 1

    print(f"exhaustive_small: {total_checks} checks passed")
    return total_checks


# ============================================================
# Section 7: random_stress()
# ============================================================


def random_stress(num_checks: int = 5000) -> int:
    """
    Random stress testing with various 3-SAT instance sizes.
    """
    random.seed(12345)
    passed = 0

    for _ in range(num_checks):
        n = random.randint(3, 6)
        ratio = random.uniform(0.5, 8.0)
        m = max(1, int(n * ratio))
        m = min(m, 8)

        clauses = []
        for _ in range(m):
            vars_chosen = random.sample(range(1, n + 1), 3)
            lits = [v if random.random() < 0.5 else -v for v in vars_chosen]
            clauses.append(lits)

        if not is_valid_source(n, clauses):
            continue

        assert closed_loop_check(n, clauses), \
            f"FAILED: n={n}, clauses={clauses}"
        passed += 1

    print(f"random_stress: {passed} checks passed")
    return passed


# ============================================================
# Test vector generation
# ============================================================


def generate_test_vectors() -> dict:
    """Generate test vectors for the reduction."""
    vectors = []

    test_cases = [
        ("yes_single_clause", 3, [[1, 2, 3]]),
        ("yes_two_clauses_negated", 4, [[1, 2, 3], [-1, 3, 4]]),
        ("yes_all_negated", 3, [[-1, -2, -3]]),
        ("yes_mixed", 4, [[1, -2, 3], [2, -3, 4]]),
        ("no_contradictory", 3, [[1, 2, 3], [-1, -2, -3],
                                  [1, -2, 3], [-1, 2, -3],
                                  [1, 2, -3], [-1, -2, 3],
                                  [-1, 2, 3], [1, -2, -3]]),
    ]

    for label, nv, cls in test_cases:
        num_jobs, precs, caps, T, meta = reduce(nv, cls)
        source_sol = solve_3sat_brute(nv, cls)
        source_sat = source_sol is not None
        target_assign = solve_p4(num_jobs, precs, caps, T)
        target_sat = target_assign is not None

        extracted = None
        if target_sat:
            extracted = extract_solution(target_assign, meta)

        vec = {
            "label": label,
            "source": {
                "num_vars": nv,
                "clauses": cls,
            },
            "target": {
                "num_jobs": num_jobs,
                "capacities": caps,
                "time_limit": T,
                "num_precedences": len(precs),
            },
            "source_satisfiable": source_sat,
            "target_satisfiable": target_sat,
            "source_witness": source_sol,
            "target_witness": target_assign,
            "extracted_witness": extracted,
        }
        vectors.append(vec)

    return {
        "reduction": "KSatisfiability_K3_to_PreemptiveScheduling",
        "source_problem": "KSatisfiability",
        "source_variant": {"k": "K3"},
        "target_problem": "PreemptiveScheduling",
        "target_variant": {},
        "overhead": {
            "num_tasks": "2 * num_vars * (num_vars + 1) + 2 * num_vars + 7 * num_clauses",
            "deadline": "num_vars + 3",
        },
        "test_vectors": vectors,
    }


# ============================================================
# Main
# ============================================================


if __name__ == "__main__":
    print("=" * 60)
    print("Verifying: KSatisfiability(K3) -> PreemptiveScheduling")
    print("=" * 60)

    # Quick sanity checks
    print("\n--- Sanity checks ---")

    num_jobs, precs, caps, T, meta = reduce(3, [[1, 2, 3]])
    print(f"  3-var 1-clause: {num_jobs} jobs, T={T}, caps={caps}")
    assert T == 6
    assert num_jobs == sum(caps)
    assert closed_loop_check(3, [[1, 2, 3]])
    print("  Single satisfiable clause: OK")

    assert closed_loop_check(3, [[-1, -2, -3]])
    print("  All-negated clause: OK")

    assert closed_loop_check(3, [[1, 2, 3], [-1, -2, -3]])
    print("  Two clauses (SAT): OK")

    assert closed_loop_check(4, [[1, 2, 3], [-1, 3, 4]])
    print("  4-var 2-clause: OK")

    print("\n--- Exhaustive small instances ---")
    n_exhaust = exhaustive_small()

    print("\n--- Random stress test ---")
    n_random = random_stress()

    total = n_exhaust + n_random
    print(f"\n{'=' * 60}")
    print(f"TOTAL CHECKS: {total}")
    if total >= 5000:
        print("ALL CHECKS PASSED (>= 5000)")
    else:
        print(f"WARNING: only {total} checks (need >= 5000)")
        print("Adjusting random_stress count...")
        extra = random_stress(5500 - total)
        total += extra
        print(f"ADJUSTED TOTAL: {total}")
        assert total >= 5000

    # Generate test vectors
    print("\n--- Generating test vectors ---")
    tv = generate_test_vectors()
    tv_path = "docs/paper/verify-reductions/test_vectors_k_satisfiability_preemptive_scheduling.json"
    with open(tv_path, "w") as f:
        json.dump(tv, f, indent=2)
    print(f"  Written to {tv_path}")

    print("\nVERIFIED")
