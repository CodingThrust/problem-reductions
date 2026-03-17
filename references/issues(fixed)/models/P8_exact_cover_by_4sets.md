---
name: Problem
about: Propose a new problem type
title: "[Model] ExactCoverBy4Sets"
labels: model
assignees: ''
---

## Motivation

EXACT COVER BY 4-SETS (P8) from Garey & Johnson, Chapter 3, Section 3.3, p.75. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, Chapter 3, Section 3.3, p.75

**Mathematical definition:**

INSTANCE: Finite set X with |X| = 4q, q an integer, and a collection C of 4-element subsets of X.
QUESTION: Is there a subcollection C' ⊆ C such that every element of X occurs in exactly one member of C'?

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

INSTANCE: Finite set X with |X| = 4q, q an integer, and a collection C of 4-element subsets of X.
QUESTION: Is there a subcollection C' ⊆ C such that every element of X occurs in exactly one member of C'?

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
