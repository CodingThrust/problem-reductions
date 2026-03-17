---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingWithReleaseTimesAndDeadlines"
labels: model
assignees: ''
---

## Motivation

SEQUENCING WITH RELEASE TIMES AND DEADLINES (P185) from Garey & Johnson, A5 SS1. A fundamental single-machine scheduling feasibility problem: given n tasks each with a processing time, release time, and deadline, can all tasks be non-preemptively scheduled on one processor such that each task starts after its release time and finishes by its deadline? This is the first problem in the Sequencing and Scheduling section of Garey & Johnson's appendix and is strongly NP-complete (by reduction from 3-PARTITION). It becomes polynomial when all task lengths are 1, when preemptions are allowed, or when all release times are 0.

**Associated rules:**
- R131: 3-Partition -> Sequencing with Release Times and Deadlines (as target)

<!-- ⚠️ Unverified: AI-generated motivation and associated rules list -->

## Definition

**Name:** `SequencingWithReleaseTimesAndDeadlines`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS1

**Mathematical definition:**

INSTANCE: Set T of tasks and, for each task t in T, a length l(t) in Z+, a release time r(t) in Z0+, and a deadline d(t) in Z+.
QUESTION: Is there a one-processor schedule for T that satisfies the release time constraints and meets all the deadlines, i.e., a one-to-one function sigma:T->Z0+, with sigma(t) > sigma(t') implying sigma(t) >= sigma(t') + l(t'), such that, for all t in T, sigma(t) >= r(t) and sigma(t) + l(t) <= d(t)?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one variable per task, representing the order position or start time)
- **Per-variable domain:** {0, 1, ..., H-1} where H = max d(t) is the time horizon; or equivalently a permutation of tasks
- **Meaning:** sigma(t) is the start time of task t. The schedule is feasible iff: (1) no two tasks overlap, (2) sigma(t) >= r(t) for all t, and (3) sigma(t) + l(t) <= d(t) for all t. This is a satisfaction (decision) problem.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequencingWithReleaseTimesAndDeadlines`
**Variants:** none (no type parameters; all values are plain positive integers)

| Field           | Type       | Description                                          |
|-----------------|------------|------------------------------------------------------|
| `lengths`       | `Vec<u64>` | Processing time l(t) for each task t in T            |
| `release_times` | `Vec<u64>` | Release time r(t) for each task t (>= 0)             |
| `deadlines`     | `Vec<u64>` | Deadline d(t) for each task t (> 0)                   |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is strongly NP-complete, so no pseudo-polynomial-time algorithm exists unless P = NP. Branch-and-bound methods are the primary exact approach. For the general case, brute-force enumeration of all n! task orderings gives O(n! * n) time. When all tasks have unit length (l(t) = 1), the problem is solvable in O(n log n) by earliest-deadline-first scheduling. When preemption is allowed, it is also polynomial. For the general non-preemptive case with arbitrary lengths, release times, and deadlines, no known exact algorithm substantially improves on exponential enumeration. [Garey & Johnson, 1977; Lenstra, Rinnooy Kan & Brucker, 1977.]

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks and, for each task t in T, a length l(t) in Z+, a release time r(t) in Z0+, and a deadline d(t) in Z+.
QUESTION: Is there a one-processor schedule for T that satisfies the release time constraints and meets all the deadlines, i.e., a one-to-one function sigma:T->Z0+, with sigma(t) > sigma(t') implying sigma(t) >= sigma(t') + l(t'), such that, for all t in T, sigma(t) >= r(t) and sigma(t) + l(t) <= d(t)?

Reference: [Garey and Johnson, 1977b]. Transformation from 3-PARTITION (see Section 4.2).

Comment: NP-complete in the strong sense. Solvable in pseudo-polynomial time if the number of allowed values for r(t) and d(t) is bounded by a constant, but remains NP-complete (in the ordinary sense) even when each can take on only two values. If all task lengths are 1, or "preemptions" are allowed, or all release times are 0, the general problem can be solved in polynomial time, even under "precedence constraints" [Lawler, 1973], [Lageweg, Lenstra, and Rinnooy Kan, 1976]. Can also be solved in polynomial time even if release times and deadlines are allowed to be arbitrary rationals and there are precedence constraints, so long as all tasks have equal length [Carlier, 1978], [Simons, 1978], [Garey, Johnson, Simons, and Tarjan, 1978], or preemptions are allowed [Blazewicz, 1976].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! orderings of tasks; for each ordering greedily assign start times and check feasibility.)
- [x] It can be solved by reducing to integer programming. (ILP: integer start-time variables sigma_t, constraints sigma_t >= r(t), sigma_t + l(t) <= d(t), and disjunctive constraints for non-overlap: sigma_t + l(t) <= sigma_{t'} or sigma_{t'} + l(t') <= sigma_t for all pairs.)
- [ ] Other: Branch-and-bound with constraint propagation (practical exact solver for moderate n).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
Lengths: l(t_1) = 3, l(t_2) = 2, l(t_3) = 4, l(t_4) = 1, l(t_5) = 2
Release times: r(t_1) = 0, r(t_2) = 1, r(t_3) = 5, r(t_4) = 0, r(t_5) = 8
Deadlines: d(t_1) = 5, d(t_2) = 6, d(t_3) = 10, d(t_4) = 3, d(t_5) = 12

**Feasible schedule:**
- sigma(t_4) = 0, runs [0, 1) -- r=0 <= 0, finish 1 <= d=3
- sigma(t_1) = 1, runs [1, 4) -- r=0 <= 1, finish 4 <= d=5
- sigma(t_2) = 4, runs [4, 6) -- r=1 <= 4, finish 6 <= d=6
- sigma(t_3) = 6, runs [6, 10) -- r=5 <= 6, finish 10 <= d=10
- sigma(t_5) = 10, runs [10, 12) -- r=8 <= 10, finish 12 <= d=12

All tasks within their release-deadline windows, no overlaps. Feasible schedule exists.

Answer: YES.
