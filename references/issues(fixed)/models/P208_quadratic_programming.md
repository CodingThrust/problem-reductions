---
name: Problem
about: Propose a new problem type
title: "[Model] QuadraticProgramming(*)"
labels: model
assignees: ''
---

## Motivation

QUADRATIC PROGRAMMING (*) (P208) from Garey & Johnson, A6 MP2. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP2

**Mathematical definition:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of rational numbers and b is a rational number, two m-tuples c̄ and d̄ of rational numbers, and a rational number B.
QUESTION: Is there an m-tuple ȳ of rational numbers such that x̄·ȳ ≤ b for all (x̄,b) ∈ X and such that Σᵢ₌₁ᵐ (cᵢyᵢ² + dᵢyᵢ) ≥ B, where cᵢ, yᵢ, and dᵢ denote the iᵗʰ components of c̄, ȳ, and d̄ respectively?

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

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of rational numbers and b is a rational number, two m-tuples c̄ and d̄ of rational numbers, and a rational number B.
QUESTION: Is there an m-tuple ȳ of rational numbers such that x̄·ȳ ≤ b for all (x̄,b) ∈ X and such that Σᵢ₌₁ᵐ (cᵢyᵢ² + dᵢyᵢ) ≥ B, where cᵢ, yᵢ, and dᵢ denote the iᵗʰ components of c̄, ȳ, and d̄ respectively?

Reference: [Sahni, 1974]. Transformation from PARTITION.
Comment: Not known to be in NP, unless the cᵢ's are all non-negative [Klee, 1978]. If the constraints are quadratic and the objective function is linear (the reverse of the situation above), then the problem is also NP-hard [Sahni, 1974]. If we add to this last problem the requirement that all entries of ȳ be integers, then the problem becomes undecidable [Jeroslow, 1973].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
