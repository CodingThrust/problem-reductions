---
name: Problem
about: Propose a new problem type
title: "[Model] PartiallyOrderedKnapsack"
labels: model
assignees: ''
---

## Motivation

PARTIALLY ORDERED KNAPSACK (P218) from Garey & Johnson, A6 MP12. An NP-complete (strong sense) knapsack variant where items are subject to a partial order: including an item in the knapsack requires including all its predecessors. This models precedence-constrained selection problems arising in manufacturing scheduling, project selection, mining operations, and network design. The precedence constraints make the problem significantly harder than standard 0-1 Knapsack — it is NP-complete in the strong sense even when s(u) = v(u) for all u, whereas standard 0-1 Knapsack is only weakly NP-complete. Also known as the Precedence Constrained Knapsack Problem (PCKP) in the optimization literature.

<!-- ⚠️ Unverified: AI-generated motivation additions -->
**Associated rules:**
- R162: CLIQUE -> PARTIALLY ORDERED KNAPSACK (establishes NP-completeness via Garey and Johnson)

## Definition

**Name:** `PartiallyOrderedKnapsack`
<!-- ⚠️ Unverified -->
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP12

**Mathematical definition:**

INSTANCE: Finite set U, partial order < on U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, positive integers B and K.
QUESTION: Is there a subset U' ⊆ U such that if u ∈ U' and u' < u, then u' ∈ U', and such that Σᵤ∈U' s(u) ≤ B and Σᵤ∈U' v(u) ≥ K?

The constraint "if u ∈ U' and u' < u, then u' ∈ U'" means U' must be a downward-closed set (lower set / order ideal) in the partial order.

## Variables

<!-- ⚠️ Unverified: AI-inferred variable mapping -->

- **Count:** n = |U| binary variables (one per item)
- **Per-variable domain:** binary {0, 1} — whether item u is included in U'
- **Meaning:** x_u = 1 if item u is selected. The configuration (x₁, ..., xₙ) encodes a candidate subset U' ⊆ U. The assignment is valid if: (a) U' is a downward-closed set (for every u ∈ U', all predecessors of u are also in U'), (b) Σ s(u)·x_u ≤ B, and (c) Σ v(u)·x_u ≥ K.

## Schema (data type)

<!-- ⚠️ Unverified: AI-designed schema -->

**Type name:** `PartiallyOrderedKnapsack`
**Variants:** none

