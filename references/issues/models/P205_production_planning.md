---
name: Problem
about: Propose a new problem type
title: "[Model] ProductionPlanning"
labels: model
assignees: ''
---

## Motivation

PRODUCTION PLANNING (P205) from Garey & Johnson, A5 SS21. A classical NP-complete lot-sizing problem: given a multi-period planning horizon with per-period demands, production capacities, set-up costs, production costs, and inventory holding costs, can all demands be met within a total cost budget? Shown NP-complete by Lenstra, Rinnooy Kan, and Florian (1978) via reduction from PARTITION. The problem is solvable in pseudo-polynomial time but remains NP-complete even with equal demands, equal set-up costs, and zero inventory costs. A foundational hardness result for operations research, supply chain management, and manufacturing planning.

<!-- ⚠️ Unverified: AI-generated motivation additions below -->
**Associated rules:**
- R150: Partition -> Production Planning (incoming, [Lenstra, Rinnooy Kan, and Florian, 1978])

## Definition

**Name:** `ProductionPlanning`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A5 SS21

**Mathematical definition:**

INSTANCE: Number n ∈ Z+ of periods, for each period i, 1 ≤ i ≤ n, a demand ri ∈ Z0+, a production capacity ci ∈ Z0+, a production set-up cost bi ∈ Z0+, an incremental production cost coefficient pi ∈ Z0+, and an inventory cost coefficient hi ∈ Z0+, and an overall bound B ∈ Z+.
QUESTION: Do there exist production amounts xi ∈ Z0+ and associated inventory levels Ii = ∑_{j=1}^{i}(xj−rj), 1 ≤ i ≤ n, such that all xi ≤ ci, all Ii ≥ 0, and
∑_{i=1}^{n}(pi·xi + hi·Ii) + ∑_{xi>0} bi ≤ B ?

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n (one integer variable per period, representing production amount)
- **Per-variable domain:** {0, 1, ..., c_i} — production amount in period i, bounded by capacity
- **Meaning:** x_i is the production amount in period i. Inventory levels I_i are derived: I_i = sum_{j=1}^{i}(x_j - r_j). The constraints require all I_i >= 0 (no backlogging), all x_i <= c_i (capacity), and total cost (production + inventory + set-up) <= B.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `ProductionPlanning`
**Variants:** none (no type parameters; all values are non-negative integers)

| Field              | Type         | Description                                                    |
|--------------------|--------------|----------------------------------------------------------------|
| `num_periods`      | `usize`      | Number of planning periods n                                   |
| `demands`          | `Vec<u64>`   | Demand r_i for each period i                                   |
| `capacities`       | `Vec<u64>`   | Production capacity c_i for each period i                      |
| `setup_costs`      | `Vec<u64>`   | Set-up cost b_i for each period i (incurred if x_i > 0)       |
| `production_costs` | `Vec<u64>`   | Incremental production cost coefficient p_i per unit           |
| `inventory_costs`  | `Vec<u64>`   | Inventory holding cost coefficient h_i per unit per period     |
| `cost_bound`       | `u64`        | Overall cost bound B                                           |

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Best known exact algorithm:** The problem is NP-complete in general (Lenstra, Rinnooy Kan, and Florian, 1978), but solvable in pseudo-polynomial time via dynamic programming. When all capacities are equal, the problem can be solved in polynomial time (Florian and Klein, 1971). For the general capacitated lot-sizing problem with piecewise concave costs and a fixed number of breakpoints p, Koca, Yaman, and Akturk (2014) gave an O(n^(2p+3)) algorithm, later improved to O(n^(p+2) log n) by Ou (2017). In the worst case with arbitrary concave costs, no algorithm significantly improves upon pseudo-polynomial dynamic programming in O(n * B) or brute-force O*(product c_i) enumeration.

## Extra Remark

**Full book text:**

