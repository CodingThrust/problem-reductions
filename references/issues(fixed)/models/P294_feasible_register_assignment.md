---
name: Problem
about: Propose a new problem type
title: "[Model] FeasibleRegisterAssignment"
labels: model
assignees: ''
---

## Motivation

FEASIBLE REGISTER ASSIGNMENT (P294) from Garey & Johnson, A11 PO2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A11 PO2

**Mathematical definition:**

INSTANCE: Directed acyclic graph G = (V,A), positive integer K, and a register assignment f: V→{R1,R2,...,Rk}.
QUESTION: Is there a computation for G using the given register assignment, i.e., an ordering v1,v2,...,vn of V and a sequence S0,S1,...,Sn of subsets of V that satisfies all the properties given in REGISTER SUFFICIENCY and that in addition satisfies, for 1 ≤ j ≤ K and 1 ≤ i ≤ n, there is at most one vertex u ∈ Si for which f(u) = Rj?

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

INSTANCE: Directed acyclic graph G = (V,A), positive integer K, and a register assignment f: V→{R1,R2,...,Rk}.
QUESTION: Is there a computation for G using the given register assignment, i.e., an ordering v1,v2,...,vn of V and a sequence S0,S1,...,Sn of subsets of V that satisfies all the properties given in REGISTER SUFFICIENCY and that in addition satisfies, for 1 ≤ j ≤ K and 1 ≤ i ≤ n, there is at most one vertex u ∈ Si for which f(u) = Rj?
Reference: [Sethi, 1975]. Transformation from 3SAT.
Comment: Remains NP-complete even if all vertices of G have out-degree 2 or less.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
