---
name: Problem
about: Propose a new problem type
title: "[Model] StaffScheduling"
labels: model
assignees: ''
---

## Motivation

STAFF SCHEDULING (P204) from Garey & Johnson, A5 SS20. A classical NP-complete problem in workforce scheduling: given a collection of binary schedule patterns (m-tuples with k ones), a requirement vector specifying minimum staffing per period, and a workforce budget n, can workers be assigned to schedules to meet all requirements? Shown NP-complete by Garey and Johnson (unpublished) via reduction from X3C. Solvable in polynomial time when schedules have the cyclic ones property (consecutive shifts), a result of Bartholdi, Orlin, and Ratliff (1977).

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R149: X3C -> Staff Scheduling (incoming, [Garey and Johnson, unpublished])

## Definition

**Name:** `StaffScheduling`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS20

**Mathematical definition:**

INSTANCE: Positive integers m and k, a collection C of m-tuples, each having k 1's and m - k 0's (representing possible worker schedules), a "requirement" m-tuple R̄ of non-negative integers, and a number n of workers.
QUESTION: Is there a schedule f: C→Z0+ such that ∑_{c̄ ∈ C} f(c̄) ≤ n and such that ∑_{c̄ ∈ C} f(c̄)·c̄ ≥ R̄?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** |C| (one integer variable per schedule pattern)
- **Per-variable domain:** {0, 1, ..., n} — how many workers are assigned to each schedule pattern
- **Meaning:** f(c̄) is the number of workers following schedule pattern c̄. The total number of workers across all patterns must not exceed n, and the sum of scheduled workers in each period must meet or exceed the requirement for that period.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `StaffScheduling`
**Variants:** none (no type parameters; all values are non-negative integers)

| Field           | Type              | Description                                                   |
|-----------------|-------------------|---------------------------------------------------------------|
| `num_periods`   | `usize`           | Number of time periods m                                      |
| `shifts_per_schedule` | `usize`     | Number of active shifts k per schedule pattern                |
| `schedules`     | `Vec<Vec<bool>>`  | Collection C of m-tuples (binary schedule patterns)           |
| `requirements`  | `Vec<u64>`        | Requirement vector R̄ (minimum staffing per period)           |
| `num_workers`   | `u64`             | Maximum number of workers n available                         |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete in general (Garey and Johnson, 1979). It can be formulated as an integer linear program and solved with ILP solvers, but no polynomial-time exact algorithm exists unless P = NP. For the special case where all schedule patterns have the cyclic ones property (all 1's are consecutive, with wraparound), the problem is solvable in polynomial time via network flow techniques (Bartholdi, Orlin, and Ratliff, 1980). In the general case, brute-force enumeration of all possible assignments f: C -> {0,...,n} requires O((n+1)^|C|) time. The nurse scheduling problem, a practical generalization, is typically solved with ILP or metaheuristics (genetic algorithms, tabu search, simulated annealing).

## Extra Remark

**Full book text:**

INSTANCE: Positive integers m and k, a collection C of m-tuples, each having k 1's and m - k 0's (representing possible worker schedules), a "requirement" m-tuple R̄ of non-negative integers, and a number n of workers.
QUESTION: Is there a schedule f: C→Z0+ such that ∑_{c̄ ∈ C} f(c̄) ≤ n and such that ∑_{c̄ ∈ C} f(c̄)·c̄ ≥ R̄?

Reference: [Garey and Johnson, ——] Transformation from X3C.

Comment: Solvable in polynomial time if every c̄ ∈ C has the cyclic one's property, i.e., has all its 1's occuring in consecutive positions with position 1 regarded as following position m [Bartholdi, Orlin, and Ratliff, 1977]. (This corresponds to workers who are available only for consecutive hours of the day, or days of the week.)

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all assignments f: C -> {0,...,n} with sum f(c̄) <= n; check if sum f(c̄)*c̄ >= R̄ component-wise.)
- [x] It can be solved by reducing to integer programming. (ILP: minimize sum f(c̄) subject to f(c̄) >= 0 integer, sum f(c̄) <= n, and for each period j: sum_{c̄} f(c̄)*c̄_j >= R̄_j.)
- [ ] Other: For cyclic ones property schedules, solvable via network flow / circular ones ILP.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
m = 7 periods (days of the week), k = 5 shifts per schedule (5-day work week)
n = 4 workers

Schedules (C):
| Schedule | Mon | Tue | Wed | Thu | Fri | Sat | Sun |
|----------|-----|-----|-----|-----|-----|-----|-----|
| c_1      | 1   | 1   | 1   | 1   | 1   | 0   | 0   |
| c_2      | 0   | 1   | 1   | 1   | 1   | 1   | 0   |
| c_3      | 0   | 0   | 1   | 1   | 1   | 1   | 1   |
| c_4      | 1   | 0   | 0   | 1   | 1   | 1   | 1   |
| c_5      | 1   | 1   | 0   | 0   | 1   | 1   | 1   |

Requirements R̄ = (2, 2, 2, 3, 3, 2, 1)

**Feasible schedule:**
f(c_1) = 1, f(c_2) = 1, f(c_3) = 1, f(c_4) = 1, f(c_5) = 0.
Total workers: 1 + 1 + 1 + 1 + 0 = 4 <= n = 4 ✓

Coverage per period:
- Mon: c_1 + c_4 = 2 >= 2 ✓
- Tue: c_1 + c_2 = 2 >= 2 ✓
- Wed: c_1 + c_2 + c_3 = 3 >= 2 ✓
- Thu: c_1 + c_2 + c_3 + c_4 = 4 >= 3 ✓
- Fri: c_1 + c_2 + c_3 + c_4 = 4 >= 3 ✓
- Sat: c_2 + c_3 + c_4 = 3 >= 2 ✓
- Sun: c_3 + c_4 = 2 >= 1 ✓

Answer: YES — a feasible staff schedule exists with 4 workers.
