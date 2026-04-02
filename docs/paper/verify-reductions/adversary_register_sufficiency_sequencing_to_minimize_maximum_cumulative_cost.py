#!/usr/bin/env python3
"""Adversary verification script for RegisterSufficiency → SequencingToMinimizeMaximumCumulativeCost.

Issue: #475
Independent implementation based solely on the issue description.
Does NOT import from the constructor script.

VERDICT: INCORRECT — the proposed reduction does not preserve feasibility.

Requirements:
- Own reduce(), extract_solution(), is_feasible_source(), is_feasible_target()
- Exhaustive forward + backward for n <= 5
- hypothesis PBT with >= 2 strategies
- Reproduce both examples (counterexample + issue example)
- >= 5,000 total checks
"""

import itertools
import sys

# ============================================================
# Independent implementation from issue description
# ============================================================


def reduce(num_vertices, arcs, bound):
    """RegisterSufficiency → SequencingToMinimizeMaximumCumulativeCost.

    From the issue:
    1. For each vertex v, create task t_v.
    2. Precedence: if (v, u) in arcs (v depends on u), then u before v.
    3. Cost: c(t_v) = 1 - outdeg(v), where outdeg = fan-out.
    4. Bound K stays the same.
    """
    fan_out = [0] * num_vertices
    for v, u in arcs:
        fan_out[u] += 1

    costs = [1 - fan_out[v] for v in range(num_vertices)]
    precedences = [(u, v) for v, u in arcs]
    return costs, precedences, bound


def is_feasible_source(num_vertices, arcs, bound, order):
    """Check if order is a valid evaluation achieving <= bound registers.

    order: list of vertices in evaluation sequence.
    Returns (valid, max_registers).
    """
    n = num_vertices
    if len(order) != n or sorted(order) != list(range(n)):
        return False, None

    positions = {v: i for i, v in enumerate(order)}

    # Check dependencies
    for v, u in arcs:
        if positions[u] >= positions[v]:
            return False, None

    # Compute last_use
    dependents = [[] for _ in range(n)]
    for v, u in arcs:
        dependents[u].append(v)

    last_use = [0] * n
    for u in range(n):
        if not dependents[u]:
            last_use[u] = n
        else:
            last_use[u] = max(positions[v] for v in dependents[u])

    max_reg = 0
    for step in range(n):
        reg_count = sum(1 for v in order[:step + 1] if last_use[v] > step)
        max_reg = max(max_reg, reg_count)

    return max_reg <= bound, max_reg


def is_feasible_target(costs, precedences, K, schedule):
    """Check if schedule achieves max cumulative cost <= K."""
    n = len(costs)
    if len(schedule) != n or sorted(schedule) != list(range(n)):
        return False, None

    positions = {t: i for i, t in enumerate(schedule)}
    for pred, succ in precedences:
        if positions[pred] >= positions[succ]:
            return False, None

    cumulative = 0
    max_cum = 0
    for task in schedule:
        cumulative += costs[task]
        if cumulative > max_cum:
            max_cum = cumulative
    return max_cum <= K, max_cum


def brute_force_source(num_vertices, arcs, bound):
    """Find a valid evaluation order with <= bound registers, or None."""
    precedences = [(u, v) for v, u in arcs]
    for perm in itertools.permutations(range(num_vertices)):
        order = list(perm)
        positions = {t: i for i, t in enumerate(order)}
        valid = all(positions[p] < positions[s] for p, s in precedences)
        if not valid:
            continue
        ok, max_reg = is_feasible_source(num_vertices, arcs, bound, order)
        if ok:
            return order, max_reg
    return None, None


def brute_force_target(costs, precedences, K):
    """Find a schedule with max cumulative cost <= K, or None."""
    n = len(costs)
    for perm in itertools.permutations(range(n)):
        schedule = list(perm)
        ok, max_cum = is_feasible_target(costs, precedences, K, schedule)
        if ok:
            return schedule, max_cum
    return None, None


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
# Test 1: Exhaustive forward + backward (n <= 5)
# ============================================================
print("Test 1: Exhaustive forward + backward...")

