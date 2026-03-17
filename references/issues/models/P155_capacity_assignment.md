---
name: Problem
about: Propose a new problem type
title: "[Model] CapacityAssignment"
labels: model
assignees: ''
---

## Motivation

CAPACITY ASSIGNMENT (P155) from Garey & Johnson, A4 SR7. An NP-complete bicriteria optimization problem arising in communication network design. Each communication link must be assigned a capacity from a discrete set, balancing total cost against total delay penalty, subject to the monotonicity constraint that higher capacity costs more but incurs less delay. Proved NP-complete by Van Sickle and Chandy (1977) via reduction from SUBSET SUM.

**Associated rules:**
- R101: SubsetSum → CapacityAssignment (as target)

## Definition

**Name:** `CapacityAssignment`
**Canonical name:** CAPACITY ASSIGNMENT
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR7

**Mathematical definition:**

INSTANCE: Set C of communication links, set M ⊆ Z+ of capacities, cost function g: C x M → Z+, delay penalty function d: C x M → Z+ such that for all c ∈ C and i < j ∈ M, g(c,i) ≤ g(c,j) and d(c,i) ≥ d(c,j), and positive integers K and J.
QUESTION: Is there an assignment σ: C → M such that the total cost ∑_{c ∈ C} g(c,σ(c)) does not exceed K and such that the total delay penalty ∑_{c ∈ C} d(c,σ(c)) does not exceed J?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->
- **Count:** |C| variables, one per communication link. Each variable selects a capacity from M.
- **Per-variable domain:** {1, 2, ..., |M|} — index into the ordered set of capacities M.
- **Meaning:** Variable i encodes the capacity assigned to link c_i. A satisfying assignment σ maps each link to a capacity such that both the total cost budget K and total delay budget J are met simultaneously. The monotonicity constraints (higher capacity = higher cost, lower delay) make this a bicriteria trade-off.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->
**Type name:** `CapacityAssignment`
**Variants:** none (no graph or weight type parameter)

| Field | Type | Description |
|-------|------|-------------|
| `num_links` | `usize` | Number of communication links |C| |
| `capacities` | `Vec<u64>` | Ordered set M of available capacities |
| `cost` | `Vec<Vec<u64>>` | Cost matrix g: cost[i][j] = g(c_i, M[j]) |
| `delay` | `Vec<Vec<u64>>` | Delay matrix d: delay[i][j] = d(c_i, M[j]) |
| `cost_budget` | `u64` | Cost budget K |
| `delay_budget` | `u64` | Delay budget J |

**Notes:**
- This is a satisfaction (decision) problem: `Metric = bool`, implementing `SatisfactionProblem`.
- The monotonicity constraints are: for all i and j1 < j2 in M, cost[i][j1] ≤ cost[i][j2] and delay[i][j1] ≥ delay[i][j2].
- No weight or graph type parameters needed.

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->
- **Best known exact algorithm:** O*(|M|^|C|) brute force over all assignments. Since the problem is solvable in pseudo-polynomial time (as noted in GJ), a dynamic programming approach runs in O(|C| · K · J) time, which is pseudo-polynomial when K and J are bounded by a polynomial in the input size. The DP iterates over links and maintains a 2D table of (cost-so-far, delay-so-far) states.
- **NP-completeness:** NP-complete (Van Sickle and Chandy, 1977). Transformation from SUBSET SUM.
- **Special cases:** With |M| = 2, the problem reduces to a form of SUBSET SUM / bicriteria knapsack. Pseudo-polynomial when the budgets are polynomially bounded.
- **References:**
  - Larry van Sickle and K. Mani Chandy (1977). "The complexity of computer network design problems". Technical report.

## Extra Remark

**Full book text:**

INSTANCE: Set C of communication links, set M ⊆ Z+ of capacities, cost function g: C×M → Z+, delay penalty function d: C×M → Z+ such that, for all c ∈ C and i < j ∈ M, g(c,i) ≤ g(c,j) and d(c,i) ≥ d(c,j), and positive integers K and J.
QUESTION: Is there an assignment σ: C → M such that the total cost ∑c ∈ C g(c,σ(c)) does not exceed K and such that the total delay penalty ∑c ∈ C d(c,σ(c)) does not exceed J?
Reference: [Van Sickle and Chandy, 1977]. Transformation from SUBSET SUM.
Comment: Solvable in pseudo-polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce — enumerate all |M|^|C| assignments and check cost/delay budgets.
- [x] It can be solved by reducing to integer programming — minimize/check cost and delay as linear constraints with integer capacity variables.
- [x] Other: Pseudo-polynomial DP in O(|C| · K · J) time. Also reducible to 0-1 knapsack variants or bicriteria shortest path.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Instance 1 (YES, satisfiable):**
- Links: C = {c_1, c_2, c_3, c_4, c_5, c_6}
- Capacities: M = {1, 2, 3}
- Cost function g:

