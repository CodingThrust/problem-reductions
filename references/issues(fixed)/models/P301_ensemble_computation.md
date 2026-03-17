---
name: Problem
about: Propose a new problem type
title: "[Model] EnsembleComputation"
labels: model
assignees: ''
---

## Motivation

ENSEMBLE COMPUTATION (P301) from Garey & Johnson, A11 PO9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO9

**Mathematical definition:**

INSTANCE: Collection C of subsets of a finite set A, positive integer J.
QUESTION: Is there a sequence S = (z1 ← x1 ∪ y1, z2 ← x2 ∪ y2,...,zj ← xj ∪ yj) of j ≤ J union operations, where each xi and yi is either {a} for some a ∈ A or zk for some k < i, such that xi and yi are disjoint, 1 ≤ i ≤ j, and such that for every subset c ∈ C there exists some zi, 1 ≤ i ≤ j, that is identical to c?

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

INSTANCE: Collection C of subsets of a finite set A, positive integer J.
QUESTION: Is there a sequence S = (z1 ← x1 ∪ y1, z2 ← x2 ∪ y2,...,zj ← xj ∪ yj) of j ≤ J union operations, where each xi and yi is either {a} for some a ∈ A or zk for some k < i, such that xi and yi are disjoint, 1 ≤ i ≤ j, and such that for every subset c ∈ C there exists some zi, 1 ≤ i ≤ j, that is identical to c?
Reference: [Garey and Johnson, ——]. Transformation from VERTEX COVER (see Section 3.2.2).
Comment: Remains NP-complete even if each c ∈ C satisfies |c| ≤ 3. The analogous problem in which xi and yi need not be disjoint for 1 ≤ i ≤ j is also NP-complete under the same restriction.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
