---
name: Problem
about: Propose a new problem type
title: "[Model] CodeGenerationWithUnlimitedRegisters"
labels: model
assignees: ''
---

## Motivation

CODE GENERATION WITH UNLIMITED REGISTERS (P297) from Garey & Johnson, A11 PO5. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO5

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A) in which no vertex has out-degree larger than 2, partition of A into disjoints sets L and R such that two arcs leaving the same vertex always belong to different sets, and a positive integer K.
QUESTION: Is there a program with K or fewer instructions for computing all the root vertices of G, starting with all the leaves of G stored in registers and using only instructions of the form "ri ← rj" or "ri ← ri op rj," i,j ∈ Z+, where a vertex v with out-degree 2 and outgoing arcs (v,u) ∈ L and (v,w) ∈ R can be computed only by an instruction ri ← ri op rj when ri contains u and rj contains w?

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

INSTANCE: Directed acyclic graph G = (V,A) in which no vertex has out-degree larger than 2, partition of A into disjoints sets L and R such that two arcs leaving the same vertex always belong to different sets, and a positive integer K.
QUESTION: Is there a program with K or fewer instructions for computing all the root vertices of G, starting with all the leaves of G stored in registers and using only instructions of the form "ri ← rj" or "ri ← ri op rj," i,j ∈ Z+, where a vertex v with out-degree 2 and outgoing arcs (v,u) ∈ L and (v,w) ∈ R can be computed only by an instruction ri ← ri op rj when ri contains u and rj contains w?
Reference: [Aho, Johnson, and Ullman, 1977a]. Transformation from FEEDBACK VERTEX SET.
Comment: Remains NP-complete even if only leaves of G have in-degree exceeding 1. The "commutative" variant in which instructions of the form "ri ← rj op ri" are also allowed is NP-complete [Aho, Johnson, and Ullman, 1977b]. Both problems can be solved in polynomial time if G is a forest or if 3-address instructions "ri ← rj op rk" are allowed [Aho, Johnson, and Ullman, 1977a].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
