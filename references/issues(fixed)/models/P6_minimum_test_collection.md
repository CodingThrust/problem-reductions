---
name: Problem
about: Propose a new problem type
title: "[Model] MinimumTestCollection"
labels: model
assignees: ''
---

## Motivation

MINIMUM TEST COLLECTION (P6) from Garey & Johnson, Chapter 3, Section 3.2.2, p.71. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.2.2, p.71

**Mathematical definition:**

INSTANCE: A finite set A of "possible diagnoses," a collection C of subsets of A, representing binary "tests," and a positive integer J ≤ |C|.
QUESTION: Is there a subcollection C' ⊆ C with |C'| ≤ J such that, for every pair a_i, a_j of possible diagnoses from A, there is some test c ∈ C' for which |{a_i,a_j} ∩ c| = 1 (that is, a test c that "distinguishes" between a_i and a_j)?

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

INSTANCE: A finite set A of "possible diagnoses," a collection C of subsets of A, representing binary "tests," and a positive integer J ≤ |C|.
QUESTION: Is there a subcollection C' ⊆ C with |C'| ≤ J such that, for every pair a_i, a_j of possible diagnoses from A, there is some test c ∈ C' for which |{a_i,a_j} ∩ c| = 1 (that is, a test c that "distinguishes" between a_i and a_j)?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
