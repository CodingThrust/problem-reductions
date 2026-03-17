---
name: Problem
about: Propose a new problem type
title: "[Model] NonLivenessOfFreeChoicePetriNets"
labels: model
assignees: ''
---

## Motivation

NON-LIVENESS OF FREE CHOICE PETRI NETS (P315) from Garey & Johnson, A12 MS3. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS3

**Mathematical definition:**

INSTANCE: Petri net P = (n,M0,T), where n ∈ Z+, M0 is an n-tuple of non-negative integers, and T is a set of transitions <a,b> in which both a and b are n-tuples of 0's and 1's, such that P has the "free choice" property, i.e., for each <a,b> ∈ T, either a contains exactly one 1 or in every other transition <c,d> ∈ T, c has a 0 in every position where a has a 1.
QUESTION: Is P not "live," i.e., is there a transition t ∈ T and a sequence σ of transitions from T such that, for every sequence τ of transitions from T, the sequence στt is not "fireable" at M0, where <a1,b1> <a2,b2> ··· <am,bm> is fireable at M0 if and only if the sequence M0,M1,...,M2m in which M2i+1 = M2i−ai and M2i+2 = M2i+1 + bi, 0 ≤ i < m, contains no vector with a negative component?

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

INSTANCE: Petri net P = (n,M0,T), where n ∈ Z+, M0 is an n-tuple of non-negative integers, and T is a set of transitions <a,b> in which both a and b are n-tuples of 0's and 1's, such that P has the "free choice" property, i.e., for each <a,b> ∈ T, either a contains exactly one 1 or in every other transition <c,d> ∈ T, c has a 0 in every position where a has a 1.
QUESTION: Is P not "live," i.e., is there a transition t ∈ T and a sequence σ of transitions from T such that, for every sequence τ of transitions from T, the sequence στt is not "fireable" at M0, where <a1,b1> <a2,b2> ··· <am,bm> is fireable at M0 if and only if the sequence M0,M1,...,M2m in which M2i+1 = M2i−ai and M2i+2 = M2i+1 + bi, 0 ≤ i < m, contains no vector with a negative component?
Reference: [Jones, Landweber, and Lien, 1977]. Transformation from 3SAT. Proof of membership in NP is nontrivial and is based on a result of [Hack, 1972].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
