---
name: Problem
about: Propose a new problem type
title: "[Model] FlowShopScheduling"
labels: model
assignees: ''
---

## Motivation

FLOW-SHOP SCHEDULING (P199) from Garey & Johnson, A5 SS15. A classical NP-complete scheduling problem: given m processors and a set of jobs, each consisting of m tasks (one per processor) that must be processed in processor order 1, 2, ..., m, can all jobs be completed by a global deadline D? The flow-shop constraint requires each job to visit every machine in the same fixed order, with no job allowed to start its task on machine i+1 until its task on machine i is completed. NP-complete in the strong sense for m = 3 [Garey, Johnson, and Sethi, 1976]; solvable in polynomial time for m = 2 via Johnson's rule [Johnson, 1954].

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R144: 3-PARTITION -> FLOW-SHOP SCHEDULING (establishes strong NP-completeness for m = 3)

## Definition

**Name:** `FlowShopScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS15

**Mathematical definition:**

INSTANCE: Number m in Z+ of processors, set J of jobs, each job j in J consisting of m tasks t1[j],t2[j], ..., tm[j], a length l(t) in Z0+ for each such task t, and an overall deadline D in Z+.
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline, where such a schedule is identical to an open-shop schedule with the additional constraint that, for each j in J and 1 <= i < m, sigma_{i+1}(j) >= sigma_i(j) + l(t_i[j])?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |J| (one variable per job, representing its position in the permutation schedule)
- **Per-variable domain:** {0, 1, ..., n-1} -- the position of job j in the job sequence (for permutation schedules, which are optimal for m = 2 and a natural encoding for general m)
- **Meaning:** pi(j) in {0, ..., n-1} is the position of job j in the processing sequence. In a flow-shop, once the sequence is fixed, the start times on each machine are determined by the precedence and no-overlap constraints. The schedule is feasible iff all jobs complete by time D.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `FlowShopScheduling`
**Variants:** none (task lengths are non-negative integers)

| Field             | Type              | Description                                                      |
|-------------------|-------------------|------------------------------------------------------------------|
| `num_processors`  | `usize`           | Number of machines m                                             |
| `task_lengths`    | `Vec<Vec<u64>>`   | task_lengths[j][i] = length of task t_{i+1}[j] on machine i+1   |
| `deadline`        | `u64`             | Global deadline D; every job must finish by time D               |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** For m = 2, solvable in O(n log n) by Johnson's rule [Johnson, 1954]. For m = 3, the problem is strongly NP-hard; a dynamic programming approach achieves O*(3^n) time [exact exponential algorithms for 3-machine flowshop, Lente et al., 2018]. For general m (part of input), strongly NP-hard; brute-force over n! permutations gives O(n! * mn) time. No known algorithm with complexity significantly better than O(n!) for general m.

## Extra Remark

**Full book text:**

INSTANCE: Number m in Z+ of processors, set J of jobs, each job j in J consisting of m tasks t1[j],t2[j], ..., tm[j], a length l(t) in Z0+ for each such task t, and an overall deadline D in Z+.
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline, where such a schedule is identical to an open-shop schedule with the additional constraint that, for each j in J and 1 <= i < m, sigma_{i+1}(j) >= sigma_i(j) + l(t_i[j])?

Reference: [Garey, Johnson, and Sethi, 1976]. Transformation from 3-PARTITION.

Comment: NP-complete in the strong sense for m = 3. Solvable in polynomial time for m = 2 [Johnson, 1954]. The same results hold if "preemptive" schedules are allowed [Gonzalez and Sahni, 1978a], although if release times are added in this case, the problem is NP-complete in the strong sense, even for m = 2 [Cho and Sahni, 1978]. If the goal is to meet a bound K on the sum, over all j in J, of sigma_m(j) + l(t_m[j]), then the non-preemptive problem is NP-complete in the strong sense even if m = 2 [Garey, Johnson, and Sethi, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! permutations of jobs; for each, compute the schedule greedily and check if makespan <= D.)
- [x] It can be solved by reducing to integer programming. (ILP with binary variables x_{j,k} = 1 if job j is in position k; add precedence constraints for each machine and no-overlap constraints.)
- [ ] Other: Johnson's algorithm for m = 2.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
m = 3 machines, J = {j_1, j_2, j_3, j_4, j_5} (n = 5 jobs)

| Job  | Machine 1 | Machine 2 | Machine 3 |
|------|-----------|-----------|-----------|
| j_1  | 3         | 4         | 2         |
| j_2  | 2         | 3         | 5         |
| j_3  | 4         | 1         | 3         |
| j_4  | 1         | 5         | 4         |
| j_5  | 3         | 2         | 3         |

Deadline D = 25.

**Feasible schedule (sequence j_3, j_1, j_5, j_4, j_2):**

Machine 1: j_3[0,4], j_1[4,7], j_5[7,10], j_4[10,11], j_2[11,13]
Machine 2: j_3[4,5], j_1[7,11], j_5[11,13], j_4[13,18], j_2[18,21]
Machine 3: j_3[5,8], j_1[11,13], j_5[13,16], j_4[18,22], j_2[22,27]

Makespan = 27 > 25. Let us try sequence j_4, j_1, j_5, j_3, j_2:

Machine 1: j_4[0,1], j_1[1,4], j_5[4,7], j_3[7,11], j_2[11,13]
Machine 2: j_4[1,6], j_1[6,10], j_5[10,12], j_3[12,13], j_2[13,16]
Machine 3: j_4[6,10], j_1[10,12], j_5[12,15], j_3[15,18], j_2[18,23]

Makespan = 23 <= 25. ✓

Answer: YES -- a valid flow-shop schedule meeting deadline D = 25 exists.
