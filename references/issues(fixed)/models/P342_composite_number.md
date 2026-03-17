---
name: Problem
about: Propose a new problem type
title: "[Model] CompositeNumber"
labels: model
assignees: ''
---

## Motivation

COMPOSITE NUMBER (P342) from Garey & Johnson, A13 OPEN11. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A13 OPEN11

**Mathematical definition:**

INSTANCE: Positive integer N.
QUESTION: Are there positive integers m, n > 1 such that N = m · n?

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

[OPEN11] COMPOSITE NUMBER
INSTANCE: Positive integer N.
QUESTION: Are there positive integers m, n > 1 such that N = m · n?
Comment: The problem is in NP ∩ co-NP [Pratt, 1975]. Although no polynomial time algorithm is known, there is an algorithm for the problem that runs in polynomial time if the "Extended Riemann Hypothesis" holds [Miller, 1976]. However, there is no such algorithm known for determining the prime factors of N, and this latter problem may be harder than the basic decision problem. Of course, all these problems are easily solved in pseudo-polynomial time.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
