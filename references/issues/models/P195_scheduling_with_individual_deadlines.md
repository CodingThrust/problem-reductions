---
name: Problem
about: Propose a new problem type
title: "[Model] SchedulingWithIndividualDeadlines"
labels: model
assignees: ''
---

## Motivation

SCHEDULING WITH INDIVIDUAL DEADLINES (P195) from Garey & Johnson, A5 SS11. A classical NP-complete scheduling problem where unit-length tasks with precedence constraints and individual deadlines must be assigned to m parallel processors so that every task meets its own deadline. The problem remains NP-complete even when the precedence order is an out-tree, but becomes polynomial for in-trees and for m = 2 with arbitrary precedence [Brucker, Garey, and Johnson, 1977].

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R140: Vertex Cover -> Scheduling with Individual Deadlines (this model is the **target**)

## Definition

**Name:** `SchedulingWithIndividualDeadlines`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS11

**Mathematical definition:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m in Z+ of processors, partial order < on T, and for each task t in T a deadline d(t) in Z+.
QUESTION: Is there an m-processor schedule sigma for T that obeys the precedence constraints and meets all the deadlines, i.e., sigma(t) + l(t) <= d(t) for all t in T?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one discrete variable per task, choosing the time slot in which it is scheduled)
- **Per-variable domain:** {0, 1, ..., D_max - 1} where D_max = max_t d(t) -- the time slot assigned to each task
- **Meaning:** sigma(t) in {0, ..., d(t)-1} is the start time of task t. Since all tasks have unit length, task t occupies [sigma(t), sigma(t)+1). The schedule must satisfy: (1) at most m tasks per time slot, (2) if t < t' then sigma(t) + 1 <= sigma(t'), and (3) sigma(t) + 1 <= d(t) for every task t.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SchedulingWithIndividualDeadlines`
**Variants:** none (no type parameters; all tasks have unit length)

| Field              | Type                 | Description                                               |
|--------------------|----------------------|-----------------------------------------------------------|
| `num_tasks`        | `usize`              | Number of unit-length tasks n = |T|                       |
| `num_processors`   | `usize`              | Number of identical processors m                          |
| `precedences`      | `Vec<(usize, usize)>`| Edges (i, j) of the precedence DAG: task i must precede j |
| `deadlines`        | `Vec<u64>`           | Individual deadline d(t) for each task t                  |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** NP-complete by reduction from VERTEX COVER [Brucker, Garey, and Johnson, 1977]. Remains NP-complete even if the precedence order is an out-tree. Solvable in polynomial time when: (a) m = 2 and precedence is arbitrary [Garey and Johnson, 1976c], (b) precedence is an in-tree, or (c) precedence is empty (solvable by matching for arbitrary m). For the general NP-complete case, no known exact algorithm improves upon O*(D_max^n) brute-force enumeration; practical solvers use branch-and-bound or ILP formulations.

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m in Z+ of processors, partial order < on T, and for each task t in T a deadline d(t) in Z+.
QUESTION: Is there an m-processor schedule sigma for T that obeys the precedence constraints and meets all the deadlines, i.e., sigma(t) + l(t) <= d(t) for all t in T?

Reference: [Brucker, Garey, and Johnson, 1977]. Transformation from VERTEX COVER.

Comment: Remains NP-complete even if < is an "out-tree" partial order (no task has more than one immediate predecessor), but can be solved in polynomial time if < is an "in-tree" partial order (no task has more than one immediate successor). Solvable in polynomial time if m = 2 and < is arbitrary [Garey and Johnson, 1976c], even if individual release times are included [Garey and Johnson, 1977b]. For < empty, can be solved in polynomial time by matching for m arbitrary, even with release times and with a single resource having 0-1 valued requirements [Blazewicz, 1977b], [Blazewicz, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all assignments of tasks to time slots, checking precedence, processor capacity, and deadlines.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{t,u} in {0,1}, sum_u x_{t,u} = 1 for each t, sum_t x_{t,u} <= m for each u, x_{t,u} = 0 for u >= d(t), and precedence constraints.)
- [ ] Other: Polynomial for m = 2 (Garey-Johnson algorithm); polynomial for in-tree precedence.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5, t_6, t_7} (n = 7 unit-length tasks)
m = 3 processors
Precedences: t_1 < t_4, t_2 < t_4, t_2 < t_5, t_3 < t_5, t_3 < t_6
Deadlines: d(t_1) = 2, d(t_2) = 1, d(t_3) = 2, d(t_4) = 2, d(t_5) = 3, d(t_6) = 3, d(t_7) = 2

**Feasible schedule:**
Time slot 0: {t_1, t_2, t_3} (3 tasks <= 3 processors; t_2 must finish by 1, so it starts at 0)
Time slot 1: {t_4, t_6, t_7} (3 tasks <= 3 processors; t_4 predecessors t_1,t_2 done; t_6 predecessor t_3 done; t_7 has no predecessors)
Time slot 2: {t_5} (1 task; predecessors t_2,t_3 done; d(t_5) = 3, finishes at 3 <= 3)

Check deadlines:
- t_1: starts 0, finishes 1 <= 2
- t_2: starts 0, finishes 1 <= 1
- t_3: starts 0, finishes 1 <= 2
- t_4: starts 1, finishes 2 <= 2
- t_5: starts 2, finishes 3 <= 3
- t_6: starts 1, finishes 2 <= 3
- t_7: starts 1, finishes 2 <= 2

Answer: YES -- all tasks meet their individual deadlines.
