---
name: Problem
about: Propose a new problem type
title: "[Model] TwoProcessorFlowShopWithBoundedBuffer"
labels: model
assignees: ''
---

## Motivation

TWO-PROCESSOR FLOW-SHOP WITH BOUNDED BUFFER (P201) from Garey & Johnson, A5 SS17. A specialization of flow-shop scheduling to exactly 2 machines, with the additional constraint that at most B jobs can wait in the intermediate buffer between machines 1 and 2. A job enters the buffer after completing on machine 1 and leaves when it starts on machine 2. This buffer constraint models physical storage limitations in manufacturing lines. NP-complete in the strong sense for any fixed B >= 1 [Papadimitriou and Kanellakis, 1980]; solvable in polynomial time if B = 0 (reduces to no-wait 2-machine flow shop, solvable by Gilmore-Gomory) or if B >= |J| - 1 (unconstrained buffer, solvable by Johnson's rule).

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R146: NUMERICAL 3-DIMENSIONAL MATCHING -> TWO-PROCESSOR FLOW-SHOP WITH BOUNDED BUFFER (establishes strong NP-completeness for any fixed B >= 1)

## Definition

**Name:** `TwoProcessorFlowShopBoundedBuffer`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS17

**Mathematical definition:**

INSTANCE: (Same as for FLOW-SHOP SCHEDULING with m = 2, with the addition of a "buffer bound" B in Z0+.)
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and such that, for all u >= 0, the number of jobs j in J for which both sigma_1(j) + l(t_1[j]) <= u and sigma_2(j) > u does not exceed B?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |J| (one variable per job, representing its position in the job sequence on each machine)
- **Per-variable domain:** {0, 1, ..., n-1} -- the position of job j in the schedule
- **Meaning:** pi(j) in {0, ..., n-1} is the position of job j in the processing sequence on machines 1 and 2 (for permutation schedules, both machines process jobs in the same order). After completing on machine 1, a job waits in the buffer until machine 2 is free. The buffer constraint limits how many jobs can be simultaneously waiting.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `TwoProcessorFlowShopBoundedBuffer`
**Variants:** none (task lengths are non-negative integers)

| Field              | Type         | Description                                                    |
|--------------------|--------------|----------------------------------------------------------------|
| `task_lengths_m1`  | `Vec<u64>`   | Processing time of each job on machine 1: l(t_1[j])           |
| `task_lengths_m2`  | `Vec<u64>`   | Processing time of each job on machine 2: l(t_2[j])           |
| `buffer_bound`     | `usize`      | Maximum number of jobs B that can wait between machines        |
| `deadline`         | `u64`        | Global deadline D; every job must finish by time D             |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** For B = 0 (no buffer = no-wait), solvable in O(n log n) [Gilmore and Gomory, 1964]. For B >= n-1 (unlimited buffer), solvable in O(n log n) by Johnson's rule [Johnson, 1954]. For any fixed B with 1 <= B < n-1, the problem is strongly NP-hard. No known exact algorithm improves significantly upon O(n!) brute-force enumeration of permutation schedules. Branch-and-bound and dynamic programming approaches are used in practice.

## Extra Remark

**Full book text:**

INSTANCE: (Same as for FLOW-SHOP SCHEDULING with m = 2, with the addition of a "buffer bound" B in Z0+.)
QUESTION: Is there a flow-shop schedule for J that meets the overall deadline and such that, for all u >= 0, the number of jobs j in J for which both sigma_1(j) + l(t_1[j]) <= u and sigma_2(j) > u does not exceed B?

Reference: [Papadimitriou and Kanellakis, 1978]. Transformation from NUMERICAL 3-DIMENSIONAL MATCHING.

Comment: NP-complete in the strong sense for any fixed B, 1 <= B < infinity. Solvable in polynomial time if B = 0 [Gilmore and Gomory, 1964] or if B >= |J|-1 [Johnson, 1954].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! permutations; for each, simulate the schedule on both machines with buffer tracking and check makespan <= D and buffer <= B at all times.)
- [x] It can be solved by reducing to integer programming. (ILP with binary sequencing variables, precedence constraints, and buffer occupancy constraints at each time point.)
- [ ] Other: Branch-and-bound with buffer-aware pruning.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
J = {j_1, j_2, j_3, j_4, j_5} (n = 5 jobs), Buffer bound B = 1, Deadline D = 22.

| Job  | Machine 1 | Machine 2 |
|------|-----------|-----------|
| j_1  | 3         | 5         |
| j_2  | 4         | 2         |
| j_3  | 2         | 4         |
| j_4  | 5         | 3         |
| j_5  | 3         | 4         |

**Feasible schedule (sequence j_1, j_3, j_5, j_4, j_2):**
Machine 1: j_1[0,3], j_3[3,5], j_5[5,8], j_4[8,13], j_2[13,17]
Machine 2: j_1[3,8], j_3[8,12], j_5[12,16], j_4[16,19], j_2[19,21]

Buffer check at each transition:
- At t=5: j_3 done on M1, j_1 still on M2 -> j_3 in buffer. Count = 1 <= B ✓
- At t=8: j_5 done on M1, j_3 on M2 (started at 8) -> j_5 in buffer? No, j_3 starts at 8. j_5 done at 8, j_4 not done on M1 yet. Only j_5 waiting. Count = 1 <= B ✓

Makespan = 21 <= D = 22. ✓

Answer: YES -- a valid 2-processor flow-shop schedule with buffer bound B = 1 meeting deadline D = 22 exists.
