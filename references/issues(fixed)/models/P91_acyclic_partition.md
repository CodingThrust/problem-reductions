---
name: Problem
about: Propose a new problem type
title: "[Model] AcyclicPartition"
labels: model
assignees: ''
---

## Motivation

ACYCLIC PARTITION (P91) from Garey & Johnson, A2 ND15. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND15

**Mathematical definition:**

INSTANCE: Directed graph G = (V,A), weight w(v) ∈ Z+ for each v ∈ V, cost c(a) ∈ Z+ for each a ∈ A, positive integers B and K.
QUESTION: Is there a partition of V into disjoint sets V1,V2,...,Vm such that the directed graph G' = (V',A'), where V' = {V1,V2,...,Vm}, and (Vi,Vj) ∈ A' if and only if (vi,vj) ∈ A for some vi ∈ Vi and some vj ∈ Vj, is acyclic, such that the sum of the weights of the vertices in each Vi does not exceed B, and such that the sum of the costs of all those arcs having their endpoints in different sets does not exceed K?

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

INSTANCE: Directed graph G = (V,A), weight w(v) ∈ Z+ for each v ∈ V, cost c(a) ∈ Z+ for each a ∈ A, positive integers B and K.
QUESTION: Is there a partition of V into disjoint sets V1,V2,...,Vm such that the directed graph G' = (V',A'), where V' = {V1,V2,...,Vm}, and (Vi,Vj) ∈ A' if and only if (vi,vj) ∈ A for some vi ∈ Vi and some vj ∈ Vj, is acyclic, such that the sum of the weights of the vertices in each Vi does not exceed B, and such that the sum of the costs of all those arcs having their endpoints in different sets does not exceed K?

Reference: [Garey and Johnson, ——]. Transformation from X3C.
Comment: Remains NP-complete even if all v ∈ V have w(v) = 1 and all a ∈ A have c(a) = 1. Can be solved in polynomial time if G contains a Hamiltonian path (a property that can be verified in polynomial time for acyclic digraphs) [Kernighan, 1971]. If G is a tree the general problem is NP-complete in the ordinary sense, but can be solved in pseudo-polynomial time [Lukes, 1974]. The tree problem can be solved in polynomial time if all edge weights are equal (see [Hadlock, 1974]) or if all vertex weights are equal [Garey and Johnson, ——].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
