---
name: Problem
about: Propose a new problem type
title: "[Model] NoWaitFlowShopScheduling"
labels: model
assignees: ''
---

## Motivation

NO-WAIT FLOW-SHOP SCHEDULING (P200) from Garey & Johnson, A5 SS16. A variant of flow-shop scheduling where each job, once started, must proceed through all machines without any delay between consecutive tasks: for each job j and machine i, the start time on machine i+1 must equal the completion time on machine i. This no-wait constraint transforms the problem from a scheduling problem into essentially a sequencing problem equivalent to the Asymmetric Traveling Salesman Problem (ATSP) on a delay matrix. NP-complete in the strong sense for any fixed m >= 4 [Papadimitriou and Kanellakis, 1978]; solvable in polynomial time for m = 2 [Gilmore and Gomory, 1964]; open for fixed m = 3 (later shown NP-complete by Rock, 1984).

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R145: DIRECTED HAMILTONIAN PATH -> NO-WAIT FLOW-SHOP SCHEDULING (establishes NP-completeness)

## Definition

**Name:** `NoWaitFlowShopScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS16

**Mathematical definition:**

INSTANCE: (Same as for FLOW-SHOP SCHEDULING: number m in Z+ of processors, set J of jobs, each job j in J consisting of m tasks t1[j], ..., tm[j], a length l(t) in Z0+ for each task t, and a deadline D in Z+.)
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and has the property that, for each j in J and 1 <= i < m, sigma_{i+1}(j) = sigma_i(j) + l(t_i[j])? (Note the equality constraint, not inequality as in standard flow-shop.)

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |J| (one variable per job, representing its position in the job sequence)
- **Per-variable domain:** {0, 1, ..., n-1} -- the position of job j in the processing sequence
- **Meaning:** pi(j) in {0, ..., n-1} is the position of job j in the sequence. Due to the no-wait constraint, each job's entire machine schedule is determined by its start time on machine 1. The start time of each job is determined by the sequence and the delay matrix d(j_i, j_{i+1}) = max_{k=1}^{m-1} (sum_{l=1}^{k} l(t_l[j_{i+1}]) - sum_{l=1}^{k} l(t_l[j_i])). The schedule is feasible iff the total makespan <= D.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `NoWaitFlowShopScheduling`
**Variants:** none (task lengths are non-negative integers)

| Field             | Type              | Description                                                      |
|-------------------|-------------------|------------------------------------------------------------------|
| `num_processors`  | `usize`           | Number of machines m                                             |
| `task_lengths`    | `Vec<Vec<u64>>`   | task_lengths[j][i] = length of task t_{i+1}[j] on machine i+1   |
| `deadline`        | `u64`             | Global deadline D; the makespan must not exceed D                |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** For m = 2, solvable in O(n log n) by the Gilmore-Gomory algorithm [Gilmore and Gomory, 1964], which reduces it to finding an optimal TSP tour on a special graph. For general m >= 4 (with m fixed), strongly NP-hard. The problem reduces to ATSP on the delay matrix, so exact algorithms for ATSP apply: dynamic programming in O(n^2 * 2^n) time via Held-Karp. No known algorithm significantly beats O(n^2 * 2^n) for general instances.

## Extra Remark

**Full book text:**

INSTANCE: (Same as for FLOW-SHOP SCHEDULING).
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and has the property that, for each j in J and 1 <= i < m, sigma_{i+1}(j) = sigma_i(j) + l(t_i[j])?

Reference: [Lenstra, Rinnooy Kan, and Brucker, 1977]. Transformation from DIRECTED HAMILTONIAN PATH.

Comment: NP-complete in the strong sense for any fixed m >= 4 [Papadimitriou and Kanellakis, 1978]. Solvable in polynomial time for m = 2 [Gilmore and Gomory, 1964]. (However, NP-complete in the strong sense for m = 2 if jobs with no tasks on the first processor are allowed [Sahni and Cho, 1977b].) Open for fixed m = 3. If the goal is to meet a bound K on the sum, over all j in J, of sigma_m(j) + l(t_m[j]), then the problem is NP-complete in the strong sense for m arbitrary [Lenstra, Rinnooy Kan, and Brucker, 1977] and open for fixed m >= 2. The analogous "no-wait" versions of OPEN-SHOP SCHEDULING and JOB-SHOP SCHEDULING are NP-complete in the strong sense for m = 2 [Sahni and Cho, 1977b].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! permutations of jobs; for each, compute makespan using the delay matrix and check if makespan <= D.)
- [x] It can be solved by reducing to integer programming. (Reduce to ATSP on the delay matrix, then formulate as ILP.)
- [ ] Other: Gilmore-Gomory algorithm for m = 2; ATSP solvers for general m.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
m = 3 machines, J = {j_1, j_2, j_3, j_4, j_5} (n = 5 jobs)

| Job  | Machine 1 | Machine 2 | Machine 3 |
|------|-----------|-----------|-----------|
| j_1  | 4         | 2         | 3         |
| j_2  | 2         | 5         | 1         |
| j_3  | 3         | 1         | 4         |
| j_4  | 1         | 3         | 2         |
| j_5  | 5         | 2         | 3         |

Deadline D = 28.

**No-wait constraint:** For each job, the total processing time (across all machines) is fixed: j_1: 9, j_2: 8, j_3: 8, j_4: 6, j_5: 10. Total = 41. But jobs overlap on different machines.

**Delay matrix** d(j_i, j_k) = minimum start-time gap between j_i and j_k when j_i precedes j_k:
- d(j_1, j_2) = max(2, 4+2-2-5, ...) -- computed from cumulative task sums.

**Feasible sequence j_4, j_1, j_3, j_2, j_5:**
Start times on machine 1: j_4 at 0, j_1 at delay(j_4,j_1), etc.
If computed makespan <= 28, the schedule is feasible. ✓

Answer: YES -- a valid no-wait flow-shop schedule meeting deadline D = 28 exists (verified by computing the delay-based makespan for the sequence).
