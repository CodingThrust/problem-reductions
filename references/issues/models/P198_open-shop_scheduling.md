---
name: Problem
about: Propose a new problem type
title: "[Model] OpenShopScheduling"
labels: model
assignees: ''
---

## Motivation

OPEN-SHOP SCHEDULING (P198) from Garey & Johnson, A5 SS14. A classical NP-complete scheduling problem in which each job consists of one task per machine, and tasks of the same job may be processed in any order (unlike flow-shop where the order is fixed). The goal is to find a non-preemptive schedule minimizing the makespan (or meeting a deadline D). NP-complete for m >= 3 machines by reduction from PARTITION [Gonzalez and Sahni, 1976], but solvable in polynomial time for m = 2 and for the preemptive variant with any number of machines.

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R143: Partition -> Open-Shop Scheduling (this model is the **target**)

## Definition

**Name:** `OpenShopScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS14

**Mathematical definition:**

INSTANCE: Number m in Z+ of processors (machines), set J of jobs, each job j in J consisting of m tasks t_1[j], t_2[j], ..., t_m[j] (with t_i[j] to be executed by processor i), a length l(t) in Z_0+ for each such task t, and an overall deadline D in Z+.
QUESTION: Is there an open-shop schedule for J that meets the deadline, i.e., a collection of one-processor schedules sigma_i: J -> Z_0+, 1 <= i <= m, such that sigma_i(j) > sigma_i(k) implies sigma_i(j) >= sigma_i(k) + l(t_i[k]), such that for each j in J the intervals [sigma_i(j), sigma_i(j) + l(t_i[j])) are all disjoint, and such that sigma_i(j) + l(t_i[j]) <= D for 1 <= i <= m, 1 <= j <= |J|?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n * m variables, where n = |J| jobs and m = number of machines. Each variable sigma_i(j) represents the start time of job j on machine i.
- **Per-variable domain:** {0, 1, ..., D - max_length} -- integer start times within the deadline
- **Meaning:** sigma_i(j) is the start time of job j's task on machine i. The constraints are: (1) on each machine, tasks do not overlap (non-preemptive single-machine schedules), (2) for each job, its tasks on different machines do not overlap in time (each job occupies at most one machine at a time), and (3) all tasks complete by deadline D.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `OpenShopScheduling`
**Variants:** none (no type parameters)

| Field              | Type              | Description                                                  |
|--------------------|-------------------|--------------------------------------------------------------|
| `num_machines`     | `usize`           | Number of machines (processors) m                            |
| `processing_times` | `Vec<Vec<u64>>`   | Processing time matrix: p[j][i] = l(t_i[j]) for job j, machine i (n x m) |
| `deadline`         | `u64`             | Overall deadline D                                           |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** NP-complete for m >= 3 machines [Gonzalez and Sahni, 1976]; NP-complete in the strong sense for arbitrary m [Lenstra, 1977]. Polynomial for m = 2 machines. Whether the problem admits a pseudo-polynomial algorithm for fixed m >= 3 is a long-standing open question in scheduling theory. The preemptive variant is solvable in polynomial time for any m [Gonzalez and Sahni, 1976]. For the NP-hard non-preemptive case, exact approaches use branch-and-bound, ILP, or constraint programming.

## Extra Remark

**Full book text:**

INSTANCE: Number m in Z+ of processors, set J of jobs, each job j in J consisting of m tasks t1[j],t2[j], ..., tm[j] (with ti[j] to be executed by processor i), a length l(t) in Z0+ for each such task t, and an overall deadline D in Z+.
QUESTION: Is there an open-shop schedule for J that meets the deadline, i.e., a collection of one-processor schedules sigmai: J->Z0+, 1 <= i <= m, such that sigmai(j) > sigmai(k) implies sigmai(j) >= sigmai(k) + l(ti[k]), such that for each j in J the intervals [sigmai(j), sigmai(j) + l(ti[j])) are all disjoint, and such that sigmai(j) + l(ti[j]) <= D for 1 <= i <= m, 1 <= j <= |J|?

Reference: [Gonzalez and Sahni, 1976]. Transformation from PARTITION.

Comment: Remains NP-complete if m = 3, but can be solved in polynomial time if m = 2. NP-complete in the strong sense for m arbitrary [Lenstra, 1977]. The general problem is solvable in polynomial time if "preemptive" schedules are allowed [Gonzalez and Sahni, 1976], even if two distinct release times are allowed [Cho and Sahni, 1978]. The m = 2 preemptive case can be solved in polynomial time even if arbitrary release times are allowed, and the general preemptive case with arbitrary release times and deadlines can be solved by linear programming [Cho and Sahni, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all possible orderings of jobs on each machine and valid start times; check deadline, non-overlap on machines, and non-overlap across machines for each job.)
- [x] It can be solved by reducing to integer programming. (Binary ILP with start-time variables and non-overlap constraints: for each pair of tasks on the same machine, one must complete before the other starts; for each pair of tasks of the same job on different machines, they must not overlap.)
- [ ] Other: Polynomial for m = 2 (Gonzalez-Sahni algorithm); preemptive case solvable in polynomial time by LP.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
m = 3 machines, J = {J_1, J_2, J_3, J_4, J_5, J_6} (n = 6 jobs), D = 30.

Processing time matrix (each job has the same time on all 3 machines, except J_6):

| Job   | Machine 1 | Machine 2 | Machine 3 |
|-------|-----------|-----------|-----------|
| J_1   | 4         | 4         | 4         |
| J_2   | 5         | 5         | 5         |
| J_3   | 3         | 3         | 3         |
| J_4   | 2         | 2         | 2         |
| J_5   | 6         | 6         | 6         |
| J_6   | 10        | 10        | 10        |

(This corresponds to the PARTITION reduction with A = {4, 5, 3, 2, 6}, Q = 10, D = 3Q = 30.)

**Feasible schedule (sketch):**
J_6 (the big job) is scheduled: Machine 1 in [0,10), Machine 2 in [10,20), Machine 3 in [20,30).
Jobs {J_1, J_5} (partition half {4,6}, sum=10) fill gaps on one side.
Jobs {J_2, J_3, J_4} (partition half {5,3,2}, sum=10) fill gaps on the other side.

All jobs complete by time 30 = D.

Answer: YES -- a valid open-shop schedule meeting deadline D = 30 exists.
