---
name: Problem
about: Propose a new problem type
title: "[Model] TimetableDesign"
labels: model
assignees: ''
---

## Motivation

TIMETABLE DESIGN (P203) from Garey & Johnson, A5 SS19. A classical NP-complete problem modeling the assignment of craftsmen to tasks across work periods, subject to availability and requirement constraints. Shown NP-complete by Even, Itai, and Shamir (1976) even in a very restricted form (|H|=3, all R(c,t) in {0,1}). This is the foundational hardness result for all timetabling and scheduling problems in education and workforce management.

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R148: 3SAT -> Timetable Design (incoming, [Even, Itai, and Shamir, 1976])

## Definition

**Name:** `TimetableDesign`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS19

**Mathematical definition:**

INSTANCE: Set H of "work periods," set C of "craftsmen," set T of "tasks," a subset A(c) ⊆ H of "available hours" for each craftsman c ∈ C, a subset A(t) ⊆ H of "available hours" for each task t ∈ T, and, for each pair (c,t) ∈ C×T, a number R(c,t) ∈ Z0+ of "required work periods."
QUESTION: Is there a timetable for completing all the tasks, i.e., a function f: C×T×H → {0,1} (where f(c,t,h) = 1 means that craftsman c works on task t during period h) such that (1) f(c,t,h) = 1 only if h ∈ A(c)∩A(t), (2) for each h ∈ H and c ∈ C there is at most one t ∈ T for which f(c,t,h) = 1, (3) for each h ∈ H and t ∈ T there is at most one c ∈ C for which f(c,t,h) = 1, and (4) for each pair (c,t) ∈ C×T there are exactly R(c,t) values of h for which f(c,t,h) = 1?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |C| * |T| * |H| (one binary variable per craftsman-task-period triple)
- **Per-variable domain:** {0, 1} — whether craftsman c works on task t during period h
- **Meaning:** f(c,t,h) = 1 if craftsman c is assigned to task t during work period h; 0 otherwise. The constraints ensure: (1) assignments respect availability windows, (2) each craftsman works on at most one task per period, (3) each task has at most one craftsman per period, and (4) total work on each (c,t) pair meets the requirement R(c,t).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `TimetableDesign`
**Variants:** none (no type parameters; all values are non-negative integers)

| Field                | Type                      | Description                                                     |
|----------------------|---------------------------|-----------------------------------------------------------------|
| `num_periods`        | `usize`                   | Number of work periods |H|                                     |
| `num_craftsmen`      | `usize`                   | Number of craftsmen |C|                                        |
| `num_tasks`          | `usize`                   | Number of tasks |T|                                            |
| `craftsman_avail`    | `Vec<Vec<bool>>`          | A(c): for each craftsman, which periods are available           |
| `task_avail`         | `Vec<Vec<bool>>`          | A(t): for each task, which periods are available                |
| `requirements`       | `Vec<Vec<u64>>`           | R(c,t): required work periods for each (craftsman, task) pair   |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete (Even, Itai, and Shamir, 1976). It remains NP-complete even when |H| = 3, A(t) = H for all tasks, and all R(c,t) in {0,1}. In the general case, brute-force enumeration of all |T|^|C| possible per-period assignments (or equivalently all valid f functions) requires exponential time. For the restricted case with |A(c)| <= 2 for all c, or when A(c) = A(t) = H for all c and t, the problem is solvable in polynomial time via bipartite matching. No known exact algorithm significantly improves upon O*(2^(|C|*|T|)) in the worst case.

## Extra Remark

**Full book text:**

INSTANCE: Set H of "work periods," set C of "craftsmen," set T of "tasks," a subset A(c) ⊆ H of "available hours" for each craftsman c ∈ C, a subset A(t) ⊆ H of "available hours" for each task t ∈ T, and, for each pair (c,t) ∈ C×T, a number R(c,t) ∈ Z0+ of "required work periods."
QUESTION: Is there a timetable for completing all the tasks, i.e., a function f: C×T×H → {0,1} (where f(c,t,h) = 1 means that craftsman c works on task t during period h) such that (1) f(c,t,h) = 1 only if h ∈ A(c)∩A(t), (2) for each h ∈ H and c ∈ C there is at most one t ∈ T for which f(c,t,h) = 1, (3) for each h ∈ H and t ∈ T there is at most one c ∈ C for which f(c,t,h) = 1, and (4) for each pair (c,t) ∈ C×T there are exactly R(c,t) values of h for which f(c,t,h) = 1?

Reference: [Even, Itai, and Shamir, 1976]. Transformation from 3SAT.

Comment: Remains NP-complete even if |H| = 3, A(t) = H for all t ∈ T, and each R(c,t) ∈ {0,1}. The general problem can be solved in polynomial time if |A(c)| ≤ 2 for all c ∈ C or if A(c) = A(t) = H for all c ∈ C and t ∈ T.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all valid assignment functions f: C x T x H -> {0,1} satisfying constraints (1)-(4); check feasibility.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: f(c,t,h) in {0,1} with constraints for availability, one-task-per-craftsman-per-period, one-craftsman-per-task-per-period, and requirement satisfaction.)
- [ ] Other: For special cases (|A(c)| <= 2 or A(c)=A(t)=H), reduce to bipartite matching.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
H = {h_1, h_2, h_3} (3 work periods)
C = {c_1, c_2, c_3, c_4, c_5} (5 craftsmen)
T = {t_1, t_2, t_3, t_4, t_5} (5 tasks)

Availability:
- A(c_1) = {h_1, h_2, h_3}, A(c_2) = {h_1, h_2}, A(c_3) = {h_2, h_3}, A(c_4) = {h_1, h_3}, A(c_5) = {h_1, h_2, h_3}
- A(t_1) = {h_1, h_2, h_3}, A(t_2) = {h_1, h_2, h_3}, A(t_3) = {h_1, h_2, h_3}, A(t_4) = {h_1, h_2, h_3}, A(t_5) = {h_1, h_2, h_3}

Requirements (R(c,t)):
| c \ t | t_1 | t_2 | t_3 | t_4 | t_5 |
|-------|-----|-----|-----|-----|-----|
| c_1   | 1   | 0   | 1   | 0   | 0   |
| c_2   | 0   | 1   | 0   | 0   | 0   |
| c_3   | 0   | 0   | 0   | 1   | 0   |
| c_4   | 0   | 0   | 0   | 0   | 1   |
| c_5   | 0   | 1   | 0   | 0   | 1   |

**Feasible timetable:**
- f(c_1, t_1, h_1) = 1, f(c_1, t_3, h_2) = 1
- f(c_2, t_2, h_1) = 1
- f(c_3, t_4, h_2) = 1
- f(c_4, t_5, h_1) = 1
- f(c_5, t_2, h_2) = 1, f(c_5, t_5, h_3) = 1

All constraints satisfied: availability respected, no craftsman double-booked in any period, no task has two craftsmen in the same period, and all requirements met. Answer: YES.
