---
name: Problem
about: Propose a new problem type
title: "[Model] OpenHemisphere"
labels: model
assignees: ''
---

## Motivation

OPEN HEMISPHERE (P212) from Garey & Johnson, A6 MP6. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP6

**Mathematical definition:**

INSTANCE: Finite set X of m-tuples of integers, and a positive integer K ≤ |X|.
QUESTION: Is there an m-tuple ȳ of rational numbers such that x̄·ȳ > 0 for at least K m-tuples x̄ ∈ X?

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

INSTANCE: Finite set X of m-tuples of integers, and a positive integer K ≤ |X|.
QUESTION: Is there an m-tuple ȳ of rational numbers such that x̄·ȳ > 0 for at least K m-tuples x̄ ∈ X?

Reference: [Johnson and Preparata, 1978]. Transformation from MAXIMUM 2-SATISFIABILITY.
Comment: NP-complete in the strong sense, but solvable in polynomial time for any fixed m, even in a "weighted" version of the problem. The same results hold for the related CLOSED HEMISPHERE problem in which we ask that ȳ satisfy x̄·ȳ ≥ 0 for at least K m-tuples x̄ ∈ X [Johnson and Preparata, 1978]. If K = 0 or K = |X|, both problems are polynomially equivalent to linear programming [Reiss and Dobkin, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
