---
name: Problem
about: Propose a new problem type
title: "[Model] NumberOfRootsForAProductPolynomial"
labels: model
assignees: ''
---

## Motivation

NUMBER OF ROOTS FOR A PRODUCT POLYNOMIAL (P230) from Garey & Johnson, A7 AN11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN11

**Mathematical definition:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0, and a positive integer K.
QUESTION: Does the polynomial ∏_{i=1}^{m} (Σ_{j=1}^{k} a_i[j]·z^{b_i[j]}) have fewer than K distinct complex roots?

## Variables

- **Count:** (TBD)
- **Per-variable domain:** (TBD)
- **Meaning:** (TBD)

## Schema (data type)

**Type name:** (TBD)
**Variants:** (TBD)

| Field | Type | Description |
|-------|------|-------------|
| (TBD) | (TBD) | (TBD) |

## Complexity

- **Best known exact algorithm:** (TBD)

## Extra Remark

**Full book text:**

INSTANCE: Sequences A_i = <(a_i[1],b_i[1]), . . . , (a_i[k],b_i[k])>, 1 ≤ i ≤ m, of pairs of integers, with each b_i[j] ≥ 0, and a positive integer K.
QUESTION: Does the polynomial ∏_{i=1}^{m} (Σ_{j=1}^{k} a_i[j]·z^{b_i[j]}) have fewer than K distinct complex roots?

Reference: [Plaisted, 1977a]. Transformation from 3SAT.
Comment: Not known to be in NP or co-NP. Remains NP-hard if each a_i[j] is either -1 or +1, as does the variant in which the instance also includes an integer M and we are asked whether the product polynomial has fewer than K complex roots of multiplicity M [Plaisted, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
