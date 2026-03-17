---
name: Problem
about: Propose a new problem type
title: "[Model] SchedulingToMinimizeWeightedCompletionTime"
labels: model
assignees: ''
---

## Motivation

SCHEDULING TO MINIMIZE WEIGHTED COMPLETION TIME (P197) from Garey & Johnson, A5 SS13. An NP-complete scheduling optimization problem: given tasks with integer lengths and weights on m identical processors (no precedence), find a schedule minimizing the total weighted completion time, or equivalently decide whether a schedule with total weighted completion time at most K exists. NP-complete even for m = 2 by reduction from PARTITION, and NP-complete in the strong sense for m arbitrary [Lenstra, Rinnooy Kan, and Brucker, 1977].

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R142: Partition -> Scheduling to Minimize Weighted Completion Time (this model is the **target**)

## Definition

**Name:** `SchedulingToMinimizeWeightedCompletionTime`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS13

**Mathematical definition:**

INSTANCE: Set T of tasks, number m in Z+ of processors, for each task t in T a length l(t) in Z+ and a weight w(t) in Z+, and a positive integer K.
QUESTION: Is there an m-processor schedule sigma for T such that the sum, over all t in T, of (sigma(t) + l(t)) * w(t) is no more than K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one discrete variable per task, choosing which processor it is assigned to, plus the ordering on that processor)
- **Per-variable domain:** {1, 2, ..., m} -- the processor index to which the task is assigned. Within each processor, tasks are ordered (the optimal order on a single processor is given by Smith's rule: sort by non-decreasing w(t)/l(t) ratio).
- **Meaning:** p_t in {1, ..., m} is the processor assignment for task t. The completion time of task t is sigma(t) + l(t), where sigma(t) depends on the processing order on processor p_t. The objective is to minimize sum_t (sigma(t) + l(t)) * w(t), and we ask whether this minimum is at most K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SchedulingToMinimizeWeightedCompletionTime`
**Variants:** none (no type parameters)

| Field            | Type       | Description                                               |
|------------------|------------|-----------------------------------------------------------|
| `lengths`        | `Vec<u64>` | Length l(t) of each task t in T                           |
| `weights`        | `Vec<u64>` | Weight w(t) of each task t in T                           |
| `num_processors` | `usize`    | Number of identical processors m                          |
| `bound`          | `u64`      | Upper bound K on the total weighted completion time       |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** NP-complete for m = 2 by reduction from PARTITION; NP-complete in the strong sense for arbitrary m [Lenstra, Rinnooy Kan, and Brucker, 1977]. For fixed m, solvable in pseudo-polynomial time by dynamic programming over load vectors. For general m, exact approaches use ILP or branch-and-bound. When all lengths are equal, solvable in polynomial time by matching. When all weights are equal, solvable in polynomial time (even for different-speed and unrelated processors) by the LPT rule or flow-based methods [Conway et al., 1967; Horn, 1973; Bruno et al., 1974].

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, number m in Z+ of processors, for each task t in T a length l(t) in Z+ and a weight w(t) in Z+, and a positive integer K.
QUESTION: Is there an m-processor schedule sigma for T such that the sum, over all t in T, of (sigma(t) + l(t))*w(t) is no more than K?

Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from PARTITION.

Comment: Remains NP-complete for m = 2, and is NP-complete in the strong sense for m arbitrary [Lageweg and Lenstra, 1977]. The problem is solvable in pseudo-polynomial time for fixed m. These results continue to hold if "preemptive" schedules are allowed [McNaughton, 1959]. Can be solved in polynomial time if all lengths are equal (by matching techniques). If instead all weights are equal, it can be solved in polynomial time even for "different speed" processors [Conway, Maxwell, and Miller, 1967] and for "unrelated" processors [Horn, 1973], [Bruno, Coffman, and Sethi, 1974]. The "preemptive" case for different speed processors also can be solved in polynomial time [Gonzalez, 1977]. If precedence constraints are allowed, the original problem is NP-complete in the strong sense even if all weights are equal, m = 2, and the partial order is either an "in-tree" or an "out-tree" [Sethi, 1977a]. If resources are allowed, the same subcases mentioned under RESOURCE CONSTRAINED SCHEDULING are NP-complete, even for equal weights [Blazewicz, 1977a].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all m^n assignments of tasks to processors; for each assignment, order tasks on each processor by Smith's rule; compute total weighted completion time; check if <= K.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{t,p} in {0,1} for assignment, ordering variables for sequencing on each processor, and linearized completion time constraints.)
- [ ] Other: Pseudo-polynomial DP for fixed m; polynomial when all lengths or all weights are equal.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
Lengths: l(t_1) = 4, l(t_2) = 5, l(t_3) = 3, l(t_4) = 2, l(t_5) = 6
Weights: w(t_1) = 4, w(t_2) = 5, w(t_3) = 3, w(t_4) = 2, w(t_5) = 6
m = 2 processors, K = 145.

**Feasible assignment:**
Processor 1: {t_1, t_5}
- Order by Smith's rule (w/l = 1 for all, so any order; use shortest first): t_1 (l=4), t_5 (l=6)
- t_1 completes at 4, contribution = 4*4 = 16
- t_5 completes at 10, contribution = 10*6 = 60
- Subtotal = 76

Processor 2: {t_4, t_3, t_2}
- Order shortest first: t_4 (l=2), t_3 (l=3), t_2 (l=5)
- t_4 completes at 2, contribution = 2*2 = 4
- t_3 completes at 5, contribution = 5*3 = 15
- t_2 completes at 10, contribution = 10*5 = 50
- Subtotal = 69

Total weighted completion time = 76 + 69 = 145 <= K = 145.

Answer: YES -- a schedule achieving weighted completion time 145 <= K exists.
