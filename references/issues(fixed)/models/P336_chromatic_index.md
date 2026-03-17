---
name: Problem
about: Propose a new problem type
title: "[Model] ChromaticIndex"
labels: model
assignees: ''
---

## Motivation

CHROMATIC INDEX (P336) from Garey & Johnson, A13 OPEN5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN5

**Mathematical definition:**

INSTANCE: Graph G = (V, E) and a positive integer K.
QUESTION: Does G have chromatic index K or less, i.e., can E be partitioned into disjoint sets E1, E2, . . . , Ek, with k ≤ K, such that, for 1 ≤ i ≤ k, no two edges in Ei share a common endpoint in G?

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

[OPEN5] CHROMATIC INDEX
INSTANCE: Graph G = (V, E) and a positive integer K.
QUESTION: Does G have chromatic index K or less, i.e., can E be partitioned into disjoint sets E1, E2, . . . , Ek, with k ≤ K, such that, for 1 ≤ i ≤ k, no two edges in Ei share a common endpoint in G?
Comment: By Vizing's Theorem (e.g., see [Berge, 1973]), the chromatic index for G is either h or h+1, where h is the maximum vertex degree in G, so the above question may be restated as "Given G, is the chromatic index of G equal to its maximum vertex degree?" The answer is always "yes" for bipartite graphs (e.g., see [Berge, 1973]), and there exist polynomial time algorithms for constructing the desired partition in this case (e.g., see [Gabow, 1976]). A particular case that is open is that for cubic graphs (i.e., regular of degree 3), in which case the problem can be restated as "Given G, can the vertices of G be covered by disjoint simple cycles, each involving an even number of vertices?" This latter problem is one of a number of open problems involving parity. Another such problem is: "Given a collection C of subsets of a finite set X, is there a nonempty subcollection C' ⊆ C such that each x ∈ X belongs to an even number (possibly 0) of sets in C'?" which is equivalent to the open problem mentioned in the comments for DECODING OF LINEAR CODES.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
