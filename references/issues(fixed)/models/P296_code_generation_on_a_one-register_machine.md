---
name: Problem
about: Propose a new problem type
title: "[Model] CodeGenerationOnAOneRegisterMachine"
labels: model
assignees: ''
---

## Motivation

CODE GENERATION ON A ONE-REGISTER MACHINE (P296) from Garey & Johnson, A11 PO4. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO4

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A) in which no vertex has out-degree larger than 2, and a positive integer K.
QUESTION: Is there a program with K or fewer instructions for computing all the root vertices of G (i.e., those with in-degree 0) on a one-register machine, starting with all the leaves of G (i.e., those with out-degree 0) in memory and using only LOAD, STORE, and OP instructions? (A LOAD instruction copies a specified vertex into the register. A STORE instruction copies the vertex in the register into memory. A new vertex v can be computed by an OP instruction if the vertex u in the register is such that (v,u) ∈ A and, if there is another vertex u' such that (v,u') ∈ A, then u' is in memory. Execution of the OP instruction replaces u by v in the register. The computation of a new vertex is not completed until it is copied into memory by a STORE instruction.)

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

INSTANCE: Directed acyclic graph G = (V,A) in which no vertex has out-degree larger than 2, and a positive integer K.
QUESTION: Is there a program with K or fewer instructions for computing all the root vertices of G (i.e., those with in-degree 0) on a one-register machine, starting with all the leaves of G (i.e., those with out-degree 0) in memory and using only LOAD, STORE, and OP instructions? (A LOAD instruction copies a specified vertex into the register. A STORE instruction copies the vertex in the register into memory. A new vertex v can be computed by an OP instruction if the vertex u in the register is such that (v,u) ∈ A and, if there is another vertex u' such that (v,u') ∈ A, then u' is in memory. Execution of the OP instruction replaces u by v in the register. The computation of a new vertex is not completed until it is copied into memory by a STORE instruction.)
Reference: [Bruno and Sethi, 1976]. Transformation from 3SAT.
Comment: Remains NP-complete even if all vertices having in-degree larger than one have arcs only to leaves of G [Aho, Johnson, and Ullman, 1977a]. Solvable in polynomial time if G is a directed forest [Sethi and Ullman, 1970].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
