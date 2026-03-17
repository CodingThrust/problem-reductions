---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeTardyTasks"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE TARDY TASKS (P186) from Garey & Johnson, A5 SS2. A single-machine scheduling problem with precedence constraints: given n tasks with unit or arbitrary lengths, deadlines, and a partial order, can we find a schedule obeying precedence constraints with at most K tardy tasks (tasks finishing after their deadline)? NP-complete by reduction from CLIQUE (Garey & Johnson, 1976). The problem remains NP-complete even with unit task lengths and chain precedence constraints. Without precedence constraints, it is solvable in polynomial time by Moore's algorithm (1968).

**Associated rules:**
- R132: Clique -> Sequencing to Minimize Tardy Tasks (as target)

<!-- ⚠️ Unverified: AI-generated motivation and associated rules list -->

## Definition

**Name:** `SequencingToMinimizeTardyTasks`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS2

**Mathematical definition:**

INSTANCE: Set T of tasks, partial order < on T, for each task t in T a length l(t) in Z+ and a deadline d(t) in Z+, and a positive integer K <= |T|.
QUESTION: Is there a one-processor schedule sigma for T that obeys the precedence constraints, i.e., such that t < t' implies sigma(t) + l(t) <= sigma(t'), and such that there are at most K tasks t in T for which sigma(t) + l(t) > d(t)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one variable per task, representing position in the schedule)
- **Per-variable domain:** A permutation of {0, 1, ..., n-1} consistent with the partial order (topological ordering)
- **Meaning:** sigma(t) is the start time of task t. A task t is "tardy" if sigma(t) + l(t) > d(t). The problem asks whether the number of tardy tasks can be kept to at most K while respecting all precedence constraints.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequencingToMinimizeTardyTasks`
**Variants:** none (no type parameters)

| Field        | Type             | Description                                              |
|--------------|------------------|----------------------------------------------------------|
| `lengths`    | `Vec<u64>`       | Processing time l(t) for each task t in T                |
| `deadlines`  | `Vec<u64>`       | Deadline d(t) for each task t in T                       |
| `precedences`| `Vec<(usize, usize)>` | Pairs (i, j) meaning task i must complete before task j starts |
| `bound_k`    | `u64`            | Maximum allowed number of tardy tasks K                  |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete. Without precedence constraints (partial order is empty), it is solvable in O(n log n) by Moore's algorithm (1968), which greedily schedules by earliest deadline and removes the longest task when a deadline is missed. With precedence constraints, exact solution requires exponential time in the worst case. Branch-and-bound and integer programming are the main practical exact methods. The problem remains NP-complete even with unit task lengths and chain-like precedence constraints (Lenstra, 1977). For K = 0 (all tasks must meet their deadlines), the problem reduces to feasibility checking and is polynomial (Lawler, 1973). [Moore, 1968; Garey & Johnson, 1976; Lenstra, 1977.]

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, partial order < on T, for each task t in T a length l(t) in Z+ and a deadline d(t) in Z+, and a positive integer K <= |T|.
QUESTION: Is there a one-processor schedule sigma for T that obeys the precedence constraints, i.e., such that t < t' implies sigma(t) + l(t) <= sigma(t'), and such that there are at most K tasks t in T for which sigma(t) + l(t) > d(t)?

Reference: [Garey and Johnson, 1976c]. Transformation from CLIQUE (see Section 3.2.3).

Comment: Remains NP-complete even if all task lengths are 1 and < consists only of "chains" (each task has at most one immediate predecessor and at most one immediate successor) [Lenstra, 1977]. The general problem can be solved in polynomial time if K = 0 [Lawler, 1973], or if < is empty [Moore, 1968] [Sidney, 1973]. The < empty case remains polynomially solvable if "agreeable" release times (i.e., r(t) < r(t') implies d(t) <= d(t')) are added [Kise, Ibaraki, and Mine, 1978], but is NP-complete for arbitrary release times (see previous problem).

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all topological orderings of the partial order; for each ordering compute start times and count tardy tasks.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: ordering variables x_{ij} in {0,1} for pairs, precedence constraints, deadline constraints, binary tardiness indicators U_t, sum U_t <= K.)
- [ ] Other: Moore's algorithm when < is empty (O(n log n)). Branch-and-bound for general case.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5, t_6} (n = 6 tasks)
Lengths: l(t_1) = 1, l(t_2) = 1, l(t_3) = 1, l(t_4) = 1, l(t_5) = 1, l(t_6) = 1 (all unit length)
Deadlines: d(t_1) = 6, d(t_2) = 6, d(t_3) = 3, d(t_4) = 3, d(t_5) = 3, d(t_6) = 6
Precedence constraints: t_1 < t_3, t_2 < t_4 (two chains: t_1->t_3 and t_2->t_4)
K = 1

**Feasible schedule:**
- sigma(t_1) = 0, finishes at 1 <= d=6 (on time)
- sigma(t_2) = 1, finishes at 2 <= d=6 (on time)
- sigma(t_5) = 2, finishes at 3 <= d=3 (on time)
- sigma(t_3) = 3, finishes at 4 > d=3 (TARDY)
- sigma(t_4) = 4, finishes at 5 > d=3... but we need at most K=1 tardy.

Better schedule:
- sigma(t_1) = 0, finishes at 1 (on time)
- sigma(t_3) = 1, finishes at 2 <= d=3 (on time, precedence t_1 < t_3 respected)
- sigma(t_2) = 2, finishes at 3 (on time)
- sigma(t_5) = 3, finishes at 4 > d=3 (TARDY)
- sigma(t_4) = 4, finishes at 5 > d=3... 2 tardy already.

Schedule with K=1:
- sigma(t_1) = 0, finishes at 1 <= d=6 (on time)
- sigma(t_2) = 1, finishes at 2 <= d=6 (on time)
- sigma(t_3) = 2, finishes at 3 <= d=3 (on time, precedence respected)
- sigma(t_4) = 3, finishes at 4 > d=3 (TARDY)
- sigma(t_5) = 4, finishes at 5 > d=3 ... 2 tardy.

Revised example with K=2:
K = 2

Schedule:
- sigma(t_1) = 0, finishes 1 <= d=6
- sigma(t_2) = 1, finishes 2 <= d=6
- sigma(t_3) = 2, finishes 3 <= d=3
- sigma(t_5) = 3, finishes 4 > d=3 (TARDY)
- sigma(t_4) = 4, finishes 5 > d=3 (TARDY)
- sigma(t_6) = 5, finishes 6 <= d=6

Tardy tasks: {t_5, t_4}, count = 2 <= K = 2. Precedence respected (t_1 before t_3, t_2 before t_4).

Answer: YES -- a schedule with at most 2 tardy tasks exists.
