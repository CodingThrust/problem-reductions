---
name: Problem
about: Propose a new problem type
title: "[Model] GroupingBySwapping"
labels: model
assignees: ''
---

## Motivation

GROUPING BY SWAPPING (P169) from Garey & Johnson, A4 SR21. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR21

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, string x ∈ Σ*, and a positive integer K.
QUESTION: Is there a sequence of K or fewer adjacent symbol interchanges that converts x into a string y in which all occurrences of each symbol a ∈ Σ are in a single block, i.e., y has no subsequences of the form aba for a,b ∈ Σ and a ≠ b?

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

INSTANCE: Finite alphabet Σ, string x ∈ Σ*, and a positive integer K.
QUESTION: Is there a sequence of K or fewer adjacent symbol interchanges that converts x into a string y in which all occurrences of each symbol a ∈ Σ are in a single block, i.e., y has no subsequences of the form aba for a,b ∈ Σ and a ≠ b?
Reference: [Howell, 1977]. Transformation from FEEDBACK EDGE SET.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