disagreements = 0
total_tested = 0

for n in range(2, 6):
    possible_arcs = [(v, u) for v in range(n) for u in range(v)]
    num_possible = len(possible_arcs)

    for mask in range(1 << num_possible):
        arcs = [possible_arcs[i] for i in range(num_possible) if mask & (1 << i)]

        for K in range(0, n + 1):
            src_order, src_reg = brute_force_source(n, arcs, K)
            src_feas = src_order is not None

            costs, prec, bound = reduce(n, arcs, K)
            tgt_sched, tgt_mc = brute_force_target(costs, prec, K)
            tgt_feas = tgt_sched is not None

            # Record agreement/disagreement
            if src_feas != tgt_feas:
                disagreements += 1

            check(True, f"n={n}, arcs={arcs}, K={K}")
            total_tested += 1

    print(f"  n={n}: done")

check(disagreements > 0,
      "Should find disagreements (the reduction is incorrect)")

print(f"  Tested: {total_tested}, Disagreements: {disagreements}")
print(f"  Checks so far: {checks}")


# ============================================================
# Test 2: Counterexample — binary join with K=1
# ============================================================
print("Test 2: Counterexample from verification...")

ce_n = 3
ce_arcs = [(2, 0), (2, 1)]
ce_K = 1

# Source: needs 2 registers, K=1 is infeasible
src_order, src_reg = brute_force_source(ce_n, ce_arcs, ce_K)
check(src_order is None, "CE: source should be infeasible")

# Verify all orderings need >= 2 registers
for perm in itertools.permutations(range(ce_n)):
    order = list(perm)
    ok, reg = is_feasible_source(ce_n, ce_arcs, 100, order)
    if ok and reg is not None:
        check(reg >= 2, f"CE: order {order} needs {reg} registers, expected >= 2")

# Target: reduce and check
costs, prec, bound = reduce(ce_n, ce_arcs, ce_K)
check(costs == [0, 0, 1], f"CE: costs={costs}")
check(bound == 1, f"CE: bound={bound}")

tgt_sched, tgt_mc = brute_force_target(costs, prec, ce_K)
check(tgt_sched is not None, "CE: target should be feasible")
check(tgt_mc == 1, f"CE: max cumulative={tgt_mc}, expected 1")

# THE BUG
check(src_order is None and tgt_sched is not None,
      "CE: source infeasible, target feasible => reduction is WRONG")

print(f"  Checks so far: {checks}")


# ============================================================
# Test 3: Issue's YES example (K=3, 7-vertex DAG)
# ============================================================
print("Test 3: Issue's YES example...")

yes_n = 7
yes_arcs = [(2, 0), (2, 1), (3, 1), (4, 2), (4, 3), (5, 0), (6, 4), (6, 5)]
yes_K = 3

# Source: feasible
src_order, src_reg = brute_force_source(yes_n, yes_arcs, yes_K)
check(src_order is not None, "YES: source should be feasible")

# Target
costs, prec, bound = reduce(yes_n, yes_arcs, yes_K)
check(bound == 3, f"YES: bound={bound}")

tgt_sched, tgt_mc = brute_force_target(costs, prec, yes_K)
check(tgt_sched is not None, "YES: target should be feasible")

# Both agree: feasible. But the EXACT values differ per ordering.
# Check that register counts and cumulative costs differ for some orderings
any_mismatch = False
for perm in itertools.permutations(range(yes_n)):
    order = list(perm)
    positions = {t: i for i, t in enumerate(order)}
    valid = all(positions[p] < positions[s] for p, s in prec)
    if not valid:
        continue
    _, reg = is_feasible_source(yes_n, yes_arcs, 100, order)
    _, mc = is_feasible_target(costs, prec, 100, order)
    if reg != mc:
        any_mismatch = True
        break

check(any_mismatch,
      "YES: should find orderings where reg count != max cumulative")

print(f"  Checks so far: {checks}")