| Link | g(·,1) | g(·,2) | g(·,3) |
|------|--------|--------|--------|
| c_1  | 2      | 5      | 9      |
| c_2  | 1      | 3      | 7      |
| c_3  | 3      | 6      | 10     |
| c_4  | 2      | 4      | 8      |
| c_5  | 1      | 2      | 6      |
| c_6  | 4      | 7      | 11     |

- Delay penalty function d:

| Link | d(·,1) | d(·,2) | d(·,3) |
|------|--------|--------|--------|
| c_1  | 10     | 5      | 1      |
| c_2  | 8      | 4      | 1      |
| c_3  | 12     | 6      | 2      |
| c_4  | 9      | 5      | 1      |
| c_5  | 7      | 3      | 1      |
| c_6  | 15     | 8      | 3      |

- Monotonicity check: for each link, cost is non-decreasing and delay is non-increasing with capacity ✓
- Cost budget: K = 25
- Delay budget: J = 30

- Assignment σ: σ(c_1)=2, σ(c_2)=2, σ(c_3)=1, σ(c_4)=2, σ(c_5)=2, σ(c_6)=1
  - Total cost = 5 + 3 + 3 + 4 + 2 + 4 = 21 ≤ 25 ✓
  - Total delay = 5 + 4 + 12 + 5 + 3 + 15 = 44 > 30 ✗
- Better assignment: σ(c_1)=2, σ(c_2)=2, σ(c_3)=2, σ(c_4)=2, σ(c_5)=2, σ(c_6)=1
  - Total cost = 5 + 3 + 6 + 4 + 2 + 4 = 24 ≤ 25 ✓
  - Total delay = 5 + 4 + 6 + 5 + 3 + 15 = 38 > 30 ✗
- Assignment: σ(c_1)=2, σ(c_2)=3, σ(c_3)=2, σ(c_4)=2, σ(c_5)=3, σ(c_6)=2
  - Total cost = 5 + 7 + 6 + 4 + 6 + 7 = 35 > 25 ✗
- Assignment: σ(c_1)=2, σ(c_2)=2, σ(c_3)=2, σ(c_4)=2, σ(c_5)=3, σ(c_6)=2
  - Total cost = 5 + 3 + 6 + 4 + 6 + 7 = 31 > 25 ✗
- Assignment: σ(c_1)=1, σ(c_2)=2, σ(c_3)=2, σ(c_4)=2, σ(c_5)=2, σ(c_6)=2
  - Total cost = 2 + 3 + 6 + 4 + 2 + 7 = 24 ≤ 25 ✓
  - Total delay = 10 + 4 + 6 + 5 + 3 + 8 = 36 > 30 ✗
- Assignment: σ(c_1)=2, σ(c_2)=2, σ(c_3)=2, σ(c_4)=3, σ(c_5)=2, σ(c_6)=2
  - Total cost = 5 + 3 + 6 + 8 + 2 + 7 = 31 > 25 ✗

**Instance 2 (YES, from Subset Sum reduction):**
Constructed from SubsetSum A = {3, 7, 1, 8, 4, 12}, B = 15:
- Links: C = {c_1, ..., c_6}
- Capacities: M = {1, 2}
- Cost: g(c_i, 1) = 0, g(c_i, 2) = a_i for each i
- Delay: d(c_i, 1) = a_i, d(c_i, 2) = 0 for each i
- K = 15, J = 20
- Assignment σ(c_1)=2, σ(c_2)=1, σ(c_3)=1, σ(c_4)=2, σ(c_5)=2, σ(c_6)=1
  - Cost = 3+0+0+8+4+0 = 15 ≤ 15 ✓
  - Delay = 0+7+1+0+0+12 = 20 ≤ 20 ✓
- Answer: YES
