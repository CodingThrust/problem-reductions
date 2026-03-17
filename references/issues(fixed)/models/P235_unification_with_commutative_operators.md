---
name: Problem
about: Propose a new problem type
title: "[Model] UnificationWithCommutativeOperators"
labels: model
assignees: ''
---

## Motivation

UNIFICATION WITH COMMUTATIVE OPERATORS (P235) from Garey & Johnson, A7 AN16. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN16

**Mathematical definition:**

INSTANCE: Set V of variables, set C of constants, ordered pairs (e_i, f_i), 1 ≤ i ≤ n, of "expressions," where an expression is either a variable from V, a constant from C, or (e + f) where e and f are expressions.
QUESTION: Is there an assignment to each v ∈ V of a variable-free expression I(v) such that, if I(e) denotes the expression obtained by replacing each occurrence of each variable v in e by I(v), then I(e_i) ≡ I(f_i) for 1 ≤ i ≤ n, where e ≡ f if e = f or if e = (a+b), f = (c+d), and either a ≡ c and b ≡ d or a ≡ d and b ≡ c?

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

INSTANCE: Set V of variables, set C of constants, ordered pairs (e_i, f_i), 1 ≤ i ≤ n, of "expressions," where an expression is either a variable from V, a constant from C, or (e + f) where e and f are expressions.
QUESTION: Is there an assignment to each v ∈ V of a variable-free expression I(v) such that, if I(e) denotes the expression obtained by replacing each occurrence of each variable v in e by I(v), then I(e_i) ≡ I(f_i) for 1 ≤ i ≤ n, where e ≡ f if e = f or if e = (a+b), f = (c+d), and either a ≡ c and b ≡ d or a ≡ d and b ≡ c?

Reference: [Sethi, 1977b]. Transformation from 3SAT.
Comment: Remains NP-complete even if no e_j or f_i contains more than 7 occurrences of constants and variables. The variant in which the operator is non-commutative (and hence e ≡ f only if e = f) is solvable in polynomial time [Paterson and Wegman, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
