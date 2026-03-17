---
name: Problem
about: Propose a new problem type
title: "[Model] ComparativeContainment"
labels: model
assignees: ''
---

## Motivation

COMPARATIVE CONTAINMENT (P137) from Garey & Johnson, A3 SP10. A classical NP-complete problem in weighted set selection: given two weighted collections of subsets over a common universe, decide whether a subset of the universe can be chosen so that the total weight of sets in the first collection containing the chosen subset meets or exceeds the total weight of sets in the second collection containing it. Introduced by Plaisted (1976), who proved its NP-completeness via reduction from VERTEX COVER. The problem captures a fundamental comparison principle over containment relations and serves as a gateway to further reductions (e.g., to COMPARATIVE VECTOR INEQUALITIES).

<!-- ⚠️ Unverified: AI-generated motivation -->

**Associated reduction rules:**
- As target: R76 (VERTEX COVER to COMPARATIVE CONTAINMENT)
- As source: R163 (COMPARATIVE CONTAINMENT (with equal weights) to COMPARATIVE VECTOR INEQUALITIES)

## Definition

**Name:** `ComparativeContainment`
<!-- ⚠️ Unverified: AI-generated Rust name -->
**Canonical name:** Comparative Containment
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP10

**Mathematical definition:**

INSTANCE: Two collections R = {R_1,R_2,...,R_k} and S = {S_1,S_2,...,S_l} of subsets of a finite set X, weights w(R_i) in Z^+, 1 <= i <= k, and w(S_j) in Z^+, 1 <= j <= l.
QUESTION: Is there a subset Y <= X such that
Sum_{Y <= R_i} w(R_i) >= Sum_{Y <= S_j} w(S_j) ?

(Here Y <= R_i means Y is contained in R_i.)

## Variables

<!-- ⚠️ Unverified: AI-generated variable description -->

- **Count:** n = |X| (one binary variable per element of the universe X)
- **Per-variable domain:** {0, 1} -- 0 means element is not in Y, 1 means element is in Y
- **Meaning:** x_i = 1 if element x_i is included in the chosen subset Y. The problem asks whether there exists an assignment such that Sum_{j: Y <= R_j} w(R_j) >= Sum_{j: Y <= S_j} w(S_j), where the containment Y <= R_j is checked by verifying that every element in Y is also in R_j.

## Schema (data type)

<!-- ⚠️ Unverified: AI-generated schema -->

**Type name:** `ComparativeContainment`
**Variants:** none (weights are positive integers)

| Field | Type | Description |
|-------|------|-------------|
| `universe_size` | `usize` | Size of the finite set X |
| `r_sets` | `Vec<Vec<usize>>` | Collection R: each inner Vec lists element indices in the subset |
| `s_sets` | `Vec<Vec<usize>>` | Collection S: each inner Vec lists element indices in the subset |
| `r_weights` | `Vec<u64>` | Positive integer weight for each set in R |
| `s_weights` | `Vec<u64>` | Positive integer weight for each set in S |

## Complexity

<!-- ⚠️ Unverified: AI-generated complexity -->

- **Best known exact algorithm:** Brute-force enumeration over all 2^n subsets Y of X, checking containment against all sets in R and S. Time complexity O(2^n * (k + l) * n). No specialized exact algorithm is known beyond general satisfaction techniques. The problem is NP-complete even with all weights equal to 1 (Garey & Johnson).

## Specialization

<!-- ⚠️ Unverified: AI-generated specialization -->

- The unit-weight case (all w(R_i) = w(S_j) = 1) remains NP-complete.
- When |R| = 0, the answer is trivially YES (choose Y = X or any Y such that no S_j contains Y; in the degenerate case, the LHS sum is 0 and the problem reduces to asking if the RHS sum can also be 0).

## Extra Remark

**Full book text:**

INSTANCE: Two collections R = {R_1,R_2,...,R_k} and S = {S_1,S_2,...,S_l} of subsets of a finite set X, weights w(R_i) in Z^+, 1 <= i <= k, and w(S_j) in Z^+, 1 <= j <= l.
QUESTION: Is there a subset Y <= X such that
Sum_{Y <= R_i} w(R_i) >= Sum_{Y <= S_j} w(S_j) ?
Reference: [Plaisted, 1976]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if all subsets in R and S have weight 1 [Garey and Johnson, ----].

## How to solve

- [x] It can be solved by (existing) bruteforce. (Enumerate all 2^n subsets Y of X; for each, compute containment sums and compare.)
- [x] It can be solved by reducing to integer programming. (Binary variables y_i for each element; indicator constraints for containment; maximize or constrain the difference of weighted containment counts.)
- [ ] Other: (TBD)

## Example Instance

<!-- ⚠️ Unverified: AI-constructed example -->

**Input:**
X = {0, 1, 2, 3, 4, 5} (n = 6 elements)

R = { R_1 = {0, 1, 2}, R_2 = {0, 1}, R_3 = {2, 3, 4} }
w(R_1) = 3, w(R_2) = 2, w(R_3) = 4

S = { S_1 = {0, 1, 2, 3}, S_2 = {1, 2}, S_3 = {3, 4, 5} }
w(S_1) = 5, w(S_2) = 2, w(S_3) = 3

**Feasible assignment:**
Choose Y = {0, 1} (elements 0 and 1).

Containment check for R:
- Y = {0,1} <= R_1 = {0,1,2}? YES -> contributes w(R_1) = 3
- Y = {0,1} <= R_2 = {0,1}? YES -> contributes w(R_2) = 2
- Y = {0,1} <= R_3 = {2,3,4}? NO (0 not in R_3)
Total R-weight: 3 + 2 = 5

Containment check for S:
- Y = {0,1} <= S_1 = {0,1,2,3}? YES -> contributes w(S_1) = 5
- Y = {0,1} <= S_2 = {1,2}? NO (0 not in S_2)
- Y = {0,1} <= S_3 = {3,4,5}? NO
Total S-weight: 5

Comparison: 5 >= 5? YES

Answer: YES -- Y = {0, 1} witnesses that the R-containment weight is at least the S-containment weight.
