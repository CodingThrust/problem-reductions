---
name: Problem
about: Propose a new problem type
title: "[Model] FiniteStateAutomataIntersection"
labels: model
assignees: ''
---

## Motivation

FINITE STATE AUTOMATA INTERSECTION (P277) from Garey & Johnson, A10 AL6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL6

**Mathematical definition:**

INSTANCE: Sequence A_1,A_2, . . . ,A_n of deterministic finite state automata having the same input alphabet Σ.
QUESTION: Is there a string x ∈ Σ* accepted by each of the A_i, 1 ≤ i ≤ n?

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

INSTANCE: Sequence A_1,A_2, . . . ,A_n of deterministic finite state automata having the same input alphabet Σ.
QUESTION: Is there a string x ∈ Σ* accepted by each of the A_i, 1 ≤ i ≤ n?
Reference: [Kozen, 1977d]. Transformation from LINEAR SPACE ACCEPTANCE.
Comment: PSPACE-complete. Solvable in polynomial time for any fixed n (e.g., see [Hopcroft and Ullman, 1969]).

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
