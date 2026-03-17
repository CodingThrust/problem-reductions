---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeMaximumCumulativeCost"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE MAXIMUM CUMULATIVE COST (P191) from Garey & Johnson, A5 SS7. A precedence-constrained single-processor scheduling problem where each task has an integer cost (possibly negative, representing a "profit"), and the goal is to order tasks so that the running total of costs never exceeds a given bound K. NP-complete even when costs are restricted to {−1, 0, 1} (via reduction from REGISTER SUFFICIENCY, R137). Can be solved in polynomial time if the precedence order is series-parallel.

**Associated rules:**
- R137: REGISTER SUFFICIENCY → Sequencing to Minimize Maximum Cumulative Cost (this model is the **target**)

## Definition

**Name:** `SequencingToMinimizeMaximumCumulativeCost`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS7

**Mathematical definition:**

INSTANCE: Set T of tasks, partial order < on T, a "cost" c(t) ∈ Z for each t ∈ T (if c(t) < 0, it can be viewed as a "profit"), and a constant K ∈ Z.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and which has the property that, for every task t ∈ T, the sum of the costs for all tasks t' with σ(t') ≤ σ(t) is at most K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one variable per task, representing its position in the schedule)
- **Per-variable domain:** {0, 1, ..., n−1} — the position index of the task in a topological ordering
- **Meaning:** π(i) ∈ {0, ..., n−1} gives the position of task t_i in a linear extension of the partial order. The schedule must be a topological sort of the precedence DAG. At each position j, the cumulative cost Σ_{k≤j} c(t_{π^{-1}(k)}) must not exceed K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequencingToMinimizeMaximumCumulativeCost`
**Variants:** none (no type parameters)

| Field          | Type              | Description                                               |
|----------------|-------------------|-----------------------------------------------------------|
| `costs`        | `Vec<i64>`        | Cost c(t) for each task t ∈ T (may be negative)          |
| `predecessors` | `Vec<Vec<usize>>` | For each task, list of tasks that must precede it         |
| `bound`        | `i64`             | Upper bound K on maximum cumulative cost                  |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete, even with costs restricted to {−1, 0, 1} [Abdel-Wahab, 1976]. It can be solved in polynomial time when the precedence order is series-parallel [Abdel-Wahab & Kameda, 1978; Monma & Sidney, 1977]. For general precedence constraints, exact algorithms use branch-and-bound or enumerate topological orderings. A dynamic programming approach over the lattice of antichains gives O*(2^w) where w is the width of the partial order (maximum antichain size). The brute-force complexity is O(n! · n) for enumerating all topological orderings.

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, partial order < on T, a "cost" c(t) ∈ Z for each t ∈ T (if c(t) < 0, it can be viewed as a "profit"), and a constant K ∈ Z.
QUESTION: Is there a one-processor schedule σ for T that obeys the precedence constraints and which has the property that, for every task t ∈ T, the sum of the costs for all tasks t' with σ(t') ≤ σ(t) is at most K?

Reference: [Abdel-Wahab, 1976]. Transformation from REGISTER SUFFICIENCY.

Comment: Remains NP-complete even if c(t) ∈ {-1,0,1} for all t ∈ T. Can be solved in polynomial time if < is series-parallel [Abdel-Wahab and Kameda, 1978], [Monma and Sidney, 1977].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all topological orderings of the precedence DAG; compute cumulative costs along each ordering; check if max cumulative cost ≤ K.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{ij} ∈ {0,1} = task i at position j; enforce topological order constraints; compute cumulative cost at each position; add constraint that each cumulative sum ≤ K.)
- [ ] Other: Polynomial algorithm for series-parallel precedence [Abdel-Wahab & Kameda, 1978].

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5, t_6} (n = 6 tasks)
Costs: c = [2, −1, 3, −2, 1, −3]
Precedence constraints (DAG edges):
- t_1 < t_3 (t_1 must precede t_3)
- t_2 < t_3
- t_2 < t_4
- t_3 < t_5
- t_4 < t_6
- t_5 < t_6
Bound K = 4.

**Feasible schedule (topological order):**
Order: t_2, t_1, t_4, t_3, t_5, t_6
Cumulative costs:
- After t_2: −1
- After t_1: −1 + 2 = 1
- After t_4: 1 + (−2) = −1
- After t_3: −1 + 3 = 2
- After t_5: 2 + 1 = 3
- After t_6: 3 + (−3) = 0

Maximum cumulative cost = 3 ≤ K = 4 ✓

**Alternative (worse) order:** t_1, t_2, t_3, t_4, t_5, t_6
Cumulative: 2, 1, 4, 2, 3, 0. Max = 4 ≤ K = 4 ✓ (just barely feasible).

**Infeasible order for K = 3:** t_1, t_2, t_3, t_4, t_5, t_6 has max cumulative = 4 > 3.

Answer: YES — a valid schedule with max cumulative cost ≤ K = 4 exists.
