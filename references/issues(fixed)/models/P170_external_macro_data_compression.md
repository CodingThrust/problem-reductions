---
name: Problem
about: Propose a new problem type
title: "[Model] ExternalMacroDataCompression"
labels: model
assignees: ''
---

## Motivation

EXTERNAL MACRO DATA COMPRESSION (P170) from Garey & Johnson, A4 SR22. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR22

**Mathematical definition:**

INSTANCE: Alphabet Σ, string s ∈ Σ*, pointer cost h ∈ Z+, and a bound B ∈ Z+.
QUESTION: Are there strings D (dictionary string) and C (compressed string) in (Σ ∪ {pi: 1 ≤ i ≤ |s|})*, where the symbols pi are "pointers," such that
|D| + |C| + (h−1)·(number of occurrences of pointers in D and C) ≤ B
and such that there is a way of identifying pointers with substrings of D so that S can be obtained from C by repeatedly replacing pointers in C by their corresponding substrings in D?

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

INSTANCE: Alphabet Σ, string s ∈ Σ*, pointer cost h ∈ Z+, and a bound B ∈ Z+.
QUESTION: Are there strings D (dictionary string) and C (compressed string) in (Σ ∪ {pi: 1 ≤ i ≤ |s|})*, where the symbols pi are "pointers," such that
|D| + |C| + (h−1)·(number of occurrences of pointers in D and C) ≤ B
and such that there is a way of identifying pointers with substrings of D so that S can be obtained from C by repeatedly replacing pointers in C by their corresponding substrings in D?
Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if h is any fixed integer 2 or greater. Many variants, including those in which D can contain no pointers and/or no pointers can refer to overlapping strings, are also NP-complete. If the alphabet size is fixed at 3 or greater, and the pointer cost is ⌈h·log|s|⌉, the problem is also NP-complete. For further variants, including the case of "original pointers," see references.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
