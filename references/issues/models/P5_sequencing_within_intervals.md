---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingWithinIntervals"
labels: model
assignees: ''
---

## Motivation

SEQUENCING WITHIN INTERVALS (P5) from Garey & Johnson, Chapter 3, Section 3.2.2, p.70. A canonical NP-complete single-machine scheduling problem: given tasks each with a release time, deadline, and length, can all tasks be non-overlappingly scheduled so that each runs strictly within its allowed window? Its NP-completeness (Theorem 3.8 in GJ) is proved by reduction from PARTITION using an "enforcer" task that pins the schedule at the midpoint of the time horizon, forcing a balanced split. This is historically significant as one of the first NP-completeness results for single-machine scheduling.

## Definition

**Name:** `SequencingWithinIntervals`
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.2.2, p.70

**Mathematical definition:**

INSTANCE: A finite set T of "tasks" and, for each t ∈ T, an integer "release time" r(t) ≥ 0, a "deadline" d(t) ∈ Z+, and a "length" l(t) ∈ Z+.
QUESTION: Does there exist a feasible schedule for T, that is, a function σ: T → Z+ such that, for each t ∈ T, σ(t) ≥ r(t), σ(t)+l(t) ≤ d(t), and, if t' ∈ T−{t}, then either σ(t')+l(t') ≤ σ(t) or σ(t') ≥ σ(t)+l(t)? (The task t is "executed" from time σ(t) to time σ(t)+l(t), cannot start executing until time r(t), must be completed by time d(t), and its execution cannot overlap the execution of any other task t'.)

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |T| (one integer variable per task, choosing its start time)
- **Per-variable domain:** For each task t, σ(t) ∈ {r(t), r(t)+1, ..., d(t)−l(t)} (all valid integer start times)
- **Meaning:** σ(t) is the start time of task t. The constraint σ(t)+l(t) ≤ d(t) ensures t finishes before its deadline; σ(t) ≥ r(t) ensures t starts after its release time. Non-overlap constraints between all pairs of tasks are additional feasibility conditions.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `SequencingWithinIntervals`
**Variants:** none (no type parameters; all times and lengths are plain non-negative integers)

| Field          | Type       | Description                                              |
|----------------|------------|----------------------------------------------------------|
| `release_times`| `Vec<u64>` | Release time r(t) ≥ 0 for each task t ∈ T               |
| `deadlines`    | `Vec<u64>` | Deadline d(t) ∈ Z+ for each task t ∈ T                  |
| `lengths`      | `Vec<u64>` | Processing length l(t) ∈ Z+ for each task t ∈ T         |

(All three vectors have the same length n = |T|; index i corresponds to task t_i.)

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** The problem is NP-complete (Garey & Johnson, Theorem 3.8, 1979). For general instances, the best known exact algorithms are exponential in n. Brute-force enumeration of all orderings of tasks runs in O(n! · n) time. Branch-and-bound algorithms are used in practice but remain exponential in the worst case. No known algorithm improves substantially upon O*(2^n). [Garey & Johnson, 1979; Lenstra & Rinnooy Kan, 1978.]

## Extra Remark

**Full book text:**

INSTANCE: A finite set T of "tasks" and, for each t ∈ T, an integer "release time" r(t) ≥ 0, a "deadline" d(t) ∈ Z+, and a "length" l(t) ∈ Z+.
QUESTION: Does there exist a feasible schedule for T, that is, a function σ: T → Z+ such that, for each t ∈ T, σ(t) ≥ r(t), σ(t)+l(t) ≤ d(t), and, if t' ∈ T−{t}, then either σ(t')+l(t') ≤ σ(t) or σ(t') ≥ σ(t)+l(t)? (The task t is "executed" from time σ(t) to time σ(t)+l(t), cannot start executing until time r(t), must be completed by time d(t), and its execution cannot overlap the execution of any other task t'.)

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! orderings; for each ordering check whether tasks can be placed feasibly without violating release/deadline/non-overlap constraints.)
- [x] It can be solved by reducing to integer programming. (Binary ILP with ordering variables x_{tt'} ∈ {0,1} indicating whether t precedes t', plus start-time variables; this gives an ILP with O(n^2) variables and constraints.)
- [ ] Other: Constraint programming / branch-and-bound with interval propagation is effective in practice.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input (from the PARTITION → SEQUENCING WITHIN INTERVALS reduction):**
Source partition: A = {3, 1, 2, 4}, total sum B = 10.

| Task | Release r | Deadline d | Length l | Notes            |
|------|-----------|------------|----------|------------------|
| t_1  | 0         | 11         | 3        | element a_1 = 3  |
| t_2  | 0         | 11         | 1        | element a_2 = 1  |
| t_3  | 0         | 11         | 2        | element a_3 = 2  |
| t_4  | 0         | 11         | 4        | element a_4 = 4  |
| t̄   | 5         | 6          | 1        | enforcer task    |

**Feasible schedule:**
- σ(t_1) = 0: runs [0, 3)
- σ(t_3) = 3: runs [3, 5)
- σ(t̄) = 5: runs [5, 6) (pinned by r = d − 1 = 5)
- σ(t_2) = 6: runs [6, 7)
- σ(t_4) = 7: runs [7, 11)

All tasks start ≥ r, finish ≤ d, no overlaps. Feasible schedule ✓

Answer: YES — a feasible schedule exists.
