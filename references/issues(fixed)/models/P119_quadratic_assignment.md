---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticAssignmentProblem"
labels: model
assignees: ''
---

## Motivation

QUADRATIC ASSIGNMENT PROBLEM (P119) from Garey & Johnson, A2 ND43. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A2 ND43

**Mathematical definition:**

INSTANCE: Non-negative integer costs c_{ij}, 1 ≤ i,j ≤ n, and distances d_{kl}, 1 ≤ k,l ≤ m, bound B ∈ Z^+.
QUESTION: Is there a one-to-one function f: {1,2,…,n} → {1,2,…,m} such that
Σ_{i=1}^{n} Σ_{j=1, j≠i}^{n} c_{ij} d_{f(i)f(j)} ≤ B ?

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

INSTANCE: Non-negative integer costs c_{ij}, 1 ≤ i,j ≤ n, and distances d_{kl}, 1 ≤ k,l ≤ m, bound B ∈ Z^+.
QUESTION: Is there a one-to-one function f: {1,2,…,n} → {1,2,…,m} such that
Σ_{i=1}^{n} Σ_{j=1, j≠i}^{n} c_{ij} d_{f(i)f(j)} ≤ B ?
Reference: [Sahni and Gonzalez, 1976]. Transformation from HAMILTONIAN CIRCUIT.
Comment: Special case in which each d_{kl} = k − l and all c_{ji} = c_{ij} ∈ {0,1} is the NP-complete OPTIMAL LINEAR ARRANGEMENT problem. The general problem is discussed, for example, in [Garfinkel and Nemhauser, 1972].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
