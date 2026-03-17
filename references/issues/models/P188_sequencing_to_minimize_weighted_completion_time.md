---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeWeightedCompletionTime"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE WEIGHTED COMPLETION TIME (P188) from Garey & Johnson, A5 SS4. A single-machine scheduling problem with precedence constraints: given n tasks with processing times, weights, and a partial order, find a schedule that minimizes total weighted completion time (sum of w(t) * C(t) where C(t) = sigma(t) + l(t)). This problem is NP-complete in the strong sense by reduction from OPTIMAL LINEAR ARRANGEMENT (Lawler, 1978). It remains strongly NP-hard even when all task lengths are 1 or all weights are 1. The problem becomes polynomial for forest/series-parallel precedence orders. Without precedence constraints, the optimal schedule is simply the Weighted Shortest Job First (WSJF) rule: sort by w(t)/l(t) in decreasing order (Smith, 1956).

**Associated rules:**
- R134: Optimal Linear Arrangement -> Sequencing to Minimize Weighted Completion Time (as target)
- R142: Partition -> Scheduling to Minimize Weighted Completion Time (multi-machine variant SS13, as target)

<!-- ⚠️ Unverified: AI-generated motivation and associated rules list -->

## Definition

**Name:** `SequencingToMinimizeWeightedCompletionTime`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS4

**Mathematical definition:**

INSTANCE: Set T of tasks, partial order < on T, for each task t in T a length l(t) in Z+ and a weight w(t) in Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule sigma for T that obeys the precedence constraints and for which the sum, over all t in T, of (sigma(t) + l(t))*w(t) is K or less?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one variable per task, representing position in the schedule)
- **Per-variable domain:** A permutation of {0, 1, ..., n-1} consistent with the partial order (topological ordering)
- **Meaning:** sigma(t) is the start time of task t. The completion time is C(t) = sigma(t) + l(t). The objective is sum_{t in T} w(t) * C(t), which must be at most K. This is an optimization problem (minimize weighted completion time) posed as a decision problem (is there a schedule with objective <= K?).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequencingToMinimizeWeightedCompletionTime`
**Variants:** none (no type parameters)

| Field        | Type                  | Description                                              |
|--------------|-----------------------|----------------------------------------------------------|
| `lengths`    | `Vec<u64>`            | Processing time l(t) for each task t in T                |
| `weights`    | `Vec<u64>`            | Weight w(t) for each task t in T                         |
| `precedences`| `Vec<(usize, usize)>` | Pairs (i, j) meaning task i must complete before task j starts |
| `bound_k`    | `u64`                 | Maximum allowed total weighted completion time K         |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete in the strong sense with arbitrary precedence constraints (Lawler, 1978). Without precedence constraints, the optimal solution is given by Smith's WSJF rule (sort by decreasing w(t)/l(t)) in O(n log n). For series-parallel precedence orders, Sidney's decomposition (1975) and Lawler's algorithm (1978) solve it in O(n log n). For forest (tree) precedence, polynomial algorithms exist (Horn, 1972; Adolphson & Hu, 1973). For general precedence, the best known exact approach is branch-and-bound over topological orderings, which is exponential. A 2-approximation algorithm exists (Chekuri & Motwani, 1999). [Lawler, 1978; Smith, 1956; Sidney, 1975.]

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, partial order < on T, for each task t in T a length l(t) in Z+ and a weight w(t) in Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule sigma for T that obeys the precedence constraints and for which the sum, over all t in T, of (sigma(t) + l(t))*w(t) is K or less?

Reference: [Lawler, 1978]. Transformation from OPTIMAL LINEAR ARRANGEMENT.

Comment: NP-complete in the strong sense and remains so even if all task lengths are 1 or all task weights are 1. Can be solved in polynomial time for < a "forest" [Horn, 1972], [Adolphson and Hu, 1973], [Garey, 1973], [Sidney, 1975] or if < is "series-parallel" or "generalized series-parallel" [Knuth, 1973], [Lawler, 1978], [Adolphson, 1977], [Monma and Sidney, 1977]. If the partial order < is replaced by individual task deadlines, the resulting problem in NP-complete in the strong sense [Lenstra, 1977], but can be solved in polynomial time if all task weights are equal [Smith, 1956]. If there are individual task release times instead of deadline, the resulting problem is NP-complete in the strong sense, even if all task weights are 1 [Lenstra, Rinnooy Kan, and Brucker, 1977]. The "preemptive" version of this latter problem is NP-complete in the strong sense [Labetoulle, Lawler, Lenstra, and Rinnooy Kan, 1978], but is solvable in polynomial time if all weights are equal [Graham, Lawler, Lenstra, and Rinnooy Kan, 1978].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all topological orderings of the partial order; for each compute total weighted completion time and check <= K.)
- [x] It can be solved by reducing to integer programming. (ILP: integer start-time variables, precedence constraints, weighted completion time objective sum w_t * (sigma_t + l_t) <= K.)
- [ ] Other: WSJF rule (O(n log n)) when no precedence constraints. Sidney decomposition for series-parallel orders.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
Lengths: l(t_1) = 2, l(t_2) = 1, l(t_3) = 3, l(t_4) = 1, l(t_5) = 2
Weights: w(t_1) = 3, w(t_2) = 5, w(t_3) = 1, w(t_4) = 4, w(t_5) = 2
Precedence constraints: t_1 < t_3, t_2 < t_5 (t_1 must finish before t_3 starts; t_2 must finish before t_5 starts)
K = 55

**Schedule (WSJF-like, respecting precedence):**
WSJF ratios: t_1: 3/2=1.5, t_2: 5/1=5, t_3: 1/3=0.33, t_4: 4/1=4, t_5: 2/2=1
Priority order (by decreasing ratio): t_2(5), t_4(4), t_1(1.5), t_5(1), t_3(0.33)
Respecting precedence: t_2 before t_5, t_1 before t_3. Order: t_2, t_4, t_1, t_5, t_3.

| Task | Start | Finish (C_t) | Weight w_t | w_t * C_t |
|------|-------|--------------|------------|-----------|
| t_2  | 0     | 1            | 5          | 5         |
| t_4  | 1     | 2            | 4          | 8         |
| t_1  | 2     | 4            | 3          | 12        |
| t_5  | 4     | 6            | 2          | 12        |
| t_3  | 6     | 9            | 1          | 9         |

Total weighted completion time = 5 + 8 + 12 + 12 + 9 = 46 <= K = 55.
Precedence: t_1 finishes at 4, t_3 starts at 6 (ok); t_2 finishes at 1, t_5 starts at 4 (ok).

Answer: YES -- a schedule with total weighted completion time <= 55 exists.
