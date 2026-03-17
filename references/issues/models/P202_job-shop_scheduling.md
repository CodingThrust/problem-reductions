---
name: Problem
about: Propose a new problem type
title: "[Model] JobShopScheduling"
labels: model
assignees: ''
---

## Motivation

JOB-SHOP SCHEDULING (P202) from Garey & Johnson, A5 SS18. One of the most studied NP-hard combinatorial optimization problems: given m processors and a set of jobs, each consisting of an ordered sequence of tasks with specified processor assignments and lengths, can all jobs be completed by a global deadline D? Unlike flow-shop scheduling, each job can have a different machine routing, and consecutive tasks of the same job must be on different processors. NP-complete in the strong sense even for m = 2 [Garey, Johnson, and Sethi, 1976]. Solvable in polynomial time for m = 2 with at most 2 tasks per job [Jackson, 1956].

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R147: 3-PARTITION -> JOB-SHOP SCHEDULING (establishes strong NP-completeness for m = 2)

## Definition

**Name:** `JobShopScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS18

**Mathematical definition:**

INSTANCE: Number m in Z+ of processors, set J of jobs, each j in J consisting of an ordered collection of tasks t_k[j], 1 <= k <= n_j, for each such task t a length l(t) in Z0+ and a processor p(t) in {1,2,...,m}, where p(t_k[j]) != p(t_{k+1}[j]) for all j in J and 1 <= k < n_j, and a deadline D in Z+.
QUESTION: Is there a job-shop schedule for J that meets the overall deadline, i.e., a collection of one-processor schedules sigma_i mapping {t: p(t) = i} into Z0+, 1 <= i <= m, such that sigma_i(t) > sigma_i(t') implies sigma_i(t) >= sigma_i(t') + l(t), such that sigma(t_{k+1}[j]) >= sigma(t_k[j]) + l(t_k[j]) for all j in J and 1 <= k < n_j, and such that for all j in J sigma(t_{n_j}[j]) + l(t_{n_j}[j]) <= D?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** Total number of tasks T = sum_{j in J} n_j (one start-time variable per task)
- **Per-variable domain:** {0, 1, ..., D-1} -- the start time of each task
- **Meaning:** sigma(t) in {0, ..., D - l(t)} is the start time of task t. Constraints: (1) tasks on the same machine do not overlap, (2) consecutive tasks of the same job respect precedence (task k+1 starts after task k finishes), (3) all tasks finish by deadline D.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `JobShopScheduling`
**Variants:** none (task lengths are non-negative integers)

| Field             | Type                        | Description                                                                 |
|-------------------|-----------------------------|-----------------------------------------------------------------------------|
| `num_processors`  | `usize`                     | Number of machines m                                                        |
| `jobs`            | `Vec<Vec<(usize, u64)>>`    | jobs[j][k] = (processor, length) for the k-th task of job j                |
| `deadline`        | `u64`                       | Global deadline D; every job must finish by time D                          |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** For m = 2 with n_j <= 2 for all j, solvable in polynomial time by Jackson's rule [Jackson, 1956]. For the general case (m >= 2 with arbitrary task counts), strongly NP-hard. The best known exact approaches use dynamic programming (Gromicho et al., 2012) or branch-and-bound (Brucker et al., 1994). Brute-force requires examining all possible orderings of tasks on each machine, giving factorial complexity. The Held-Karp style DP approach for related problems achieves O*(2^T) where T is the total number of tasks, but no significantly better bound is known for general job-shop instances.

## Extra Remark

**Full book text:**

INSTANCE: Number m in Z+ of processors, set J of jobs, each j in J consisting of an ordered collection of tasks t_k[j], 1 <= k <= n_j, for each such task t a length l(t) in Z0+ and a processor p(t) in {1,2,...,m}, where p(t_k[j]) != p(t_{k+1}[j]) for all j in J and 1 <= k < n_j, and a deadline D in Z+.
QUESTION: Is there a job-shop schedule for J that meets the overall deadline, i.e., a collection of one-processor schedules sigma_i mapping {t: p(t) = i} into Z0+, 1 <= i <= m, such that sigma_i(t) > sigma_i(t') implies sigma_i(t) >= sigma_i(t') + l(t), such that sigma(t_{k+1}[j]) >= sigma(t_k[j]) + l(t_k[j]) for all j in J and 1 <= k < n_j, and such that for all j in J sigma(t_{n_j}[j]) + l(t_{n_j}[j]) <= D?

Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense for m = 2. Can be solved in polynomial time if m = 2 and n_j <= 2 for all j in J [Jackson, 1956]. NP-complete (in the ordinary sense) if m = 2 and n_j <= 3 for all j in J, or if m = 3 and n_j <= 2 for all j in J [Gonzalez and Sahni, 1978a]. All the above results continue to hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a]. If in the nonpreemptive case all tasks have the same length, the problem is NP-complete for m = 3 and open for m = 2 [Lenstra and Rinnooy Kan, 1978b].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all possible orderings of tasks on each machine; check precedence and deadline constraints.)
- [x] It can be solved by reducing to integer programming. (ILP with start-time variables sigma(t), precedence constraints, and disjunctive constraints for tasks sharing a machine: for each pair (t, t') on the same machine, either sigma(t) + l(t) <= sigma(t') or sigma(t') + l(t') <= sigma(t).)
- [ ] Other: Disjunctive graph / branch-and-bound (Brucker et al., 1994); constraint programming.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
m = 2 processors, J = {j_1, j_2, j_3, j_4, j_5} (n = 5 jobs), Deadline D = 20.

| Job  | Task 1 (proc, len) | Task 2 (proc, len) | Task 3 (proc, len) |
|------|---------------------|---------------------|---------------------|
| j_1  | (P1, 3)            | (P2, 4)            | --                  |
| j_2  | (P2, 2)            | (P1, 3)            | (P2, 2)            |
| j_3  | (P1, 4)            | (P2, 3)            | --                  |
| j_4  | (P2, 5)            | (P1, 2)            | --                  |
| j_5  | (P1, 2)            | (P2, 3)            | (P1, 1)            |

**Feasible schedule:**
P1: j_1.t1[0,3], j_3.t1[3,7], j_5.t1[7,9], j_2.t2[9,12], j_4.t2[12,14], j_5.t3[14,15]
P2: j_2.t1[0,2], j_4.t1[2,7], j_1.t2[7,11], j_3.t2[11,14], j_5.t2[14,17], j_2.t3[17,19]

Job completion times: j_1 at 11, j_2 at 19, j_3 at 14, j_4 at 14, j_5 at 17. All <= D = 20. ✓
Precedence: each job's tasks are in order. ✓
No overlap on each machine. ✓

Answer: YES -- a valid job-shop schedule meeting deadline D = 20 exists.
