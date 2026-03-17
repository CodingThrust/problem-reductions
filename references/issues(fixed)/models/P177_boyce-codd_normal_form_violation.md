---
name: Problem
about: Propose a new problem type
title: "[Model] BoyceCoddNormalFormViolation"
labels: model
assignees: ''
---

## Motivation

BOYCE-CODD NORMAL FORM VIOLATION (P177) from Garey & Johnson, A4 SR29. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR29

**Mathematical definition:**

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a subset A' ⊆ A.
QUESTION: Does A' violate Boyce-Codd normal form for the relational system <A,F>, i.e., is there a subset X ⊆ A' and two attribute names y,z ∈ A' − X such that (X,{y}) ∈ F* and (X,{z}) ∉ F*, where F* is the closure of F?

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

INSTANCE: A set A of attribute names, a collection F of functional dependencies on A, and a subset A' ⊆ A.
QUESTION: Does A' violate Boyce-Codd normal form for the relational system <A,F>, i.e., is there a subset X ⊆ A' and two attribute names y,z ∈ A' − X such that (X,{y}) ∈ F* and (X,{z}) ∉ F*, where F* is the closure of F?
Reference: [Bernstein and Beeri, 1976], [Beeri and Bernstein, 1978]. Transformation from HITTING SET.
Comment: Remains NP-complete even if A' is required to satisfy "third normal form," i.e., if X ⊆ A' is a key for the system <A',F> and if two names y,z ∈ A'−X satisfy (X,{y}) ∈ F* and (X,{z}) ∉ F*, then z is a prime attribute for <A',F>.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
