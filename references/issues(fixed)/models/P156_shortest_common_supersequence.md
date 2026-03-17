---
name: Problem
about: Propose a new problem type
title: "[Model] ShortestCommonSupersequence"
labels: model
assignees: ''
---

## Motivation

SHORTEST COMMON SUPERSEQUENCE (P156) from Garey & Johnson, A4 SR8. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR8

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a subsequence of w, i.e., w = w0x1w1x2w2 ··· xkwk where each wi ∈ Σ* and x = x1x2 ··· xk?

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
QUESTION: Is there a string w ∈ Σ* with |w| ≤ K such that each string x ∈ R is a subsequence of w, i.e., w = w0x1w1x2w2 ··· xkwk where each wi ∈ Σ* and x = x1x2 ··· xk?
Reference: [Maier, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if |Σ| = 5. Solvable in polynomial time if |R| = 2 (by first computing the largest common subsequence) or if all x ∈ R have |x| ≤ 2.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
