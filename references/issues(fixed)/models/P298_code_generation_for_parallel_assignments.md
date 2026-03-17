---
name: Problem
about: Propose a new problem type
title: "[Model] CodeGenerationForParallelAssignments"
labels: model
assignees: ''
---

## Motivation

CODE GENERATION FOR PARALLEL ASSIGNMENTS (P298) from Garey & Johnson, A11 PO6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO6

**Mathematical definition:**

INSTANCE: Set V = {v1,v2,...,vn} of variables, set A = {A1,A2,...,An} of assignments, each Ai of the form "vi ← op(Bi)" for some subset Bi ⊆ V, and a positive integer K.
QUESTION: Is there an ordering vπ(1),vπ(2),...,vπ(n) of V such that there are at most K values of i, 1 ≤ i ≤ n, for which vπ(i) ∈ Bπ(j) for some j > i?

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

INSTANCE: Set V = {v1,v2,...,vn} of variables, set A = {A1,A2,...,An} of assignments, each Ai of the form "vi ← op(Bi)" for some subset Bi ⊆ V, and a positive integer K.
QUESTION: Is there an ordering vπ(1),vπ(2),...,vπ(n) of V such that there are at most K values of i, 1 ≤ i ≤ n, for which vπ(i) ∈ Bπ(j) for some j > i?
Reference: [Sethi, 1973]. Transformation from FEEDBACK VERTEX SET.
Comment: Remains NP-complete even if each Bi satisfies |Bi| ≤ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
