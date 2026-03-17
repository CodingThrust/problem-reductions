---
name: Problem
about: Propose a new problem type
title: "[Model] LinearProgramming"
labels: model
assignees: ''
---

## Motivation

LINEAR PROGRAMMING (P340) from Garey & Johnson, A13 OPEN9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN9

**Mathematical definition:**

INSTANCE: Integer-valued vectors Vi = (vi[1], vi[2], . . . , vi[n]), 1 ≤ i ≤ m, D = (d1, d2, . . . , dm), and C = (c1, c2, . . . , cn), and an integer B.
QUESTION: Is there a vector X = (x1, x2, . . . , xn) of rational numbers such that, for 1 ≤ i ≤ m, Vi · X ≤ di and such that C · X ≥ B?

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

[OPEN9] LINEAR PROGRAMMING
INSTANCE: Integer-valued vectors Vi = (vi[1], vi[2], . . . , vi[n]), 1 ≤ i ≤ m, D = (d1, d2, . . . , dm), and C = (c1, c2, . . . , cn), and an integer B.
QUESTION: Is there a vector X = (x1, x2, . . . , xn) of rational numbers such that, for 1 ≤ i ≤ m, Vi · X ≤ di and such that C · X ≥ B?
Comment: The problem is in NP ∩ co-NP (membership in co-NP follows from the fundamental duality theorem of linear programming). For any fixed value of m, the problem can be solved in polynomial time. There are many variants of LINEAR PROGRAMMING that are polynomially equivalent to it (e.g., see [Reiss and Dobkin, 1976]). One such variant is that in which we drop the vector C from the instance and drop the requirement that C · X ≥ B (see also [Papadimitriou, 1978b]). Examples of network flow problems polynomially equivalent to LINEAR PROGRAMMING are mentioned in the comments for UNDIRECTED FLOW WITH LOWER BOUNDS, PATH CONSTRAINED NETWORK FLOW, and TWO COMMODITY INTEGRAL FLOW. A generalization of LINEAR PROGRAMMING that is also open though still in NP is the "linear complementarity" problem (see [Murty, 1976]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
