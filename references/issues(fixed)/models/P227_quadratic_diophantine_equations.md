---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticDiophantineEquations"
labels: model
assignees: ''
---

## Motivation

QUADRATIC DIOPHANTINE EQUATIONS (P227) from Garey & Johnson, A7 AN8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN8

**Mathematical definition:**

INSTANCE: Positive integers a, b, and c.
QUESTION: Are there positive integers x and y such that ax^2 + by = c?

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

INSTANCE: Positive integers a, b, and c.
QUESTION: Are there positive integers x and y such that ax^2 + by = c?

Reference: [Manders and Adleman, 1978]. Transformation from 3SAT.
Comment: Diophantine equations of the forms ax^k = c and Σ_{i=1}^{k} a_i·x_i = c are solvable in polynomial time for arbitrary values of k. The general Diophantine problem, "Given a polynomial with integer coefficients in k variables, does it have an integer solution?" is undecidable, even for k = 13 [Matijasevic and Robinson, 1975]. However, the given problem can be generalized considerably (to simultaneous equations in many variables) while remaining in NP, so long as only one variable enters into the equations in a non-linear way (see [Gurari and Ibarra, 1978]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
