---
name: Problem
about: Propose a new problem type
title: "[Model] Satisfiability"
labels: model
assignees: ''
---

## Motivation

SATISFIABILITY (P253) from Garey & Johnson, A9 LO1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO1

**Mathematical definition:**

INSTANCE: Set U of variables, collection C of clauses over U (see Section 2.6 for definitions).
QUESTION: Is there a satisfying truth assignment for C?

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

INSTANCE: Set U of variables, collection C of clauses over U (see Section 2.6 for definitions).
QUESTION: Is there a satisfying truth assignment for C?
Reference: [Cook, 1971a]. Generic transformation.
Comment: Remains NP-complete even if each c∈C satisfies |c|=3 (3SAT), or if each c∈C satisfies |c|≤3 and, for each u∈U, there are at most 3 clauses in C that contain either u or ū. Also remains NP-complete if each c∈C has |c|≤3 and the bipartite graph G=(V,E), where V=U∪C and E contains exactly those pairs {u,c} such that either u or ū belongs to the clause c, is planar (PLANAR 3SAT) [Lichtenstein, 1977]. The general problem is solvable in polynomial time if each c∈C has |c|≤2 (e.g., see [Even, Itai, and Shamir, 1976]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
