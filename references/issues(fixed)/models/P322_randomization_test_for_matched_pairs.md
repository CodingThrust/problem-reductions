---
name: Problem
about: Propose a new problem type
title: "[Model] RandomizationTestForMatchedPairs(*)"
labels: model
assignees: ''
---

## Motivation

RANDOMIZATION TEST FOR MATCHED PAIRS (*) (P322) from Garey & Johnson, A12 MS10. A classical NP-complete problem useful for reductions.

## Definition

**Name:** (TBD — Rust name)
**Reference:** Garey & Johnson, *Computers and Intractability*, A12 MS10

**Mathematical definition:**

INSTANCE: Sequence (x1,y1),(x2,y2),...,(xn,yn) of ordered pairs of integers, nonnegative integer K.
QUESTION: Are there at least K subsets S ⊆ {1,2,...,n} for which
∑i∈S |xi−yi| ≤ ∑xi>yi (xi−yi) ?

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

INSTANCE: Sequence (x1,y1),(x2,y2),...,(xn,yn) of ordered pairs of integers, nonnegative integer K.
QUESTION: Are there at least K subsets S ⊆ {1,2,...,n} for which
∑i∈S |xi−yi| ≤ ∑xi>yi (xi−yi) ?
Reference: [Shamos, 1976]. Transformation from PARTITION.
Comment: Not known to be in NP. The corresponding enumeration problem is #P-complete, but solvable in pseudo-polynomial time by dynamic programming.

## How to solve

- [ ] It can be solved by (existing) bruteforce.
- [ ] It can be solved by reducing to integer programming.
- [ ] Other: (TBD)

## Example Instance

(TBD)
