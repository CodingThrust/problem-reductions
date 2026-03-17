---
name: Problem
about: Propose a new problem type
title: "[Model] PrecedenceConstrainedScheduling"
labels: model
assignees: ''
---

## Motivation

PRECEDENCE CONSTRAINED SCHEDULING (P193) from Garey & Johnson, A5 SS9. A fundamental NP-complete multiprocessor scheduling problem: given unit-length tasks with precedence constraints, m processors, and a deadline D, can all tasks be scheduled to meet D while respecting precedences? NP-complete via reduction from 3SAT (R138) [Ullman, 1975]. Remains NP-complete even for D = 3 [Lenstra & Rinnooy Kan, 1978a]. Solvable in polynomial time for m = 2 [Coffman & Graham, 1972], for forest-structured precedences [Hu, 1961], and for chordal complement precedences [Papadimitriou & Yannakakis, 1978b].

**Associated rules:**
- R138: 3SAT → Precedence Constrained Scheduling (this model is the **target**)

## Definition

**Name:** `PrecedenceConstrainedScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS9

**Mathematical definition:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, partial order < on T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the precedence constraints, i.e., such that t < t' implies σ(t') ≥ σ(t) + l(t) = σ(t) + 1?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one variable per task, representing the time slot it is assigned to)
- **Per-variable domain:** {1, 2, ..., D} — the time slot in which the task is executed
- **Meaning:** σ(t_i) ∈ {1, ..., D} is the time slot assigned to task t_i. At most m tasks can be assigned to the same time slot (processor capacity). If t_i < t_j in the partial order, then σ(t_j) ≥ σ(t_i) + 1. A valid schedule assigns all n tasks to time slots in {1, ..., D} respecting both processor capacity and precedence constraints.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `PrecedenceConstrainedScheduling`
**Variants:** none (tasks are unit-length; no type parameters)

| Field            | Type              | Description                                                |
|------------------|-------------------|------------------------------------------------------------|
| `num_tasks`      | `usize`           | Number of tasks n = |T|                                   |
| `num_processors` | `usize`           | Number of processors m                                     |
| `deadline`       | `usize`           | Global deadline D                                          |
| `precedences`    | `Vec<(usize, usize)>` | Precedence pairs (i, j) meaning t_i < t_j            |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete [Ullman, 1975]. For m = 2, the Coffman-Graham algorithm solves it in O(n^2) time [Coffman & Graham, 1972]. For fixed m ≥ 3, the complexity is open. For variable m, the problem is strongly NP-hard. Exact algorithms for general instances use branch-and-bound or ILP formulations. A fixed-parameter algorithm based on the width w of the precedence graph runs in O(n^3 + n · w · 2^{4w}) time [Bodlaender & Fellows]. The brute-force complexity is O(D^n) for assigning each of n tasks to one of D time slots, followed by feasibility checking.

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m ∈ Z+ of processors, partial order < on T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule σ for T that meets the overall deadline D and obeys the precedence constraints, i.e., such that t < t' implies σ(t') ≥ σ(t) + l(t) = σ(t) + 1?

Reference: [Ullman, 1975]. Transformation from 3SAT.

Comment: Remains NP-complete for D = 3 [Lenstra and Rinnooy Kan, 1978a]. Can be solved in polynomial time if m = 2 (e.g., see [Coffman and Graham, 1972]) or if m is arbitrary and < is a "forest" [Hu, 1961] or has a chordal graph as complement [Papadimitriou and Yannakakis, 1978b]. Complexity remains open for all fixed m ≥ 3 when < is arbitrary. The m = 2 case becomes NP-complete if both task lengths 1 and 2 are allowed [Ullman, 1975]. If each task t can only be executed by a specified processor p(t), the problem is NP-complete for m = 2 and < arbitrary, and for m arbitrary and < a forest, but can be solved in polynomial time for m arbitrary if < is a "cyclic forest" [Goyal, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all assignments of n tasks to D time slots; check processor capacity ≤ m per slot and precedence constraints.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{it} ∈ {0,1} = task i in time slot t; Σ_t x_{it} = 1 for each task; Σ_i x_{it} ≤ m for each slot t; precedence: Σ_t t·x_{jt} ≥ Σ_t t·x_{it} + 1 for each (i,j) ∈ precedences.)
- [ ] Other: Coffman-Graham algorithm for m = 2 [Coffman & Graham, 1972]; Hu's algorithm for forest precedences [Hu, 1961].

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
n = 8 tasks: T = {t_1, t_2, t_3, t_4, t_5, t_6, t_7, t_8}
m = 3 processors, D = 4 time slots.
Precedences:
- t_1 < t_3
- t_1 < t_4
- t_2 < t_4
- t_2 < t_5
- t_3 < t_6
- t_4 < t_7
- t_5 < t_7
- t_6 < t_8
- t_7 < t_8

**Feasible schedule:**

| Time slot | Tasks (up to 3 per slot) |
|-----------|--------------------------|
| 1         | t_1, t_2                 |
| 2         | t_3, t_4, t_5            |
| 3         | t_6, t_7                 |
| 4         | t_8                      |

Check precedences:
- t_1(1) < t_3(2) ✓, t_1(1) < t_4(2) ✓
- t_2(1) < t_4(2) ✓, t_2(1) < t_5(2) ✓
- t_3(2) < t_6(3) ✓, t_4(2) < t_7(3) ✓
- t_5(2) < t_7(3) ✓, t_6(3) < t_8(4) ✓
- t_7(3) < t_8(4) ✓

Processor capacity: max tasks per slot = 3 ≤ m = 3 ✓
All tasks complete by slot 4 = D ✓

Answer: YES — a valid 3-processor schedule meeting deadline D = 4 exists.
