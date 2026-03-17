---
name: Problem
about: Propose a new problem type
title: "[Model] IntegerProgramming"
labels: model
assignees: ''
---

## Motivation

INTEGER PROGRAMMING (P207) from Garey & Johnson, A6 MP1. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A6 MP1

**Mathematical definition:**

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, an m-tuple c̄ of integers, and an integer B.
QUESTION: Is there an m-tuple ȳ of integers such that x̄·ȳ ≤ b for all (x̄,b) ∈ X and such that c̄·ȳ ≥ B (where the dot-product ū·v̄ of two m-tuples ū = (u₁,u₂,...,uₘ) and v̄ = (v₁,v₂,...,vₘ) is given by Σᵢ₌₁ᵐ uᵢvᵢ)?

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

INSTANCE: Finite set X of pairs (x̄,b), where x̄ is an m-tuple of integers and b is an integer, an m-tuple c̄ of integers, and an integer B.
QUESTION: Is there an m-tuple ȳ of integers such that x̄·ȳ ≤ b for all (x̄,b) ∈ X and such that c̄·ȳ ≥ B (where the dot-product ū·v̄ of two m-tuples ū = (u₁,u₂,...,uₘ) and v̄ = (v₁,v₂,...,vₘ) is given by Σᵢ₌₁ᵐ uᵢvᵢ)?

Reference: [Karp, 1972], [Borosh and Treybig, 1976]. Transformation from 3SAT. The second reference proves membership in NP.
Comment: NP-complete in the strong sense. Variant in which all components of ȳ are required to belong to {0,1} (ZERO-ONE INTEGER PROGRAMMING) is also NP-complete, even if each b, all components of each x̄, and all components of c̄ are required to belong to {0,1}. Also NP-complete are the questions of whether a ȳ with non-negative integer entries exists such that x̄·ȳ = b for all (x̄,b) ∈ X, and the question of whether there exists any ȳ with integer entries such that x̄·ȳ ≥ 0 for all (x̄,b) ∈ X [Sahni, 1974].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
