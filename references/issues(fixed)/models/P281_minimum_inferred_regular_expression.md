---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumInferredRegularExpression"
labels: model
assignees: ''
---

## Motivation

MINIMUM INFERRED REGULAR EXPRESSION (P281) from Garey & Johnson, A10 AL10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A10 AL10

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, two finite subsets S, T ⊆ Σ*, positive integer K.
QUESTION: Is there a regular expression E over Σ that has K or fewer occurrences of symbols from Σ and such that, if L ⊆ Σ* is the language represented by E, then S ⊆ L and T ⊆ Σ*-L?

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

INSTANCE: Finite alphabet Σ, two finite subsets S, T ⊆ Σ*, positive integer K.
QUESTION: Is there a regular expression E over Σ that has K or fewer occurrences of symbols from Σ and such that, if L ⊆ Σ* is the language represented by E, then S ⊆ L and T ⊆ Σ*-L?
Reference: [Angluin, 1977]. Transformation from 3SAT.
Comment: Remains NP-complete even if E is required to contain no "∪" operations or to be "star-free" (contain no "*" operations) [Angluin, 1976].

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
