---
name: Problem
about: Propose a new problem type
title: "[Model] SpanningTreeParityProblem"
labels: model
assignees: ''
---

## Motivation

SPANNING TREE PARITY PROBLEM (P337) from Garey & Johnson, A13 OPEN6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN6

**Mathematical definition:**

INSTANCE: Graph G = (V, E) and a partition of E into disjoint 2-element sets E1, E2, . . . , Em.
QUESTION: Is there a spanning tree T = (V, E') for G such that for each Ei, 1 ≤ i ≤ m, either Ei ⊆ E' or Ei ∩ E' = ∅?

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

[OPEN6] SPANNING TREE PARITY PROBLEM
INSTANCE: Graph G = (V, E) and a partition of E into disjoint 2-element sets E1, E2, . . . , Em.
QUESTION: Is there a spanning tree T = (V, E') for G such that for each Ei, 1 ≤ i ≤ m, either Ei ⊆ E' or Ei ∩ E' = ∅?
Comment: This is a typical special case of the general "matroid parity problem" (e.g., see [Lawler, 1976a]), which is itself a generalization of graph matching and the two matroid intersection problem, both of which can be solved in polynomial time (assuming, in the matroid case, that there exist polynomial time algorithms for telling whether a set is an independent set of the matroids in question). The related "multiple choice spanning tree" problem, where at most one member of each Ei can be in E', is a special case of the two matroid intersection problem and hence can be solved in polynomial time (see MULTIPLE CHOICE BRANCHING).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
