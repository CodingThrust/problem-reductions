---
name: Problem
about: Propose a new problem type
title: "[Model] ComparativeContainment"
labels: model
assignees: ''
---

## Motivation

COMPARATIVE CONTAINMENT (P137) from Garey & Johnson, A3 SP10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A3 SP10

**Mathematical definition:**

INSTANCE: Two collections R = {R_1,R_2,…,R_k} and S = {S_1,S_2,…,S_l} of subsets of a finite set X, weights w(R_i) ∈ Z^+, 1 ≤ i ≤ k, and w(S_j) ∈ Z^+, 1 ≤ j ≤ l.
QUESTION: Is there a subset Y ⊆ X such that
Σ_{Y ⊆ R_i} w(R_i) ≥ Σ_{Y ⊆ S_j} w(S_j) ?

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

INSTANCE: Two collections R = {R_1,R_2,…,R_k} and S = {S_1,S_2,…,S_l} of subsets of a finite set X, weights w(R_i) ∈ Z^+, 1 ≤ i ≤ k, and w(S_j) ∈ Z^+, 1 ≤ j ≤ l.
QUESTION: Is there a subset Y ⊆ X such that
Σ_{Y ⊆ R_i} w(R_i) ≥ Σ_{Y ⊆ S_j} w(S_j) ?
Reference: [Plaisted, 1976]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if all subsets in R and S have weight 1 [Garey and Johnson, ——].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
