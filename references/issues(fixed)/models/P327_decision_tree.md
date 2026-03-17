---
name: Problem
about: Propose a new problem type
title: "[Model] DecisionTree"
labels: model
assignees: ''
---

## Motivation

DECISION TREE (P327) from Garey & Johnson, A12 MS15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS15

**Mathematical definition:**

INSTANCE: Finite set X of objects, collection T = {T1,T2,...,Tm} of binary tests Ti: X→{0,1}, positive integer K.
QUESTION: Is there a decision tree for X using the tests in T that has total external path length K or less? (A decision tree is a binary tree in which each non-leaf vertex is labelled by a test from T, each leaf is labelled by an object from X, the edge from a non-leaf vertex to its left son is labelled 0 and the one to its right son is labelled 1, and, if Ti1,Oi1,Ti2,Oi2,...,Tik,Oik is the sequence of vertex and edge labels on the path from the root to a leaf labelled by x ∈ X, then x is the unique object for which Tij(x) = Oij for all j, 1 ≤ j ≤ k. The total external path length of such a tree is the sum, over all leaves, of the number of edges on the path from the root to that leaf.)

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

INSTANCE: Finite set X of objects, collection T = {T1,T2,...,Tm} of binary tests Ti: X→{0,1}, positive integer K.
QUESTION: Is there a decision tree for X using the tests in T that has total external path length K or less? (A decision tree is a binary tree in which each non-leaf vertex is labelled by a test from T, each leaf is labelled by an object from X, the edge from a non-leaf vertex to its left son is labelled 0 and the one to its right son is labelled 1, and, if Ti1,Oi1,Ti2,Oi2,...,Tik,Oik is the sequence of vertex and edge labels on the path from the root to a leaf labelled by x ∈ X, then x is the unique object for which Tij(x) = Oij for all j, 1 ≤ j ≤ k. The total external path length of such a tree is the sum, over all leaves, of the number of edges on the path from the root to that leaf.)
Reference: [Hyafil and Rivest, 1976]. Transformation from X3C.
Comment: Remains NP-complete even if for each Ti ∈ T there are at most three distinct objects x ∈ X for which Ti(x) = 1.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
