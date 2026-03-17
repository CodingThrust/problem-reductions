---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumAxiomSet"
labels: model
assignees: ''
---

## Motivation

MINIMUM AXIOM SET (P269) from Garey & Johnson, A9 LO17. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A9 LO17

**Mathematical definition:**

INSTANCE: Finite set S of "sentences," subset T⊆S of "true sentences," an "implication relation" R consisting of pairs (A,s) where A⊆S and s∈S, and a positive integer K≤|S|.
QUESTION: Is there a subset S_0⊆T with |S_0|≤K and a positive integer n such that, if we define S_i, 1≤i≤n, to consist of exactly those s∈S for which either s∈S_{i-1} or there exists a U⊆S_{i-1} such that (U,s)∈R, then S_n=T?

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

INSTANCE: Finite set S of "sentences," subset T⊆S of "true sentences," an "implication relation" R consisting of pairs (A,s) where A⊆S and s∈S, and a positive integer K≤|S|.
QUESTION: Is there a subset S_0⊆T with |S_0|≤K and a positive integer n such that, if we define S_i, 1≤i≤n, to consist of exactly those s∈S for which either s∈S_{i-1} or there exists a U⊆S_{i-1} such that (U,s)∈R, then S_n=T?
Reference: [Pudlák, 1975]. Transformation from X3C.
Comment: Remains NP-complete even if T=S.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
