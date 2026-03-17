---
name: Problem
about: Propose a new problem type
title: "[Model] PreemptiveScheduling"
labels: model
assignees: ''
---

## Motivation

PREEMPTIVE SCHEDULING (P196) from Garey & Johnson, A5 SS12. A fundamental NP-complete scheduling problem that extends multiprocessor scheduling by allowing tasks to be interrupted and resumed later (preemption), while still requiring precedence constraints to be respected. Despite the flexibility of preemption, the problem remains NP-complete in general (by reduction from 3SAT), though it becomes polynomial for m = 2 processors, forest-structured precedence, or when precedence is empty [Ullman, 1975].

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R141: 3SAT -> Preemptive Scheduling (this model is the **target**)

## Definition

**Name:** `PreemptiveScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS12

**Mathematical definition:**

INSTANCE: Set T of tasks, number m in Z+ of processors, partial order < on T, length l(t) in Z+ for each t in T, and an overall deadline D in Z+.
QUESTION: Is there an m-processor "preemptive" schedule for T that obeys the precedence constraints and meets the overall deadline? (Such a schedule sigma allows subdividing each task t into subtasks t_1, ..., t_k with sum of lengths = l(t), where subtasks run non-overlapping on any processor, and precedence constraints extend to subtasks.)

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** The decision variable is the preemptive schedule itself: for each task t and each time unit u in {0, ..., D-1}, a binary variable indicating whether (a piece of) task t is running on some processor at time u. Equivalently, the schedule can be described as a mapping from (task, time-unit) pairs to processor assignments.
- **Per-variable domain:** For a discretized formulation, one can use binary variables x_{t,p,u} in {0,1} indicating task t runs on processor p at time u.
- **Meaning:** The schedule assigns processing time to tasks across processors and time slots, allowing a task to be split across non-contiguous time slots and different processors, subject to: (1) each task receives exactly l(t) time units of processing, (2) at most m tasks run per time slot, (3) precedence constraints are respected (all subtasks of t complete before any subtask of t' if t < t'), (4) all processing completes by time D.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `PreemptiveScheduling`
**Variants:** none (no type parameters)

| Field            | Type                  | Description                                              |
|------------------|-----------------------|----------------------------------------------------------|
| `num_tasks`      | `usize`               | Number of tasks n = |T|                                  |
| `num_processors` | `usize`               | Number of identical processors m                         |
| `lengths`        | `Vec<u64>`            | Length l(t) for each task t                              |
| `precedences`    | `Vec<(usize, usize)>` | Edges (i, j) of the precedence DAG: task i precedes j   |
| `deadline`       | `u64`                 | Overall deadline D                                       |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** NP-complete by reduction from 3SAT [Ullman, 1975], even with unit-length tasks. Polynomial-time solvable when m = 2 [Muntz and Coffman, 1969], when precedence is a forest [Muntz and Coffman, 1970], or when precedence is empty with individual deadlines [Horn, 1974]. For the general NP-complete case, exact algorithms use ILP or constraint programming. No known improvement over O*(2^(n*D)) brute-force enumeration for the general case.

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, number m in Z+ of processors, partial order < on T, length l(t) in Z+ for each t in T, and an overall deadline D in Z+.
QUESTION: Is there an m-processor "preemptive" schedule for T that obeys the precedence constraints and meets the overall deadline? (Such a schedule sigma is identical to an ordinary m-processor schedule, except that we are allowed to subdivide each task t in T into any number of subtasks t1, t2, ..., tk such that sum_{i=1}^{k} l(ti) = l(t) and it is required that sigma(ti + 1) >= sigma(ti)+l(ti) for 1 <= i < k. The precedence constraints are extended to subtasks by requiring that every subtask of t precede every subtask of t' whenever t < t'.)

Reference: [Ullman, 1975]. Transformation from 3SAT.

Comment: Can be solved in polynomial time if m = 2 [Muntz and Coffman, 1969], if < is a "forest" [Muntz and Coffman, 1970], or if < is empty and individual task deadlines are allowed [Horn, 1974]. If "(uniform) different speed" processors are allowed, the problem can be solved in polynomial time if m = 2 or if < is empty [Horvath, Lam, and Sethi, 1977], [Gonzalez and Sahni, 1978b] in the latter case even if individual task deadlines are allowed [Sahni and Cho, 1977a]; if both m = 2 and < is empty, it can be solved in polynomial time, even if both integer release times and deadlines are allowed [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1977]. For "unrelated" processors, the case with m fixed and < empty can be solved in polynomial time [Gonzalez, Lawler, and Sahni, 1978], and the case with m arbitrary and < empty can be solved by linear programming [Lawler and Labetoulle, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all preemptive schedules: at each time slot, choose up to m tasks to process, subject to precedence; check if all tasks complete by D.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{t,u} in {0,1} for each task t and time u; sum_u x_{t,u} = l(t); sum_t x_{t,u} <= m; precedence constraints on subtask ordering.)
- [ ] Other: Polynomial for m = 2 (Muntz-Coffman level algorithm); polynomial for forest precedence; LP-based for unrelated processors with empty precedence.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
m = 2 processors
Lengths: l(t_1) = 3, l(t_2) = 2, l(t_3) = 2, l(t_4) = 1, l(t_5) = 2
Precedences: t_1 < t_4, t_2 < t_5
D = 5 (overall deadline)
Total work = 3 + 2 + 2 + 1 + 2 = 10 = m * D = 2 * 5 (tight).

**Feasible preemptive schedule:**
Time 0: t_1, t_2 (both processors busy)
Time 1: t_1, t_2 (t_2 completes at end of time 1, having received 2 units)
Time 2: t_1, t_3 (t_1 has received 3 units at end of time 2; t_3 gets 1 unit)
Time 3: t_3, t_4 (t_3 gets its 2nd unit; t_4 gets its 1 unit; t_1 done so t_4 can run)
Time 4: t_5, (idle) ... no, we need both processors. t_5 needs 2 units. t_5 at time 3 and 4?

Revised:
Time 0: t_1, t_2
Time 1: t_1, t_3
Time 2: t_1, t_3 (t_1 done with 3 units; t_3 done with 2 units)
Time 3: t_4, t_5 (t_4 = 1 unit, done; t_5 gets 1 unit; t_2 finished, so t_5 can run)
Time 4: t_5, t_2 ... t_2 already done.

Revised again with correct tracking:
Time 0: t_1 (1/3), t_2 (1/2)
Time 1: t_1 (2/3), t_2 (2/2, done)
Time 2: t_1 (3/3, done), t_3 (1/2)
Time 3: t_4 (1/1, done; t_1 done so OK), t_3 (2/2, done)
Time 4: t_5 (1/2), idle ... but t_5 needs 2 units and deadline is 5.
Time 4: t_5 (1/2), [need something for other processor] -- t_5 only has 1 unit left at time 5 but we only have D=5 slots (0..4).

t_5 needs t_2 done (done at time 1). So t_5 can start at time 2.
Time 2: t_1 (3/3), t_5 (1/2) -- but t_1 takes its 3rd unit. Wait, let me preempt t_3.
Time 0: t_1, t_2
Time 1: t_1, t_2 (t_2 done)
Time 2: t_1 (done), t_5 (1/2)
Time 3: t_4 (done), t_5 (2/2, done)
Time 4: t_3 (1/2), idle
But t_3 needs 2 units and only 1 slot left. Infeasible.

Simpler instance: l = {2, 2, 2, 1, 1}, no precedences, m = 2, D = 4. Total = 8 = 2*4.
Time 0: t_1, t_2; Time 1: t_1, t_2; Time 2: t_3, t_4; Time 3: t_3, t_5. Done at 4.

**Correct example:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
m = 2 processors
Lengths: l(t_1) = 2, l(t_2) = 2, l(t_3) = 2, l(t_4) = 1, l(t_5) = 1
Precedences: t_1 < t_4, t_2 < t_5
D = 4 (total work = 8 = 2 * 4)

Feasible preemptive schedule:
Time 0: t_1, t_2
Time 1: t_1 (done), t_2 (done)
Time 2: t_3, t_4 (t_1 done, so t_4 OK)
Time 3: t_3 (done), t_5 (t_2 done, so t_5 OK)
All tasks complete by time 4 = D.

Answer: YES -- a valid preemptive schedule exists meeting the deadline D = 4.
