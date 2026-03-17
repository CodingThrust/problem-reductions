---
name: Problem
about: Propose a new problem type
title: "[Model] MultiprocessorScheduling"
labels: model
assignees: ''
---

## Motivation

MULTIPROCESSOR SCHEDULING (P192) from Garey & Johnson, A5 SS8. A fundamental NP-complete scheduling problem: given n tasks with integer lengths and m identical processors, can all tasks be completed by a global deadline D? It generalizes PARTITION (taking m=2 and D = half the total task length) and is a key target for reductions from partition-type problems. For fixed m the problem is weakly NP-hard (pseudo-polynomial DP exists); for m as part of the input it is strongly NP-hard (3-PARTITION is a special case).

## Definition

**Name:** `MultiprocessorScheduling`
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS8

**Mathematical definition:**

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, length l(t) ∈ Z+ for each t ∈ T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule for T that meets the overall deadline D, i.e., a function σ:T→Z0+ such that, for all u ≥ 0, the number of tasks t ∈ T for which σ(t) ≤ u < σ(t) + l(t) is no more than m and such that, for all t ∈ T, σ(t) + l(t) ≤ D?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |T| (one discrete variable per task, choosing which processor it is assigned to)
- **Per-variable domain:** {1, 2, ..., m} — the processor index to which the task is assigned
- **Meaning:** p_t ∈ {1,...,m} is the processor for task t. The total load on each processor i must not exceed D. (Since tasks on the same processor must not overlap, this reduces to: Σ_{t: p_t = i} l(t) ≤ D for each i, given that start times can be freely chosen within [0, D − l(t)].)

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `MultiprocessorScheduling`
**Variants:** none (no type parameters; lengths are plain positive integers)

| Field            | Type       | Description                                              |
|------------------|------------|----------------------------------------------------------|
| `lengths`        | `Vec<u64>` | Length l(t) of each task t ∈ T                           |
| `num_processors` | `u64`      | Number of identical processors m                         |
| `deadline`       | `u64`      | Global deadline D; every task must finish by time D      |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** When m is fixed (e.g., m = 2), the problem is weakly NP-hard and can be solved by pseudo-polynomial DP in O(n · D^(m−1)) time. For general m (as part of the input), the problem is strongly NP-hard. No known exact algorithm improves upon O*(2^n) brute-force enumeration of all processor assignments in the strongly NP-hard general case. [Garey & Johnson, 1979; Lenstra, Rinnooy Kan & Brucker, *Op. Res.*, 1977.]

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, number m ∈ Z+ of processors, length l(t) ∈ Z+ for each t ∈ T, and a deadline D ∈ Z+.
QUESTION: Is there an m-processor schedule for T that meets the overall deadline D, i.e., a function σ:T→Z0+ such that, for all u ≥ 0, the number of tasks t ∈ T for which σ(t) ≤ u < σ(t) + l(t) is no more than m and such that, for all t ∈ T, σ(t) + l(t) ≤ D?

Reference: Transformation from PARTITION (see Section 3.2.1).

Comment: Remains NP-complete for m = 2, but can be solved in pseudo-polynomial time for any fixed m. NP-complete in the strong sense for m arbitrary (3-PARTITION is a special case). If all tasks have the same length, then this problem is trivial to solve in polynomial time, even for "different speed" processors.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all m^n assignments of tasks to processors; check max load ≤ D.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{t,i} ∈ {0,1}, Σ_i x_{t,i} = 1 for each t, Σ_t x_{t,i} · l(t) ≤ D for each i.)
- [ ] Other: Pseudo-polynomial DP for fixed m (dynamic programming over the load vector of m−1 processors).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
Lengths: l(t_1) = 4, l(t_2) = 5, l(t_3) = 3, l(t_4) = 2, l(t_5) = 6
m = 2 processors, D = 10 (total sum = 20, D = 10 = sum/2).

**Feasible assignment:**
Processor 1: {t_1, t_5} — total load 4 + 6 = 10 ≤ D ✓
Processor 2: {t_2, t_3, t_4} — total load 5 + 3 + 2 = 10 ≤ D ✓

Answer: YES — a valid schedule meeting deadline D = 10 exists.
