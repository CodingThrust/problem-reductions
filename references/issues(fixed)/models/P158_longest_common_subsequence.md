---
name: Problem
about: Propose a new problem type
title: "[Model] LongestCommonSubsequence"
labels: model
assignees: ''
---

## Motivation

LONGEST COMMON SUBSEQUENCE (P158) from Garey & Johnson, A4 SR10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR10

**Mathematical definition:**

INSTANCE: Finite alphabet Σ, finite set R of strings from Σ*, and a positive integer K.
QUESTION: Is there a string w ∈ Σ* with |w| ≥ K such that w is a subsequence of each x ∈ R?

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
QUESTION: Is there a string w ∈ Σ* with |w| ≥ K such that w is a subsequence of each x ∈ R?
Reference: [Maier, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if |Σ| = 2. Solvable in polynomial time for any fixed K or for fixed |R| (by dynamic programming, e.g., see [Wagner and Fischer, 1974]). The analogous LONGEST COMMON SUBSTRING problem is trivially solvable in polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