INSTANCE: Number n ∈ Z+ of periods, for each period i, 1 ≤ i ≤ n, a demand ri ∈ Z0+, a production capacity ci ∈ Z0+, a production set-up cost bi ∈ Z0+, an incremental production cost coefficient pi ∈ Z0+, and an inventory cost coefficient hi ∈ Z0+, and an overall bound B ∈ Z+.
QUESTION: Do there exist production amounts xi ∈ Z0+ and associated inventory levels Ii = ∑'_{j=1}(xj−rj), 1 ≤ i ≤ n, such that all xi ≤ ci, all Ii ≥ 0, and

∑_{i=1}^{n}(pi·xi + hi·Ii) + ∑_{xi>0} bi ≤ B ?

Reference: [Lenstra, Rinnooy Kan, and Florian, 1978]. Transformation from PARTITION.

Comment: Solvable in pseudo-polynomial time, but remains NP-complete even if all demands are equal, all set-up costs are equal, and all inventory costs are 0. If all capacities are equal, the problem can be solved in polynomial time [Florian and Klein, 1971]. The cited algorithms can be generalized to allow for arbitrary monotone non-decreasing concave cost functions, if these can be computed in polynomial time.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all production vectors (x_1,...,x_n) with 0 <= x_i <= c_i; compute inventory levels and total cost; check feasibility.)
- [x] It can be solved by reducing to integer programming. (ILP: integer variables x_i, binary variables y_i (y_i=1 iff x_i>0), constraints I_i >= 0, x_i <= c_i, x_i <= c_i * y_i, and sum(p_i*x_i + h_i*I_i + b_i*y_i) <= B.)
- [ ] Other: Dynamic programming (pseudo-polynomial time) for fixed or bounded capacities.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
n = 6 periods

| Period i | Demand r_i | Capacity c_i | Setup cost b_i | Prod. cost p_i | Inv. cost h_i |
|----------|-----------|-------------|---------------|---------------|--------------|
| 1        | 5         | 12          | 10            | 1             | 1            |
| 2        | 3         | 12          | 10            | 1             | 1            |
| 3        | 7         | 12          | 10            | 1             | 1            |
| 4        | 2         | 12          | 10            | 1             | 1            |
| 5        | 8         | 12          | 10            | 1             | 1            |
| 6        | 5         | 12          | 10            | 1             | 1            |

Total demand = 5+3+7+2+8+5 = 30
Cost bound B = 65

**Feasible production plan:**
Produce in 3 batches: x_1 = 8 (covers periods 1-2), x_3 = 9 (covers periods 3-4), x_5 = 12 (covers periods 5-6), all other x_i = 0.

Inventory levels:
- I_1 = 8 - 5 = 3
- I_2 = 3 - 3 = 0 (x_2 = 0)
- I_3 = 0 + 9 - 7 = 2
- I_4 = 2 - 2 = 0 (x_4 = 0)
- I_5 = 0 + 12 - 8 = 4 (x_5 = 12, but cap is 12)
- I_6 = 4 - 5 = -1 ... wait, need x_5 = 13 but cap is 12.

Let us redo: x_1 = 8, x_3 = 9, x_5 = 12, x_6 = 1.
- I_1 = 3, I_2 = 0, I_3 = 2, I_4 = 0, I_5 = 4, I_6 = 4 + 1 - 5 = 0. All >= 0 ✓

All x_i <= 12 ✓
Production cost: 1*(8+9+12+1) = 30
Inventory cost: 1*(3+0+2+0+4+0) = 9
Setup cost: 4 setups * 10 = 40
Total: 30 + 9 + 40 = 79 > B = 65. Too high.

Revised plan: x_1 = 12 (covers periods 1-3), x_4 = 12 (covers periods 4-6).
- I_1 = 12 - 5 = 7, I_2 = 7 - 3 = 4, I_3 = 4 - 7 = -3. Infeasible.

Revised plan with B = 80: x_1 = 8, x_3 = 9, x_5 = 12, x_6 = 1.
Total cost = 30 + 9 + 40 = 79 <= 80 ✓

Answer: YES — a feasible production plan exists with cost bound B = 80.
