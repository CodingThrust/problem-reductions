---
name: Problem
about: Propose a new problem type
title: "[Model] ShortestCommonSuperstring"
labels: model
assignees: ''
---

## Motivation

SHORTEST COMMON SUPERSTRING (P157) from Garey & Johnson, A4 SR9. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR9

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a substring of w, i.e., w = w0xw1 where each wi ∈ Σ*?

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

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a substring of w, i.e., w = w0xw1 where each wi ∈ Σ*?
Reference: [Maier and Storer, 1977]. Transformation from VERTEX COVER for cubic graphs.
Comment: Remains NP-complete even if |Σ| = 2 or if all x ∈ R have |x| ≤ 8 and contain no repeated symbols. Solvable in polynomial time if all x ∈ R have |x| ≤ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
