---
name: Problem
about: Propose a new problem type
title: "[Model] EquilibriumPoint"
labels: model
assignees: ''
---

## Motivation

EQUILIBRIUM POINT (P234) from Garey & Johnson, A7 AN15. Given a set of variables, each with a finite range set of integers, and a collection of product polynomials (one per variable), the problem asks whether there exists an assignment where each variable simultaneously maximizes its own polynomial (a discrete Nash equilibrium). NP-complete by reduction from 3SAT (Sahni, 1974). The problem remains NP-complete even when all range sets are binary ({0, 1}), making it a fundamental hardness result for computing equilibria in discrete games. Connects computational complexity theory to game theory and nonlinear programming.

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R178 (3SAT to EQUILIBRIUM POINT)
- As source: (none known)

## Definition

**Name:** `EquilibriumPoint`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Equilibrium Point
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN15

**Mathematical definition:**

INSTANCE: Set x = {x_1, x_2, . . . , x_n} of variables, collection {F_i: 1 ≤ i ≤ n} of product polynomials over X and the integers, and a finite "range-set" M_i ⊆ Z for 1 ≤ i ≤ n.
QUESTION: Does there exist a sequence y_1, y_2, . . . , y_n of integers, with y_i ∈ M_i, such that for 1 ≤ i ≤ n and all y ∈ M_i,
F_i(y_1, y_2, . . . , y_{i-1}, y_i, y_{i+1}, . . . , y_n) ≥ F_i(y_1, y_2, . . . , y_{i-1}, y, y_{i+1}, . . . , y_n)?

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n (one variable per player/polynomial)
- **Per-variable domain:** M_i (a finite subset of integers; in the hardest case, M_i = {0, 1})
- **Meaning:** y_i is the strategy chosen by player i. The assignment (y_1, ..., y_n) is an equilibrium point if no player can unilaterally improve their payoff F_i by changing y_i to another value in M_i.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `EquilibriumPoint`
**Variants:** none

| Field          | Type                    | Description                                                |
|----------------|-------------------------|------------------------------------------------------------|
| `polynomials`  | `Vec<ProductPolynomial>` | Collection of n product polynomials F_1, ..., F_n          |
| `range_sets`   | `Vec<Vec<i64>>`         | Finite range set M_i ⊆ Z for each variable x_i            |

Where `ProductPolynomial` represents a polynomial expressed as a product of linear or low-degree factors over the variables x_1, ..., x_n with integer coefficients.

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** NP-complete even for binary range sets M_i = {0, 1}. With binary variables, brute-force enumeration of all 2^n assignments and checking the equilibrium condition for each takes O(2^n · n · max|M_i|) time. No significantly better worst-case algorithm is known. The problem generalizes to continuous domains where computing Nash equilibria is PPAD-complete (Daskalakis, Goldberg, Papadimitriou, 2009), but the discrete version with finite range sets is in NP and NP-complete. [Sahni, SIAM J. Comput. 3:262-279, 1974.]

## Extra Remark

**Full book text:**

INSTANCE: Set x = {x_1, x_2, . . . , x_n} of variables, collection {F_i: 1 ≤ i ≤ n} of product polynomials over X and the integers, and a finite "range-set" M_i ⊆ Z for 1 ≤ i ≤ n.
QUESTION: Does there exist a sequence y_1, y_2, . . . , y_n of integers, with y_i ∈ M_i, such that for 1 ≤ i ≤ n and all y ∈ M_i,

    F_i(y_1, y_2, . . . , y_{i-1}, y_i, y_{i+1}, . . . , y_n) ≥ F_i(y_1, y_2, . . . , y_{i-1}, y, y_{i+1}, . . . , y_n)?

Reference: [Sahni, 1974]. Transformation from 3SAT.
Comment: Remains NP-complete even if M_i = {0,1} for 1 ≤ i ≤ n.

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all Π|M_i| assignments; for each, check whether every player's choice is a best response. Return YES if any assignment is an equilibrium.)
- [ ] It can be solved by reducing to integer programming. (Encode each variable's domain and the payoff-maximization constraints as integer constraints; however, the product polynomial structure makes this non-trivial.)
- [ ] Other: For the binary case (M_i = {0,1}), reduce to SAT by encoding the equilibrium conditions as Boolean constraints.

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
n = 2 variables: x_1, x_2
Range sets: M_1 = {0, 1}, M_2 = {0, 1}
Product polynomials:
- F_1(x_1, x_2) = x_1 · x_2 (player 1's payoff)
- F_2(x_1, x_2) = (1 - x_1) · x_2 + x_1 · (1 - x_2) = x_1 + x_2 - 2·x_1·x_2 (player 2's payoff; equivalently x_1 XOR x_2)

**Analysis of all assignments:**

| (x_1, x_2) | F_1 | F_2 | x_1 best response? | x_2 best response? | Equilibrium? |
|-------------|-----|-----|---------------------|---------------------|--------------|
| (0, 0)      | 0   | 0   | 0 vs 1: F_1(0,0)=0, F_1(1,0)=0. Tie ✓ | 0 vs 1: F_2(0,0)=0, F_2(0,1)=1. No ✗ | No |
| (0, 1)      | 0   | 1   | 0 vs 1: F_1(0,1)=0, F_1(1,1)=1. No ✗ | - | No |
| (1, 0)      | 0   | 1   | 1 vs 0: F_1(1,0)=0, F_1(0,0)=0. Tie ✓ | 0 vs 1: F_2(1,0)=1, F_2(1,1)=0. Yes ✓ | Yes ✓ |
| (1, 1)      | 1   | 0   | 1 vs 0: F_1(1,1)=1, F_1(0,1)=0. Yes ✓ | 1 vs 0: F_2(1,1)=0, F_2(1,0)=1. No ✗ | No |

Answer: **YES** -- (x_1, x_2) = (1, 0) is an equilibrium point.