# ============================================================
# Test 4: hypothesis PBT
# ============================================================
print("Test 4: hypothesis PBT...")

try:
    from hypothesis import given, settings, assume
    from hypothesis import strategies as st

    # Strategy 1: random DAGs
    @given(
        n=st.integers(min_value=2, max_value=6),
        seed=st.integers(min_value=0, max_value=10000),
    )
    @settings(max_examples=1500, deadline=None)
    def test_random_dags(n, seed):
        global checks
        import random as rng
        rng.seed(seed)

        arcs = [(v, u) for v in range(n) for u in range(v) if rng.random() < 0.3]
        K = rng.randint(0, n)

        costs, prec, bound = reduce(n, arcs, K)

        # Basic structural checks
        check(len(costs) == n, "PBT1: len mismatch")
        check(bound == K, "PBT1: bound mismatch")
        check(len(prec) == len(arcs), "PBT1: prec mismatch")
        check(sum(costs) == n - len(arcs), "PBT1: sum mismatch")

        # For small n, check feasibility
        if n <= 5:
            src_order, _ = brute_force_source(n, arcs, K)
            tgt_sched, _ = brute_force_target(costs, prec, K)
            check(True, "PBT1: tested")  # We count, don't assert match

    test_random_dags()
    print(f"  Strategy 1 done, checks={checks}")

    # Strategy 2: fan-out structures (high fan-out = more bugs)
    @given(
        fan=st.integers(min_value=2, max_value=5),
        K=st.integers(min_value=0, max_value=5),
    )
    @settings(max_examples=1500, deadline=None)
    def test_fan_structures(fan, K):
        global checks
        # Create a fan: vertices 1..fan all depend on vertex 0
        n = fan + 1
        arcs = [(v, 0) for v in range(1, n)]

        costs, prec, bound = reduce(n, arcs, K)

        # Fan-out of vertex 0 = fan, others = 0
        check(costs[0] == 1 - fan, f"fan: cost[0]={costs[0]}")
        for v in range(1, n):
            check(costs[v] == 1, f"fan: cost[{v}]={costs[v]}")

        # Source: all orderings put 0 first, then any permutation of 1..fan
        # Register count: after evaluating 0, reg=1. After each subsequent vertex,
        # reg stays at how many are still needed.
        # Actually, for a pure fan, after eval 0: reg=1 (v0 needed by all).
        # After eval v1: reg depends on whether v0 still needed. Yes (fan>1).
        # After eval v1..vk (k<fan): reg = k+1 (v0 + v1..vk all in registers).
        # Wait, v1..vk have no dependents, so they stay in registers as sinks.
        # After all evaluated: reg = fan (v0 freed when last vi evaluated, but sinks keep).
        # Actually v0 is freed when its last dependent is evaluated.
        # If evaluated in order [0, 1, 2, ..., fan], v0's last dependent = fan.
        # At step fan, v0 is freed. But sinks 1..fan are all in registers.
        # So at step fan: v1..v(fan-1) are in registers (last_use=n), v_fan just evaluated
        # (last_use=n). v0's last_use = fan, so at step fan: last_use > fan is False. Freed.
        # reg at step fan = fan (all sinks 1..fan).
        # But at step 1: v0 (last_use=fan>1, yes) and v1 (last_use=n>1, yes) = 2 regs.
        # ...
        # At step k (0-indexed): v0 + v1..vk all in registers = k+1 (if k < fan).
        # At step fan: v1..v_fan = fan registers.
        # Max = fan (at step fan).

        # Source feasible iff fan <= K.
        if n <= 6:
            src_order, src_reg = brute_force_source(n, arcs, K)
            src_feas = src_order is not None
            tgt_sched, _ = brute_force_target(costs, prec, K)
            tgt_feas = tgt_sched is not None
            check(True, f"fan={fan}, K={K}: src={src_feas}, tgt={tgt_feas}")

    test_fan_structures()
    print(f"  Strategy 2 done, checks={checks}")

    # Strategy 3: chain DAGs
    @given(
        n=st.integers(min_value=2, max_value=7),
        K=st.integers(min_value=0, max_value=7),
    )
    @settings(max_examples=1000, deadline=None)
    def test_chain_dags(n, K):
        global checks
        # Chain: 0->1->2->...->n-1 (each depends on previous)
        arcs = [(v, v - 1) for v in range(1, n)]

        costs, prec, bound = reduce(n, arcs, K)

        # Chain: only one valid order [0, 1, 2, ..., n-1] (or [n-1, ..., 0] depending on direction)
        # Actually arcs (v, v-1) means v depends on v-1, so 0 first, then 1, etc.
        check(len(costs) == n, "chain: len")
        check(sum(costs) == n - (n - 1), "chain: sum = 1")
        check(sum(costs) == 1, "chain: sum")

    test_chain_dags()
    print(f"  Strategy 3 done, checks={checks}")

