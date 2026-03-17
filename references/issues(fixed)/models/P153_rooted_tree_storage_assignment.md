---
name: Problem
about: Propose a new problem type
title: "[Model] RootedTreeStorageAssignment"
labels: model
assignees: ''
---

## Motivation

ROOTED TREE STORAGE ASSIGNMENT (P153) from Garey & Johnson, A4 SR5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR5

**Mathematical definition:**

INSTANCE: Finite set X, collection C = {X1,X2,...,Xn} of subsets of X, positive integer K.
QUESTION: Is there a collection C' = {X1',X2',...,Xn'} of subsets of X such that Xi ⊆ Xi' for 1 ≤ i ≤ n, such that ∑n i=1 |Xi' − Xi| ≤ K, and such that there is a directed rooted tree T = (X,A) in which the elements of each Xi', 1 ≤ i ≤ n, form a directed path?

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

INSTANCE: Finite set X, collection C = {X1,X2,...,Xn} of subsets of X, positive integer K.
QUESTION: Is there a collection C' = {X1',X2',...,Xn'} of subsets of X such that Xi ⊆ Xi' for 1 ≤ i ≤ n, such that ∑n i=1 |Xi' − Xi| ≤ K, and such that there is a directed rooted tree T = (X,A) in which the elements of each Xi', 1 ≤ i ≤ n, form a directed path?
Reference: [Gavril, 1977a]. Transformation from ROOTED TREE ARRANGEMENT.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
