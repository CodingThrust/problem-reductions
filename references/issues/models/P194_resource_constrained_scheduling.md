---
name: Problem
about: Propose a new problem type
title: "[Model] ResourceConstrainedScheduling"
labels: model
assignees: ''
---

## Motivation

RESOURCE CONSTRAINED SCHEDULING (P194) from Garey & Johnson, A5 SS10. A classical NP-complete scheduling problem where unit-length tasks must be assigned to identical processors under both a processor capacity limit and resource usage constraints per time slot. NP-complete in the strong sense even for r = 1 resource and m = 3 processors, established by reduction from 3-PARTITION [Garey and Johnson, 1975].

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R139: 3-Partition -> Resource Constrained Scheduling (this model is the **target**)

## Definition

**Name:** `ResourceConstrainedScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS10

**Mathematical definition:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m in Z+ of processors, number r in Z+ of resources, resource bounds Bi, 1 <= i <= r, resource requirement Ri(t), 0 <= Ri(t) <= Bi, for each task t and resource i, and an overall deadline D in Z+.
QUESTION: Is there an m-processor schedule sigma for T that meets the overall deadline D and obeys the resource constraints, i.e., such that for all u >= 0, if S(u) is the set of all t in T for which sigma(t) <= u < sigma(t) + l(t), then for each resource i the sum of Ri(t) over all t in S(u) is at most Bi?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one discrete variable per task, choosing the time slot in which it is scheduled)
- **Per-variable domain:** {0, 1, ..., D-1} -- the time slot assigned to each task
- **Meaning:** sigma(t) in {0, 1, ..., D-1} is the start time of task t. All tasks have unit length, so task t occupies time slot [sigma(t), sigma(t)+1). At each time slot u, at most m tasks can execute simultaneously, and the total requirement for each resource i must not exceed Bi.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ResourceConstrainedScheduling`
**Variants:** none (no type parameters; resource requirements and bounds are plain positive integers)

| Field               | Type             | Description                                                   |
|---------------------|------------------|---------------------------------------------------------------|
| `num_tasks`         | `usize`          | Number of unit-length tasks n = |T|                           |
| `num_processors`    | `usize`          | Number of identical processors m                              |
| `resource_bounds`   | `Vec<u64>`       | Resource bound Bi for each resource i (length = r)            |
| `resource_requirements` | `Vec<Vec<u64>>` | Ri(t) for each task t and resource i (n x r matrix)        |
| `deadline`          | `u64`            | Overall deadline D                                            |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** NP-complete in the strong sense even for r = 1 and m = 3 [Garey and Johnson, 1975]. For fixed m, the problem is strongly NP-hard (no pseudo-polynomial algorithm unless P=NP). Exact approaches use branch-and-bound, integer programming (binary ILP: x_{t,u} in {0,1} indicating task t starts at time u), or constraint programming. For m = 2 with arbitrary r, the problem can be solved in polynomial time by matching. No known exact algorithm improves upon O*(D^n) brute-force enumeration for the general strongly NP-hard case.

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, each having length l(t) = 1, number m in Z+ of processors, number r in Z+ of resources, resource bounds Bi, 1 <= i <= r, resource requirement Ri(t), 0 <= Ri(t) <= Bi, for each task t and resource i, and an overall deadline D in Z+.
QUESTION: Is there an m-processor schedule sigma for T that meets the overall deadline D and obeys the resource constraints, i.e., such that for all u >= 0, if S(u) is the set of all t in T for which sigma(t) <= u < sigma(t) + l(t), then for each resource i the sum of Ri(t) over all t in S(u) is at most Bi?

Reference: [Garey and Johnson, 1975]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense, even if r = 1 and m = 3. Can be solved in polynomial time by matching for m = 2 and r arbitrary. If a partial order < is added, the problem becomes NP-complete in the strong sense for r = 1, m = 2, and < a "forest." If each resource requirement is restricted to be either 0 or Bi, the problem is NP-complete for m = 2, r = 1, and < arbitrary [Ullman, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all D^n assignments of tasks to time slots; check processor and resource constraints at each slot.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{t,u} in {0,1}, sum_u x_{t,u} = 1 for each t, sum_t x_{t,u} <= m for each u, sum_t Ri(t)*x_{t,u} <= Bi for each i,u.)
- [ ] Other: For m = 2, solvable in polynomial time by bipartite matching.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5, t_6} (n = 6 unit-length tasks)
m = 3 processors, r = 1 resource, B_1 = 20, D = 2.
Resource requirements: R_1(t_1) = 6, R_1(t_2) = 7, R_1(t_3) = 7, R_1(t_4) = 6, R_1(t_5) = 8, R_1(t_6) = 6.

**Feasible schedule:**
Time slot 0: {t_1, t_2, t_3} -- 3 tasks <= 3 processors, resource usage = 6+7+7 = 20 <= 20.
Time slot 1: {t_4, t_5, t_6} -- 3 tasks <= 3 processors, resource usage = 6+8+6 = 20 <= 20.

Answer: YES -- a valid schedule meeting deadline D = 2 and resource bound B_1 = 20 exists.