| Field        | Type                  | Description                                              |
|--------------|-----------------------|----------------------------------------------------------|
| `sizes`      | `Vec<i64>`            | Size s(u) for each item u ∈ U                           |
| `values`     | `Vec<i64>`            | Value v(u) for each item u ∈ U                          |
| `precedences`| `Vec<(usize, usize)>` | Precedence relations: (u', u) means u' < u (u' must be included before u) |
| `capacity`   | `i64`                 | Knapsack capacity B                                      |

**Notes:**
- This is a satisfaction (decision) problem in the GJ formulation: `Metric = bool`. Can also be modeled as optimization (maximize Σ v(u)·x_u subject to precedence + capacity constraints).
- The `precedences` field encodes the Hasse diagram (cover relations) of the partial order. The full partial order is the transitive closure.
- An alternative representation stores the partial order as a DAG adjacency list.
- Key getter methods needed: `num_items()` (= |U|), `num_precedences()` (= number of cover relations), `capacity()` (= B).

## Complexity

<!-- ⚠️ Unverified: AI-researched complexity -->

- **Decision complexity:** NP-complete in the strong sense (Garey and Johnson; transformation from CLIQUE). Remains NP-complete in the strong sense even when s(u) = v(u) for all u.
- **Best known exact algorithm:** Branch-and-bound with Lagrangian relaxation bounds. Dynamic programming approaches work when the partial order has special structure. For general partial orders, the problem is strongly NP-hard, so no pseudo-polynomial algorithm exists unless P = NP.
- **Special cases:**
  - Tree partial orders (Hasse diagram is a tree): solvable in pseudo-polynomial time O(n · B) by tree DP (Garey and Johnson; Johnson and Niemi, 1983).
  - 2-dimensional partial orders: FPTAS exists with running time O(n⁴/ε) for any ε > 0 (Kolliopoulos and Steiner, 2007).
  - Bipartite orders (all elements are either minimal or maximal): NP-complete in the strong sense.
- **Approximation:** For general partial orders, no FPTAS exists (strongly NP-hard). Johnson and Niemi (1983) gave an FPTAS for tree orders. Kolliopoulos and Steiner extended this to 2-dimensional orders.
- **References:**
  - M. R. Garey and D. S. Johnson. "Computers and Intractability." Original NP-completeness result.
  - D. S. Johnson and K. A. Niemi (1983). "On Knapsacks, Partitions, and a New Dynamic Programming Technique for Trees." *Mathematics of Operations Research* 8(1), pp. 1–14. Tree DP and FPTAS for tree orders.
  - O. H. Ibarra and C. E. Kim (1975). "Scheduling for maximum profit." CS Dept., University of Minnesota. Early discussion of the problem.
  - S. G. Kolliopoulos and G. Steiner (2007). "Partially-Ordered Knapsack and Applications to Scheduling." *Discrete Applied Mathematics* 155(8), pp. 889–897.

## Extra Remark

**Full book text:**

INSTANCE: Finite set U, partial order < on U, for each u ∈ U a size s(u) ∈ Z⁺ and a value v(u) ∈ Z⁺, positive integers B and K.
QUESTION: Is there a subset U' ⊆ U such that if u ∈ U' and u' < u, then u' ∈ U', and such that Σᵤ∈U' s(u) ≤ B and Σᵤ∈U' v(u) ≥ K?

Reference: [Garey and Johnson, ——]. Transformation from CLIQUE. Problem is discussed in [Ibarra and Kim, 1975b].
Comment: NP-complete in the strong sense, even if s(u) = v(u) for all u ∈ U. General problem is solvable in pseudo-polynomial time if < is a "tree" partial order [Garey and Johnson, ——].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all 2ⁿ subsets; filter to downward-closed sets; check capacity and value constraints.)
- [x] It can be solved by reducing to integer programming. (ILP with binary variables x_u for each item; constraints: x_u ≤ x_{u'} for each precedence u' < u; Σ s(u)·x_u ≤ B; objective maximize Σ v(u)·x_u.)
- [ ] Other: Branch-and-bound with Lagrangian relaxation; tree DP for tree orders in O(n·B).

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
U = {a, b, c, d, e, f} (n = 6 items)
Partial order (Hasse diagram):
```
    a       b
   / \      |
  c   d     e
       \   /
        f
```
Precedences: a < c, a < d, b < e, d < f, e < f
(Meaning: to include c, must include a; to include f, must include both d and e, hence also a and b.)

Sizes:  s(a) = 2, s(b) = 3, s(c) = 4, s(d) = 1, s(e) = 2, s(f) = 3
Values: v(a) = 3, v(b) = 2, v(c) = 5, v(d) = 4, v(e) = 3, v(f) = 8
Capacity B = 11, Target K = 18

**Solution:** U' = {a, b, d, e, f}
- Downward-closed check:
  - f ∈ U': predecessors d, e, a, b all in U' ✓
  - d ∈ U': predecessor a ∈ U' ✓
  - e ∈ U': predecessor b ∈ U' ✓
  - a, b: no predecessors ✓
- Total size: 2 + 3 + 1 + 2 + 3 = 11 ≤ 11 ✓
- Total value: 3 + 2 + 4 + 3 + 8 = 20 ≥ 18 ✓

Answer: YES.

**Why not include c?** U' = {a, b, c, d, e, f} has total size = 2+3+4+1+2+3 = 15 > 11 = B. Exceeds capacity.

**Invalid subset:** U' = {d, f} — includes f but not e and not b (predecessors of f via e). Not downward-closed. ✗

**Negative instance:** Same items but B = 5, K = 18.
Best downward-closed sets within capacity 5:
- {a, d}: size 3, value 7
- {b, e}: size 5, value 5
- {a}: size 2, value 3
None achieves value ≥ 18. Answer: NO.
