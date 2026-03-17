---
name: Problem
about: Propose a new problem type
title: "[Model] SequencingToMinimizeTardyTaskWeight"
labels: model
assignees: ''
---

## Motivation

SEQUENCING TO MINIMIZE TARDY TASK WEIGHT (P187) from Garey & Johnson, A5 SS3. A single-machine scheduling problem: given n tasks with processing times, weights, and deadlines, can we find a schedule where the total weight of tardy tasks (those finishing after their deadline) is at most K? This is the weighted generalization of Moore's tardy-tasks problem and is NP-complete by reduction from PARTITION (Karp, 1972). When all tasks share a common deadline, the problem reduces to the KNAPSACK problem. It admits a pseudo-polynomial-time algorithm (Lawler & Moore, 1969) and is thus only weakly NP-hard. No precedence constraints appear in this formulation.

**Associated rules:**
- R133: Partition -> Sequencing to Minimize Tardy Task Weight (as target)

<!-- ⚠️ Unverified: AI-generated motivation and associated rules list -->

## Definition

**Name:** `SequencingToMinimizeTardyTaskWeight`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS3

**Mathematical definition:**

INSTANCE: Set T of tasks, for each task t in T a length l(t) in Z+, a weight w(t) in Z+, and a deadline d(t) in Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule sigma for T such that the sum of w(t), taken over all t in T for which sigma(t) + l(t) > d(t), does not exceed K?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |T| (one binary variable per task indicating whether it is on-time or tardy)
- **Per-variable domain:** {0, 1} -- 0 means the task is scheduled on-time (before its deadline), 1 means tardy
- **Meaning:** u_t = 1 if task t is tardy (sigma(t) + l(t) > d(t)), u_t = 0 otherwise. The problem asks whether there exists a permutation of tasks such that sum_{t: u_t=1} w(t) <= K. Note: the actual decision of which tasks are on-time must be consistent with a valid schedule (the total processing time of on-time tasks whose deadlines are <= d must not exceed d for any threshold d).

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `SequencingToMinimizeTardyTaskWeight`
**Variants:** none (no type parameters)

| Field      | Type       | Description                                              |
|------------|------------|----------------------------------------------------------|
| `lengths`  | `Vec<u64>` | Processing time l(t) for each task t in T                |
| `weights`  | `Vec<u64>` | Weight w(t) for each task t in T                         |
| `deadlines`| `Vec<u64>` | Deadline d(t) for each task t in T                       |
| `bound_k`  | `u64`      | Maximum allowed total weight of tardy tasks K            |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem (1||sum w_j U_j) is NP-complete (Karp, 1972) but only weakly NP-hard. It can be solved in pseudo-polynomial time O(n * sum l(t) * log(sum w(t))) by the Lawler-Moore dynamic programming algorithm (1969). For the unweighted case (all w(t) = 1), Moore's algorithm gives an O(n log n) greedy solution. For the weighted case with "agreeable" weights (w(t) < w(t') implies l(t) >= l(t')), Lawler (1976) gives a polynomial algorithm. The problem is W[1]-hard parameterized by the number of tardy tasks (Hermelin et al., 2019), ruling out FPT algorithms under standard assumptions. [Karp, 1972; Lawler & Moore, 1969; Lawler, 1976.]

## Extra Remark

**Full book text:**

INSTANCE: Set T of tasks, for each task t in T a length l(t) in Z+, a weight w(t) in Z+, and a deadline d(t) in Z+, and a positive integer K.
QUESTION: Is there a one-processor schedule sigma for T such that the sum of w(t), taken over all t in T for which sigma(t) + l(t) > d(t), does not exceed K?

Reference: [Karp, 1972]. Transformation from PARTITION.

Comment: Can be solved in pseudo-polynomial time (time polynomial in |T|, sum l(t), and log sum w(t)) [Lawler and Moore, 1969]. Can be solved in polynomial time if weights are "agreeable" (i.e., w(t) < w(t') implies l(t) >= l(t')) [Lawler, 1976c].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all n! orderings; for each compute start times and total tardy weight.)
- [x] It can be solved by reducing to integer programming. (Binary ILP: x_{ij} ordering variables, tardiness indicators U_t, minimize sum w_t * U_t <= K.)
- [ ] Other: Lawler-Moore pseudo-polynomial DP in O(n * sum l(t)) time.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
T = {t_1, t_2, t_3, t_4, t_5} (n = 5 tasks)
Lengths: l(t_1) = 3, l(t_2) = 2, l(t_3) = 4, l(t_4) = 1, l(t_5) = 2
Weights: w(t_1) = 5, w(t_2) = 3, w(t_3) = 7, w(t_4) = 2, w(t_5) = 4
Deadlines: d(t_1) = 6, d(t_2) = 4, d(t_3) = 10, d(t_4) = 2, d(t_5) = 8
K = 5

Total processing time = 3 + 2 + 4 + 1 + 2 = 12.

**Feasible schedule:**
sigma: t_4, t_2, t_1, t_5, t_3

| Task | Start | Finish | Deadline | Tardy? | Weight if tardy |
|------|-------|--------|----------|--------|-----------------|
| t_4  | 0     | 1      | 2        | No     | -               |
| t_2  | 1     | 3      | 4        | No     | -               |
| t_1  | 3     | 6      | 6        | No     | -               |
| t_5  | 6     | 8      | 8        | No     | -               |
| t_3  | 8     | 12     | 10       | Yes    | 7               |

Total tardy weight = 7 > K = 5. Not feasible with this ordering.

Better schedule:
sigma: t_4, t_2, t_5, t_1, t_3

| Task | Start | Finish | Deadline | Tardy? | Weight if tardy |
|------|-------|--------|----------|--------|-----------------|
| t_4  | 0     | 1      | 2        | No     | -               |
| t_2  | 1     | 3      | 4        | No     | -               |
| t_5  | 3     | 5      | 8        | No     | -               |
| t_1  | 5     | 8      | 6        | Yes    | 5               |
| t_3  | 8     | 12     | 10       | Yes    | 7               |

Total tardy weight = 5 + 7 = 12 > K. Still too much.

Best schedule (make high-weight tasks on-time):
sigma: t_4, t_2, t_1, t_3, t_5

| Task | Start | Finish | Deadline | Tardy? | Weight if tardy |
|------|-------|--------|----------|--------|-----------------|
| t_4  | 0     | 1      | 2        | No     | -               |
| t_2  | 1     | 3      | 4        | No     | -               |
| t_1  | 3     | 6      | 6        | No     | -               |
| t_3  | 6     | 10     | 10       | No     | -               |
| t_5  | 10    | 12     | 8        | Yes    | 4               |

Total tardy weight = 4 <= K = 5.

Answer: YES -- a schedule with total tardy weight <= 5 exists.