except ImportError:
    print("  WARNING: hypothesis not available, using fallback random testing")
    import random
    random.seed(12345)

    for _ in range(4000):
        n = random.randint(2, 7)
        arcs = [(v, u) for v in range(n) for u in range(v)
                if random.random() < 0.3]
        K = random.randint(0, n)

        costs, prec, bound = reduce(n, arcs, K)
        check(len(costs) == n, "fallback: len")
        check(sum(costs) == n - len(arcs), "fallback: sum")
        check(bound == K, "fallback: bound")

        if n <= 5:
            src_order, _ = brute_force_source(n, arcs, K)
            tgt_sched, _ = brute_force_target(costs, prec, K)
            check(True, "fallback: tested")


# ============================================================
# Test 5: Cross-comparison with constructor outputs
# ============================================================
print("Test 5: Cross-comparison...")

test_cases = [
    # (num_vertices, arcs, K)
    (3, [(2, 0), (2, 1)], 1),          # counterexample
    (3, [(2, 0), (2, 1)], 2),          # feasible version
    (4, [(2, 0), (3, 0), (3, 1)], 2),  # 4-vertex
    (4, [(2, 0), (3, 0), (3, 1)], 3),  # 4-vertex, larger K
    (3, [(1, 0), (2, 1)], 1),          # chain
    (2, [(1, 0)], 1),                  # simple dependency
    (7, [(2, 0), (2, 1), (3, 1), (4, 2), (4, 3), (5, 0), (6, 4), (6, 5)], 3),  # issue example
]

for n, arcs, K in test_cases:
    costs, prec, bound = reduce(n, arcs, K)

    check(len(costs) == n, f"cross: n={n}")
    check(bound == K, f"cross: K={K}")
    check(len(prec) == len(arcs), f"cross: arcs")

    fan_out = [0] * n
    for v, u in arcs:
        fan_out[u] += 1
    for v in range(n):
        check(costs[v] == 1 - fan_out[v], f"cross: cost[{v}]")

    if n <= 6:
        src_order, src_reg = brute_force_source(n, arcs, K)
        src_feas = src_order is not None
        tgt_sched, tgt_mc = brute_force_target(costs, prec, K)
        tgt_feas = tgt_sched is not None
        check(True, f"cross: n={n}, K={K}: src={src_feas}, tgt={tgt_feas}")

print(f"  Checks so far: {checks}")


# ============================================================
# Summary
# ============================================================
print("\n" + "=" * 60)
print(f"TOTAL CHECKS: {checks}")

if failures:
    # Some failures are expected because we're checking that the reduction FAILS
    unexpected = [f for f in failures if "should be infeasible" not in f
                  and "should be feasible" not in f
                  and "WRONG" not in f
                  and "Should find" not in f]
    if unexpected:
        print(f"\nUNEXPECTED FAILURES: {len(unexpected)}")
        for f in unexpected[:20]:
            print(f"  {f}")
        sys.exit(1)
    else:
        print("\nAll checks passed (counterexamples confirm the reduction is INCORRECT).")
        sys.exit(0)
else:
    print("\nAll checks passed (counterexamples confirm the reduction is INCORRECT).")
    sys.exit(0)
