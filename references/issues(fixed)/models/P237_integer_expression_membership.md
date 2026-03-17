---
name: Problem
about: Propose a new problem type
title: "[Model] IntegerExpressionMembership"
labels: model
assignees: ''
---

## Motivation

INTEGER EXPRESSION MEMBERSHIP (P237) from Garey & Johnson, A7 AN18. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A7 AN18

**Mathematical definition:**

INSTANCE: Integer expression e over the operations ∪ and +, where if n ∈ Z^+, the binary representation of n is an integer expression representing n, and if f and g are integer expressions representing the sets F and G, then f ∪ g is an integer expression representing the set F ∪ G and f + g is an integer expression representing the set {m + n: m ∈ F and n ∈ G}, and a positive integer K.
QUESTION: Is K in the set represented by e?

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

INSTANCE: Integer expression e over the operations ∪ and +, where if n ∈ Z^+, the binary representation of n is an integer expression representing n, and if f and g are integer expressions representing the sets F and G, then f ∪ g is an integer expression representing the set F ∪ G and f + g is an integer expression representing the set {m + n: m ∈ F and n ∈ G}, and a positive integer K.
QUESTION: Is K in the set represented by e?

Reference: [Stockmeyer and Meyer, 1973]. Transformation from SUBSET SUM.
Comment: The related INTEGER EXPRESSION INEQUIVALENCE problem, "given two integer expressions e and f, do they represent different sets?" is NP-hard and in fact complete for Σ_2^p in the polynomial hierarchy ([Stockmeyer and Meyer, 1973], [Stockmeyer, 1976a], see also Section 7.2). If the operator "¬" is allowed, with ¬e representing the set of all positive integers not represented by e, then both the membership and inequivalence problems become PSPACE-complete [Stockmeyer and Meyer, 1973].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
