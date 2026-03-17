---
name: Problem
about: Propose a new problem type
title: "[Model] FaultDetectionWithTestPoints"
labels: model
assignees: ''
---

## Motivation

FAULT DETECTION WITH TEST POINTS (P331) from Garey & Johnson, A12 MS19. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS19

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A) having exactly one vertex s ∈ V with in-degree 0 and exactly one vertex t ∈ V with out-degree 0, and a positive integer K.
QUESTION: Can all "single faults" in G be located by attaching K or fewer "test points" to arcs in A, i.e., is there a subset A' ⊆ A with |A'| ≤ K such that the test set
T = ({s} ∪ {u1: (u1,u2) ∈ A'}) × ({t} ∪ {u2: (u1,u2) ∈ A'})
has the property that, for each pair v,v' ∈ V−{s,t}, there is some (u1,u2) ∈ T such that v is on a directed path from u1 to u2 but v' is on no such path?

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

INSTANCE: Directed acyclic graph G = (V,A) having exactly one vertex s ∈ V with in-degree 0 and exactly one vertex t ∈ V with out-degree 0, and a positive integer K.
QUESTION: Can all "single faults" in G be located by attaching K or fewer "test points" to arcs in A, i.e., is there a subset A' ⊆ A with |A'| ≤ K such that the test set
T = ({s} ∪ {u1: (u1,u2) ∈ A'}) × ({t} ∪ {u2: (u1,u2) ∈ A'})
has the property that, for each pair v,v' ∈ V−{s,t}, there is some (u1,u2) ∈ T such that v is on a directed path from u1 to u2 but v' is on no such path?
Reference: [Ibaraki, Kameda, and Toida, 1977]. Transformation from X3C.
Comment: Variants in which we are asked to locate all single faults by using K or fewer "test connections" or "blocking gates" are also NP-complete, as are the problems of finding a test set T with |T| ≤ K in the presence of a fixed set of "test points," "test connections," or "blocking gates." See reference for more details.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
