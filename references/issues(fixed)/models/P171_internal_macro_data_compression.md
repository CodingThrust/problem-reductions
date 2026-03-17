---
name: Problem
about: Propose a new problem type
title: "[Model] InternalMacroDataCompression"
labels: model
assignees: ''
---

## Motivation

INTERNAL MACRO DATA COMPRESSION (P171) from Garey & Johnson, A4 SR23. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A4 SR23

**Mathematical definition:**

INSTANCE: Alphabet Σ, string s ∈ Σ*, pointer cost h ∈ Z+, and a bound B ∈ Z+.
QUESTION: Is there a single string C ∈ (Σ ∪ {pi: 1 ≤ i ≤ |s|})* such that
|C| + (h−1)· (number of occurences of pointers in C) ≤ B
and such that there is a way of identifying pointers with substrings of C so that s can be obtained from C by using C as both compressed string and dictionary string in the manner indicated in the previous problem?

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
QUESTION: Is there a single string C ∈ (Σ ∪ {pi: 1 ≤ i ≤ |s|})* such that
|C| + (h−1)· (number of occurences of pointers in C) ≤ B
and such that there is a way of identifying pointers with substrings of C so that s can be obtained from C by using C as both compressed string and dictionary string in the manner indicated in the previous problem?
Reference: [Storer, 1977], [Storer and Szymanski, 1978]. Transformation from VERTEX COVER.
Comment: Remains NP-complete even if h is any fixed integer 2 or greater. For other NP-complete variants (as in the previous problem), see references.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
